use std::sync::Arc;

use bytemuck::{NoUninit, Zeroable};
use wgpu::{Features, ShaderModule, ShaderModuleDescriptor};
use winit::window::Window;

use super::{
  bind::{BindGroupLayoutBuilder, BindGroupWithLayoutBuilder},
  buffer::{
    ArrayBuffer, ArrayBufferBuilder, Buffer, BufferBuilder, IntoBufferData,
    VectorBuffer, VectorBufferBuilder,
  },
  encoder::CommandEncoder,
  pipeline::{ComputePipelineBuilder, RenderPipelineBuilder},
  texture::TextureBuilder,
};

pub struct WGPUController<'window> {
  pub surface: wgpu::Surface<'window>,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
}

impl<'window> WGPUController<'window> {
  pub async fn new_with_features(
    window: Arc<Window>,
    features: Features,
  ) -> Self {
    let size = window.inner_size();
    let wgpu_instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });
    let surface = wgpu_instance.create_surface(window.clone()).unwrap();
    let adapter = wgpu_instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();
    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          required_features: features,
          required_limits: wgpu::Limits::default(),
          label: None,
          memory_hints: Default::default(),
        },
        None,
      )
      .await
      .unwrap();
    let surface_capabilities = surface.get_capabilities(&adapter);
    let surface_format = surface_capabilities
      .formats
      .iter()
      .copied()
      .filter(|f| f.is_srgb())
      .next()
      .unwrap_or(surface_capabilities.formats[0]);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_capabilities.present_modes[0],
      alpha_mode: surface_capabilities.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);
    Self {
      surface,
      device,
      queue,
      config,
    }
  }
  pub async fn new(window: Arc<Window>) -> Self {
    Self::new_with_features(window, Features::empty()).await
  }
  pub fn create_encoder(&self) -> CommandEncoder {
    CommandEncoder::new(
      self
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
          label: None,
        }),
    )
  }
  pub fn finish_encoder(&self, encoder: CommandEncoder) {
    self.queue.submit(std::iter::once(encoder.encoder.finish()));
  }
  pub fn shader(&self, source: ShaderModuleDescriptor<'_>) -> ShaderModule {
    self.device.create_shader_module(source)
  }
  pub fn build_texture_2d<'w>(
    &'w self,
    width: u32,
    height: u32,
  ) -> TextureBuilder<'w, 'w, 'w> {
    TextureBuilder::new_2d(self, width, height)
  }
  pub fn build_buffer<'a, 'w, T: NoUninit>(
    &'w self,
    contents: &'a [T],
  ) -> BufferBuilder<'a, 'w, 'w, 'window, T> {
    BufferBuilder::new(self, contents)
  }
  pub fn buffer<T: NoUninit>(&self, contents: T) -> Buffer<T> {
    BufferBuilder::new(self, &[contents]).build()
  }
  pub fn build_array_buffer_owned<T: NoUninit>(
    &self,
    contents: Vec<T>,
  ) -> ArrayBufferBuilder<T> {
    ArrayBufferBuilder::from_owned_contents(self, contents)
  }
  pub fn build_array_buffer<'a, T: NoUninit>(
    &'a self,
    contents: &'a [T],
  ) -> ArrayBufferBuilder<'a, 'a, 'a, 'a, T> {
    ArrayBufferBuilder::from_contents(self, contents)
  }
  pub fn array_buffer<T: NoUninit>(&self, contents: &[T]) -> ArrayBuffer<T> {
    ArrayBufferBuilder::from_contents(self, contents).build()
  }
  pub fn zeroed_array_buffer<T: NoUninit + Zeroable>(
    &self,
    length: usize,
  ) -> ArrayBuffer<T> {
    self.array_buffer(
      &std::iter::repeat(T::zeroed())
        .take(length)
        .collect::<Vec<T>>(),
    )
  }
  pub fn build_empty_array_buffer<T: NoUninit + Zeroable>(
    &self,
    size: usize,
  ) -> ArrayBufferBuilder<T> {
    ArrayBufferBuilder::empty(self, size)
  }
  pub fn empty_array_buffer<T: NoUninit + Zeroable>(
    &self,
    size: usize,
  ) -> ArrayBuffer<T> {
    self.build_empty_array_buffer(size).build()
  }
  pub fn vector_buffer<T: NoUninit>(&self) -> VectorBuffer<T> {
    VectorBufferBuilder::new(self).build()
  }
  pub fn write_buffer<T: NoUninit>(
    &self,
    buffer: &Buffer<T>,
    data: impl IntoBufferData<T>,
  ) -> &Self {
    self.queue.write_buffer(
      buffer,
      0,
      bytemuck::cast_slice(&[data.into_buffer_data()]),
    );
    self
  }
  pub fn write_array_buffer<T: NoUninit>(
    &self,
    buffer: &ArrayBuffer<T>,
    data: &[T],
  ) -> &Self {
    self
      .queue
      .write_buffer(buffer, 0, bytemuck::cast_slice(data));
    self
  }
  pub fn build_render_pipeline(&self) -> RenderPipelineBuilder {
    RenderPipelineBuilder::new(self)
  }
  pub fn build_compute_pipeline(&self) -> ComputePipelineBuilder {
    ComputePipelineBuilder::new(self)
  }
  pub fn build_bind_group_layout(&self) -> BindGroupLayoutBuilder {
    BindGroupLayoutBuilder::new(self)
  }
  pub fn build_bind_group_with_layout(&self) -> BindGroupWithLayoutBuilder {
    BindGroupWithLayoutBuilder::new(self)
  }
  pub fn with_encoder(&self, mut f: impl FnMut(&mut CommandEncoder)) {
    let mut encoder = self.create_encoder();
    f(&mut encoder);
    self.finish_encoder(encoder);
  }
}
