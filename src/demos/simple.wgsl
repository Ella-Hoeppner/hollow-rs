@group(0) @binding(0) var<uniform> dimensions: vec2f;
@group(0) @binding(1) var<uniform> time: f32;

struct VertexInput {
    @location(0) corner_position: vec2f,
}

@vertex
fn vertex(in: VertexInput) -> @builtin(position) vec4f {
    return vec4f(in.corner_position, 0.0, 1.0);
}

@fragment
fn fragment(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    let osc = sin(time * 9.) * 0.5 + 0.5;
    return vec4f(pow(vec3f(pos.xy / dimensions, osc), vec3f(2.2)), 1.);
}
