use gl::types::*;

pub struct Vbo {
    id: GLuint,
    target: GLenum,
}
impl Vbo {
    pub fn create(vertices: Vec<f32>) -> Vbo {
        Vbo::from(vertices)
    }
    pub fn create_elements(indices: Vec<u16>) -> Vbo {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, // target
                (indices.len() * std::mem::size_of::<u16>()) as gl::types::GLsizeiptr, // size of data in bytes
                indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                              // usage
            );
        }
        Vbo {
            id: vbo,
            target: gl::ELEMENT_ARRAY_BUFFER,
        }
    }
    pub fn id(&self) -> GLuint {
        self.id
    }
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }
}
impl From<Vec<f32>> for Vbo {
    fn from(vertices: Vec<f32>) -> Self {
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                       // target
                (vertices.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                               // usage
            );
        }
        Vbo {
            id: vbo,
            target: gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}
impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id()].as_ptr());
        }
    }
}
pub struct VboSettings {
    pub location: u32,
    pub size: i32,
    pub stride: i32,
    pub offset: i32,
}
impl Default for VboSettings {
    fn default() -> Self {
        VboSettings {
            location: 0,
            size: 0,
            stride: 0,
            offset: 0,
        }
    }
}
trait VboSettingsSize: Sized {}
impl VboSettingsSize for VboSettings {}
pub struct Vao {
    id: GLuint,
    _vbos: Vec<Vbo>,
    vbo_indices: Vbo,
}

impl Vao {
    pub fn create(vbo_indices: Vbo, vbo: Vbo, vbo_settings: &[VboSettings]) -> Vao {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.id());
            for vbo_setting in vbo_settings {
                gl::EnableVertexAttribArray(vbo_setting.location); // this is "layout (location = 0)" in vertex shader
                gl::VertexAttribPointer(
                    vbo_setting.location, // index of the generic vertex attribute ("layout (location = 0)")
                    vbo_setting.size,     // the number of components per generic vertex attribute
                    gl::FLOAT,            // data type
                    gl::FALSE,            // normalized (int-to-float conversion)
                    vbo_setting.stride * std::mem::size_of::<f32>() as i32, // stride (byte offset between consecutive attributes)
                    (vbo_setting.offset * std::mem::size_of::<f32>() as i32)
                        as *const std::ffi::c_void, // offset of the first component
                );
            }
        }
        Vao {
            id: vao,
            _vbos: vec![vbo],
            vbo_indices,
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            self.vbo_indices.bind();
        }
    }
}
impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}
