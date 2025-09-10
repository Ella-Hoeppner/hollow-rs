use std::collections::HashSet;

use wgpu::{Features, TextureView};
use winit::keyboard::SmolStr;

use crate::{app::run_sketch, wgpu::controller::WGPUController};

pub struct FrameData {
  pub t: f32,
  pub frame_index: usize,
  pub delta_t: f32,
  pub dimensions: [u32; 2],
  pub mouse_pos: Option<(f32, f32)>,
  pub mouse_down: bool,
  pub scroll_delta: (f32, f32),
  pub down_keys: HashSet<SmolStr>,
}

impl FrameData {
  pub fn is_key_down(&self, key: &str) -> bool {
    self.down_keys.contains(key)
  }
}

pub trait Sketch: Sized {
  fn init(&mut self, wgpu: &WGPUController);
  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    data: FrameData,
  );
  fn required_features() -> Features {
    Features::empty()
  }
  fn run(self) {
    pollster::block_on(run_sketch(self));
  }
  fn key_down(&mut self, _key: &str, _data: FrameData) {}
}
