use std::{marker::PhantomData, ops::Deref};

use bytemuck::NoUninit;
use wgpu::{
  util::DeviceExt, Buffer as WGPUBuffer, BufferUsages, VertexAttribute,
  VertexBufferLayout,
};

use super::controller::WGPUController;

pub struct Buffer<T: NoUninit> {
  _phantom: PhantomData<T>,
  pub buffer: WGPUBuffer,
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

impl<T: NoUninit> Deref for Buffer<T> {
  type Target = WGPUBuffer;
  fn deref(&self) -> &Self::Target {
    &self.buffer
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
