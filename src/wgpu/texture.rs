use wgpu::{
  Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat,
  TextureUsages,
};

use super::controller::WGPUController;

pub struct TextureBuilder<'s, 'w, 'window> {
  wgpu: &'w WGPUController<'window>,
  label: Option<&'s str>,
  size: Extent3d,
  mip_level_count: u32,
  sample_count: u32,
  dimension: TextureDimension,
  format: TextureFormat,
  usage: TextureUsages,
  view_formats: Vec<TextureFormat>,
}

impl<'s, 'w, 'window> TextureBuilder<'s, 'w, 'window> {
  pub fn new_2d(
    wgpu: &'w WGPUController<'window>,
    width: u32,
    height: u32,
  ) -> Self {
    Self {
      wgpu,
      label: None,
      size: Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      usage: wgpu::TextureUsages::TEXTURE_BINDING
        | wgpu::TextureUsages::RENDER_ATTACHMENT,
      view_formats: vec![],
    }
  }
  pub fn with_label(mut self, label: &'s str) -> Self {
    self.label = Some(label);
    self
  }
  pub fn with_mip_level_count(mut self, mip_level_count: u32) -> Self {
    self.mip_level_count = mip_level_count;
    self
  }
  pub fn with_sample_count(mut self, sample_count: u32) -> Self {
    self.sample_count = sample_count;
    self
  }
  pub fn with_format(mut self, format: TextureFormat) -> Self {
    self.format = format;
    self
  }
  pub fn with_usage(mut self, usage: TextureUsages) -> Self {
    self.usage = usage;
    self
  }
  pub fn with_view_formats(mut self, view_formats: Vec<TextureFormat>) -> Self {
    self.view_formats = view_formats;
    self
  }
  pub fn add_view_format(mut self, view_format: TextureFormat) -> Self {
    self.view_formats.push(view_format);
    self
  }
  pub fn build(self) -> Texture {
    self.wgpu.device.create_texture(&TextureDescriptor {
      label: self.label,
      size: self.size,
      mip_level_count: self.mip_level_count,
      sample_count: self.sample_count,
      dimension: self.dimension,
      format: self.format,
      usage: self.usage,
      view_formats: &self.view_formats,
    })
  }
}
