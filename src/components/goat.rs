use bevy::prelude::*;

#[derive(Component)]
pub struct Goat;

/// Posición discreta en grilla
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub struct GridPos { pub x: i32, pub y: i32 }

/// Celdas que avanza por paso
#[derive(Component)]
pub struct Step(pub i32);

impl Default for Step { fn default() -> Self { Step(1) } }
