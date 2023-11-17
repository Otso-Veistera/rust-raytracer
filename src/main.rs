use image::{ImageBuffer, Rgba};
use nalgebra as na;
use wgpu::{Device, Queue, SwapChain, SwapChainDescriptor, Texture, TextureUsage, TextureView};
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Triangle {
    vertices: [Vec3; 3],
    color: Rgba<u8>,
}

impl Triangle {
    fn new(v1: Vec3, v2: Vec3, v3: Vec3, color: Rgba<u8>) -> Triangle {
        Triangle {
            vertices: [v1, v2, v3],
            color,
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Interactive Triangle Rotation")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let (device, queue) = futures::executor::block_on(async {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            })
            .await
            .unwrap();
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap()
    });

    let swap_chain_descriptor = SwapChainDescriptor {
        usage: TextureUsage::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: WIDTH,
        height: HEIGHT,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    let swap_chain = device.create_swap_chain(&window, &swap_chain_descriptor);

    let mut rotation_angle = 0.0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::DeviceEvent {
                event:
                DeviceEvent::Key(KeyboardInput {
                                     state,
                                     virtual_keycode: Some(VirtualKeyCode::Escape),
                                     ..
                                 }),
                ..
            }
            | Event::WindowEvent {
                event:
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                },
                ..
            } => {
                if *state == ElementState::Pressed {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::DeviceEvent {
                event:
                DeviceEvent::MouseWheel {
                    delta,
                    phase,
                    ..
                },
                ..
            } => {
                if let winit::event::MouseScrollDelta::LineDelta(_, y) = delta {
                    if *phase == winit::event::TouchPhase::Moved {
                        rotation_angle += y;
                        let rotated_triangle = rotate_triangle(rotation_angle);
                        render(&device, &queue, &swap_chain, &rotated_triangle);
                    }
                }
            }
            Event::RedrawRequested(_) => {
                let rotated_triangle = rotate_triangle(rotation_angle);
                render(&device, &queue, &swap_chain, &rotated_triangle);
            }
            _ => (),
        }
    });
}

fn rotate_triangle(angle: f32) -> Triangle {
    let triangle = Triangle::new(
        Vec3 {
            x: -0.5,
            y: -0.5,
            z: 0.0,
        },
        Vec3 {
            x: 0.5,
            y: -0.5,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
        Rgba([255, 0, 0, 255]),
    );

    let mut rotated_vertices = triangle.vertices;

    for vertex in rotated_vertices.iter_mut() {
        let new_x = vertex.x * angle.cos() - vertex.z * angle.sin();
        let new_z = vertex.x * angle.sin() + vertex.z * angle.cos();
        vertex.x = new_x;
        vertex.z = new_z;
    }

    Triangle {
        vertices: rotated_vertices,
        color: triangle.color,
    }
}

fn render(device: &Device, queue: &Queue, swap_chain: &SwapChain, triangle: &Triangle) {
    let frame = swap_chain.get_next_texture().unwrap();
    let output = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        // Render the triangle
        let vertices: Vec<[f32; 3]> = triangle
            .vertices
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .collect();
        render_pass.set_vertex_buffer(0, device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsage::VERTEX,
        }));

        render_pass.set_pipeline(&device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("vertex_shader.wgsl").into()),
                    flags: wgpu::ShaderFlags::empty(),
                }),
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Fragment Shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("fragment_shader.wgsl").into()),
                    flags: wgpu::ShaderFlags::empty(),
                }),
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: swap_chain_descriptor.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                strip_index_format: None,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        }));

        render_pass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));
}
