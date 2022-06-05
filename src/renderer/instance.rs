use glam::{Mat4, Quat, Vec3};

pub struct Instance {
    position: Vec3,
    rotation: Quat
}

impl Instance {
    /// Used to convert an instance into a 4x4 matrix to be used in shader code
    pub fn to_raw_instance(&self) -> InstanceRaw {
        InstanceRaw {
            model: Mat4::from_rotation_translation(self.rotation, self.position)
        }
    }
}

pub struct InstanceRaw {
    model: Mat4
}