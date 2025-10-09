@group(0) @binding(0) var<storage, read_write> points: array<vec2f>;

override A: f32 = -1.4;
override B: f32 = 1.6;
override C: f32 = 1.;
override D: f32 = 0.7;

fn clifford(x: vec2f) -> vec2f {
    return vec2f(
        sin(A * x.y) + C * cos(A * x.x),
        sin(B * x.x) + D * cos(B * x.y)
    );
}

@compute @workgroup_size(256)
fn compute(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    points[i] = clifford(points[i]);
}
