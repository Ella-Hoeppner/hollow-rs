use bytemuck::NoUninit;
use wgpu::BufferSlice;

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
impl IntoBufferData<f32> for u32 {
  fn into_buffer_data(self) -> f32 {
    self as f32
  }
}
impl IntoBufferData<[f32; 2]> for [u32; 2] {
  fn into_buffer_data(self) -> [f32; 2] {
    [self[0] as f32, self[1] as f32]
  }
}
impl IntoBufferData<[f32; 3]> for [u32; 3] {
  fn into_buffer_data(self) -> [f32; 3] {
    [self[0] as f32, self[1] as f32, self[2] as f32]
  }
}
impl IntoBufferData<[f32; 4]> for [u32; 4] {
  fn into_buffer_data(self) -> [f32; 4] {
    [
      self[0] as f32,
      self[1] as f32,
      self[2] as f32,
      self[3] as f32,
    ]
  }
}

pub trait IntoVertexBufferData<'s> {
  fn into_vertex_buffer_data(self) -> BufferSlice<'s>;
}
impl<'s> IntoVertexBufferData<'s> for BufferSlice<'s> {
  fn into_vertex_buffer_data(self) -> BufferSlice<'s> {
    self
  }
}
