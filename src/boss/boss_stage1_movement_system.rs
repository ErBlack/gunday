use super::components::*;
use super::config::BOSS_SETTINGS;
use crate::components::MainCamera;
use crate::constants::{GROUND_RECT_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::*;

pub fn boss_stage1_movement_system(
    time: Res<Time>,
    mut q: Query<
        (
            &mut Transform,
            &mut BossMovementTimer,
            &mut BossStage1MovementState,
            &BossStage1ShootingState,
        ),
        With<Boss>,
    >,
    camera_q: Query<&Transform, (With<MainCamera>, Without<Boss>)>,
) {
    let camera_x = camera_q
        .iter()
        .next()
        .map(|t| t.translation.x)
        .unwrap_or(0.0);
    for (mut tf, mut mov, mut movement, shooting) in q.iter_mut() {
        mov.timer += time.delta_secs();

        let ground_y = -(SCREEN_HEIGHT * 0.5) + GROUND_RECT_HEIGHT;
        let default_base = ground_y + BOSS_SETTINGS.stage1.hover_ground_offset;
        if movement.hover_base_y.is_none() {
            movement.hover_base_y = Some(default_base);
        }
        let freq = BOSS_SETTINGS.stage1.wave_frequency;

        if movement.pending_move_request
            && !shooting.aiming
            && !shooting.shooting
            && !movement.moving
        {
            let anchors = current_screen_anchors(camera_x);
            let current = tf.translation.truncate();
            let nearest = nearest_anchor_index(current, &anchors);
            let others = [(nearest + 1) % 3, (nearest + 2) % 3];
            let chosen_idx = if fastrand::bool() {
                others[0]
            } else {
                others[1]
            };
            movement.move_from = current;
            movement.move_to = anchors[chosen_idx];
            movement.move_timer = 0.0;
            movement.move_duration = BOSS_SETTINGS.stage1.movement.move_duration;
            movement.moving = true;
        }

        if movement.moving {
            movement.move_timer += time.delta_secs();
            let t = (movement.move_timer / movement.move_duration).clamp(0.0, 1.0);
            let eased = t * t * (3.0 - 2.0 * t);
            let p = movement.move_from + (movement.move_to - movement.move_from) * eased;
            tf.translation.x = p.x;
            tf.translation.y = p.y;
            if t >= 1.0 {
                movement.moving = false;
                movement.pending_move_request = false;
                movement.hover_base_y = Some(movement.move_to.y);
            }
        }

        let target_amp = if shooting.aiming || shooting.shooting || movement.moving {
            0.0
        } else {
            BOSS_SETTINGS.stage1.idle_target_amplitude
        };

        let lerp_speed = BOSS_SETTINGS.stage1.amplitude_lerp_speed;
        mov.waving_amplitude +=
            (target_amp - mov.waving_amplitude) * lerp_speed * time.delta_secs();

        if (mov.waving_amplitude - target_amp).abs()
            < BOSS_SETTINGS.stage1.amplitude_snap_threshold
        {
            mov.waving_amplitude = target_amp;
        }

        if !movement.moving {
            let base = movement.hover_base_y.unwrap_or(default_base);
            tf.translation.y =
                base + mov.waving_amplitude * (2.0 * std::f32::consts::PI * freq * mov.timer).sin();
        }
    }
}

fn current_screen_anchors(camera_x: f32) -> [Vec2; 3] {
    let half_w = SCREEN_WIDTH * 0.5;
    let left = camera_x - half_w;
    let right = camera_x + half_w;
    let top = SCREEN_HEIGHT * 0.5;
    let ground_y = -SCREEN_HEIGHT * 0.5 + GROUND_RECT_HEIGHT;
    let top_y = top - BOSS_SETTINGS.stage1.movement.anchor_top_margin;
    let side_margin = BOSS_SETTINGS.stage1.movement.anchor_side_margin;
    let mid_y = ground_y + BOSS_SETTINGS.stage1.hover_ground_offset;

    [
        Vec2::new(right - side_margin, top_y),
        Vec2::new(left + side_margin, top_y),
        Vec2::new(camera_x, mid_y),
    ]
}

fn nearest_anchor_index(pos: Vec2, anchors: &[Vec2; 3]) -> usize {
    let mut idx = 0usize;
    let mut best = f32::INFINITY;
    for (i, a) in anchors.iter().enumerate() {
        let d2 = pos.distance_squared(*a);
        if d2 < best {
            best = d2;
            idx = i;
        }
    }
    idx
}
