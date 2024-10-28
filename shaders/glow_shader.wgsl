struct VertexInput {
    @location(0) position: vec3<f32>,
    // @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

const res: vec2<i32> = vec2(1000, 1000);

// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // out.color = model.color;
    // out.clip_position = vec4<f32>(
    //     model.position.x,
    //     model.position.y,
    //     model.position.z,
    //     1.0
    // );
    out.clip_position = vec4(model.position.xyz, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.clip_position;
    var pos = 0.5 - uv;

    pos.y /= (f32(res.x)/f32(res.y));

    var dist = 1.0/length(pos);

    dist *= 0.1;

    dist = pow(dist, 0.8);

    var col = dist * vec3(1.0, 0.5, 0.25);

    col = 1.0 - exp( -col);

    // return uv;
    return vec4<f32>(col, 1.0);
}
