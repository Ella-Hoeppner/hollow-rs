use std::{sync::Arc, time::Instant};

use crate::{renderer::Renderer, wgpu::WGPUController};
use winit::{
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

struct App<'w> {
  window: Arc<Window>,
  start_instant: Instant,
  last_frame_timestamp: f32,
  wgpu: WGPUController<'w>,
  renderer: Renderer,
  surface_pixel_dimensions: (usize, usize),
}

impl<'w> App<'w> {
  async fn new(window: Window) -> Self {
    let window_arc = Arc::new(window);
    let wgpu = WGPUController::new(window_arc.clone()).await;
    let renderer = Renderer::new(&wgpu);
    Self {
      window: window_arc,
      start_instant: Instant::now(),
      last_frame_timestamp: 0.,
      wgpu,
      renderer,
      surface_pixel_dimensions: (1, 1),
    }
  }
  fn time(&self) -> f32 {
    self.start_instant.elapsed().as_secs_f32()
  }
  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    let width = new_size.width;
    let height = new_size.height;
    if width > 0 && height > 0 {
      self.wgpu.config.width = width;
      self.wgpu.config.height = height;
      self
        .wgpu
        .surface
        .configure(&self.wgpu.device, &self.wgpu.config);
    }
    self.surface_pixel_dimensions = (width as usize, height as usize);
  }
  fn handle_window_event(&self, event: &WindowEvent) -> bool {
    false
  }
  fn update(&mut self) {
    let t = self.time();
    self.last_frame_timestamp = t;
  }
}

pub async fn run_app() {
  let event_loop = EventLoop::new().unwrap();
  let window = WindowBuilder::new()
    .with_title("cast")
    .build(&event_loop)
    .unwrap();
  let mut state = App::new(window).await;
  event_loop
    .run(move |event, event_loop_window_target| match event {
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == state.window.id() => {
        if !state.handle_window_event(event) {
          match event {
            WindowEvent::CloseRequested => event_loop_window_target.exit(),
            WindowEvent::Resized(physical_size) => {
              state.resize(*physical_size);
            }
            WindowEvent::RedrawRequested => {
              state.update();
              let t = state.time();
              match state.renderer.render(
                &state.wgpu,
                state.surface_pixel_dimensions,
                t,
              ) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => {
                  state.resize(state.window.inner_size())
                }
                Err(e) => panic!("{:?}", e),
              }
            }
            _ => {}
          }
        }
      }
      Event::AboutToWait => {
        state.window.request_redraw();
      }
      _ => {}
    })
    .unwrap();
}
