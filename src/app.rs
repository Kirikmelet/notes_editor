use std::future::Future;

use anyhow::{Context, Result};
use wgpu::{util::DeviceExt, *};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::widgets::WidgetRender;

pub struct AppRender {
    _instance: Instance,
    surface: Surface,
    _adapter: Adapter,
    device: Device,
    queue: Queue,
    window: Window,
    pub size: PhysicalSize<u32>,
    pub config: SurfaceConfiguration,
    widgets: Vec<WidgetObject>,
}

impl AppRender {
    pub async fn new(window: Window) -> Result<Self> {
        let size = window.inner_size();
        let instance = Self::init_instance();
        let surface = unsafe { instance.create_surface(&window) }
            .with_context(|| "Failed to create surface")?;
        let adapter = Self::init_adapter(&instance, &surface)
            .await
            .with_context(|| "Failed to request adapter")?;
        let (device, queue) = Self::init_device_and_queue(&adapter)
            .await
            .with_context(|| "Failed to got Device and Queue")?;
        let surface_capabilities = surface.get_capabilities(&adapter);
        let config = Self::init_config(surface_capabilities, size);
        surface.configure(&device, &config);
        Ok(Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            size,
            config,
            window,
            widgets: vec![],
        })
    }
    fn init_device_and_queue(
        adapter: &Adapter,
    ) -> impl Future<Output = Result<(Device, Queue), RequestDeviceError>> + Send {
        adapter.request_device(
            &DeviceDescriptor {
                label: Some("devque"),
                features: Features::empty(),
                limits: Limits::default(),
            },
            None,
        )
    }
    fn init_instance() -> Instance {
        Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Dx12Compiler::default(),
        })
    }
    fn init_adapter(
        instance: &Instance,
        surface: &Surface,
    ) -> impl Future<Output = Option<Adapter>> + Send {
        instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
    }
    fn init_config(
        surface_capabilities: SurfaceCapabilities,
        size: PhysicalSize<u32>,
    ) -> SurfaceConfiguration {
        let format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);
        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        }
    }
    // Getter/Setters
    pub fn get_device(&self) -> &Device {
        &self.device
    }
    pub fn get_window(&self) -> &Window {
        &self.window
    }
    // Methods
    pub fn render(&self) -> Result<(), SurfaceError> {
        let background = self.surface.get_current_texture()?;
        let background_view = background.texture.create_view(&Default::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Encode"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &background_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::GREEN),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            for i in &self.widgets {
                render_pass.set_pipeline(&i.render_pipeline);
                render_pass.set_vertex_buffer(0, i.vertex_buffer.slice(..));
                render_pass.set_index_buffer(i.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..i.index_len, 0, 0..1)
            }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        background.present();
        Ok(())
    }
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.height = new_size.height;
        self.config.width = new_size.width;
        self.surface.configure(&self.device, &self.config);
    }
    pub fn update(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
struct WidgetObject {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub _vertex_len: u32,
    pub index_len: u32,
    pub render_pipeline: RenderPipeline,
}

pub struct App {
    widgets: Vec<Box<dyn WidgetRender>>,
}

impl App {
    pub fn new(widgets: Vec<Box<dyn WidgetRender>>) -> Self {
        Self { widgets }
    }
    pub async fn run(&self) -> Result<()> {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop)?;
        let inner_size = LogicalSize::new(600, 300);
        window.set_min_inner_size(Some(inner_size));
        window.set_inner_size(inner_size);
        window.set_resizable(false);
        let mut renderer = AppRender::new(window).await?;
        for i in &self.widgets {
            App::register_object(&mut renderer, i.as_ref());
        }
        event_loop.run(move |event, _window_target: _, control_flow| match event {
            Event::WindowEvent { window_id, event } if window_id == renderer.get_window().id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => {
                        renderer.resize(new_size);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        scale_factor: _,
                    } => {
                        renderer.resize(*new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == renderer.get_window().id() => {
                renderer.update().expect("Update supposed to succeed!");
                match renderer.render() {
                    Ok(_) => {}
                    Err(SurfaceError::Lost) => renderer.resize(renderer.size),
                    Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::ExitWithCode(-1),
                    Err(x) => eprintln!("{:?}", x),
                }
            }
            Event::MainEventsCleared => {
                renderer.get_window().request_redraw();
            }
            _ => {}
        });
    }
    fn register_object(renderer: &mut AppRender, object: &dyn WidgetRender) {
        let device = &renderer.device;
        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(object.get_vertices()),
            usage: BufferUsages::VERTEX,
        });
        let vertex_len = object.get_vertices().len() as u32;
        let index_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(object.get_indices()),
            usage: BufferUsages::INDEX,
        });
        let index_len = object.get_indices().len() as u32;
        let widget = WidgetObject {
            vertex_buffer,
            index_buffer,
            _vertex_len: vertex_len,
            index_len,
            render_pipeline: object.get_pipeline(renderer),
        };
        renderer.widgets.push(widget);
    }
}
