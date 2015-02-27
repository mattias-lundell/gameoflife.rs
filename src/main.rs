extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;
extern crate quack;

use std::str::FromStr;
use std::io::BufferedReader;
use std::io::File;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL::_3_2;
use std::collections::HashSet;
use std::collections::HashMap;
use std::os;
use std::cell::RefCell;
use piston::{
    RenderArgs,
};
use graphics::*;
use event::{
    RenderEvent,
    UpdateEvent,
    Ups,
    MaxFps,
};
use quack::Set;
use CC::{Node, Leaf};
use std::num::Int;
use std::rc::Rc;
use std::hash::Hash;

#[derive(Show)]
enum CC {
    Node(i32, bool, i32, Rc<CC>, Rc<CC>, Rc<CC>, Rc<CC>),
    Leaf(bool),
}

fn leaf(alive: bool) -> Rc<CC> {
    Rc::new(Leaf(alive))
}

fn node(nw: Rc<CC>, ne: Rc<CC>, sw: Rc<CC>, se: Rc<CC>) -> Rc<CC> {
    let level = nw.level();
    let population = nw.population() + ne.population() + sw.population() + se.population();
    let alive = population > 0;
    Rc::new(Node(level + 1, alive, population,
                 nw.clone(), ne.clone(), sw.clone(), se.clone()))
}

impl CC {
    fn render(&self, ctx: &Context, gl: &mut Gl, size: f64, left: f64, top: f64) -> () {
        match *self {
            Leaf(true) => {
                Rectangle::new(BLACK).draw([top, left, size, size], ctx, gl);
            },
            Leaf(false) => (),
            Node(_, true, _, ref nw, ref ne, ref sw, ref se) => {
                let _size = size / 2.0;
                nw.render(ctx, gl, _size, left, top);
                ne.render(ctx, gl, _size, left + _size, top);
                sw.render(ctx, gl, _size, left, top + _size);
                se.render(ctx, gl, _size, left + _size, top + _size);
            },
            Node(_, false, _, _, _, _, _) => ()
        }
    }

    fn population(&self) -> i32 {
        match *self {
            Leaf(alive) => alive as i32,
            Node(_, _, population, _, _, _, _) => population
        }
    }
    fn level(&self) -> i32 {
        match *self {
            Leaf(_) => 0,
            Node(level, _, _, _, _, _, _) => level
        }
    }
    fn nw(&self) -> Rc<CC> {
        match *self {
            Node(_, _, _, ref nw, _, _, _) => nw.clone(),
            Leaf(alive) => leaf(alive)
        }
    }
    fn ne(&self) -> Rc<CC> {
        match *self {
            Node(_, _, _, _, ref ne, _, _) => ne.clone(),
            Leaf(alive) => leaf(alive)
        }
    }
    fn sw(&self) -> Rc<CC> {
        match *self {
            Node(_, _, _, _, _, ref sw, _) => sw.clone(),
            Leaf(alive) => leaf(alive)
        }
    }
    fn se(&self) -> Rc<CC> {
        match *self {
            Node(_, _, _, _, _, _, ref se) => se.clone(),
            Leaf(alive) => leaf(alive)
        }
    }
    fn set_bit(&self, x: i32, y: i32) -> Rc<CC> {
        match *self {
            Leaf(_) => leaf(true),
            Node(level, _, _, ref nw, ref ne, ref sw, ref se) => {
                let mut offset: i32;
                if level == 1 {
                    offset = 0;
                } else {
                    offset = 1 << (level - 2);
                }
                if x < 0 {
                    if y < 0 {
                        node(nw.clone().set_bit(x + offset, y + offset), ne.clone(),
                             sw.clone(), se.clone())
                    } else {
                        node(nw.clone(), ne.clone(),
                             sw.clone().set_bit(x + offset, y - offset), se.clone())
                    }
                } else {
                    if y < 0 {
                        node(nw.clone(), ne.clone().set_bit(x - offset, y + offset),
                             sw.clone(), se.clone())
                    } else {
                            node(nw.clone(), ne.clone(),
                                 sw.clone(), se.clone().set_bit(x - offset, y - offset))
                    }
                }
            }
        }
    }
    fn get_bit(&self, x: i32, y: i32) -> u32 {
        match *self {
            Leaf(alive) => alive as u32,
            Node(_, false, _, _, _, _, _) => 0,
            Node(level, _, _, ref nw, ref ne, ref sw, ref se) => {
                let mut offset: i32;
                if level == 1 {
                    offset = 0;
                } else {
                    offset = 1 << (level - 2);
                }
                if x < 0 {
                    if y < 0 {
                        nw.clone().get_bit(x + offset, y + offset)
                    } else {
                        sw.clone().get_bit(x + offset, y - offset)
                    }
                } else {
                    if y < 0 {
                        ne.clone().get_bit(x - offset, y + offset)
                    } else {
                        se.clone().get_bit(x - offset, y - offset)
                    }
                }
            }
        }
    }
    fn expand_universe(&self) -> Rc<CC> {
        match *self {
            Leaf(alive) => {
                let l = leaf(alive);
                node(node(leaf(false), leaf(false),
                          leaf(false), l.clone()),
                     node(leaf(false), leaf(false),
                          l.clone(), leaf(false)),
                     node(leaf(false), l.clone(),
                          leaf(false), leaf(false)),
                     node(l.clone(), leaf(false),
                          leaf(false), leaf(false)))
            },
            Node(level, _, _, ref nw, ref ne, ref sw, ref se) => {
                let border = empty_tree(level-1);
                node(node(border.clone(), border.clone(),
                          border.clone(), nw.clone()),
                     node(border.clone(), border.clone(),
                          ne.clone(), border.clone()),
                     node(border.clone(), sw.clone(),
                          border.clone(), border.clone()),
                     node(se.clone(), border.clone(),
                          border.clone(), border.clone()))
            }
        }
    }
}

#[test]
fn neighborcount_test() {
    assert_eq!(((0x151)&(0x757)).count_ones(), 4);
    assert_eq!(((0x020)&(0x757)).count_ones(), 0);
    assert_eq!(((0x1)&(0x757)).count_ones(), 1);
}

#[test]
fn one_gen_test() {
    assert_eq!((*one_gen(0x700)).population(), 1);
    assert_eq!((*one_gen(0x777)).population(), 0);
    assert_eq!((*one_gen(0x151)).population(), 0);
    assert_eq!((*one_gen(0x101)).population(), 0);
    assert_eq!((*one_gen(0x121)).population(), 1);
}

fn empty_tree(level: i32) -> Rc<CC> {
    if level == 1 {
        let l = leaf(false);
        node(l.clone(), l.clone(), l.clone(), l.clone())
    } else {
        let l = empty_tree(level - 1);
        node(l.clone(), l.clone(), l.clone(), l.clone())
    }
}

struct World {
    ngen: u64,
    root: Rc<CC>,
    bitcount: HashMap<u32, bool>,
}

impl World {
    fn new(grid: Grid, bitcount: HashMap<u32, bool>) -> World {
        let mut tree = empty_tree(9);
        for cell in grid.iter() {
            tree = tree.set_bit(cell.x, cell.y);
        }
        World { root: tree, ngen: 0, bitcount: bitcount.clone()}
    }

    fn render(&self, ctx: &Context, gl: &mut Gl, size: f64, left: f64, top: f64) -> () {
        self.root.render(ctx, gl, size, left, top);
    }
    fn get_bit(&self, x: i32, y: i32) -> u32 {
        self.root.get_bit(x, y)
    }
    fn set_bit(&self, x: i32, y: i32) -> World {
        let mut _root = self.root.clone();
        loop {
            let level: i32 = _root.level();
            let max_coordinate: i32 = 1 << (level);
            if (-max_coordinate <= x && x <= max_coordinate-1 &&
                -max_coordinate <= y && y <= max_coordinate-1) {
                break;
            } else {
                _root = _root.expand_universe();
            }
        }
        _root = _root.set_bit(x, y);
        World { root: _root, ngen: self.ngen, bitcount: self.bitcount.clone() }
    }
    fn step(&self) -> World {
        let mut _root = self.root.clone();
//        let level: i32 = _root.level();
        loop {
  //           if ((_root.level() >= 3) ||
  //               (_root.nw().population() == _root.nw().se().se().population()) ||
  //               (_root.ne().population() == _root.ne().sw().sw().population()) ||
  //               (_root.sw().population() == _root.sw().ne().ne().population()) ||
  //               (_root.se().population() == _root.se().nw().nw().population())) {
  //               break;
            if _root.level() > 9 {
                break;
            } else {
                _root = _root.expand_universe();
            }
        }

        _root = self.next_generation(_root);
        let next = self.ngen + 1;
        World { root: _root.clone(), ngen: next, bitcount: self.bitcount.clone()}
    }

    fn slow_simulation(&self, root: Rc<CC>) -> Rc<CC> {
        let mut allbits: u32 = 0;
        for y in range(-2i32, 2) {
            for x in range(-2i32, 2) {
                allbits = (allbits << 1) | root.get_bit(x, y);
            }
        }
        node(self.one_gen(allbits >> 5), self.one_gen(allbits >> 4),
             self.one_gen(allbits >> 1), self.one_gen(allbits))
    }

    fn next_generation(&self, root: Rc<CC>) -> Rc<CC> {
        if root.population() == 0 {
            root.nw()
        } else if root.level() == 2 {
            self.slow_simulation(root)
        } else {
            let n00 = node(root.nw().nw().se(), root.nw().ne().sw(),
                           root.nw().sw().ne(), root.nw().se().nw());
            let n01 = node(root.nw().ne().se(), root.ne().nw().sw(),
                           root.nw().se().ne(), root.ne().sw().nw());
            let n02 = node(root.ne().nw().se(), root.ne().ne().sw(),
                           root.ne().sw().ne(), root.ne().se().nw());
            let n10 = node(root.nw().sw().se(), root.nw().se().sw(),
                           root.sw().nw().ne(), root.sw().ne().nw());
            let n11 = node(root.nw().se().se(), root.ne().sw().sw(),
                           root.sw().ne().ne(), root.se().nw().nw());
            let n12 = node(root.ne().sw().se(), root.ne().se().sw(),
                           root.se().nw().ne(), root.se().ne().nw());
            let n20 = node(root.sw().nw().se(), root.sw().ne().sw(),
                           root.sw().sw().ne(), root.sw().se().nw());
            let n21 = node(root.sw().ne().se(), root.se().nw().sw(),
                           root.sw().se().ne(), root.se().sw().nw());
            let n22 = node(root.se().nw().se(), root.se().ne().sw(),
                           root.se().sw().ne(), root.se().se().nw());

            let nw = node(n00.clone(), n01.clone(),
                          n10.clone(), n11.clone());
            let ne = node(n01.clone(), n02.clone(),
                          n11.clone(), n12.clone());
            let sw = node(n10.clone(), n11.clone(),
                          n20.clone(), n21.clone());
            let se = node(n11.clone(), n12.clone(),
                          n21.clone(), n22.clone());

            node(self.next_generation(nw),
                 self.next_generation(ne),
                 self.next_generation(sw),
                 self.next_generation(se))
        }
    }

    // Advance the center of a 3x3 square one generation
    // lookup instead of evaluate
    fn one_gen(&self, mask: u32) -> Rc<CC> {
        match self.bitcount.get(&(mask & 0x777)) {
            Some(&alive) => {
                leaf(alive)
            },
            None => {
                panic!("{:x}", mask)
            }
        }
    }
}

pub static WINDOW_HEIGHT: i32 = 1000;
pub static WINDOW_WIDTH: i32 = 1000;
pub static BLOCKSIZE: f64 = 1.0;

pub static BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub static WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

 #[derive(PartialEq, Eq, Hash, Clone, Show, Copy)]
pub struct Cell { x: i32, y: i32 }

pub type Grid = HashSet<Cell>;

pub struct App {
    gl: Gl,
    world: World
}
impl App {
    fn new(grid: Grid) -> App {
        let mut bitcount: HashMap<u32, bool> = HashMap::new();
        for i in range(0x0, 0x778) {
            if i == 0 {
                bitcount.insert(i, false);
            } else {
                let ones: u32 = (i & 0x757).count_ones() as u32;
                let center: u32 = (i >> 5) & 1;
                if ones == 3 || (ones == 2 && center == 1) {
                    bitcount.insert(i, true);
                } else {
                    bitcount.insert(i, false);
                }
            }
        }

        App { gl: Gl::new(_3_2), world: World::new(grid, bitcount) }
    }

    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        let w = 1024.0; //args.width as f64;
        let h = 1024.0; //args.height as f64;
        let zoom: f64 = 2.0;
        let translate: f64 = -1.0/zoom;
        let ctx = Context::abs(w, h).trans(translate*w, translate*h).zoom(zoom);
        graphics::clear(WHITE, &mut self.gl);
        println!("population {} gen {}", self.world.root.population(), self.world.ngen);
        self.world.render(&ctx, &mut self.gl, w, 0.0, 0.0);
    }

    fn update(&mut self, _: &mut Window) {
        let _world = self.world.step();
        self.world = _world;
    }
}

fn read_life_file(path: String) -> Grid {
    let mut grid: Grid = HashSet::new();

    let filepath = Path::new(path);
    let mut file = BufferedReader::new(File::open(&filepath));
    let mut lines = file.lines();
    lines.next();

    for line in lines {
        match line {
            Ok(l) => {
                let row: Vec<i32> = l.trim().split(' ').filter_map(FromStr::from_str).collect::<Vec<i32>>();
                let x = row[0];
                let y = row[1];
                grid.insert(Cell {x: x, y: y});
            }
            Err(error) => print!("{}", error.desc)
        }
    }
    grid
}

fn main() {
    let infile: String = os::args()[1].clone();
    if infile == "BENCH" {
        bench_steps(os::args()[2].clone());
    } else {
        let grid: Grid = read_life_file(infile);
        let window = Window::new(
            _3_2,
            piston::WindowSettings {
                title: "Piston".to_string(),
                size: [1024, 1024],
                samples: 0,
                fullscreen: false,
                exit_on_esc: true,
            });

        let mut app = App::new(grid);

        let window = RefCell::new(window);

        for e in event::events(&window).set(Ups(10)).set(MaxFps(60)) {
            e.render(|args| {
                app.render(&mut *window.borrow_mut(), args);
            });
            e.update(|_| {
                app.update(&mut *window.borrow_mut());
            });
        }
    }
}

fn bench_steps(infile: String) {
    let mut bitcount: HashMap<u32, bool> = HashMap::new();
    for i in range(0x0, 0x778) {
        if i == 0 {
            bitcount.insert(i, false);
        } else {
            let ones: u32 = (i & 0x757).count_ones() as u32;
            let center: u32 = (i >> 5) & 1;
            if ones == 3 || (ones == 2 && center == 1) {
                bitcount.insert(i, true);
            } else {
                bitcount.insert(i, false);
            }
        }
    }

    let grid: Grid = read_life_file(infile);
    let mut app = World::new(grid, bitcount);
    for _ in range(0, 100) {
        app = app.step();
    }
}
