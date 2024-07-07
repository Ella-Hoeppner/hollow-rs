use std::ops::{Deref, DerefMut};

use wgpu::TextureView;

use super::render_pass::{RenderPass, RenderPassBuilder};

pub struct CommandEncoder {
  pub encoder: wgpu::CommandEncoder,
}

impl CommandEncoder {
  pub fn new(encoder: wgpu::CommandEncoder) -> Self {
    Self { encoder }
  }
  pub fn build_render_pass(&mut self) -> RenderPassBuilder {
    RenderPassBuilder::new(self)
  }
  pub fn simple_render_pass<'a>(
    &'a mut self,
    view: &'a TextureView,
  ) -> RenderPass<'a> {
    self
      .build_render_pass()
      .add_simple_color_attachment(view)
      .build()
  }
}

impl Deref for CommandEncoder {
  type Target = wgpu::CommandEncoder;

  fn deref(&self) -> &Self::Target {
    &self.encoder
  }
}

impl DerefMut for CommandEncoder {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.encoder
  }
}
