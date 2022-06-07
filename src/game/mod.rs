use std::f32::consts::PI;
use bevy_ecs::prelude::*;
use glam::{Mat4, Quat, Vec3};
use wgpu::{Device, Queue};
use crate::renderer::model::{Model, ModelLoadType};
use crate::renderer::pipeline::RenderPipeline;
use crate::renderer::Transform;

pub fn start(mut commands: Commands, device: Res<Device>, queue: Res<Queue>, render_pipeline: Res<RenderPipeline>) {
    let model = Model::load_model(
        ModelLoadType::OBJ,
        "cube.obj",
        &device,
        &queue,
        &render_pipeline.diffuse_bind_group_layout
    ).expect("Unable to load cube model");

    commands.spawn().insert(model).insert(
        Transform::from_mat4(
            Mat4::from_rotation_translation(
                Quat::from_rotation_x(PI * 0.25),
                Vec3::new(3.0, 1.0, -3.0))
        )
    );
}