use wgpu::util::DeviceExt;
use wgpu::VertexState;

use crate::frame_rate::FrameRate;
use crate::pipeline::{PipelineError, PipelineFuncs};
use crate::ShaderBuilderForLibrary;


/// A simple struct to store a wgpu pass with a uniform buffer.
#[derive(Debug)]
pub struct Pass {
    /// Pipeline that will be called to render the pass
    pub pipeline: wgpu::RenderPipeline,
    /// Buffer bind group for this pass.
    pub bind_group: wgpu::BindGroup,
    /// Single uniform buffer for this pass.
    pub uniform_buf: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ScreenResolution([f32; 2]);

const TEST_RESOLUTION: &[ScreenResolution] = &[ScreenResolution([1000.0, 1000.0])];

const VERTICES: &[Vertex] = &[



    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];


impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

const INDICES: &[u16] = &[0, 1, 2, 3, 4, 5,/* padding */ 0];


/// Settings for the `PipelineFuncs`
/// `polygon_edge_count` is not exposed in ui on purpose for  purposes
/// change it in the code with hot-reload enable to see it working.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlowSettings {
    res: [f32; 2],
}

impl GlowSettings {
    // pub fn new() -> Self {
    //     Self {
    //         res_x: 1000.0,
    //         res_y: 1000.0,
    //     }
    // }

    pub fn get_size() -> u64 {
        std::mem::size_of::<Self>() as _
    }
}
///  Pipeline showcasing the three type of live updates via the rotation of a regular polygon
///
///     shader: `draw.wgsl`
///     rust: `polygon_edge_count` in [`PipelineFuncs::update`]
///     ui: `size` and `speed`
#[derive(Debug)]
pub struct Pipeline {
    render_pass: Pass,
    _start_time: web_time::Instant, // std::time::Instant is not compatible with wasm
    last_update: web_time::Instant,
    settings: GlowSettings,
    frame_rate: FrameRate,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32
}

impl PipelineFuncs for Pipeline {
    /// Create pipeline.
    /// Assume the `render_pipeline` will be properly initialized.
    fn init(
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
        _surface_configuration: &wgpu::SurfaceConfiguration,
    ) -> Result<Self, PipelineError> {
        let render_pass = Self::create_render_pass(surface, device, adapter)?;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // let screen_resolution = device.create_bind_group(&wgpu::util::b)



        let num_indices = INDICES.len() as u32;

        Ok(Self {
            //render_pass,
            _start_time: web_time::Instant::now(),
            last_update: web_time::Instant::now(),
            settings: GlowSettings {
                res: [_surface_configuration.width as f32, _surface_configuration.height as f32],
            },
            frame_rate: FrameRate::default(),
            vertex_buffer,
            index_buffer,
            num_indices,
            render_pass,
            //screen_resolution: screen_resolution_bindgroup,
        })
    }

    /// Get pipeline name.
    fn get_name() -> &'static str {
        "glow_shader"
    }

    /// Recreate render pass.
    fn update_passes(
        &mut self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    ) -> Result<(), PipelineError> {
        self.render_pass = Self::create_render_pass(surface, device, adapter)?;
        Ok(())
    }

    // Resize owned textures if needed, nothing for the demo here.
    fn resize(
        &mut self,
        _surface_configuration: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    /// Update pipeline before rendering.
    fn update(&mut self, queue: &wgpu::Queue) {
        // Set the edge count of the regular polygon.
        // This is not exposed in the ui on purpose to demonstrate the rust hot reload.
        //self.settings.polygon_edge_count = 7;

        // update elapsed time, taking speed into consideration.
        let last_frame_duration = self.last_update.elapsed().as_secs_f32();
        //self.settings.elapsed += last_frame_duration * self.settings.speed;
        self.frame_rate.update(last_frame_duration);
        self.last_update = web_time::Instant::now();
        
        queue.write_buffer(
            &self.render_pass.uniform_buf,
            0,
            bytemuck::cast_slice(&[self.settings]),
        );
    }

    /// Render pipeline.
    fn render(&self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        // We draw a regular polygon with n edges
        // by drawing the n triangles starting from the center and with two adjacent vertices
        // hence the * 3 vertex count, a square results in 4 triangles so 12 vertices to draw.
        //let vertex_count = self.settings.polygon_edge_count * 3;


        
        // Create a command encoder.
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            // render pass.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pass.pipeline);
            // render_pass.set_bind_group(0, &self.screen_resolution, &[]);

            render_pass.set_bind_group(0, &self.render_pass.bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));
    }

    /// Draw ui with egui.
    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();
        // ui.add(egui::Slider::new(&mut self.settings.polygon_size, 0.0..=1.0).text("size"));
        // ui.add(egui::Slider::new(&mut self.settings.speed, 0.0..=20.0).text("speed"));
        ui.separator();
        // ui.label(std::format!(
        //     "edge count: {} (rust only for demo purposes)",
        //     self.settings.polygon_edge_count
        // ));
        ui.label(std::format!("framerate: {:.0}fps", self.frame_rate.get()));
    }
}

impl Pipeline {
    /// Create render pipeline.
    /// In debug mode it will return a `PipelineError` if it failed compiling a shader
    /// In release/wasm, il will crash since wgpu does not return errors in such situations.
    fn create_render_pipeline(
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
        uniforms_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<wgpu::RenderPipeline, PipelineError> {
        
        
        
        let shader = ShaderBuilderForLibrary::create_module(device, "glow_shader.wgsl")?;
        // let shader = ShaderBuilder::create_module(device, "test_preprocessor/draw.wgsl")?; // uncomment to test preprocessor

        let swapchain_capabilities = surface.get_capabilities(adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                uniforms_bind_group_layout
            ],
            push_constant_ranges: &[],
        });



        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Ok(pipeline)
    }

    /// Create render pass.
    /// Will return an error in debug, and crash in release/wasm if a shader is malformed.
    fn create_render_pass(
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    ) -> Result<Pass, PipelineError> {
        // let screen_resolution =
        // device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Screen resolution"),
        //     contents: bytemuck::cast_slice(TEST_RESOLUTION),
        //     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        // });
        let screen_resolution = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffers"),
            size: GlowSettings::get_size(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });


        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scene bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let screen_resolution_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene bind_group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_resolution.as_entire_binding(),
                },
            ],
        });

        let pipeline =
            Self::create_render_pipeline(surface, device, adapter, &bind_group_layout)?;

        Ok(Pass {
            pipeline,
            bind_group: screen_resolution_bindgroup,
            uniform_buf: screen_resolution
            //bind_group: uniforms_bind_group,
            //uniform_buf: uniforms,
        })
    }
}
