use super::{
    data::{self, Vertex},
    texture::Texture,
};
use anyhow::Result;
use wgpu::util::DeviceExt;
use winit::window::Window;

impl super::Renderer {
    pub async fn init(window: &Window, num_vertices: u64) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    limits: wgpu::Limits::default(),
                    features: wgpu::Features::default(),
                },
                None,
            )
            .await?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // texture stuff
        let texture_bytes = include_bytes!("../images/tetris-atlas.png");
        let texture = Texture::from_bytes(&device, &queue, texture_bytes, "Tetris Atlas Texture")?;

        // buffer stuff
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: std::mem::size_of::<Vertex>() as u64 * num_vertices,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Size Buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&[size.width as f32, size.height as f32]),
        });

        //bindings
        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &size_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        let vert_shader =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/basic.vert.wgsl"));
        let frag_shader =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/image.frag.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "vs_main",
                buffers: &[data::Vertex::get_buffer_layout()],
            },

            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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

        Ok(Self {
            adapter,
            surface,
            device,
            queue,
            size,
            pipeline,

            vertex_buffer,

            bind_group,
        })
    }
}
