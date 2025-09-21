use super::components::*;
use super::config::MINIBOSS_CONFIG;
use crate::components::MainCamera;
use crate::player::components::Player;
use bevy::prelude::*;

fn update_volley(
    entity: Entity,
    volley_opt: &mut Option<MinibossVolley>,
    dt: f32,
    writer: &mut EventWriter<MinibossFireEvent>,
) {
    let shots_per_volley = (MINIBOSS_CONFIG.grenade_points.len() / 2) as u8;
    if let Some(volley) = volley_opt.as_mut() {
        volley.next_shot_timer -= dt;
        if volley.next_shot_timer <= 0.0 && volley.shot_index < shots_per_volley {
            writer.write(MinibossFireEvent {
                entity,
                volley_kind: volley.kind,
                shot_index: volley.shot_index,
            });
            volley.shot_index += 1;
            if volley.shot_index < shots_per_volley {
                volley.next_shot_timer += MINIBOSS_CONFIG.grenade_spacing;
            } else {
                *volley_opt = None;
            }
        }
    }
}

pub fn miniboss_behavior_system(
    time: Res<Time>,
    mut fire_writer: EventWriter<MinibossFireEvent>,
    player_q: Query<&Transform, With<Player>>,
    camera_q: Query<&Transform, With<MainCamera>>,
    mut q: Query<
        (
            Entity,
            &Transform,
            &mut MinibossBehavior,
            &mut MinibossAnimation,
            &mut Sprite,
        ),
        With<Miniboss>,
    >,
) {
    let dt = time.delta_secs();
    let player_x = player_q.iter().next().map(|tf| tf.translation.x);
    let Some(camera_tf) = camera_q.iter().next() else {
        return;
    };
    let camera_x = camera_tf.translation.x;
    let screen_left = miniboss_screen_left_x(camera_x);
    let screen_right = miniboss_screen_right_x(camera_x);
    for (entity, transform, mut behavior, mut animation, mut sprite) in q.iter_mut() {
        match behavior.phase {
            MinibossPhase::Dying | MinibossPhase::Dead => {
                sprite.flip_x = behavior.facing_right;
                animation.variant = MinibossAnimVariant::Dead;
                animation.paused = true;
                continue;
            }
            _ => {}
        }

        sprite.flip_x = behavior.facing_right;
        behavior.forward_anchor_x = screen_right;

        handle_forced_retreat(
            &mut behavior,
            &mut animation,
            transform.translation.x,
            player_x,
        );

        match behavior.phase {
            MinibossPhase::MoveToForwardAnchor => {
                handle_move_to_forward_anchor(&mut behavior, &mut animation);
            }
            MinibossPhase::PreVolley => {
                handle_pre_volley(&mut behavior, &mut animation, dt);
            }
            MinibossPhase::VolleyFirst => {
                handle_volley_first(&mut behavior, &mut animation);
            }
            MinibossPhase::PostVolley => {
                handle_post_volley(
                    &mut behavior,
                    &mut animation,
                    transform.translation.x,
                    dt,
                    screen_left,
                );
            }
            MinibossPhase::RetreatRight => {
                handle_retreat_right(&mut behavior, &mut animation);
            }
            MinibossPhase::ReturnForward => {
                handle_return_forward(&mut behavior, &mut animation);
            }
            MinibossPhase::Dying | MinibossPhase::Dead => {}
        }

        if !matches!(behavior.phase, MinibossPhase::Dying | MinibossPhase::Dead) {
            update_volley(entity, &mut behavior.volley, dt, &mut fire_writer);
            update_volley(
                entity,
                &mut behavior.mid_retreat_volley,
                dt,
                &mut fire_writer,
            );
        }
    }
}

fn handle_move_to_forward_anchor(
    behavior: &mut MinibossBehavior,
    animation: &mut MinibossAnimation,
) {
    behavior.facing_right = behavior.movement_dir > 0.0;
    animation.variant = MinibossAnimVariant::Move;
    animation.paused = false;
}

fn handle_pre_volley(behavior: &mut MinibossBehavior, animation: &mut MinibossAnimation, dt: f32) {
    behavior.facing_right = false;
    animation.variant = MinibossAnimVariant::Shoot;
    animation.paused = true;
    animation.frame = 0;
    animation.timer = 0.0;
    behavior.timer -= dt;
    if behavior.timer <= 0.0 {
        behavior.phase = MinibossPhase::VolleyFirst;
        behavior.volley = Some(MinibossVolley::new(MinibossVolleyKind::First));
        animation.paused = false;
    }
}

fn handle_volley_first(behavior: &mut MinibossBehavior, animation: &mut MinibossAnimation) {
    behavior.facing_right = false;
    animation.variant = MinibossAnimVariant::Shoot;
    animation.paused = false;
    if behavior.volley.is_none() {
        behavior.phase = MinibossPhase::PostVolley;
        behavior.timer = MINIBOSS_CONFIG.post_volley_wait;
        animation.paused = true;
        animation.frame = 0;
        animation.timer = 0.0;
    }
}

fn handle_post_volley(
    behavior: &mut MinibossBehavior,
    animation: &mut MinibossAnimation,
    current_x: f32,
    dt: f32,
    screen_left: f32,
) {
    behavior.facing_right = false;
    animation.variant = MinibossAnimVariant::Shoot;
    animation.paused = true;
    behavior.timer -= dt;
    if behavior.timer <= 0.0 {
        transition_to_retreat(behavior, animation, current_x, screen_left);
    }
}

fn handle_retreat_right(behavior: &mut MinibossBehavior, animation: &mut MinibossAnimation) {
    behavior.facing_right = false;
    animation.variant = MinibossAnimVariant::Shoot;
    animation.paused = false;
}

fn handle_return_forward(behavior: &mut MinibossBehavior, animation: &mut MinibossAnimation) {
    behavior.facing_right = false;
    animation.variant = MinibossAnimVariant::Move;
    animation.paused = false;
}

fn transition_to_retreat(
    behavior: &mut MinibossBehavior,
    animation: &mut MinibossAnimation,
    current_x: f32,
    screen_left: f32,
) {
    behavior.phase = MinibossPhase::RetreatRight;
    behavior.target_x = screen_left;
    behavior.phase_start_x = current_x;
    behavior.movement_dir = if behavior.target_x >= current_x {
        1.0
    } else {
        -1.0
    };
    behavior.half_retreat_triggered = false;
    behavior.mid_retreat_volley = None;
    behavior.clamp_max_x = behavior.home_right_limit_x;
    animation.variant = MinibossAnimVariant::Shoot;
    animation.paused = false;
    animation.timer = 0.0;
    animation.frame = 0;
}

fn handle_forced_retreat(
    behavior: &mut MinibossBehavior,
    animation: &mut MinibossAnimation,
    current_x: f32,
    player_x: Option<f32>,
) {
    let Some(player_x) = player_x else {
        return;
    };

    if !behavior.forced_retreat {
        let gap = (current_x - player_x).abs();
        if gap < MINIBOSS_CONFIG.forced_retreat_trigger_distance {
            let release_distance = MINIBOSS_CONFIG.forced_retreat_release_distance;
            let stage_max = MINIBOSS_CONFIG.right_limit_x - MINIBOSS_CONFIG.width / 2.0;
            let max_target = (behavior.entry_max_x + release_distance).min(stage_max);
            let desired_target = player_x + release_distance;
            let mut target_x = desired_target.max(current_x + 1.0);
            if target_x > max_target {
                target_x = max_target;
            }
            if target_x > current_x + 0.5 {
                behavior.phase = MinibossPhase::RetreatRight;
                behavior.target_x = target_x;
                behavior.phase_start_x = current_x;
                behavior.movement_dir = 1.0;
                behavior.half_retreat_triggered = false;
                behavior.mid_retreat_volley = None;
                behavior.volley = None;
                behavior.clamp_max_x = target_x.max(behavior.entry_max_x).min(stage_max);
                behavior.forced_retreat = true;
                behavior.timer = 0.0;
                animation.variant = MinibossAnimVariant::Shoot;
                animation.paused = false;
                animation.timer = 0.0;
                animation.frame = 0;
                behavior.facing_right = false;
            }
        }
    } else {
        let gap = (current_x - player_x).abs();
        if gap >= MINIBOSS_CONFIG.forced_retreat_release_distance {
            behavior.forced_retreat = false;
        }
    }
}
