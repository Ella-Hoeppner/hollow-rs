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

pub struct BindGroupLayoutBuilder<'s, 'w, 'window> {
  wgpu: &'w WGPUController<'window>,
  entries: Vec<BindGroupLayoutEntry>,
  label: Option<&'s str>,
}

impl<'s, 'w, 'window> BindGroupLayoutBuilder<'s, 'w, 'window> {
  pub fn new(wgpu: &'w WGPUController<'window>) -> Self {
    Self {
      wgpu,
      entries: vec![],
      label: None,
    }
  }
  pub fn with_label(mut self, label: &'s str) -> Self {
    self.label = Some(label);
    self
  }
  pub fn with_entry(mut self, entry: BindGroupLayoutEntryBuilder) -> Self {
    self.entries.push(
      entry
        .with_binding_if_none(self.entries.len() as u32)
        .build(),
    );
    self
  }
  pub fn with_uniform_entry(self) -> Self {
    self.with_entry(BindGroupLayoutEntryBuilder::new())
  }
  pub fn build(self) -> BindGroupLayout {
    BindGroupLayout::new(self.wgpu.device.create_bind_group_layout(
      &BindGroupLayoutDescriptor {
        entries: &self.entries,
        label: self.label,
      },
    ))
  }
}

pub struct BindGroupBuilder<'l, 's, 'a, 'w, 'window> {
  wgpu: &'w WGPUController<'window>,
  layout: Option<&'l BindGroupLayout>,
  label: Option<&'s str>,
  entries: Vec<BindGroupEntry<'a>>,
}

impl<'l, 's, 'a, 'w, 'window> BindGroupBuilder<'l, 's, 'a, 'w, 'window> {
  pub fn new(wgpu: &'w WGPUController<'window>) -> Self {
    Self {
      wgpu,
      layout: None,
      label: None,
      entries: vec![],
    }
  }
  pub fn with_layout(mut self, layout: &'l BindGroupLayout) -> Self {
    self.layout = Some(layout);
    self
  }
  pub fn with_label(mut self, label: &'s str) -> Self {
    self.label = Some(label);
    self
  }
  pub fn with_buffer_entry<'b: 'a, T: NoUninit>(
    mut self,
    buffer: &'b Buffer<T>,
  ) -> Self {
    self.entries.push(BindGroupEntry {
      binding: self.entries.len() as u32,
      resource: buffer.as_entire_binding(),
    });
    self
  }
  pub fn build(self) -> BindGroup {
    self.wgpu.device.create_bind_group(&BindGroupDescriptor {
      layout: self.layout.expect(
        "Attempted to build a BindGroup with no layout (did you forget to \
        call with_layout on your BindGroupBuilder?)",
      ),
      entries: &self.entries,
      label: None,
    })
  }
}

pub struct BindGroupLayout {
  pub layout: WGPUBindGroupLayout,
}
impl BindGroupLayout {
  pub fn new(layout: WGPUBindGroupLayout) -> Self {
    Self { layout }
  }
  pub fn build_group<'l, 'w, 'window>(
    &'l self,
    wgpu: &'w WGPUController<'window>,
  ) -> BindGroupBuilder<'l, '_, '_, 'w, 'window> {
    BindGroupBuilder::new(wgpu).with_layout(self)
  }
}
impl Deref for BindGroupLayout {
  type Target = WGPUBindGroupLayout;

  fn deref(&self) -> &Self::Target {
    &self.layout
  }
}

pub struct BindGroupWithLayout {
  pub layout: BindGroupLayout,
  pub group: BindGroup,
}

impl BindGroupWithLayout {
  pub fn new(layout: BindGroupLayout, group: BindGroup) -> Self {
    Self { layout, group }
  }
}
impl Deref for BindGroupWithLayout {
  type Target = BindGroup;

  fn deref(&self) -> &Self::Target {
    &self.group
  }
}

pub struct BindGroupWithLayoutBuilder<'s, 'l, 'a, 'w, 'window> {
  layout_builder: BindGroupLayoutBuilder<'s, 'w, 'window>,
  group_builder: BindGroupBuilder<'l, 's, 'a, 'w, 'window>,
}
impl<'s, 'l, 'a, 'w, 'window>
  BindGroupWithLayoutBuilder<'s, 'l, 'a, 'w, 'window>
{
  pub fn new(wgpu: &'w WGPUController<'window>) -> Self {
    Self {
      layout_builder: BindGroupLayoutBuilder::new(wgpu),
      group_builder: BindGroupBuilder::new(wgpu),
    }
  }
  pub fn with_label(mut self, label: &'s str) -> Self {
    self.layout_builder = self.layout_builder.with_label(label);
    self.group_builder = self.group_builder.with_label(label);
    self
  }
  pub fn with_buffer_entry<'b: 'a, T: NoUninit>(
    mut self,
    entry: BindGroupLayoutEntryBuilder,
    buffer: &'b Buffer<T>,
  ) -> Self {
    self.layout_builder = self.layout_builder.with_entry(entry);
    self.group_builder = self.group_builder.with_buffer_entry(buffer);
    self
  }
  pub fn with_uniform_buffer_entry<'b: 'a, T: NoUninit>(
    mut self,
    buffer: &'b Buffer<T>,
  ) -> Self {
    self.layout_builder = self.layout_builder.with_uniform_entry();
    self.group_builder = self.group_builder.with_buffer_entry(buffer);
    self
  }
  pub fn build(self) -> BindGroupWithLayout {
    let layout = self.layout_builder.build();
    let group = self.group_builder.with_layout(&layout).build();
    BindGroupWithLayout::new(layout, group)
  }
}
