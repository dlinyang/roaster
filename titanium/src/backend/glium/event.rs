use super::GLApplication;
use crate::event::*;
use glium::glutin;
use glutin::platform::desktop::EventLoopExtDesktop;
use glutin::event::Event as GEvent;
use glutin::event::{WindowEvent,DeviceEvent};
use glutin::event_loop::ControlFlow;

impl EventBackend for GLApplication {
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    fn get_event(&mut self) -> Event {
        let mut result = Event::Other;

        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            #[cfg(debug)]
            println!("{:?}",event);

            match event {
                GEvent::NewEvents(e) => (),
                GEvent::WindowEvent{window_id,event} => match event {
                    WindowEvent::CloseRequested => result = Event::Exit,
                    WindowEvent::CursorMoved{position,..} => result = Event::CursorEvent{x: position.x as f32, y: position.y as f32},
                    _ => (),
                },
                GEvent::DeviceEvent{device_id,event} => match event {
                    DeviceEvent::Key(event) => result = match_keyboard(event),
                    _ => (),
                },
                GEvent::UserEvent(e) => (),
                GEvent::Suspended => (),
                GEvent::Resumed => (),
                GEvent::MainEventsCleared => *control_flow = ControlFlow::Exit,
                GEvent::RedrawEventsCleared => (),
                GEvent::RedrawRequested(window_id) => (),
                GEvent::LoopDestroyed => (),
            }
        });
        result
    }
}

use glutin::event::ElementState;
use glutin::event::VirtualKeyCode;
fn match_keyboard(event: glutin::event::KeyboardInput) -> Event {
    let state = match event.state {
        ElementState::Pressed => ButtonState::Press,
        ElementState::Released => ButtonState::Release,
    };

    let mut key = Key::OTHER;
    if let Some(g_key) = event.virtual_keycode {
        match g_key {
            VirtualKeyCode::Key1 => key = Key::Num1,
            VirtualKeyCode::Key2 => key = Key::Num2,
            VirtualKeyCode::Key3 => key = Key::Num3,
            VirtualKeyCode::Key4 => key = Key::Num4,
            VirtualKeyCode::Key5 => key = Key::Num5,
            VirtualKeyCode::Key6 => key = Key::Num6,
            VirtualKeyCode::Key7 => key = Key::Num7,
            VirtualKeyCode::Key8 => key = Key::Num8,
            VirtualKeyCode::Key9 => key = Key::Num9,
            VirtualKeyCode::Key0 => key = Key::Num0,
            VirtualKeyCode::A => key = Key::A,
            VirtualKeyCode::B => key = Key::B,
            VirtualKeyCode::C => key = Key::C,
            VirtualKeyCode::D => key = Key::D,
            VirtualKeyCode::E => key = Key::E,
            VirtualKeyCode::F => key = Key::F,
            VirtualKeyCode::G => key = Key::G,
            VirtualKeyCode::H => key = Key::H,
            VirtualKeyCode::I => key = Key::I,
            VirtualKeyCode::J => key = Key::J,
            VirtualKeyCode::K => key = Key::K,
            VirtualKeyCode::L => key = Key::L,
            VirtualKeyCode::M => key = Key::M,
            VirtualKeyCode::N => key = Key::N,
            VirtualKeyCode::O => key = Key::O,
            VirtualKeyCode::P => key = Key::P,
            VirtualKeyCode::Q => key = Key::Q,
            VirtualKeyCode::R => key = Key::R,
            VirtualKeyCode::S => key = Key::S,
            VirtualKeyCode::T => key = Key::T,
            VirtualKeyCode::U => key = Key::U,
            VirtualKeyCode::V => key = Key::V,
            VirtualKeyCode::W => key = Key::W,
            VirtualKeyCode::X => key = Key::X,
            VirtualKeyCode::Y => key = Key::Y,
            VirtualKeyCode::Z => key = Key::Z,
            VirtualKeyCode::Up => key = Key::Up,
            VirtualKeyCode::Down => key = Key::Down,
            VirtualKeyCode::Left => key = Key::Left,
            VirtualKeyCode::Right => key = Key::Right,
            _=>(),
        };
    }

    Event::KeyEvent{state,key}
}