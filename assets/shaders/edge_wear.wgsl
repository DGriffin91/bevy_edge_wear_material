#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

struct Material {
    wear: f32,
    corner_wear: f32,
}

@group(1) @binding(0)
var<uniform> material: Material;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var roughness_texture: texture_2d<f32>;
@group(1) @binding(4)
var roughness_sampler: sampler;
@group(1) @binding(5)
var noise_texture: texture_2d<f32>;
@group(1) @binding(6)
var noise_sampler: sampler;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

    // Sample textures
    var color = textureSample(base_color_texture, base_color_sampler, in.uv).rgb;
    var roughness = textureSample(roughness_texture, roughness_sampler, in.uv).x;
    var noise = textureSample(noise_texture, noise_sampler, in.uv).x;
    
    // grayscale version of base_color_texture
    let gray = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722)); 

    var fade = in.uv.x * in.uv.y * (1.0 - in.uv.y) * (1.0 - in.uv.x);

    fade = pow(fade, 2.0) * 5000.0; // Adjust brightness/contrast

    let noise1 = pow(noise, 2.0) * 500.0; // Adjust brightness/contrast

    var noise_fade = saturate(fade * noise1); // saturate here means clamp 0.0-1.0

    //blend in greyscale corner wear
    color = mix(color, vec3<f32>(gray), (1.0 - noise_fade) * material.corner_wear); 

    
    color = mix(color, color * noise * 20.0, material.wear); // blend in wear

    color = pow(color, vec3<f32>(1.3)) * vec3<f32>(3.0); // Adjust brightness/contrast

    roughness = saturate(roughness * gray * 10.0); // vary roughness
    
    
    // --- Minimal PBR boilerplate ---
    
    var pbr_input: PbrInput;

    pbr_input.material.base_color = vec4<f32>(color, 1.0);

    pbr_input.material.reflectance = 0.5;
    pbr_input.material.alpha_cutoff = 0.0;
    pbr_input.material.flags = 16u;
    pbr_input.material.emissive = vec4<f32>(0.0,0.0,0.0,1.0);
    pbr_input.material.metallic = 0.0;
    pbr_input.material.perceptual_roughness = roughness;

    pbr_input.occlusion = 1.0;
    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = prepare_normal(0u, in.world_normal, in.uv, in.is_front);
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    let output_color = pbr(pbr_input);

    return tone_mapping(pbr(pbr_input));

}