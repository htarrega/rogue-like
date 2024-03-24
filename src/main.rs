use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const PLAYER_SPEED: f32 = 500.0;

const DEFAULT_COLOR: Color = Color::AQUAMARINE;
#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
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
        })
        .insert(Player);
}

fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player = query.single_mut();
    let key = input.get_pressed().next();
    let mut movement = Vec2::ZERO;

    if let Some(key) = key {
        match key {
            KeyCode::ArrowUp => movement.y += PLAYER_SPEED * time.delta_seconds(),
            KeyCode::ArrowDown => movement.y -= PLAYER_SPEED * time.delta_seconds(),
            KeyCode::ArrowLeft => movement.x -= PLAYER_SPEED * time.delta_seconds(),
            KeyCode::ArrowRight => movement.x += PLAYER_SPEED * time.delta_seconds(),
            k => println!("Other key: {:?}", k),
        }

        player.translation.x += movement.x;
        player.translation.y += movement.y;
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::CRIMSON))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, player_input)
        .run();
}
