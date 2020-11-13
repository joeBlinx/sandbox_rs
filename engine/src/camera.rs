use crate::handle_event::HandleEvent;
use nalgebra_glm;
use sdl2::{event::Event, keyboard::Keycode};
use std::f32::consts::PI;

pub struct Camera {
    position: nalgebra_glm::Vec3,
    center: nalgebra_glm::Vec3,
    proj: nalgebra_glm::Mat4,
}

impl Camera {
    pub fn create_perspective(position: nalgebra_glm::Vec3, center: nalgebra_glm::Vec3, aspect:f32) -> Camera{
        Camera {
            position,
            center,
            proj: nalgebra_glm::perspective(aspect, 3.14 / 4.0, 0.1, 1000.),
        }
    }
    pub fn create_orthographic(position: nalgebra_glm::Vec3, center: nalgebra_glm::Vec3 ) -> Camera{
        Camera {
            position,
            center,
            proj: nalgebra_glm::perspective(aspect, 3.14 / 4.0, 0.1, 1000.),
        }
    }
    pub fn new(position: nalgebra_glm::Vec3, center: nalgebra_glm::Vec3) -> Camera {
        Camera {
            position,
            center,
            proj: nalgebra_glm::perspective(16. / 9., 3.14 / 4.0, 0.1, 1000.),
        }
    }
    pub fn get_view(&self) -> nalgebra_glm::Mat4 {
        nalgebra_glm::look_at(
            &self.position,
            &self.center,
            &nalgebra_glm::make_vec3(&[0., 1., 0.]),
        )
    }
    pub fn get_proj(&self) -> &nalgebra_glm::Mat4 {
        &self.proj
    }
    pub fn get_position(&self) -> &nalgebra_glm::Vec3 {
        &self.position
    }
    pub fn move_xyz(&mut self, xyz: &[f32]) {
        self.position += nalgebra_glm::make_vec3(xyz);
        self.center += nalgebra_glm::make_vec3(xyz);
    }
    pub fn move_sphere(&mut self, rho: f32, teta: f32, phi: f32) {
        let (cur_rho, cur_teta, cur_phi) = carthesian_to_spherical(&self.position);
        let new_rho = cur_rho + rho;
        let teta = cur_teta + teta;
        let phi = cur_phi + phi;
        self.position = spherical_to_carthesian(new_rho, teta, phi);
    }
}

impl HandleEvent for Camera {
    fn handle_event(&mut self, event: &Event) {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. } => {
                let keycode = keycode.unwrap();
                let teta = PI / 180.;
                let phi = PI / 180.;
                let delta = 0.5;
                match keycode {
                    Keycode::Q => {
                        self.move_sphere(0., 0., phi);
                    }
                    Keycode::D => {
                        self.move_sphere(0., 0., -phi);
                    }
                    Keycode::Z => {
                        self.move_sphere(0., teta, 0.);
                    }
                    Keycode::S => {
                        self.move_sphere(0., -teta, 0.);
                    }
                    Keycode::Left => {
                        self.move_xyz(&[-delta, 0., 0.]);
                    }
                    Keycode::Right => {
                        self.move_xyz(&[delta, 0., 0.]);
                    }
                    Keycode::Up => {
                        self.move_xyz(&[0., delta, 0.]);
                    }
                    Keycode::Down => {
                        self.move_xyz(&[0., -delta, 0.]);
                    }
                    _ => {}
                }
            }
            sdl2::event::Event::MouseWheel { y, .. } => {
                self.move_sphere((y / y.abs()) as f32, 0., 0.);
            }
            _ => {}
        }
    }
}

fn spherical_to_carthesian(rho: f32, teta: f32, phi: f32) -> nalgebra_glm::Vec3 {
    nalgebra_glm::make_vec3(&[
        rho * teta.sin() * phi.sin(),
        rho * teta.cos(),
        rho * teta.sin() * phi.cos(),
    ])
}

fn carthesian_to_spherical(xyz: &nalgebra_glm::Vec3) -> (f32, f32, f32) {
    let rho = nalgebra_glm::l2_norm(xyz);
    let teta = (xyz.y / rho).acos();
    let phi = {
        if xyz.x >= 0. {
            (xyz.z / (xyz.z * xyz.z + xyz.x * xyz.x).sqrt()).acos()
        } else {
            2. * PI - (xyz.z / (xyz.z * xyz.z + xyz.x * xyz.x).sqrt()).acos()
        }
    };
    (rho, teta, phi)
}
