use imgui::{im_str, Condition, Window};
use nalgebra_glm::make_vec3;
use std::path::Path;
extern crate gl;
extern crate rand;
extern crate sdl2;
use engine::component::camera;
use engine::component::entity_render_info::{EntityRenderInfo, RigidBody};
use engine::render_info::RenderInfo;
use std::collections::HashMap;
use engine::component::event::{
    CloseEvent
};
use engine::legion::{Read, IntoQuery, Schedule};
use engine::system::{
    draw::*,
    event::*
};
use legion::system;

fn create_textures(world: &mut RenderInfo) {
    world.add_textures("lava", Path::new("assets/lava.png"));
    world.add_textures("brick", Path::new("assets/normal_mapping/brickwall.jpg"));
    world.add_textures(
        "brick_normal",
        Path::new("assets/normal_mapping/brickwall_normal.jpg"),
    );
    world.add_cube_map("sky", Path::new("assets/skybox"));
}

#[system(for_each)]
pub fn disable_imgui(imgui_info : &mut engine::component::imgui::ImGuiInfo, #[resource]event: &sdl2::event::Event){
    match event{
        sdl2::event::Event::KeyDown { keycode, ..} =>{
            match keycode.unwrap(){
                sdl2::keyboard::Keycode::F2 =>{
                    imgui_info.display = !imgui_info.display;
                },
                _=>{}
            }
        },
        _ => {}
    }
}


#[system(for_each)]
pub fn imgui_draw(imgui_info: &mut engine::component::imgui::ImGuiInfo,
#[resource] window: &mut engine::Window){
    if imgui_info.display {
        imgui_info.imgui_sdl2.prepare_frame(
            imgui_info.context.io_mut(),
            window.sdl_window(),
            &window.sdl().event_pump().unwrap().mouse_state(),
        );
        let ui = imgui_info.context.frame();
        let mut left = 2.0;
        let mut right = 2.0;
        let mut bottom = 2.0;
        let mut top = 2.0;
        Window::new(im_str!("Hello world"))
            .size([300.0, 500.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.drag_float(im_str!("left"), &mut left).build();
                ui.drag_float(im_str!("right"), &mut right).build();
                ui.drag_float(im_str!("bottom"), &mut bottom).build();
                ui.drag_float(im_str!("top"), &mut top).build();
            });
        imgui_info.imgui_sdl2.prepare_render(&ui, window.sdl_window());
        imgui_info.renderer.render(ui);
    }
}

fn main() {
    let width = 1366;
    let height = 768;

    let plane = EntityRenderInfo {
        mesh: String::from("plane"),
        program: String::from("normal_map"),
        textures: {
            let mut textures = HashMap::new();
            textures.insert("color_map".to_owned(), "brick".to_owned());
            textures.insert("normal_map".to_owned(), "brick_normal".to_owned());
            textures
        },
    };
    let left = -width as f32;
    let right = width as f32;
    let bottom = -height as f32;
    let top = height as f32;

    let mut new_world = engine::world::World::new(
        (4, 5), width, height,
        Schedule::builder()
            .add_system(draw_entity_system())
            .add_system(update_camera_system())
            .add_system(imgui_draw_system())
            .build(),
        Schedule::builder()
            .add_system(quit_event_system())
            .add_system(imgui_event_system())
            .add_system(disable_imgui_system())
            .build());
    new_world.use_render_info(|mut render| {
        create_textures(&mut render);
    });
    new_world.add_imgui();
    new_world.add_components((1, plane,
    RigidBody{
        position:nalgebra_glm::make_vec3(&[0., 0., 0.]),
        rotation:nalgebra_glm::make_vec3(&[0., 0., 0.]),
        scale:nalgebra_glm::make_vec3(&[100., 100., 1.]),
    }));
    let _camera = new_world.add_components((
        camera::Camera::create_orthographic(
            make_vec3(&[0.7, 1., 10.]),
            make_vec3(&[0., 0., 0.]),
            left,
            right,
            bottom,
            top,
        ),
    ));
    let sprite_sheet = engine::reader_json::sprite_sheet::read_sprite_sheet(Path::new("assets/Sprite-0001.json"));
    'main: loop {

        let mut query = Read::<CloseEvent>::query();
        for close in query.iter(new_world.ecs_world()){
            if close.event {
                break 'main;
            }
        }
        new_world.run();
        // let mut cam_components = new_world.entry(camera).unwrap();
        // let cam = cam_components.get_component_mut::<Camera>().unwrap();
        // cam.new_orthographic(left, right, bottom, top);
        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
    }
}
