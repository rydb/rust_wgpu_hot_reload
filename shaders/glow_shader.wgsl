struct Settings {
    res: vec2<f32>
}

@group(0) @binding(0)
var<uniform> settings: Settings;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

/// signed distance field
fn sdfCircle(p: vec2<f32>, r: f32) -> f32 {
  return length(p) - r;
}


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
    var white = vec4(1.0, 1.0, 1.0, 1.0);
    var color = vec4(0.0, 0.0, 0.0, 0.0);
    var uv = vec2(in.clip_position.xy / settings.res);
    uv = uv - 0.5;
    uv = uv * settings.res / 100.0;

    var radius = 1.0;
    var center = vec2(0.0, 0.0);

    var distanceToCircle = sdfCircle(uv - center, radius);

    if distanceToCircle > 0.0 {
        color = vec4(0.0, 0.0, 1.0, 1.0);
    } else {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    
    color = color * exp(distanceToCircle);
    
    color = color * (1.0 - exp(-2.0 * abs(distanceToCircle)));
    
    //color = mix(white, color, 2.0 * abs(distanceToCircle));
    //color = vec4<f32>(uv.x, uv.y, 0.0, 0.0);

    return color;

    // pos.y /= (f32(uniforms.res_x)/f32(uniforms.res_y));

    // var dist = 1.0/length(pos);

    // dist *= 0.1;

    // dist = pow(dist, 0.8);

    // var col = dist * vec3(1.0, 0.5, 0.25);

    // col = 1.0 - exp( -col);

    // // return uv;
    // return vec4<f32>(col, 1.0);
}
