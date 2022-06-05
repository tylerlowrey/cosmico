use bevy_input::ElementState;
use bevy_input::keyboard::{KeyboardInput, KeyCode};
use winit::event::{VirtualKeyCode};

pub fn convert_winit_keyboard_input(&input: &winit::event::KeyboardInput) -> KeyboardInput {
    let mut key_code = KeyCode::Return;
    if let Some(virtual_keycode) = input.virtual_keycode {
        key_code = match virtual_keycode {
            VirtualKeyCode::W => KeyCode::W,
            VirtualKeyCode::A => KeyCode::A,
            VirtualKeyCode::S => KeyCode::S,
            VirtualKeyCode::D => KeyCode::D,
            _ => KeyCode::Return
        };
    }

    let state = match input.state {
        winit::event::ElementState::Pressed => ElementState::Pressed,
        winit::event::ElementState::Released => ElementState::Released
    };

    KeyboardInput {
        scan_code: input.scancode,
        key_code: Some(key_code),
        state
    }
}