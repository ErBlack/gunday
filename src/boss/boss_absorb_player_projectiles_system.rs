use super::components::*;
use bevy::prelude::*;
pub fn boss_absorb_player_projectiles_system(
    mut commands: Commands,
    bosses: Query<(&BossStage, &BossParts), With<Boss>>,
    projectiles: Query<
        (Entity, &Transform),
        (
            With<crate::projectile::components::Projectile>,
            With<crate::projectile::components::PlayerProjectile>,
        ),
    >,
) {
    for (stage, _parts) in bosses.iter() {
        let aabbs: Vec<(Vec2, Vec2)> = Vec::new();
        match stage.0 {
            BossStageKind::Stage1 => {}
            BossStageKind::Stage2 => {}
            _ => {}
        }

        'proj: for (proj_e, proj_tf) in projectiles.iter() {
            if aabbs.is_empty() {
                break 'proj;
            }
            let pmin = proj_tf.translation.truncate() - Vec2::splat(5.0);
            for (center, size) in &aabbs {
                let min = *center - *size * 0.5;
                let overlap = pmin.x < min.x + size.x
                    && pmin.x + 10.0 > min.x
                    && pmin.y < min.y + size.y
                    && pmin.y + 10.0 > min.y;
                if overlap {
                    commands.entity(proj_e).despawn();
                    continue 'proj;
                }
            }
        }
    }
}
