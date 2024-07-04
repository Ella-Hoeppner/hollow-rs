use std::{marker::PhantomData, ops::Deref, sync::Arc};

use bytemuck::NoUninit;
use wgpu::{util::DeviceExt, Buffer as WGPUBuffer};
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

impl<T: NoUninit> Buffer<T> {
  fn new(buffer: WGPUBuffer) -> Self {
    Self {
      _phantom: PhantomData,
      buffer,
    }
  }
}

impl<T: NoUninit> Deref for Buffer<T> {
  type Target = WGPUBuffer;
  fn deref(&self) -> &Self::Target {
    &self.buffer
  }
}

impl WGPUController<'_> {
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
    use wgpu::BufferUsages;
    Buffer::new(self.device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(contents),
        usage: BufferUsages::COPY_SRC
          | BufferUsages::COPY_DST
          | BufferUsages::INDEX
          | BufferUsages::VERTEX
          | BufferUsages::UNIFORM
          | BufferUsages::STORAGE
          | BufferUsages::INDIRECT
          | BufferUsages::QUERY_RESOLVE,
      },
    ))
  }
}
