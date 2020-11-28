use gl::types::GLint;
use nalgebra_glm::TMat4;

pub trait SetUniform {
    fn set_uniform(&self, uni_id: GLint);
}

impl SetUniform for f32 {
    fn set_uniform(&self, uni_id: GLint) {
        unsafe {
            gl::Uniform1f(uni_id, *self);
        }
    }
}

impl SetUniform for i32 {
    fn set_uniform(&self, uni_id: GLint) {
        unsafe {
            gl::Uniform1i(uni_id, *self);
        }
    }
}

impl SetUniform for [f32; 3] {
    fn set_uniform(&self, uni_id: GLint) {
        unsafe {
            gl::Uniform3f(
                uni_id,
                *self.get(0).unwrap(),
                *self.get(1).unwrap(),
                *self.get(2).unwrap(),
            );
        }
    }
}
impl SetUniform for [f32; 2] {
    fn set_uniform(&self, uni_id: GLint) {
        unsafe {
            gl::Uniform2f(
                uni_id,
                *self.get(0).unwrap(),
                *self.get(1).unwrap(),
            );
        }
    }
}

impl SetUniform for TMat4<f32> {
    fn set_uniform(&self, uni_id: GLint) {
        unsafe { gl::UniformMatrix4fv(uni_id, 1, gl::FALSE, self.as_ptr()) };
    }
}
impl SetUniform for &TMat4<f32> {
    fn set_uniform(&self, uni_id: GLint) {
        unsafe { gl::UniformMatrix4fv(uni_id, 1, gl::FALSE, self.as_ptr()) };
    }
}
impl SetUniform for &nalgebra_glm::TVec3<f32> {
    fn set_uniform(&self, uni_id: GLint) {
        unsafe {
            gl::Uniform3fv(uni_id, 1, self.as_ptr());
        }
    }
}
