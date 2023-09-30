use wgpu::{RenderPipeline, ShaderModule, VertexBufferLayout};

use crate::{vertex::Vertex, app::AppRender};

pub mod square;

pub trait Widget {
    fn create() -> Self
    where
        Self: Sized;
    fn set_color(&mut self, color: [f32; 4]);
    fn get_color(&mut self) -> [f32; 4];
    fn set_x(&mut self, x: f32);
    fn get_x(&self) -> f32;
    fn set_y(&mut self, y: f32);
    fn get_y(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_width(&mut self) -> f32;
    fn set_height(&mut self, height: f32);
    fn get_height(&mut self) -> f32;
    fn get_vertices(&self) -> &[Vertex];
    fn get_indices(&self) -> &[u16];
    fn build(&self) -> Box<Self>
    where
        Self: Sized;
}

pub trait WidgetVertex {
    fn get_vertex_desc(&self) -> VertexBufferLayout<'static>;
}

pub trait WidgetRender: Widget + WidgetVertex {
    fn get_pipeline(&self, renderer: &AppRender) -> RenderPipeline;
    fn get_shader(&self, renderer: &AppRender) -> ShaderModule;
}
