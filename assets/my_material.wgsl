#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::pbr_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::pbr_ambient
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions

@group(1) @binding(1)
var my_texture: texture_2d<f32>;
@group(1) @binding(2)
var tex_sampler: sampler;
@group(1) @binding(3)
var pallete: texture_2d<f32>;
@group(1) @binding(4)
var pallete_sampler: sampler;

struct VertexOutput {
	#import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
	let index = textureSample(my_texture, tex_sampler, input.uv).r * 3.0;
	//let output_color = textureSample(pallete, pallete_sampler, vec2((index+ 0.5) / 4.0, 0.5));
	let output_color = textureSample(pallete, pallete_sampler, vec2(index/ 4.0, 0.5));
	let normal = normalize(input.world_normal);
	//return output_color * vec4(normal, 0.0);
	return output_color;
}
