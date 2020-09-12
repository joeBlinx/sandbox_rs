use crate::shader::Shader;
use crate::uniform::SetUniform;
use crate::utils;
use gl;
use gl::types::{GLint, GLuint};
use std::collections::HashMap;
use std::{cell::RefCell, ffi::CString};
pub struct Program {
    id: GLuint,
    unis: RefCell<HashMap<String, GLint>>,
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
            unis: RefCell::new(HashMap::new()),
        })
    }
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    fn get_uni<T: Into<Vec<u8>>>(&self, uni_name: T) -> Option<GLint> {
        let uni_gl = CString::new(uni_name).unwrap();
        let uni_string = uni_gl.to_string_lossy().into_owned();
        let uni_id: Option<GLint>;
        let uni = self.unis.borrow().get(&uni_string).cloned();
        match uni {
            None => {
                let id = unsafe { gl::GetUniformLocation(self.id, uni_gl.as_ptr()) };
                let _error = unsafe { gl::GetError() };
                if id != -1 {
                    self.unis.borrow_mut().insert(uni_string, id);
                    uni_id = Some(id);
                } else {
                    eprintln!("This name {:?} does not refer to a uniform name", &uni_gl);
                    uni_id = None;
                }
            }
            Some(value) => {
                uni_id = Some(value);
            }
        }
        return uni_id;
    }

    pub fn set_uni<T: Into<Vec<u8>>, U: SetUniform>(&self, uni_name: T, value: U) {
        self.set_used();
        let uni_id = self.get_uni(uni_name);
        match uni_id {
            Some(id) => value.set_uniform(id),
            _ => {}
        }
    }
}
impl Default for Program {
    fn default() -> Self {
        Program {
            id: 0,
            unis: RefCell::new(HashMap::new()),
        }
    }
}
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
