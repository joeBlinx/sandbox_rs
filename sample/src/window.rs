extern crate gl;
extern crate sdl2;
use gl::types::*;
use std::ffi::{c_void, CStr};

pub(crate) struct Window {
    sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
}
extern "system" fn debug_callback(
    _source: GLenum,
    _gltype: GLenum,
    _id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    let string = unsafe { CStr::from_ptr(message) };
    println!("{:?}", string);
    if severity == gl::DEBUG_SEVERITY_HIGH {
        panic!("Unrecovable error");
    }
}
impl Window {
    pub(crate) fn new() -> Window {
        let sdl = sdl2::init().expect("Error while init sdl2");
        let video_subsystem = sdl.video().expect("Error while init sdl video");
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);
        let window = video_subsystem
            .window("Tartes aux poireaux", 1366, 768)
            .opengl()
            .resizable()
            .build()
            .expect("Error while creating window");

        let gl_context = window.gl_create_context().expect("Error while init OpenGL");
        let _gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                std::ptr::null(),
                gl::TRUE,
            )
        }
        Window {
            sdl,
            _video_subsystem: video_subsystem,
            window,
            _gl_context: gl_context,
        }
    }

    pub(crate) fn sdl(&self) -> &sdl2::Sdl {
        &self.sdl
    }

    pub(crate) fn refresh(&self) {
        self.window.gl_swap_window();
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
