use std::ops::{Deref, DerefMut, Range};

use wgpu::{
  BufferSlice, Color, QuerySet, RenderPassColorAttachment,
  RenderPassDepthStencilAttachment, RenderPassTimestampWrites, TextureView,
};

use super::{
  buffer::{ArrayBuffer, Buffer, IntoVertexBufferData},
  encoder::CommandEncoder,
};

pub struct RenderPassBuilder<'e, 's, 'query, 'tex, 'desc> {
  encoder: &'e mut CommandEncoder,
  label: Option<&'s str>,
  color_attachments: Vec<Option<RenderPassColorAttachment<'tex>>>,
  depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'tex>>,
  occlusion_query_set: Option<&'query QuerySet>,
  timestamp_writes: Option<RenderPassTimestampWrites<'desc>>,
}

impl<'e, 's, 'query: 'e, 'tex: 'e, 'desc>
  RenderPassBuilder<'e, 's, 'query, 'tex, 'desc>
{
  pub fn new(encoder: &'e mut CommandEncoder) -> Self {
    Self {
      encoder,
      label: None,
      color_attachments: vec![],
      depth_stencil_attachment: None,
      occlusion_query_set: None,
      timestamp_writes: None,
    }
  }
  pub fn with_label(mut self, label: &'s str) -> Self {
    self.label = Some(label);
    self
  }
  pub fn add_color_attachment(
    mut self,
    attachment: Option<RenderPassColorAttachment<'tex>>,
  ) -> Self {
    self.color_attachments.push(attachment);
    self
  }
  pub fn add_simple_color_attachment<'v: 'e + 'tex>(
    self,
    view: &'v TextureView,
  ) -> Self {
    self.add_color_attachment(Some(wgpu::RenderPassColorAttachment {
      view,
      resolve_target: None,
      ops: wgpu::Operations {
        load: wgpu::LoadOp::Load,
        store: wgpu::StoreOp::Store,
      },
    }))
  }
  pub fn add_clearing_color_attachment<'v: 'e + 'tex>(
    self,
    view: &'v TextureView,
    color: Color,
  ) -> Self {
    self.add_color_attachment(Some(wgpu::RenderPassColorAttachment {
      view,
      resolve_target: None,
      ops: wgpu::Operations {
        load: wgpu::LoadOp::Clear(color),
        store: wgpu::StoreOp::Store,
      },
    }))
  }
  pub fn with_depth_stencil_attachment(
    mut self,
    attachment: RenderPassDepthStencilAttachment<'tex>,
  ) -> Self {
    self.depth_stencil_attachment = Some(attachment);
    self
  }
  pub fn with_optional_depth_stencil_attachment(
    mut self,
    attachment: Option<RenderPassDepthStencilAttachment<'tex>>,
  ) -> Self {
    self.depth_stencil_attachment = attachment;
    self
  }
  pub fn with_occlusion_query_set(mut self, set: &'query QuerySet) -> Self {
    self.occlusion_query_set = Some(set);
    self
  }
  pub fn with_timestamp_writes(
    mut self,
    writes: RenderPassTimestampWrites<'desc>,
  ) -> Self {
    self.timestamp_writes = Some(writes);
    self
  }
  pub fn build(self) -> RenderPass<'e> {
    RenderPass::new(self.encoder.begin_render_pass(
      &wgpu::RenderPassDescriptor {
        label: self.label,
        color_attachments: &self.color_attachments,
        depth_stencil_attachment: self.depth_stencil_attachment,
        occlusion_query_set: self.occlusion_query_set,
        timestamp_writes: self.timestamp_writes,
      },
    ))
  }
}

pub struct RenderPass<'p> {
  pass: wgpu::RenderPass<'p>,
}
impl<'p> RenderPass<'p> {
  pub fn new(pass: wgpu::RenderPass<'p>) -> Self {
    Self { pass }
  }
  pub fn set_index_buffer_u16(
    &mut self,
    data: impl IntoIndexBufferDataU16<'p>,
  ) {
    self.pass.set_index_buffer(
      data.into_index_buffer_data_u16(),
      wgpu::IndexFormat::Uint16,
    )
  }
  pub fn with_index_buffer_u16(
    mut self,
    data: impl IntoIndexBufferDataU16<'p>,
  ) -> Self {
    self.set_index_buffer_u16(data);
    self
  }
  pub fn set_index_buffer_u32(
    &mut self,
    data: impl IntoIndexBufferDataU32<'p>,
  ) {
    self.pass.set_index_buffer(
      data.into_index_buffer_data_u32(),
      wgpu::IndexFormat::Uint32,
    )
  }
  pub fn with_index_buffer_u32(
    mut self,
    data: impl IntoIndexBufferDataU32<'p>,
  ) -> Self {
    self.set_index_buffer_u32(data);
    self
  }
  pub fn with_stencil_reference(mut self, reference: u32) -> Self {
    self.set_stencil_reference(reference);
    self
  }
  pub fn with_offset_bind_group(
    mut self,
    index: u32,
    bind_group: &'p wgpu::BindGroup,
    offsets: &[wgpu::DynamicOffset],
  ) -> Self {
    self.set_bind_group(index, bind_group, offsets);
    self
  }
  pub fn with_bind_group(
    self,
    index: u32,
    bind_group: &'p wgpu::BindGroup,
  ) -> Self {
    self.with_offset_bind_group(index, bind_group, &[])
  }
  pub fn with_bind_groups<const N: usize>(
    self,
    bind_groups: [&'p wgpu::BindGroup; N],
  ) -> Self {
    bind_groups
      .iter()
      .enumerate()
      .fold(self, |pass, (i, group)| {
        pass.with_bind_group(i as u32, group)
      })
  }
  pub fn with_vertex_buffer<'s: 'p>(
    mut self,
    slot: u32,
    buffer_slice: impl IntoVertexBufferData<'s>,
  ) -> Self {
    self.set_vertex_buffer(slot, buffer_slice.into_vertex_buffer_data());
    self
  }
  pub fn with_pipeline<'a: 'p>(
    mut self,
    pipeline: &'a wgpu::RenderPipeline,
  ) -> Self {
    self.set_pipeline(pipeline);
    self
  }
  pub fn draw_indexed_u16(
    mut self,
    index_data: impl IntoIndexBufferDataU16<'p>,
    indices: Range<u32>,
    base_vertex: i32,
    instances: Range<u32>,
  ) -> Self {
    self.set_index_buffer_u16(index_data);
    self.draw_indexed(indices, base_vertex, instances);
    self
  }
  pub fn draw(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
    self.pass.draw(vertices, instances);
    self
  }
}

pub trait IntoIndexBufferDataU16<'s> {
  fn into_index_buffer_data_u16(self) -> BufferSlice<'s>;
}
impl<'s> IntoIndexBufferDataU16<'s> for BufferSlice<'s> {
  fn into_index_buffer_data_u16(self) -> BufferSlice<'s> {
    self
  }
}
impl<'s, 'b: 's> IntoIndexBufferDataU16<'s> for &'b Buffer<u16> {
  fn into_index_buffer_data_u16(self) -> BufferSlice<'s> {
    self.slice(..)
  }
}
impl<'s, 'b: 's> IntoIndexBufferDataU16<'s> for &'b ArrayBuffer<u16> {
  fn into_index_buffer_data_u16(self) -> BufferSlice<'s> {
    self.slice(..)
  }
}
pub trait IntoIndexBufferDataU32<'s> {
  fn into_index_buffer_data_u32(self) -> BufferSlice<'s>;
}
impl<'s> IntoIndexBufferDataU32<'s> for BufferSlice<'s> {
  fn into_index_buffer_data_u32(self) -> BufferSlice<'s> {
    self
  }
}
impl<'s, 'b: 's> IntoIndexBufferDataU32<'s> for &'b Buffer<u32> {
  fn into_index_buffer_data_u32(self) -> BufferSlice<'s> {
    self.slice(..)
  }
}
impl<'s, 'b: 's> IntoIndexBufferDataU32<'s> for &'b ArrayBuffer<u32> {
  fn into_index_buffer_data_u32(self) -> BufferSlice<'s> {
    self.slice(..)
  }
}
impl<'s> Deref for RenderPass<'s> {
  type Target = wgpu::RenderPass<'s>;

  fn deref(&self) -> &Self::Target {
    &self.pass
  }
}
impl<'s> DerefMut for RenderPass<'s> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.pass
  }
}
