use glam::{Mat4, Vec3};
use bytemuck::{ Pod, Zeroable };
use bevy_ecs::prelude::*;
use bevy_input::keyboard::{KeyCode};

#[derive(Component)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub uniform: CameraUniform,
    pub speed: f32
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

    pub fn update(&mut self, key_inputs: &Vec<KeyCode>, delta_time: f32) {
        for key_code in key_inputs {
            let forward = self.target - self.eye;
            let forward_normalized = forward.normalize();
            if *key_code == KeyCode::W {
                self.eye += forward_normalized * self.speed * delta_time
            }
            if *key_code == KeyCode::S {
                self.eye -= forward_normalized * self.speed * delta_time
            }

            let forward = self.target - self.eye;
            let right = forward.normalize().cross(self.up);
            if *key_code == KeyCode::A {
                self.eye -= right * self.speed * delta_time;
            }
            if *key_code == KeyCode::D {
                self.eye += right * self.speed * delta_time;
            }
        }
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
