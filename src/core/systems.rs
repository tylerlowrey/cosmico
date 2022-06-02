use bevy_ecs::prelude::*;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use wgpu::util::DeviceExt;
use crate::renderer::pipeline::{create_wgpu_render_pipeline, RenderPipeline, Vertex};
use crate::renderer::texture::Texture;

pub struct Count(pub usize);

pub(crate) fn counter(mut count: ResMut<Count>) {
    count.0 = count.0 + 1;
    println!("{}", count.0);
}

pub(crate) fn renderer_startup(mut commands: Commands, device: Res<Device>,
                               queue: Res<Queue>, config: Res<SurfaceConfiguration>) {
    println!("Renderer starting up...");

    let texture = Texture::from_bytes(
        &device,
        &queue,
        include_bytes!("../../assets/tree.png"),
        "tree_texture"
    ).unwrap();

    let diffuse_bind_group_layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        }
    );

    let diffuse_bind_group = device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &diffuse_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                }
            ],
            label: Some("diffuse_bind_group"),
        }
    );


    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
    });


    const VERTICES: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], texture_coordinates: [0.4131759, 0.00759614], },
        Vertex { position: [-0.49513406, 0.06958647, 0.0], texture_coordinates: [0.0048659444, 0.43041354], },
        Vertex { position: [-0.21918549, -0.44939706, 0.0], texture_coordinates: [0.28081453, 0.949397], },
        Vertex { position: [0.35966998, -0.3473291, 0.0], texture_coordinates: [0.85967, 0.84732914], },
        Vertex { position: [0.44147372, 0.2347359, 0.0], texture_coordinates: [0.9414737, 0.2652641], },
    ];

    const INDICES: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];

    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX
        }
    );

    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX
        }
    );


    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&diffuse_bind_group_layout],
        push_constant_ranges: &[]
    });

    let wgpu_render_pipeline = create_wgpu_render_pipeline(
        render_pipeline_layout,
        shader,
        &device,
        &config
    );

    let render_pipeline = RenderPipeline {
        wgpu_render_pipeline,
        vertex_buffer,
        index_buffer,
        num_vertices: VERTICES.len() as u32,
        num_indices: INDICES.len() as u32,
        diffuse_bind_group,
        diffuse_bind_group_layout,
        diffuse_texture: texture,
    };

    commands.insert_resource(render_pipeline);
}

pub(crate) fn render(surface: Res<Surface>, device: Res<Device>, queue: Res<Queue>, render_pipeline: Res<RenderPipeline>) {
    println!("Rendering...");

    let output = surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render encoder")
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&render_pipeline.wgpu_render_pipeline);
        render_pass.set_bind_group(0, &render_pipeline.diffuse_bind_group, &[]);
        //render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, render_pipeline.vertex_buffer.slice(..));
        render_pass.set_index_buffer(render_pipeline.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..render_pipeline.num_indices, 0, 0..1);
    }

    // submit will accept anything that implements IntoIter
    queue.submit(std::iter::once(encoder.finish()));
    output.present();
}