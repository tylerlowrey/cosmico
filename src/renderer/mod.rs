use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub mod pipeline;
pub mod texture;
pub mod camera;
pub mod instance;

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

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox
    };

    surface.configure(&device, &config);

    (device, queue, config)
}
