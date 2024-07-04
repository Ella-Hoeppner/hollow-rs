use wgpu::{CommandEncoder, TextureView};

use crate::{app::run_sketch, wgpu::WGPUController};

pub trait Sketch: Sized {
  fn init(wgpu: &WGPUController) -> Self;
  fn update(
    self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    encoder: &mut CommandEncoder,
    surface_pixel_dimensions: (usize, usize),
    t: f32,
  ) -> Self;
  fn run() {
    pollster::block_on(run_sketch::<Self>());
  }
}
