use crate::utils;
use gl;
use gl::types::{GLenum, GLint, GLuint};
use std::ffi::{CStr, CString};
use std::fs::read_to_string;
use std::path::Path;
use std::rc::Rc;

fn shader_from_source(source: &CStr, shader_type: GLuint) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    };

    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    };
    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        };

        let error = utils::create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        };
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(id)
}
pub struct ShaderSettings<'a> {
    pub stage: GLenum,
    pub path: &'a Path,
}
pub struct Shader {
    id: GLuint,
    counter: Rc<i32>,
}

impl Shader {
    pub fn from_file(path: &Path, shader_type: GLenum) -> Result<Shader, String> {
        let source = read_to_string(path).unwrap();
        let source_cstring = unsafe { CString::from_vec_unchecked(source.into_bytes()) };

        Self::from_source(&source_cstring, shader_type)
    }
    pub fn from_source(source: &CStr, shader_type: GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, shader_type)?;
        Ok(Shader {
            id,
            counter: Rc::new(1),
        })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
    pub fn from_vert_file(file_path: &Path) -> Result<Shader, String> {
        Shader::from_file(file_path, gl::VERTEX_SHADER)
    }

    pub fn from_frag_file(file_path: &Path) -> Result<Shader, String> {
        Shader::from_file(file_path, gl::FRAGMENT_SHADER)
    }
}

impl Clone for Shader {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            counter: Rc::clone(&self.counter),
        }
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        if Rc::strong_count(&self.counter) == 1 {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }
}
