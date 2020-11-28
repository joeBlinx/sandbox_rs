use glish_rs::uniform::SetUniform;
use crate::resources::sprite_sheet::{Size, Position};
use gl::types::GLint;
impl SetUniform for Position{
    fn set_uniform(&self, uni_id: GLint) {
        [self.x, self.y].set_uniform(uni_id);
    }
}
impl SetUniform for Size{
    fn set_uniform(&self, uni_id: GLint) {
        [self.w, self.h].set_uniform(uni_id);
    }
}
impl SetUniform for &Position{
    fn set_uniform(&self, uni_id: GLint) {
        [self.x, self.y].set_uniform(uni_id);
    }
}
impl SetUniform for &Size{
    fn set_uniform(&self, uni_id: GLint) {
        [self.w, self.h].set_uniform(uni_id);
    }
}