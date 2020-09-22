use std::rc::Rc;
use std::path::Path;
use imgui::im_str;
use engine::imgui_wrap::ImguiWrap;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};

pub struct DebugGui {
    obj_arrays: Vec<imgui::ImString>,
    choose: Rc<RefCell<i32>>,
    old_one: i32,
    normal_map: Rc<RefCell<bool>>,
}
impl Default for DebugGui {
    fn default() -> Self {
        DebugGui {
            obj_arrays: get_all_obj(),
            choose: Rc::new(RefCell::new(0)),
            old_one: 0,
            normal_map: Rc::new(RefCell::new(false)),
        }
    }
}
impl DebugGui {
    pub fn create_gui(&self, imgui: &mut ImguiWrap) {
        let copy = self.obj_arrays.clone();
        let copy_choose = Rc::clone(&self.choose);
        let copy_normal_map = Rc::clone(&self.normal_map);
        imgui.add_item(Rc::new(move |ui: &imgui::Ui| {
            {
                let mut obj_ref_arrays = Vec::new();
                for it in copy.iter() {
                    obj_ref_arrays.push(it);
                }
                ui.list_box(
                    im_str!("Hello"),
                    copy_choose.deref().borrow_mut().deref_mut(),
                    &obj_ref_arrays,
                    obj_ref_arrays.len() as i32,
                );
                ui.checkbox(
                    im_str!("Use normal_map"),
                    copy_normal_map.deref().borrow_mut().deref_mut(),
                )

            };
        }));
    }
    pub fn get_obj_path_if_change(&mut self) -> Option<&Path> {
        if *self.choose.borrow() != self.old_one {
            self.old_one = *self.choose.borrow();
            Some(&Path::new(self.obj_arrays[*self.choose.borrow() as usize].to_str()))
        } else {
            None
        }
    }
    pub fn get_obj_path(&self) -> &Path {
        &Path::new(self.obj_arrays[self.old_one as usize].to_str())
    }
    pub fn use_normal_map(&self) -> bool {
        *self.normal_map.borrow()
    }
}
fn get_all_obj<'a>() -> Vec<imgui::ImString> {
    let assets_path = Path::new("assets/obj");
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