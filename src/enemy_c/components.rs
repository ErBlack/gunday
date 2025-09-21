use super::config::ENEMY_C_CONFIG;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component)]
pub struct EnemyC;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyCStateKind {
    Running,
    JumpWindup,
    Jumping,
    Dying,
}

#[derive(Component)]
pub struct EnemyCState {
    pub state: EnemyCStateKind,
    pub time_in_state: f32,
    pub facing_right: bool,
}

impl Default for EnemyCState {
    fn default() -> Self {
        Self {
            state: EnemyCStateKind::Running,
            time_in_state: 0.0,
            facing_right: false,
        }
    }
}

#[derive(Component)]
pub struct EnemyCHitPoints {
    pub current: u8,
}

impl EnemyCHitPoints {
    pub const fn new(max: u8) -> Self {
        Self { current: max }
    }

    pub fn take_hit(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
        }
        self.current == 0
    }
}

#[derive(Component, Default)]
pub struct EnemyCVelocity {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct EnemyCAnimation {
    pub timer: f32,
    pub frame: usize,
}

impl Default for EnemyCAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            frame: 0,
        }
    }
}

#[derive(Component)]
pub struct EnemyCJumpController {
    pub cooldown: f32,
}

impl Default for EnemyCJumpController {
    fn default() -> Self {
        Self { cooldown: 0.0 }
    }
}

#[derive(Component)]
pub struct EnemyCHitFlash {
    pub timer: f32,
}

impl EnemyCHitFlash {
    pub const fn new() -> Self {
        Self {
            timer: ENEMY_C_CONFIG.hit_flash_duration,
        }
    }
}

#[derive(Component)]
pub struct EnemyCDeathBlink {
    pub toggles_left: u8,
    pub toggles_total: u8,
    pub timer: f32,
    pub interval: f32,
    pub dir: f32,
    pub moved: f32,
    pub total_move: f32,
}

impl EnemyCDeathBlink {
    pub const fn new(dir: f32) -> Self {
        Self {
            toggles_left: ENEMY_C_CONFIG.death_blink_toggles,
            toggles_total: ENEMY_C_CONFIG.death_blink_toggles,
            timer: 0.0,
            interval: ENEMY_C_CONFIG.death_blink_interval,
            dir,
            moved: 0.0,
            total_move: ENEMY_C_CONFIG.death_total_move,
        }
    }
}

#[derive(Component)]
pub struct EnemyCDespawnTimer {
    pub timer: f32,
}

impl EnemyCDespawnTimer {
    pub const fn new() -> Self {
        Self {
            timer: ENEMY_C_CONFIG.death_despawn_time,
        }
    }
}

#[derive(Component)]
pub struct EnemyCSpawnPause {
    pub timer: f32,
}

impl EnemyCSpawnPause {
    pub const fn new(duration: f32) -> Self {
        Self { timer: duration }
    }
}

#[derive(Bundle)]
pub struct EnemyCBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub enemy: EnemyC,
    pub state: EnemyCState,
    pub hp: EnemyCHitPoints,
    pub velocity: EnemyCVelocity,
    pub animation: EnemyCAnimation,
    pub jump: EnemyCJumpController,
}

impl EnemyCBundle {
    pub fn new(image: Handle<Image>, translation: Vec3, facing_right: bool) -> Self {
        let mut sprite = Sprite::from_image(image);
        sprite.anchor = Anchor::BottomCenter;
        Self {
            sprite,
            transform: Transform::from_translation(translation),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            enemy: EnemyC,
            state: EnemyCState {
                state: EnemyCStateKind::Running,
                time_in_state: 0.0,
                facing_right,
            },
            hp: EnemyCHitPoints::new(ENEMY_C_CONFIG.hit_points),
            velocity: EnemyCVelocity::default(),
            animation: EnemyCAnimation::default(),
            jump: EnemyCJumpController {
                cooldown: ENEMY_C_CONFIG.jump_cooldown_duration,
            },
        }
    }
}

pub const ENEMY_C_WIDTH: f32 = 48.0;
pub const ENEMY_C_HEIGHT: f32 = 70.0;
