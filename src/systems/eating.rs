use bevy::prelude::*;
use crate::components::goat::GridPos;
use crate::components::grass::{Cell, Grass};

/// Presiona E para comer el bloque de pasto más cercano (verde -> café).
/// Estrategia:
/// 1) Con p0 (solo lectura) buscamos el Entity objetivo.
/// 2) Con p1 (solo escritura) pedimos ese Entity específico y lo actualizamos.
/// Así evitamos iteraciones mutables sobre todo el conjunto y no hay choques.
pub fn eat_nearest_grass(
    kb: Res<ButtonInput<KeyCode>>,
    q_goat: Query<&GridPos>,
    mut set: ParamSet<(
        Query<(Entity, &Cell, &Grass)>, // p0: SOLO LECTURA (trae también Entity)
        Query<&mut Grass>,              // p1: ESCRITURA (por Entity puntual)
    )>,
) {
    if !kb.just_pressed(KeyCode::KeyE) {
        return;
    }
    let Ok(goat_pos) = q_goat.get_single() else { return; };

    // -------- PASO 1: buscar el Entity de la celda más cercana (solo lectura) --------
    let mut best: Option<(Entity, i32)> = None;
    for (ent, cell, grass) in set.p0().iter() {
        if grass.eaten {
            continue;
        }
        let dist = (goat_pos.x - cell.x).abs() + (goat_pos.y - cell.y).abs();
        match best {
            None => best = Some((ent, dist)),
            Some((_, d)) if dist < d => best = Some((ent, dist)),
            _ => {}
        }
    }

    // -------- PASO 2: marcar como comido (solo ese Entity) --------
    if let Some((target_ent, _)) = best {
        if let Ok(mut g) = set.p1().get_mut(target_ent) {
            if !g.eaten {
                g.eaten = true; // no vuelve a crecer
            }
        }
    }
}
