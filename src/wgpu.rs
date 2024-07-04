use std::sync::Arc;

use winit::window::Window;

pub struct WGPUController<'window> {
  pub surface: wgpu::Surface<'window>,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
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
}
