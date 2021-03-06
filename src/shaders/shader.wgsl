struct CameraUniform {
    view_projection_matrix: mat4x4<f32>;
};

struct WorldTransformUniform {
    transform_matrix: mat4x4<f32>;
};

[[group(1), binding(0)]]
var<uniform> camera: CameraUniform;
[[group(2), binding(0)]]
var<uniform> world: WorldTransformUniform;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] texture_coordinates: vec2<f32>;
};


struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] texture_coordinates: vec2<f32>;
};

[[stage(vertex)]]
fn vertex_shader_main(
    model: VertexInput,
) -> VertexOutput {
    var outVertex: VertexOutput;
    outVertex.texture_coordinates = model.texture_coordinates;
    outVertex.clip_position = camera.view_projection_matrix * world.transform_matrix * vec4<f32>(model.position, 1.0);
    return outVertex;
}

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fragment_shader_main(inputVertex: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, inputVertex.texture_coordinates);
}
