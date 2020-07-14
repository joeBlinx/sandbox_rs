mod camera;
mod handle_event;
mod load;
mod sample_3d;
mod skybox;
mod traits;
mod window;
use crate::handle_event::HandleEvent;
use crate::sample_3d::Sample3d;
use crate::traits::Draw;
use nalgebra_glm::make_vec3;
use std::path::Path;

extern crate gl;
extern crate rand;
extern crate sdl2;

fn main() {
    let window = window::Window::new((3, 3));
    let sdl = window.sdl();
    let mut event = sdl.event_pump().unwrap();
    let mut cam = camera::Camera::new(make_vec3(&[0.7, 1., -10.]), make_vec3(&[0., 0., 0.]));

    let mut sample_3d = Sample3d::new(Path::new("assets/sphere.obj"), Path::new("assets/lava.png"));
    let mut skybox = skybox::Skybox::new(Path::new("assets/skybox"));
    'main: loop {
        window.clear();
        for event in event.poll_iter() {
            cam.handle_event(&event);
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    let keycode = keycode.unwrap();
                    match keycode {
                        sdl2::keyboard::Keycode::Escape => break 'main,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        sample_3d.draw(&cam);
        skybox.draw(&cam);

        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
        window.refresh();
    }
}
