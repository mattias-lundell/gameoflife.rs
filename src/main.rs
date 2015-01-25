extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;
extern crate quack;

use sdl2_window::Sdl2Window as Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL::_3_2;
use std::collections::HashSet;

use std::cell::RefCell;
use piston::{
    RenderArgs,
    UpdateArgs,
};

use graphics::*;

use event::{
    RenderEvent,
    UpdateEvent,
    WindowSettings,
    Events,
    Event,
    Ups,
    MaxFps,
};
use quack::Set;


pub static WINDOW_HEIGHT: i32 = 600;
pub static WINDOW_WIDTH: i32 = 600;
pub static BLOCKSIZE: f64 = 10.0;

pub static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(PartialEq, Eq, Hash, Clone, Show, Copy)]
pub struct Cell { x: i32, y: i32 }
impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        Cell { x: x, y: y }
    }

    pub fn render(&self,  ctx: &Context, gl: &mut Gl) {
        let x0 = self.x as f64 * BLOCKSIZE;
        let y0 = self.y as f64 * BLOCKSIZE;
        Rectangle::new(BLACK).draw([x0, y0, BLOCKSIZE, BLOCKSIZE], ctx, gl);
    }
}

type Grid = HashSet<Cell>;

pub struct World {
    grid: Grid
}
impl World {
    pub fn new() -> World {
        let mut grid: Grid = HashSet::new();

        let g = Cell::new(2, 1);
        let h = Cell::new(2, 2);
        let i = Cell::new(2, 3);

        grid.insert(g);
        grid.insert(h);
        grid.insert(i);

        World {grid: grid}
    }

    pub fn render(&mut self, ctx: &Context, gl: &mut Gl) -> () {
        for cell in self.grid.iter() {
            cell.render(ctx, gl)
        }
    }

    fn step(&mut self) -> () {
        let mut grid: Grid = HashSet::new();
        let neighbors =  self.neighbors();
        for cell in neighbors.iter() {
            let n_alive: i32 = self.count_neighbors(cell);
            // alive
            if self.grid.contains(cell) {
                if n_alive == 2 || n_alive == 3 {
                    grid.insert(cell.clone());
                } else {
                }
            }
            // dead
            else {
                if n_alive == 3 {
                    grid.insert(cell.clone());
                }
            }

        }
        self.grid = grid;
    }


    fn neighbors(&self) -> Grid {
        let mut neighbors: Grid = HashSet::new();
        for cell in self.grid.iter() {
            for &i in [-1, 0, 1].iter() {
                for &j in [-1, 0, 1].iter() {
                    neighbors.insert(Cell::new(cell.x + i, cell.y + j));
                }
            }
        }
        neighbors
    }

    fn count_neighbors(&self, cell: &Cell) -> i32 {
        let mut n = 0;
        for &i in [-1, 0, 1].iter() {
            for &j in [-1, 0, 1].iter() {
                if !(i == 0 && j == 0) {
                    if self.grid.contains(&Cell::new(cell.x + i, cell.y + j)) {
                        n += 1;
                    }
                }
            }
        }
        n
    }

}

pub struct App {
    gl: Gl,
    world: World
}
impl App {
    fn new() -> App {
        App { gl: Gl::new(_3_2), world: World::new() }
    }

    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        let w = args.width as f64;
        let h = args.height as f64;

        let ctx = Context::abs(w, h);
        graphics::clear(WHITE, &mut self.gl);

        self.world.render(&ctx, &mut self.gl);
    }

    fn update(&mut self, _: &mut Window) {
        self.world.step();
    }
}

fn main() {
    let window = Window::new(
        _3_2,
        piston::WindowSettings::default());

    let mut app = App::new();

    let window = RefCell::new(window);

    for e in event::events(&window).set(Ups(2)).set(MaxFps(60)) {
        e.render(|args| {
            app.render(&mut *window.borrow_mut(), args);
        });
        e.update(|_| {
            app.update(&mut *window.borrow_mut());
        });
    }
}
