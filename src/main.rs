use std::{sync::Arc, time::Instant};

use wgpu::{
  util::DeviceExt, BindGroup, BindGroupDescriptor, BindGroupEntry,
  BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer, RenderPipeline,
  RenderPipelineDescriptor,
};
use winit::{
  event::{Event, WindowEvent},
  event_loop::EventLoop,
  window::{Window, WindowBuilder},
};

struct WGPUController<'window> {
  surface: wgpu::Surface<'window>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
}

impl WGPUController<'_> {
  async fn new(window: Arc<Window>) -> Self {
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

pub struct Renderer {
  physical_surface_dimensions: (usize, usize),
  primary_bind_group: BindGroup,
  corner_vertex_buffer: Buffer,
  corner_index_buffer: Buffer,
  dimensions_buffer: Buffer,
  background_pipeline: RenderPipeline,
}

impl Renderer {
  pub(crate) fn new(wgpu: &WGPUController) -> Self {
    let shader = wgpu
      .device
      .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let dimensions_buffer =
      wgpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Dimensions Buffer"),
          contents: bytemuck::cast_slice(&[[0f32, 0f32]]),
          usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
    let primary_bind_group_layout =
      wgpu
        .device
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
          entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
              ty: wgpu::BufferBindingType::Uniform,
              has_dynamic_offset: false,
              min_binding_size: None,
            },
            count: None,
          }],
          label: Some("Primary Bind Group Layout"),
        });
    let primary_bind_group =
      wgpu.device.create_bind_group(&BindGroupDescriptor {
        layout: &primary_bind_group_layout,
        entries: &[BindGroupEntry {
          binding: 0,
          resource: dimensions_buffer.as_entire_binding(),
        }],
        label: Some("Primary Bind Group"),
      });
    let corner_vertices: &[[f32; 2]; 4] =
      &[[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]];
    let corner_indeces: &[u16; 6] = &[2, 0, 1, 0, 2, 3];
    let corner_vertex_buffer =
      wgpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Form Vertex Buffer"),
          contents: bytemuck::cast_slice(corner_vertices),
          usage: wgpu::BufferUsages::VERTEX,
        });
    let corner_index_buffer =
      wgpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("Form Index Buffer"),
          contents: bytemuck::cast_slice(corner_indeces),
          usage: wgpu::BufferUsages::INDEX,
        });
    let corner_vertex_buffer_layout = wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &wgpu::vertex_attr_array![0 => Float32x2],
    };
    let background_pipeline_layout =
      wgpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
          label: Some("Background Render Pipeline Layout"),
          bind_group_layouts: &[&primary_bind_group_layout],
          push_constant_ranges: &[],
        });
    let background_pipeline =
      wgpu
        .device
        .create_render_pipeline(&RenderPipelineDescriptor {
          label: Some("Background Render Pipeline"),
          layout: Some(&background_pipeline_layout),
          vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vertex",
            buffers: &[corner_vertex_buffer_layout.clone()],
          },
          fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fragment",
            targets: &[Some(wgpu::ColorTargetState {
              format: wgpu.config.format,
              blend: Some(wgpu::BlendState::REPLACE),
              write_mask: wgpu::ColorWrites::ALL,
            })],
          }),
          primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
          },
          depth_stencil: None,
          multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
          },
          multiview: None,
        });
    Self {
      dimensions_buffer,
      primary_bind_group,
      corner_vertex_buffer,
      corner_index_buffer,
      physical_surface_dimensions: (1, 1),
      background_pipeline,
    }
  }
  pub(crate) fn render(
    &mut self,
    wgpu: &WGPUController,
    t: f32,
  ) -> Result<(), wgpu::SurfaceError> {
    wgpu.queue.write_buffer(
      &self.dimensions_buffer,
      0,
      bytemuck::cast_slice(&[1f32, 1f32]),
    );

    let surface_texture = wgpu.surface.get_current_texture()?;
    let surface_view_descriptor = wgpu::TextureViewDescriptor::default();
    let surface_view = surface_texture
      .texture
      .create_view(&surface_view_descriptor);

    let mut encoder =
      wgpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
          label: Some("Render Encoder"),
        });

    {
      let mut render_pass =
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("Main Render Pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &surface_view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            },
          })],
          depth_stencil_attachment: None,
          occlusion_query_set: None,
          timestamp_writes: None,
        });
      render_pass.set_bind_group(0, &self.primary_bind_group, &[]);
      render_pass.set_index_buffer(
        self.corner_index_buffer.slice(..),
        wgpu::IndexFormat::Uint16,
      );
      render_pass.set_vertex_buffer(0, self.corner_vertex_buffer.slice(..));
      render_pass.set_pipeline(&self.background_pipeline);
      render_pass.draw_indexed(0..6, 0, 0..1);
    }

    wgpu.queue.submit(std::iter::once(encoder.finish()));
    surface_texture.present();
    Ok(())
  }
}

struct App<'w> {
  window: Arc<Window>,
  start_instant: Instant,
  last_frame_timestamp: f32,
  wgpu: WGPUController<'w>,
  renderer: Renderer,
}

impl<'w> App<'w> {
  async fn new(window: Window) -> Self {
    let window_arc = Arc::new(window);
    let wgpu = WGPUController::new(window_arc.clone()).await;
    let renderer = Renderer::new(&wgpu);
    Self {
      window: window_arc,
      start_instant: Instant::now(),
      last_frame_timestamp: 0.,
      wgpu,
      renderer,
    }
  }
  fn time(&self) -> f32 {
    self.start_instant.elapsed().as_secs_f32()
  }
  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    let width = new_size.width;
    let height = new_size.height;
    if width > 0 && height > 0 {
      self.wgpu.config.width = width;
      self.wgpu.config.height = height;
      self
        .wgpu
        .surface
        .configure(&self.wgpu.device, &self.wgpu.config);
    }
  }
  fn handle_window_event(&self, event: &WindowEvent) -> bool {
    false
  }
  fn update(&mut self) {
    let t = self.time();
    self.last_frame_timestamp = t;
  }
}

pub async fn run() {
  let event_loop = EventLoop::new().unwrap();
  let window = WindowBuilder::new()
    .with_title("cast")
    .build(&event_loop)
    .unwrap();
  let mut state = App::new(window).await;
  event_loop
    .run(move |event, event_loop_window_target| match event {
      Event::WindowEvent {
        ref event,
        window_id,
      } if window_id == state.window.id() => {
        if !state.handle_window_event(event) {
          match event {
            WindowEvent::CloseRequested => event_loop_window_target.exit(),
            WindowEvent::Resized(physical_size) => {
              state.resize(*physical_size);
            }
            WindowEvent::RedrawRequested => {
              state.update();
              let t = state.time();
              match state.renderer.render(&state.wgpu, t) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => {
                  state.resize(state.window.inner_size())
                }
                Err(e) => panic!("{:?}", e),
              }
            }
            _ => {}
          }
        }
      }
      Event::AboutToWait => {
        state.window.request_redraw();
      }
      _ => {}
    })
    .unwrap();
}

fn main() {
  pollster::block_on(run());
}
