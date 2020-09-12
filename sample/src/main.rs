#![feature(get_mut_unchecked)]
use engine::{
    camera, handle_event::HandleEvent, sample::sample_3d::Sample3d, skybox,
    traits::Draw, window,
};
use std::rc::Rc;
use imgui::im_str;
use nalgebra_glm::make_vec3;
use std::path::Path;
extern crate gl;
extern crate rand;
extern crate sdl2;
mod debug_gui;
use debug_gui::DebugGui;
use engine::world;

fn main() {
    let window = window::Window::new((3, 3));
    let sdl = window.sdl();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut cam = camera::Camera::new(make_vec3(&[0.7, 1., 10.]), make_vec3(&[0., 0., 0.]));
    let mut world = world::World::default();
    let skybox = skybox::Skybox::new(Path::new("assets/skybox")).unwrap();
    //Imgui creation
    let mut plane = Box::new({
        let mut plane = Sample3d::create_plane();
        plane.add_shader(
            gl::VERTEX_SHADER,
            Path::new("assets/shader/vertex/triangle.vert"),
        );
        plane.add_shader(
            gl::FRAGMENT_SHADER,
            Path::new("assets/shader/fragment/triangle.frag"),
        );
        plane.add_texture(
            "color_map",
            Path::new("assets/normal_mapping/brickwall.jpg"),
        );
        plane.add_texture(
            "normal_map",
            Path::new("assets/normal_mapping/brickwall_normal.jpg"),
        );
        plane
    });
    world.add_drawable(plane);
    let mut imgui = window.create_imgui();
    imgui.add_item(Rc::new(|ui| {
        ui.text(im_str!("Hello world!"));
    }));
    let _old_one = 0;
    //
    let _choose = 0;
    let mut display_gui = false;
    let mut debug_gui = DebugGui::default();
    debug_gui.create_gui(&mut imgui);
    let mut sample_3d = Sample3d::from_obj_file(debug_gui.get_obj_path()).unwrap();
    sample_3d.add_texture("color_map", Path::new("assets/lava.png"));
    sample_3d.add_shader(
        gl::VERTEX_SHADER,
        &Path::new("assets/shader/vertex/triangle.vert"),
    );
    sample_3d.add_shader(
        gl::FRAGMENT_SHADER,
        &Path::new("assets/shader/fragment/triangle.frag"),
    );
    'main: loop {
        window.clear();
        for event in event_pump.poll_iter() {
            cam.handle_event(&event);
            imgui.handle_event(&event);
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    let keycode = keycode.unwrap();
                    match keycode {
                        sdl2::keyboard::Keycode::Escape => break 'main,
                        sdl2::keyboard::Keycode::F2 => display_gui = !display_gui,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        sample_3d.draw(&cam);
        world.do_the_thing();
        skybox.draw(&cam);
        if display_gui {
            imgui.render(&event_pump.mouse_state());
        }
        match debug_gui.get_obj_path_if_change() {
            Some(path) => {
                let _ = sample_3d.add_obj_file(path);
            }
            _ => {}
        }
        // if debug_gui.use_normal_map() {
        //     plane.add_shader(
        //         gl::FRAGMENT_SHADER,
        //         &Path::new("assets/shader/fragment/normal_mapping.frag"),
        //     );
        // } else {
        //     plane.add_shader(
        //         gl::FRAGMENT_SHADER,
        //         &Path::new("assets/shader/fragment/triangle.frag"),
        //     );
        // }
        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
        window.refresh();
    }
}
