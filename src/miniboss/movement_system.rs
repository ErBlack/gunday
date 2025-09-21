use super::components::*;
use super::config::MINIBOSS_CONFIG;
use crate::components::MainCamera;
use bevy::prelude::*;

pub fn miniboss_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Miniboss>)>,
    mut q: Query<
        (
            Entity,
            &mut MinibossBehavior,
            &mut MinibossAnimation,
            &mut Transform,
        ),
        With<Miniboss>,
    >,
) {
    let dt = time.delta_secs();
    let Some(camera_tf) = camera_q.iter().next() else {
        return;
    };
    let camera_x = camera_tf.translation.x;
    let screen_left_bound = miniboss_screen_left_x(camera_x);
    let screen_right_bound = miniboss_screen_right_x(camera_x);
    let retreat_screen_limit = screen_right_bound - 20.0;
    for (entity, mut behavior, mut animation, mut transform) in q.iter_mut() {
        let offscreen_left = miniboss_offscreen_left_bound(camera_x);
        if transform.translation.x < offscreen_left {
            commands.entity(entity).despawn();
            continue;
        }

        match behavior.phase {
            MinibossPhase::MoveToForwardAnchor
            | MinibossPhase::RetreatRight
            | MinibossPhase::ReturnForward => {
                let dir = behavior.movement_dir;
                if dir.abs() < f32::EPSILON {
                    continue;
                }

                let stage_max = behavior.home_right_limit_x;

                let mut min_x = screen_left_bound
                    .min(behavior.target_x)
                    .min(transform.translation.x);
                let mut max_x = screen_right_bound
                    .max(behavior.clamp_max_x)
                    .max(behavior.target_x)
                    .max(transform.translation.x);

                max_x = max_x.min(stage_max);

                if matches!(behavior.phase, MinibossPhase::RetreatRight) {
                    max_x = max_x.min(retreat_screen_limit.max(behavior.target_x));
                }

                if max_x < min_x {
                    std::mem::swap(&mut min_x, &mut max_x);
                }

                let target = behavior.target_x.clamp(min_x, max_x);
                if matches!(behavior.phase, MinibossPhase::RetreatRight) {
                    behavior.target_x = target;
                }

                let mut new_x = transform.translation.x + dir * behavior.move_speed * dt;
                new_x = new_x.clamp(min_x, max_x);
                transform.translation.x = new_x;

                let arrived = if dir > 0.0 {
                    new_x + 0.5 >= target
                } else {
                    new_x - 0.5 <= target
                };

                if arrived {
                    transform.translation.x = target;
                    finalize_movement(
                        &mut behavior,
                        &mut animation,
                        transform.translation.x,
                        screen_right_bound,
                    );
                }

                if matches!(behavior.phase, MinibossPhase::RetreatRight)
                    && !behavior.half_retreat_triggered
                {
                    let start = behavior.phase_start_x;
                    let end = target;
                    let distance = (end - start).abs();
                    if distance > f32::EPSILON {
                        let travelled = (transform.translation.x - start).abs();
                        let progress = travelled / distance;
                        if progress >= 0.5 {
                            behavior.half_retreat_triggered = true;
                            behavior.mid_retreat_volley =
                                Some(MinibossVolley::new(MinibossVolleyKind::Second));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn finalize_movement(
    behavior: &mut MinibossBehavior,
    animation: &mut MinibossAnimation,
    current_x: f32,
    screen_right: f32,
) {
    behavior.phase_start_x = current_x;
    behavior.movement_dir = 0.0;
    behavior.volley = None;
    behavior.mid_retreat_volley = None;
    behavior.half_retreat_triggered = false;
    behavior.forced_retreat = false;

    if matches!(behavior.phase, MinibossPhase::RetreatRight) {
        behavior.phase = MinibossPhase::ReturnForward;
        behavior.target_x = screen_right;
        behavior.movement_dir = if behavior.target_x < current_x {
            -1.0
        } else {
            1.0
        };
        behavior.phase_start_x = current_x;
        behavior.clamp_max_x = behavior.home_right_limit_x;
        behavior.forward_anchor_x = screen_right;
        animation.variant = MinibossAnimVariant::Move;
        animation.timer = 0.0;
        animation.frame = 0;
        animation.paused = false;
        behavior.facing_right = false;
    } else {
        if matches!(behavior.phase, MinibossPhase::MoveToForwardAnchor) {
            behavior.clamp_max_x = behavior.home_right_limit_x;
            behavior.forward_anchor_x = screen_right;
        } else {
            behavior.forward_anchor_x = screen_right;
        }
        behavior.phase = MinibossPhase::PreVolley;
        behavior.timer = MINIBOSS_CONFIG.pre_volley_wait;
        behavior.movement_dir = 0.0;
        behavior.facing_right = false;
        animation.variant = MinibossAnimVariant::Shoot;
        animation.timer = 0.0;
        animation.frame = 0;
        animation.paused = true;
    }
}
