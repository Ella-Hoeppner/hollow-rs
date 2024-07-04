use hollow_rs::{
  sketch::Sketch,
  wgpu::{
    bind::BindGroupLayoutDescriptorBuilder, buffer::Buffer,
    controller::WGPUController,
  },
};
use wgpu::{
  BindGroup, CommandEncoder, RenderPipeline, RenderPipelineDescriptor,
  TextureView,
};

struct SimpleSketch {
  primary_bind_group: BindGroup,
  corner_vertex_buffer: Buffer<[f32; 2]>,
  corner_index_buffer: Buffer<u16>,
  time_buffer: Buffer<f32>,
  dimensions_buffer: Buffer<[f32; 2]>,
  background_pipeline: RenderPipeline,
}

impl Sketch for SimpleSketch {
  fn init(wgpu: &WGPUController) -> Self {
    let shader = wgpu
      .device
      .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let time_buffer = wgpu.buffer(&[0.]);
    let dimensions_buffer = wgpu.buffer(&[[0., 0.]]);
    let corner_vertex_buffer =
      wgpu.buffer(&[[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]]);
    let corner_index_buffer = wgpu
      .build_buffer(&[2, 0, 1, 0, 2, 3])
      .with_usage(wgpu::BufferUsages::INDEX)
      .build();
    let primary_bind_group_layout = wgpu.create_bind_group_layout(
      &mut BindGroupLayoutDescriptorBuilder::new()
        .with_default_entry()
        .with_default_entry(),
    );
    let primary_bind_group = primary_bind_group_layout
      .build_group()
      .with_buffer_entry(&dimensions_buffer)
      .with_buffer_entry(&time_buffer)
      .build(wgpu);
    let corner_vertex_buffer_layout = wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &wgpu::vertex_attr_array![0 => Float32x2],
    };
    let background_pipeline_layout =
      wgpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
          label: Some("Background Render Pipeline Layout"),
          bind_group_layouts: &[&primary_bind_group_layout],
          push_constant_ranges: &[],
        });
    let background_pipeline =
      wgpu
        .device
        .create_render_pipeline(&RenderPipelineDescriptor {
          label: Some("Background Render Pipeline"),
          layout: Some(&background_pipeline_layout),
          vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex",
            buffers: &[corner_vertex_buffer_layout.clone()],
          },
          fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment",
            targets: &[Some(wgpu::ColorTargetState {
              format: wgpu.config.format,
              blend: Some(wgpu::BlendState::REPLACE),
              write_mask: wgpu::ColorWrites::ALL,
            })],
          }),
          primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
          },
          depth_stencil: None,
          multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
          },
          multiview: None,
        });
    Self {
      time_buffer,
      dimensions_buffer,
      primary_bind_group,
      corner_vertex_buffer,
      corner_index_buffer,
      background_pipeline,
    }
  }
  fn update(
    self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    encoder: &mut CommandEncoder,
    surface_pixel_dimensions: (usize, usize),
    t: f32,
  ) -> Self {
    wgpu.queue.write_buffer(
      &self.dimensions_buffer,
      0,
      bytemuck::cast_slice(&[
        surface_pixel_dimensions.0 as f32,
        surface_pixel_dimensions.1 as f32,
      ]),
    );
    wgpu
      .queue
      .write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&[t]));

    {
      let mut render_pass =
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("Main Render Pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &surface_view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            },
          })],
          depth_stencil_attachment: None,
          occlusion_query_set: None,
          timestamp_writes: None,
        });
      render_pass.set_bind_group(0, &self.primary_bind_group, &[]);
      render_pass.set_index_buffer(
        self.corner_index_buffer.slice(..),
        wgpu::IndexFormat::Uint16,
      );
      render_pass.set_vertex_buffer(0, self.corner_vertex_buffer.slice(..));
      render_pass.set_pipeline(&self.background_pipeline);
      render_pass.draw_indexed(0..6, 0, 0..1);
    }
    self
  }
}

fn main() {
  SimpleSketch::run();
}
