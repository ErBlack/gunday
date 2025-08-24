use bevy::prelude::*;
use crate::components::{Ground, GROUND_HEIGHT};
use super::components::*;

/// Handle collision with ground
pub fn ground_collision_system(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
    ground_query: Query<&Transform, (With<Ground>, Without<Player>)>,
) {
    for (mut player_transform, mut velocity, mut grounded) in player_query.iter_mut() {
        for ground_transform in ground_query.iter() {
            let player_bottom = player_transform.translation.y - (PLAYER_HEIGHT / 2.0); // Bottom of player
            let ground_top = ground_transform.translation.y + (GROUND_HEIGHT / 2.0); // Top of ground

            // Check if player is touching or below ground
            if player_bottom <= ground_top {
                // Place player on top of ground (bottom of player touching top of ground)
                player_transform.translation.y = ground_top + (PLAYER_HEIGHT / 2.0);
                
                // Stop downward velocity
                if velocity.y < 0.0 {
                    velocity.y = 0.0;
                }
                
                grounded.is_grounded = true;
            } else {
                grounded.is_grounded = false;
            }
        }
    }
}
