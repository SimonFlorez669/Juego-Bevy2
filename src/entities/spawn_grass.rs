use bevy::prelude::*;
use crate::components::grass::{Cell, Grass, GridConfig};

pub fn spawn_grass_grid(mut commands: Commands, config: Res<GridConfig>) {
    for y in 0..config.rows {
        for x in 0..config.cols {
            commands.spawn((
                Grass::default(),
                Cell { x, y },
                SpriteBundle {
                    sprite: Sprite {
                        color: config.grass_color,
                        custom_size: Some(Vec2::splat(config.tile_size - 4.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(config.grid_to_world(x, y)),
                    ..default()
                },
            ));
        }
    }
}
