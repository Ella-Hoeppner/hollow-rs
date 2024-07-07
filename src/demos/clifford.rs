use crate::{
  sketch::Sketch,
  wgpu::{
    bind::BindGroupWithLayout, buffer::Buffer, controller::WGPUController,
    encoder::CommandEncoder,
  },
};
use rand::Rng;
use wgpu::{RenderPipeline, TextureView};

const POINTS: usize = 10;

pub struct CliffordSketch {
  uniform_bind_group: BindGroupWithLayout,
  render_points_bind_group: BindGroupWithLayout,
  corner_vertex_buffer: Buffer<[f32; 2]>,
  scale_buffer: Buffer<[f32; 2]>,
  point_buffer: Buffer<[f32; 2]>,
  background_pipeline: RenderPipeline,
}

impl Sketch for CliffordSketch {
  fn init(wgpu: &WGPUController) -> Self {
    let scale_buffer = wgpu.buffer([0., 0.]);
    let mut rng = rand::thread_rng();
    let initial_points: [[f32; 2]; POINTS] = std::iter::repeat_with(|| {
      [rng.gen::<f32>() * 2. - 1., rng.gen::<f32>() * 2. - 1.]
    })
    .take(POINTS)
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();
    let point_buffer = wgpu.array_buffer(&initial_points);
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
      .with_storage_buffer_entry(&point_buffer, true)
      .build();
    let background_pipeline = wgpu
      .build_render_pipeline()
      .add_bind_group_layout(&uniform_bind_group.layout)
      .add_bind_group_layout(&render_points_bind_group.layout)
      .add_vertex_buffer_layout(
        corner_vertex_buffer
          .vertex_layout(&wgpu::vertex_attr_array![0 => Float32x2]),
      )
      .build_with_shader(&wgpu.shader(wgpu::include_wgsl!("clifford.wgsl")));
    Self {
      scale_buffer,
      point_buffer,
      uniform_bind_group,
      render_points_bind_group,
      corner_vertex_buffer,
      background_pipeline,
    }
  }

  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    encoder: &mut CommandEncoder,
    dimensions: [usize; 2],
    _t: f32,
    _delta_t: f32,
  ) {
    let dim_min = dimensions[0].min(dimensions[1]) as f32;
    wgpu.write_buffer(
      &self.scale_buffer,
      [
        dim_min / dimensions[0] as f32,
        dim_min / dimensions[1] as f32,
      ],
    );
    encoder
      .simple_render_pass(&surface_view)
      .with_bind_groups([
        &self.uniform_bind_group,
        &self.render_points_bind_group,
      ])
      .with_vertex_buffer(0, &self.corner_vertex_buffer)
      .with_pipeline(&self.background_pipeline)
      .draw(0..6, 0..POINTS as u32);
  }
}
