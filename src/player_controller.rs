use minifb::Window;
use minifb::Key;
use crate::player::Player;
use std::f32::consts::PI;
use rodio::Sink;

pub fn process_events(
    window: &mut Window,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    walking_sound_sink: &Sink, // Add the Sink for the walking sound effect as a parameter
) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = 0.3;
    const DEAD_ZONE: f32 = 250.0;

    // Mouse movement for rotating the player
    if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let center_x = window.get_size().0 as f32 / 2.0;
        let left_dead_zone = center_x - DEAD_ZONE / 2.0;
        let right_dead_zone = center_x + DEAD_ZONE / 2.0;

        if mouse_x < left_dead_zone {
            player.a -= ROTATION_SPEED * (left_dead_zone - mouse_x) / left_dead_zone;
        } else if mouse_x > right_dead_zone {
            player.a += ROTATION_SPEED * (mouse_x - right_dead_zone) / (window.get_size().0 as f32 - right_dead_zone);
        }
    }

    // Movement with arrow keys
    let forward_x = player.a.cos() * MOVE_SPEED;
    let forward_y = player.a.sin() * MOVE_SPEED;

    let mut new_x = player.pos.x;
    let mut new_y = player.pos.y;

    let mut is_moving = false;

    if window.is_key_down(Key::Up) {
        new_x += forward_x; // Move forward in X
        new_y += forward_y; // Move forward in Y
        is_moving = true;
    }
    if window.is_key_down(Key::Down) {
        new_x -= forward_x; // Move backward in X
        new_y -= forward_y; // Move backward in Y
        is_moving = true;
    }

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED; // Rotate left
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED; // Rotate right
    }

    // Check if the new position collides with a wall
    let i = new_x as usize / block_size;
    let j = new_y as usize / block_size;

    if maze[j][i] == ' ' {
        player.pos.x = new_x;
        player.pos.y = new_y;
    }

    // Handle walking sound effect
    if is_moving {
        if walking_sound_sink.is_paused() {
            walking_sound_sink.play();
        }
    } else {
        if !walking_sound_sink.is_paused() {
            walking_sound_sink.pause();
        }
    }
}
