use super::*;
use bevy::sprite::collide_aabb;

pub fn bird_collision(
mut state: ResMut<State<AppState>>,
    bird: Query<&Transform, With<Bird>>,
    collider: Query<&Transform, With<super::world::World>>,
) {
    let bird = bird.single();
    let bird_translation = bird.translation;
    if !collider.iter().any(|c| {
        // check if bird collided with wall or floor
        collide_aabb::collide(
            bird_translation,
            Vec2::new(34., 24.),
            c.translation,
            Vec2::new(52., 320.),
        )
        .is_some()
    }) { return; }
    state.set(AppState::GameOver).unwrap();
}
