pub mod data;
mod init;
mod render;
mod texture;
pub struct Renderer {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,

    pub pipeline: wgpu::RenderPipeline,

    pub vertex_buffer: wgpu::Buffer,

    pub bind_group: wgpu::BindGroup,
}
