use super::components::*;
use super::config::BOSS_SETTINGS;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub fn load_boss_audio(mut commands: Commands, assets: Res<AssetServer>) {
    let hit: Handle<AudioSource> = assets.load("ost/hit.ogg");
    let shot: Handle<AudioSource> = assets.load("ost/boss_shot.ogg");
    let defeat: Handle<AudioSource> = assets.load("ost/boss_defeat.ogg");
    let win: Handle<AudioSource> = assets.load("ost/win.ogg");
    commands.insert_resource(BossAudio {
        hit,
        shot,
        defeat,
        win,
    });
}

pub fn spawn_boss(
    commands: &mut Commands,
    asset_server: &AssetServer,
    translation: Vec3,
) -> Entity {
    let parent_id = commands
        .spawn((
            Boss,
            BossStage(BossStageKind::Stage1),
            BossStage1ShootingState::default(),
            BossStage1MovementState::default(),
            BossMovementTimer {
                timer: 0.0,
                waving_amplitude: BOSS_SETTINGS.initial_waving_amplitude,
            },
            BossStage1State::default(),
            BossCollider {
                half_size: BOSS_SETTINGS.collider_half_size,
            },
            Transform::from_translation(translation),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            BossFacing { right: true },
            BossParts::default(),
        ))
        .id();

    let head: Handle<Image> = asset_server.load("sprites/boss_head.png");
    let torso: Handle<Image> = asset_server.load("sprites/boss_torso.png");
    let spine: Handle<Image> = asset_server.load("sprites/boss_spine.png");
    let gun: Handle<Image> = asset_server.load("sprites/boss_gun.png");
    let left: Handle<Image> = asset_server.load("sprites/boss_left_hand.png");
    let right: Handle<Image> = asset_server.load("sprites/boss_right_hand.png");

    let anchor_from_px = |w: f32, h: f32, x: f32, y: f32| -> Anchor {
        Anchor::Custom(Vec2::new(x / w - 0.5, 0.5 - y / h))
    };
    let offset_from_center =
        |w: f32, h: f32, x: f32, y: f32| -> Vec3 { Vec3::new(x - w * 0.5, h * 0.5 - y, 0.0) };

    let (torso_w, torso_h) = (88.0, 96.0);
    let (head_w, head_h) = (48.0, 42.0);
    let (spine_w, spine_h) = (30.0, 48.0);
    let (gun_w, gun_h) = (176.0, 40.0);
    let (lhand_w, lhand_h) = (72.0, 92.0);
    let (rhand_w, rhand_h) = (68.0, 110.0);

    let torso_t = Transform::from_translation(Vec3::new(0.0, 22.0, 0.0));
    let torso_head = (36.0, 36.0);
    let torso_left = (64.0, 43.0);
    let torso_right = (32.0, 35.0);
    let torso_spine = (44.0, 93.0);

    let head_anchor = anchor_from_px(head_w, head_h, 33.0, 26.0);
    let spine_anchor_on_torso = anchor_from_px(spine_w, spine_h, 11.0, 5.0);
    let gun_anchor = anchor_from_px(gun_w, gun_h, 134.0, 18.0);
    let lhand_anchor = anchor_from_px(lhand_w, lhand_h, 23.0, 15.0);
    let rhand_anchor = anchor_from_px(rhand_w, rhand_h, 59.0, 13.0);

    let head_pos = offset_from_center(torso_w, torso_h, torso_head.0, torso_head.1)
        + Vec3::new(0.0, 0.0, 0.40);
    let left_pos = offset_from_center(torso_w, torso_h, torso_left.0, torso_left.1)
        + Vec3::new(0.0, 0.0, 0.50);
    let right_pos = offset_from_center(torso_w, torso_h, torso_right.0, torso_right.1)
        + Vec3::new(0.0, 0.0, -0.20);
    let spine_pos = offset_from_center(torso_w, torso_h, torso_spine.0, torso_spine.1)
        + Vec3::new(0.0, 0.0, -0.10);

    let spine_to_gun_dx = 15.0 - 11.0;
    let spine_to_gun_dy = 5.0 - 40.0;

    let gun_pos_rel_spine = Vec3::new(spine_to_gun_dx, spine_to_gun_dy, 0.40);

    let mut parts = BossParts::default();
    commands.entity(parent_id).with_children(|parent| {
        let mut torso_ec = parent.spawn((
            Sprite {
                image: torso,
                anchor: Anchor::Center,
                ..Default::default()
            },
            torso_t,
            BossTorso,
        ));

        torso_ec.with_children(|torso_parent| {
            let head_e = torso_parent
                .spawn((
                    Sprite {
                        image: head,
                        anchor: head_anchor,
                        ..Default::default()
                    },
                    Transform::from_translation(head_pos)
                        .with_rotation(Quat::from_rotation_z(
                            BOSS_SETTINGS.head.neutral_angle_deg.to_radians(),
                        )),
                    BossHead,
                ))
                .id();

            let left_e = torso_parent
                .spawn((
                    Sprite {
                        image: left,
                        anchor: lhand_anchor,
                        ..Default::default()
                    },
                    Transform::from_translation(left_pos)
                        .with_rotation(Quat::from_rotation_z(
                            BOSS_SETTINGS.arms.relaxed_left_deg.to_radians(),
                        )),
                    BossArm { is_left: true },
                ))
                .id();

            let right_e = torso_parent
                .spawn((
                    Sprite {
                        image: right,
                        anchor: rhand_anchor,
                        ..Default::default()
                    },
                    Transform::from_translation(right_pos)
                        .with_rotation(Quat::from_rotation_z(
                            BOSS_SETTINGS.arms.relaxed_right_deg.to_radians(),
                        )),
                    BossArm { is_left: false },
                ))
                .id();

            let mut spine_ec = torso_parent.spawn((
                Sprite {
                    image: spine,
                    anchor: spine_anchor_on_torso,
                    ..Default::default()
                },
                Transform::from_translation(spine_pos),
                BossSpine::default(),
            ));

            spine_ec.with_children(|spine_parent| {
                let gun_e = spine_parent
                    .spawn((
                        Sprite {
                            image: gun,
                            anchor: gun_anchor,
                            ..Default::default()
                        },
                        Transform::from_translation(gun_pos_rel_spine)
                            .with_rotation(Quat::from_rotation_z(0.0)),
                        BossCannon,
                        BossGunRotation::default(),
                    ))
                    .id();
                parts.insert(BossPartKind::Cannon, gun_e);
            });

            let spine_e = spine_ec.id();
            parts.insert(BossPartKind::Head, head_e);
            parts.insert(BossPartKind::LeftArm, left_e);
            parts.insert(BossPartKind::RightArm, right_e);
            parts.insert(BossPartKind::Spine, spine_e);
        });

        let torso_e = torso_ec.id();
        parts.insert(BossPartKind::Torso, torso_e);
    });
    commands.entity(parent_id).insert(parts);

    parent_id
}
