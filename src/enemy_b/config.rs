use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct EnemyBConfig {
    pub throw_interval: f32,
    pub throw_frame_time: f32,
    pub spawn_protection_time: f32,
    pub death_despawn_time: f32,
    pub death_blink_interval: f32,
    pub death_blink_toggles: u8,
    pub death_total_move: f32,
    pub grenade_spawn_offset: Vec2,
    pub grenade_time_of_flight: f32,
    pub grenade_rotation_fps: f32,
    pub grenade_rotation_step: f32,
    pub explosion_frame_time: f32,
}

pub const ENEMY_B_CONFIG: EnemyBConfig = EnemyBConfig {
    throw_interval: 0.8,
    throw_frame_time: 0.12,
    spawn_protection_time: 0.5,
    death_despawn_time: 0.35,
    death_blink_interval: 0.05,
    death_blink_toggles: 6,
    death_total_move: 10.0,
    grenade_spawn_offset: Vec2::new(12.0, 10.0),
    grenade_time_of_flight: 0.9,
    grenade_rotation_fps: 8.0,
    grenade_rotation_step: std::f32::consts::FRAC_PI_4,
    explosion_frame_time: 0.06,
};
