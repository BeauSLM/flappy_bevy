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
                        .with_system(bird_movement)
                        .with_system(bird_animation))
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

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    ) {
    const BIRD_SPRITES: [&str; 3] = [
        "sprites/bluebird-upflap.png",
        "sprites/bluebird-midflap.png",
        "sprites/bluebird-downflap.png",
    ];
    let mut atlas_builder = TextureAtlasBuilder::default();
    for path in BIRD_SPRITES {
        let handle = assets.get_handle(path);
        let texture = textures.get(handle.clone_weak()).unwrap();
        atlas_builder.add_texture(handle, texture);
    }
    let atlas = atlas_builder.finish(&mut textures).unwrap();
    let _atlas_texture = atlas.texture.clone();
    let atlas_handle = atlases.add(atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let bird_transform = Transform {
        translation: Vec2::ZERO.extend(1.),
        ..Default::default()
    };
    // show spritesheet
    // commands.spawn_bundle(SpriteBundle {
    //     texture: _atlas_texture,
    //     transform: bird_transform,
    //     ..Default::default()
    // });
    commands.spawn_bundle(SpriteSheetBundle {
        transform: bird_transform,
        texture_atlas: atlas_handle,
        ..Default::default()
    })
    .insert(Bird { speed: 0. })
    .insert(Timer::from_seconds(0.1, true));
    commands.spawn_bundle(SpriteBundle {
        texture: assets.get_handle("sprites/background-day.png"),
        ..Default::default()
    });
}

fn bird_animation(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
    // XXX: I think I'm condemmed for this
    mut flap_ix_ix: Local<usize>,
    ) {
    static FLAP_IX: [usize; 4] = [2, 1, 2, 0];
    let (mut timer, mut sprite) = query.single_mut();
    timer.tick(time.delta());
    if timer.finished() {
        // XXX: LISTEN MAN IT MAKES THE BIRD FLAP RIGHT I'M SORRY
        sprite.index = FLAP_IX[*flap_ix_ix];
        *flap_ix_ix = (*flap_ix_ix + 1) % FLAP_IX.len();
    }
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
