#[macro_use]
extern crate glium;
extern crate glutin;
extern crate ezing;

#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn draw_lines() -> Vec<Vertex> {
  let vpad = 0.02;
  let hpad = 0.02;
  let stroke = 0.01;

  let fns = &[
    ("quad", [ezing::quad_in, ezing::quad_out, ezing::quad_inout]),
    ("cubic", [ezing::cubic_in, ezing::cubic_out, ezing::cubic_inout]),
    ("quart", [ezing::quart_in, ezing::quart_out, ezing::quart_inout]),
    ("quint", [ezing::quint_in, ezing::quint_out, ezing::quint_inout]),
    ("sine", [ezing::sine_in, ezing::sine_out, ezing::sine_inout]),
    ("circ", [ezing::circ_in, ezing::circ_out, ezing::circ_inout]),
    ("expo", [ezing::expo_in, ezing::expo_out, ezing::expo_inout]),
    ("elastic", [ezing::elastic_in, ezing::elastic_out, ezing::elastic_inout]),
    ("back", [ezing::back_in, ezing::back_out, ezing::back_inout]),
    ("bounce", [ezing::bounce_in, ezing::bounce_out, ezing::bounce_inout]),
  ];

  let width = (1.0 - 4.0 * hpad) / 3.0;
  let height = (1.0 - ((fns.len() + 1) as f32 * vpad)) / fns.len() as f32;

  let mut vertices = vec![];

  for (i, row) in fns.iter().enumerate() {
    let top = (height + vpad) * i as f32 + vpad;

    for (j, func) in row.1.iter().enumerate() {
      let left = (width + hpad) * j as f32 + hpad;

      let vertex = |x: f32, y: f32| -> Vertex {
        Vertex{ position: [x * width + left, (1.0 - y) * height + top] }
      };

      let n = 100;

      vertices.push(vertex(0.0, 0.0));
      vertices.push(vertex(0.0, 0.0));

      for i in 1..n {
        let x = i as f32 / n as f32;

        let y = func(x);
        vertices.push(vertex(x, y + stroke));
        vertices.push(vertex(x, y - stroke));
      }

      vertices.push(vertex(1.0, 1.0));
      vertices.push(vertex(1.0, 1.0));
    }
  }

  vertices
}

fn main() {
  use glium::DisplayBuild;

  let display = glium::glutin::WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("ezing demo")
    .with_vsync()
    .with_multisampling(8)
    .build_glium()
    .unwrap();

  let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

  let vertices = draw_lines();

  loop {
    for event in display.poll_events() {
      match event {
        glium::glutin::Event::Closed
          | glium::glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => return,
        _ => {},
      }
    }

    {
      use glium::Surface;
      let mut target = display.draw();

      let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
      let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

      target.clear_color(1.0, 1.0, 1.0, 1.0);

      target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default()).unwrap();

      target.finish().unwrap();
    }
  }
}

const VERTEX_SHADER: &'static str = r#"
  #version 140

  in vec2 position;

  void main() {
    vec2 full = position * vec2(2, -2) + vec2(-1, 1);
    gl_Position = vec4(full, 0.0, 1.0);
  }
"#;

const FRAGMENT_SHADER: &'static str = r#"
  #version 140

  out vec4 color;

  void main() {
    color = vec4(0.0, 0.0, 0.0, 1.0);
  }
"#;
