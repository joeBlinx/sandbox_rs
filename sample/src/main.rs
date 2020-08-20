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
use imgui::{Window, Condition, im_str};

extern crate gl;
extern crate rand;
extern crate sdl2;


fn main() {
    let window = window::Window::new((3, 3));
    let sdl = window.sdl();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut cam = camera::Camera::new(make_vec3(&[0.7, 1., -10.]), make_vec3(&[0., 0., 0.]));

    let mut sample_3d = Sample3d::new(Path::new("assets/sphere.obj"), Path::new("assets/lava.png"));
    let mut skybox = skybox::Skybox::new(Path::new("assets/skybox"));
    //Imgui creation
    let (mut imgui, mut imgui_sdl2, renderer) = window.create_imgui();
   
    //
    let mut choose = 0;
    'main: loop {
        window.clear();
        for event in event_pump.poll_iter() {
            cam.handle_event(&event);
            imgui_sdl2.handle_event(&mut imgui, &event);
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
        imgui_sdl2.prepare_frame(imgui.io_mut(), window.window_sdl2(), &event_pump.mouse_state());

        let ui = imgui.frame();
        Window::new(im_str!("Hello world"))
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.list_box(im_str!("Hello"), &mut choose, &[
                    &im_str!("pouet"), &im_str!("prout")], 2);
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
        sample_3d.draw(&cam);
        skybox.draw(&cam);

        imgui_sdl2.prepare_render(&ui, window.window_sdl2());
        renderer.render(ui);
        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
        window.refresh();
    }
}
