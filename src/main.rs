extern crate rand;
pub mod dissector;
use std::fmt;

use dissector::Rect;

#[derive(Clone, Copy)]
pub enum Cell {
    Blank,
    Wall,
    Coin,
}

impl Cell {
    fn as_char(&self) -> char {
        match self {
            Cell::Blank => '.',
            Cell::Wall => 'X',
            Cell::Coin => 'o',
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
    let mut it = g.rooms();
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
    println!("{}", grid);
}
