use std::{marker::PhantomData, ops::Deref};

use bytemuck::NoUninit;
use wgpu::{
  util::DeviceExt, BufferUsages, VertexAttribute, VertexBufferLayout,
};

use super::controller::WGPUController;

pub struct Buffer<T: NoUninit> {
  _phantom: PhantomData<T>,
  pub buffer: wgpu::Buffer,
}

impl<T: NoUninit> Buffer<T> {
  pub fn vertex_layout<'a>(
    &self,
    attributes: &'a [VertexAttribute],
  ) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
      array_stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes,
    }
  }
  pub fn instance_stepped_vertex_layout<'a>(
    &self,
    attributes: &'a [VertexAttribute],
  ) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
      array_stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes,
    }
  }
}

impl<'a> Into<VertexBufferLayout<'a>> for &'a Buffer<f32> {
  fn into(self) -> VertexBufferLayout<'a> {
    self.vertex_layout(&wgpu::vertex_attr_array![0 => Float32])
  }
}
impl<'a> Into<VertexBufferLayout<'a>> for &'a Buffer<[f32; 2]> {
  fn into(self) -> VertexBufferLayout<'a> {
    self.vertex_layout(&wgpu::vertex_attr_array![0 => Float32x2])
  }
}
impl<'a> Into<VertexBufferLayout<'a>> for &'a Buffer<[f32; 3]> {
  fn into(self) -> VertexBufferLayout<'a> {
    self.vertex_layout(&wgpu::vertex_attr_array![0 => Float32x3])
  }
}
impl<'a> Into<VertexBufferLayout<'a>> for &'a Buffer<[f32; 4]> {
  fn into(self) -> VertexBufferLayout<'a> {
    self.vertex_layout(&wgpu::vertex_attr_array![0 => Float32x4])
  }
}

impl<T: NoUninit> Deref for Buffer<T> {
  type Target = wgpu::Buffer;
  fn deref(&self) -> &Self::Target {
    &self.buffer
  }
}

pub trait IntoBufferData<T: NoUninit> {
  fn into_buffer_data(self) -> T;
}
impl<T: NoUninit> IntoBufferData<T> for T {
  fn into_buffer_data(self) -> T {
    self
  }
}
impl IntoBufferData<f32> for usize {
  fn into_buffer_data(self) -> f32 {
    self as f32
  }
}
impl IntoBufferData<[f32; 2]> for [usize; 2] {
  fn into_buffer_data(self) -> [f32; 2] {
    [self[0] as f32, self[1] as f32]
  }
}
impl IntoBufferData<[f32; 3]> for [usize; 3] {
  fn into_buffer_data(self) -> [f32; 3] {
    [self[0] as f32, self[1] as f32, self[2] as f32]
  }
}
impl IntoBufferData<[f32; 4]> for [usize; 4] {
  fn into_buffer_data(self) -> [f32; 4] {
    [
      self[0] as f32,
      self[1] as f32,
      self[2] as f32,
      self[3] as f32,
    ]
  }
}

pub struct BufferBuilder<'c, 's, 'w, 'window, T: NoUninit> {
  initial_contents: &'c [T],
  label: Option<&'s str>,
  wgpu: &'w WGPUController<'window>,
  usage: Option<BufferUsages>,
}

impl<'c, 's, 'w, 'window, T: NoUninit> BufferBuilder<'c, 's, 'w, 'window, T> {
  pub fn new(
    wgpu: &'w WGPUController<'window>,
    initial_contents: &'c [T],
  ) -> Self {
    Self {
      initial_contents,
      label: None,
      usage: None,
      wgpu: wgpu,
    }
  }
  pub fn with_label(mut self, label: &'s str) -> Self {
    self.label = Some(label);
    self
  }
  pub fn with_usage(mut self, usage: BufferUsages) -> Self {
    self.usage = Some(usage);
    self
  }
  pub fn build(self) -> Buffer<T> {
    Buffer {
      _phantom: PhantomData,
      buffer: self.wgpu.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: self.label,
          contents: bytemuck::cast_slice(self.initial_contents),
          usage: self.usage.unwrap_or(
            BufferUsages::COPY_SRC
              | BufferUsages::COPY_DST
              | BufferUsages::INDEX
              | BufferUsages::VERTEX
              | BufferUsages::UNIFORM
              | BufferUsages::STORAGE
              | BufferUsages::INDIRECT
              | BufferUsages::QUERY_RESOLVE,
          ),
        },
      ),
    }
  }
}
