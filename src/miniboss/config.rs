use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct MinibossConfig {
    pub width: f32,
    pub height: f32,
    pub right_limit_x: f32,
    pub move_speed: f32,
    pub hit_points: i32,
    pub pre_volley_wait: f32,
    pub post_volley_wait: f32,
    pub death_wait: f32,
    pub death_blink_duration: f32,
    pub death_blink_interval: f32,
    pub death_explosion_delay: f32,
    pub grenade_spacing: f32,
    pub grenade_initial_velocity_y: f32,
    pub grenade_rotation_fps: f32,
    pub grenade_rotation_step: f32,
    pub grenade_points: &'static [Vec2],
    pub move_frame_time: f32,
    pub shoot_frame_time: f32,
    pub forced_retreat_trigger_distance: f32,
    pub forced_retreat_release_distance: f32,
}

pub const MINIBOSS_GRENADE_POINTS: [Vec2; 6] = [
    Vec2::new(189.0, 24.0),
    Vec2::new(182.0, 34.0),
    Vec2::new(180.0, 11.0),
    Vec2::new(169.0, 23.0),
    Vec2::new(169.0, 3.0),
    Vec2::new(159.0, 9.0),
];

pub const MINIBOSS_CONFIG: MinibossConfig = MinibossConfig {
    width: 224.0,
    height: 176.0,
    right_limit_x: 6000.0,
    move_speed: 120.0,
    hit_points: 50,
    pre_volley_wait: 0.25,
    post_volley_wait: 0.5,
    death_wait: 1.1,
    death_blink_duration: 1.0,
    death_blink_interval: 0.05,
    death_explosion_delay: 0.12,
    grenade_spacing: 0.12,
    grenade_initial_velocity_y: 620.0,
    grenade_rotation_fps: 8.0,
    grenade_rotation_step: std::f32::consts::FRAC_PI_4,
    grenade_points: &MINIBOSS_GRENADE_POINTS,
    move_frame_time: 0.18,
    shoot_frame_time: 0.14,
    forced_retreat_trigger_distance: 100.0,
    forced_retreat_release_distance: 300.0,
};
