struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(2) var<uniform> size: vec2<f32>;


@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    var scale = 1.25;

    var aspect = size.x / size.y;

    out.position = vec4(
        in.position.x * scale,
        in.position.y * aspect * scale,
        in.position.z,
        1.0,
    );
    out.uv = in.uv;

    return out;
}