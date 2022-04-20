pub use bevy::{
    prelude::*,
    asset::LoadState,
};
use crate::bird::*;
use crate::world::*;
mod bird;
mod world;

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Waiting,
    Playing,
}

pub fn run() {
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
        .add_system_set(SystemSet::on_enter(AppState::Waiting)
            .with_system(bird::setup)
            .with_system(world::setup)
        )
        .add_system_set(SystemSet::on_update(AppState::Waiting)
            .with_system(bird_hover)
            .with_system(check_waiting)
        )
        .add_system_set(SystemSet::on_update(AppState::Playing)
                        .with_system(bird_movement)
                        .with_system(bird_animation)
                        .with_system(spawn_pipes)
                        .with_system(pipe_movement.after(spawn_pipes))
                        )
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
        state.set(AppState::Waiting).unwrap();
    }
}

fn check_waiting(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if !keys.pressed(KeyCode::Space) { return; }
    state.set(AppState::Playing).unwrap();
}
