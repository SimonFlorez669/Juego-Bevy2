use bevy::prelude::*;
use crate::components::goat::{Goat, GridPos, Step};
use crate::components::grass::GridConfig;

/// Mover con WASD/Flechas, acotado a la grilla.
pub fn move_goat(
    kb: Res<ButtonInput<KeyCode>>,
    config: Res<GridConfig>,
    mut q_goat: Query<(&mut GridPos, &mut Transform, &Step), With<Goat>>,
) {
    if let Ok((mut pos, mut tf, step)) = q_goat.get_single_mut() {
        let mut dx = 0;
        let mut dy = 0;
        if kb.just_pressed(KeyCode::ArrowLeft) || kb.just_pressed(KeyCode::KeyA) { dx -= step.0; }
        if kb.just_pressed(KeyCode::ArrowRight) || kb.just_pressed(KeyCode::KeyD) { dx += step.0; }
        if kb.just_pressed(KeyCode::ArrowDown) || kb.just_pressed(KeyCode::KeyS) { dy -= step.0; }
        if kb.just_pressed(KeyCode::ArrowUp) || kb.just_pressed(KeyCode::KeyW) { dy += step.0; }

        if dx != 0 || dy != 0 {
            pos.x = (pos.x + dx).clamp(0, config.cols - 1);
            pos.y = (pos.y + dy).clamp(0, config.rows - 1);
            tf.translation = config.grid_to_world(pos.x, pos.y) + Vec3::Z * 1.0;
        }
    }
}
