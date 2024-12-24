use wgpu::{Features, TextureView};

use crate::{app::run_sketch, wgpu::controller::WGPUController};

pub struct SketchData {
  pub t: f32,
  pub frame_index: usize,
  pub delta_t: f32,
  pub dimensions: [u32; 2],
  pub mouse_pos: Option<(f32, f32)>,
}

pub trait Sketch: Sized {
  fn init(wgpu: &WGPUController) -> Self;
  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    data: SketchData,
  );
  fn required_features() -> Features {
    Features::empty()
  }
  fn run() {
    pollster::block_on(run_sketch::<Self>());
  }
}
