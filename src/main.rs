use image::GenericImageView;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

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

fn load_texture(path: &str) -> (Vec<u32>, usize, usize) {
    let img = image::open(path).expect("Failed to load texture");
    let (width, height) = img.dimensions();
    let data = img.to_rgba8().into_raw();
    let texture: Vec<u32> = data.chunks(4).map(|p| {
        let r = p[0] as u32;
        let g = p[1] as u32;
        let b = p[2] as u32;
        let a = p[3] as u32;
        (a << 24) | (r << 16) | (g << 8) | b
    }).collect();
    (texture, width as usize, height as usize)
}

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

    framebuffer.set_current_color(0xFFDDD);

    // Tamaño del "punto" que representa al jugador
    let player_size = 5;

    // Dibujar un cuadrado en torno a la posición del jugador
    for y in (player.pos.y as usize).saturating_sub(player_size)..=(player.pos.y as usize + player_size) {
        for x in (player.pos.x as usize).saturating_sub(player_size)..=(player.pos.x as usize + player_size) {
            framebuffer.point(x, y);
        }
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, texture: &Vec<u32>, texture_width: usize, texture_height: usize) {
    let maze = load_maze("./maze.txt");
    let block_size = 55;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(0x3f3d4d);
    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    framebuffer.set_current_color(0x333355);
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);
        let distance = intersect.distance;
    
        let wall_height = (block_size as f32 / distance) * 200.0;
    
        let y0 = hh - (wall_height / 2.0);
        let y1 = hh + (wall_height / 2.0);
    
        let wall_x = if intersect.impact == '|' {
            intersect.impact_pos.1 % block_size as f32
        } else {
            intersect.impact_pos.0 % block_size as f32
        };

        let texture_x = ((wall_x / block_size as f32) * texture_width as f32).clamp(0.0, (texture_width - 1) as f32) as usize;
    
        for y in y0 as usize..y1 as usize {
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

    // Configuración de la reproducción de audio WAV
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    // Crear los archivos de audio y sus respectivos sinks (controladores de audio)
    let file1 = BufReader::new(File::open("./Assets/maintheme.wav").unwrap());
    let source1 = Decoder::new(file1).unwrap().amplify(0.1); // Reducir volumen al 50%
    let sink1 = Sink::try_new(&stream_handle).unwrap();
    sink1.append(source1);
    sink1.pause(); // Pausamos para controlar cuándo iniciar

    let file2 = BufReader::new(File::open("./Assets/taylor.wav").unwrap());
    let source2 = Decoder::new(file2).unwrap().amplify(0.1); // Reducir volumen al 50%
    let sink2 = Sink::try_new(&stream_handle).unwrap();
    sink2.append(source2);
    sink2.pause(); // También pausamos el segundo audio

    // Cargar el efecto de sonido de caminar
    let walking_sound_file = BufReader::new(File::open("./Assets/walking.wav").unwrap());
    let walking_sound_source = Decoder::new(walking_sound_file).unwrap().amplify(0.4).repeat_infinite();
    let walking_sound_sink = Sink::try_new(&stream_handle).unwrap();
    walking_sound_sink.append(walking_sound_source);
    walking_sound_sink.pause(); // Inicialmente, pausa el sonido

    // Por defecto, comenzamos con el primer archivo
    sink1.play();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Proyecto 1",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let maze = load_maze("./maze.txt");
    let block_size = 55;

    framebuffer.set_background_color(0x333355);

    let mut player = Player {
        pos: Vec2::new(135.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0
    };

    let mut mode = "2D"; 
    let mut playing_first = true; // Bandera para controlar cuál archivo está sonando

    let (texture, texture_width, texture_height) = load_texture("./Assets/prueba2.jpg");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();

        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        if window.is_key_pressed(Key::T, minifb::KeyRepeat::No) {
            if playing_first {
                // Pausar el primer archivo y reproducir el segundo
                sink1.pause();
                sink2.play();
            } else {
                // Pausar el segundo archivo y reproducir el primero
                sink2.pause();
                sink1.play();
            }
            playing_first = !playing_first;
        }

        // Pass the walking sound sink to the process_events function
        process_events(&mut window, &mut player, &maze, block_size, &walking_sound_sink);

        if mode == "2D" {
            render_2d(&mut framebuffer, &player, &maze, block_size, &texture, texture_width, texture_height); 
        } else {
            render3d(&mut framebuffer, &player, &texture, texture_width, texture_height); 
        }

        framebuffer.draw_fps(750, 10); 

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
