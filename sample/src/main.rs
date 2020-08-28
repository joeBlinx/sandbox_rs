use nalgebra_glm::make_vec3;
use std::path::Path;
use imgui::{Window, Condition, im_str};
use engine::{camera, 
    window, skybox, 
    sample_3d::Sample3d, 
    handle_event::HandleEvent, 
    traits::Draw,
    imgui_wrap::ImguiWrap
};
extern crate gl;
extern crate rand;
extern crate sdl2;

fn get_all_obj<'a>() -> Vec<imgui::ImString>{
    let assets_path = Path::new("assets");
    let mut obj_arrays = std::vec::Vec::new();
    for it in std::fs::read_dir(assets_path).unwrap() {
        let file = it.unwrap().path();
        if let Some(ext) = file.extension(){
            if ext == "obj" {
                let file = file.to_str().unwrap();
                let file = String::from(file);
                obj_arrays.push(imgui::ImString::from(file));
            }
        }
    }
   
    obj_arrays
}

fn create_imgui(choose: &mut i32, obj_arrays: &Vec<imgui::ImString>, 
    imgui: &mut ImguiWrap,
    event_pump :&sdl2::EventPump
) {
    let mut obj_ref_arrays = Vec::new();
    for it in obj_arrays.iter() {
        obj_ref_arrays.push(it);
    }
    imgui.render(&event_pump.mouse_state(), |ui|{
            Window::new(im_str!("Hello world"))
            .size([300.0, 500.0], Condition::FirstUseEver)
            .build(&ui, || 
                {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.list_box(im_str!("Hello"), choose, 
                &obj_ref_arrays[..], obj_ref_arrays.len() as i32);
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
                });
            });
}
fn main() {
    let window = window::Window::new((3, 3));
    let sdl = window.sdl();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut cam = camera::Camera::new(make_vec3(&[0.7, 1., -10.]), make_vec3(&[0., 0., 0.]));

    let mut skybox = skybox::Skybox::new(Path::new("assets/skybox"));
    //Imgui creation
    let mut imgui = window.create_imgui();
    let mut old_one = 0;
    //
    let mut choose = 0;
    let mut display_gui = false;
   
    let obj_arrays = get_all_obj();
    let mut sample_3d = Sample3d::new(Path::new(obj_arrays.first().unwrap().to_str()), Path::new("assets/lava.png"));
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
        skybox.draw(&cam);
        if display_gui{
           create_imgui(&mut choose, &obj_arrays, &mut imgui, &event_pump);
        }
        if old_one != choose {
            sample_3d = Sample3d::new(Path::new(obj_arrays[choose as usize].to_str()), Path::new("assets/lava.png"));
            old_one = choose;

        }
        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
        window.refresh();
    }
}
