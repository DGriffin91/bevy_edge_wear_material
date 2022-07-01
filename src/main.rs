use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<WearMaterial>::default())
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WearMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load Textures
    let base_color_texture = asset_server.load("textures/Wood_Crate_001_basecolor.jpg");
    let roughness_texture = asset_server.load("textures/Wood_Crate_001_roughness.jpg");
    let noise_texture = asset_server.load("textures/Metal_scratched_009_roughness.jpg");

    let material = materials.add(WearMaterial {
        wear: 0.0,
        corner_wear: 0.0,
        base_color_texture: Some(base_color_texture),
        roughness_texture: Some(roughness_texture),
        noise_texture: Some(noise_texture),
    });

    // cube
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material,
        ..default()
    });

    // light 1
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-5.0, 2.0, 10.0)),
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        ..default()
    });

    // light 2
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(7.0, 2.0, -2.0)),
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Rotates the cube, oscillates wear & corner_wear
fn rotate(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Handle<Mesh>>>,
    mut materials: ResMut<Assets<WearMaterial>>,
    material_handles: Query<&Handle<WearMaterial>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(0.25 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_z(0.25 * time.delta_seconds());
    }
    for mat_handle in material_handles.iter() {
        if let Some(mut mat) = materials.get_mut(mat_handle) {
            let t = time.seconds_since_startup() as f32;
            mat.wear = (t.sin() + 1.0) / 2.0; // map sin to 0.0 - 1.0
            mat.corner_wear = ((t * 0.33).sin() + 1.0) / 2.0;
        }
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for WearMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/edge_wear.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "1b2a3c6b-6642-4f87-e2df-24816dffcd82"]
pub struct WearMaterial {
    #[uniform(0)]
    wear: f32,
    #[uniform(0)]
    corner_wear: f32,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub roughness_texture: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    pub noise_texture: Option<Handle<Image>>,
}
