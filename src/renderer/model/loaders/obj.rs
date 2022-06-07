use glam::{Vec2, Vec3};
use wgpu::util::DeviceExt;
use crate::app;
use crate::renderer::model::{Material, Mesh, Model, ModelVertex};
use crate::renderer::texture::Texture;

pub fn load_obj_file(file_name: &str, device: &wgpu::Device, queue: &wgpu::Queue,
                     layout: &wgpu::BindGroupLayout) -> Result<Model, tobj::LoadError> {
    let loaded_obj = tobj::load_obj(
        format!("{}/{}", app::ASSETS_DIR, file_name),
        &tobj::GPU_LOAD_OPTIONS
    );

    let (obj_models, obj_materials_result) = loaded_obj?;

    let obj_materials = obj_materials_result?;
    let mut model_materials = Vec::with_capacity(obj_materials.len());
    for material in obj_materials {
        let texture = Texture::load_texture(material.diffuse_texture.as_str(), device, queue).unwrap();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler.as_ref().unwrap()),
                }
            ]
        });

        model_materials.push(Material{
            name: material.name,
            texture,
            bind_group
        })
    }

    let mut model_meshes = Vec::with_capacity(obj_models.len());
    for obj_model in obj_models {
        let num_vertices = obj_model.mesh.positions.len() / 3;
        let mut vertices = Vec::with_capacity(num_vertices);
        for i in 0..num_vertices {
            vertices.push(ModelVertex {
                position: Vec3::new(
                    obj_model.mesh.positions[i * 3],
                    obj_model.mesh.positions[i * 3 + 1],
                    obj_model.mesh.positions[i * 3 + 2],
                ),
                texture_coordinates: Vec2::new(
                    obj_model.mesh.texcoords[i * 2],
                    obj_model.mesh.texcoords[i * 2 + 1]
                ),
                normal: Vec3::new(
                    obj_model.mesh.normals[i * 3],
                    obj_model.mesh.normals[i * 3 + 1],
                    obj_model.mesh.normals[i * 3 + 2],
                )
            })
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&obj_model.mesh.indices),
            usage: wgpu::BufferUsages::INDEX
        });

        model_meshes.push(Mesh {
            name: obj_model.name,
            vertex_buffer,
            index_buffer,
            num_vertices: obj_model.mesh.indices.len(),
            material_index: obj_model.mesh.material_id.unwrap_or(0)
        })
    }

    Ok(Model {
        meshes: model_meshes,
        materials: model_materials
    })
}