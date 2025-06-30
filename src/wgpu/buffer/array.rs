use std::{marker::PhantomData, ops::Deref};

use bytemuck::NoUninit;
use wgpu::{
  util::DeviceExt, BufferSlice, BufferUsages, VertexAttribute,
  VertexBufferLayout,
};

use crate::wgpu::controller::WGPUController;

use super::data::IntoVertexBufferData;

#[derive(Debug, Clone)]
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
  pub fn write(&self, wgpu: &WGPUController, position: usize, data: &[T]) {
    wgpu.queue.write_buffer(
      &self.buffer,
      (position * std::mem::size_of::<T>()) as u64,
      bytemuck::cast_slice(data),
    )
  }
  pub fn copy_from(
    &self,
    wgpu: &WGPUController,
    source: &Self,
    source_offset: usize,
    destination_offset: usize,
    length: usize,
  ) {
    let mut encoder = wgpu
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(
      &source.buffer,
      (source_offset * std::mem::size_of::<T>()) as u64,
      &self.buffer,
      (destination_offset * std::mem::size_of::<T>()) as u64,
      (length * std::mem::size_of::<T>()) as u64,
    );
    wgpu.queue.submit(std::iter::once(encoder.finish()));
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

pub(crate) enum Contents<'c> {
  Owned(Vec<u8>),
  Borrowed(&'c [u8]),
}

pub struct ArrayBufferBuilder<'c, 's, 'w, 'window, T: NoUninit> {
  _phantom: PhantomData<T>,
  initial_contents: Contents<'c>,
  label: Option<&'s str>,
  wgpu: &'w WGPUController<'window>,
  usage: Option<BufferUsages>,
}

impl<'c, 's, 'w, 'window, T: NoUninit>
  ArrayBufferBuilder<'c, 's, 'w, 'window, T>
{
  pub fn from_owned_contents(
    wgpu: &'w WGPUController<'window>,
    initial_contents: Vec<T>,
  ) -> Self {
    Self {
      _phantom: PhantomData,
      initial_contents: Contents::Owned(
        initial_contents
          .into_iter()
          .map(|a| bytemuck::cast(a))
          .collect(),
      ),
      label: None,
      usage: None,
      wgpu: wgpu,
    }
  }
  pub fn from_contents(
    wgpu: &'w WGPUController<'window>,
    initial_contents: &'c [T],
  ) -> Self {
    Self {
      _phantom: PhantomData,
      initial_contents: Contents::Borrowed(bytemuck::cast_slice(
        initial_contents,
      )),
      label: None,
      usage: None,
      wgpu: wgpu,
    }
  }
  pub fn empty(wgpu: &'w WGPUController<'window>, size: usize) -> Self {
    let data = vec![0u8; size * std::mem::size_of::<T>()];
    Self {
      _phantom: PhantomData,
      initial_contents: Contents::Owned(data),
      label: None,
      usage: None,
      wgpu,
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
      len: match &self.initial_contents {
        Contents::Owned(vec) => vec.len(),
        Contents::Borrowed(slice) => slice.len(),
      } / std::mem::size_of::<T>(),
      buffer: self.wgpu.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: self.label,
          contents: match &self.initial_contents {
            Contents::Owned(vec) => &vec,
            Contents::Borrowed(slice) => slice,
          },
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
