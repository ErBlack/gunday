use super::components::*;
use crate::components::{CameraState, MainCamera};
use crate::constants::{SCREEN_WIDTH, WORLD_WIDTH};
use bevy::prelude::*;

pub fn player_movement_system(
    time: Res<Time>,
    camera_query: Query<&CameraState, With<MainCamera>>,
    mut player_query: Query<
        (
            &mut Transform,
            &Velocity,
            &crate::player::setup_player::SpriteSize,
            &mut super::components::PlayerPrevPosition,
        ),
        With<Player>,
    >,
) {
    let mut camera_iter = camera_query.iter();
    let camera_left_edge = camera_iter
        .next()
        .map(|state| {
            let half_screen = SCREEN_WIDTH / 2.0;
            let clamped_max = state
                .max_reached_x
                .clamp(half_screen, WORLD_WIDTH - half_screen);
            clamped_max - half_screen
        })
        .unwrap_or(0.0);
    debug_assert!(
        camera_iter.next().is_none(),
        "Expected exactly one MainCamera entity"
    );

    for (mut transform, velocity, sprite_size, mut prev_pos) in player_query.iter_mut() {
        prev_pos.x = transform.translation.x;
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        let world_min_x = sprite_size.width / 2.0;
        let camera_min_x = camera_left_edge + sprite_size.width / 2.0;
        let min_x = camera_min_x.max(world_min_x);
        let max_x = WORLD_WIDTH - sprite_size.width / 2.0;
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}
