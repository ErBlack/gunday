use super::components::{Miniboss, miniboss_bundle_at, miniboss_screen_right_x};
use super::config::MINIBOSS_CONFIG;
use crate::assets::GameAssets;
use crate::components::MainCamera;
use crate::constants::SCREEN_WIDTH;
use crate::game_state::{GamePhase, GamePhaseChanged};
use bevy::prelude::*;

pub fn spawn_miniboss_on_phase_start(
    mut commands: Commands,
    mut events: EventReader<GamePhaseChanged>,
    camera_q: Query<&Transform, With<MainCamera>>,
    miniboss_q: Query<Entity, With<Miniboss>>,
    game_assets: Res<GameAssets>,
) {
    let mut should_spawn = false;
    for event in events.read() {
        if event.next == GamePhase::MinibossFight {
            should_spawn = true;
        }
    }

    if !should_spawn {
        return;
    }

    if miniboss_q.iter().next().is_some() {
        return;
    }

    let Some(camera_tf) = camera_q.iter().next() else {
        return;
    };

    let camera_x = camera_tf.translation.x;
    let right_screen_edge = camera_x + SCREEN_WIDTH * 0.5;
    let desired_spawn_x = right_screen_edge + MINIBOSS_CONFIG.width * 0.5;
    let mut spawn_x = desired_spawn_x;

    let max_spawn_x = MINIBOSS_CONFIG.right_limit_x - MINIBOSS_CONFIG.width * 0.5;
    if spawn_x > max_spawn_x {
        spawn_x = max_spawn_x;
    }

    let desired_forward_anchor = miniboss_screen_right_x(camera_x);
    let forward_anchor_x = desired_forward_anchor.min(spawn_x);

    let bundle = miniboss_bundle_at(game_assets.as_ref(), spawn_x, spawn_x, forward_anchor_x);
    commands.spawn((bundle, Name::new("Miniboss")));
}
