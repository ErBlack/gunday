use std::collections::BTreeSet;

use crate::boss::events::BossDefeatedEvent;
use bevy::{
    audio::{AudioPlayer, AudioSink, AudioSource, PlaybackSettings, Volume},
    prelude::*,
};

const TRACK_COUNT: usize = 9;
pub const TRACK_LOOP_DURATION_SECONDS: f32 = 7.059;

#[derive(Clone, Copy, Debug, PartialEq)]
struct TrackTiming {
    fade_in_offset_seconds: f32,
    fade_out_seconds: f32,
}

impl TrackTiming {
    const fn new(fade_in_offset_seconds: f32, fade_out_seconds: f32) -> Self {
        Self {
            fade_in_offset_seconds,
            fade_out_seconds,
        }
    }
}

const TRACK_TIMINGS: [TrackTiming; TRACK_COUNT] = [
    TrackTiming::new(0.0, 0.0),
    TrackTiming::new(0.0, 0.0),
    TrackTiming::new(0.5, 1.0),
    TrackTiming::new(0.0, 0.0),
    TrackTiming::new(0.0, 0.0),
    TrackTiming::new(0.0, 0.0),
    TrackTiming::new(0.0, 1.0),
    TrackTiming::new(0.2, 0.0),
    TrackTiming::new(0.66, 0.0),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrackSetName {
    Basic,
    Ambient,
    LightAction,
    HeavyAction,
    LightLab,
    HeavyLab,
    Entering,
    BossStage1,
    BossStage2,
}

impl TrackSetName {
    pub fn tracks(self) -> &'static [usize] {
        match self {
            TrackSetName::Basic => &[1],
            TrackSetName::Ambient => &[1, 2],
            TrackSetName::LightAction => &[1, 2, 3],
            TrackSetName::HeavyAction => &[1, 2, 3, 4],
            TrackSetName::LightLab => &[1, 2, 6],
            TrackSetName::HeavyLab => &[1, 2, 5, 6],
            TrackSetName::Entering => &[7],
            TrackSetName::BossStage1 => &[7, 8],
            TrackSetName::BossStage2 => &[8, 9],
        }
    }

    pub fn as_set(self) -> BTreeSet<usize> {
        self.tracks().iter().copied().collect()
    }
}

pub struct SoundtrackPlugin;

impl Plugin for SoundtrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundtrackSetRequest>()
            .init_resource::<SoundtrackDebugState>()
            .init_resource::<BossSoundtrackState>()
            .add_systems(Startup, preload_soundtrack_tracks)
            .add_systems(
                Startup,
                setup_soundtrack_entities.after(preload_soundtrack_tracks),
            )
            .add_systems(
                Update,
                (
                    soundtrack_loading_gate,
                    process_soundtrack_requests,
                    update_soundtrack_loop,
                    update_soundtrack_fades,
                    handle_soundtrack_activation_effects,
                )
                    .chain(),
            );
    }
}

#[derive(Resource)]
pub struct SoundtrackHandles {
    pub tracks: [Handle<AudioSource>; TRACK_COUNT],
}

#[derive(Resource)]
pub(crate) struct SoundtrackController {
    track_entities: Vec<Entity>,
    track_states: Vec<TrackRuntimeState>,
    active_set: TrackSetName,
    pending_set: Option<TrackSetName>,
    loop_timer: Timer,
    loop_counter: u64,
    ready: bool,
    apply_at_half: bool,
    half_mark_passed: bool,
}

impl SoundtrackController {
    fn new(track_entities: Vec<Entity>) -> Self {
        let track_count = track_entities.len();
        Self {
            track_entities,
            track_states: vec![TrackRuntimeState::default(); track_count],
            active_set: TrackSetName::Basic,
            pending_set: None,
            loop_timer: Timer::from_seconds(TRACK_LOOP_DURATION_SECONDS, TimerMode::Repeating),
            loop_counter: 0,
            ready: false,
            apply_at_half: false,
            half_mark_passed: false,
        }
    }

    fn has_pending(&self, name: TrackSetName) -> bool {
        self.pending_set == Some(name)
    }

    pub(crate) fn request_track_set(&mut self, name: TrackSetName) -> TrackQueueResult {
        if self.active_set == name {
            self.set_pending(None, false);
            return TrackQueueResult::unchanged();
        }

        if self.has_pending(name) {
            return TrackQueueResult::unchanged();
        }

        let apply_at_half = should_apply_at_half(self);
        self.set_pending(Some(name), apply_at_half);
        let eta = self.time_until_next_change(apply_at_half);

        TrackQueueResult::scheduled(eta)
    }

    fn set_pending(&mut self, set: Option<TrackSetName>, apply_at_half: bool) {
        self.pending_set = set;
        self.apply_at_half = self.pending_set.is_some() && apply_at_half;
    }

    fn time_until_next_change(&self, apply_at_half: bool) -> Option<f32> {
        if !self.ready {
            return None;
        }

        let duration = TRACK_LOOP_DURATION_SECONDS;
        let elapsed = self.loop_timer.elapsed_secs().clamp(0.0, duration);
        let remaining_to_end = (duration - elapsed).max(0.0);

        if apply_at_half {
            let half_point = duration * 0.5;
            if elapsed < half_point {
                Some((half_point - elapsed).max(0.0))
            } else {
                Some(remaining_to_end)
            }
        } else {
            Some(remaining_to_end)
        }
    }

    fn stop_all_tracks(&mut self, sinks: &mut Query<&mut AudioSink, With<SoundtrackTrack>>) {
        for (index, entity) in self.track_entities.iter().enumerate() {
            let Ok(mut sink) = sinks.get_mut(*entity) else {
                continue;
            };

            let state = &mut self.track_states[index];
            state.fade_in = None;
            state.fade_out = None;
            sink.set_volume(Volume::Linear(0.0));
            sink.mute();
        }

        self.pending_set = None;
        self.apply_at_half = false;
    }
}

#[derive(Clone, Default)]
struct TrackRuntimeState {
    fade_in: Option<FadeInState>,
    fade_out: Option<FadeOutState>,
}

#[derive(Clone)]
struct FadeInState {
    timer: Timer,
}

#[derive(Clone)]
struct FadeOutState {
    timer: Timer,
    start_volume: f32,
}

fn should_apply_at_half(controller: &SoundtrackController) -> bool {
    if !controller.ready {
        return true;
    }
    if controller.half_mark_passed {
        return false;
    }
    controller.loop_timer.elapsed_secs() < TRACK_LOOP_DURATION_SECONDS * 0.5
}

#[derive(Component)]
struct SoundtrackTrack;

#[derive(Event, Clone, Debug)]
pub struct SoundtrackSetRequest {
    pub name: TrackSetName,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TrackQueueResult {
    pub eta_seconds: Option<f32>,
}

impl TrackQueueResult {
    pub fn unchanged() -> Self {
        Self {
            eta_seconds: Some(0.0),
        }
    }

    pub fn scheduled(eta_seconds: Option<f32>) -> Self {
        Self { eta_seconds }
    }
}

#[derive(Resource, Clone)]
pub struct SoundtrackDebugState {
    pub loop_counter: u64,
    pub active_set: TrackSetName,
}

impl Default for SoundtrackDebugState {
    fn default() -> Self {
        Self {
            loop_counter: 0,
            active_set: TrackSetName::Basic,
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct BossSoundtrackState {
    pub(crate) music_cut_on_defeat: bool,
}

fn preload_soundtrack_tracks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tracks =
        std::array::from_fn(|index| asset_server.load(format!("ost/track{}.ogg", index + 1)));
    commands.insert_resource(SoundtrackHandles { tracks });
}

fn setup_soundtrack_entities(mut commands: Commands, handles: Res<SoundtrackHandles>) {
    let mut track_entities = Vec::with_capacity(TRACK_COUNT);
    for (index, handle) in handles.tracks.iter().enumerate() {
        let handle = handle.clone();
        let playback_settings = if index == 0 {
            PlaybackSettings::LOOP.with_volume(Volume::Linear(1.0))
        } else {
            PlaybackSettings::LOOP.with_volume(Volume::SILENT)
        };
        let entity = commands
            .spawn((
                Name::new(format!("soundtrack_track_{:02}", index + 1)),
                SoundtrackTrack,
                AudioPlayer::new(handle),
                playback_settings,
            ))
            .id();
        track_entities.push(entity);
    }
    let controller = SoundtrackController::new(track_entities);
    commands.insert_resource(controller);
}

fn soundtrack_loading_gate(
    handles: Res<SoundtrackHandles>,
    audio_sources: Res<Assets<AudioSource>>,
    mut controller: ResMut<SoundtrackController>,
    mut debug_state: ResMut<SoundtrackDebugState>,
    sink_query: Query<Option<&AudioSink>, With<SoundtrackTrack>>,
) {
    if controller.ready {
        return;
    }

    let assets_ready = handles
        .tracks
        .iter()
        .all(|handle| audio_sources.get(handle).is_some());

    if !assets_ready {
        return;
    }

    let sinks_ready = controller
        .track_entities
        .iter()
        .all(|entity| sink_query.get(*entity).ok().and_then(|sink| sink).is_some());

    if !sinks_ready {
        return;
    }

    controller.ready = true;
    controller.loop_counter = 1;
    controller.loop_timer.reset();
    controller.half_mark_passed = false;
    controller.apply_at_half = false;
    debug_state.loop_counter = controller.loop_counter;
    debug_state.active_set = controller.active_set;
}

fn process_soundtrack_requests(
    mut controller: ResMut<SoundtrackController>,
    mut events: EventReader<SoundtrackSetRequest>,
) {
    for request in events.read() {
        let _ = controller.request_track_set(request.name);
    }
}

fn update_soundtrack_loop(
    time: Res<Time>,
    mut controller: ResMut<SoundtrackController>,
    mut debug_state: ResMut<SoundtrackDebugState>,
    mut sinks: Query<&mut AudioSink, With<SoundtrackTrack>>,
) {
    if !controller.ready {
        return;
    }

    controller.loop_timer.tick(time.delta());
    let half_point = TRACK_LOOP_DURATION_SECONDS * 0.5;

    if controller.pending_set.is_some()
        && controller.apply_at_half
        && !controller.half_mark_passed
        && controller.loop_timer.elapsed_secs() >= half_point
    {
        controller.half_mark_passed = true;
        apply_pending_tracks(&mut controller, &mut sinks, debug_state.as_mut());
    }

    if controller.loop_timer.just_finished() {
        controller.loop_counter += 1;
        controller.half_mark_passed = false;

        if controller.pending_set.is_some() {
            apply_pending_tracks(&mut controller, &mut sinks, debug_state.as_mut());
        } else {
            debug_state.active_set = controller.active_set;
        }
        controller.loop_timer.reset();
        debug_state.loop_counter = controller.loop_counter;
        debug_state.active_set = controller.active_set;
    }
}

fn update_soundtrack_fades(
    time: Res<Time>,
    mut controller: ResMut<SoundtrackController>,
    mut sinks: Query<&mut AudioSink, With<SoundtrackTrack>>,
) {
    let delta = time.delta();

    for entity_index in 0..controller.track_entities.len() {
        let entity = controller.track_entities[entity_index];
        let state = &mut controller.track_states[entity_index];

        let Ok(mut sink) = sinks.get_mut(entity) else {
            continue;
        };

        if state.fade_out.is_some() {
            state.fade_in = None;
        }

        if let Some(fade_in_state) = state.fade_in.as_mut() {
            fade_in_state.timer.tick(delta);
            let duration_secs = fade_in_state.timer.duration().as_secs_f32();
            if duration_secs <= f32::EPSILON {
                sink.set_volume(Volume::Linear(1.0));
                state.fade_in = None;
            } else {
                let progress = (fade_in_state.timer.elapsed_secs() / duration_secs).clamp(0.0, 1.0);
                sink.set_volume(Volume::Linear(progress));
                sink.unmute();

                if fade_in_state.timer.finished() {
                    sink.set_volume(Volume::Linear(1.0));
                    state.fade_in = None;
                }
            }
        }

        if let Some(fade_out_state) = state.fade_out.as_mut() {
            fade_out_state.timer.tick(delta);
            let duration_secs = fade_out_state.timer.duration().as_secs_f32();
            if duration_secs > 0.0 {
                let progress =
                    (fade_out_state.timer.elapsed_secs() / duration_secs).clamp(0.0, 1.0);
                let volume = fade_out_state.start_volume * (1.0 - progress);
                sink.set_volume(Volume::Linear(volume.max(0.0)));
            }
            sink.unmute();

            if fade_out_state.timer.finished() {
                sink.set_volume(Volume::Linear(0.0));
                sink.mute();
            }
        }

        if state
            .fade_out
            .as_ref()
            .map_or(false, |fade| fade.timer.finished())
        {
            state.fade_out = None;
        }
    }
}

fn apply_pending_tracks(
    controller: &mut SoundtrackController,
    sinks: &mut Query<&mut AudioSink, With<SoundtrackTrack>>,
    debug_state: &mut SoundtrackDebugState,
) {
    let Some(next_name) = controller.pending_set.take() else {
        return;
    };

    if next_name != controller.active_set {
        apply_track_set(next_name, controller, sinks);
        debug_state.active_set = controller.active_set;
    } else {
        debug_state.active_set = controller.active_set;
    }

    controller.apply_at_half = false;
}

fn apply_track_set(
    new_set: TrackSetName,
    controller: &mut SoundtrackController,
    sinks: &mut Query<&mut AudioSink, With<SoundtrackTrack>>,
) {
    let desired_tracks = new_set.as_set();
    let current_set = controller.active_set.as_set();

    for (entity_index, entity) in controller.track_entities.iter().enumerate() {
        let track_index = entity_index + 1;
        let desired_active = desired_tracks.contains(&track_index);
        let was_active = current_set.contains(&track_index);
        let timing = TRACK_TIMINGS[entity_index];
        let state = &mut controller.track_states[entity_index];

        let Ok(mut sink) = sinks.get_mut(*entity) else {
            continue;
        };

        match (was_active, desired_active) {
            (true, true) => {
                state.fade_out = None;
                sink.unmute();
                if state.fade_in.is_none() {
                    sink.set_volume(Volume::Linear(1.0));
                }
            }
            (true, false) => {
                state.fade_in = None;
                if timing.fade_out_seconds > 0.0 {
                    let start_volume = match sink.volume() {
                        Volume::Linear(value) => value,
                        _ => 1.0,
                    };
                    let timer = Timer::from_seconds(timing.fade_out_seconds, TimerMode::Once);
                    sink.unmute();
                    state.fade_out = Some(FadeOutState {
                        timer,
                        start_volume,
                    });
                } else {
                    state.fade_out = None;
                    sink.set_volume(Volume::Linear(0.0));
                    sink.mute();
                }
            }
            (false, true) => {
                state.fade_out = None;
                sink.unmute();
                if timing.fade_in_offset_seconds > 0.0 {
                    sink.set_volume(Volume::Linear(0.0));
                    let timer = Timer::from_seconds(timing.fade_in_offset_seconds, TimerMode::Once);
                    state.fade_in = Some(FadeInState { timer });
                } else {
                    state.fade_in = None;
                    sink.set_volume(Volume::Linear(1.0));
                }
            }
            (false, false) => {
                state.fade_in = None;
                if state.fade_out.is_none() {
                    sink.set_volume(Volume::Linear(0.0));
                    sink.mute();
                }
            }
        }
    }

    controller.active_set = new_set;
}

fn handle_soundtrack_activation_effects(
    mut controller: ResMut<SoundtrackController>,
    mut boss_state: ResMut<BossSoundtrackState>,
    mut debug_state: ResMut<SoundtrackDebugState>,
    mut sinks: Query<&mut AudioSink, With<SoundtrackTrack>>,
    mut defeat_events: EventReader<BossDefeatedEvent>,
) {
    let mut boss_defeated = false;
    for _ in defeat_events.read() {
        boss_defeated = true;
    }

    if boss_defeated && !boss_state.music_cut_on_defeat {
        boss_state.music_cut_on_defeat = true;
        controller.stop_all_tracks(&mut sinks);
        debug_state.active_set = controller.active_set;
    }
}
