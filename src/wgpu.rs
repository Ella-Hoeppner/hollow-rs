use std::{marker::PhantomData, ops::Deref, sync::Arc};

use bytemuck::NoUninit;
use wgpu::{util::DeviceExt, Buffer as WGPUBuffer, BufferUsages};
use winit::window::Window;

pub struct WGPUController<'window> {
  pub surface: wgpu::Surface<'window>,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
}

pub struct Buffer<T: NoUninit> {
  _phantom: PhantomData<T>,
  pub buffer: WGPUBuffer,
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
  fn new(wgpu: &'w WGPUController<'window>, initial_contents: &'c [T]) -> Self {
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
  pub fn buffer<T: NoUninit>(&self, contents: &[T]) -> Buffer<T> {
    BufferBuilder::new(self, contents).build()
  }
  pub fn build_buffer<'a, 'w, T: NoUninit>(
    &'w self,
    contents: &'a [T],
  ) -> BufferBuilder<'a, '_, 'w, 'window, T> {
    BufferBuilder::new(self, contents)
  }
}
