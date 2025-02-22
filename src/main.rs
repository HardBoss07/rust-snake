use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

struct Game {
    window: Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    cell_size: usize,
}

impl Game {
    fn new() -> Self {
        let width = 600;
        let height = 400;

        let window = Window::new("Snake", width, height, WindowOptions::default())
            .expect("Failed to create window");

        let buffer = vec![0x000000; width * height];

        Game {
            window,
            width,
            height,
            buffer,
            cell_size: 20,
        }
    }

    fn run(&mut self) {
        let starting_position: [usize; 2] = [10, 15];

        while self.window.is_open() {
            if self.window.is_key_down(Key::Space) {
                self.set_cell_color(starting_position[0], starting_position[1], 0xFF0000);
                self.update();
                thread::sleep(Duration::from_millis(2000));
                self.reset_buffer();
            }

            self.update();

            if self.window.is_key_down(Key::Escape) {
                break;
            }
        }
    }

    fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .expect("Failed to update window");
    }

    fn reset_buffer(&mut self) {
        let cols = self.width / self.cell_size;
        let rows = self.height / self.cell_size;

        for i in 0..self.buffer.len() {
            self.buffer[i] = 0x318F40;
        }

        for row in 0..=rows {
            let y = row * self.cell_size;
            for col in 0..cols {
                let x = col * self.cell_size;

                if (row + col) % 2 == 0 {
                    for i in 0..self.cell_size {
                        for j in 0..self.cell_size {
                            if x + i < self.width && y + j < self.height {
                                self.buffer[(y + j) * self.width + (x + i)] = 0x4CAD5C;
                            }
                        }
                    }
                }
            }
        }
    }

    fn set_cell_color(&mut self, row: usize, col: usize, color: u32) {
        let x = col * self.cell_size;
        let y = row * self.cell_size;

        for i in 0..self.cell_size {
            for j in 0..self.cell_size {
                if x + i < self.width && y + j < self.height {
                    self.buffer[(y + j) * self.width + (x + i)] = color;
                }
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.reset_buffer();
    game.run();
}