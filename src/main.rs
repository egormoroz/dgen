extern crate rand;
pub mod dissector;
pub mod algs;
pub mod util;
pub mod grid;

use grid::*;
use util::Rect;

fn connect_rooms(grid: &mut Grid, p: &Rect, q: &Rect) {
   let (px, py) = p.center();
   let (qx, qy) = q.center();
   grid.put_hor_line(px, py, qx, Cell::Blank);
   grid.put_vert_line(qx, qy, py, Cell::Blank);
}

pub fn test() {
    let mut grid = Grid::new();
    let g = dissector::Dissector::new(Rect {
        left: 1, top: 1, right: GRID_SIZE - 2, bottom: GRID_SIZE - 2,
    });
    let mut it = g.rooms(GRID_SIZE as usize / 2);
    if let Some(r) = it.next() {
        grid.put_box(r);
        let mut prev = r;
        while let Some(r) = it.next() {
            connect_rooms(&mut grid, &prev, &r);
            grid.put_box(r);
            prev = r;
        }
    }

    println!("{}", grid);
}

extern crate piston_window;

use piston_window::*;

fn regen(grid: &mut Grid) {
    grid.clear();
    let g = dissector::Dissector::new(Rect {
        left: 1, top: 1, right: GRID_SIZE - 2, bottom: GRID_SIZE - 2,
    });
    let mut it = g.rooms(GRID_SIZE as usize / 2);
    if let Some(r) = it.next() {
        grid.put_box(r);
        let mut prev = r;
        while let Some(r) = it.next() {
            connect_rooms(grid, &prev, &r);
            grid.put_box(r);
            prev = r;
        }
    }

    let start = grid.first().unwrap();
    let (dpath, dcost) = algs::farthest(grid, start);
    let (apath, acost) = algs::pf_a_star(grid, start, dpath.first().unwrap().clone());
    for i in &dpath {
        grid.put(Cell::DPath, i.x, i.y);
    }
    for i in &apath {
        let c = match grid.get(i.x, i.y) {
            &Cell::DPath => Cell::BothPath,
            _ => Cell::APath,
        };
        grid.put(c, i.x, i.y);
    }
    println!("dcost: {}\tacost: {}", dcost, acost);
}

fn main() {
    const TILE_SIZE: f64 = 8.0;
    let d = (GRID_SIZE as f64 * TILE_SIZE) as u32;

    let mut grid = Grid::new();
    regen(&mut grid);

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [d, d])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        if let Some(btn) = event.press_args() {
            if btn == Button::Keyboard(Key::Space) {
                regen(&mut grid);
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            let (w, h) = grid.size();

            for y in 0..h {
                for x in 0..w {
                    let color = match grid.get(x, y) {
                        &Cell::Blank => [1.0; 4], //white
                        &Cell::Coin => [1.0, 1.0, 0.0, 1.0], //yellow
                        &Cell::DPath => [1.0, 0.0, 0.0, 1.0], //red
                        &Cell::APath => [0.0, 1.0, 0.0, 1.0], //green
                        &Cell::BothPath => [1.0, 1.0, 0.0, 1.0], //yellow
                        &Cell::Wall => [0.0, 0.0, 0.0, 1.0], //black
                    };
                    let (x, y) = (x as f64 * TILE_SIZE, y as f64 * TILE_SIZE);
                    rectangle(color, 
                        [x, y, x + TILE_SIZE, y + TILE_SIZE], 
                        context.transform, 
                        graphics);
                }
            }

            // rectangle(black,
                    //   [0.0, 0.0, 100.0, 100.0],
                    //   context.transform,
                    //   graphics);
        });
    }
}