use image::GenericImageView;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;

mod framebuffer;
mod maze;
mod player;
mod caster;
mod player_controller;

use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::Player;
use crate::caster::cast_ray;
use crate::player_controller::process_events;

// Función para cargar una textura desde un archivo PNG
fn load_texture(path: &str) -> (Vec<u32>, usize, usize) {
    let img = image::open(path).expect("Failed to load texture");
    let (width, height) = img.dimensions();
    let data = img.to_rgba8().into_raw();
    let texture: Vec<u32> = data.chunks(4).map(|p| {
        let r = p[0] as u32;
        let g = p[1] as u32;
        let b = p[2] as u32;
        let a = p[3] as u32;
        (r << 24) | (g << 16) | (b << 8) | a
    }).collect();
    (texture, width as usize, height as usize)
}

// Función para renderizar en modo 2D con texturas
fn render_2d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    texture: &Vec<u32>,
    texture_width: usize,
    texture_height: usize
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell != ' ' {
                for y in 0..block_size {
                    for x in 0..block_size {
                        let tx = (x * texture_width) / block_size;
                        let ty = (y * texture_height) / block_size;
                        let color = texture[ty * texture_width + tx];
                        framebuffer.set_current_color(color);
                        framebuffer.point(col_index * block_size + x, row_index * block_size + y);
                    }
                }
            }
        }
    }

    // Dibuja al jugador usando su posición en píxeles
    framebuffer.set_current_color(0xFFDDD);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);
}


// Función para renderizar en modo 3D con texturas
fn render3d(framebuffer: &mut Framebuffer, player: &Player, texture: &Vec<u32>, texture_width: usize, texture_height: usize) {
    let maze = load_maze("./maze.txt");
    let block_size = 50;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    // Renderizar el techo
    framebuffer.set_current_color(0x87CEEB); // Color del techo
    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    // Renderizar el suelo
    framebuffer.set_current_color(0x333355); // Color del suelo
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    // Renderizar las paredes en 3D usando raycasting
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);
        let distance = intersect.distance;
    
        // Proyección de la pared en 3D
        let wall_height = (block_size as f32 / distance) * 200.0;
    
        let y0 = hh - (wall_height / 2.0);
        let y1 = hh + (wall_height / 2.0);
    
        // Asegurarse de que texture_x esté dentro de los límites
        let texture_x = ((intersect.impact as u8 as f32 / block_size as f32) * texture_width as f32).clamp(0.0, (texture_width - 1) as f32) as usize;
    
        for y in y0 as usize..y1 as usize {
            // Asegurarse de que texture_y esté dentro de los límites
            let texture_y = (((y - y0 as usize) * texture_height) / wall_height as usize).clamp(0, texture_height - 1);
            let color = texture[texture_y * texture_width + texture_x];
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
    
}

fn main() {
    let window_width = 900;
    let window_height = 600;
    let framebuffer_width = 900;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Proyecto 1",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let maze = load_maze("./maze.txt");
    let block_size = 50;

    framebuffer.set_background_color(0x333355);

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0
    };

    let mut mode = "2D"; // Modo inicial

    // Cargar la textura
    let (texture, texture_width, texture_height) = load_texture("./bricks.png");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();

        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        process_events(&window, &mut player); // Procesar eventos de teclado

        if mode == "2D" {
            render_2d(&mut framebuffer, &player, &maze, block_size, &texture, texture_width, texture_height); // Renderizado 2D
        } else {
            render3d(&mut framebuffer, &player, &texture, texture_width, texture_height); // Renderizado 3D
        }

        framebuffer.draw_fps(750, 10); // Ubicación aproximada en la parte superior derecha


        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
