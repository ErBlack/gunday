#[derive(Debug, Clone, Copy)]
pub struct EnemyCConfig {
    pub hit_points: u8,
    pub run_speed: f32,
    pub jump_horizontal_speed: f32,
    pub jump_vertical_speed: f32,
    pub jump_trigger_distance: f32,
    pub jump_windup_duration: f32,
    pub jump_cooldown_duration: f32,
    pub run_frame_time: f32,
    pub hit_flash_duration: f32,
    pub death_despawn_time: f32,
    pub death_blink_interval: f32,
    pub death_blink_toggles: u8,
    pub death_total_move: f32,
    pub death_ground_offset: f32,
    pub explosion_vertical_offset: f32,
    pub spawn_ground_offset: f32,
}

pub const ENEMY_C_CONFIG: EnemyCConfig = EnemyCConfig {
    hit_points: 3,
    run_speed: 280.0,
    jump_horizontal_speed: 360.0,
    jump_vertical_speed: 550.0,
    jump_trigger_distance: 320.0,
    jump_windup_duration: 0.25,
    jump_cooldown_duration: 1.5,
    run_frame_time: 0.09,
    hit_flash_duration: 0.16,
    death_despawn_time: 0.45,
    death_blink_interval: 0.05,
    death_blink_toggles: 6,
    death_total_move: 10.0,
    death_ground_offset: 34.0,
    explosion_vertical_offset: 80.0,
    spawn_ground_offset: 5.0,
};

#[derive(Debug, Clone, Copy)]
pub struct EnemyCConstants {
    pub dynamic_spawn_interval: f32,
    pub dynamic_spawn_limit: usize,
}

pub const ENEMY_C_CONSTANTS: EnemyCConstants = EnemyCConstants {
    dynamic_spawn_interval: 0.8,
    dynamic_spawn_limit: 6,
};
