use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::InnerSizeWriter,
    event_loop::{self, EventLoop},
    window::{self, Window},
};

#[derive(Default)]
struct Pixie {
    window: Option<Window>,
}

impl ApplicationHandler for Pixie {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::Resized(physical_size) => {
                self.window.as_mut().map(|win| {
                    let scale_f = win.scale_factor();
                    let new_logical_size: LogicalSize<f64> = physical_size.to_logical(scale_f);
                    match win.request_inner_size(new_logical_size) {
                        Some(new_physical_size) => {
                            println!("New size of the window is {:?}", new_physical_size)
                        }
                        None => win.request_redraw(),
                    }
                });
            }
            winit::event::WindowEvent::RedrawRequested => {
                self.window.as_ref().map(|win| win.request_redraw());
            }
            winit::event::WindowEvent::Moved(physical_position) => todo!(),
            winit::event::WindowEvent::CloseRequested => todo!(),
            winit::event::WindowEvent::Destroyed => todo!(),
            winit::event::WindowEvent::DroppedFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFileCancelled => todo!(),
            winit::event::WindowEvent::Focused(_) => todo!(),
            winit::event::WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => todo!(),
            winit::event::WindowEvent::ModifiersChanged(modifiers) => todo!(),
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
            } => todo!(),
            winit::event::WindowEvent::CursorEntered { device_id } => todo!(),
            winit::event::WindowEvent::CursorLeft { device_id } => todo!(),
            winit::event::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => todo!(),
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => todo!(),
            winit::event::WindowEvent::PanGesture {
                device_id,
                delta,
                phase,
            } => todo!(),
            winit::event::WindowEvent::DoubleTapGesture { device_id } => todo!(),
            winit::event::WindowEvent::RotationGesture {
                device_id,
                delta,
                phase,
            } => todo!(),
            winit::event::WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => todo!(),
            winit::event::WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => todo!(),
            winit::event::WindowEvent::Touch(touch) => todo!(),
            winit::event::WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer,
            } => {
                // NOTE: Handle the scale factor changed.
            }
            _ => {}
        }
    }
}

fn main() {
    let eve_loop = match event_loop::EventLoop::new() {
        Ok(eve_loop) => eve_loop,
        Err(e) => panic!("Event loop creation failed {}", e.to_string()),
    };

    eve_loop.set_control_flow(event_loop::ControlFlow::Wait);

    let mut pixie = Pixie::default();
    let _ = eve_loop.run_app(&mut pixie);
}
