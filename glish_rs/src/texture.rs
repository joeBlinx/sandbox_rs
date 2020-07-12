use gl;
use gl::types::*;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use std::convert::TryInto;
use std::path::{Path, PathBuf};
pub struct PathCubeMaps {
    pub x: PathBuf,
    pub x_neg: PathBuf,
    pub y: PathBuf,
    pub y_neg: PathBuf,
    pub z: PathBuf,
    pub z_neg: PathBuf,
}
pub struct Texture {
    id: GLuint,
    target: GLenum,
}

impl Texture {
    pub fn texture_2d_from_file(path_to_file: &Path) -> Texture {
        let target = gl::TEXTURE_2D;
        let texture = Texture {
            id: create_texture_id(),
            target: gl::TEXTURE_2D,
        };
        texture.bind();
        unsafe {
            tex_image_2d_from_file(path_to_file, target, gl::RGBA);
            gl::TexParameteri(
                target,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST.try_into().unwrap(),
            );

            gl::TexParameteri(
                target,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST.try_into().unwrap(),
            );
        }
        texture
    }
    pub fn texture_3d_from_files(files_path: PathCubeMaps) -> Texture {
        let texture = Texture {
            id: create_texture_id(),
            target: gl::TEXTURE_CUBE_MAP,
        };
        texture.bind();
        let array_of_path = [
            files_path.x,
            files_path.x_neg,
            files_path.y,
            files_path.y_neg,
            files_path.z,
            files_path.z_neg,
        ];
        let mut i = 0;
        for path in array_of_path.iter() {
            tex_image_2d_from_file(path, gl::TEXTURE_CUBE_MAP_POSITIVE_X + i, gl::RGB);
            i += 1;
        }
        unsafe {
            gl::TexParameteri(
                texture.target,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR.try_into().unwrap(),
            );
            gl::TexParameteri(
                texture.target,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR.try_into().unwrap(),
            );
            gl::TexParameteri(
                texture.target,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE.try_into().unwrap(),
            );
            gl::TexParameteri(
                texture.target,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE.try_into().unwrap(),
            );
            gl::TexParameteri(
                texture.target,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE.try_into().unwrap(),
            );
        }

        return texture;
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.target, self.id);
        }
    }
    pub fn active(&self, number: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + number);
            gl::BindTexture(self.target, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

fn tex_image_2d_from_file(file_path: &Path, target: GLenum, format: GLenum) {
    let sdl2_surface = Surface::from_file(file_path).unwrap();
    unsafe {
        gl::TexImage2D(
            target,
            0,
            format.try_into().unwrap(),
            sdl2_surface.width().try_into().unwrap(),
            sdl2_surface.height().try_into().unwrap(),
            0,
            format.try_into().unwrap(),
            gl::UNSIGNED_BYTE,
            (*sdl2_surface.raw()).pixels,
        );
    }
}

fn create_texture_id() -> GLuint {
    let mut id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
    }
    id
}
