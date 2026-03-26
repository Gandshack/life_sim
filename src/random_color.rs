use rand::RngExt;
use raylib::color::Color;

pub struct ColorPicker {
    options: Vec<Colors>,
}

impl ColorPicker {
    pub fn new() -> Self {
        Self {
            options: vec![
                Colors::Red,
                Colors::Green,
                Colors::Blue,
                Colors::Yellow,
                Colors::Cyan,
                Colors::Magenta,
            ],
        }
    }

    pub fn random_color(&mut self) -> Color {
        let index = rand::rng().random_range(0..self.options.len());
        self.options.swap_remove(index).to_color()
    }
}

#[derive(Clone, Copy)]
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

impl Colors {
    fn to_color(self) -> Color {
        match self {
            Colors::Red => Color::RED,
            Colors::Green => Color::GREEN,
            Colors::Blue => Color::BLUE,
            Colors::Yellow => Color::YELLOW,
            Colors::Cyan => Color::CYAN,
            Colors::Magenta => Color::MAGENTA,
        }
    }
}
