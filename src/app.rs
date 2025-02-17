use std::{sync::Arc, time::Instant};

use crate::{
  sketch::{Sketch, SketchData},
  wgpu::controller::WGPUController,
};
use wgpu::Features;
use winit::{
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

struct SketchApp<'w> {
  window: Arc<Window>,
  start_instant: Instant,
  last_frame_timestamp: f32,
  wgpu: WGPUController<'w>,
  surface_pixel_dimensions: [u32; 2],
}

impl<'w> SketchApp<'w> {
  async fn new(window: Window, features: Features) -> Self {
    let window_arc = Arc::new(window);
    let wgpu =
      WGPUController::new_with_features(window_arc.clone(), features).await;
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
    self.surface_pixel_dimensions = [width, height];
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
  let mut app = SketchApp::new(window, S::required_features()).await;
  let mut sketch = S::init(&app.wgpu);
  let mut mouse_pos = Some((0., 0.));
  let mut frame_index = 0;
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
              let min_dim = app.surface_pixel_dimensions[0]
                .min(app.surface_pixel_dimensions[1])
                as f32;
              sketch.update(
                &app.wgpu,
                surface_view,
                SketchData {
                  dimensions: app.surface_pixel_dimensions,
                  t: app.time(),
                  delta_t: app.delta_time(),
                  mouse_pos: mouse_pos.map(|mouse_pos| {
                    (
                      (app.surface_pixel_dimensions[0] as f32 / min_dim)
                        * ((2.
                          * (mouse_pos.0
                            / app.surface_pixel_dimensions[0] as f32))
                          - 1.),
                      (app.surface_pixel_dimensions[1] as f32 / min_dim)
                        * ((2.
                          * (mouse_pos.1
                            / app.surface_pixel_dimensions[1] as f32))
                          - 1.),
                    )
                  }),
                  frame_index,
                },
              );
              surface_texture.present();
              frame_index += 1;
            }
          }
        }
        WindowEvent::CursorMoved { position, .. } => {
          mouse_pos = Some((position.x as f32, position.y as f32));
        }
        WindowEvent::CursorLeft { .. } => {
          mouse_pos = None;
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
