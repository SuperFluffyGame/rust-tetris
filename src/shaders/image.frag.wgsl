struct FragInput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var t: texture_2d<f32>;
@group(0) @binding(1) var s: sampler;

@fragment
fn fs_main( in: FragInput) -> @location(0) vec4<f32> {
    return textureSample(t, s, in.uv);
}