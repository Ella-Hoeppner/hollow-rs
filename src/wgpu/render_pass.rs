use wgpu::{
  CommandEncoder, QuerySet, RenderPass, RenderPassColorAttachment,
  RenderPassDepthStencilAttachment, RenderPassTimestampWrites, TextureView,
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
  pub fn with_depth_stencil_attachment(
    mut self,
    attachment: RenderPassDepthStencilAttachment<'tex>,
  ) -> Self {
    self.depth_stencil_attachment = Some(attachment);
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
    self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: self.label,
      color_attachments: &self.color_attachments,
      depth_stencil_attachment: self.depth_stencil_attachment,
      occlusion_query_set: self.occlusion_query_set,
      timestamp_writes: self.timestamp_writes,
    })
  }
}
