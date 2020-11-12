use crate::sample::sample_3d::EntityRenderInfo;
use glish_rs::utils;
use crate::render_manager::RenderInfo;
use legion::{system};
use crate::camera::Camera;
use legion::component;
use crate::mesh::SkyBox;

fn draw(render_information: &EntityRenderInfo, world_manager: &RenderInfo){
    let mesh = world_manager.meshs.get(&render_information.mesh).unwrap();
    mesh.vao.bind();
    let opengl_prog = world_manager.programs.get(&render_information.program).unwrap();
    opengl_prog.set_used();

    let mut i = 0 as u32;
    for (key, texture_name) in render_information.textures.iter() {
        opengl_prog.set_uni(&key[..], i as i32);
        let texture = world_manager.textures.get(texture_name).unwrap();
        texture.active(i);
        i += 1;
    }

    utils::draw_elements(mesh.number_indices);
}

#[system(for_each)]
#[filter(!component::<SkyBox>())]
pub fn draw_entity(render_information:&EntityRenderInfo,
                   #[resource] world_manager: &RenderInfo
               ){
   draw(render_information, world_manager);
}

#[system(for_each)]
pub fn draw_skybox(render_information:&EntityRenderInfo, _: &SkyBox,
                   #[resource] world_manager: &RenderInfo
){
    unsafe {
        gl::DepthFunc(gl::LEQUAL);
    }
    draw(render_information, world_manager);
    unsafe {
        gl::DepthFunc(gl::LESS);
    }
}
#[system(for_each)]
pub fn update_camera(cam: &Camera,
                 #[resource] world_manager: &RenderInfo

){
    for (_, program) in world_manager.programs.iter(){
        program.set_used();
        program.set_uni("pos_cam", cam.get_position());
        program.set_uni("proj", cam.get_proj());
        program.set_uni("view", cam.get_view());
    }
}