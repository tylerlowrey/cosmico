use std::f32::consts::PI;
use glam::{Mat4, Quat, Vec3};
use wgpu::{vertex_attr_array, VertexAttribute};
use bytemuck::{Pod, Zeroable};

pub struct Instance {
    position: Vec3,
    rotation: Quat
}

impl Instance {
    /// Used to convert an instance into a 4x4 matrix to be used in shader code
    pub fn to_raw_instance(&self) -> InstanceRaw {
        InstanceRaw {
            matrix: Mat4::from_rotation_translation(self.rotation, self.position)
        }
    }

    pub fn raw_test_instances() -> Vec<InstanceRaw> {
        let test_instances: [Instance; 6] = [
            Instance { position: Vec3::new(0.0, 0.0, 2.0), rotation: Quat::from_rotation_x(PI * 1.0) },
            Instance { position: Vec3::new(2.0, 0.0, 0.0), rotation: Quat::from_rotation_x(PI * 0.5) },
            Instance { position: Vec3::new(-2.0, 0.0, 0.0), rotation: Quat::from_rotation_y(PI * 1.0) },
            Instance { position: Vec3::new(0.0, 0.0, -2.0), rotation: Quat::from_rotation_y(PI * 0.5) },
            Instance { position: Vec3::new(2.0, 0.0, 2.0), rotation: Quat::from_rotation_z(PI * 1.0) },
            Instance { position: Vec3::new(-2.0, 0.0, -2.0), rotation: Quat::from_rotation_z(PI * 0.5) },
        ];
        test_instances.iter().map(Instance::to_raw_instance).collect()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct InstanceRaw {
    matrix: Mat4
}

impl InstanceRaw {
    const VERTEX_BUFFER_ATTRIBUTES: [VertexAttribute; 4] = vertex_attr_array![
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4
    ];

    pub fn buffer_layout_description<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::VERTEX_BUFFER_ATTRIBUTES
        }
    }
}