#![windows_subsystem = "windows"]

mod board;
mod renderer;
use renderer::Renderer;

use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub async fn run() -> Result<()> {
    #[cfg(debug_assertions)]
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;
    window.set_title("Hello Window");
    window.set_resizable(false);
    window.set_inner_size(LogicalSize::new(600.0, 800.0));

    let tile_size = 16.0 / 200.0;
    let width = 10;
    let height = 20;

    let board = board::Board::new(
        width,
        height,
        tile_size,
        tile_size * (width / 2) as f32,
        tile_size * (height / 2) as f32,
    );
    let renderer = Renderer::init(&window, board.num_vertices()).await?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: window_event,
            ..
        } => match window_event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,

                _ => (),
            },

            _ => (),
        },

        Event::RedrawRequested(_) => {
            let result = renderer.render(&board.to_vertices());
            match result {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => panic!("Surface Lost"),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => (),
    });
}
