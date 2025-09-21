use crate::components::{CameraState, MainCamera};
use crate::constants::SCREEN_WIDTH;
use crate::game_state::{GamePhase, GamePhaseTransitionTimer};
use crate::player::components::Player;
use crate::soundtrack::{SoundtrackController, TrackSetName};
use crate::spawn::STATIC_SPAWN_COORDINATES;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub(crate) struct PositionTriggerState {
    pub(crate) first_steps: bool,
    pub(crate) reached_hangar: bool,
    pub(crate) reached_miniboss: bool,
    pub(crate) finished_hangar: bool,
    pub(crate) entering_lab: bool,
    pub(crate) deep_in_lab: bool,
    pub(crate) exit_lab: bool,
    pub(crate) reach_boss: bool,
    pub(crate) boss_fight: bool,
    pub(crate) edge_reached: bool,
}

struct PositionTriggerCoordinates {
    first_steps: f32,
    hangar: f32,
    miniboss: f32,
    hangar_exit: f32,
    entering_lab: f32,
    deep_lab: f32,
    exit_lab: f32,
    reach_boss: f32,
    boss_fight: f32,
    edge_reached: f32,
}

const BOSS_FIGHT: f32 = 10304.0;

const POSITION_TRIGGER_COORDINATES: PositionTriggerCoordinates = PositionTriggerCoordinates {
    first_steps: 500.0,
    hangar: 1100.0,
    miniboss: 4500.0,
    hangar_exit: 6000.0,
    entering_lab: STATIC_SPAWN_COORDINATES.infected_enemy - (SCREEN_WIDTH / 4.0) * 3.0,
    deep_lab: 6700.0,
    exit_lab: 8800.0,
    reach_boss: 9000.0,
    boss_fight: BOSS_FIGHT - SCREEN_WIDTH / 2.0,
    edge_reached: BOSS_FIGHT,
};

pub fn track_player_position_system(
    mut commands: Commands,
    mut trigger_state: ResMut<PositionTriggerState>,
    player_query: Query<&Transform, With<Player>>,
    mut controller: ResMut<SoundtrackController>,
    mut camera_state_q: Query<&mut CameraState, With<MainCamera>>,
) {
    let mut iter = player_query.iter();
    let Some(transform) = iter.next() else {
        return;
    };

    let x = transform.translation.x;

    if !trigger_state.first_steps && x >= POSITION_TRIGGER_COORDINATES.first_steps {
        trigger_state.first_steps = true;
        controller.request_track_set(TrackSetName::Ambient);
    }

    if !trigger_state.reached_hangar && x >= POSITION_TRIGGER_COORDINATES.hangar {
        trigger_state.reached_hangar = true;
        let result = controller.request_track_set(TrackSetName::LightAction);

        let delay = result.eta_seconds.unwrap_or(0.0);
        commands.spawn(GamePhaseTransitionTimer::new(GamePhase::HangarFight, delay));
    }

    if !trigger_state.reached_miniboss && x >= POSITION_TRIGGER_COORDINATES.miniboss {
        trigger_state.reached_miniboss = true;
        let result = controller.request_track_set(TrackSetName::HeavyAction);

        let delay = result.eta_seconds.unwrap_or(0.0);
        commands.spawn(GamePhaseTransitionTimer::new(
            GamePhase::MinibossFight,
            delay,
        ));
    }

    if !trigger_state.finished_hangar && x >= POSITION_TRIGGER_COORDINATES.hangar_exit {
        trigger_state.finished_hangar = true;
        controller.request_track_set(TrackSetName::Ambient);
    }

    if !trigger_state.entering_lab && x >= POSITION_TRIGGER_COORDINATES.entering_lab {
        trigger_state.entering_lab = true;
        controller.request_track_set(TrackSetName::LightLab);
    }

    if !trigger_state.deep_in_lab && x >= POSITION_TRIGGER_COORDINATES.deep_lab {
        trigger_state.deep_in_lab = true;
        let result = controller.request_track_set(TrackSetName::HeavyLab);
        let delay = result.eta_seconds.unwrap_or(0.0);
        commands.spawn(GamePhaseTransitionTimer::new(GamePhase::LabFight, delay));
    }

    if !trigger_state.exit_lab && x >= POSITION_TRIGGER_COORDINATES.exit_lab {
        trigger_state.exit_lab = true;
        let result = controller.request_track_set(TrackSetName::Basic);
        let delay = result.eta_seconds.unwrap_or(0.0);
        commands.spawn(GamePhaseTransitionTimer::new(
            GamePhase::BossEntering,
            delay,
        ));
    }

    if !trigger_state.reach_boss && x >= POSITION_TRIGGER_COORDINATES.reach_boss {
        trigger_state.reach_boss = true;
        controller.request_track_set(TrackSetName::Entering);
    }

    if !trigger_state.boss_fight && x >= POSITION_TRIGGER_COORDINATES.boss_fight {
        trigger_state.boss_fight = true;
        let result = controller.request_track_set(TrackSetName::BossStage1);

        let delay = result.eta_seconds.unwrap_or(0.0);
        commands.spawn(GamePhaseTransitionTimer::new(GamePhase::BossFight, delay));
    }

    if !trigger_state.edge_reached && x >= POSITION_TRIGGER_COORDINATES.edge_reached {
        trigger_state.edge_reached = true;

        if let Some(mut camera_state) = camera_state_q.iter_mut().next() {
            camera_state.lock_position = Some(camera_state.current_x);
            camera_state.max_reached_x = camera_state.current_x;
        }
    }
}
