use std::{num::NonZero, ops::Deref};

use bytemuck::NoUninit;
use wgpu::{
  BindGroup, BindGroupDescriptor, BindGroupEntry,
  BindGroupLayout as WGPUBindGroupLayout, BindGroupLayoutDescriptor,
  BindGroupLayoutEntry, BindingType, ShaderStages,
};

use super::{buffer::Buffer, controller::WGPUController};

#[derive(Default)]
pub struct BindGroupLayoutEntryBuilder {
  binding: Option<u32>,
  count: Option<NonZero<u32>>,
  ty: Option<BindingType>,
  visibility: Option<ShaderStages>,
}

impl BindGroupLayoutEntryBuilder {
  pub fn new() -> Self {
    BindGroupLayoutEntryBuilder::default()
  }
  pub fn with_binding(mut self, binding: u32) -> Self {
    self.binding = Some(binding);
    self
  }
  fn with_binding_if_none(mut self, binding: u32) -> Self {
    if self.binding.is_none() {
      self.binding = Some(binding);
    }
    self
  }
  pub fn with_count(mut self, count: NonZero<u32>) -> Self {
    self.count = Some(count);
    self
  }
  pub fn with_ty(mut self, ty: BindingType) -> Self {
    self.ty = Some(ty);
    self
  }
  pub fn with_visibility(mut self, visibility: ShaderStages) -> Self {
    self.visibility = Some(visibility);
    self
  }
  pub fn build(self) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
      binding: self.binding.unwrap_or(0),
      count: self.count,
      ty: self.ty.unwrap_or(wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
      }),
      visibility: self.visibility.unwrap_or(
        wgpu::ShaderStages::VERTEX
          | wgpu::ShaderStages::FRAGMENT
          | wgpu::ShaderStages::COMPUTE,
      ),
    }
  }
}

pub struct BindGroupLayoutDescriptorBuilder<'s> {
  entries: Vec<BindGroupLayoutEntry>,
  label: Option<&'s str>,
  descriptor: Option<BindGroupLayoutDescriptor<'s>>,
}

impl<'s> BindGroupLayoutDescriptorBuilder<'s> {
  pub fn new() -> Self {
    Self {
      entries: vec![],
      label: None,
      descriptor: None,
    }
  }
  pub fn with_entry(mut self, entry: BindGroupLayoutEntryBuilder) -> Self {
    self.entries.push(
      entry
        .with_binding_if_none(self.entries.len() as u32)
        .build(),
    );
    self
  }
  pub fn with_default_entry(self) -> Self {
    self.with_entry(BindGroupLayoutEntryBuilder::new())
  }
  pub fn build<'a: 'b + 's, 'b>(
    &'a mut self,
  ) -> &'b BindGroupLayoutDescriptor<'s> {
    self.descriptor = Some(BindGroupLayoutDescriptor {
      entries: &self.entries,
      label: self.label,
    });
    self.descriptor.as_ref().unwrap()
  }
}

pub struct BindGroupLayout {
  pub layout: WGPUBindGroupLayout,
}
impl BindGroupLayout {
  pub fn new(layout: WGPUBindGroupLayout) -> Self {
    Self { layout }
  }
  pub fn create_group(
    &self,
    wgpu: &WGPUController,
    buffers: &[&Buffer<impl NoUninit>],
  ) -> BindGroup {
    let entries: Vec<_> = buffers
      .iter()
      .enumerate()
      .map(|(i, buffer)| BindGroupEntry {
        binding: i as u32,
        resource: buffer.as_entire_binding(),
      })
      .collect();
    wgpu.device.create_bind_group(&BindGroupDescriptor {
      layout: &self,
      entries: &entries,
      label: None,
    })
  }
}
impl Deref for BindGroupLayout {
  type Target = WGPUBindGroupLayout;

  fn deref(&self) -> &Self::Target {
    &self.layout
  }
}
