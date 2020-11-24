use crate::mesh::Mesh;
use crate::render_info;
use crate::render_info::RenderInfo;
use crate::system::draw::*;
use crate::system::event::*;
use glish_rs::shader::Shader;
use legion;
use legion::storage::IntoComponentSource;
use legion::world::Entry;
use legion::{Entity, Schedule};
use std::path::Path;
use crate::component::event::CloseEvent;
use crate::resources::window::Window;
use std::sync::Mutex;

pub struct World {
    world_legion: legion::World,
    resources: legion::Resources,
    event_resources: legion::Resources,
    schedule: legion::Schedule,
    event_schedule: legion::Schedule,
}

impl World {
    pub fn run(&mut self) {
        {
            let mut window = &mut *self.resources.get_mut::<Window>().unwrap();
            window.clear();
            for event in window.sdl().event_pump().unwrap().poll_iter() {
                self.event_resources.insert(event);
                self.event_schedule.execute(&mut self.world_legion, &mut self.event_resources);
            }
        }
        self.schedule
            .execute(&mut self.world_legion, &mut self.resources);
        let mut window = &mut *self.resources.get_mut::<Window>().unwrap();
        window.refresh();
    }
    pub fn use_render_info<T: Fn(&mut RenderInfo)>(&mut self, function: T) {
        function(&mut *self.resources.get_mut::<RenderInfo>().unwrap());
    }

    pub fn add_components<T>(&mut self, components: T) -> Entity
    where
        Option<T>: IntoComponentSource,
    {
        self.world_legion.push(components)
    }
    pub fn entry(&mut self, entity: legion::Entity) -> Option<Entry> {
        self.world_legion.entry(entity)
    }

    fn run_event(&mut self){
        self.event_schedule.execute(&mut self.world_legion, &mut self.resources);
    }
    pub fn ecs_world(&mut self) -> &mut legion::World{
        &mut self.world_legion
    }
    pub fn new(ogl_version:(u8, u8), width:i32, height:i32,
               schedule: Schedule, event_schedule:
    Schedule) -> World{
        let window = Window::new(ogl_version, width, height);
        let mut render_info = render_info::RenderInfo::default();
        create_program(&mut render_info);
        render_info.add_mesh("plane", Mesh::create_plane());

        let mut resources = legion::Resources::default();
        resources.insert(render_info);
        resources.insert(window);
        let mut event_resources = legion::Resources::default();
        let mut world_legion = legion::World::default();
        world_legion.push((1, CloseEvent{event:false}));
        World {
            world_legion,
            resources,
            event_resources,
            schedule,
            event_schedule
        }
    }
    pub fn add_imgui(&mut self){
        let imgui_info = self.resources.get_mut::<Window>().unwrap().create_imgui();
        self.add_components((1, imgui_info));
    }
}


fn create_program(world: &mut RenderInfo) {
    let shaders_classic = [
        Shader::from_vert_file(Path::new("assets/shader/vertex/triangle.vert")).unwrap(),
        Shader::from_frag_file(Path::new("assets/shader/fragment/triangle.frag")).unwrap(),
    ];
    let shaders_with_normal = [
        Shader::from_vert_file(Path::new("assets/shader/vertex/triangle.vert")).unwrap(),
        Shader::from_frag_file(Path::new("assets/shader/fragment/normal_mapping.frag")).unwrap(),
    ];

    let _skybox_shaders = [
        Shader::from_vert_file(Path::new("assets/shader/vertex/skybox.vert")).unwrap(),
        Shader::from_frag_file(Path::new("assets/shader/fragment/skybox.frag")).unwrap(),
    ];

    world.add_program_from_shaders("classic", &shaders_classic);
    world.add_program_from_shaders("normal_map", &shaders_with_normal);
    // world.add_program_from_shaders("skybox", &skybox_shaders);
}
