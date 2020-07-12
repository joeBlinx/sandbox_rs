use crate::shader::Shader;
use crate::uniform::SetUniform;
use crate::utils;
use gl;
use gl::types::{GLint, GLuint};
use std::collections::HashMap;
use std::ffi::CString;
pub struct Program {
    id: GLuint,
    unis: HashMap<String, GLint>,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = utils::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program {
            id: program_id,
            unis: HashMap::new(),
        })
    }
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    fn get_uni<T: Into<Vec<u8>>>(&mut self, uni_name: T) -> GLint {
        let uni_gl = CString::new(uni_name).unwrap();
        let uni_string = uni_gl.to_string_lossy().into_owned();
        let uni = self.unis.get(&uni_string);
        let uni_id: GLint;
        match uni {
            None => {
                let id = unsafe { gl::GetUniformLocation(self.id, uni_gl.as_ptr()) };
                let _error = unsafe { gl::GetError() };
                self.unis.insert(uni_string, id);
                uni_id = id;
            }
            Some(value) => {
                uni_id = *value;
            }
        }
        return uni_id;
    }

    pub fn set_uni<T: Into<Vec<u8>>, U: SetUniform>(&mut self, uni_name: T, value: U) {
        self.set_used();
        let uni_id = self.get_uni(uni_name);
        value.set_uniform(uni_id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
