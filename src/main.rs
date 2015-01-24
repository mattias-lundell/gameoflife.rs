extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;

use sdl2_window::Sdl2Window as Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL::_3_2;
use std::collections::HashSet;

use std::cell::RefCell;
use piston::{
    RenderArgs,
    UpdateArgs
};

use graphics::*;

use event::{
    RenderEvent,
    UpdateEvent,
};

pub static WINDOW_HEIGHT: i32 = 600;
pub static WINDOW_WIDTH: i32 = 600;
pub static BLOCKSIZE: f64 = 10.0;

pub static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

#[derive(PartialEq, Eq, Hash, Clone, Show)]
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
        World {grid: grid}
    }

    pub fn render(&mut self, ctx: &Context, gl: &mut Gl) -> () {
        for cell in self.grid.iter() {
            cell.render(ctx, gl)
        }
    }

    fn step(&mut self) -> () {
        let mut grid: Grid = HashSet::new();
        let neighbors =  self.neighbors(&self.grid);
        for cell in neighbors.iter() {
            // alive
            if self.grid.contains(cell) {
                let n_alive: i32 = self.count_neighbors(&self.grid, cell);
                if n_alive == 2 || n_alive == 3 {
                    grid.insert(cell.clone());
                }
            }
            // dead
            else {
                let n_alive: i32 = self.count_neighbors(&self.grid, cell);
                if n_alive == 3 {
                    grid.insert(cell.clone());
                }
            }

        }
        self.grid = grid;
    }


    fn neighbors(&self, grid: &Grid) -> Grid {
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

    fn count_neighbors(&self, grid: &Grid, cell: &Cell) -> i32 {
        let mut n = 0;
        for &i in [-1, 0, 1].iter() {
            for &j in [-1, 0, 1].iter() {
                if i != 0 && j != 0 {
                    if grid.contains(&Cell::new(cell.x + i, cell.y + j)) {
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
    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        let w = args.width as f64;
        let h = args.height as f64;

        let ctx = Context::abs(w, h);
        graphics::clear(WHITE, &mut self.gl);

        let f = Cell::new(0, 0);
        let g = Cell::new(1, 1);
        let h = Cell::new(2, 2);
        let i = Cell::new(3, 3);

        self.world.grid.insert(f);
        self.world.grid.insert(g);
        self.world.grid.insert(h);
        self.world.grid.insert(i);
        self.world.step();
        self.world.render(&ctx, &mut self.gl);
    }

    fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
    }
}

fn main() {
    let window = Window::new(
        _3_2,
        piston::WindowSettings::default());

    let mut app = App { gl: Gl::new(_3_2), world: World::new() };

    let window = RefCell::new(window);
    for e in event::events(&window) {
        if let Some(r) = e.render_args() {
            app.render(&mut *window.borrow_mut(), &r);
        }
        if let Some(u) = e.update_args() {
            app.update(&mut *window.borrow_mut(), &u);
        }
    }
}
