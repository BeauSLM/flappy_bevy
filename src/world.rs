use fastrand;
use super::*;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
struct Base;

const WORLD_SPEED: f32 = 1.5;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct PipeSpawnLabel;

pub struct PipeSpawnTimer {
    timer: Timer
}

impl Default for PipeSpawnTimer {
    fn default() -> Self {
        PipeSpawnTimer {
            timer: Timer::from_seconds(1.25, true),
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

pub fn pipe_movement(
    mut pipes: Query<(&mut Transform, Entity), With<Pipe>>,
    mut commands: Commands,
    ) {
    for (mut transform, entity) in pipes.iter_mut() {
        let x = &mut transform.translation.x;
        if *x < -500. {
            commands.entity(entity).despawn();
        } else {
            *x -= WORLD_SPEED;
        }
    }
}

fn _base_movement(
    mut _bases: Query<(&mut Transform, Entity), With<Base>>,
    mut _commands: Commands,
    ) {
    todo!();
}

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    ) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.get_handle("sprites/background-day.png"),
        ..Default::default()
    });
}
