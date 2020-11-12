use std::collections::HashMap;

pub struct EntityRenderInfo {
    pub mesh: String,
    pub program: String,
    pub textures: HashMap<String, String>,
}
