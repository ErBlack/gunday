use super::config::MINIBOSS_CONFIG;
use crate::assets::GameAssets;
use crate::constants::{
    DEFAULT_GRAVITY, GROUND_RECT_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, Z_ENEMY_BASE,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Component)]
pub struct Miniboss;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MinibossPhase {
    MoveToForwardAnchor,
    PreVolley,
    VolleyFirst,
    PostVolley,
    RetreatRight,
    ReturnForward,
    Dying,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MinibossAnimVariant {
    Move,
    Shoot,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MinibossVolleyKind {
    First,
    Second,
}

#[derive(Clone, Copy, Debug)]
pub struct MinibossVolley {
    pub kind: MinibossVolleyKind,
    pub next_shot_timer: f32,
    pub shot_index: u8,
}

impl MinibossVolley {
    pub fn new(kind: MinibossVolleyKind) -> Self {
        Self {
            kind,
            next_shot_timer: 0.0,
            shot_index: 0,
        }
    }
}

#[derive(Component, Debug)]
pub struct MinibossBehavior {
    pub phase: MinibossPhase,
    pub timer: f32,
    pub target_x: f32,
    pub movement_dir: f32,
    pub phase_start_x: f32,
    pub forward_anchor_x: f32,
    pub clamp_max_x: f32,
    pub home_right_limit_x: f32,
    pub entry_max_x: f32,
    pub move_speed: f32,
    pub facing_right: bool,
    pub volley: Option<MinibossVolley>,
    pub mid_retreat_volley: Option<MinibossVolley>,
    pub half_retreat_triggered: bool,
    pub forced_retreat: bool,
}

impl MinibossBehavior {
    pub fn new(
        spawn_x: f32,
        forward_anchor_x: f32,
        clamp_max_x: f32,
        home_right_limit_x: f32,
    ) -> Self {
        let target_x = forward_anchor_x;
        let movement_dir = if target_x < spawn_x { -1.0 } else { 1.0 };
        Self {
            phase: MinibossPhase::MoveToForwardAnchor,
            timer: 0.0,
            target_x,
            movement_dir,
            phase_start_x: spawn_x,
            forward_anchor_x,
            clamp_max_x,
            home_right_limit_x,
            entry_max_x: clamp_max_x,
            move_speed: MINIBOSS_CONFIG.move_speed,
            facing_right: movement_dir > 0.0,
            volley: None,
            mid_retreat_volley: None,
            half_retreat_triggered: false,
            forced_retreat: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct MinibossAnimation {
    pub timer: f32,
    pub frame: u8,
    pub variant: MinibossAnimVariant,
    pub paused: bool,
}

impl Default for MinibossAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            frame: 0,
            variant: MinibossAnimVariant::Move,
            paused: true,
        }
    }
}

#[derive(Component)]
pub struct MinibossHealth {
    pub hp: i32,
}

impl MinibossHealth {
    pub fn new() -> Self {
        Self {
            hp: MINIBOSS_CONFIG.hit_points,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinibossDeathPhase {
    Wait,
    Blink,
}

#[derive(Component, Debug)]
pub struct MinibossDeath {
    pub phase: MinibossDeathPhase,
    pub timer: f32,
    pub blink_timer: f32,
    pub visible: bool,
    pub blink_duration: f32,
    pub blink_interval: f32,
    pub explosion_timer: f32,
    pub explosion_index: usize,
}

impl MinibossDeath {
    pub fn new() -> Self {
        let blink_interval = MINIBOSS_CONFIG.death_blink_interval;
        Self {
            phase: MinibossDeathPhase::Wait,
            timer: MINIBOSS_CONFIG.death_wait,
            blink_timer: blink_interval,
            visible: true,
            blink_duration: MINIBOSS_CONFIG.death_blink_duration,
            blink_interval,
            explosion_timer: 0.0,
            explosion_index: 0,
        }
    }
}

pub const MINIBOSS_DEATH_EXPLOSION_OFFSETS: [Vec2; 3] = [
    Vec2::new(-40.0, 80.0),
    Vec2::new(30.0, 110.0),
    Vec2::new(15.0, 40.0),
];

#[derive(Component, Debug)]
pub struct MinibossGrenade {
    pub velocity: Vec2,
    pub rotation_timer: f32,
}

#[derive(Event)]
pub struct MinibossFireEvent {
    pub entity: Entity,
    pub volley_kind: MinibossVolleyKind,
    pub shot_index: u8,
}

#[derive(Bundle)]
pub struct MinibossBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub miniboss: Miniboss,
    pub behavior: MinibossBehavior,
    pub animation: MinibossAnimation,
    pub health: MinibossHealth,
}

impl MinibossBundle {
    pub fn new(
        image: Handle<Image>,
        position: Vec3,
        behavior: MinibossBehavior,
        health: MinibossHealth,
    ) -> Self {
        let mut sprite = Sprite::from_image(image);
        sprite.anchor = Anchor::BottomCenter;
        sprite.flip_x = behavior.facing_right;
        Self {
            sprite,
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            miniboss: Miniboss,
            behavior,
            animation: MinibossAnimation::default(),
            health,
        }
    }
}

pub fn miniboss_right_bound_x() -> f32 {
    MINIBOSS_CONFIG.right_limit_x - MINIBOSS_CONFIG.width / 2.0
}

pub fn miniboss_screen_right_x(camera_x: f32) -> f32 {
    camera_x + SCREEN_WIDTH / 2.0 - MINIBOSS_CONFIG.width / 2.0
}

pub fn miniboss_screen_left_x(camera_x: f32) -> f32 {
    camera_x - SCREEN_WIDTH / 4.0 + MINIBOSS_CONFIG.width / 2.0
}

pub fn miniboss_offscreen_left_bound(camera_x: f32) -> f32 {
    camera_x - SCREEN_WIDTH / 2.0 - MINIBOSS_CONFIG.width / 2.0
}

pub fn miniboss_ground_y() -> f32 {
    GROUND_RECT_HEIGHT - SCREEN_HEIGHT / 2.0
}

pub fn miniboss_time_of_flight() -> f32 {
    (2.0 * MINIBOSS_CONFIG.grenade_initial_velocity_y) / -DEFAULT_GRAVITY
}

pub fn miniboss_grenade_offset(point: Vec2) -> Vec2 {
    Vec2::new(
        point.x - MINIBOSS_CONFIG.width / 2.0,
        MINIBOSS_CONFIG.height - point.y,
    )
}

pub fn miniboss_bundle_at(
    game_assets: &GameAssets,
    spawn_x: f32,
    clamp_max_x: f32,
    forward_anchor_x: f32,
) -> MinibossBundle {
    let position = Vec3::new(spawn_x, miniboss_ground_y(), Z_ENEMY_BASE);
    let behavior = MinibossBehavior::new(
        spawn_x,
        forward_anchor_x,
        clamp_max_x,
        miniboss_right_bound_x(),
    );
    MinibossBundle::new(
        game_assets.miniboss_move_a.clone(),
        position,
        behavior,
        MinibossHealth::new(),
    )
}
