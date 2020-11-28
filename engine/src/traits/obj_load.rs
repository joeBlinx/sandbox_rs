use crate::component::camera::Camera;
pub trait Normals {
    fn get_normals(&self) -> &[f32; 3];
}
pub trait Position {
    fn get_position(&self) -> &[f32; 3];
}
pub trait TextCoords {
    fn get_tex_coords(&self) -> &[f32; 3];
}
pub trait Draw {
    fn draw(&self, cam: &Camera);
}

impl Normals for obj::Vertex {
    fn get_normals(&self) -> &[f32; 3] {
        return &self.normal;
    }
}
impl Position for obj::Vertex {
    fn get_position(&self) -> &[f32; 3] {
        &self.position
    }
}
impl Normals for obj::TexturedVertex {
    fn get_normals(&self) -> &[f32; 3] {
        return &self.normal;
    }
}
impl Position for obj::TexturedVertex {
    fn get_position(&self) -> &[f32; 3] {
        &self.position
    }
}
impl TextCoords for obj::TexturedVertex {
    fn get_tex_coords(&self) -> &[f32; 3] {
        &self.texture
    }
}
