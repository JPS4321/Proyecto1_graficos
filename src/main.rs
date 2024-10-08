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
mod menu;

use menu::Menu;
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
    texture_height: usize,
    flag_texture: &Vec<u32>,
    flag_texture_width: usize,
    flag_texture_height: usize
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == 'F' {
                
                for y in 0..block_size {
                    for x in 0..block_size {
                        let tx = (x * flag_texture_width) / block_size;
                        let ty = (y * flag_texture_height) / block_size;
                        let color = flag_texture[ty * flag_texture_width + tx];
                        framebuffer.set_current_color(color);
                        framebuffer.point(col_index * block_size + x, row_index * block_size + y);
                    }
                }
            } else if cell != ' ' {  
                
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

    
    let player_size = 5;

    
    for y in (player.pos.y as usize).saturating_sub(player_size)..=(player.pos.y as usize + player_size) {
        for x in (player.pos.x as usize).saturating_sub(player_size)..=(player.pos.x as usize + player_size) {
            framebuffer.point(x, y);
        }
    }
}



fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    wall_texture: &Vec<u32>,
    wall_texture_width: usize,
    wall_texture_height: usize,
    floor_texture: &Vec<u32>,
    floor_texture_width: usize,
    floor_texture_height: usize
) {
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    
    framebuffer.set_current_color(0x03a9f4);
    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y);
        }
    }

    
    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            
            let texture_x = (x * floor_texture_width) / framebuffer.width;
            let texture_y = ((y - hh as usize) * floor_texture_height) / (framebuffer.height - hh as usize);
            let color = floor_texture[texture_y * floor_texture_width + texture_x];
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }

    
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false);
        let distance = intersect.distance;

        
        if intersect.impact == 'F' {
            continue;
        }

        let wall_height = (block_size as f32 / distance) * 200.0;

        let y0 = hh - (wall_height / 2.0);
        let y1 = hh + (wall_height / 2.0);

        let wall_x = if intersect.impact == '|' {
            intersect.impact_pos.1 % block_size as f32
        } else {
            intersect.impact_pos.0 % block_size as f32
        };

        let texture_x = ((wall_x / block_size as f32) * wall_texture_width as f32).clamp(0.0, (wall_texture_width - 1) as f32) as usize;

        for y in y0 as usize..y1 as usize {
            let texture_y = (((y - y0 as usize) * wall_texture_height) / wall_height as usize).clamp(0, wall_texture_height - 1);
            let color = wall_texture[texture_y * wall_texture_width + texture_x];
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}


fn render_minimap(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    texture: &Vec<u32>,
    texture_width: usize,
    texture_height: usize,
    flag_texture: &Vec<u32>,
    flag_texture_width: usize,
    flag_texture_height: usize,
    minimap_scale: usize 
) {
    let scaled_block_size = block_size / minimap_scale;

    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == 'F' {
                
                for y in 0..scaled_block_size {
                    for x in 0..scaled_block_size {
                        let tx = (x * flag_texture_width) / scaled_block_size;
                        let ty = (y * flag_texture_height) / scaled_block_size;
                        let color = flag_texture[ty * flag_texture_width + tx];
                        framebuffer.set_current_color(color);
                        framebuffer.point(col_index * scaled_block_size + x, row_index * scaled_block_size + y);
                    }
                }
            } else if cell != ' ' {  
                
                for y in 0..scaled_block_size {
                    for x in 0..scaled_block_size {
                        let tx = (x * texture_width) / scaled_block_size;
                        let ty = (y * texture_height) / scaled_block_size;
                        let color = texture[ty * texture_width + tx];
                        framebuffer.set_current_color(color);
                        framebuffer.point(col_index * scaled_block_size + x, row_index * scaled_block_size + y);
                    }
                }
            }
        }
    }

    framebuffer.set_current_color(0xFFDDD);

    
    let minimap_player_size = 2;

    
    let minimap_player_x = (player.pos.x as usize) / minimap_scale;
    let minimap_player_y = (player.pos.y as usize) / minimap_scale;

    for y in minimap_player_y.saturating_sub(minimap_player_size)..=(minimap_player_y + minimap_player_size) {
        for x in minimap_player_x.saturating_sub(minimap_player_size)..=(minimap_player_x + minimap_player_size) {
            framebuffer.point(x, y);
        }
    }
}

fn load_texture_from_buffer(path: &str) -> (Vec<u32>, usize, usize) {
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


fn calculate_player_pos(player: &Player, block_size: usize) -> (usize, usize) {
    let player_row = (player.pos.y / block_size as f32) as usize;
    let player_col = (player.pos.x / block_size as f32) as usize;
    (player_row, player_col)
}


fn draw_centered_image(framebuffer: &mut Framebuffer, image: &Vec<u32>, img_width: usize, img_height: usize, framebuffer_width: usize, framebuffer_height: usize) {
    let x_offset = (framebuffer_width - img_width) / 2;
    let y_offset = (framebuffer_height - img_height) / 2;

    for y in 0..img_height {
        for x in 0..img_width {
            if x + x_offset < framebuffer_width && y + y_offset < framebuffer_height {
                let color = image[y * img_width + x];
                framebuffer.set_current_color(color);
                framebuffer.point(x + x_offset, y + y_offset);
            }
        }
    }
}






fn main() {
    let window_width = 900;
    let window_height = 600;

    let mut menu = Menu::new(window_width, window_height);
    let selected_mode = menu.run();

    let maze_file = match selected_mode {
        Some("easy") => "./maze_easy.txt",
        Some("hard") => "./maze_hard.txt",
        _ => {
            println!("No mode selected, exiting...");
            return;
        }
    };

    
    let framebuffer_width = 900;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    
    let file1 = BufReader::new(File::open("./Assets/maintheme.wav").unwrap());
    let source1 = Decoder::new(file1).unwrap().amplify(0.1); 
    let sink1 = Sink::try_new(&stream_handle).unwrap();
    sink1.append(source1);
    sink1.pause(); 

    let file2 = BufReader::new(File::open("./Assets/taylor.wav").unwrap());
    let source2 = Decoder::new(file2).unwrap().amplify(0.1); 
    let sink2 = Sink::try_new(&stream_handle).unwrap();
    sink2.append(source2);
    sink2.pause(); 

    
    let walking_sound_file = BufReader::new(File::open("./Assets/walking.wav").unwrap());
    let walking_sound_source = Decoder::new(walking_sound_file).unwrap().amplify(0.4).repeat_infinite();
    let walking_sound_sink = Sink::try_new(&stream_handle).unwrap();
    walking_sound_sink.append(walking_sound_source);
    walking_sound_sink.pause(); 

    
    sink1.play();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Proyecto 1",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let maze = load_maze(maze_file);
    let block_size = 55;

    framebuffer.set_background_color(0x333355);

    let mut player = Player {
        pos: Vec2::new(135.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0
    };

    let mut mode = "2D"; 
    let mut playing_first = true; 

    let (wall_texture, wall_texture_width, wall_texture_height) = load_texture("./Assets/prueba2.jpg");

    
    let (floor_texture, floor_texture_width, floor_texture_height) = load_texture("./Assets/grass.jpg");
    let (flag_texture, flag_texture_width, flag_texture_height) = load_texture_from_buffer("./Assets/marioflag.png");
    let (final_screen_texture, final_screen_texture_width, final_screen_texture_height) = load_texture_from_buffer("./Assets/FinalScreen.png");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();

        let player_pos = calculate_player_pos(&player, block_size);
        if player_pos.0 < maze.len() && player_pos.1 < maze[player_pos.0].len() {
            if maze[player_pos.0][player_pos.1] == 'F' {
                framebuffer.clear(); 
                let img_width = final_screen_texture_width.min(framebuffer_width);
                let img_height = final_screen_texture_height.min(framebuffer_height);
        
                println!("Jugador alcanzó la posición 'F', mostrando imagen de victoria...");
                draw_centered_image(&mut framebuffer, &final_screen_texture, img_width, img_height, framebuffer_width, framebuffer_height);
                window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
                std::thread::sleep(Duration::from_secs(2)); 
                break;
            }
        } else {
            println!("Posición del jugador fuera de los límites: ({}, {})", player_pos.0, player_pos.1);
        }

        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        if window.is_key_pressed(Key::T, minifb::KeyRepeat::No) {
            if playing_first {
                
                sink1.pause();
                sink2.play();
            } else {
                
                sink2.pause();
                sink1.play();
            }
            playing_first = !playing_first;
        }

        
        process_events(&mut window, &mut player, &maze, block_size, &walking_sound_sink);

        if mode == "2D" {
            render_2d(&mut framebuffer, &player, &maze, block_size, &wall_texture, wall_texture_width, wall_texture_height, &flag_texture, flag_texture_width, flag_texture_height); 
        } else {
            render3d(&mut framebuffer, &player, &maze, block_size, &wall_texture, wall_texture_width, wall_texture_height, &floor_texture, floor_texture_width, floor_texture_height); 
            let minimap_scale = 5; 
            render_minimap(
                &mut framebuffer, 
                &player, 
                &maze, 
                block_size, 
                &wall_texture, 
                wall_texture_width, 
                wall_texture_height, 
                &flag_texture, 
                flag_texture_width, 
                flag_texture_height, 
                minimap_scale
            );
               
        }

        framebuffer.draw_fps(750, 10); 

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
    std::process::exit(0); 

}