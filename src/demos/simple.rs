use crate::{
  sketch::{FrameData, Sketch},
  wgpu::{
    bind::BindGroupWithLayout,
    buffer::{ArrayBuffer, Buffer},
    controller::WGPUController,
  },
};
use wgpu::{RenderPipeline, TextureView};

pub struct SimpleSketchInner {
  primary_bind_group: BindGroupWithLayout,
  corner_vertex_buffer: ArrayBuffer<[f32; 2]>,
  corner_index_buffer: ArrayBuffer<u16>,
  time_buffer: Buffer<f32>,
  dimensions_buffer: Buffer<[f32; 2]>,
  render_pipeline: RenderPipeline,
}

pub struct SimpleSketch(Option<SimpleSketchInner>);
impl SimpleSketch {
  pub fn new() -> Self {
    Self(None)
  }
}

impl Sketch for SimpleSketch {
  fn init(&mut self, wgpu: &WGPUController) {
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
    self.0 = Some(SimpleSketchInner {
      time_buffer,
      dimensions_buffer,
      primary_bind_group,
      corner_vertex_buffer,
      corner_index_buffer,
      render_pipeline,
    });
  }

  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    data: FrameData,
  ) {
    if let Self(Some(inner)) = self {
      wgpu
        .write_buffer(&inner.dimensions_buffer, data.dimensions)
        .write_buffer(&inner.time_buffer, data.t);
      wgpu.with_encoder(|encoder| {
        encoder
          .simple_render_pass(&surface_view)
          .with_bind_groups([&inner.primary_bind_group])
          .with_vertex_buffer(0, &inner.corner_vertex_buffer)
          .with_pipeline(&inner.render_pipeline)
          .draw_indexed_u16(&inner.corner_index_buffer, 0..6, 0, 0..1);
      });
    }
  }
}
