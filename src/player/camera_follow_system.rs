use bevy::prelude::*;
use crate::components::{MainCamera, CameraState, SCREEN_WIDTH, WORLD_WIDTH};
use super::components::*;

/// Camera following system with direction-change animation
pub fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<(&Transform, &PlayerDirection), (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), (With<MainCamera>, Without<Player>)>,
) {
    for (player_transform, player_direction) in player_query.iter() {
        for (mut camera_transform, mut camera_state) in camera_query.iter_mut() {
            let player_x = player_transform.translation.x;
            
            // Calculate ideal camera position for current direction
            let quarter_screen = SCREEN_WIDTH / 4.0;
            
            let ideal_camera_x = if player_direction.facing_right {
                // Player facing right: place player at 1/4 from left edge
                player_x + quarter_screen
            } else {
                // Player facing left: place player at 1/4 from right edge  
                player_x - quarter_screen
            };
            
            // Apply world bounds
            let half_screen_width = SCREEN_WIDTH / 2.0;
            let world_left_bound = half_screen_width;
            let world_right_bound = WORLD_WIDTH - half_screen_width;
            let clamped_ideal = ideal_camera_x.clamp(world_left_bound, world_right_bound);
            
            // Check if direction has changed
            if player_direction.facing_right != camera_state.last_facing_right {
                // Direction changed - start animation
                camera_state.last_facing_right = player_direction.facing_right;
                camera_state.start_x = camera_state.current_x;
                camera_state.target_x = clamped_ideal;
                camera_state.animation_timer = 0.0;
                camera_state.is_animating = true;
            }
            
            if camera_state.is_animating {
                // During animation, continuously update target to follow player movement
                camera_state.target_x = clamped_ideal;
                
                // Currently animating - interpolate towards updated target
                camera_state.animation_timer += time.delta_secs();
                
                if camera_state.animation_timer >= camera_state.animation_duration {
                    // Animation complete - snap to current target and switch to following mode
                    camera_state.current_x = camera_state.target_x;
                    camera_state.is_animating = false;
                } else {
                    // Continue animation with smooth interpolation to updated target
                    let t = camera_state.animation_timer / camera_state.animation_duration;
                    let eased_t = t * t * (3.0 - 2.0 * t); // Smoothstep
                    camera_state.current_x = camera_state.start_x + (camera_state.target_x - camera_state.start_x) * eased_t;
                }
            } else {
                // Not animating - snap to follow player immediately
                camera_state.current_x = clamped_ideal;
            }
            
            // Apply camera position
            camera_transform.translation.x = camera_state.current_x;
        }
    }
}
