use super::config::ENEMY_B_CONFIG;
use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyB;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyBStateKind {
    Sitting,
    Throwing,
    Hit,
}

#[derive(Component)]
pub struct EnemyBState {
    pub state: EnemyBStateKind,
    pub facing_right: bool,
}
impl Default for EnemyBState {
    fn default() -> Self {
        Self {
            state: EnemyBStateKind::Sitting,
            facing_right: true,
        }
    }
}

#[derive(Component, Default)]
pub struct EnemyBThrowAnim {
    pub timer: f32,
    pub frame: u8,
    pub thrown: bool,
}

impl EnemyBThrowAnim {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            frame: 0,
            thrown: false,
        }
    }
}

#[derive(Component, Default)]
pub struct EnemyBThrowTimer {
    pub timer: f32,
}

#[derive(Component)]
pub struct EnemyBDespawnTimer {
    pub timer: f32,
}

impl EnemyBDespawnTimer {
    pub fn new() -> Self {
        Self {
            timer: ENEMY_B_CONFIG.death_despawn_time,
        }
    }
}

#[derive(Component)]
pub struct EnemyBSpawnProtection {
    pub timer: f32,
}

impl EnemyBSpawnProtection {
    pub fn new() -> Self {
        Self {
            timer: ENEMY_B_CONFIG.spawn_protection_time,
        }
    }
}

#[derive(Component)]
pub struct EnemyBDeathBlink {
    pub toggles_left: u8,
    pub toggles_total: u8,
    pub timer: f32,
    pub interval: f32,
    pub dir: f32,
    pub moved: f32,
    pub total_move: f32,
}

impl EnemyBDeathBlink {
    pub fn new(dir: f32) -> Self {
        Self {
            toggles_left: ENEMY_B_CONFIG.death_blink_toggles,
            toggles_total: ENEMY_B_CONFIG.death_blink_toggles,
            timer: 0.0,
            interval: ENEMY_B_CONFIG.death_blink_interval,
            dir,
            moved: 0.0,
            total_move: ENEMY_B_CONFIG.death_total_move,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub enemy: EnemyB,
    pub state: EnemyBState,
    pub anim: EnemyBThrowAnim,
    pub throw_timer: EnemyBThrowTimer,
}

impl EnemyBBundle {
    pub fn new(image: Handle<Image>, translation: Vec3, facing_right: bool) -> Self {
        Self {
            sprite: Sprite::from_image(image),
            transform: Transform::from_translation(translation),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            enemy: EnemyB,
            state: EnemyBState {
                state: EnemyBStateKind::Sitting,
                facing_right,
            },
            anim: EnemyBThrowAnim::new(),
            throw_timer: EnemyBThrowTimer::default(),
        }
    }
}

#[derive(Component)]
pub struct Grenade {
    pub velocity: Vec2,
    pub rotation_timer: f32,
}

#[derive(Component)]
pub struct ExplosionAnim {
    pub timer: f32,
    pub frame: u8,
}

pub const ENEMY_B_WIDTH: f32 = 50.0;
pub const ENEMY_B_HEIGHT: f32 = 70.0;
