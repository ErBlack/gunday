use bevy::prelude::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct EnemyAConfig {
    pub run_speed: f32,
    pub run_distance_before_shoot: f32,
    pub run_frame_time: f32,
    pub shoot_fire_delay: f32,
    pub shoot_pose_duration: f32,
    pub spawn_protection_time: f32,
    pub death_despawn_time: f32,
    pub death_blink_interval: f32,
    pub death_blink_toggles: u8,
    pub death_total_move: f32,
    pub projectile_spawn_offset: Vec3,
    pub spawn_ground_offset: f32,
}

pub const ENEMY_A_CONFIG: EnemyAConfig = EnemyAConfig {
    run_speed: 220.0,
    run_distance_before_shoot: 200.0,
    run_frame_time: 0.12,
    shoot_fire_delay: 0.25,
    shoot_pose_duration: 0.5,
    spawn_protection_time: 0.6,
    death_despawn_time: 0.35,
    death_blink_interval: 0.05,
    death_blink_toggles: 6,
    death_total_move: 5.0,
    projectile_spawn_offset: Vec3::new(0.0, 16.0, 0.0),
    spawn_ground_offset: 5.0,
};
