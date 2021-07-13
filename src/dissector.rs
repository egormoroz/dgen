use crate::rand::{Rng, rngs::SmallRng, SeedableRng};

type Range = std::ops::Range<u16>;

const MIN_SIZE: u16 = 4;
const AVG_AREA: u16 = 36;
const AREA_THRESHOLD: u16 = 4;
const SFACTOR_THRESHOLD: f32 = 1.5;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

impl Rect {
    pub fn center(&self) -> (u16, u16) {
        ((self.left + self.right) / 2, (self.top + self.bottom) / 2)
    }

    pub fn area(&self) -> u16 {
        self.width() * self.height()
    }

    pub fn shape_factor(&self) -> f32 {
        let (w, h) = (self.width() as f32, self.height() as f32);
        w.max(h) / w.min(h)
    }

    pub fn width(&self) -> u16 {
        self.right - self.left
    }

    pub fn height(&self) -> u16 {
        self.bottom - self.top
    }

    fn vsplittable(&self) -> bool {
        self.width() > MIN_SIZE * 2
    }

    fn hsplittable(&self) -> bool {
        self.height() > MIN_SIZE * 2
    }

    fn vsplit_range(&self) -> Range {
        assert!(self.vsplittable());
        self.left+MIN_SIZE..self.right-MIN_SIZE
    }

    fn hsplit_range(&self) -> Range {
        assert!(self.hsplittable());
        self.top+MIN_SIZE..self.bottom-MIN_SIZE
    }

    fn vsplit(&self, midpoint: u16) -> (Rect, Rect) {
        (Rect { 
            left: self.left, top: self.top, 
            right: midpoint, bottom: self.bottom 
        }, Rect {
            left: midpoint, top: self.top, 
            right: self.right, bottom: self.bottom 
        })
    }

    fn hsplit(&self, midpoint: u16) -> (Rect, Rect) {
        (Rect { 
            left: self.left, top: self.top, 
            right: self.right, bottom: midpoint 
        }, Rect {
            left: self.left, top: midpoint, 
            right: self.right, bottom: self.bottom 
        })
    }

    fn trimmed(self, w: f32, h: f32) -> Self {
        let width = (self.width() as f32) * w;
        let height = (self.height() as f32) * h;
        Self {
            left: self.left, 
            top: self.top,
            right: self.left + MIN_SIZE.max(width as u16),
            bottom: self.top + MIN_SIZE.max(height as u16),
        }
    }
}

#[derive(Clone, Copy)]
struct Node {
    bounds: Rect,
    children: Option<(usize, usize)>,
}

pub struct Dissector {
    nodes: Vec<Node>,
    rng: SmallRng,
}

impl Dissector {
    pub fn new(bounds: Rect) -> Dissector {
        let mut inst = Self {
            nodes: vec![Node {
                bounds,
                children: None,
            }],
            rng: SmallRng::from_entropy(),
        };
        println!("{}", inst.dissect(0));
        inst
    }

    fn dissect(&mut self, idx: usize) -> u16 {
        let bounds = self.nodes[idx].bounds;
        let n = self.nodes.len();

        //basically oddity of a room. the stranger the room, the more we want to split it
        let mut chance = 1.0 - (AVG_AREA.min(bounds.area()) as f32) / (AVG_AREA.max(bounds.area()) as f32);
        chance += 0.3 * (bounds.shape_factor() - SFACTOR_THRESHOLD) / SFACTOR_THRESHOLD ;
        let do_split = self.rng.gen_range(0.0..=1.0) <= chance || bounds.area() / AVG_AREA >= AREA_THRESHOLD;

        let children = if bounds.hsplittable() && do_split {
            if bounds.vsplittable() && self.rng.gen() {
                let midpoint = self.rng.gen_range(bounds.vsplit_range());
                Some(bounds.vsplit(midpoint))
            } else {
                let midpoint = self.rng.gen_range(bounds.hsplit_range());
                Some(bounds.hsplit(midpoint))
            }
        } else if bounds.vsplittable() && do_split {
            let midpoint = self.rng.gen_range(bounds.vsplit_range());
            Some(bounds.vsplit(midpoint))
        } else {
            None
        };

        let mut n_leaves = 1;
        if let Some((p, q)) = children {
            self.nodes[idx].children = Some((n, n+1));

            let pw = 1.0;//self.rng.gen_range(0.95..=1.0);
            let ph = 1.0;//self.rng.gen_range(0.95..=1.0);
            let qw = 1.0;//self.rng.gen_range(0.95..=1.0);
            let qh = 1.0;//self.rng.gen_range(0.95..=1.0);

            self.nodes.push(Node {
                bounds: p.trimmed(pw, ph),
                children: None,
            });
            self.nodes.push(Node {
                bounds: q.trimmed(qw, qh),
                children: None,
            });
            n_leaves = self.dissect(n);
            n_leaves += self.dissect(n+1);
        }
        
        n_leaves
    }

    pub fn rooms(&self, n: usize) -> RoomIterator {
        RoomIterator {
            nodes: &self.nodes,
            rng: self.rng.clone(),
            idx: 0,
            cnt: 0,
            n: n.min(self.nodes.len())
        }
    }
}

pub struct RoomIterator<'a> {
    nodes: &'a [Node],
    idx: usize,
    rng: SmallRng,
    cnt: usize,
    n: usize,
}

impl<'a> Iterator for RoomIterator<'a> {
    type Item = &'a Rect;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cnt < self.n {
            if self.idx >= self.nodes.len() {
                self.idx = 0;
            }
            let idx = self.idx;
            self.idx += 1;
            if self.nodes[idx].children.is_none() {
                if self.rng.gen_range(1..=100) <= 25 {
                    // let bounds = self.nodes[idx].bounds;
                    // assert!(!bounds.vsplittable() && !bounds.hsplittable());
                    self.cnt += 1;
                    return Some(&self.nodes[idx].bounds);
                } 
            }
        }
        None
    }
}
