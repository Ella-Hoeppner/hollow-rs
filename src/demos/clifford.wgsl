@group(0) @binding(0) var<uniform> scale: vec2f;
@group(1) @binding(0) var<storage, read> points: array<vec2f>;

struct VertexInput {
  @location(0) corner_position: vec2f,
}

struct VertexOutput {
  @builtin(position) vertex_pos: vec4f,
  @location(0) square_pos: vec2f,
};

@vertex
fn vertex(@builtin(instance_index) instance_index: u32, vertex_in: VertexInput) -> VertexOutput {
  return VertexOutput(
    vec4f((points[instance_index] + vertex_in.corner_position * 0.025) * scale, 0.0, 1.0),
    vertex_in.corner_position
  );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4f {
  if length(in.square_pos)>=1. {
    discard;
  }
  return vec4f(vec3f(1.), 1.);
}
