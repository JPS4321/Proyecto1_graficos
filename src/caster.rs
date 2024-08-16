use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub impact_pos: (f32, f32),
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();

        
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        
        let i = x / block_size;
        let j = y / block_size;

        
        if j >= maze.len() || i >= maze[j].len() {
            
            return Intersect {
                distance: d,
                impact: ' ', 
                impact_pos: (player.pos.x + cos, player.pos.y + sin),
            };
        }

        if maze[j][i] != ' ' && maze[j][i] != 'F' {
            
            if d > 0.0 {
                return Intersect {
                    distance: d,
                    impact: maze[j][i],
                    impact_pos: (player.pos.x + cos, player.pos.y + sin),
                };
            }
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        d += 1.0;
    }
}
