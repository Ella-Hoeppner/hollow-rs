use std::f32::consts::TAU;

use crate::{
  sketch::{Sketch, SketchData},
  wgpu::{
    bind::BindGroupWithLayout,
    buffer::{ArrayBuffer, Buffer},
    controller::WGPUController,
  },
};
use bytemuck::{NoUninit, Zeroable};
use wgpu::{RenderPipeline, TextureView};

const CIRCLES: usize = 40;

#[repr(C)]
#[derive(Clone, Copy, NoUninit, Default, Zeroable)]
struct Circle {
  x: f32,
  y: f32,
  radius: f32,
}

pub struct VertexSketch {
  primary_bind_group: BindGroupWithLayout,
  corner_vertex_buffer: ArrayBuffer<[f32; 2]>,
  circles: [Circle; CIRCLES],
  circle_instance_buffer: ArrayBuffer<Circle>,
  scale_buffer: Buffer<[f32; 2]>,
  render_pipeline: RenderPipeline,
}

impl Sketch for VertexSketch {
  fn init(wgpu: &WGPUController) -> Self {
    let scale_buffer = wgpu.buffer([0., 0.]);
    let corner_vertex_buffer = wgpu.array_buffer(&[
      [1., 1.],
      [-1., -1.],
      [1., -1.],
      [-1., -1.],
      [1., 1.],
      [-1., 1.],
    ]);
    let circles: [Circle; CIRCLES] = Zeroable::zeroed();
    let circle_instance_buffer = wgpu.array_buffer(&circles);
    let primary_bind_group = wgpu
      .build_bind_group_with_layout()
      .with_uniform_buffer_entry(&scale_buffer)
      .build();
    let render_pipeline = wgpu
      .build_render_pipeline()
      .add_bind_group_layout(&primary_bind_group.layout)
      .add_vertex_buffer_layout(
        corner_vertex_buffer
          .vertex_layout(&wgpu::vertex_attr_array![0 => Float32x2]),
      )
      .add_vertex_buffer_layout(circle_instance_buffer.instance_layout(
        &wgpu::vertex_attr_array![1 => Float32, 2 => Float32, 3 => Float32],
      ))
      .build_with_shader(&wgpu.shader(wgpu::include_wgsl!("vertex.wgsl")));
    Self {
      circles,
      scale_buffer,
      primary_bind_group,
      corner_vertex_buffer,
      circle_instance_buffer,
      render_pipeline,
    }
  }

  fn update(
    &mut self,
    wgpu: &WGPUController,
    surface_view: TextureView,
    data: SketchData,
  ) {
    let dim_min = data.dimensions[0].min(data.dimensions[1]) as f32;
    for i in 0..CIRCLES {
      let angle = TAU * (i as f32) / CIRCLES as f32;
      let position_phase = angle + data.t * 0.9;
      let radius_phase = angle * 3. + data.t * 6.2;
      self.circles[i] = Circle {
        x: position_phase.cos() * 0.75,
        y: position_phase.sin() * 0.75,
        radius: 0.025 + 0.025 * ((radius_phase.cos() + 1.) * 0.5),
      };
    }
    wgpu
      .write_buffer(
        &self.scale_buffer,
        [
          dim_min / data.dimensions[0] as f32,
          dim_min / data.dimensions[1] as f32,
        ],
      )
      .write_array_buffer(&self.circle_instance_buffer, &self.circles);
    wgpu.with_encoder(|encoder| {
      encoder
        .simple_render_pass(&surface_view)
        .with_bind_groups([&self.primary_bind_group])
        .with_vertex_buffer(0, &self.corner_vertex_buffer)
        .with_vertex_buffer(1, &self.circle_instance_buffer)
        .with_pipeline(&self.render_pipeline)
        .draw(0..6, 0..CIRCLES as u32);
    })
  }
}
