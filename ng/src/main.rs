// TASK: Reload model whenever it changes.
// TASK: Add `egui`-based GUI. Use it to replace the current ad-hoc GUI.
// TASK: Display model tree.
// TASK: Make model tree clickable, display the part that has been clicked.

mod args;
mod geometry;
mod graphics;
mod input;
mod math;

use std::{process::Command, time::Instant};

use futures::executor::block_on;
use tracing::trace;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    args::Args,
    geometry::{bounding_volume::BoundingVolume as _, faces::Faces as _},
    graphics::{DrawConfig, Renderer, Transform},
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let model_dir = format!("models/{}", args.model);

    // This can be made a bit more contact using `ExitStatus::exit_ok`, once
    // that is stable.
    let status = Command::new("cargo")
        .arg("build")
        .args(["--manifest-path", &format!("{}/Cargo.toml", model_dir)])
        .status()?;
    assert!(status.success());

    // TASK: Read up why those calls are unsafe. Make sure calling them is
    //       sound, and document why that is.
    let shape = unsafe {
        let lib = libloading::Library::new(format!(
            "{}/target/debug/lib{}.so",
            model_dir, args.model,
        ))?;
        let func: libloading::Symbol<ModelFn> = lib.get(b"model")?;
        func()
    };

    // TASK: Choose tolerance value intelligently.
    let triangles = shape.triangles(0.1);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let mut input_handler = input::Handler::new();
    let mut renderer = block_on(Renderer::new(&window, triangles.into()))?;

    let mut draw_config = DrawConfig::default();
    let mut transform = Transform::new(shape.aabb());

    let mut previous_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                renderer.handle_resize(size);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                input_handler.handle_keyboard_input(input, &mut actions);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                input_handler.handle_cursor_moved(position, &mut transform);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                input_handler.handle_mouse_input(button, state);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                input_handler.handle_mouse_wheel(delta);
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_t = now.duration_since(previous_time);
                previous_time = now;

                input_handler.update(delta_t.as_secs_f32(), &mut transform);

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                match renderer.draw(&transform, &draw_config) {
                    Ok(()) => {}
                    Err(err) => {
                        panic!("Draw error: {}", err);
                    }
                }
            }
            _ => {}
        }

        if actions.exit {
            *control_flow = ControlFlow::Exit;
        }
        if actions.toggle_model {
            draw_config.draw_model = !draw_config.draw_model;
        }
        if actions.toggle_mesh {
            draw_config.draw_mesh = !draw_config.draw_mesh;
        }
    });
}

type ModelFn = unsafe extern "C" fn() -> fj::Shape;
