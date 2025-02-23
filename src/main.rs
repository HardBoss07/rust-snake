use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

struct Game {
    window: Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    cell_size: usize,
    snake: Snake,
}

struct Snake {
    color: u32,
    current_position: [usize; 2],
    direction: u8,
    snake_length: u16,
}

impl Game {
    fn new() -> Self {
        let width = 600;
        let height = 400;

        let window = Window::new("Snake", width, height, WindowOptions::default())
            .expect("Failed to create window");

        let buffer = vec![0x000000; width * height];

        let color = 0xFF73F1;
        let starting_position: [usize; 2] = [10, 15];
        let direction = 0;
        let snake_length = 1;

        let snake = Snake {
            color: color,
            current_position: starting_position,
            direction: direction,
            snake_length: snake_length,
        };

        Game {
            window,
            width,
            height,
            buffer,
            cell_size: 20,
            snake,
        }
    }

    fn run(&mut self) {
        let mut has_started = false;

        while self.window.is_open() {
            if self.window.is_key_down(Key::Space) && has_started == false {
                has_started = true;
                self.set_cell_color(self.snake.current_position[0], self.snake.current_position[1], self.snake.color);
                self.update();
            }

            if has_started == true {
                self.capture_input(300);
                self.calculate_next_position();
                self.set_cell_color(self.snake.current_position[0], self.snake.current_position[1], self.snake.color);
            }

            self.update();

            if self.window.is_key_down(Key::Escape) {
                break;
            }
        }
    }

    fn capture_input(&mut self, amount: u16) {
        for i in 0..=amount {
            self.change_direction();
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn change_direction(&mut self) {
        if self.window.is_key_down(Key::W) {
            self.snake.direction = 1;                  // Up
        } else if self.window.is_key_down(Key::A) {
            self.snake.direction = 2;                  // Left
        } else if self.window.is_key_down(Key::S) {
            self.snake.direction = 3;                  // Down
        } else if (self.window.is_key_down(Key::D)) {
            self.snake.direction = 0;                  // Right
        } else {

        }
    }

    fn calculate_next_position(&mut self) {
        match self.snake.direction {
            0 => {
                if self.snake.current_position[1] < self.width / self.cell_size - 1 {
                    self.snake.current_position[1] += 1;
                }
            }
            1 => {
                if self.snake.current_position[0] > 0 {
                    self.snake.current_position[0] -= 1;
                }
            }
            2 => {
                if self.snake.current_position[1] > 0 {
                    self.snake.current_position[1] -= 1;
                }
            }
            3 => {
                if self.snake.current_position[0] < self.height / self.cell_size - 1 {
                    self.snake.current_position[0] += 1;
                }
            }
            _ => {}
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