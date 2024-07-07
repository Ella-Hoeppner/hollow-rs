use std::ops::{Deref, DerefMut};

use wgpu::TextureView;

use super::{
  compute_pass::ComputePass,
  render_pass::{RenderPass, RenderPassBuilder},
};

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
  pub fn compute_pass(&mut self) -> ComputePass {
    ComputePass::new(self.begin_compute_pass(&wgpu::ComputePassDescriptor {
      label: None,
      timestamp_writes: None,
    }))
  }
  pub fn compute_pass_with_timestamp_writes<'a>(
    &mut self,
    timestamp_writes: wgpu::ComputePassTimestampWrites<'a>,
  ) -> ComputePass {
    ComputePass::new(self.begin_compute_pass(&wgpu::ComputePassDescriptor {
      label: None,
      timestamp_writes: Some(timestamp_writes),
    }))
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
