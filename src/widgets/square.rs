use std::borrow::Cow;

use wgpu::{PipelineLayoutDescriptor, RenderPipeline, RenderPipelineDescriptor};

use crate::{app::AppRender, vertex::Vertex};

use super::{Widget, WidgetRender, WidgetVertex};

/*
 * Vertex style-guide
 * 1: top-left
 * 2: top-right
 * 3: bottom-left
 * 4: bottom-right
 */

/*
 * Formula for Viewport -> NDC
 * screenX = (ndc.x + 1) * viewport.width * 0.5 * viewport.topLeftX
 * screenY = (1 - ndc.y) * viewport.height * 0.5 * viewport.topLeftY
 */

#[derive(Debug, Clone, Copy, Default)]
pub struct SquareWidgetDesc {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SquareWidget<'a> {
    verticies: [Vertex; 4],
    indicies: &'a [u16],
    description: SquareWidgetDesc,
}

impl<'a> SquareWidget<'a> {
    pub fn new(desc: SquareWidgetDesc) -> Self {
        let x = (desc.x / 100.0) * 2.0 - 1.0;
        let y = (desc.y / 100.0) * -2.0 + 1.0;
        let width = desc.width * 2.0 / 100.0;
        let height = desc.height * 2.0 / 100.0;
        Self {
            verticies: [
                Vertex {
                    // Top Left
                    position: [x, y, 0.0],
                    color: desc.color,
                },
                Vertex {
                    // Top Right
                    position: [x + width, y, 0.0],
                    color: desc.color,
                },
                Vertex {
                    // Bottom Left
                    position: [x, y - height, 0.0],
                    color: desc.color,
                },
                Vertex {
                    // Bottom Right
                    position: [x + width, y - height, 0.0],
                    color: desc.color,
                },
            ],
            indicies: &[0, 2, 1, 2, 3, 1],
            description: desc,
        }
    }
}

impl<'a> Widget for SquareWidget<'a> {
    fn create() -> Self {
        Self::default()
    }
    fn set_color(&mut self, color: [f32; 4]) {
        self.description.color = color;
    }
    fn get_color(&mut self) -> [f32; 4] {
        self.description.color
    }
    fn set_x(&mut self, x: f32) {
        self.description.x = x
    }
    fn get_x(&self) -> f32 {
        self.description.x
    }
    fn set_y(&mut self, y: f32) {
        self.description.y = y
    }
    fn get_y(&self) -> f32 {
        self.description.y
    }

    fn set_width(&mut self, width: f32) {
        self.description.width = width
    }

    fn get_width(&mut self) -> f32 {
        self.description.width
    }

    fn set_height(&mut self, height: f32) {
        self.description.height = height
    }

    fn get_height(&mut self) -> f32 {
        self.description.height
    }
    fn get_vertices(&self) -> &[Vertex] {
        self.verticies.as_slice()
    }
    fn get_indices(&self) -> &[u16] {
        self.indicies
    }
    fn build(&self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(*self)
    }
}

impl<'a> WidgetRender for SquareWidget<'a> {
    fn get_pipeline(&self, app: &AppRender) -> RenderPipeline {
        let device = app.get_device();
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let shader = self.get_shader(app);
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[self.get_vertex_desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: Some(wgpu::IndexFormat::Uint16),
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: app.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    }
    fn get_shader(&self, app: &AppRender) -> wgpu::ShaderModule {
        app.get_device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "../shader/main.wgsl"
                ))),
            })
    }
}

impl<'a> WidgetVertex for SquareWidget<'a> {
    fn get_vertex_desc(&self) -> wgpu::VertexBufferLayout<'static> {
        Vertex::desc()
    }
}
