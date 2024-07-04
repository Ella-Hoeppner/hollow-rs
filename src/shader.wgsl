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
  let pos = in.vertex_pos.xy/dimensions;
  return vec4f(pow(vec3f(pos, 0.),vec3f(2.2)), 1.);
}
