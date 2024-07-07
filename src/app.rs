use std::{sync::Arc, time::Instant};

use crate::{sketch::Sketch, wgpu::controller::WGPUController};
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
  surface_pixel_dimensions: [usize; 2],
}

impl<'w> App<'w> {
  async fn new(window: Window) -> Self {
    let window_arc = Arc::new(window);
    let wgpu = WGPUController::new(window_arc.clone()).await;
    Self {
      window: window_arc,
      start_instant: Instant::now(),
      last_frame_timestamp: 0.,
      wgpu,
      surface_pixel_dimensions: [0, 0],
    }
  }
  fn time(&self) -> f32 {
    self.start_instant.elapsed().as_secs_f32()
  }
  fn delta_time(&self) -> f32 {
    self.time() - self.last_frame_timestamp
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
    self.surface_pixel_dimensions = [width as usize, height as usize];
  }
  fn update(&mut self) {
    let t = self.time();
    self.last_frame_timestamp = t;
  }
}

pub async fn run_sketch<S: Sketch>() {
  let event_loop = EventLoop::new().unwrap();
  let window = WindowBuilder::new()
    .with_title("cast")
    .build(&event_loop)
    .unwrap();
  let mut app = App::new(window).await;
  let mut sketch = S::init(&app.wgpu);
  event_loop
    .run(move |event, event_loop_window_target| match event {
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == app.window.id() => match event {
        WindowEvent::CloseRequested => event_loop_window_target.exit(),
        WindowEvent::Resized(physical_size) => {
          app.resize(*physical_size);
        }
        WindowEvent::RedrawRequested => {
          app.update();
          match app.wgpu.surface.get_current_texture() {
            Err(wgpu::SurfaceError::Lost) => {
              app.resize(app.window.inner_size())
            }
            Err(err) => panic!("{err:?}"),
            Ok(surface_texture) => {
              let surface_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
              let mut encoder = app.wgpu.device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor { label: None },
              );
              sketch.update(
                &app.wgpu,
                surface_view,
                &mut encoder,
                app.surface_pixel_dimensions,
                app.time(),
                app.delta_time(),
              );
              app.wgpu.queue.submit(std::iter::once(encoder.finish()));
              surface_texture.present();
            }
          }
        }
        _ => {}
      },
      Event::AboutToWait => {
        app.window.request_redraw();
      }
      _ => {}
    })
    .unwrap();
}
