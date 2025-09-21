use super::config::ENEMY_A_CONFIG;
use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyRobot;

#[derive(Component)]
pub struct EnemyRunAnim {
    pub timer: f32,
    pub frame: u8,
}
impl Default for EnemyRunAnim {
    fn default() -> Self {
        Self {
            timer: ENEMY_A_CONFIG.run_frame_time,
            frame: 1,
        }
    }
}

#[derive(Component)]
pub struct EnemyShootTimer {
    pub timer: f32,
    pub fire_delay: f32,
    pub pose_duration: f32,
    pub fired: bool,
}

impl Default for EnemyShootTimer {
    fn default() -> Self {
        Self {
            timer: 0.0,
            fire_delay: ENEMY_A_CONFIG.shoot_fire_delay,
            pose_duration: ENEMY_A_CONFIG.shoot_pose_duration,
            fired: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EnemyRobotStateKind {
    Running,
    Shooting,
    Hit,
}
#[derive(Component)]
pub struct EnemyRobotState {
    pub state: EnemyRobotStateKind,
    pub distance_run: f32,
    pub facing_right: bool,
}
impl Default for EnemyRobotState {
    fn default() -> Self {
        Self {
            state: EnemyRobotStateKind::Running,
            distance_run: 0.0,
            facing_right: false,
        }
    }
}

#[derive(Component)]
pub struct EnemyDespawnTimer {
    pub timer: f32,
}

impl Default for EnemyDespawnTimer {
    fn default() -> Self {
        Self {
            timer: ENEMY_A_CONFIG.death_despawn_time,
        }
    }
}

#[derive(Component)]
pub struct EnemyDeathBlink {
    pub toggles_left: u8,
    pub toggles_total: u8,
    pub timer: f32,
    pub interval: f32,
    pub dir: f32,
    pub moved: f32,
    pub total_move: f32,
}

impl EnemyDeathBlink {
    pub fn new(dir: f32) -> Self {
        Self {
            toggles_left: ENEMY_A_CONFIG.death_blink_toggles,
            toggles_total: ENEMY_A_CONFIG.death_blink_toggles,
            timer: 0.0,
            interval: ENEMY_A_CONFIG.death_blink_interval,
            dir,
            moved: 0.0,
            total_move: ENEMY_A_CONFIG.death_total_move,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyRobotBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub enemy: EnemyRobot,
    pub state: EnemyRobotState,
    pub run_anim: EnemyRunAnim,
    pub shoot_timer: EnemyShootTimer,
}

impl EnemyRobotBundle {
    pub fn new(image: Handle<Image>, translation: Vec3, facing_right: bool) -> Self {
        Self {
            sprite: Sprite::from_image(image),
            transform: Transform::from_translation(translation),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            enemy: EnemyRobot,
            state: EnemyRobotState {
                state: EnemyRobotStateKind::Running,
                distance_run: 0.0,
                facing_right,
            },
            run_anim: EnemyRunAnim::default(),
            shoot_timer: EnemyShootTimer::default(),
        }
    }
}
#[derive(Component)]
pub struct EnemySpawnProtection {
    pub timer: f32,
}

impl Default for EnemySpawnProtection {
    fn default() -> Self {
        Self {
            timer: ENEMY_A_CONFIG.spawn_protection_time,
        }
    }
}

pub const ENEMY_ROBOT_WIDTH: f32 = 50.0;
pub const ENEMY_ROBOT_HEIGHT: f32 = 70.0;
