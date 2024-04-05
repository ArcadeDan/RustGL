use glium::Surface;
use winit::event_loop;

mod geometry;
use geometry::Vertex;

#[macro_use]
extern crate glium;

fn main() {
    //println!("Hello, world!");
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop build");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 matrix;


        void main() {
            vertex_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        
        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;




    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // animation step
    let mut t: f32 = 0.0;

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::Resized(window_sized) => {
                    display.resize(window_sized.into());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let x_off = t.sin() * 0.2;

                    let shape = vec![
                        Vertex {
                            position: [-0.5 + x_off, -0.5],
                            color: [1.0, 0.0, 0.0]
                        },
                        Vertex {
                            position: [0.0 + x_off, 0.5],
                            color: [0.0, 1.0, 0.0]
                        },
                        Vertex {
                            position: [0.5 + x_off, -0.25],
                            color: [0.0, 0.0, 1.0]
                        },
                    ];

                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [x_off, 0.0, 0.0, 1.0f32],
                        ]
                    };



                    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.0, 1.0, 1.0);
                    frame
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    frame.finish().unwrap();
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        };
    });
}
