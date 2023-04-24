#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_types

#import bevy_pbr::utils


struct MyMat {
	color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> uniform_data: MyMat;

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) world_position: vec4<f32>,
	@location(1) world_normal: vec3<f32>,
	@location(2) uv: vec2<f32>,
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
	var output_color = vec4<f32>(input.uv,0.0,1.0);
	output_color = uniform_data.color / 5.0;
	return output_color;
}
