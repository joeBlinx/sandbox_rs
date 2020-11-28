use crate::component::camera::Camera;
use crate::component::entity_render_info::Animation;
use crate::component::entity_render_info::{EntityRenderInfo, RigidBody};
use crate::mesh::SkyBox;
use crate::render_info::RenderInfo;
use glish_rs::utils;
use legion::component;
use legion::system;

fn draw(render_information: &EntityRenderInfo, render_info: &RenderInfo) {
    let mesh = render_info.meshs.get(&render_information.mesh).unwrap();
    mesh.vao.bind();
    let opengl_prog = render_info
        .programs
        .get(&render_information.program)
        .unwrap();
    opengl_prog.set_used();

    let mut i = 0 as u32;
    for (key, texture_name) in render_information.textures.iter() {
        opengl_prog.set_uni(&key[..], i as i32);
        let texture = render_info.textures.get(texture_name).unwrap();
        texture.active(i);
        i += 1;
    }

    utils::draw_elements(mesh.number_indices);
}

#[system(for_each)]
#[filter(!component::<SkyBox>())]
pub fn draw_entity(render_information: &EntityRenderInfo, rigid_body: &RigidBody, #[resource] world_manager: &RenderInfo) {
    let opengl_prog = world_manager
        .programs
        .get(&render_information.program)
        .unwrap();
    opengl_prog.set_uni("model", rigid_body.model_matrix());
    draw(render_information, world_manager);
}

#[system(for_each)]
pub fn draw_skybox(
    render_information: &EntityRenderInfo,
    _: &SkyBox,
    #[resource] world_manager: &RenderInfo,
) {
    unsafe {
        gl::DepthFunc(gl::LEQUAL);
    }
    draw(render_information, world_manager);
    unsafe {
        gl::DepthFunc(gl::LESS);
    }
}
#[system(for_each)]
pub fn update_camera(cam: &Camera, #[resource] world_manager: &RenderInfo) {
    for (_, program) in world_manager.programs.iter() {
        program.set_used();
        program.set_uni("pos_cam", cam.get_position());
        program.set_uni("proj", cam.get_proj());
        program.set_uni("view", cam.get_view());
    }
}
#[system(for_each)]
pub fn draw_animation(render_information: &EntityRenderInfo, rigid_body: &RigidBody, animation: &Animation, #[resource] world_manager: &RenderInfo) {
    let opengl_prog = world_manager
        .programs
        .get(&render_information.program)
        .unwrap();
    opengl_prog.set_uni("model", rigid_body.model_matrix());
    let sprite = &world_manager.sprite_sheets.get(&animation.name).unwrap().sprites[animation.frame as usize];

    opengl_prog.set_uni("uv_orig", &sprite.frame.0);
    opengl_prog.set_uni("uv_size", &sprite.frame.1);
    draw(render_information, world_manager);
}
