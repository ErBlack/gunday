use super::components::*;
use super::config::MINIBOSS_CONFIG;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::constants::PROJECTILE_SIZE;
use crate::game_state::{GamePhase, GamePhaseTransitionTimer};
use crate::projectile::components::{
    PlayerProjectile, Projectile, ProjectileHitEvent, swept_projectile_hit_center,
};
use crate::soundtrack::{SoundtrackController, TrackSetName};
use bevy::prelude::*;

pub fn miniboss_hit_system(
    mut commands: Commands,
    mut q: Query<
        (
            Entity,
            &Transform,
            &mut MinibossBehavior,
            &mut MinibossHealth,
            &mut MinibossAnimation,
            &mut Sprite,
            Option<&MinibossDeath>,
        ),
        With<Miniboss>,
    >,
    projectiles: Query<
        (Entity, &Transform, &Projectile),
        (With<Projectile>, With<PlayerProjectile>),
    >,
    mut hit_writer: EventWriter<ProjectileHitEvent>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut controller: ResMut<SoundtrackController>,
) {
    let projectile_data: Vec<(Entity, Vec2, Vec2)> = projectiles
        .iter()
        .map(|(entity, transform, projectile)| {
            (
                entity,
                projectile.previous_translation,
                Vec2::new(transform.translation.x, transform.translation.y),
            )
        })
        .collect();

    for (entity, transform, mut behavior, mut health, mut animation, mut sprite, death_opt) in
        q.iter_mut()
    {
        if matches!(behavior.phase, MinibossPhase::Dying | MinibossPhase::Dead) {
            continue;
        }

        let enemy_min = Vec2::new(
            transform.translation.x - MINIBOSS_CONFIG.width / 2.0,
            transform.translation.y,
        );
        let enemy_size = Vec2::new(MINIBOSS_CONFIG.width, MINIBOSS_CONFIG.height);
        let mut hit = false;

        for (proj_entity, start_center, end_center) in projectile_data.iter() {
            if let Some(hit_center) = swept_projectile_hit_center(
                *start_center,
                *end_center,
                Vec2::splat(PROJECTILE_SIZE),
                enemy_min,
                enemy_size,
            ) {
                hit = true;
                health.hp -= 1;
                play_sfx_once(
                    &mut commands,
                    emitters.enemy_hit,
                    assets.enemy_hit_sfx.clone(),
                );
                let hit_pos = Vec3::new(hit_center.x, hit_center.y, transform.translation.z);
                hit_writer.write(ProjectileHitEvent { position: hit_pos });
                commands.entity(*proj_entity).despawn();

                if health.hp <= 0 {
                    let death_started = begin_miniboss_death(
                        &mut commands,
                        entity,
                        &mut behavior,
                        &mut animation,
                        &mut sprite,
                        death_opt.is_some(),
                        &assets,
                    );

                    if death_started {
                        let track_name = TrackSetName::Ambient;
                        controller.request_track_set(track_name);

                        commands.spawn(GamePhaseTransitionTimer::new(GamePhase::LabEntering, 0.0));
                    }
                }

                break;
            }
        }

        if hit {
            health.hp = health.hp.max(0);
        }
    }
}

fn begin_miniboss_death(
    commands: &mut Commands,
    entity: Entity,
    behavior: &mut MinibossBehavior,
    animation: &mut MinibossAnimation,
    sprite: &mut Sprite,
    already_dead: bool,
    assets: &Res<GameAssets>,
) -> bool {
    if already_dead {
        return false;
    }
    let facing_right = behavior.facing_right;
    behavior.phase = MinibossPhase::Dying;
    behavior.movement_dir = 0.0;
    behavior.volley = None;
    behavior.mid_retreat_volley = None;
    behavior.half_retreat_triggered = false;
    behavior.timer = 0.0;
    animation.variant = MinibossAnimVariant::Dead;
    animation.paused = true;
    animation.timer = 0.0;
    animation.frame = 0;
    sprite.image = assets.miniboss_dead.clone();
    sprite.flip_x = !facing_right;

    let mut death = MinibossDeath::new();
    death.explosion_timer = 0.0;
    death.explosion_index = 0;
    commands.entity(entity).insert(death);
    true
}
