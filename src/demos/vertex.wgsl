@group(0) @binding(0) var<uniform> scale: vec2f;

struct VertexInput {
  @location(0) corner_position: vec2f,
}

struct InstanceInput {
  @location(1) x: f32,
  @location(2) y: f32,
  @location(3) radius: f32,
}

struct VertexOutput {
  @builtin(position) vertex_pos: vec4f,
  @location(0) square_pos: vec2f,
};

@vertex
fn vertex(vertex_in: VertexInput, instance_in: InstanceInput) -> VertexOutput {
  var out: VertexOutput;
  out.square_pos = vertex_in.corner_position;
  out.vertex_pos = vec4f((vertex_in.corner_position * instance_in.radius + vec2f(instance_in.x,instance_in.y)) * scale, 0.0, 1.0);
  return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4f {
  if length(in.square_pos)>=1. {
    discard;
  }
  return vec4f(vec3f(1.), 1.);
}
