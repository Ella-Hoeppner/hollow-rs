use std::ops::{Deref, DerefMut};

pub struct ComputePass<'p> {
  pass: wgpu::ComputePass<'p>,
}
impl<'s> Deref for ComputePass<'s> {
  type Target = wgpu::ComputePass<'s>;

  fn deref(&self) -> &Self::Target {
    &self.pass
  }
}
impl<'s> DerefMut for ComputePass<'s> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.pass
  }
}
impl<'p> ComputePass<'p> {
  pub fn new(pass: wgpu::ComputePass<'p>) -> Self {
    Self { pass }
  }
  pub fn with_pipeline<'a: 'p>(
    mut self,
    pipeline: &'a wgpu::ComputePipeline,
  ) -> Self {
    self.set_pipeline(pipeline);
    self
  }
  pub fn with_offset_bind_group(
    mut self,
    index: u32,
    bind_group: &'p wgpu::BindGroup,
    offsets: &[wgpu::DynamicOffset],
  ) -> Self {
    self.set_bind_group(index, bind_group, offsets);
    self
  }
  pub fn with_bind_group(
    self,
    index: u32,
    bind_group: &'p wgpu::BindGroup,
  ) -> Self {
    self.with_offset_bind_group(index, bind_group, &[])
  }
  pub fn with_bind_groups<const N: usize>(
    self,
    bind_groups: [&'p wgpu::BindGroup; N],
  ) -> Self {
    bind_groups
      .iter()
      .enumerate()
      .fold(self, |pass, (i, group)| {
        pass.with_bind_group(i as u32, group)
      })
  }
  pub fn dispatch(mut self, x: u32, y: u32, z: u32) -> Self {
    self.dispatch_workgroups(x, y, z);
    self
  }
}
