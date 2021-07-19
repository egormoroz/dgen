extern crate rand;
pub mod dissector;
pub mod algs;
pub mod util;
pub mod grid;

use glutin_window::GlutinWindow;
use grid::*;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::{Button, EventSettings, Events, Key, PressEvent, RenderEvent, WindowSettings};
use util::Rect;

fn connect_rooms(grid: &mut Grid, p: &Rect, q: &Rect) {
   let (px, py) = p.center();
   let (qx, qy) = q.center();
   grid.put_hor_line(px, py, qx, Cell::Blank);
   grid.put_vert_line(qx, qy, py, Cell::Blank);
}

extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;


fn regen(grid: &mut Grid) {
    grid.clear();
    let g = dissector::Dissector::new(Rect {
        left: 1, top: 1, right: GRID_SIZE - 2, bottom: GRID_SIZE - 2,
    });
    let mut it = g.rooms((GRID_SIZE as f32 * 0.4) as usize);
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
    const TILE_SIZE: [f64; 2] = [12.0; 2];
    let (width, height) = (GRID_SIZE as f64 * TILE_SIZE[0], GRID_SIZE as f64 * TILE_SIZE[1]);
    let window_size = [width.ceil() as u32, height.ceil() as u32];
    let mut grid = Grid::new();
    regen(&mut grid);


    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow =
        WindowSettings::new("Hello Piston!", window_size)
        .exit_on_esc(true).build().unwrap();
    let mut gl = GlGraphics::new(opengl);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/consola.ttf", (), texture_settings)
        .expect("failed to load font");

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(btn) = event.press_args() {
            if btn == Button::Keyboard(Key::Space) {
                regen(&mut grid);
            }
        }

        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |c, g| {
                use graphics::*;
                clear([0.0, 0.0, 0.0, 1.0], g);
                let (w, h) = grid.size();

                for y in 0..h {
                    for x in 0..w {
                        let cell = grid.get(x, y);
                        let color = match cell {
                            &Cell::Blank => [0.2, 0.2, 0.2, 1.0], //gray
                            &Cell::Coin => [1.0, 1.0, 0.0, 1.0], //yellow
                            &Cell::DPath => [1.0, 0.0, 0.0, 1.0], //red
                            &Cell::APath => [0.0, 1.0, 0.0, 1.0], //green
                            &Cell::BothPath => [1.0, 1.0, 0.0, 1.0], //yellow
                            &Cell::Wall => [1.0; 4], //white
                        };

                        let [w, h] = TILE_SIZE;
                        let character = glyphs.character(16, cell.as_char()).unwrap();
                        let (x, y) = (x as f64 * w, y as f64 * h);

                        image::Image::new_color(color).src_rect([
                            character.atlas_offset[0], character.atlas_offset[1],
                            character.atlas_size[0], character.atlas_size[1]])
                        .rect([x, y, w, h])
                        .draw(
                            character.texture,
                            &c.draw_state,
                            c.transform,
                            g
                        );
                    }
                }
            });
        }
    }
}