#![feature(get_mut_unchecked)]
use engine::{
    camera, handle_event::HandleEvent, imgui_wrap::ImguiWrap, sample_3d::Sample3d, skybox,
    traits::Draw, window,
};
use imgui::im_str;
use nalgebra_glm::make_vec3;
use std::path::Path;
use std::rc::Rc;
extern crate gl;
extern crate rand;
extern crate sdl2;
fn get_all_obj<'a>() -> Vec<imgui::ImString> {
    let assets_path = Path::new("assets");
    let mut obj_arrays = std::vec::Vec::new();
    for it in std::fs::read_dir(assets_path).unwrap() {
        let file = it.unwrap().path();
        if let Some(ext) = file.extension() {
            if ext == "obj" {
                let file = file.to_str().unwrap();
                let file = String::from(file);
                obj_arrays.push(imgui::ImString::from(file));
            }
        }
    }

    obj_arrays
}
struct DebugGui {
    obj_arrays: Vec<imgui::ImString>,
    choose: Rc<i32>,
    old_one: i32,
}
impl Default for DebugGui {
    fn default() -> Self {
        DebugGui {
            obj_arrays: get_all_obj(),
            choose: Rc::new(0),
            old_one: 0,
        }
    }
}
impl DebugGui {
    fn create_gui(&self, imgui: &mut ImguiWrap) {
        let copy = self.obj_arrays.clone();
        let mut copy_choose = Rc::clone(&self.choose);
        imgui.add_item(Rc::new(move |ui: &imgui::Ui| {
            {
                let mut obj_ref_arrays = Vec::new();
                for it in copy.iter() {
                    obj_ref_arrays.push(it);
                }
                unsafe {
                    ui.list_box(
                        im_str!("Hello"),
                        Rc::get_mut_unchecked(&mut copy_choose),
                        &obj_ref_arrays,
                        obj_ref_arrays.len() as i32,
                    );
                }
            };
        }));
    }
    fn get_obj_path_if_change(&mut self) -> Option<&Path> {
        if *self.choose != self.old_one {
            self.old_one = *self.choose;
            Some(&Path::new(self.obj_arrays[*self.choose as usize].to_str()))
        } else {
            None
        }
    }
    fn get_obj_path(&self) -> &Path {
        &Path::new(self.obj_arrays[self.old_one as usize].to_str())
    }
}

fn main() {
    let window = window::Window::new((3, 3));
    let sdl = window.sdl();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut cam = camera::Camera::new(make_vec3(&[0.7, 1., 10.]), make_vec3(&[0., 0., 0.]));

    let mut skybox = skybox::Skybox::new(Path::new("assets/skybox"));
    //Imgui creation
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
    let mut sample_3d = Sample3d::new(debug_gui.get_obj_path(), Path::new("assets/lava.png"));
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
        if display_gui {
            imgui.render(&event_pump.mouse_state());
        }
        match debug_gui.get_obj_path_if_change() {
            Some(path) => sample_3d = Sample3d::new(path, Path::new("assets/lava.png")),
            _ => {}
        }
        let ten_millis = std::time::Duration::from_millis(17);
        std::thread::sleep(ten_millis);
        window.refresh();
    }
}
