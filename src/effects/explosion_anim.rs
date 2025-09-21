use crate::assets::GameAssets;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ExplosionKind {
    A,
    B,
    C,
    D,
}

#[derive(Component)]
pub struct Explosion {
    pub kind: ExplosionKind,
    pub timer: f32,
    pub frame: usize,
}

impl Explosion {
    pub fn new(kind: ExplosionKind) -> Self {
        Self {
            kind,
            timer: 0.0,
            frame: 0,
        }
    }
}

const FRAME_TIME: f32 = 0.06;

pub fn explosion_anim_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut q: Query<(Entity, &mut Sprite, &mut Explosion)>,
) {
    for (e, mut sprite, mut exp) in q.iter_mut() {
        exp.timer += time.delta_secs();
        if exp.timer < FRAME_TIME {
            continue;
        }
        exp.timer = 0.0;
        exp.frame += 1;
        match exp.kind {
            ExplosionKind::A => match exp.frame {
                1 => sprite.image = assets.explosion_a_a.clone(),
                2 => sprite.image = assets.explosion_a_b.clone(),
                3 => sprite.image = assets.explosion_a_c.clone(),
                4 => sprite.image = assets.explosion_a_d.clone(),
                5 => sprite.image = assets.explosion_a_e.clone(),
                6 => sprite.image = assets.explosion_a_f.clone(),
                7 => sprite.image = assets.explosion_a_g.clone(),
                _ => {
                    commands.entity(e).despawn();
                }
            },
            ExplosionKind::B => match exp.frame {
                1 => sprite.image = assets.explosion_b_a.clone(),
                2 => sprite.image = assets.explosion_b_b.clone(),
                3 => sprite.image = assets.explosion_b_c.clone(),
                4 => sprite.image = assets.explosion_b_d.clone(),
                5 => sprite.image = assets.explosion_b_e.clone(),
                6 => sprite.image = assets.explosion_b_f.clone(),
                7 => sprite.image = assets.explosion_b_g.clone(),
                _ => {
                    commands.entity(e).despawn();
                }
            },
            ExplosionKind::C => match exp.frame {
                1 => sprite.image = assets.explosion_c_a.clone(),
                2 => sprite.image = assets.explosion_c_b.clone(),
                3 => sprite.image = assets.explosion_c_c.clone(),
                4 => sprite.image = assets.explosion_c_d.clone(),
                5 => sprite.image = assets.explosion_c_e.clone(),
                6 => sprite.image = assets.explosion_c_f.clone(),
                7 => sprite.image = assets.explosion_c_g.clone(),
                8 => sprite.image = assets.explosion_c_h.clone(),
                9 => sprite.image = assets.explosion_c_i.clone(),
                10 => sprite.image = assets.explosion_c_j.clone(),
                _ => {
                    commands.entity(e).despawn();
                }
            },
            ExplosionKind::D => match exp.frame {
                1 => sprite.image = assets.explosion_d_a.clone(),
                2 => sprite.image = assets.explosion_d_b.clone(),
                3 => sprite.image = assets.explosion_d_c.clone(),
                4 => sprite.image = assets.explosion_d_d.clone(),
                5 => sprite.image = assets.explosion_d_e.clone(),
                6 => sprite.image = assets.explosion_d_f.clone(),
                7 => sprite.image = assets.explosion_d_g.clone(),
                8 => sprite.image = assets.explosion_d_h.clone(),
                9 => sprite.image = assets.explosion_d_i.clone(),
                10 => sprite.image = assets.explosion_d_j.clone(),
                11 => sprite.image = assets.explosion_d_k.clone(),
                12 => sprite.image = assets.explosion_d_l.clone(),
                13 => sprite.image = assets.explosion_d_m.clone(),
                14 => sprite.image = assets.explosion_d_n.clone(),
                15 => sprite.image = assets.explosion_d_o.clone(),
                16 => sprite.image = assets.explosion_d_p.clone(),
                _ => {
                    commands.entity(e).despawn();
                }
            },
        }
    }
}

pub fn spawn_explosion_c(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    commands.spawn((
        Sprite::from_image(assets.explosion_c_a.clone()),
        Transform::from_translation(pos),
        Explosion::new(ExplosionKind::C),
    ));
}
pub fn spawn_explosion_b(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    commands.spawn((
        Sprite::from_image(assets.explosion_b_a.clone()),
        Transform::from_translation(pos),
        Explosion::new(ExplosionKind::B),
    ));
}
pub fn spawn_explosion_d(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    commands.spawn((
        Sprite::from_image(assets.explosion_d_a.clone()),
        Transform::from_translation(pos),
        Explosion::new(ExplosionKind::D),
    ));
}
pub fn spawn_explosion_a(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    commands.spawn((
        Sprite::from_image(assets.explosion_a_a.clone()),
        Transform::from_translation(pos),
        Explosion::new(ExplosionKind::A),
    ));
}
