extern crate shader_version;
extern crate input;
extern crate event;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{ Gl,Texture };
use sdl2_window::Sdl2Window;
use image::GenericImage;
use input::{ Button, MouseButton };
use std::rand::distributions::{IndependentSample, Range};
use std::rand;

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
    let (width, height) = (300, 300);
    let window = Sdl2Window::new(
        opengl,
        event::WindowSettings {
            title: "gameoflife.rs".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut image = image::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture = Texture::from_image(&image);
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);

    let mut rng = rand::thread_rng();
    let xs = Range::new(0u32, width);
    let ys = Range::new(0u32, height);

    for n in range(0u32, 1000) {
        let x = xs.ind_sample(&mut rng);
        let y = ys.ind_sample(&mut rng);
        image.put_pixel(x, y, image::Rgba([0,0,0,255]));
        texture.update(&image);
    }

    for e in event::events(&window) {
        gl.draw([0, 0, width as i32, height as i32], |c, gl| {
            graphics::clear([1.0; 4], gl);
            graphics::image(&texture, &c, gl);
        });

    }
}
