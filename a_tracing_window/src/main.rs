use a_tracing_lib::tracer::ATracer;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::gui::GuiFramework;

mod gui;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() -> Result<(), Error> {
    env_logger::init();

    // Create window
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("ATracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Building window failed.")
    };

    // Create drawing surface
    let (mut pixels, mut gui_framework) = {
        let window_size = window.inner_size();
        let width = window_size.width;
        let height = window_size.height;
        let surface_texture = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

        let scale_factor = window.scale_factor() as f32;
        let framework = GuiFramework::new(&event_loop, width, height, scale_factor, &pixels);

        (pixels, framework)
    };

    let mut tracer = ATracer::new(WIDTH, HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // When the window resizes also resize the tracer
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Err(err) = pixels.resize_buffer(size.width, size.height) {
                    log_error("pixels.resize_buffer", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                gui_framework.resize(size.width, size.height);
                tracer.resize(size.width, size.height);
            }

            if let Some(scale_factor) = input.scale_factor() {
                gui_framework.scale_factor(scale_factor as f32);
            }

            tracer.update();
            tracer.start_render();
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                gui_framework.handle_event(&event);
            }

            Event::RedrawRequested(_) => {
                tracer.draw(pixels.frame_mut());

                gui_framework.prepare(&window);

                let render_result = pixels.render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);
                    gui_framework.render(encoder, render_target, context);

                    Ok(())
                });

                if let Err(err) = render_result {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => (),
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
}
