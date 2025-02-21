use minifb::{Window, WindowOptions};

fn main() {
    let mut window = Window::new("Snake", 800, 600, WindowOptions::default()).unwrap();

    let mut buffer: Vec<u32> = vec!(0; 800 * 600);

    while window.is_open() {
        for i in 0..buffer.len() {
            buffer[i] = 0x000000;
        }

        window.update_with_buffer(&buffer, 800, 600).unwrap();
    
        if window.is_key_down(minifb::Key::Escape) {
            break;
        }
    }
}
