use bevy::prelude::*;
use crate::components::goat::{Goat, GridPos, Step};
use crate::components::grass::GridConfig;

pub fn spawn_goat(mut commands: Commands, config: Res<GridConfig>) {
    let start = GridPos { x: 1, y: 1 };
    commands.spawn((
        Goat,
        start,
        Step::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.9, 0.9, 0.95),
                custom_size: Some(Vec2::splat(config.tile_size * 0.6)),
                ..default()
            },
            transform: Transform::from_translation(
                config.grid_to_world(start.x, start.y) + Vec3::Z * 1.0
            ),
            ..default()
        },
    ));
}
