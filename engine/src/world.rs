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

pub struct World {
    world_legion: legion::World,
    resources: legion::Resources,
    schedule: legion::Schedule,
    event_schedule: legion::Schedule,
}

impl World {
    pub fn run(&mut self, event_pump: &mut sdl2::EventPump) {
        for event in event_pump.poll_iter() {
            self.resources.insert(event);
            self.event_schedule.execute(&mut self.world_legion, &mut self.resources);
        }
        self.schedule
            .execute(&mut self.world_legion, &mut self.resources);
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
    pub fn legion_world(&mut self) -> &mut legion::World{
        &mut self.world_legion
    }
}

impl Default for World {
    fn default() -> Self {
        let schedule = Schedule::builder()
            .add_system(draw_skybox_system())
            .flush()
            .add_system(draw_entity_system())
            .add_system(update_camera_system())
            .build();
        let mut render_info = render_info::RenderInfo::default();
        create_program(&mut render_info);
        render_info.add_mesh("plane", Mesh::create_plane());

        let mut resources = legion::Resources::default();
        resources.insert(render_info);
        let mut world_legion = legion::World::default();
        world_legion.push((1, CloseEvent{event:false}));
        World {
            world_legion,
            resources,
            schedule,
            event_schedule: Schedule::builder()
                .add_system(quit_event_system())
                .build()
        }
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
