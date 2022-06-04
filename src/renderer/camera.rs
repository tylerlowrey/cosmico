use glam::{Mat4, Vec3};
use bytemuck::{ Pod, Zeroable };
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub uniform: CameraUniform
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let projection = Mat4::perspective_rh(
            self.fov_y,
            self.aspect,
            self.z_near,
            self.z_far
        );

        return projection * view;
    }
    pub fn update(&mut self) {
        let forward = self.target - self.eye;
        let speed_coefficient = 0.001;
        self.eye -= forward * speed_coefficient;
        self.uniform.view_projection = self.build_view_projection_matrix();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_projection: Mat4
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_projection: Mat4::IDENTITY
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection_matrix();
    }
}
