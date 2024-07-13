use std::{marker::PhantomData, ops::Deref};

use bytemuck::NoUninit;
use wgpu::{
  util::DeviceExt, BufferUsages, VertexAttribute, VertexBufferLayout,
};

use super::controller::WGPUController;

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

pub struct Buffer<T: NoUninit> {
  _phantom: PhantomData<T>,
  buffer: wgpu::Buffer,
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
  pub fn instance_layout<'a>(
    &self,
    attributes: &'a [VertexAttribute],
  ) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
      array_stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
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

impl<T: NoUninit> Deref for Buffer<T> {
  type Target = wgpu::Buffer;
  fn deref(&self) -> &Self::Target {
    &self.buffer
  }
}

impl<'b, T: NoUninit> Into<&'b wgpu::Buffer> for &'b Buffer<T> {
  fn into(self) -> &'b wgpu::Buffer {
    &*self
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

pub struct ArrayBuffer<T: NoUninit> {
  _phantom: PhantomData<T>,
  len: usize,
  buffer: wgpu::Buffer,
}

impl<T: NoUninit> ArrayBuffer<T> {
  pub fn len(&self) -> usize {
    self.len
  }
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
  pub fn instance_layout<'a>(
    &self,
    attributes: &'a [VertexAttribute],
  ) -> VertexBufferLayout<'a> {
    VertexBufferLayout {
      array_stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
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

impl<T: NoUninit> Deref for ArrayBuffer<T> {
  type Target = wgpu::Buffer;
  fn deref(&self) -> &Self::Target {
    &self.buffer
  }
}

impl<'b, T: NoUninit> Into<&'b wgpu::Buffer> for &'b ArrayBuffer<T> {
  fn into(self) -> &'b wgpu::Buffer {
    &*self
  }
}

pub struct ArrayBufferBuilder<'c, 's, 'w, 'window, T: NoUninit> {
  initial_contents: &'c [T],
  label: Option<&'s str>,
  wgpu: &'w WGPUController<'window>,
  usage: Option<BufferUsages>,
}

impl<'c, 's, 'w, 'window, T: NoUninit>
  ArrayBufferBuilder<'c, 's, 'w, 'window, T>
{
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
  pub fn build(self) -> ArrayBuffer<T> {
    ArrayBuffer {
      _phantom: PhantomData,
      len: self.initial_contents.len(),
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
