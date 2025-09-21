use super::config::BOSS_SETTINGS;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossTorso;

#[derive(Component)]
pub struct BossHead;

#[derive(Component)]
pub struct BossSpine {
    pub hp: u8,
}

impl Default for BossSpine {
    fn default() -> Self {
        Self {
            hp: BOSS_SETTINGS.stage1.spine_hp,
        }
    }
}

#[derive(Component)]
pub struct BossSpineHitAnimation {
    pub timer: f32,
    pub original_rotation: f32,
    pub stored_gun_angle: Option<f32>,
    pub cannon_entity: Option<Entity>,
}

#[derive(Component)]
pub struct BossCannon;

#[derive(Component)]
pub struct BossArm {
    pub is_left: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BossPartKind {
    Torso,
    Head,
    Spine,
    Cannon,
    LeftArm,
    RightArm,
}

#[derive(Resource, Clone)]
pub struct BossAudio {
    pub hit: Handle<AudioSource>,
    pub shot: Handle<AudioSource>,
    pub defeat: Handle<AudioSource>,
    pub win: Handle<AudioSource>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BossStageKind {
    Stage1,
    TransitionToStage2,
    Stage2,
    Exploding,
}

#[derive(Component)]
pub struct BossStage(pub BossStageKind);

#[derive(Component, Default)]
pub struct BossMovementTimer {
    pub timer: f32,
    pub waving_amplitude: f32,
}

#[derive(Component)]
pub struct BossStage1ShootingState {
    pub shoot_timer: f32,
    pub aim_timer: f32,
    pub aiming: bool,
    pub shooting: bool,
    pub target: Vec2,
    pub locked_target: Option<Vec2>,
    pub shot_count: u8,
    pub shots_per_burst: u8,
    pub aim_cooldown: f32,
}

impl Default for BossStage1ShootingState {
    fn default() -> Self {
        Self {
            shoot_timer: BOSS_SETTINGS.stage1.shooting.initial_shoot_timer,
            aim_timer: BOSS_SETTINGS.stage1.shooting.initial_aim_timer,
            aiming: false,
            shooting: false,
            target: Vec2::ZERO,
            locked_target: None,
            shot_count: 0,
            shots_per_burst: BOSS_SETTINGS.stage1.shooting.shots_per_burst,
            aim_cooldown: 0.0,
        }
    }
}

#[derive(Component, Default)]
pub struct BossStage1State;

#[derive(Component)]
pub struct BossStage1MovementState {
    pub hover_base_y: Option<f32>,
    pub pending_move_request: bool,
    pub moving: bool,
    pub move_timer: f32,
    pub move_duration: f32,
    pub move_from: Vec2,
    pub move_to: Vec2,
}

impl Default for BossStage1MovementState {
    fn default() -> Self {
        Self {
            hover_base_y: None,
            pending_move_request: false,
            moving: false,
            move_timer: 0.0,
            move_duration: BOSS_SETTINGS.stage1.movement.move_duration,
            move_from: Vec2::ZERO,
            move_to: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct BossStage2Pose {
    pub initialized: bool,
}

impl Default for BossStage2Pose {
    fn default() -> Self {
        Self { initialized: false }
    }
}

#[derive(Component)]
pub struct BossStage2State {
    pub crawl_speed: f32,
    pub head_hp: u8,
    pub crawl_timer: f32,
}

impl Default for BossStage2State {
    fn default() -> Self {
        Self {
            crawl_speed: BOSS_SETTINGS.stage2.crawl_speed,
            head_hp: BOSS_SETTINGS.stage2.head_hp,
            crawl_timer: 0.0,
        }
    }
}

#[derive(Component)]
pub struct BossExplodingState {
    pub timer: f32,
    pub converted: bool,
    pub released: usize,
    pub defeat_sound_played: bool,
    pub win_started: bool,
    pub win_timer: f32,
    pub result_sent: bool,
    pub pre_explosions_spawned: u8,
    pub pre_explosion_timer: f32,
    pub final_blast_triggered: bool,
}

impl Default for BossExplodingState {
    fn default() -> Self {
        Self {
            timer: 0.0,
            converted: false,
            released: 0,
            defeat_sound_played: false,
            win_started: false,
            win_timer: 0.0,
            result_sent: false,
            pre_explosions_spawned: 0,
            pre_explosion_timer: 0.0,
            final_blast_triggered: false,
        }
    }
}

#[derive(Component)]
pub struct ExplodingPart {
    pub velocity: Vec2,
    pub angular_velocity: f32,
}

impl Default for ExplodingPart {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            angular_velocity: 0.0,
        }
    }
}

#[derive(Component, Default)]
pub struct BossFacing {
    pub right: bool,
}

#[derive(Component)]
pub struct BossGunRotation {
    pub current_angle: f32,
    pub max_angular_speed: f32,
}

impl Default for BossGunRotation {
    fn default() -> Self {
        Self {
            current_angle: 0.0,
            max_angular_speed: BOSS_SETTINGS.cannon.max_rotation_speed,
        }
    }
}

#[derive(Component, Default)]
pub struct BossParts {
    entities: HashMap<BossPartKind, Entity>,
}

impl BossParts {
    pub fn insert(&mut self, kind: BossPartKind, entity: Entity) {
        self.entities.insert(kind, entity);
    }

    pub fn get(&self, kind: BossPartKind) -> Option<Entity> {
        self.entities.get(&kind).copied()
    }
}

#[derive(Component)]
pub struct BossCollider {
    pub half_size: Vec2,
}

#[derive(Component)]
pub struct BossStage2TransitionState {
    pub timer: f32,
    pub phase: TransitionPhase,
    pub velocity: Vec2,
    pub target_edge_x: f32,
    pub downed_wait: f32,
    pub detach_to_right: bool,
    pub pending_music_eta: f32,
}

impl Default for BossStage2TransitionState {
    fn default() -> Self {
        Self {
            timer: 0.0,
            phase: TransitionPhase::Blast,
            velocity: Vec2::ZERO,
            target_edge_x: 0.0,
            downed_wait: BOSS_SETTINGS.transition.downed_wait,
            detach_to_right: true,
            pending_music_eta: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionPhase {
    Blast,
    Downed,
    AwaitStage2Music,
    TurnToPlayer,
}

#[derive(Component)]
pub struct BossTransitionExplosionQueue {
    pub position: Vec3,
    pub remaining: u8,
    pub total: u8,
    pub timer: f32,
    pub interval: f32,
}

#[derive(Component)]
pub struct DetachedCannon {
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub target_angle: Option<f32>,
}

impl Default for DetachedCannon {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            target_angle: None,
        }
    }
}

#[derive(Component)]
pub struct DetachCannonNow {
    pub go_right: bool,
}


