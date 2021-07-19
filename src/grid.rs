use super::util::*;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Cell {
    Blank,
    Wall,
    Coin,
    DPath,
    APath,
    BothPath,
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
            Cell::DPath => '-',
            Cell::APath => '_',
            Cell::BothPath => '+',
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

    pub fn clear(&mut self) {
        for row in self.data.iter_mut() {
            for i in row.iter_mut() {
                *i = Cell::Wall;
            }
        }
    }

    pub fn walkable(&self, x: u16, y: u16) -> bool {
        self.data[y as usize][x as usize].walkable()
    }

    pub fn size(&self) -> (u16, u16) {
        (GRID_SIZE, GRID_SIZE)
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        assert!(self.contains(x, y));
        &self.data[y as usize][x as usize]
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
