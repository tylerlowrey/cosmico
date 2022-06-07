use glam::{Mat4, Quat, Vec3};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use bevy_ecs::prelude::*;

pub mod pipeline;
pub mod texture;
pub mod camera;
pub mod instance;
pub mod model;

#[derive(Component)]
pub struct Transform {
    pub matrix: Mat4
}

impl Transform {
    pub fn new() -> Self {
        Self {
            matrix: Mat4::IDENTITY
        }
    }

    pub fn from_rotation_translation(quat: Quat, x: f32, y: f32, z: f32) -> Self {
        Self {
            matrix: Mat4::from_rotation_translation(
                quat,
                Vec3::new(x, y, z),
            )
        }
    }

    pub fn from_mat4(matrix: Mat4) -> Self {
        Self {
            matrix
        }
    }
}

pub async fn initialize_wgpu(window: &Window) -> (Instance, Surface, Adapter, PhysicalSize<u32>){
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe {  instance.create_surface(window) };
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();

    (instance, surface, adapter, size)
}

pub async fn initialize_renderer(adapter: &Adapter, surface: &Surface, size: &PhysicalSize<u32>)
    -> (Device, Queue, SurfaceConfiguration) {

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None
        },
        None
    ).await.unwrap();

    let config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox
    };

    surface.configure(&device, &config);

    (device, queue, config)
}
