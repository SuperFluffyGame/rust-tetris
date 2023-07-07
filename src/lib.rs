mod board;
mod renderer;
mod tetrimino;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

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

    let mut board = board::Board::new(width, height, tile_size, tile_size * 5.0, tile_size * 10.0);

    board.place_tetrimino(&tetrimino::Tetrimino::I, 0, -3, 0);
    board.place_tetrimino(&tetrimino::Tetrimino::I, 4, -3, 0);
    board.place_tetrimino(&tetrimino::Tetrimino::I, 0, -2, 0);
    board.place_tetrimino(&tetrimino::Tetrimino::I, 4, -2, 0);
    board.place_tetrimino(&tetrimino::Tetrimino::I, 0, -1, 0);
    board.place_tetrimino(&tetrimino::Tetrimino::I, 4, -1, 0);
    board.place_tetrimino(&tetrimino::Tetrimino::I, 0, 0, 0);
    board.place_tetrimino(&tetrimino::Tetrimino::I, 4, 0, 0);

    board.place_tetrimino(&tetrimino::Tetrimino::I, 6, -1, 1);

    board.set_current_tetrimino(&tetrimino::Tetrimino::I, 3, 17, 0);

    let renderer = Renderer::init(&window, board.num_vertices()).await?;

    let mut previous_key_pressed: Option<VirtualKeyCode> = None;

    let board_arc = Arc::new(Mutex::new(board));

    let board_cloned = board_arc.clone();
    let _timer_thread = std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));
        let mut board = board_cloned.lock().unwrap();

        board.update();
    });

    let board_cloned = board_arc;
    event_loop.run(move |event, _, control_flow| {
        let mut board = board_cloned.lock().unwrap();

        match event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                    Some(VirtualKeyCode::Q) => match previous_key_pressed {
                        Some(VirtualKeyCode::Q) => {
                            previous_key_pressed = None;
                        }
                        _ => {
                            board.rotate_left();
                            previous_key_pressed = Some(VirtualKeyCode::Q);
                        }
                    },
                    Some(VirtualKeyCode::E) => match previous_key_pressed {
                        Some(VirtualKeyCode::E) => {
                            previous_key_pressed = None;
                        }
                        _ => {
                            board.rotate_right();
                            previous_key_pressed = Some(VirtualKeyCode::E);
                        }
                    },
                    Some(VirtualKeyCode::A) => match previous_key_pressed {
                        Some(VirtualKeyCode::A) => {
                            previous_key_pressed = None;
                        }
                        _ => {
                            board.move_left();
                            previous_key_pressed = Some(VirtualKeyCode::A);
                        }
                    },
                    Some(VirtualKeyCode::D) => match previous_key_pressed {
                        Some(VirtualKeyCode::D) => {
                            previous_key_pressed = None;
                        }
                        _ => {
                            board.move_right();
                            previous_key_pressed = Some(VirtualKeyCode::D);
                        }
                    },
                    Some(VirtualKeyCode::S) => match previous_key_pressed {
                        Some(VirtualKeyCode::S) => {
                            previous_key_pressed = None;
                        }
                        _ => {
                            board.move_down();
                            previous_key_pressed = Some(VirtualKeyCode::D);
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
        }
    });
}
