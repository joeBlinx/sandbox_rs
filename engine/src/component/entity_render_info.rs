use std::collections::HashMap;

pub struct EntityRenderInfo {
    pub mesh: String,
    pub program: String,
    pub textures: HashMap<String, String>,
}
pub struct RigidBody{
    pub position: nalgebra_glm::Vec3,
    pub rotation: nalgebra_glm::Vec3,
    pub scale: nalgebra_glm::Vec3,
}
impl RigidBody{
    pub fn model_matrix(&self) -> nalgebra_glm::Mat4{
        let scale_matrix = nalgebra_glm::scaling(&self.scale);
        let translation_matrix = nalgebra_glm::translation(&self.position);
        let rotate_x = nalgebra_glm::rotate_x(&nalgebra_glm::identity(), self.rotation.x);
        let rotate_y = nalgebra_glm::rotate_y(&nalgebra_glm::identity(), self.rotation.y);
        let rotate_z = nalgebra_glm::rotate_z(&nalgebra_glm::identity(), self.rotation.z);

        return scale_matrix*translation_matrix*rotate_x*rotate_y*rotate_z;
    }
}