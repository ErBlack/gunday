use bevy::audio::{AudioPlayer, AudioSink, AudioSource, PlaybackSettings, SpatialAudioSink};
use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct SfxEmitters {
    pub player_shoot: Entity,
    pub player_hit: Entity,
    pub player_game_over: Entity,
    pub enemy_shoot: Entity,
    pub enemy_hit: Entity,
    pub enemy_explosion: Entity,
    pub enemy_death: Entity,
    pub enemy_transform: Entity,
    pub boss_hit: Entity,
    pub boss_shot: Entity,
    pub boss_defeat: Entity,
    pub boss_win: Entity,
}

#[derive(Component)]
struct SfxEmitter;

pub fn setup_audio_emitters(mut commands: Commands) {
    let spawn_emitter = |commands: &mut Commands, label: &str| -> Entity {
        commands
            .spawn((Name::new(label.to_owned()), SfxEmitter))
            .id()
    };

    let player_shoot = spawn_emitter(&mut commands, "sfx: player_shoot");
    let player_hit = spawn_emitter(&mut commands, "sfx: player_hit");
    let player_game_over = spawn_emitter(&mut commands, "sfx: player_game_over");
    let enemy_shoot = spawn_emitter(&mut commands, "sfx: enemy_shoot");
    let enemy_explosion = spawn_emitter(&mut commands, "sfx: enemy_explosion");
    let enemy_hit = spawn_emitter(&mut commands, "sfx: enemy_hit");
    let enemy_death = spawn_emitter(&mut commands, "sfx: enemy_death");
    let enemy_transform = spawn_emitter(&mut commands, "sfx: enemy_transform");
    let boss_hit = spawn_emitter(&mut commands, "sfx: boss_hit");
    let boss_shot = spawn_emitter(&mut commands, "sfx: boss_shot");
    let boss_defeat = spawn_emitter(&mut commands, "sfx: boss_defeat");
    let boss_win = spawn_emitter(&mut commands, "sfx: boss_win");

    commands.insert_resource(SfxEmitters {
        player_shoot,
        player_hit,
    player_game_over,
        enemy_shoot,
        enemy_hit,
        enemy_explosion,
        enemy_death,
        enemy_transform,
        boss_hit,
        boss_shot,
        boss_defeat,
        boss_win,
    });
}

pub fn play_sfx(
    commands: &mut Commands,
    emitter: Entity,
    clip: Handle<AudioSource>,
    settings: PlaybackSettings,
) {
    let mut entity_commands = commands.entity(emitter);
    entity_commands.remove::<AudioPlayer<AudioSource>>();
    entity_commands.remove::<PlaybackSettings>();
    entity_commands.remove::<AudioSink>();
    entity_commands.remove::<SpatialAudioSink>();
    entity_commands.insert(AudioPlayer::new(clip));
    entity_commands.insert(settings);
}

pub fn play_sfx_once(commands: &mut Commands, emitter: Entity, clip: Handle<AudioSource>) {
    play_sfx(commands, emitter, clip, PlaybackSettings::REMOVE);
}
