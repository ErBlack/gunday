use super::components::*;
use super::config::ENEMY_B_CONFIG;
use bevy::prelude::*;

pub fn enemy_b_behavior_system(
    time: Res<Time>,
    cam_q: Query<&GlobalTransform, With<Camera>>,
    player_q: Query<&Transform, With<crate::player::components::Player>>,
    mut q: Query<(
        &Transform,
        &mut EnemyBState,
        &mut EnemyBThrowTimer,
        &mut EnemyBThrowAnim,
    )>,
) {
    let Ok(cam) = cam_q.single() else {
        return;
    };
    let cam_x = cam.translation().x;
    let half_w = crate::constants::SCREEN_WIDTH * 0.5;
    let on_screen = |x: f32| x > cam_x - half_w && x < cam_x + half_w;
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    for (tf, mut st, mut timer, mut anim) in q.iter_mut() {
        st.facing_right = player_tf.translation.x >= tf.translation.x;
        match st.state {
            EnemyBStateKind::Sitting => {
                timer.timer += time.delta_secs();
                if on_screen(tf.translation.x) && timer.timer >= ENEMY_B_CONFIG.throw_interval {
                    timer.timer = 0.0;
                    st.state = EnemyBStateKind::Throwing;
                    anim.frame = 1;
                    anim.timer = ENEMY_B_CONFIG.throw_frame_time;
                    anim.thrown = false;
                }
            }
            EnemyBStateKind::Throwing => {
                anim.timer -= time.delta_secs();
                if anim.timer <= 0.0 {
                    anim.timer = ENEMY_B_CONFIG.throw_frame_time;
                    if anim.frame == 1 {
                        anim.frame = 2;
                    } else {
                        st.state = EnemyBStateKind::Sitting;
                        anim.frame = 0;
                    }
                }
            }
            EnemyBStateKind::Hit => {}
        }
    }
}
