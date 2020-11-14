use imgui::{im_str, Condition, Window};
use nalgebra_glm::make_vec3;
use std::path::Path;
extern crate gl;
extern crate rand;
extern crate sdl2;

use engine::component::camera;
use engine::component::camera::Camera;
use engine::component::entity_render_info::{EntityRenderInfo, RigidBody};
use engine::render_info::RenderInfo;
use std::collections::HashMap;
use engine::component::event::CloseEvent;
use engine::legion::{Read, IntoQuery};

fn create_textures(world: &mut RenderInfo) {
    world.add_textures("lava", Path::new("assets/lava.png"));
    world.add_textures("brick", Path::new("assets/normal_mapping/brickwall.jpg"));
    world.add_textures(
        "brick_normal",
        Path::new("assets/normal_mapping/brickwall_normal.jpg"),
    );
    world.add_cube_map("sky", Path::new("assets/skybox"));
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
    let mut left = -width as f32;
    let mut right = width as f32;
    let mut bottom = -height as f32;
    let mut top = height as f32;
    //let (mut imgui, mut imgui_sdl2, imgui_renderer) = window.create_imgui();
    let mut display_gui = false;

    let mut new_world = engine::world::World::new((4, 5), width, height);
    new_world.use_render_info(|mut render| {
        create_textures(&mut render);
    });
    new_world.add_components((1, plane,
    RigidBody{
        position:nalgebra_glm::make_vec3(&[0., 0., 0.]),
        rotation:nalgebra_glm::make_vec3(&[0., 0., 0.]),
        scale:nalgebra_glm::make_vec3(&[10., 10., 10.]),
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

    'main: loop {
        // for event in event_pump.poll_iter() {
        //     imgui_sdl2.handle_event(&mut imgui, &event);
        //     match event {
        //         sdl2::event::Event::Quit { .. } => break 'main,
        //         sdl2::event::Event::KeyDown { keycode, .. } => {
        //             let keycode = keycode.unwrap();
        //             match keycode {
        //                 sdl2::keyboard::Keycode::Escape => break 'main,
        //                 sdl2::keyboard::Keycode::F2 => display_gui = !display_gui,
        //                 _ => {}
        //             }
        //         }
        //         _ => {}
        //     }
        // }
        let mut query = Read::<CloseEvent>::query();
        for close in query.iter(new_world.legion_world()){
            if close.event {
                break 'main;
            }
        }
        new_world.run();
        // if display_gui {
        //     imgui_sdl2.prepare_frame(
        //         imgui.io_mut(),
        //         window.sdl_window(),
        //         &event_pump.mouse_state(),
        //     );
        //     let ui = imgui.frame();
        //     Window::new(im_str!("Hello world"))
        //         .size([300.0, 500.0], Condition::FirstUseEver)
        //         .build(&ui, || {
        //             ui.drag_float(im_str!("left"), &mut left).build();
        //             ui.drag_float(im_str!("right"), &mut right).build();
        //             ui.drag_float(im_str!("bottom"), &mut bottom).build();
        //             ui.drag_float(im_str!("top"), &mut top).build();
        //         });
        //     imgui_sdl2.prepare_render(&ui, window.sdl_window());
        //     imgui_renderer.render(ui);
        // }
        // let mut cam_components = new_world.entry(camera).unwrap();
        // let cam = cam_components.get_component_mut::<Camera>().unwrap();
        // cam.new_orthographic(left, right, bottom, top);
        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
    }
}
