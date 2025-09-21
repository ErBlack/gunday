use crate::assets::GameAssets;
use crate::components::MainCamera;
use crate::constants::{GROUND_RECT_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, Z_ENEMY_BASE};
use crate::game_state::{GamePhase, GameState};
use crate::spawn::{ScreenEdge, SpawnedFromEdge};
use bevy::prelude::*;

use super::components::{ENEMY_C_WIDTH, EnemyC, EnemyCBundle};
use super::config::{ENEMY_C_CONFIG, ENEMY_C_CONSTANTS};

fn enemy_c_ground_y() -> f32 {
    GROUND_RECT_HEIGHT - SCREEN_HEIGHT * 0.5 + ENEMY_C_CONFIG.spawn_ground_offset
}

pub fn enemy_c_dynamic_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<f32>,
    game_state: Res<GameState>,
    camera_q: Query<&Transform, With<MainCamera>>,
    enemy_c_q: Query<&Transform, With<EnemyC>>,
    game_assets: Res<GameAssets>,
) {
    if game_state.phase() != GamePhase::LabFight {
        *timer = ENEMY_C_CONSTANTS.dynamic_spawn_interval;
        return;
    }

    let Some(camera_tf) = camera_q.iter().next() else {
        return;
    };

    *timer -= time.delta_secs();
    if *timer > 0.0 {
        return;
    }

    let half_width = SCREEN_WIDTH * 0.5;
    let cam_x = camera_tf.translation.x;
    let left_edge = cam_x - half_width;
    let right_edge = cam_x + half_width;

    let active_on_screen = enemy_c_q
        .iter()
        .filter(|tf| {
            let x = tf.translation.x;
            x > left_edge - ENEMY_C_WIDTH && x < right_edge + ENEMY_C_WIDTH
        })
        .count();

    if active_on_screen >= ENEMY_C_CONSTANTS.dynamic_spawn_limit {
        *timer = ENEMY_C_CONSTANTS.dynamic_spawn_interval;
        return;
    }

    let spawn_edge = if fastrand::bool() {
        ScreenEdge::Left
    } else {
        ScreenEdge::Right
    };

    let spawn_x = match spawn_edge {
        ScreenEdge::Left => left_edge + ENEMY_C_WIDTH * 0.5,
        ScreenEdge::Right => right_edge - ENEMY_C_WIDTH * 0.5,
    };

    let facing_right = matches!(spawn_edge, ScreenEdge::Left);
    let spawn_position = Vec3::new(spawn_x, enemy_c_ground_y(), Z_ENEMY_BASE);

    commands.spawn((
        EnemyCBundle::new(
            game_assets.enemy_c_run_a.clone(),
            spawn_position,
            facing_right,
        ),
        Name::new("EnemyC"),
        SpawnedFromEdge { edge: spawn_edge },
    ));

    *timer = ENEMY_C_CONSTANTS.dynamic_spawn_interval;
}
