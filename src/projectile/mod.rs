pub mod components;
pub mod projectile_movement_system;
pub mod projectile_collision_system;
pub mod projectile_spawning_system;

pub use projectile_movement_system::projectile_movement_system;
pub use projectile_collision_system::projectile_collision_system;
pub use projectile_spawning_system::spawn_projectile;
