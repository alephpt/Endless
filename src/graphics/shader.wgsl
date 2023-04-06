// Vertex Shader

struct VertexIn {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOut{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vertex_main(model: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.color = model.color;
    out.clip_position = model.position;
    return out;
}

// Fragment Shader

@fragment
fn fragment_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}