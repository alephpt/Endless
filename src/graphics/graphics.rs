
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
use crate::graphics::Vertex;
use crate::graphics::Position;
use crate::graphics::Cube;


#[derive(Debug)]
pub struct Mouse {
    pub mouse_position: winit::dpi::PhysicalPosition<f64>,
    pub prev_mouse_position: winit::dpi::PhysicalPosition<f64>,
    pub l_mouse_down: bool,
    pub m_mouse_down: bool,
    pub r_mouse_down: bool,
}

#[derive(Debug)]
pub struct Graphics {
    pub window: Window,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub mouse_state: Mouse,
    pub cube: Cube,
    pub n_vertices: u32,
    pub n_indices: u32,
}

impl Graphics {
    pub async fn new(window: Window, cube: Cube) -> Self {
        const WINDOW_HEIGHT: u32 = 1200;
        const WINDOW_WIDTH: u32 = 1600;
        let n_vertices: u32 = cube.mesh.vertices.len() as u32;
        let n_indices = cube.mesh.indices.len() as u32;

        // Initialize logger
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }

        // Initialize wgpu
        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            use winit::platform::web::WindowExtWebSys;


            window.set_inner_size(PhysicalSize::new(450, 600));
            
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("gfx")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }
        
        // set the window size
        window.set_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

        // get the window size
        let size = window.inner_size();

        // create the wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // create the wgpu surface
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // get the wgpu adapter
        let adapter = instance.request_adapter(
                                    &wgpu::RequestAdapterOptions {
                                        power_preference: wgpu::PowerPreference::default(),
                                        compatible_surface: Some(&surface),
                                        force_fallback_adapter: false,
                                    },
                                ).await.unwrap();
        
        // create the wgpu device and queue
        let (device, queue) = adapter.request_device(
                                          &wgpu::DeviceDescriptor {
                                              features: wgpu::Features::empty(),
                                              limits: if cfg!(target_arch = "wasm32") {
                                                wgpu::Limits::downlevel_webgl2_defaults()
                                                } else {
                                                    wgpu::Limits::default()
                                                },
                                              label: None
                                          }, None)
                                      .await.unwrap();

        // create the wgpu surface configuration
        let capabilities = surface.get_capabilities(&adapter);

        // get the surface format
        let surface_format = capabilities.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(capabilities.formats[0]);

        // create the wgpu surface configuration
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        // create the wgpu shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // create the wgpu render pipeline
        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
     
        // create the wgpu render pipeline for our shader
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex_main", 
                buffers: &[Vertex::desc(),], 
            },
            fragment: Some(wgpu::FragmentState { 
                module: &shader,
                entry_point: "fragment_main",
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
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
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

        // create the vertex buffer that will be used to draw our shapes
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&cube.mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // create the index buffer that will be used to draw our shapes
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&cube.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        // configure the surface
        surface.configure(&device, &config);
        
        // create the mouse state
        let mouse_state = Mouse {
            mouse_position: winit::dpi::PhysicalPosition::new(0.0, 0.0),
            prev_mouse_position: winit::dpi::PhysicalPosition::new(0.0, 0.0),
            l_mouse_down: false,
            m_mouse_down: false,
            r_mouse_down: false,
        };

        Self {
            window,
            surface,
            device,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            queue,
            config,
            size,
            mouse_state,
            cube,
            n_vertices,
            n_indices,
        }
    }

    pub fn new_window(event_loop: &EventLoop<()>) -> Window {
        WindowBuilder::new().build(event_loop).unwrap()
    }

    pub fn input(&mut self, event: &WindowEvent) {
        if let WindowEvent::CursorMoved { position, .. } = event {
            self.mouse_state.mouse_position = *position;
            // if wasm then console.log
            #[cfg(target_arch = "wasm32")]
            {
                web_sys::console::log_1(&format!("Mouse position: {:?}", self.mouse_state.mouse_position).into());
            }
        }

        if let WindowEvent::MouseInput { state, button, .. } = event {
            match button {
                MouseButton::Left => {
                    self.mouse_state.l_mouse_down = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };
                    println!("Left mouse button pressed: {}", self.mouse_state.l_mouse_down);
                    // if wasm then console.log
                    #[cfg(target_arch = "wasm32")]
                    {
                        web_sys::console::log_1(&"Left mouse button pressed".into());
                    }
                },
                MouseButton::Middle => {
                    self.mouse_state.m_mouse_down = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };
                    println!("Middle mouse button pressed: {}", self.mouse_state.m_mouse_down);
                    // if wasm then console.log
                    #[cfg(target_arch = "wasm32")]
                    {
                        web_sys::console::log_1(&"Middle mouse button pressed".into());
                    }
                },
                MouseButton::Right => {
                    self.mouse_state.r_mouse_down = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };
                    println!("Right mouse button pressed: {}", self.mouse_state.r_mouse_down);
                    // if wasm then console.log
                    #[cfg(target_arch = "wasm32")]
                    {
                        web_sys::console::log_1(&"Right mouse button pressed".into());
                    }
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        let current_mouse_pos = self.mouse_state.mouse_position;

        // if the mouse button is pushed down
        if self.mouse_state.l_mouse_down {

            // calculate dx and dy
            let dx = (current_mouse_pos.x - self.mouse_state.prev_mouse_position.x) as f32;
            let dy = (current_mouse_pos.y - self.mouse_state.prev_mouse_position.y) as f32;

            // check if dx and dy are 0
            if dx == 0.0 && dy == 0.0 {
                return;
            }

            // find the normalized direction of the mouse movement
            let magnitude = (dx * dx + dy * dy).sqrt();
            let x = dy / magnitude as f32;
            let y = dx / magnitude as f32;

            // check that dx and dy are numbers
            if x.is_nan() || y.is_nan() {
                return;
            }

            // create xyz axis
            let axis = Position::new(x, y, 0.0, 1.0);

            // convert x and y displacement to an angle in degrees
            let angle = (magnitude / 100.0) * 360.0  * 0.01;

            // rotate the mesh based on the mouse position against the previous mouse position
            self.cube.rotate(
                angle,
                axis
            );

            // println!("dx: {}, dy: {}, magnitude: {}, x: {}, y: {}, angle: {}", dx, dy, magnitude, x, y, angle);
            // println!("Cube verts: {:?}", self.cube.mesh.vertices);

            // recreate the vertex buffer
            self.vertex_buffer = self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&self.cube.mesh.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            );
        }

        // update the previous mouse position
        self.mouse_state.prev_mouse_position = current_mouse_pos;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        
        // allows for resize if using wasm in the browser
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(&format!("Resized to: {:?}", self.size).into());
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    })
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline); 
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.n_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run(cube: Cube) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let window = Graphics::new_window(&event_loop);
    let mut graphics = Graphics::new(window, cube).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == graphics.window.id() => { // UPDATED!
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    graphics.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    graphics.resize(**new_inner_size);
                },
                WindowEvent::MouseInput { .. } | WindowEvent::CursorMoved { .. } => {
                    graphics.input(event);
                },
                _ => {}
            }
        },
        Event::RedrawRequested(_) => {
            graphics.update();
            match graphics.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => graphics.resize(graphics.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
            
        },
        Event::MainEventsCleared => {
            graphics.window.request_redraw();
        },
        _ => {}
    });
}