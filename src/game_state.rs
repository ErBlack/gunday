use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamePhase {
    HangarEntering,
    HangarFight,
    MinibossFight,
    LabEntering,
    LabFight,
    BossEntering,
    BossFight,
}

impl GamePhase {
    fn default() -> Self {
        GamePhase::HangarEntering
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct GameState {
    phase: GamePhase,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            phase: GamePhase::default(),
        }
    }
}

impl GameState {
    pub fn phase(&self) -> GamePhase {
        self.phase
    }

    pub fn transition_to(&mut self, next: GamePhase) -> Option<GamePhase> {
        if self.phase == next {
            None
        } else {
            let previous = self.phase;
            self.phase = next;
            Some(previous)
        }
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct GamePhaseRequest {
    pub next: GamePhase,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct GamePhaseChanged {
    pub next: GamePhase,
}

#[derive(Component)]
pub struct GamePhaseTransitionTimer {
    timer: Timer,
    phase: GamePhase,
}

impl GamePhaseTransitionTimer {
    pub fn new(phase: GamePhase, delay_seconds: f32) -> Self {
        let mut timer = Timer::from_seconds(delay_seconds.max(0.0), TimerMode::Once);
        if delay_seconds <= 0.0 {
            timer.set_elapsed(timer.duration());
        }
        Self { timer, phase }
    }

    pub fn phase(&self) -> GamePhase {
        self.phase
    }
}

fn handle_game_phase_requests(
    mut state: ResMut<GameState>,
    mut requests: EventReader<GamePhaseRequest>,
    mut changed: EventWriter<GamePhaseChanged>,
) {
    for request in requests.read() {
        if state.transition_to(request.next).is_some() {
            changed.write(GamePhaseChanged {
                next: request.next,
            });
        }
    }
}

fn process_game_phase_transition_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: Query<(Entity, &mut GamePhaseTransitionTimer)>,
    mut requests: EventWriter<GamePhaseRequest>,
) {
    for (entity, mut transition) in timers.iter_mut() {
        transition.timer.tick(time.delta());
        if transition.timer.finished() {
            requests.write(GamePhaseRequest {
                next: transition.phase(),
            });
            commands.entity(entity).despawn();
        }
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_event::<GamePhaseRequest>()
            .add_event::<GamePhaseChanged>()
            .add_systems(
                Update,
                (
                    handle_game_phase_requests,
                    process_game_phase_transition_timers,
                ),
            );
    }
}
