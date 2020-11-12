use engine::{
    camera, handle_event::HandleEvent, sample::sample_3d::Sample3d,
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
use std::cell::RefCell;
use engine::mesh::{Mesh, SkyBox};
use engine::world::WorldManager;
use glish_rs::shader::Shader;
use engine::sample::sample_3d::RenderInfo;
use std::collections::HashMap;
use engine::legion;
use engine::legion::{Schedule, Resources};
use engine::system::draw::*;

fn create_mesh(world: &mut WorldManager){
    let cube = Mesh::from_obj_file(Path::new("assets/obj/cube.obj")).unwrap();
    let sphere = Mesh::from_obj_file(Path::new("assets/obj/sphere.obj")).unwrap();
    let susan = Mesh::from_obj_file(Path::new("assets/obj/susan.obj")).unwrap();

    world.add_mesh("cube", cube);
    world.add_mesh("sphere", sphere);
    world.add_mesh("susan", susan);
    world.add_mesh("plane", Mesh::create_plane());
}

fn create_textures(world: &mut WorldManager){
    world.add_textures("lava", Path::new("assets/lava.png"));
    world.add_textures("brick", Path::new("assets/normal_mapping/brickwall.jpg"));
    world.add_textures("brick_normal", Path::new("assets/normal_mapping/brickwall_normal.jpg"));
    world.add_cube_map("sky", Path::new("assets/skybox"));
}
fn create_program(world: &mut WorldManager){
    let shaders_classic=[
        Shader::from_vert_file(Path::new("assets/shader/vertex/triangle.vert")).unwrap(),
        Shader::from_frag_file(Path::new("assets/shader/fragment/triangle.frag")).unwrap()
    ];
    let shaders_with_normal=[
        Shader::from_vert_file(Path::new("assets/shader/vertex/triangle.vert")).unwrap(),
        Shader::from_frag_file(Path::new("assets/shader/fragment/normal_mapping.frag")).unwrap()
    ];

    let skybox_shaders = [
        Shader::from_vert_file(Path::new("assets/shader/vertex/skybox.vert")).unwrap(),
        Shader::from_frag_file(Path::new("assets/shader/fragment/skybox.frag")).unwrap()
    ];

    world.add_program_from_shaders("classic", &shaders_classic);
    world.add_program_from_shaders("normal_map", &shaders_with_normal);
    world.add_program_from_shaders("skybox", &skybox_shaders);
}
fn main() {
    let window = window::Window::new((4, 5));
    let sdl = window.sdl();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut cam = camera::Camera::new(make_vec3(&[0.7, 1., 10.]), make_vec3(&[0., 0., 0.]));
    let mut world = world::WorldManager::default();

    create_mesh(&mut world);
    create_textures(&mut world);
    create_program(&mut world);

    let main_object = RenderInfo {
        mesh: String::from("cube"),
        program: String::from("classic"),
        textures:{
            let mut textures= HashMap::new();
            textures.insert("color_map".to_owned(), "lava".to_owned());
            textures
        }
    };
    let plane = RenderInfo{
        mesh: String::from("plane"),
        program: String::from("normal_map"),
        textures:{
            let mut textures= HashMap::new();
            textures.insert("color_map".to_owned(), "brick".to_owned());
            textures.insert("normal_map".to_owned(), "brick_normal".to_owned());
            textures
        }
    };

    let skybox = RenderInfo{
        mesh: String::from("cube"),
        program: String::from("skybox"),
        textures:{
            let mut textures = HashMap::new();
            textures.insert(String::from("cubemap"), String::from("sky"));
            textures
        }
    };
    let mut world_legion = legion::World::default();
    world_legion.push((SkyBox, skybox));
    world_legion.push((1, plane));
    // world_legion.push((2, main_object));
    world_legion.push((1, camera::Camera::new(make_vec3(&[0.7, 1., 10.]), make_vec3(&[0., 0., 0.]))));
    let mut schedule = Schedule::builder()
        .add_system(draw_entity_system())
        .add_system(update_camera_system())
        .add_system(draw_skybox_system())
        .build();

    let mut resources = Resources::default();
    resources.insert(world);
    //Imgui creation

    let mut imgui = window.create_imgui();
    imgui.add_item(Rc::new(|ui| {
        ui.text(im_str!("Hello world!"));
    }));
    let mut display_gui = false;
    let mut debug_gui = DebugGui::default();
    debug_gui.create_gui(&mut imgui);

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
        schedule.execute(&mut world_legion, &mut resources);
        if display_gui {
            imgui.render(&event_pump.mouse_state());
        }
        match debug_gui.get_obj_path_if_change() {
            Some(path) => {}
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
