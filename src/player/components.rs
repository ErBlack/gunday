#[derive(Component, Default)]
pub struct PlayerJumpAnim {
    pub timer: f32,
    pub frame: u8,
    pub rotation: f32,
}
#[derive(Component, Default)]
pub struct PlayerRunAnim {
    pub timer: f32,
    pub frame: u8,
}
#[derive(Component, Default)]
pub struct PlayerShootingAnim {
    pub timer: f32,
    pub frame: u8,
}
use super::config::PLAYER_CONFIG;
use crate::constants::DEFAULT_GRAVITY;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayerSpriteKind {
    Static,
    Up,
    Shooting,
    Run(u8),
    RunUp(u8),
    RunDown(u8),
    Jump,
    Hit,
    Fallen,
    Win,
}

impl Default for PlayerSpriteKind {
    fn default() -> Self {
        PlayerSpriteKind::Static
    }
}

#[derive(Component)]
pub struct PlayerSpriteOffset(pub Vec2);

impl Default for PlayerSpriteOffset {
    fn default() -> Self {
        PlayerSpriteOffset(Vec2::ZERO)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerSpriteEntity(pub Entity);

#[derive(Component)]
pub struct PlayerLives {
    pub current: u8,
    pub max: u8,
}

impl PlayerLives {
    pub fn new(current: u8, max: u8) -> Self {
        Self {
            current: current.min(max),
            max,
        }
    }

    pub fn lose_life(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn restore_full(&mut self, target: u8) {
        self.current = target.min(self.max);
    }
}

#[derive(Component)]
pub struct PlayerInvincibility {
    pub timer: f32,
}

#[derive(Component)]
pub struct PlayerProne {
    pub landed: bool,
    pub timer: f32,
}

impl PlayerProne {
    pub fn new(duration: f32) -> Self {
        Self {
            landed: false,
            timer: duration,
        }
    }
}

#[derive(Component)]
pub struct PlayerRespawning {
    pub timer: f32,
}

#[derive(Component)]
pub struct PlayerGameOver {
    pub timer: f32,
    pub started: bool,
    pub sfx_played: bool,
    pub sfx_delay: f32,
    pub result_sent: bool,
}

impl PlayerGameOver {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: duration,
            started: false,
            sfx_played: false,
            sfx_delay: 0.5,
            result_sent: false,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerSpawnPoint(pub Vec3);

#[derive(Component)]
pub struct PlayerBlink {
    pub timer: f32,
    pub interval: f32,
    pub visible: bool,
}

impl PlayerBlink {
    pub fn new(interval: f32) -> Self {
        Self {
            timer: interval,
            interval,
            visible: true,
        }
    }

    pub fn reset(&mut self) {
        self.timer = self.interval;
        self.visible = true;
    }
}

#[derive(Component)]
pub struct PlayerHeartsRoot;

#[derive(Component)]
pub struct PlayerHeartIcon {
    pub index: u8,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub player: Player,
    pub prev_position: PlayerPrevPosition,
    pub velocity: Velocity,
    pub gravity: Gravity,
    pub grounded: Grounded,
    pub jump_state: JumpState,
    pub direction: PlayerDirection,
    pub shooting_state: ShootingState,
    pub sprite_size: super::setup_player::SpriteSize,
    pub shooting_anim: PlayerShootingAnim,
    pub run_anim: PlayerRunAnim,
    pub jump_anim: PlayerJumpAnim,
}

impl PlayerBundle {
    pub fn new(transform: Transform, sprite_size: super::setup_player::SpriteSize) -> Self {
        let mut gravity = Gravity::default();
        gravity.force = PLAYER_CONFIG.gravity_force;

        let mut jump_state = JumpState::default();
        jump_state.max_jump_duration = PLAYER_CONFIG.max_jump_duration;
        jump_state.jump_buffer_time = PLAYER_CONFIG.jump_buffer_time;

        let mut shooting_state = ShootingState::default();
        shooting_state.shot_cooldown = PLAYER_CONFIG.shot_cooldown;

        Self {
            transform,
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            player: Player,
            prev_position: PlayerPrevPosition::default(),
            velocity: Velocity::default(),
            gravity,
            grounded: Grounded::default(),
            jump_state,
            direction: PlayerDirection::default(),
            shooting_state,
            sprite_size,
            shooting_anim: PlayerShootingAnim::default(),
            run_anim: PlayerRunAnim::default(),
            jump_anim: PlayerJumpAnim::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct PlayerPrevPosition {
    pub x: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct Gravity {
    pub force: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            force: DEFAULT_GRAVITY,
        }
    }
}

#[derive(Component)]
pub struct Grounded {
    pub is_grounded: bool,
}

impl Default for Grounded {
    fn default() -> Self {
        Self { is_grounded: true }
    }
}

#[derive(Component)]
pub struct JumpState {
    pub is_jumping: bool,
    pub jump_timer: f32,
    pub max_jump_duration: f32,
    pub jump_buffer_timer: f32,
    pub jump_buffer_time: f32,
}

impl Default for JumpState {
    fn default() -> Self {
        Self {
            is_jumping: false,
            jump_timer: 0.0,
            max_jump_duration: 0.7,
            jump_buffer_timer: 0.0,
            jump_buffer_time: 0.1,
        }
    }
}

#[derive(Component)]
pub struct PlayerDirection {
    pub facing_right: bool,
    pub last_movement_direction: f32,
}

impl Default for PlayerDirection {
    fn default() -> Self {
        Self {
            facing_right: true,
            last_movement_direction: 1.0,
        }
    }
}

#[derive(Component)]
pub struct ShootingState {
    pub last_shot_timer: f32,
    pub shot_cooldown: f32,
}

impl Default for ShootingState {
    fn default() -> Self {
        Self {
            last_shot_timer: 0.0,
            shot_cooldown: 0.12,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ActionTrigger {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
    pub held_for: Option<f32>,
}

impl ActionTrigger {
    pub fn update(&mut self, pressed: bool, dt: f32) {
        let was_pressed = self.pressed;
        let previous_held = self.held_for.unwrap_or(0.0);

        self.just_pressed = pressed && !was_pressed;
        self.just_released = !pressed && was_pressed;
        self.pressed = pressed;

        self.held_for = if pressed {
            Some(if was_pressed { previous_held + dt } else { 0.0 })
        } else {
            None
        };
    }
}

#[derive(Debug, Clone, Resource)]
pub struct PlayerActions {
    pub move_axis: f32,
    pub aim_axis: Vec2,
    pub jump: ActionTrigger,
    pub shoot: ActionTrigger,
    pub dash: ActionTrigger,
    pub aim_up: ActionTrigger,
    pub aim_down: ActionTrigger,
}

impl Default for PlayerActions {
    fn default() -> Self {
        Self {
            move_axis: 0.0,
            aim_axis: Vec2::ZERO,
            jump: ActionTrigger::default(),
            shoot: ActionTrigger::default(),
            dash: ActionTrigger::default(),
            aim_up: ActionTrigger::default(),
            aim_down: ActionTrigger::default(),
        }
    }
}

impl PlayerActions {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
