use bevy::{ecs::query, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const DEFAULT_COLOR: Color = Color::AQUAMARINE;
#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: DEFAULT_COLOR,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(65.0, 55.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player = query.single_mut();
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::CRIMSON))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}
