use std::sync::Arc;

use eyre::Result;
use pg_renderer::{
    renderers::{Renderer, TriangleRenderer},
    RenderContext,
};
use tracing::{error, info};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

struct Application {
    event_loop: EventLoop<()>,
    window: Window,

    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,

    render_ctx: RenderContext,

    triangle_renderer: TriangleRenderer,
}

impl Application {
    async fn new(event_loop: EventLoop<()>, window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface =
            unsafe { instance.create_surface(&window) }.expect("Unable to create surface");

        #[cfg(not(target_arch = "wasm32"))]
        {
            info!("Available adapters:");
            for a in instance.enumerate_adapters(wgpu::Backends::all()) {
                info!("\t{:#?}", a.get_info());
            }
        }
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("No suitable GPU adapter found!");

        #[cfg(not(target_arch = "wasm32"))]
        {
            let adapter_info = adapter.get_info();
            info!("Using {} - ({:?})", adapter_info.name, adapter_info.backend);
        }

        let trace_dir = None;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                trace_dir,
            )
            .await
            .expect("Unable to find a suitable GPU adapter!");
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let mut render_ctx = RenderContext::new(&adapter, device, queue);

        let triangle_renderer = TriangleRenderer::new(&mut render_ctx, swapchain_format);

        Self {
            event_loop,
            window,
            surface,
            surface_config,
            render_ctx,

            triangle_renderer,
        }
    }

    fn run(mut self) -> Result<(), impl std::error::Error> {
        self.event_loop.run(move |event, elwt| {
            let event_closure = || -> Result<()> {
                match event {
                    Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
                        match event {
                            WindowEvent::CloseRequested => elwt.exit(),
                            WindowEvent::Resized(new_size) => {
                                self.surface_config.width = new_size.width;
                                self.surface_config.height = new_size.height;
                                self.surface
                                    .configure(&self.render_ctx.device, &self.surface_config);
                                self.window.request_redraw();
                            }
                            WindowEvent::RedrawRequested => {
                                self.window.pre_present_notify();
                                let frame = self
                                    .surface
                                    .get_current_texture()
                                    .expect("Failed to acquire next swap chain texture");
                                let view = frame
                                    .texture
                                    .create_view(&wgpu::TextureViewDescriptor::default());
                                let mut encoder = self.render_ctx.device.create_command_encoder(
                                    &wgpu::CommandEncoderDescriptor { label: None },
                                );

                                {
                                    let mut rpass =
                                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                            label: None,
                                            color_attachments: &[Some(
                                                wgpu::RenderPassColorAttachment {
                                                    view: &view,
                                                    resolve_target: None,
                                                    ops: wgpu::Operations {
                                                        load: wgpu::LoadOp::Clear(
                                                            wgpu::Color::GREEN,
                                                        ),
                                                        store: wgpu::StoreOp::Store,
                                                    },
                                                },
                                            )],
                                            depth_stencil_attachment: None,
                                            timestamp_writes: None,
                                            occlusion_query_set: None,
                                        });
                                    self.triangle_renderer
                                        .draw(&mut rpass, &self.render_ctx.resources)?;
                                }

                                self.render_ctx.queue.submit(Some(encoder.finish()));
                                frame.present();
                            }
                            _ => (),
                        }
                    }
                    _ => {}
                }
                Ok(())
            };
            if let Err(event_closure) = event_closure() {
                error!("Error {event_closure}");
            }
        })
    }
}

fn main() -> Result<()> {
    pg_logger::setup_logger();

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("WOWZAH")
        .with_inner_size(winit::dpi::LogicalSize::new(1920.0, 1080.0))
        .build(&event_loop)?;

    let app = pollster::block_on(Application::new(event_loop, window));
    app.run()?;

    Ok(())
}
