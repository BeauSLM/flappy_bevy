use bevy::{
    prelude::*,
    asset::LoadState,
};

#[derive(Component)]
struct Bird {
    speed: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn main() {
    App::new()
        // TODO: load in background then size window to it
        .insert_resource(WindowDescriptor {
            width: 288.,
            height: 512.,
            ..Default::default()
        })
        .init_resource::<SpriteHandles>()
        .add_state(AppState::Setup)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_sprites))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_loading))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .add_system_set(SystemSet::on_update(AppState::Finished)
                        .with_system(bird_movement))
        .run();
}

fn load_sprites(mut sprite_handles: ResMut<SpriteHandles>, assets: Res<AssetServer>) {
    sprite_handles.handles = assets.load_folder("sprites").unwrap();
}

fn check_loading(
    mut state: ResMut<State<AppState>>,
    sprites: ResMut<SpriteHandles>,
    assets: Res<AssetServer>,
    ) {
    if let LoadState::Loaded = assets.get_group_load_state(sprites.handles.iter().map(|handle| handle.id)) {
        state.set(AppState::Finished).unwrap();
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: assets.get_handle("sprites/bluebird-midflap.png"),
        ..Default::default()
    })
    .insert(Bird { speed: 0. });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.get_handle("sprites/background-day.png"),
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
