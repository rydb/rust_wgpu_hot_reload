struct Settings {
    res_x: f32,
    res_y: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Settings;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};


// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4(model.position.xyz, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.clip_position;
    var pos = 0.5 - uv;

    pos.y /= (f32(uniforms.res_x)/f32(uniforms.res_y));

    var dist = 1.0/length(pos);

    dist *= 0.1;

    dist = pow(dist, 0.8);

    var col = dist * vec3(1.0, 0.5, 0.25);

    col = 1.0 - exp( -col);

    // return uv;
    return vec4<f32>(col, 1.0);
}
