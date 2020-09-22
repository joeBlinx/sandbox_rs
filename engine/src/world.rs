use crate::traits::Draw;
use crate::camera::Camera;
use nalgebra_glm::make_vec3;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::handle_event::HandleEvent;

pub struct World{
    drawable: Vec<Rc<RefCell<dyn Draw>>>,
    cam: Camera
}


impl World{
    pub fn register_drawable<T:Draw + 'static>(&mut self, drawable: Rc<RefCell<T>>) -> Weak<RefCell<T>>{
        let weak = Rc::downgrade(&drawable);
        self.drawable.push(drawable);
        weak

    }
    pub fn do_the_thing(&self){
        for drawable in self.drawable.iter(){
            (*drawable).borrow().draw(&self.cam);
        }
    }
    pub fn handle_event(&mut self, event: &sdl2::event::Event){
        self.cam.handle_event(event);
    }
}
impl Default for World{
    fn default() -> Self{
        World{
            drawable: Vec::new(),
            cam: Camera::new(make_vec3(&[0.7, 1., 10.]), make_vec3(&[0., 0., 0.]))
        }
    }
}