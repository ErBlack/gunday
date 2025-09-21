use super::components::{Boss, BossMovementTimer, BossStage1MovementState};
use super::config::BOSS_SETTINGS;
use super::setup_boss::spawn_boss;
use crate::components::MainCamera;
use crate::constants::{GROUND_RECT_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, Z_ENEMY_BASE};
use crate::game_state::{GamePhase, GamePhaseChanged};
use bevy::prelude::*;

pub fn boss_spawn_system(
    mut commands: Commands,
    mut phase_events: EventReader<GamePhaseChanged>,
    camera_q: Query<&Transform, With<MainCamera>>,
    asset_server: Res<AssetServer>,
    boss_q: Query<Entity, With<Boss>>,
) {
    let should_spawn = phase_events
        .read()
        .any(|event| event.next == GamePhase::BossFight);

    if !should_spawn {
        return;
    }

    if boss_q.iter().next().is_some() {
        return;
    }

    let Some(camera_tf) = camera_q.iter().next() else {
        return;
    };

    let camera_x = camera_tf.translation.x;
    let right_edge = camera_x + SCREEN_WIDTH * 0.5;

    let spawn_x = right_edge
        + BOSS_SETTINGS.width * 0.5 + 30.0;
    let spawn_y = GROUND_RECT_HEIGHT + BOSS_SETTINGS.stage1.hover_ground_offset - SCREEN_HEIGHT / 2.0;

    let spawn_translation = Vec3::new(spawn_x, spawn_y, Z_ENEMY_BASE);

    let approach_target = Vec2::new(right_edge - BOSS_SETTINGS.width / 2.0 - 16.0, spawn_y);

    let boss_entity = spawn_boss(&mut commands, asset_server.as_ref(), spawn_translation);

    commands.entity(boss_entity).insert(BossStage1MovementState {
        hover_base_y: Some(approach_target.y),
        pending_move_request: false,
        moving: true,
        move_timer: 0.0,
        move_duration: BOSS_SETTINGS.spawn_move_duration,
        move_from: spawn_translation.truncate(),
        move_to: approach_target,
    });

    commands.entity(boss_entity).insert(BossMovementTimer {
        timer: 0.0,
        waving_amplitude: 0.0,
    });
}
