use hollow_rs::{
  sketch::Sketch,
  wgpu::{
    bind::BindGroupLayoutDescriptorBuilder, buffer::Buffer,
    controller::WGPUController,
  },
};
use wgpu::{BindGroup, CommandEncoder, RenderPipeline, TextureView};

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
    let time_buffer = wgpu.buffer(0.);
    let dimensions_buffer = wgpu.buffer([0., 0.]);
    let corner_vertex_buffer =
      wgpu.array_buffer(&[[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]]);
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
    let corner_vertex_buffer_layout = corner_vertex_buffer
      .vertex_layout(&wgpu::vertex_attr_array![0 => Float32x2]);
    let background_pipeline = wgpu
      .build_render_pipeline()
      .add_bind_group_layout(&primary_bind_group_layout)
      .build_with_shader(&shader, &[corner_vertex_buffer_layout.clone()]);
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
    dimensions: (usize, usize),
    t: f32,
  ) -> Self {
    wgpu.write_buffer(
      &self.dimensions_buffer,
      [dimensions.0 as f32, dimensions.1 as f32],
    );
    wgpu.write_buffer(&self.time_buffer, t);
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
