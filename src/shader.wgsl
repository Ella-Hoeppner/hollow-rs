@group(0) @binding(0) var<uniform> dimensions: vec2f;

// Forms

struct VertexInput {
  @location(0) corner_position: vec2f,
}

struct VertexOutput {
  @builtin(position) vertex_pos: vec4f,
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.vertex_pos = vec4f(in.corner_position, 0.0, 1.0);
  return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4f {
  return vec4f(1., 0., 0., 1.);
}
