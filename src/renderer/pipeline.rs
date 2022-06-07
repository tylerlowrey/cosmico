use wgpu::{Device, PipelineLayout, ShaderModule};

use crate::renderer::instance::{InstanceRaw};
use crate::renderer::model::{ModelVertex};
use crate::renderer::texture::Texture;

pub trait Vertex {
    fn buffer_layout_description<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub struct RenderPipeline {
    pub wgpu_render_pipeline: wgpu::RenderPipeline,
    pub diffuse_bind_group_layout: wgpu::BindGroupLayout,
    pub depth_texture: Texture,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub world_transform_bind_group_layout: wgpu::BindGroupLayout,
    pub world_transform_bind_group: wgpu::BindGroup,
    pub world_transform_buffer: wgpu::Buffer
}

pub fn create_wgpu_render_pipeline(pipeline_layout: PipelineLayout, shader: ShaderModule, device: &Device,
                              config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_shader_main",
            buffers: &[ModelVertex::buffer_layout_description()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment_shader_main",
            targets: &[wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default()
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
