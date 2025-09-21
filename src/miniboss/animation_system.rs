use super::components::*;
use super::config::MINIBOSS_CONFIG;
use crate::assets::GameAssets;
use bevy::prelude::*;

pub fn miniboss_animation_system(
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut q: Query<(&MinibossBehavior, &mut MinibossAnimation, &mut Sprite), With<Miniboss>>,
) {
    let dt = time.delta_secs();
    for (_behavior, mut anim, mut sprite) in q.iter_mut() {
        match anim.variant {
            MinibossAnimVariant::Move => {
                if anim.paused {
                    sprite.image = assets.miniboss_move_a.clone();
                    anim.frame = 0;
                    anim.timer = 0.0;
                    continue;
                }
                anim.timer += dt;
                if anim.timer >= MINIBOSS_CONFIG.move_frame_time {
                    anim.timer -= MINIBOSS_CONFIG.move_frame_time;
                    anim.frame = (anim.frame + 1) % 2;
                }
                sprite.image = if anim.frame == 0 {
                    assets.miniboss_move_a.clone()
                } else {
                    assets.miniboss_move_b.clone()
                };
            }
            MinibossAnimVariant::Shoot => {
                if anim.paused {
                    sprite.image = assets.miniboss_shoot_a.clone();
                    anim.frame = 0;
                    anim.timer = 0.0;
                    continue;
                }
                anim.timer += dt;
                if anim.timer >= MINIBOSS_CONFIG.shoot_frame_time {
                    anim.timer -= MINIBOSS_CONFIG.shoot_frame_time;
                    anim.frame = (anim.frame + 1) % 2;
                }
                sprite.image = if anim.frame == 0 {
                    assets.miniboss_shoot_a.clone()
                } else {
                    assets.miniboss_shoot_b.clone()
                };
            }
            MinibossAnimVariant::Dead => {
                sprite.image = assets.miniboss_dead.clone();
                anim.timer = 0.0;
                anim.frame = 0;
            }
        }
    }
}
