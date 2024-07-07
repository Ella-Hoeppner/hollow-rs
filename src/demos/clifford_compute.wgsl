@group(0) @binding(0) var<storage, read_write> points: array<vec2f>;

fn clifford(x: vec2f, a: f32, b: f32, c: f32, d: f32) -> vec2f {
  return vec2f(
    sin(a*x.y)+c*cos(a*x.x),
    sin(b*x.x)+d*cos(b*x.y)
  );
}

@compute @workgroup_size(256)
fn compute(@builtin(global_invocation_id) id: vec3<u32>) {
  let i=id.x;
  points[i] = clifford(points[i], -1.4, 1.6, 1., 0.7);
}
