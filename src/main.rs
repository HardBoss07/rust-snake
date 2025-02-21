use minifb::{Window, WindowOptions};
use std::thread;
use std::time::Duration;

fn main() {
    let width = 600;
    let height = 400;

    let mut window = Window::new("Snake", width, height, WindowOptions::default()).unwrap();
    let mut buffer: Vec<u32> = vec![0; width * height];

    let cell_size = 20;

    let starting_position: [usize; 2] = [10, 15];

    reset_buffer(&mut buffer, width, height, cell_size);

    while window.is_open() {
        if window.is_key_down(minifb::Key::Space) {
            set_cell_color(&mut buffer, width, height, cell_size, starting_position[0], starting_position[1], 0xFF0000);

            window.update_with_buffer(&buffer, width, height);

            thread::sleep(Duration::from_millis(2000));

            reset_buffer(&mut buffer, width, height, cell_size);
        }

        window.update_with_buffer(&buffer, width, height).unwrap();

        if window.is_key_down(minifb::Key::Escape) {
            break;
        }
    }
}

fn reset_buffer(buffer: &mut Vec<u32>, width: usize, height: usize, cell_size: usize) {
    let cols = width / cell_size;
    let rows = height / cell_size;

    for i in 0..buffer.len() {
        buffer[i] = 0x318F40;
    }

    for row in 0..=rows {
        let y = row * cell_size;
        for col in 0..=cols {
            let x = col * cell_size;

            if (row + col) % 2 == 0 {
                for i in 0..cell_size {
                    for j in 0..cell_size {
                        if x + i < width && y + j < height {
                            buffer[(y + j) * width + (x + i)] = 0x4CAD5C;
                        }
                    }
                }
            }
        }
    }
}

fn set_cell_color(buffer: &mut Vec<u32>, width: usize, height: usize, cell_size: usize, row: usize, col: usize, color: u32) {
    let x = col * cell_size;
    let y = row * cell_size;

    for i in 0..cell_size {
        for j in 0..cell_size {
            if x + i < width && y + j < height {
                buffer[(y + j) * width + (x + i)] = color;
            }
        }
    }
}