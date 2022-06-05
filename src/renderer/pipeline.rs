use wgpu::{Device, PipelineLayout, ShaderModule, VertexAttribute, vertex_attr_array};
use bytemuck::{ Pod, Zeroable };
use crate::renderer::texture::Texture;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coordinates: [f32; 2]
}

impl Vertex {
    const VERTEX_BUFFER_ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];
    fn buffer_layout_description<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_BUFFER_ATTRIBUTES
        }
    }
}

pub struct RenderPipeline {
    pub wgpu_render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub diffuse_bind_group_layout: wgpu::BindGroupLayout,
    pub diffuse_texture: Texture,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_buffer: wgpu::Buffer
}

pub fn create_wgpu_render_pipeline(pipeline_layout: PipelineLayout, shader: ShaderModule, device: &Device,
                              config: &wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex_shader_main",
            buffers: &[Vertex::buffer_layout_description()],
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
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
