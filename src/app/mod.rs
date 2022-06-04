use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::window::Fullscreen;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::RunOnce;
use bevy_ecs::world::WorldCell;
use wgpu::{Device, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use crate::core;
use crate::renderer;

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    window.set_cursor_grab(true);
    window.set_cursor_visible(true);

    let (instance, surface, adapter, size) = pollster::block_on(renderer::initialize_wgpu(&window));
    let (device, queue, config) = pollster::block_on(renderer::initialize_renderer(&adapter, &surface, &size));

    let mut world = World::new();
    world.insert_resource(core::systems::Count(0));
    world.insert_resource(instance);
    world.insert_resource(device);
    world.insert_resource(queue);
    world.insert_resource(surface);
    world.insert_resource(config);
    let mut schedule = Schedule::default();
    schedule
        .add_stage(
            "startup",
            SystemStage::parallel()
                .with_run_criteria(RunOnce::default())
                .with_system(core::systems::renderer_startup)
        )
        .add_stage(
        "update",
       SystemStage::parallel()
           .with_system(core::systems::counter)
           .with_system(core::systems::camera_control)
        )
        .add_stage(
            "render",
            SystemStage::parallel()
                .with_system(core::systems::render)
        );

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            schedule.run(&mut world);
            window.request_redraw();
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            println!("RedrawRequested");
        },
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            let world_cell = world.cell();
            match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    resize_window(world_cell, new_size)
                },
                WindowEvent::ScaleFactorChanged { new_inner_size: new_size, ..} => {
                    resize_window(world_cell, new_size);
                }
                _ => {}
            }
        },
        _ => {}
    });
}

fn resize_window(world_cell: WorldCell, new_size: &PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
        let device = world_cell.get_resource_mut::<Device>().unwrap();
        let mut config = world_cell.get_resource_mut::<SurfaceConfiguration>().unwrap();
        let surface = world_cell.get_resource_mut::<Surface>().unwrap();
        config.width = new_size.width;
        config.height = new_size.height;
        surface.configure(&device, &config);
    }
}

