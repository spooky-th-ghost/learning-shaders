use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<MushroomMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(mush_material)
        .run();
}

#[derive(Component)]
pub struct Inserted;

#[derive(Component)]
pub struct Mushroom;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MushroomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("mushroom.glb#Scene0"),
            transform: Transform::from_xyz(-3.0, -2.0, 0.0),
            ..default()
        })
        .insert(Mushroom)
        .insert(Name::new("Mushroom Fella"));
    commands.spawn(MaterialMeshBundle::<MushroomMaterial> {
        mesh: meshes.add(Mesh::from(shape::Box::default())),
        material: materials.add(MushroomMaterial { color: Color::GOLD }),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let mut light_transform = Transform::default();
    light_transform.look_at(Vec3::NEG_Y, Vec3::Z);
    //light
    commands.spawn(DirectionalLightBundle {
        transform: light_transform,
        ..default()
    });
}

fn mush_material(
    mut commands: Commands,
    mushroom_query: Query<(Entity, &Name), (Without<Inserted>, With<Mushroom>)>,
    mut mushroom_materials: ResMut<Assets<MushroomMaterial>>,
) {
    for (entity, name) in &mushroom_query {
        // Create a mushroom material
        let mushroom_material = mushroom_materials.add(MushroomMaterial { color: Color::GOLD });

        commands
            .entity(entity)
            .remove::<Handle<StandardMaterial>>()
            .insert(mushroom_material)
            .insert(Inserted);
        println!("Pallete has been swapped for {:?}", name);
    }
}
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct MushroomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for MushroomMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_material.wgsl".into()
    }
}
