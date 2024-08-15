use std::time::Instant;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
    last_frame_time: Instant,

}

impl Framebuffer {
    const DIGITS: [[u8; 5]; 10] = [
        [0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // 0
        [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
        [0b01110, 0b00001, 0b01110, 0b10000, 0b11111], // 2
        [0b01110, 0b00001, 0b01110, 0b00001, 0b01110], // 3
        [0b10001, 0b10001, 0b11111, 0b00001, 0b00001], // 4
        [0b11111, 0b10000, 0b11110, 0b00001, 0b11110], // 5
        [0b01110, 0b10000, 0b11110, 0b10001, 0b01110], // 6
        [0b11111, 0b00001, 0b00010, 0b00100, 0b01000], // 7
        [0b01110, 0b10001, 0b01110, 0b10001, 0b01110], // 8
        [0b01110, 0b10001, 0b01111, 0b00001, 0b01110], // 9
    ];
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
            last_frame_time: Instant::now(),

        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }
    

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    fn update_frame_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;
        delta.as_secs_f32()
    }

    pub fn draw_fps(&mut self, x: usize, y: usize) {
        let fps = 1.0 / self.update_frame_time();
        let fps_text = format!("{:.2} FPS", fps);
        self.draw_text(x as u32, y as u32, &fps_text);
    }
    fn draw_text(&mut self, x: u32, y: u32, text: &str) {
        let mut offset_x = x;
        for ch in text.chars() {
            self.draw_char(offset_x, y, ch);
            offset_x += 8; // Asumiendo un ancho fijo de 8 píxeles por carácter
        }
    }

    fn draw_char(&mut self, x: u32, y: u32, char: char) {
        if let Some(digit) = char.to_digit(10) {
            let pattern = Self::DIGITS[digit as usize];
            for (i, row) in pattern.iter().enumerate() {
                for j in 0..5 {
                    if (row >> (4 - j)) & 1 == 1 {
                        self.point((x + j as u32) as usize, (y + i as u32) as usize);
                    }
                }
            }
        }
    }
    
}
