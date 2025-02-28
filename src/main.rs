#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use rand::Rng;

struct Game {
    window: Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    cell_size: usize,
    snake: Snake,
    has_started: bool,
    cookie_pos: [usize; 2],
}

struct Snake {
    color: u32,
    body: VecDeque<[usize; 2]>,
    direction: u8,
    length: usize,
}

impl Snake {
    fn new() -> Self {
        let color = 0xFF73F1;

        let direction = 0;
        let snake_length = 5;

        let starting_position: [usize; 2] = [10, 12];
        let mut snake_body = VecDeque::new();
        snake_body.push_front(starting_position);

        Snake {
            color: color,
            body: snake_body,
            direction: direction,
            length: snake_length,
        }
    }
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
            snake: Snake::new(),
            has_started: false,
            cookie_pos: [8, 9],
        }
    }

    fn run(&mut self) {
        while self.window.is_open() {
            if self.window.is_key_down(Key::Space) && self.has_started == false {
                self.reset_buffer();
                self.has_started = true;
                let head = *self.snake.body.front().unwrap();
                self.set_cell_color(head[0], head[1], self.snake.color);
                self.set_cell_color(self.cookie_pos[0], self.cookie_pos[1], 0xF59B42);
                self.update();
            }

            if self.has_started == true {
                self.capture_input(150);
                self.calculate_next_position();

                self.cookie_check();

                let head = *self.snake.body.front().unwrap();
                self.set_cell_color(head[0], head[1], self.snake.color);
            }

            if !self.has_started {
                self.snake = Snake::new();
            }

            self.update();
            
            if self.window.is_key_down(Key::Escape) {
                break;
            }
        }
    }

    fn cookie_check(&mut self) {
        let head = *self.snake.body.front().unwrap();
        if head == self.cookie_pos {
            self.snake.length += 1;
            self.rand_cookie_pos();
            self.set_cell_color(self.cookie_pos[0], self.cookie_pos[1], 0xF59B42);
        }
    }

    fn rand_cookie_pos(&mut self) {
        let mut rng = rand::thread_rng();
        let max_attempts = 100;
        let mut attempts = 0;
    
        while attempts < max_attempts {
            let x = rng.gen_range(0..(self.height / self.cell_size));
            let y = rng.gen_range(0..(self.width / self.cell_size));
            let target = [x, y];
    
            if !self.snake.body.contains(&target) {
                self.cookie_pos = target;
                return;
            }
    
            attempts += 1;
        }
    
        // fallback
        for x in 0..(self.width / self.cell_size) {
            for y in 0..(self.height / self.cell_size) {
                let fallback = [x, y];
                if !self.snake.body.contains(&fallback) {
                    self.cookie_pos = fallback;
                    return;
                }
            }
        }
    }

    fn capture_input(&mut self, mut amount: u64) {
        amount *= 2;
        for i in 0..=amount {
            self.change_direction();
            thread::sleep(Duration::from_micros(5));
        }
    }

    fn change_direction(&mut self) {
        if self.window.is_key_down(Key::W) || self.window.is_key_down(Key::Up) && self.snake.direction != 3 { // Up
            self.snake.direction = 1;
        } else if self.window.is_key_down(Key::A) || self.window.is_key_down(Key::Left) && self.snake.direction != 0 { // Left
            self.snake.direction = 2;
        } else if self.window.is_key_down(Key::S) || self.window.is_key_down(Key::Down) && self.snake.direction != 1 { // Down
            self.snake.direction = 3;
        } else if (self.window.is_key_down(Key::D)) || self.window.is_key_down(Key::Right) && self.snake.direction != 2 { // Right
            self.snake.direction = 0;
        } else {

        }
    }

    fn calculate_next_position(&mut self) {
        let head = *self.snake.body.front().unwrap();
        let mut new_head = head;

        match self.snake.direction {
            0 => new_head[1] += 1,
            1 => {
                if new_head[0] == 0 {
                    self.has_started = false;
                    return;
                }
                new_head[0] -= 1;
            }
            2 => {
                if new_head[1] == 0 {
                    self.has_started = false;
                    return;
                }
                new_head[1] -= 1;
            }
            3 => new_head[0] += 1,
            _ => {}
        }

        if new_head[0] >= self.height / self.cell_size || new_head[1] >= self.width / self.cell_size || self.snake.body.contains(&new_head) {
            self.has_started = false;
            return;
        }

        self.snake.body.push_front(new_head);

        if self.snake.body.len() > self.snake.length {
            let tail = self.snake.body.pop_back().unwrap();

            let original_color = if (tail[0] + tail[1]) % 2 == 0 {
                0x4CAD5C 
            } else {
                0x318F40
            };

            self.set_cell_color(tail[0], tail[1], original_color);
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
    println!("Controls:\nStart and Restart Game: Space\nQuit Game: Escape\nUp: W or Arrow Up\nLeft: A or Arrow Left\nDown: S or Arrow Down\nRight: D or Arrow Right");
    let mut game = Game::new();
    game.reset_buffer();
    game.run();
}