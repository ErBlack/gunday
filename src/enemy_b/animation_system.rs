use super::components::*;
use crate::assets::GameAssets;
use bevy::prelude::*;

pub fn enemy_b_animation_system(
    mut q: Query<(&mut Sprite, &EnemyBState, &EnemyBThrowAnim)>,
    assets: Res<GameAssets>,
) {
    for (mut sprite, state, anim) in q.iter_mut() {
        sprite.flip_x = state.facing_right;
        match state.state {
            EnemyBStateKind::Sitting => {
                sprite.image = assets.enemy_b_sit.clone();
            }
            EnemyBStateKind::Throwing => {
                sprite.image = if anim.frame == 1 {
                    assets.enemy_b_fire_a.clone()
                } else {
                    assets.enemy_b_fire_b.clone()
                };
            }
            EnemyBStateKind::Hit => {
                sprite.image = assets.enemy_b_hit.clone();
            }
        }
    }
}
