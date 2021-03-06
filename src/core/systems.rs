use bevy_ecs::prelude::*;
use bevy_input::keyboard::{KeyboardInput, KeyCode};
use glam::{Vec3, Mat4};
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, Device, Queue, ShaderStages, Surface, SurfaceConfiguration};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::core::time::Time;
use crate::renderer::camera::{Camera, CameraUniform};
use crate::renderer::model::{DrawModel, Model};
use crate::renderer::pipeline::{create_wgpu_render_pipeline, RenderPipeline};
use crate::renderer::texture::Texture;
use crate::renderer::Transform;

pub struct Count(pub usize);

pub(crate) fn counter(mut count: ResMut<Count>, mut input_events: EventReader<KeyboardInput>) {
    count.0 = count.0 + 1;
    let mut num_events = 0;
    for event in input_events.iter() {
        match event.key_code.unwrap() {
            KeyCode::W => println!("Key event: W"),
            KeyCode::A => println!("Key event: A"),
            KeyCode::S => println!("Key event: S"),
            KeyCode::D => println!("Key event: D"),
            KeyCode::Return => {
                println!("Key event: Return")
            },
            _ => println!("Unknown key event")
        }
        num_events += 1;
    }

    if num_events > 0 {
        println!("Num events: {}", num_events);
    }
}

pub(crate) fn camera_control(mut query: Query<&mut Camera>, mut input_events: EventReader<KeyboardInput>, time: Res<Time>) {
    let mut key_inputs = Vec::new();
    for event in input_events.iter() {
        key_inputs.push(event.key_code.unwrap_or(KeyCode::Return))
    }
    for mut camera in query.iter_mut() {
        camera.update(&key_inputs, time.delta_seconds);
    }
}

pub(crate) fn renderer_startup(mut commands: Commands, device: Res<Device>, config: Res<SurfaceConfiguration>) {
    /*
    let texture = Texture::from_bytes(
        &device,
        &queue,
        include_bytes!("../../assets/tree.png"),
        "tree_texture"
    ).unwrap();
    */

    let diffuse_bind_group_layout = device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        }
    );

    let world_transform_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("World Transform Bind Group Layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None
            },
            count: None
        }]
    });

    let world_transform_buffer = device.create_buffer_init(&BufferInitDescriptor{
        label: Some("World Transform Uniform Buffer"),
        contents: bytemuck::bytes_of(&Mat4::IDENTITY),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });

    let world_transform_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("World Transform Bind Group"),
        layout: &world_transform_bind_group_layout,
        entries: &[BindGroupEntry{
            binding: 0,
            resource: world_transform_buffer.as_entire_binding()
        }]
    });

    /*
    let diffuse_bind_group = device.create_bind_group(
        &BindGroupDescriptor {
            layout: &diffuse_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler.as_ref().unwrap()),
                }
            ],
            label: Some("diffuse_bind_group"),
        }
    );*/


    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
    });


    /*
    let vertices: &[ModelVertex] = &[
        ModelVertex { position: Vec3::new(-0.0868241, 0.49240386, 0.0), texture_coordinates: Vec2::new(0.4131759, 0.00759614), normal: Vec3::default()},
        ModelVertex { position: Vec3::new(-0.49513406, 0.06958647, 0.0), texture_coordinates: Vec2::new(0.0048659444, 0.43041354), normal: Vec3::default()},
        ModelVertex { position: Vec3::new(-0.21918549, -0.44939706, 0.0), texture_coordinates: Vec2::new(0.28081453, 0.949397), normal: Vec3::default()},
        ModelVertex { position: Vec3::new(0.35966998, -0.3473291, 0.0), texture_coordinates: Vec2::new(0.85967, 0.84732914), normal: Vec3::default()},
        ModelVertex { position: Vec3::new(0.44147372, 0.2347359, 0.0), texture_coordinates: Vec2::new(0.9414737, 0.2652641), normal: Vec3::default()},
    ];

    let indices: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ]; */

    /*
    let vertex_buffer = device.create_buffer_init(
        &BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX
        }
    );

    let index_buffer = device.create_buffer_init(
        &BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX
        }
    );

    let instances = Instance::raw_test_instances();

    let instance_buffer = device.create_buffer_init(
        &BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: BufferUsages::VERTEX
        }
    );
     */

    let camera_matrix_buffer = device.create_buffer_init(&BufferInitDescriptor{
        label: Some("Camera Buffer"),
        contents: bytemuck::bytes_of(&Mat4::IDENTITY),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
    });

    let camera_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Camera Bind Group Layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None
            },
            count: None
        }]
    });

    let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Camera Bind Group"),
        layout: &camera_bind_group_layout,
        entries: &[BindGroupEntry{
            binding: 0,
            resource: camera_matrix_buffer.as_entire_binding()
        }]
    });

    let camera = Camera {
        eye: (0.0, 1.0, 2.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: Vec3::Y,
        aspect: config.width as f32 / config.height as f32,
        fov_y: 45.0,
        z_near: 0.1,
        z_far: 100.0,
        speed: 10.0,
        uniform: CameraUniform::new(),
        buffer: camera_matrix_buffer,
        bind_group: camera_bind_group
    };

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&diffuse_bind_group_layout, &camera_bind_group_layout, &world_transform_bind_group_layout],
        push_constant_ranges: &[]
    });

    let wgpu_render_pipeline = create_wgpu_render_pipeline(
        render_pipeline_layout,
        shader,
        &device,
        &config
    );

    let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

    let render_pipeline = RenderPipeline {
        wgpu_render_pipeline,
        diffuse_bind_group_layout,
        depth_texture,
        camera_bind_group_layout,
        world_transform_bind_group_layout,
        world_transform_bind_group,
        world_transform_buffer
    };

    commands.insert_resource(render_pipeline);
    commands.spawn().insert(camera);
}

pub(crate) fn render(surface: Res<Surface>, device: Res<Device>, queue: Res<Queue>,
                     render_pipeline: Res<RenderPipeline>, mut camera_query: Query<&mut Camera>,
                     models_query: Query<(&Model, &Transform)>) {
    let output = surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render encoder")
    });

    let camera = camera_query.iter_mut().next().unwrap();

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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &render_pipeline.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&render_pipeline.wgpu_render_pipeline);
        //render_pass.set_bind_group(0, &render_pipeline.diffuse_bind_group, &[]);
        render_pass.set_bind_group(1, &camera.bind_group, &[]);
        render_pass.set_bind_group(2, &render_pipeline.world_transform_bind_group, &[]);

        for (model, transform) in models_query.iter() {
            queue.write_buffer(&render_pipeline.world_transform_buffer, 0, bytemuck::bytes_of(&transform.matrix));
            render_pass.draw_model(model);
        }
        //render_pass.set_vertex_buffer(0, render_pipeline.vertex_buffer.slice(..));
        //render_pass.set_vertex_buffer(1, render_pipeline.instance_buffer.slice(..));
        //render_pass.set_index_buffer(render_pipeline.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        //render_pass.draw_indexed(0..render_pipeline.num_indices, 0, 0..render_pipeline.instances.len() as u32);
    }



    queue.write_buffer(&camera.buffer, 0, bytemuck::bytes_of(&camera.uniform.view_projection));
    // submit will accept anything that implements IntoIter
    queue.submit(std::iter::once(encoder.finish()));
    output.present();
}