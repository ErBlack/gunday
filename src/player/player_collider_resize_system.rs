use super::components::{Grounded, Player};
use crate::player::PLAYER_CONFIG;
use bevy::prelude::*;

pub fn player_collider_resize_system(
    mut query: Query<
        (
            &mut super::setup_player::SpriteSize,
            &Grounded,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    for (mut size, grounded, mut transform) in query.iter_mut() {
        let target = if grounded.is_grounded {
            (
                PLAYER_CONFIG.ground_collider.x,
                PLAYER_CONFIG.ground_collider.y,
            )
        } else {
            (PLAYER_CONFIG.air_collider.x, PLAYER_CONFIG.air_collider.y)
        };
        if (size.width, size.height) != target {
            let old_half = size.height / 2.0;
            size.width = target.0;
            size.height = target.1;
            let new_half = size.height / 2.0;
            transform.translation.y += old_half - new_half;
        }
    }
}
