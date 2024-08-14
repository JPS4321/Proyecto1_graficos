use minifb::Window;
use minifb::Key;
use crate::player::Player;
use std::f32::consts::PI;

pub fn process_events(window: &Window, player: &mut Player) {
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

    if window.is_key_down(Key::Up) {
        player.pos.x += forward_x; // Mover hacia adelante en X
        player.pos.y += forward_y; // Mover hacia adelante en Y
    }
    if window.is_key_down(Key::Down) {
        player.pos.x -= forward_x; // Mover hacia atrás en X
        player.pos.y -= forward_y; // Mover hacia atrás en Y
    }
}
