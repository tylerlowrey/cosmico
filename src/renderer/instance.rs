use glam::{Quat, Vec3};
use bytemuck::{ Pod, Zeroable };

pub struct Instance {
    position: Vec3,
    rotation: Quat
}