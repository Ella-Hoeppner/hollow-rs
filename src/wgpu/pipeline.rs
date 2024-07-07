use std::num::NonZero;

use wgpu::{
  BindGroupLayout, ColorTargetState, DepthStencilState, FragmentState,
  MultisampleState, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
  ShaderModule, VertexBufferLayout, VertexState,
};

use super::controller::WGPUController;

pub struct RenderPipelineBuilder<'w, 'window, 's, 'v, 'b, 'shader> {
  wgpu: &'w WGPUController<'window>,
  label: Option<&'s str>,
  bind_group_layouts: Vec<&'b BindGroupLayout>,
  vertex_buffer_layouts: Vec<VertexBufferLayout<'v>>,
  vertex: Option<VertexState<'shader>>,
  fragment: Option<FragmentState<'shader>>,
  primitive: Option<PrimitiveState>,
  depth_stencil: Option<DepthStencilState>,
  multisample: Option<MultisampleState>,
  multiview: Option<NonZero<u32>>,
}

impl<'w, 'window, 's, 'v, 'b, 'shader>
  RenderPipelineBuilder<'w, 'window, 's, 'v, 'b, 'shader>
{
  pub fn new(wgpu: &'w WGPUController<'window>) -> Self {
    Self {
      wgpu,
      label: None,
      bind_group_layouts: vec![],
      vertex_buffer_layouts: vec![],
      vertex: None,
      fragment: None,
      primitive: None,
      depth_stencil: None,
      multisample: None,
      multiview: None,
    }
  }
  pub fn with_label(mut self, label: &'s str) -> Self {
    self.label = Some(label);
    self
  }
  pub fn add_bind_group_layout(mut self, layout: &'b BindGroupLayout) -> Self {
    self.bind_group_layouts.push(layout);
    self
  }
  pub fn with_vertex(mut self, vertex: VertexState<'shader>) -> Self {
    self.vertex = Some(vertex);
    self
  }
  pub fn with_fragment(mut self, fragment: FragmentState<'shader>) -> Self {
    self.fragment = Some(fragment);
    self
  }
  pub fn with_primitive(mut self, primitive: PrimitiveState) -> Self {
    self.primitive = Some(primitive);
    self
  }
  pub fn with_depth_stencil(
    mut self,
    depth_stencil: DepthStencilState,
  ) -> Self {
    self.depth_stencil = Some(depth_stencil);
    self
  }
  pub fn with_multisample(mut self, multisample: MultisampleState) -> Self {
    self.multisample = Some(multisample);
    self
  }
  pub fn with_multiview(mut self, multiview: NonZero<u32>) -> Self {
    self.multiview = Some(multiview);
    self
  }
  pub fn add_vertex_buffer_layout<V: Into<VertexBufferLayout<'v>>>(
    mut self,
    layout: V,
  ) -> Self {
    self.vertex_buffer_layouts.push(layout.into());
    println!("{:?}", self.vertex_buffer_layouts.len());
    self
  }
  pub fn build_with_shader_entry_points<'vs, 'fs>(
    self,
    shader: &ShaderModule,
    vertex: &'vs str,
    fragment: Option<&'fs str>,
  ) -> RenderPipeline {
    let fragment_targets = &[Some(ColorTargetState {
      format: self.wgpu.config.format,
      blend: Some(wgpu::BlendState::REPLACE),
      write_mask: wgpu::ColorWrites::ALL,
    })];
    self
      .wgpu
      .device
      .create_render_pipeline(&RenderPipelineDescriptor {
        label: self.label,
        layout: Some(&self.wgpu.device.create_pipeline_layout(
          &wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &[], // todo! handle these
          },
        )),
        vertex: wgpu::VertexState {
          module: &shader,
          entry_point: vertex,
          buffers: &self.vertex_buffer_layouts,
        },
        fragment: fragment.map(|fragment| FragmentState {
          module: &shader,
          entry_point: fragment,
          targets: fragment_targets,
        }),
        primitive: self.primitive.unwrap_or(wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::TriangleList,
          strip_index_format: None,
          front_face: wgpu::FrontFace::Ccw,
          cull_mode: Some(wgpu::Face::Back),
          polygon_mode: wgpu::PolygonMode::Fill,
          unclipped_depth: false,
          conservative: false,
        }),
        depth_stencil: self.depth_stencil,
        multisample: self.multisample.unwrap_or(wgpu::MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: false,
        }),
        multiview: self.multiview,
      })
  }
  pub fn build_with_shader(self, shader: &ShaderModule) -> RenderPipeline {
    self.build_with_shader_entry_points(shader, "vertex", Some("fragment"))
  }
  pub fn build(self) -> RenderPipeline {
    self
      .wgpu
      .device
      .create_render_pipeline(&RenderPipelineDescriptor {
        label: self.label,
        layout: Some(&self.wgpu.device.create_pipeline_layout(
          &wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &[], // todo! handle these
          },
        )),
        vertex: self.vertex.expect(
          "tried to build RenderPipeline with no vertex stage (did you forget \
          to call with_vertex(), or use build_with_shader() instead?)",
        ),
        fragment: self.fragment,
        primitive: self.primitive.unwrap_or(wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::TriangleList,
          strip_index_format: None,
          front_face: wgpu::FrontFace::Ccw,
          cull_mode: Some(wgpu::Face::Back),
          polygon_mode: wgpu::PolygonMode::Fill,
          unclipped_depth: false,
          conservative: false,
        }),
        depth_stencil: self.depth_stencil,
        multisample: self.multisample.unwrap_or(wgpu::MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: false,
        }),
        multiview: self.multiview,
      })
  }
}
