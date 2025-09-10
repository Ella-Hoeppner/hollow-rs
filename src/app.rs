use std::{collections::HashSet, sync::Arc};

use take_mut::take;
use web_time::Instant;

use crate::{
  sketch::{FrameData, Sketch},
  wgpu::controller::WGPUController,
};
use wgpu::Features;
use winit::{
  application::ApplicationHandler,
  event::{ElementState, KeyEvent, MouseButton, WindowEvent},
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{Key, SmolStr},
  window::{Window, WindowId},
};

struct SketchApp<'w, S: Sketch> {
  sketch: S,
  mouse_pos: Option<(f32, f32)>,
  mouse_down: bool,
  frame_index: usize,
  window: Arc<Window>,
  start_instant: Instant,
  last_frame_timestamp: f32,
  wgpu: WGPUController<'w>,
  surface_pixel_dimensions: [u32; 2],
  scroll_delta: [f32; 2],
  down_keys: HashSet<SmolStr>,
}

impl<'w, S: Sketch> SketchApp<'w, S> {
  async fn new(mut sketch: S, window: Window, features: Features) -> Self {
    let window_arc = Arc::new(window);
    let wgpu =
      WGPUController::new_with_features(window_arc.clone(), features).await;
    sketch.init(&wgpu);
    Self {
      window: window_arc,
      start_instant: Instant::now(),
      last_frame_timestamp: 0.,
      wgpu,
      surface_pixel_dimensions: [0, 0],
      sketch,
      mouse_pos: Some((0., 0.)),
      frame_index: 0,
      scroll_delta: [0., 0.],
      mouse_down: false,
      down_keys: HashSet::new(),
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

enum SketchRunner<'w, S: Sketch> {
  Initialized(SketchApp<'w, S>),
  Uninitialized(S),
}

impl<S: Sketch> ApplicationHandler for SketchRunner<'_, S> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    take(self, |runner| {
      let window = event_loop
        .create_window(Window::default_attributes().with_title("hollow"))
        .unwrap();
      if let SketchRunner::Uninitialized(sketch) = runner {
        SketchRunner::Initialized(pollster::block_on(SketchApp::new(
          sketch,
          window,
          S::required_features(),
        )))
      } else {
        runner
      }
    })
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    if let SketchRunner::Initialized(app) = self {
      let frame = |app: &mut SketchApp<'_, S>| match app
        .wgpu
        .surface
        .get_current_texture()
      {
        Err(err) => Err(err),
        Ok(surface_texture) => {
          let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
          let min_dim = app.surface_pixel_dimensions[0]
            .min(app.surface_pixel_dimensions[1])
            as f32;
          Ok((
            surface_texture,
            surface_view,
            FrameData {
              dimensions: app.surface_pixel_dimensions,
              t: app.time(),
              delta_t: app.delta_time(),
              mouse_pos: app.mouse_pos.map(|mouse_pos| {
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
              frame_index: app.frame_index,
              scroll_delta: (app.scroll_delta[0], app.scroll_delta[1]),
              mouse_down: app.mouse_down,
              down_keys: app.down_keys.clone(),
            },
          ))
        }
      };
      match event {
        WindowEvent::CloseRequested => event_loop.exit(),
        WindowEvent::Resized(physical_size) => {
          app.resize(physical_size);
        }
        WindowEvent::RedrawRequested => {
          app.update();
          match frame(app) {
            Err(wgpu::SurfaceError::Lost) => {
              app.resize(app.window.inner_size())
            }
            Err(err) => panic!("{err:?}"),
            Ok((surface_texture, surface_view, frame_data)) => {
              app.sketch.update(&app.wgpu, surface_view, frame_data);
              surface_texture.present();
              app.frame_index += 1;
              app.scroll_delta = [0., 0.];
            }
          }
        }
        WindowEvent::CursorMoved { position, .. } => {
          app.mouse_pos = Some((position.x as f32, position.y as f32));
        }
        WindowEvent::MouseInput {
          state,
          button: MouseButton::Left,
          ..
        } => app.mouse_down = state == ElementState::Pressed,
        WindowEvent::CursorLeft { .. } => {
          app.mouse_pos = None;
        }
        WindowEvent::MouseWheel { delta, .. } => match delta {
          winit::event::MouseScrollDelta::LineDelta(x, y) => {
            app.scroll_delta[0] += x;
            app.scroll_delta[1] += y;
          }
          winit::event::MouseScrollDelta::PixelDelta(
            winit::dpi::PhysicalPosition { x, y },
          ) => {
            app.scroll_delta[0] += x as f32;
            app.scroll_delta[1] += y as f32;
          }
        },
        WindowEvent::KeyboardInput {
          event:
            KeyEvent {
              state: pressed_or_released,
              logical_key,
              ..
            },
          ..
        } => {
          if let Key::Character(char) = logical_key {
            match pressed_or_released {
              ElementState::Pressed => {
                app.down_keys.insert(char.clone());
                if let Ok((_, _, frame_data)) = frame(app) {
                  app.sketch.key_down(&char, frame_data);
                }
              }
              ElementState::Released => {
                app.down_keys.remove(&char);
              }
            }
          }
        }
        _ => {}
      }
    }
  }
  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    if let SketchRunner::Initialized(app) = self {
      app.window.request_redraw();
    }
  }
}

pub async fn run_sketch<S: Sketch>(sketch: S) {
  let mut runner: SketchRunner<'_, S> = SketchRunner::Uninitialized(sketch);
  EventLoop::new().unwrap().run_app(&mut runner).unwrap();
}
