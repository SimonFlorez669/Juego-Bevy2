use bevy::prelude::*;
use crate::components::grass::{Grass, GridConfig};

/// Sincroniza color del sprite con el estado del pasto.
pub fn sync_colors(config: Res<GridConfig>, mut q: Query<(&Grass, &mut Sprite)>) {
    for (g, mut sprite) in q.iter_mut() {
        sprite.color = if g.eaten { config.dirt_color } else { config.grass_color };
    }
}
