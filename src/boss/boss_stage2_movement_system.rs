use super::components::*;
use crate::player::components::Player;
use bevy::prelude::*;

pub fn boss_stage2_movement_system(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Player>, Without<Boss>)>,
    mut bosses: Query<
        (
            &mut BossStage2State,
            &mut Transform,
            Option<&mut BossFacing>,
        ),
        (With<Boss>, Without<Player>),
    >,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    for (mut s2, mut root_transform, facing_opt) in bosses.iter_mut() {
        let offset = (player_tf.translation - root_transform.translation).truncate();
        let horizontal_distance = offset.x.abs();
        let dir = offset.normalize_or_zero();

        if horizontal_distance > 50.0 {
            root_transform.translation +=
                Vec3::new(dir.x, 0.0, 0.0) * s2.crawl_speed * time.delta_secs();
        }

        s2.crawl_timer += time.delta_secs();
        if let Some(mut facing) = facing_opt {
            if dir.x.abs() > 0.001 {
                facing.right = dir.x >= 0.0;
            }
        }
    }
}
