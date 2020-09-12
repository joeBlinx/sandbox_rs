use crate::traits::Draw;
use crate::camera::Camera;
use nalgebra_glm::make_vec3;
pub struct World{
    drawable: Vec<Box<dyn Draw>>,
    cam: Camera
}


impl World{
    pub fn add_drawable(&mut self, drawable: Box<dyn Draw>) {
        self.drawable.push(drawable);
    }
    pub fn do_the_thing(&self){
        for drawable in self.drawable.iter(){
            drawable.draw(&self.cam);
        }
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