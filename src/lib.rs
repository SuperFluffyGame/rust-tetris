mod board;
mod renderer;
mod tetrimino;
use std::thread;

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
    window.set_title("Tetris In Rust");
    window.set_resizable(false);
    window.set_inner_size(LogicalSize::new(500.0, 700.0));

    let tile_size = 20.0 / 200.0;
    let width = 10;
    let height = 40;

    let mut board = board::Board::new(
        width,
        height,
        tile_size,
        tile_size * 5 as f32,
        tile_size * 10 as f32,
    );

    board.set_current_tetrimino(&tetrimino::Tetrimino::I, 3, 17, 0);

    board.place_tetrimino(&tetrimino::Tetrimino::I, 0, -3, 0);

    let renderer = Renderer::init(&window, board.num_vertices()).await?;

    let mut previous_key_pressed: Option<VirtualKeyCode> = None;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: window_event,
            ..
        } => match window_event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

            WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                Some(VirtualKeyCode::Space) => match previous_key_pressed {
                    Some(VirtualKeyCode::Space) => {
                        previous_key_pressed = None;
                    }
                    _ => {
                        board.rotate_current_tetrimino(1);
                        println!("{:?}", board.current_tetrimino_rotation);
                        previous_key_pressed = Some(VirtualKeyCode::Space);
                    }
                },

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
