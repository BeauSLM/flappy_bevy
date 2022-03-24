use bevy::{
    prelude::*,
    asset::LoadState,
};
use std::f32::consts;
use fastrand;

#[derive(Component)]
struct Bird {
    speed: f32,
}

#[derive(Component)]
struct Pipe;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Waiting,
    Playing,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
struct PipeSpawnLabel;

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
        .add_system_set(SystemSet::on_enter(AppState::Waiting).with_system(setup))
        .add_system_set(SystemSet::on_update(AppState::Playing)
                        .with_system(bird_movement)
                        .with_system(bird_animation)
                        .with_system(spawn_pipes.label(PipeSpawnLabel))
                        .with_system(pipe_movement.after(PipeSpawnLabel))
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

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut state: ResMut<State<AppState>>,
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
    state.set(AppState::Playing).unwrap();
}

struct PipeSpawnTimer {
    timer: Timer
}

impl Default for PipeSpawnTimer {
    fn default() -> Self {
        PipeSpawnTimer {
            timer: Timer::from_seconds(1.25, true),
        }
    }
}

fn spawn_pipes(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut timer: Local<PipeSpawnTimer>,
    ) {
    if timer.timer.tick(time.delta()).finished() {
        // TODO: math out these constants
        const PIPE_GAP: f32 = 450.;
        const PIPE_Y_ADJUST: f32 = -340.;
        const PIPE_Y_RANGE: f32 = 256.;
        let y = fastrand::f32() * PIPE_Y_RANGE + PIPE_Y_ADJUST;
        let mut transform = Transform {
            translation: Vec3::new(200., y, 0.),
            ..Default::default()
        };
        let texture = assets.get_handle("sprites/pipe-green.png");
        let bottom = SpriteBundle {
            texture: texture.clone(),
            transform,
            ..Default::default()
        };

        transform.translation.y += PIPE_GAP;
        let top = SpriteBundle {
            texture,
            transform,
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            ..Default::default()
        };
        commands.spawn_bundle(bottom).insert(Pipe);
        commands.spawn_bundle(top).insert(Pipe);
    }
}

fn pipe_movement(
    mut pipes: Query<(&mut Transform, Entity), With<Pipe>>,
    mut commands: Commands,
    ) {
    for (mut transform, entity) in pipes.iter_mut() {
        let x = &mut transform.translation.x;
        if *x < -500. {
            commands.entity(entity).despawn();
        } else {
            *x -= 2.;
        }
    }
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
    const GRAVITY: f32 = 0.13;
    const FLAP: f32 = 2.9;
    let (mut bird, mut trans) = bird.single_mut();
    if keyboard.pressed(KeyCode::Space) {
        bird.speed = FLAP;
    }
    trans.translation.y += bird.speed;
    let angle = if bird.speed > -1. {
        consts::FRAC_PI_8
    } else {
        (bird.speed + 2.).atan() * 0.8
    };
    trans.rotation = Quat::from_rotation_z(angle);
    bird.speed -= GRAVITY;
    // terminal velocity?
}
