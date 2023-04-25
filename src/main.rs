use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<MushroomMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(update_mushroom_materials)
        .run();
}

#[derive(Component)]
pub struct Inserted;

#[derive(Component)]
pub struct Mushroom;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("mushroom.glb#Scene0"),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..default()
    });

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

fn update_mushroom_materials(
    mut commands: Commands,
    mut material_events: EventReader<AssetEvent<StandardMaterial>>,
    query: Query<(Entity, &Handle<StandardMaterial>, &Name)>,
    mut mushroom_materials: ResMut<Assets<MushroomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for event in material_events.iter() {
        let handle = match event {
            AssetEvent::Created { handle } => handle,
            _ => continue,
        };

        println!("Handle Found: {:?}", handle);
        let mushroom_handle = mushroom_materials.add(MushroomMaterial {
            base: asset_server.load("mushroom.png"),
            pallete: asset_server.load("mushroom_pal_00.png"),
        });

        for (entity, standard_handle, name) in &query {
            println!("Comparing Handles");
            if standard_handle == handle && name.as_str() == "Cylinder.001" {
                commands.entity(entity).insert(mushroom_handle.clone());
                println!("Replace material for {:?}", name);
            }
        }
    }
}
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a3d71c04-d054-4946-80f8-ba6cfbc90cad"]
struct MushroomMaterial {
    #[texture(1)]
    #[sampler(2)]
    base: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pallete: Handle<Image>,
}

impl Material for MushroomMaterial {
    fn fragment_shader() -> ShaderRef {
        "my_material.wgsl".into()
    }
}
