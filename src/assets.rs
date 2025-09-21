use crate::constants::{CAMERA_OFFSET, WORLD_WIDTH, Z_FOREGROUND, Z_LEVEL, Z_PARALLAX_BACKGROUND};
use bevy::audio::AudioSource;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub level: Handle<Image>,
    pub parallax_background: Handle<Image>,
    pub foreground: Handle<Image>,
    pub player_static: Handle<Image>,
    pub player_shooting: Handle<Image>,
    pub player_run_a: Handle<Image>,
    pub player_run_b: Handle<Image>,
    pub player_run_c: Handle<Image>,
    pub player_run_d: Handle<Image>,
    pub player_jump: Handle<Image>,
    pub player_up: Handle<Image>,
    pub player_win: Handle<Image>,
    pub player_hit: Handle<Image>,
    pub player_fall: Handle<Image>,
    pub ui_heart: Handle<Image>,
    pub player_run_up_a: Handle<Image>,
    pub player_run_up_b: Handle<Image>,
    pub player_run_up_c: Handle<Image>,
    pub player_run_up_d: Handle<Image>,
    pub player_run_down_a: Handle<Image>,
    pub player_run_down_b: Handle<Image>,
    pub player_run_down_c: Handle<Image>,
    pub player_run_down_d: Handle<Image>,
    pub enemy_a_run_a: Handle<Image>,
    pub enemy_a_run_b: Handle<Image>,
    pub enemy_a_run_c: Handle<Image>,
    pub enemy_a_run_d: Handle<Image>,
    pub enemy_a_shoot: Handle<Image>,
    pub enemy_a_projectile: Handle<Image>,
    pub enemy_a_hit: Handle<Image>,
    pub enemy_b_sit: Handle<Image>,
    pub enemy_b_fire_a: Handle<Image>,
    pub enemy_b_fire_b: Handle<Image>,
    pub enemy_b_hit: Handle<Image>,
    pub enemy_b_grenade: Handle<Image>,
    pub enemy_c_run_a: Handle<Image>,
    pub enemy_c_run_b: Handle<Image>,
    pub enemy_c_run_c: Handle<Image>,
    pub enemy_c_jump: Handle<Image>,
    pub enemy_c_hit: Handle<Image>,
    pub enemy_c_transform_a: Handle<Image>,
    pub enemy_c_transform_b: Handle<Image>,
    pub enemy_c_transform_c: Handle<Image>,
    pub enemy_c_transform_d: Handle<Image>,
    pub miniboss_move_a: Handle<Image>,
    pub miniboss_move_b: Handle<Image>,
    pub miniboss_shoot_a: Handle<Image>,
    pub miniboss_shoot_b: Handle<Image>,
    pub miniboss_dead: Handle<Image>,
    pub explosion_a_a: Handle<Image>,
    pub explosion_a_b: Handle<Image>,
    pub explosion_a_c: Handle<Image>,
    pub explosion_a_d: Handle<Image>,
    pub explosion_a_e: Handle<Image>,
    pub explosion_a_f: Handle<Image>,
    pub explosion_a_g: Handle<Image>,
    pub explosion_b_a: Handle<Image>,
    pub explosion_b_b: Handle<Image>,
    pub explosion_b_c: Handle<Image>,
    pub explosion_b_d: Handle<Image>,
    pub explosion_b_e: Handle<Image>,
    pub explosion_b_f: Handle<Image>,
    pub explosion_b_g: Handle<Image>,
    pub explosion_c_a: Handle<Image>,
    pub explosion_c_b: Handle<Image>,
    pub explosion_c_c: Handle<Image>,
    pub explosion_c_d: Handle<Image>,
    pub explosion_c_e: Handle<Image>,
    pub explosion_c_f: Handle<Image>,
    pub explosion_c_g: Handle<Image>,
    pub explosion_c_h: Handle<Image>,
    pub explosion_c_i: Handle<Image>,
    pub explosion_c_j: Handle<Image>,
    pub explosion_d_a: Handle<Image>,
    pub explosion_d_b: Handle<Image>,
    pub explosion_d_c: Handle<Image>,
    pub explosion_d_d: Handle<Image>,
    pub explosion_d_e: Handle<Image>,
    pub explosion_d_f: Handle<Image>,
    pub explosion_d_g: Handle<Image>,
    pub explosion_d_h: Handle<Image>,
    pub explosion_d_i: Handle<Image>,
    pub explosion_d_j: Handle<Image>,
    pub explosion_d_k: Handle<Image>,
    pub explosion_d_l: Handle<Image>,
    pub explosion_d_m: Handle<Image>,
    pub explosion_d_n: Handle<Image>,
    pub explosion_d_o: Handle<Image>,
    pub explosion_d_p: Handle<Image>,
    pub player_projectile: Handle<Image>,
    pub player_projectile_hit_a: Handle<Image>,
    pub player_projectile_hit_b: Handle<Image>,
    pub player_projectile_hit_c: Handle<Image>,
    pub player_shoot_flash: Handle<Image>,
    pub shoot_sfx: Handle<AudioSource>,
    pub player_hit_sfx: Handle<AudioSource>,
    pub player_game_over_sfx: Handle<AudioSource>,
    pub enemy_shoot_sfx: Handle<AudioSource>,
    pub enemy_hit_sfx: Handle<AudioSource>,
    pub enemy_death_sfx: Handle<AudioSource>,
    pub enemy_c_death_sfx: Handle<AudioSource>,
    pub enemy_transform_sfx: Handle<AudioSource>,
    pub boss_projectile: Handle<Image>,
    pub enemy_explosion_sfx: Handle<AudioSource>,
    pub miniboss_explosion_sfx: Handle<AudioSource>,
}

pub fn load_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_assets = GameAssets {
        level: asset_server.load("levels/level.png"),
        parallax_background: asset_server.load("levels/background.png"),
        foreground: asset_server.load("levels/foreground.png"),
        player_static: asset_server.load("sprites/player_static.png"),
        player_shooting: asset_server.load("sprites/player_shooting.png"),
        player_run_a: asset_server.load("sprites/player_run_a.png"),
        player_run_b: asset_server.load("sprites/player_run_b.png"),
        player_run_c: asset_server.load("sprites/player_run_c.png"),
        player_run_d: asset_server.load("sprites/player_run_d.png"),
        player_jump: asset_server.load("sprites/player_jump.png"),
        player_up: asset_server.load("sprites/player_up.png"),
        player_win: asset_server.load("sprites/player_win.png"),
        player_hit: asset_server.load("sprites/player_hit.png"),
        player_fall: asset_server.load("sprites/player_fall.png"),
        ui_heart: asset_server.load("ui/heart.png"),
        player_run_up_a: asset_server.load("sprites/player_run_up_a.png"),
        player_run_up_b: asset_server.load("sprites/player_run_up_b.png"),
        player_run_up_c: asset_server.load("sprites/player_run_up_c.png"),
        player_run_up_d: asset_server.load("sprites/player_run_up_d.png"),
        player_run_down_a: asset_server.load("sprites/player_run_down_a.png"),
        player_run_down_b: asset_server.load("sprites/player_run_down_b.png"),
        player_run_down_c: asset_server.load("sprites/player_run_down_c.png"),
        player_run_down_d: asset_server.load("sprites/player_run_down_d.png"),
        enemy_a_run_a: asset_server.load("sprites/enemy_a_run_a.png"),
        enemy_a_run_b: asset_server.load("sprites/enemy_a_run_b.png"),
        enemy_a_run_c: asset_server.load("sprites/enemy_a_run_c.png"),
        enemy_a_run_d: asset_server.load("sprites/enemy_a_run_d.png"),
        enemy_a_shoot: asset_server.load("sprites/enemy_a_shoot.png"),
        enemy_a_projectile: asset_server.load("sprites/enemy_a_projectile.png"),
        enemy_a_hit: asset_server.load("sprites/enemy_a_hit.png"),
        enemy_b_sit: asset_server.load("sprites/enemy_b_sit.png"),
        enemy_b_fire_a: asset_server.load("sprites/enemy_b_fire_a.png"),
        enemy_b_fire_b: asset_server.load("sprites/enemy_b_fire_b.png"),
        enemy_b_hit: asset_server.load("sprites/enemy_b_hit.png"),
        enemy_b_grenade: asset_server.load("sprites/enemy_b_projectile.png"),
        enemy_c_run_a: asset_server.load("sprites/enemy_c_run_a.png"),
        enemy_c_run_b: asset_server.load("sprites/enemy_c_run_b.png"),
        enemy_c_run_c: asset_server.load("sprites/enemy_c_run_c.png"),
        enemy_c_jump: asset_server.load("sprites/enemy_c_jump.png"),
        enemy_c_hit: asset_server.load("sprites/enemy_c_hit.png"),
        enemy_c_transform_a: asset_server.load("sprites/enemy_c_transform_a.png"),
        enemy_c_transform_b: asset_server.load("sprites/enemy_c_transform_b.png"),
        enemy_c_transform_c: asset_server.load("sprites/enemy_c_transform_c.png"),
        enemy_c_transform_d: asset_server.load("sprites/enemy_c_transform_d.png"),
        miniboss_move_a: asset_server.load("sprites/miniboss_move_a.png"),
        miniboss_move_b: asset_server.load("sprites/miniboss_move_b.png"),
        miniboss_shoot_a: asset_server.load("sprites/miniboss_shoot_a.png"),
        miniboss_shoot_b: asset_server.load("sprites/miniboss_shoot_b.png"),
        miniboss_dead: asset_server.load("sprites/miniboss_dead.png"),
        explosion_a_a: asset_server.load("sprites/explosion_a_a.png"),
        explosion_a_b: asset_server.load("sprites/explosion_a_b.png"),
        explosion_a_c: asset_server.load("sprites/explosion_a_c.png"),
        explosion_a_d: asset_server.load("sprites/explosion_a_d.png"),
        explosion_a_e: asset_server.load("sprites/explosion_a_e.png"),
        explosion_a_f: asset_server.load("sprites/explosion_a_f.png"),
        explosion_a_g: asset_server.load("sprites/explosion_a_g.png"),
        explosion_b_a: asset_server.load("sprites/explosion_b_a.png"),
        explosion_b_b: asset_server.load("sprites/explosion_b_b.png"),
        explosion_b_c: asset_server.load("sprites/explosion_b_c.png"),
        explosion_b_d: asset_server.load("sprites/explosion_b_d.png"),
        explosion_b_e: asset_server.load("sprites/explosion_b_e.png"),
        explosion_b_f: asset_server.load("sprites/explosion_b_f.png"),
        explosion_b_g: asset_server.load("sprites/explosion_b_g.png"),
        explosion_c_a: asset_server.load("sprites/explosion_c_a.png"),
        explosion_c_b: asset_server.load("sprites/explosion_c_b.png"),
        explosion_c_c: asset_server.load("sprites/explosion_c_c.png"),
        explosion_c_d: asset_server.load("sprites/explosion_c_d.png"),
        explosion_c_e: asset_server.load("sprites/explosion_c_e.png"),
        explosion_c_f: asset_server.load("sprites/explosion_c_f.png"),
        explosion_c_g: asset_server.load("sprites/explosion_c_g.png"),
        explosion_c_h: asset_server.load("sprites/explosion_c_h.png"),
        explosion_c_i: asset_server.load("sprites/explosion_c_i.png"),
        explosion_c_j: asset_server.load("sprites/explosion_c_j.png"),
        explosion_d_a: asset_server.load("sprites/explosion_d_a.png"),
        explosion_d_b: asset_server.load("sprites/explosion_d_b.png"),
        explosion_d_c: asset_server.load("sprites/explosion_d_c.png"),
        explosion_d_d: asset_server.load("sprites/explosion_d_d.png"),
        explosion_d_e: asset_server.load("sprites/explosion_d_e.png"),
        explosion_d_f: asset_server.load("sprites/explosion_d_f.png"),
        explosion_d_g: asset_server.load("sprites/explosion_d_g.png"),
        explosion_d_h: asset_server.load("sprites/explosion_d_h.png"),
        explosion_d_i: asset_server.load("sprites/explosion_d_i.png"),
        explosion_d_j: asset_server.load("sprites/explosion_d_j.png"),
        explosion_d_k: asset_server.load("sprites/explosion_d_k.png"),
        explosion_d_l: asset_server.load("sprites/explosion_d_l.png"),
        explosion_d_m: asset_server.load("sprites/explosion_d_m.png"),
        explosion_d_n: asset_server.load("sprites/explosion_d_n.png"),
        explosion_d_o: asset_server.load("sprites/explosion_d_o.png"),
        explosion_d_p: asset_server.load("sprites/explosion_d_p.png"),
        player_projectile: asset_server.load("sprites/player_projectile.png"),
        player_projectile_hit_a: asset_server.load("sprites/player_projectile_hit_a.png"),
        player_projectile_hit_b: asset_server.load("sprites/player_projectile_hit_b.png"),
        player_projectile_hit_c: asset_server.load("sprites/player_projectile_hit_c.png"),
        player_shoot_flash: asset_server.load("sprites/player_shoot.png"),
        shoot_sfx: asset_server.load("ost/shoot.ogg"),
        player_hit_sfx: asset_server.load("ost/player_hit.ogg"),
    player_game_over_sfx: asset_server.load("ost/game_over.ogg"),
        enemy_shoot_sfx: asset_server.load("ost/enemy_gun_1.ogg"),
        enemy_hit_sfx: asset_server.load("ost/hit.ogg"),
        enemy_death_sfx: asset_server.load("ost/enemy_death.ogg"),
        enemy_c_death_sfx: asset_server.load("ost/enemy_c_death.ogg"),
        enemy_transform_sfx: asset_server.load("ost/transform.ogg"),
        boss_projectile: asset_server.load("sprites/boss_projectile.png"),
        enemy_explosion_sfx: asset_server.load("ost/enemy_explosion.ogg"),
        miniboss_explosion_sfx: asset_server.load("ost/miniboss_explosion.ogg"),
    };
    commands.insert_resource(game_assets);
}

#[derive(Component)]
pub struct LevelBackground;

#[derive(Component)]
pub struct ParallaxBackground;

#[derive(Component)]
pub struct ForegroundLayer;

pub fn setup_level_background(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.level.clone()),
        Transform::from_xyz(0.0, 0.0, Z_LEVEL),
        LevelBackground,
    ));

    commands.spawn((
        Sprite::from_image(game_assets.parallax_background.clone()),
        Transform::from_xyz(0.0, 0.0, Z_PARALLAX_BACKGROUND),
        ParallaxBackground,
    ));

    commands.spawn((
        Sprite::from_image(game_assets.foreground.clone()),
        Transform::from_xyz(0.0, 0.0, Z_FOREGROUND),
        ForegroundLayer,
    ));
}

pub fn position_level_background(
    mut background_query: Query<&mut Transform, With<LevelBackground>>,
    mut foreground_query: Query<&mut Transform, (With<ForegroundLayer>, Without<LevelBackground>)>,
    game_assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
) {
    if let Some(image) = images.get(&game_assets.level) {
        let image_width = image.width() as f32;
        let background_x = image_width / 2.0;

        for mut transform in background_query.iter_mut() {
            transform.translation.x = background_x;
            transform.translation.y = 0.0;
            transform.translation.z = Z_LEVEL;
        }

        for mut transform in foreground_query.iter_mut() {
            transform.translation.x = background_x;
            transform.translation.y = 0.0;
        }
    }
}

pub fn parallax_movement_system(
    mut parallax_query: Query<&mut Transform, With<ParallaxBackground>>,
    camera_query: Query<&Transform, (With<Camera>, Without<ParallaxBackground>)>,
    game_assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
    mut last_cam_x: Local<Option<f32>>,
) {
    if let Ok(camera_transform) = camera_query.single() {
        if let Some(background_image) = images.get(&game_assets.parallax_background) {
            let camera_x = camera_transform.translation.x;

            let cam_changed = match *last_cam_x {
                Some(prev) => (camera_x - prev).abs() >= 1.0,
                None => true,
            };
            if !cam_changed {
                return;
            }

            for mut transform in parallax_query.iter_mut() {
                let background_width = background_image.width() as f32;
                let start = background_width / 2.0 - CAMERA_OFFSET * 2.0;
                let end = WORLD_WIDTH - background_width / 2.0 - CAMERA_OFFSET;

                let level_movement_progress = camera_x / WORLD_WIDTH;
                let new_x = start + level_movement_progress * end;
                if (transform.translation.x - new_x).abs() >= 0.5 {
                    transform.translation.x = new_x;
                }
                if transform.translation.y != 0.0 {
                    transform.translation.y = 0.0;
                }
                if transform.translation.z != Z_PARALLAX_BACKGROUND {
                    transform.translation.z = Z_PARALLAX_BACKGROUND;
                }
            }

            *last_cam_x = Some(camera_x);
        }
    }
}
