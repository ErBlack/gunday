use super::components::{Player, PlayerDirection, PlayerSpriteEntity};
use bevy::prelude::*;

pub fn player_sprite_flip_system(
    mut parent_query: Query<(&PlayerDirection, &PlayerSpriteEntity), With<Player>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    for (direction, sprite_entity) in parent_query.iter_mut() {
        if let Ok(mut sprite) = sprite_query.get_mut(**sprite_entity) {
            sprite.flip_x = !direction.facing_right;
        }
    }
}
