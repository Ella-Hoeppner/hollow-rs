use wgpu::TextureView;

use crate::{app::run_sketch, wgpu::controller::WGPUController};

pub trait Sketch: Sized {
  fn init(wgpu: &WGPUController) -> Self;
  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    surface_pixel_dimensions: [usize; 2],
    t: f32,
    delta_t: f32,
  );
  fn run() {
    pollster::block_on(run_sketch::<Self>());
  }
}
