extern crate rand;
pub mod dissector;
pub mod pf;
use std::fmt;

use dissector::Rect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Blank,
    Wall,
    Coin,
    Path,
}

impl Cell {
    pub fn walkable(&self) -> bool {
        match self {
            Cell::Wall => false,
            _ => true,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Cell::Blank => '.',
            Cell::Wall => 'X',
            Cell::Coin => 'o',
            Cell::Path => '-',
        }
    }
}

pub const GRID_SIZE: u16 = 64;

pub struct Grid {
    data: [[Cell; GRID_SIZE as usize]; GRID_SIZE as usize],
}

impl Grid {
    fn contains(&self, x: u16, y: u16) -> bool {
        x < GRID_SIZE && y < GRID_SIZE
    }

    pub fn new() -> Self {
        Self {
            data: [[Cell::Wall; GRID_SIZE as usize]; GRID_SIZE as usize],
        }
    }

    pub fn put(&mut self, c: Cell, x: u16, y: u16) {
        assert!(self.contains(x, y));
        self.data[y as usize][x as usize] = c;
    }

    pub fn put_vert_line(&mut self, x: u16, y: u16, yy: u16, cell: Cell) {
        for y in y.min(yy)..=y.max(yy) {
            self.data[y as usize][x as usize] = cell;
        }
    }

    pub fn put_hor_line(&mut self, x: u16, y: u16, xx: u16, cell: Cell) {
        for x in x.min(xx)..=x.max(xx) {
            self.data[y as usize][x as usize] = cell;
        }
    }

    pub fn put_box(&mut self, r: &Rect) {
        for y in r.top..=r.bottom {
            for x in r.left..=r.right {
                self.data[y as usize][x as usize] = Cell::Blank;
            }
        }
        // self.put_hor_line(r.left, r.top, r.right);
        // self.put_hor_line(r.left, r.bottom, r.right);
        // self.put_vert_line(r.left, r.top, r.bottom);
        // self.put_vert_line(r.right, r.top, r.bottom);
    }

    pub fn first(&self) -> Option<Vec2> {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if self.data[y as usize][x as usize].walkable() {
                    return Some(Vec2 { x, y });
                }
            }
        }
        None
    }

    pub fn last(&self) -> Option<Vec2> {
        for y in (0..GRID_SIZE).rev() {
            for x in (0..GRID_SIZE).rev() {
                if self.data[y as usize][x as usize].walkable() {
                    return Some(Vec2 { x, y });
                }
            }
        }
        None
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            for c in row.iter() {
                write!(f, "{}", c.as_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut grid = Grid::new();
    let g = dissector::Dissector::new(Rect {
        left: 1, top: 1, right: GRID_SIZE - 2, bottom: GRID_SIZE - 2,
    });
    let mut it = g.rooms(25);
    if let Some(r) = it.next() {
        grid.put_box(r);
        let mut prev = r;
        while let Some(r) = it.next() {
            let (px, py) = prev.center();
            let (qx, qy) = r.center();
            grid.put_hor_line(px, py, qx, Cell::Blank);
            grid.put_vert_line(qx, qy, py, Cell::Blank);
            grid.put_box(r);
            prev = r;
        }
    }


    let (f, t) = (grid.first().unwrap(), grid.last().unwrap());
    let (path, cost) = pf::pf_a_star(&grid.data, f, t);
    for i in path.iter() {
        grid.put(Cell::Path, i.x, i.y);
    }
    grid.put(Cell::Coin, f.x, f.y);
    grid.put(Cell::Coin, t.x, t.y);
    println!("{}", grid);
    assert_eq!(cost, pf::pf_dijkstra(&grid.data, f, t).1);
}
