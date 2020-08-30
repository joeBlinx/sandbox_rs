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
    pub fn texture_2d_from_file(path_to_file: &Path) -> Result<Texture, String> {
        let target = gl::TEXTURE_2D;
        let texture = Texture {
            id: create_texture_id(),
            target: gl::TEXTURE_2D,
        };
        texture.bind();
        let format;
        if let Some(extension) = path_to_file.extension() {
            let extension = extension.to_str().unwrap();
            match extension {
                "jpg" => format = gl::RGB,
                "png" => format = gl::RGBA,
                _ => {
                    return Err(format!(
                        "Extension {} not handle find in path {:?}",
                        extension, path_to_file
                    ));
                }
            }
        } else {
            return Err(format!("No extension find in path {:?}", path_to_file));
        }
        unsafe {
            tex_image_2d_from_file(path_to_file, target, format)?;
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
        Ok(texture)
    }
    pub fn texture_3d_from_files(files_path: PathCubeMaps) -> Result<Texture, String> {
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
            tex_image_2d_from_file(path, gl::TEXTURE_CUBE_MAP_POSITIVE_X + i, gl::RGB)?;
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

        return Ok(texture);
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

fn tex_image_2d_from_file(
    file_path: &Path,
    target: GLenum,
    format: GLenum,
) -> Result<bool, String> {
    let sdl2_surface = Surface::from_file(file_path);
    match sdl2_surface {
        Ok(surface) => {
            unsafe {
                gl::TexImage2D(
                    target,
                    0,
                    format.try_into().unwrap(),
                    surface.width().try_into().unwrap(),
                    surface.height().try_into().unwrap(),
                    0,
                    format.try_into().unwrap(),
                    gl::UNSIGNED_BYTE,
                    (*surface.raw()).pixels,
                );
            };
            Ok(true)
        }
        Err(err) => Err(err),
    }
}

fn create_texture_id() -> GLuint {
    let mut id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
    }
    id
}
