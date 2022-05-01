use fastrand;
use super::*;

// XXX: World is a REALLY IMPORTANT TYPE in bevy so name this something better ffs
#[derive(Component)]
pub struct World;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct Base;

const WORLD_SPEED: f32 = -125.;
const BASE_WIDTH: f32 = 336.;
const BASE_OFFSCREEN_X: f32 = -(BASE_WIDTH - (BASE_WIDTH - WORLD_WIDTH) / 2.);
const PIPE_OFFSCREEN_X: f32 = -(52. + WORLD_WIDTH) / 2.;

pub struct PipeSpawnTimer {
    timer: Timer
}

impl Default for PipeSpawnTimer {
    fn default() -> Self {
        PipeSpawnTimer {
            timer: Timer::from_seconds(1.5, true),
        }
    }
}

pub fn spawn_pipes(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut timer: Local<PipeSpawnTimer>,
    ) {
    if timer.timer.tick(time.delta()).finished() {
        // TODO: math out these constants
        const PIPE_GAP: f32 = 450.;
        const PIPE_Y_ADJUST: f32 = -350.;
        const PIPE_Y_RANGE: f32 = 256.;
        let y = fastrand::f32() * PIPE_Y_RANGE + PIPE_Y_ADJUST;
        let mut transform = Transform {
            translation: Vec3::new(200., y, 10.),
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
        commands.spawn_bundle(bottom).insert(Pipe).insert(World);
        commands.spawn_bundle(top).insert(Pipe).insert(World);
    }
}

pub fn despawn_pipes(
    pipes: Query<(&Transform, Entity), With<Pipe>>,
    mut commands: Commands,
    ) {
    for (transform, entity) in pipes.iter() {
        if transform.translation.x < PIPE_OFFSCREEN_X {
            commands.entity(entity).despawn();
        }
    }
}

pub fn world_movement(
    mut query: Query<&mut Transform, With<World>>,
    time: Res<Time>,
    ) {
    for mut transform in query.iter_mut() {
        transform.translation.x += WORLD_SPEED * time.delta_seconds();
    }
}

pub fn base_leapfrog(mut query: Query<&mut Transform, With<Base>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x < BASE_OFFSCREEN_X {
            transform.translation.x = BASE_WIDTH;
        }
    }
}

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    ) {
    const BASE_Y: f32 = -260.;
    commands.spawn_bundle(SpriteBundle {
        texture: assets.get_handle("sprites/background-day.png"),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: assets.get_handle("sprites/base.png"),
        transform: Transform {
            translation: Vec3::new(0., BASE_Y, 10.),
            ..default()
        },
        ..Default::default()
   })
    .insert(Base)
    .insert(World);
    commands.spawn_bundle(SpriteBundle {
        texture: assets.get_handle("sprites/base.png"),
        transform: Transform {
            translation: Vec3::new(BASE_WIDTH, BASE_Y, 10.),
            ..default()
        },
        ..Default::default()
    })
    .insert(Base)
    .insert(World);
}
