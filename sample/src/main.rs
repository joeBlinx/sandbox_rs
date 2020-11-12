use engine::{
    camera, handle_event::HandleEvent, window,
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
use engine::render_info;

use engine::mesh::{Mesh, SkyBox};
use engine::render_info::RenderInfo;
use glish_rs::shader::Shader;
use std::collections::HashMap;
use engine::legion;
use engine::legion::{Schedule, Resources};
use engine::system::draw::*;
use engine::component::entity_render_info::EntityRenderInfo;

fn create_textures(world: &mut RenderInfo){
    world.add_textures("lava", Path::new("assets/lava.png"));
    world.add_textures("brick", Path::new("assets/normal_mapping/brickwall.jpg"));
    world.add_textures("brick_normal", Path::new("assets/normal_mapping/brickwall_normal.jpg"));
    world.add_cube_map("sky", Path::new("assets/skybox"));
}

fn main() {
    let window = window::Window::new((4, 5));
    let sdl = window.sdl();
    let mut event_pump = sdl.event_pump().unwrap();

    let plane = EntityRenderInfo{
        mesh: String::from("plane"),
        program: String::from("normal_map"),
        textures:{
            let mut textures= HashMap::new();
            textures.insert("color_map".to_owned(), "brick".to_owned());
            textures.insert("normal_map".to_owned(), "brick_normal".to_owned());
            textures
        }
    };


    let mut imgui = window.create_imgui();
    imgui.add_item(Rc::new(|ui| {
        ui.text(im_str!("Hello world!"));
    }));
    let mut display_gui = false;
    let mut debug_gui = DebugGui::default();
    debug_gui.create_gui(&mut imgui);

    let mut new_world = engine::world::World::default();
    new_world.use_render_info(|mut render|{
        create_textures(&mut render);
    });
    new_world.add_components((1, plane));
    new_world.add_components((1, camera::Camera::new(make_vec3(&[0.7, 1., 10.]), make_vec3(&[0., 0., 0.]))));
    'main: loop {
        window.clear();
        for event in event_pump.poll_iter() {
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
        new_world.run();
        if display_gui {
            imgui.render(&event_pump.mouse_state());
        }
        match debug_gui.get_obj_path_if_change() {
            Some(_path) => {}
            _ => {}
        }
        if debug_gui.use_normal_map() {

        } else {

        }
        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
        window.refresh();
    }
}
