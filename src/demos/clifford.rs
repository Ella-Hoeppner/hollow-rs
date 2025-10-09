use crate::{
  sketch::{FrameData, Sketch},
  wgpu::{
    bind::BindGroupWithLayout,
    buffer::{ArrayBuffer, Buffer},
    controller::WGPUController,
  },
};
use rand::Rng;
use wgpu::{include_wgsl, ComputePipeline, RenderPipeline, TextureView};

const PARAMS: &[(&str, f64)] =
  &[("A", -1.4), ("B", 1.6), ("C", 1.), ("D", 0.7)];

const POINT_GROUP_MULTIPLE: usize = 1000;
const POINTS: usize = 256 * POINT_GROUP_MULTIPLE;

pub struct CliffordSketchInner {
  uniform_bind_group: BindGroupWithLayout,
  render_points_bind_group: BindGroupWithLayout,
  compute_bind_group: BindGroupWithLayout,
  corner_vertex_buffer: ArrayBuffer<[f32; 2]>,
  scale_buffer: Buffer<[f32; 2]>,
  render_pipeline: RenderPipeline,
  compute_pipeline: ComputePipeline,
}
pub struct CliffordSketch(Option<CliffordSketchInner>);
impl CliffordSketch {
  pub fn new() -> Self {
    Self(None)
  }
}

impl Sketch for CliffordSketch {
  fn init(&mut self, wgpu: &WGPUController) {
    let scale_buffer = wgpu.buffer([0., 0.]);
    let mut rng = rand::rng();
    let point_buffer = wgpu.array_buffer(
      &std::iter::repeat_with(|| {
        [rng.random::<f32>() * 2. - 1., rng.random::<f32>() * 2. - 1.]
      })
      .take(POINTS)
      .collect::<Vec<_>>(),
    );
    let corner_vertex_buffer = wgpu.array_buffer(&[
      [1., 1.],
      [-1., -1.],
      [1., -1.],
      [-1., -1.],
      [1., 1.],
      [-1., 1.],
    ]);
    let uniform_bind_group = wgpu
      .build_bind_group_with_layout()
      .with_uniform_buffer_entry(&scale_buffer)
      .build();
    let render_points_bind_group = wgpu
      .build_bind_group_with_layout()
      .with_read_only_storage_buffer_entry(&point_buffer)
      .build();
    let compute_bind_group = wgpu
      .build_bind_group_with_layout()
      .with_compute_writable_storage_buffer_entry(&point_buffer)
      .build();
    let render_pipeline = wgpu
      .build_render_pipeline()
      .add_bind_group_layout(&uniform_bind_group.layout)
      .add_bind_group_layout(&render_points_bind_group.layout)
      .add_vertex_buffer_layout(
        corner_vertex_buffer
          .vertex_layout(&wgpu::vertex_attr_array![0 => Float32x2]),
      )
      .build_with_shader(
        &wgpu.shader(wgpu::include_wgsl!("clifford_render.wgsl")),
      );
    let compute_pipeline = wgpu
      .build_compute_pipeline()
      .add_bind_group_layout(&compute_bind_group.layout)
      .with_override_constants(&PARAMS)
      .build_with_shader(&wgpu.shader(include_wgsl!("clifford_compute.wgsl")));
    self.0 = Some(CliffordSketchInner {
      scale_buffer,
      uniform_bind_group,
      render_points_bind_group,
      compute_bind_group,
      corner_vertex_buffer,
      render_pipeline,
      compute_pipeline,
    });
  }

  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    data: FrameData,
  ) {
    if let CliffordSketch(Some(inner)) = self {
      let dim_min = data.dimensions[0].min(data.dimensions[1]) as f32;
      wgpu.write_buffer(
        &inner.scale_buffer,
        [
          dim_min / data.dimensions[0] as f32,
          dim_min / data.dimensions[1] as f32,
        ],
      );
      wgpu.with_encoder(|encoder| {
        encoder
          .compute_pass()
          .with_pipeline(&inner.compute_pipeline)
          .with_bind_groups([&inner.compute_bind_group])
          .dispatch(POINT_GROUP_MULTIPLE as u32, 1, 1);
        encoder
          .simple_render_pass(&surface_view)
          .with_bind_groups([
            &inner.uniform_bind_group,
            &inner.render_points_bind_group,
          ])
          .with_vertex_buffer(0, &inner.corner_vertex_buffer)
          .with_pipeline(&inner.render_pipeline)
          .draw(0..6, 0..POINTS as u32);
      })
    }
  }
}
