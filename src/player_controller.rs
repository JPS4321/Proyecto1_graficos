use minifb::Window;
use minifb::Key;
use crate::player::Player;
use std::f32::consts::PI;

pub fn process_events(window: &mut Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = 0.3; // Ajusta la sensibilidad de la rotación aquí
    const DEAD_ZONE: f32 = 250.0; // Define el tamaño de la zona muerta en el centro en píxeles

    // Movimiento del mouse para rotar al jugador
    if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let center_x = window.get_size().0 as f32 / 2.0;
        let left_dead_zone = center_x - DEAD_ZONE / 2.0;
        let right_dead_zone = center_x + DEAD_ZONE / 2.0;

        if mouse_x < left_dead_zone {
            // Si el mouse está a la izquierda de la zona muerta, rotar a la izquierda
            player.a -= ROTATION_SPEED * (left_dead_zone - mouse_x) / left_dead_zone;
        } else if mouse_x > right_dead_zone {
            // Si el mouse está a la derecha de la zona muerta, rotar a la derecha
            player.a += ROTATION_SPEED * (mouse_x - right_dead_zone) / (window.get_size().0 as f32 - right_dead_zone);
        }
    }

    // Movimiento con las teclas WASD
    let forward_x = player.a.cos() * MOVE_SPEED;
    let forward_y = player.a.sin() * MOVE_SPEED;

    let mut new_x = player.pos.x;
    let mut new_y = player.pos.y;

    if window.is_key_down(Key::Up) {
        new_x += forward_x; // Mover hacia adelante en X
        new_y += forward_y; // Mover hacia adelante en Y
    }
    if window.is_key_down(Key::Down) {
        new_x -= forward_x; // Mover hacia atrás en X
        new_y -= forward_y; // Mover hacia atrás en Y
    }

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED; // Rotar a la izquierda
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED; // Rotar a la derecha
    }
    // Verificar si la nueva posición no colisiona con una pared
    let i = new_x as usize / block_size;
    let j = new_y as usize / block_size;

    if maze[j][i] == ' ' {
        // Si no hay colisión, actualizar la posición del jugador
        player.pos.x = new_x;
        player.pos.y = new_y;
    }
}

