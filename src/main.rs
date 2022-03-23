use bevy::prelude::*;

#[derive(Component)]
struct Bird {
    speed: f32,
}

fn main() {
    App::new()
        // TODO: load in background then size window to it
        .insert_resource(WindowDescriptor {
            width: 288.,
            height: 512.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bird_movement)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("sprites/bluebird-midflap.png"),
        ..Default::default()
    })
    .insert(Bird { speed: 0. });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("sprites/background-day.png"),
        ..Default::default()
    });
}

fn bird_movement(
    mut bird: Query<(&mut Bird, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    ) {
    const GRAVITY: f32 = 0.15;
    const FLAP: f32 = 3.;
    let (mut bird, mut trans) = bird.single_mut();
    if keyboard.pressed(KeyCode::Space) {
        bird.speed = FLAP;
    }
    trans.translation.y += bird.speed;
    bird.speed -= GRAVITY;
    // terminal velocity?
}
