//! Renderer - WGPU-based 3D renderer

use winit::window::Window;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureFormat, Color};
use crate::app::AppState;
use std::error::Error;

/// WGPU renderer
pub struct Renderer<'a> {
    device: Device,
    queue: Queue,
    surface: Surface<'a>,
    config: SurfaceConfiguration,
}

impl<'_> Renderer<'_> {
    /// Create a new renderer
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });
        
        let surface = unsafe { instance.create_surface(window) }?;
        
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        })).ok_or("Failed to get adapter")?;
        
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None
        ))?;
        
        let size = window.inner_size();
        let config = SurfaceConfiguration {
            desired_maximum_frame_latency: 1,
            view_formats: vec![TextureFormat::Bgra8Unorm],
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        
        surface.configure(&device, &config);
        
        Ok(Self {
            device,
            queue,
            surface,
            config,
        })
    }

    /// Resize the renderer
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Render the scene
    pub fn render(&mut self, _state: &AppState) -> Option<wgpu::SurfaceTexture> {
        let output = self.surface.get_current_texture().ok()?;
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render encoder"),
        });
        
        // Clear with background color
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            occlusion_query_set: None,
            timestamp_writes: None,
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.15,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
        });
        
        // TODO: Render point cloud and geometries
        // This is where we would render the point cloud using instanced rendering
        // and the geometries using standard mesh rendering
        
        drop(render_pass);
        
        self.queue.submit(std::iter::once(encoder.finish()));
        
        Some(output)
    }

    /// Present the rendered frame
    pub fn present(&self, output: wgpu::SurfaceTexture) {
        output.present();
    }

    /// Get the device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get the queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}