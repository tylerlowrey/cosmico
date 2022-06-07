use std::error::Error;
use std::{fmt, mem};
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};
use wgpu::{BufferAddress, Device, Queue, vertex_attr_array, VertexAttribute, VertexBufferLayout, VertexStepMode};
use bevy_ecs::prelude::*;
use crate::renderer::pipeline::Vertex;
use crate::renderer::{texture, Transform};

mod loaders;

#[derive(Component)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: usize,
    pub material_index: usize
}

pub struct Material {
    pub name: String,
    pub texture: texture::Texture,
    pub bind_group: wgpu::BindGroup
}

impl Model {
    pub fn load_model(model_type: ModelLoadType, file_name: &str, device: &Device, queue: &Queue,
                            layout: &wgpu::BindGroupLayout) -> Result<Model, ModelLoadError> {
        let result = match model_type {
            ModelLoadType::OBJ => loaders::obj::load_obj_file(file_name, device, queue, layout)
        };

        if let Ok(model) = result {
            Ok(model)
        } else {
            Err(ModelLoadError)
        }
    }
}

pub enum ModelLoadType {
    OBJ
}

#[derive(Debug, Clone, Copy)]
pub struct ModelLoadError;

impl fmt::Display for ModelLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("Unable to load model")
    }
}


impl Error for ModelLoadError {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: Vec3,
    pub texture_coordinates: Vec2,
    pub normal: Vec3
}

const VERTEX_BUFFER_ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3];

impl Vertex for ModelVertex {
    fn buffer_layout_description<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &VERTEX_BUFFER_ATTRIBUTES
        }
    }
}

pub trait DrawModel<'a> {
    fn draw_model(&mut self, model: &'a Model);
}

impl<'a, 'b: 'a> DrawModel<'b> for wgpu::RenderPass<'a> {
    fn draw_model(&mut self, model: &'b Model) {
        for mesh in &model.meshes {
            self.set_bind_group(0, &model.materials[mesh.material_index].bind_group, &[]);
            self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            //render_pass.set_vertex_buffer(1, render_pipeline.instance_buffer.slice(..));
            self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..mesh.num_vertices as u32, 0, 0..1);
        }
    }
}