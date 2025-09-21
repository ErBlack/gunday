use super::components::ExplosionAnim;
use super::config::ENEMY_B_CONFIG;
use crate::assets::GameAssets;
use bevy::prelude::*;

pub fn enemy_b_explosion_anim_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut q: Query<(Entity, &mut Sprite, &mut ExplosionAnim)>,
) {
    let frame_time = ENEMY_B_CONFIG.explosion_frame_time;
    for (e, mut sprite, mut anim) in q.iter_mut() {
        anim.timer += time.delta_secs();
        if anim.timer >= frame_time {
            anim.timer = 0.0;
            anim.frame += 1;
            match anim.frame {
                1 => sprite.image = assets.explosion_a_b.clone(),
                2 => sprite.image = assets.explosion_a_c.clone(),
                3 => sprite.image = assets.explosion_a_d.clone(),
                4 => sprite.image = assets.explosion_a_e.clone(),
                5 => sprite.image = assets.explosion_a_f.clone(),
                6 => sprite.image = assets.explosion_a_g.clone(),
                _ => {
                    commands.entity(e).despawn();
                }
            }
        }
    }
}
