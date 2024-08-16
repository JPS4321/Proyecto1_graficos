use minifb::{Window, WindowOptions, Key, MouseMode, MouseButton};
use image::GenericImageView;

pub struct Menu {
    window: Window,
    easy_selected: bool,
    hard_selected: bool,
    background_image: Vec<u32>,
    easy_highlight_image: Vec<u32>,
    hard_highlight_image: Vec<u32>,
    width: usize,
    height: usize,
}

impl Menu {
    pub fn new(window_width: usize, window_height: usize) -> Menu {
        // Cargar las imágenes del menú
        let (background_image, width, height) = load_texture("./Assets/Menus/Menu(basic).png");
        let (easy_highlight_image, _, _) = load_texture("./Assets/Menus/Menu(easy).png");
        let (hard_highlight_image, _, _) = load_texture("./Assets/Menus/Menu(hard).png");

        let window = Window::new(
            "Menu",
            window_width,
            window_height,
            WindowOptions::default(),
        ).unwrap();

        Menu {
            window,
            easy_selected: false,
            hard_selected: false,
            background_image,
            easy_highlight_image,
            hard_highlight_image,
            width,
            height,
        }
    }

    pub fn run(&mut self) -> Option<&str> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let mouse_pos = self.window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
            let (mouse_x, mouse_y) = (mouse_pos.0 as usize, mouse_pos.1 as usize);
    
            self.easy_selected = self.is_mouse_over_easy(mouse_x, mouse_y);
            self.hard_selected = self.is_mouse_over_hard(mouse_x, mouse_y);
    
            // Dibujar el menú dependiendo de la selección
            let buffer = if self.easy_selected {
                &self.easy_highlight_image
            } else if self.hard_selected {
                &self.hard_highlight_image
            } else {
                &self.background_image
            };
    
            self.window.update_with_buffer(buffer, self.width, self.height).unwrap();
    
            if self.window.get_mouse_down(MouseButton::Left) {
                if self.easy_selected {
                    break;  // Salir del ciclo
                } else if self.hard_selected {
                    break;  // Salir del ciclo
                }
            }
        }
    
        if self.easy_selected {
            return Some("easy");
        } else if self.hard_selected {
            return Some("hard");
        }
    
        None
    }
    

    fn is_mouse_over_easy(&self, x: usize, y: usize) -> bool {
        // Definir el área donde se encuentra la opción "Easy Mode"
        x > 200 && x < 400 && y > 300 && y < 350
    }

    fn is_mouse_over_hard(&self, x: usize, y: usize) -> bool {
        // Definir el área donde se encuentra la opción "Hard Mode"
        x > 500 && x < 700 && y > 300 && y < 350
    }
}

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
        (a << 24) | (r << 16) | (g << 8) | b
    }).collect();
    (texture, width as usize, height as usize)
}
