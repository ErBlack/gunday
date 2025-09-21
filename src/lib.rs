use crate::player::player_jump_anim_system::player_jump_anim_system;
use bevy::app::AppExit;
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use player::player_run_anim_system::player_run_anim_system;
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

mod assets;
pub mod audio;
mod boss;
mod collision;
mod components;
mod constants;
mod effects;
mod enemy_a;
mod enemy_b;
mod enemy_c;
mod game_state;
mod miniboss;
mod player;
mod projectile;
mod soundtrack;
mod spawn;
mod systems;
mod web_console;

static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

use audio::setup_audio_emitters;
use player::{
    camera_follow_system, player_collider_resize_system, player_collision_system,
    player_damage_system, player_enemy_contact_damage_system, player_enemy_projectile_hit_system,
    player_game_over_system, player_gravity_system, player_hearts_update_system, player_input_system,
    player_invincibility_blink_system, player_invincibility_system, player_movement_system,
    player_prone_system, player_respawn_system, player_shooting_system,
    player_sprite_offset_system, player_win_pose_system, setup_player, setup_player_hearts_ui,
    track_player_position_system,
};
use soundtrack::SoundtrackPlugin;
use systems::{setup_camera, setup_layer_geometry};
use assets::{
    ForegroundLayer, LevelBackground, ParallaxBackground, load_game_assets,
    parallax_movement_system, position_level_background, setup_level_background,
};
use components::LayerGeometryStorage;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use enemy_a::{
    enemy_robot_animation_system, enemy_robot_behavior_system, enemy_robot_death_anim_system,
    enemy_robot_despawn_system, enemy_robot_hit_system, enemy_robot_projectile_system,
    enemy_spawn_protection_system,
};
use enemy_b::{
    enemy_b_animation_system, enemy_b_behavior_system, enemy_b_death_anim_system,
    enemy_b_explosion_anim_system, enemy_b_grenade_collision_system,
    enemy_b_grenade_movement_system, enemy_b_hit_system,
};
use enemy_c::{
    enemy_c_animation_system, enemy_c_behavior_system, enemy_c_death_system,
    enemy_c_dynamic_spawn_system, enemy_c_hit_system, enemy_c_movement_system,
};
use game_state::GameStatePlugin;
use miniboss::{
    MinibossFireEvent, miniboss_animation_system, miniboss_behavior_system, miniboss_death_system,
    miniboss_grenade_collision_system, miniboss_grenade_fire_system,
    miniboss_grenade_physics_system, miniboss_hit_system, miniboss_movement_system,
    spawn_miniboss_on_phase_start,
};
use player::player_sprite_flip_system::player_sprite_flip_system;
use projectile::projectile_fx_systems::projectile_hit_anim_update_system;
use projectile::projectile_spawning_system::one_shot_lifetime_system;
use projectile::{
    projectile_hit_fx_system, projectile_movement_system, projectile_shoot_fx_flash_system,
    projectile_shoot_fx_projectile_system,
};
use spawn::{
    EdgeSpawnManager, configure_default_spawns, edge_spawn_system, enemy_edge_cleanup_system,
    hangar_enemy_spawn_system,
};

#[wasm_bindgen]
pub fn run_app() {
    main();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn request_stop() {
    STOP_REQUESTED.store(true, Ordering::SeqCst);
}

pub fn main() {
    STOP_REQUESTED.store(false, Ordering::SeqCst);

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Gunday".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    resizable: false,
                    canvas: Some("#bevy-canvas".to_string()),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "assets".to_string(),
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            }),
    );

    #[cfg(target_arch = "wasm32")]
    {
        crate::web_console::init_panic_hook();
    }

    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(LayerGeometryStorage::default())
        .insert_resource(crate::player::components::PlayerActions::default())
        .insert_resource(player::track_player_position_system::PositionTriggerState::default())
        .insert_resource(EdgeSpawnManager::default())
        .insert_resource(enemy_a::InfectedEnemyConfig::default())
        .add_plugins(GameStatePlugin)
        .add_plugins(boss::BossPlugin)
        .add_plugins(SoundtrackPlugin)
        .add_event::<crate::projectile::components::ProjectileHitEvent>()
        .add_event::<MinibossFireEvent>()
        .add_event::<player::PlayerDamagedEvent>()
        .add_systems(
            Startup,
            (
                load_game_assets,
                setup_camera,
                setup_layer_geometry,
                setup_player.after(load_game_assets),
                setup_player_hearts_ui
                    .after(setup_camera)
                    .after(load_game_assets),
                setup_level_background.after(load_game_assets),
                configure_default_spawns.after(setup_layer_geometry),
                setup_audio_emitters.after(load_game_assets),
            ),
        )
        .add_systems(
            PreUpdate,
            player::player_input_system::gather_player_actions,
        )
        .add_systems(
            Update,
            (
                player_gravity_system,
                player_input_system,
                player_movement_system,
            ),
        )
        .add_systems(
            Update,
            track_player_position_system.after(player_movement_system),
        )
        .add_systems(
            Update,
            (
                player_collider_resize_system.after(player_movement_system),
                player_collision_system.after(player_collider_resize_system),
            ),
        )
        .add_systems(
            Update,
            (
                player_enemy_projectile_hit_system.after(projectile_movement_system),
                player_enemy_contact_damage_system
                    .after(player_enemy_projectile_hit_system)
                    .after(enemy_robot_behavior_system)
                    .after(enemy_b_behavior_system)
                    .after(enemy_c_movement_system)
                    .after(miniboss_movement_system),
                player_damage_system.after(player_enemy_contact_damage_system),
                player_prone_system.after(player_damage_system),
                player_game_over_system.after(player_prone_system),
                player_invincibility_system.after(player_damage_system),
                player_respawn_system.after(player_damage_system),
                player_invincibility_blink_system.after(player_invincibility_system),
                player_hearts_update_system.after(player_prone_system),
            ),
        )
        .add_systems(
            Update,
            (
                camera_follow_system,
                player_shooting_system,
                player_jump_anim_system,
                player_run_anim_system,
            ),
        )
        .add_systems(
            Update,
            player_win_pose_system
                .after(player_run_anim_system)
                .after(player_shooting_system)
                .after(player_jump_anim_system),
        )
        .add_systems(
            Update,
            (
                player_sprite_flip_system.after(player_win_pose_system),
                player_sprite_offset_system.after(player_win_pose_system),
            ),
        )
        .add_systems(Update, projectile_movement_system)
        .add_systems(
            Update,
            (
                projectile_shoot_fx_projectile_system.after(player_shooting_system),
                projectile_shoot_fx_flash_system.after(player_shooting_system),
            ),
        )
        .add_systems(
            Update,
            projectile_hit_fx_system
                .after(projectile_movement_system)
                .after(enemy_robot_hit_system)
                .after(enemy_c_hit_system)
                .after(miniboss_hit_system),
        )
        .add_systems(Update, effects::explosion_anim::explosion_anim_system)
        .add_systems(
            Update,
            (projectile_hit_anim_update_system, one_shot_lifetime_system),
        )
        .add_systems(
            Update,
            (
                hangar_enemy_spawn_system,
                edge_spawn_system,
                enemy_c_dynamic_spawn_system.after(edge_spawn_system),
                enemy_spawn_protection_system.after(edge_spawn_system),
                enemy_robot_behavior_system.after(edge_spawn_system),
                enemy_robot_animation_system.after(enemy_robot_behavior_system),
            ),
        )
        .add_systems(
            Update,
            (
                enemy_robot_projectile_system.after(enemy_robot_behavior_system),
                enemy_robot_hit_system.after(projectile_movement_system),
                enemy_a::infected_transform_system.after(enemy_robot_hit_system),
                enemy_robot_death_anim_system.after(enemy_robot_hit_system),
                enemy_robot_despawn_system.after(enemy_robot_death_anim_system),
            ),
        )
        .add_systems(
            Update,
            (
                enemy_b_behavior_system.after(edge_spawn_system),
                enemy_b_animation_system.after(enemy_b_behavior_system),
                enemy_b::grenade_system::enemy_b_grenade_physics_system,
                enemy_b_grenade_movement_system.after(enemy_b_behavior_system),
                enemy_b_grenade_collision_system
                    .after(enemy_b::grenade_system::enemy_b_grenade_physics_system),
                enemy_b_hit_system.after(projectile_movement_system),
                enemy_b_death_anim_system.after(enemy_b_hit_system),
                enemy_b::hit_system::enemy_b_despawn_system.after(enemy_b_death_anim_system),
                enemy_b::hit_system::enemy_b_spawn_protection_system.after(edge_spawn_system),
                enemy_b_explosion_anim_system,
            ),
        )
        .add_systems(
            Update,
            (
                enemy_c_behavior_system.after(edge_spawn_system),
                enemy_c_movement_system.after(enemy_c_behavior_system),
                enemy_c_animation_system.after(enemy_c_movement_system),
            ),
        )
        .add_systems(
            Update,
            (
                enemy_c_hit_system.after(projectile_movement_system),
                enemy_c_death_system.after(enemy_c_hit_system),
            ),
        )
        .add_systems(
            Update,
            enemy_edge_cleanup_system
                .after(enemy_c_movement_system)
                .after(enemy_robot_behavior_system)
                .after(enemy_b_behavior_system),
        )
        .add_systems(
            Update,
            (
                spawn_miniboss_on_phase_start,
                miniboss_behavior_system
                    .after(spawn_miniboss_on_phase_start)
                    .after(edge_spawn_system),
                miniboss_movement_system.after(miniboss_behavior_system),
                miniboss_animation_system.after(miniboss_behavior_system),
                miniboss_grenade_fire_system.after(miniboss_behavior_system),
                miniboss_grenade_physics_system.after(miniboss_grenade_fire_system),
                miniboss_grenade_collision_system.after(miniboss_grenade_physics_system),
                miniboss_hit_system.after(projectile_movement_system),
                miniboss_death_system.after(miniboss_hit_system),
            ),
        )
        .add_systems(
            Update,
            (
                position_level_background,
                parallax_movement_system.after(camera_follow_system),
                pixel_perfect_snap_system.after(camera_follow_system),
                stop_request_system,
            ),
        )
        .run();
}

fn stop_request_system(mut exit: EventWriter<AppExit>) {
    if STOP_REQUESTED.swap(false, Ordering::SeqCst) {
        exit.write(AppExit::Success);
    }
}

fn pixel_perfect_snap_system(
    mut q: Query<
        &mut Transform,
        (
            With<Sprite>,
            Without<Camera>,
            Without<ParallaxBackground>,
            Without<ForegroundLayer>,
            Without<LevelBackground>,
        ),
    >,
) {
    for mut t in q.iter_mut() {
        if t.translation.z >= 0.0 {
            let rx = t.translation.x.round();
            let ry = t.translation.y.round();
            if rx != t.translation.x || ry != t.translation.y {
                t.translation.x = rx;
                t.translation.y = ry;
            }
        }
    }
}
