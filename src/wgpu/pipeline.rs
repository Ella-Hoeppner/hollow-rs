use std::num::NonZero;

use wgpu::{
  BindGroupLayout, BlendState, ColorTargetState, ComputePipeline,
  DepthStencilState, FragmentState, MultisampleState,
  PipelineCompilationOptions, PrimitiveState, RenderPipeline,
  RenderPipelineDescriptor, ShaderModule, TextureFormat, VertexBufferLayout,
  VertexState,
};

use super::controller::WGPUController;

pub struct RenderPipelineBuilder<'w, 'window, 's, 'v, 'b, 'p, 'shader> {
  wgpu: &'w WGPUController<'window>,
  label: Option<&'s str>,
  bind_group_layouts: Vec<&'b BindGroupLayout>,
  vertex_buffer_layouts: Vec<VertexBufferLayout<'v>>,
  vertex: Option<VertexState<'shader>>,
  fragment: Option<FragmentState<'shader>>,
  primitive: Option<PrimitiveState>,
  depth_stencil: Option<DepthStencilState>,
  multisample: Option<MultisampleState>,
  blend_state: Option<BlendState>,
  multiview: Option<NonZero<u32>>,
  texture_format: Option<TextureFormat>,
  fragment_compilation_options: Option<PipelineCompilationOptions<'p>>,
  vertex_compilation_options: Option<PipelineCompilationOptions<'p>>,
}

impl<'w, 'window, 's, 'v, 'b, 'p, 'shader>
  RenderPipelineBuilder<'w, 'window, 's, 'v, 'b, 'p, 'shader>
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
      blend_state: None,
      texture_format: None,
      fragment_compilation_options: None,
      vertex_compilation_options: None,
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
  pub fn with_blend_state(mut self, blend_state: BlendState) -> Self {
    self.blend_state = Some(blend_state);
    self
  }
  pub fn with_texture_format(mut self, texture_format: TextureFormat) -> Self {
    self.texture_format = Some(texture_format);
    self
  }
  pub fn add_vertex_buffer_layout<V: Into<VertexBufferLayout<'v>>>(
    mut self,
    layout: V,
  ) -> Self {
    self.vertex_buffer_layouts.push(layout.into());
    self
  }
  pub fn with_fragment_compilation_options(
    mut self,
    options: PipelineCompilationOptions<'p>,
  ) -> Self {
    self.fragment_compilation_options = Some(options);
    self
  }
  pub fn with_vertex_compilation_options(
    mut self,
    options: PipelineCompilationOptions<'p>,
  ) -> Self {
    self.vertex_compilation_options = Some(options);
    self
  }
  pub fn with_compilation_options(
    self,
    options: PipelineCompilationOptions<'p>,
  ) -> Self {
    self
      .with_fragment_compilation_options(options.clone())
      .with_vertex_compilation_options(options)
  }
  pub fn with_override_constants(self, constants: &'p [(&str, f64)]) -> Self {
    self.with_compilation_options(PipelineCompilationOptions {
      constants: constants,
      zero_initialize_workgroup_memory: true,
    })
  }
  pub fn build_with_shader_entry_points<'vs, 'fs>(
    self,
    shader: &ShaderModule,
    vertex_entry_point: Option<&'vs str>,
    fragment_entry_point: Option<Option<&'fs str>>,
  ) -> RenderPipeline {
    let fragment_targets = &[Some(ColorTargetState {
      format: self.texture_format.unwrap_or(self.wgpu.config.format),
      blend: self.blend_state,
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
            push_constant_ranges: &[],
          },
        )),
        vertex: wgpu::VertexState {
          module: &shader,
          entry_point: vertex_entry_point,
          buffers: &self.vertex_buffer_layouts,
          compilation_options: self
            .vertex_compilation_options
            .unwrap_or_default(),
        },
        fragment: fragment_entry_point.map(|fragment| FragmentState {
          module: &shader,
          entry_point: fragment,
          targets: fragment_targets,
          compilation_options: self
            .fragment_compilation_options
            .unwrap_or_default(),
        }),
        primitive: self.primitive.unwrap_or(wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::TriangleList,
          strip_index_format: None,
          front_face: wgpu::FrontFace::Ccw,
          cull_mode: None,
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
        cache: None,
      })
  }
  pub fn build_with_shader(self, shader: &ShaderModule) -> RenderPipeline {
    self.build_with_shader_entry_points(
      shader,
      Some("vertex"),
      Some(Some("fragment")),
    )
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
            push_constant_ranges: &[],
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
          cull_mode: None,
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
        cache: None,
      })
  }
}

pub struct ComputePipelineBuilder<'w, 'p, 'window, 's, 'b> {
  wgpu: &'w WGPUController<'window>,
  label: Option<&'s str>,
  bind_group_layouts: Vec<&'b BindGroupLayout>,
  compilation_options: Option<PipelineCompilationOptions<'p>>,
}

impl<'w, 'p, 'window, 's, 'b> ComputePipelineBuilder<'w, 'p, 'window, 's, 'b> {
  pub fn new(wgpu: &'w WGPUController<'window>) -> Self {
    Self {
      wgpu,
      label: None,
      bind_group_layouts: vec![],
      compilation_options: None,
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
  pub fn build_with_shader_entry_point<'cs>(
    self,
    shader: &ShaderModule,
    entry_point: Option<&'cs str>,
  ) -> ComputePipeline {
    self
      .wgpu
      .device
      .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: self.label,
        layout: Some(&self.wgpu.device.create_pipeline_layout(
          &wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &[],
          },
        )),
        module: shader,
        entry_point: entry_point,
        compilation_options: self.compilation_options.unwrap_or_default(),
        cache: None,
      })
  }
  pub fn build_with_shader(self, shader: &ShaderModule) -> ComputePipeline {
    self.build_with_shader_entry_point(shader, None)
  }
  pub fn with_compilation_options(
    mut self,
    options: PipelineCompilationOptions<'p>,
  ) -> Self {
    self.compilation_options = Some(options);
    self
  }
  pub fn with_override_constants(self, constants: &'p [(&str, f64)]) -> Self {
    self.with_compilation_options(PipelineCompilationOptions {
      constants: constants,
      zero_initialize_workgroup_memory: true,
    })
  }
}
