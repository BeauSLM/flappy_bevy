use std::f32::consts;
use super::*;

#[derive(Component)]
pub struct Bird {
    speed: f32,
    flap_time: Timer,
}

impl Default for Bird {
    fn default() -> Self {
        Bird {
            speed: 0.,
            flap_time: Timer::from_seconds(1., true),
        }
    }
}

pub fn bird_animation(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Bird)>,
    // XXX: I think I'm condemmed for this
    mut flap_ix_ix: Local<usize>,
    ) {
    static FLAP_IX: [usize; 4] = [2, 1, 2, 0];
    let (mut sprite, mut bird) = query.single_mut();
    bird.flap_time.tick(time.delta());
    if bird.flap_time.finished() {
        // XXX: LISTEN MAN IT MAKES THE BIRD FLAP RIGHT I'M SORRY
        sprite.index = FLAP_IX[*flap_ix_ix];
        *flap_ix_ix = (*flap_ix_ix + 1) % FLAP_IX.len();
    }
}

pub fn bird_hover(
    mut bird: Query<&mut Transform, With<Bird>>,
    mut coef: Local<f32>,
    time: Res<Time>
    ) {
    const AMPLITUDE: f32 = 3.5;
    bird.single_mut().translation.y = AMPLITUDE * coef.sin();
    *coef += 8.5 * time.delta_seconds();
    *coef %= consts::TAU;
}

pub fn bird_gravity(mut bird: Query<&mut Bird>) {
    const GRAVITY: f32 = 10.;
    let mut bird = bird.single_mut();
    bird.speed -= GRAVITY;
}

pub fn bird_flap(
    mut bird: Query<&mut Bird>,
    keyboard: Res<Input<KeyCode>>,
    ) {
    const FLAP: f32 = 300.;
    let mut bird = bird.single_mut();
    if keyboard.pressed(KeyCode::Space) {
        bird.speed = FLAP;
    }
}

pub fn bird_movement(
    mut bird: Query<(&mut Bird, &mut Transform)>,
    time: Res<Time>,
) {
    let (bird, mut trans) = bird.single_mut();
    let angle = if bird.speed > -300. {
        consts::FRAC_PI_8
    } else {
        (bird.speed * time.delta_seconds() / 3.).atan()
    };
    trans.rotation = Quat::from_rotation_z(angle);
    trans.translation.y += bird.speed * time.delta_seconds();
}

pub fn setup(
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
    .insert(Bird::default());
}