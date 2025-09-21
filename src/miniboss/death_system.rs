use super::components::*;
use super::config::MINIBOSS_CONFIG;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::effects::explosion_anim::spawn_explosion_d;
use bevy::prelude::*;

pub fn miniboss_death_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut q: Query<
        (
            Entity,
            &Transform,
            &mut MinibossBehavior,
            &mut MinibossDeath,
            &mut Visibility,
        ),
        With<Miniboss>,
    >,
) {
    let dt = time.delta_secs();
    for (entity, transform, mut behavior, mut death, mut visibility) in q.iter_mut() {
        if death.explosion_index < MINIBOSS_DEATH_EXPLOSION_OFFSETS.len() {
            death.explosion_timer -= dt;
            if death.explosion_timer <= 0.0 {
                let offset = MINIBOSS_DEATH_EXPLOSION_OFFSETS[death.explosion_index];
                let pos = Vec3::new(
                    transform.translation.x + offset.x,
                    transform.translation.y + offset.y,
                    transform.translation.z + 0.2,
                );
                spawn_explosion_d(&mut commands, &assets, pos);
                play_sfx_once(
                    &mut commands,
                    emitters.enemy_explosion,
                    assets.miniboss_explosion_sfx.clone(),
                );
                death.explosion_index += 1;
                if death.explosion_index < MINIBOSS_DEATH_EXPLOSION_OFFSETS.len() {
                    death.explosion_timer = MINIBOSS_CONFIG.death_explosion_delay;
                }
            }
        }

        match death.phase {
            MinibossDeathPhase::Wait => {
                death.timer -= dt;
                if death.timer <= 0.0 {
                    death.phase = MinibossDeathPhase::Blink;
                    death.timer = death.blink_duration;
                    death.blink_timer = death.blink_interval;
                    death.visible = false;
                    *visibility = Visibility::Hidden;
                }
            }
            MinibossDeathPhase::Blink => {
                death.timer -= dt;
                death.blink_timer -= dt;
                if death.blink_timer <= 0.0 {
                    death.blink_timer += death.blink_interval;
                    death.visible = !death.visible;
                    *visibility = if death.visible {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
                if death.timer <= 0.0 {
                    behavior.phase = MinibossPhase::Dead;
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
