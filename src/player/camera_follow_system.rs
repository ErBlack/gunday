use super::components::Player;
use crate::components::{CameraState, MainCamera};
use crate::constants::{SCREEN_WIDTH, WORLD_WIDTH};
use bevy::prelude::*;

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<
        (&mut Transform, &mut CameraState),
        (With<MainCamera>, Without<Player>),
    >,
) {
    let mut player_iter = player_query.iter();
    let Some(player_transform) = player_iter.next() else {
        return;
    };
    debug_assert!(
        player_iter.next().is_none(),
        "Expected exactly one Player entity"
    );
    let player_x = player_transform.translation.x;

    let half_screen_width = SCREEN_WIDTH / 2.0;
    let world_left_bound = half_screen_width;
    let world_right_bound = WORLD_WIDTH - half_screen_width;

    for (mut camera_transform, mut camera_state) in camera_query.iter_mut() {
        if let Some(lock_x) = camera_state.lock_position {
            let final_x = lock_x.clamp(world_left_bound, world_right_bound);
            if final_x != camera_state.current_x {
                camera_state.current_x = final_x;
                camera_transform.translation.x = final_x;
            }
            camera_state.max_reached_x = final_x;
            continue;
        }

        let centered_target = player_x;
        let clamped_target = centered_target.clamp(world_left_bound, world_right_bound);
        let forward_target = clamped_target.max(camera_state.max_reached_x);
        let final_x = forward_target.clamp(world_left_bound, world_right_bound);

        if final_x != camera_state.current_x {
            camera_state.current_x = final_x;
            camera_transform.translation.x = final_x;
        }
        if final_x > camera_state.max_reached_x {
            camera_state.max_reached_x = final_x;
        }
    }
}
