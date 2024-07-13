use crate::{
  sketch::Sketch,
  wgpu::{
    bind::BindGroupWithLayout,
    buffer::{ArrayBuffer, Buffer},
    controller::WGPUController,
    encoder::CommandEncoder,
  },
};
use wgpu::{RenderPipeline, TextureView};

pub struct SimpleSketch {
  primary_bind_group: BindGroupWithLayout,
  corner_vertex_buffer: ArrayBuffer<[f32; 2]>,
  corner_index_buffer: ArrayBuffer<u16>,
  time_buffer: Buffer<f32>,
  dimensions_buffer: Buffer<[f32; 2]>,
  render_pipeline: RenderPipeline,
}

impl Sketch for SimpleSketch {
  fn init(wgpu: &WGPUController) -> Self {
    let time_buffer = wgpu.buffer(0.);
    let dimensions_buffer = wgpu.buffer([0., 0.]);
    let corner_vertex_buffer =
      wgpu.array_buffer(&[[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]]);
    let corner_index_buffer = wgpu.array_buffer(&[2, 0, 1, 0, 2, 3]);
    let primary_bind_group = wgpu
      .build_bind_group_with_layout()
      .with_uniform_buffer_entry(&dimensions_buffer)
      .with_uniform_buffer_entry(&time_buffer)
      .build();
    let render_pipeline = wgpu
      .build_render_pipeline()
      .add_bind_group_layout(&primary_bind_group.layout)
      .add_vertex_buffer_layout(
        corner_vertex_buffer
          .vertex_layout(&wgpu::vertex_attr_array![0 => Float32x2]),
      )
      .build_with_shader(&wgpu.shader(wgpu::include_wgsl!("simple.wgsl")));
    Self {
      time_buffer,
      dimensions_buffer,
      primary_bind_group,
      corner_vertex_buffer,
      corner_index_buffer,
      render_pipeline,
    }
  }

  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    encoder: &mut CommandEncoder,
    dimensions: [usize; 2],
    t: f32,
    _delta_t: f32,
  ) {
    wgpu
      .write_buffer(&self.dimensions_buffer, dimensions)
      .write_buffer(&self.time_buffer, t);
    encoder
      .simple_render_pass(&surface_view)
      .with_bind_groups([&self.primary_bind_group])
      .with_vertex_buffer(0, &self.corner_vertex_buffer)
      .with_pipeline(&self.render_pipeline)
      .draw_indexed_u16(&self.corner_index_buffer, 0..6, 0, 0..1);
  }
}
