use wgpu::{CommandEncoder, TextureView};

use crate::wgpu::WGPUController;

pub trait Sketch {
  fn start(wgpu: &WGPUController) -> Self;
  fn update(
    self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    encoder: &mut CommandEncoder,
    surface_pixel_dimensions: (usize, usize),
    t: f32,
  ) -> Self;
}
