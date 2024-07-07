use crate::{
  sketch::Sketch,
  wgpu::{
    bind::BindGroupWithLayout, buffer::Buffer, controller::WGPUController,
    encoder::CommandEncoder,
  },
};
use rand::Rng;
use wgpu::{ComputePipeline, RenderPipeline, TextureView};

const POINT_GROUP_MULTIPLE: usize = 1000;
const POINTS: usize = 256 * POINT_GROUP_MULTIPLE;

pub struct CliffordSketch {
  uniform_bind_group: BindGroupWithLayout,
  render_points_bind_group: BindGroupWithLayout,
  compute_points_bind_group: BindGroupWithLayout,
  corner_vertex_buffer: Buffer<[f32; 2]>,
  scale_buffer: Buffer<[f32; 2]>,
  render_pipeline: RenderPipeline,
  compute_pipeline: ComputePipeline,
}

impl Sketch for CliffordSketch {
  fn init(wgpu: &WGPUController) -> Self {
    let scale_buffer = wgpu.buffer([0., 0.]);
    let mut rng = rand::thread_rng();
    let point_buffer = wgpu.array_buffer(
      &std::iter::repeat_with(|| {
        [rng.gen::<f32>() * 2. - 1., rng.gen::<f32>() * 2. - 1.]
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
    let compute_points_bind_group = wgpu
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
    let compute_pipeline =
      wgpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
          label: None,
          layout: Some(&wgpu.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
              label: None,
              bind_group_layouts: &[&compute_points_bind_group.layout],
              push_constant_ranges: &[],
            },
          )),
          module: &wgpu.shader(wgpu::include_wgsl!("clifford_compute.wgsl")),
          entry_point: "compute",
        });
    Self {
      scale_buffer,
      uniform_bind_group,
      render_points_bind_group,
      compute_points_bind_group,
      corner_vertex_buffer,
      render_pipeline,
      compute_pipeline,
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
    {
      let mut compute_pass =
        encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
          label: None,
          timestamp_writes: None,
        });
      compute_pass.set_pipeline(&self.compute_pipeline);
      compute_pass.set_bind_group(0, &self.compute_points_bind_group, &[]);
      compute_pass.dispatch_workgroups(POINT_GROUP_MULTIPLE as u32, 1, 1);
    }
    encoder
      .simple_render_pass(&surface_view)
      .with_bind_groups([
        &self.uniform_bind_group,
        &self.render_points_bind_group,
      ])
      .with_vertex_buffer(0, &self.corner_vertex_buffer)
      .with_pipeline(&self.render_pipeline)
      .draw(0..6, 0..POINTS as u32);
  }
}