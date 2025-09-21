use super::robot_components::{EnemyDeathBlink, EnemyRobot};
use bevy::prelude::*;

pub fn enemy_robot_death_anim_system(
    time: Res<Time>,
    mut q: Query<(&mut Sprite, &mut Transform, &mut EnemyDeathBlink), With<EnemyRobot>>,
) {
    for (mut sprite, mut tf, mut blink) in q.iter_mut() {
        if blink.moved < blink.total_move {
            let remaining = blink.total_move - blink.moved;
            let total_duration = blink.interval * blink.toggles_total as f32;
            let speed = if total_duration > 0.0 {
                blink.total_move / total_duration
            } else {
                0.0
            };
            let dx = (speed * time.delta_secs()).min(remaining) * blink.dir;
            tf.translation.x += dx;
            blink.moved += dx.abs();
        }

        if blink.toggles_left > 0 {
            blink.timer += time.delta_secs();
            if blink.timer >= blink.interval {
                blink.timer = 0.0;
                let cur = sprite.color.to_srgba();
                let next_a = if cur.alpha > 0.5 { 0.2 } else { 1.0 };
                sprite.color = Color::srgba(cur.red, cur.green, cur.blue, next_a);
                blink.toggles_left -= 1;
            }
        } else {
            let cur = sprite.color.to_srgba();
            sprite.color = Color::srgba(cur.red, cur.green, cur.blue, 1.0);
        }
    }
}
