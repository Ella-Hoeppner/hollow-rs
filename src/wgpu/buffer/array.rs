use std::{marker::PhantomData, ops::Deref};

use bytemuck::NoUninit;
use wgpu::{
  util::DeviceExt, BufferSlice, BufferUsages, VertexAttribute,
  VertexBufferLayout,
};

use crate::wgpu::controller::WGPUController;

use super::data::IntoVertexBufferData;

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

impl<'s, T: NoUninit> IntoVertexBufferData<'s> for &'s ArrayBuffer<T> {
  fn into_vertex_buffer_data(self) -> BufferSlice<'s> {
    self.slice(..)
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
