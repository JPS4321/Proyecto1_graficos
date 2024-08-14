use minifb::Window;
use minifb::Key;
use crate::player::Player;
use std::f32::consts::PI;

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = PI / 30.0; // Cambia este valor para ajustar la velocidad de rotación

    // Rotación con flechas izquierda/derecha
    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED; // Rotar a la izquierda
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED; // Rotar a la derecha
    }

    // Movimiento con WASD
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

    // Verificamos si la nueva posición no colisiona con una pared
    let i = new_x as usize / block_size;
    let j = new_y as usize / block_size;

    if maze[j][i] == ' ' {
        // Si no hay colisión, actualizamos la posición del jugador
        player.pos.x = new_x;
        player.pos.y = new_y;
    }
}
