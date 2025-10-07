use bevy::prelude::*;

/// Celda de pasto
#[derive(Component)]
pub struct Grass { pub eaten: bool }

impl Default for Grass { fn default() -> Self { Self { eaten: false } } }

/// Coordenadas de celda
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell { pub x: i32, pub y: i32 }

/// Configuración de grilla/colores
#[derive(Resource, Clone)]
pub struct GridConfig {
    pub cols: i32,
    pub rows: i32,
    pub tile_size: f32,
    pub grass_color: Color,
    pub dirt_color: Color,
}

impl GridConfig {
    pub fn grid_to_world(&self, x: i32, y: i32) -> Vec3 {
        let w = self.cols as f32 * self.tile_size;
        let h = self.rows as f32 * self.tile_size;
        let origin = Vec2::new(-w / 2.0 + self.tile_size / 2.0, -h / 2.0 + self.tile_size / 2.0);
        Vec3::new(origin.x + x as f32 * self.tile_size, origin.y + y as f32 * self.tile_size, 0.0)
    }
}
