use super::components::{EnemyC, EnemyCDeathBlink, EnemyCDespawnTimer};
use bevy::prelude::*;

pub fn enemy_c_death_system(
    time: Res<Time>,
    mut commands: Commands,
    mut blink_q: Query<(&mut Sprite, &mut Transform, &mut EnemyCDeathBlink), With<EnemyC>>,
    mut despawn_q: Query<(Entity, &mut EnemyCDespawnTimer), With<EnemyC>>,
) {
    let delta = time.delta_secs();
    if delta <= 0.0 {
        return;
    }

    for (mut sprite, mut tf, mut blink) in blink_q.iter_mut() {
        if blink.moved < blink.total_move {
            let remaining = blink.total_move - blink.moved;
            let total_duration = blink.interval * blink.toggles_total as f32;
            let speed = if total_duration > 0.0 {
                blink.total_move / total_duration
            } else {
                0.0
            };
            let dx = (speed * delta).min(remaining) * blink.dir;
            tf.translation.x += dx;
            blink.moved += dx.abs();
        }

        if blink.toggles_left > 0 {
            blink.timer += delta;
            if blink.timer >= blink.interval {
                blink.timer = 0.0;
                let cur = sprite.color.to_srgba();
                let next_alpha = if cur.alpha > 0.5 { 0.15 } else { 1.0 };
                sprite.color = Color::srgba(cur.red, cur.green, cur.blue, next_alpha);
                blink.toggles_left -= 1;
            }
        } else {
            let cur = sprite.color.to_srgba();
            if (cur.alpha - 1.0).abs() > f32::EPSILON {
                sprite.color = Color::srgba(cur.red, cur.green, cur.blue, 1.0);
            }
        }
    }

    for (entity, mut timer) in despawn_q.iter_mut() {
        timer.timer -= delta;
        if timer.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
