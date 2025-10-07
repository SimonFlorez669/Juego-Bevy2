use bevy::prelude::*;

mod components;
mod entities;
mod systems;

use components::grass::GridConfig;
use entities::{spawn_goat::spawn_goat, spawn_grass::spawn_grass_grid};
use systems::{camera::setup_camera, coloring::sync_colors, eating::eat_nearest_grass, movement::move_goat};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.12, 0.12, 0.12)))
        .insert_resource(GridConfig {
            cols: 6,
            rows: 6,
            tile_size: 64.0,
            grass_color: Color::srgb(0.12, 0.6, 0.12), // verde
            dirt_color: Color::srgb(0.41, 0.29, 0.18), // café
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cabra & Pasto (Bevy)".into(),
                resolution: (900., 800.).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, spawn_grass_grid, spawn_goat))
        .add_systems(Update, (move_goat, eat_nearest_grass, sync_colors))
        .run();
}
