use super::components::*;
use bevy::prelude::*;

pub fn boss_sprite_flip_system(mut bosses: Query<(&BossFacing, &mut Transform), With<Boss>>) {
    for (facing, mut root_tr) in bosses.iter_mut() {
        let desired = if facing.right { -1.0 } else { 1.0 };
        if root_tr.scale.x != desired {
            root_tr.scale.x = desired;
        }
    }
}
