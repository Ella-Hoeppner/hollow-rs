use std::sync::Arc;

use bytemuck::NoUninit;
use winit::window::Window;

use super::{
  bind::{BindGroupLayout, BindGroupLayoutDescriptorBuilder},
  buffer::{Buffer, BufferBuilder},
  pipeline::RenderPipelineBuilder,
};

pub struct WGPUController<'window> {
  pub surface: wgpu::Surface<'window>,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
}

impl<'window> WGPUController<'window> {
  pub async fn new(window: Arc<Window>) -> Self {
    let size = window.inner_size();
    let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
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
          required_features: wgpu::Features::empty(),
          required_limits: wgpu::Limits::default(),
          label: None,
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
  pub fn build_buffer<'a, 'w, T: NoUninit>(
    &'w self,
    contents: &'a [T],
  ) -> BufferBuilder<'a, '_, 'w, 'window, T> {
    BufferBuilder::new(self, contents)
  }
  pub fn buffer<T: NoUninit>(&self, contents: T) -> Buffer<T> {
    BufferBuilder::new(self, &[contents]).build()
  }
  pub fn array_buffer<T: NoUninit>(&self, contents: &[T]) -> Buffer<T> {
    BufferBuilder::new(self, contents).build()
  }
  pub fn write_buffer<T: NoUninit>(&self, buffer: &Buffer<T>, data: T) {
    self
      .queue
      .write_buffer(buffer, 0, bytemuck::cast_slice(&[data]))
  }
  pub fn write_array_buffer<T: NoUninit>(
    &self,
    buffer: &Buffer<T>,
    data: &[T],
  ) {
    self
      .queue
      .write_buffer(buffer, 0, bytemuck::cast_slice(data))
  }
  pub fn create_bind_group_layout<'a>(
    &self,
    descriptor_builder: &'a mut BindGroupLayoutDescriptorBuilder<'a>,
  ) -> BindGroupLayout {
    BindGroupLayout::new(
      self
        .device
        .create_bind_group_layout(descriptor_builder.build()),
    )
  }
  pub fn build_render_pipeline(&self) -> RenderPipelineBuilder {
    RenderPipelineBuilder::new(self)
  }
}
