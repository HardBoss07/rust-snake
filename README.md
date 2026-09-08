# Snake - Rust Framebuffer Engine

A 2D Snake game built in **Rust** using the **`minifb`** window management and minimal framebuffer library. Developed as a foundational exploration into the Rust ecosystem, this project focuses on low-level memory safety, custom pixel-buffer rendering, efficient data structure design, and synchronized game loop management.

## Technical Overview & Core Learning Objectives

- **Memory Safety & Ownership:** Built around Rust's strict ownership model to handle raw array indexing and state mutation safely without relying on garbage collection or risking buffer overflows.
- **Low-Level Pixel Manipulation:** Designed a direct-to-buffer rendering system that translates grid coordinates into 32-bit color values inside a continuous standard vector (`Vec<u32>`), bypassing higher-level game engines.
- **Data Structure Efficiency:** Used double-ended queues (`VecDeque`) to maintain $O(1)$ constant time complexity for snake body operations (head insertion and tail removal during movement and growth).
- **Encapsulated State Architecture:** Structured core state into distinct `Game` and `Snake` primitives, separating application logic, user input capture, frame updates, and world reset recovery.

## Tech Stack

- **Language:** Rust
- **Windowing & Graphics:** `minifb` (Cross-platform minimal framebuffer library)
- **Data Structures:** `std::collections::VecDeque`

## Architecture & Features

- **Direct Framebuffer Engine:** Operates on a fixed pixel array where cell coordinates dynamically calculate index offsets, rendering a 600x400 window using raw color values.
- **$O(1)$ Body Queue Management:** The snake's body segments are stored in a `VecDeque`. When moving, a new head position is pushed to the front while the tail is popped off the back. When food ("cookies") is consumed, tail removal is skipped, seamlessly growing the entity.
- **Synchronized Game Loop:** Implements a state-driven loop operating at a fixed tick rate (150ms). It handles continuous keyboard input polling (Spacebar start, directional navigation, Escape exit) while guaranteeing immediate window frame updates.
- **Automatic State Recovery:** Handled game resets gracefully upon boundary or self-collision by resetting entity structs and zeroing out the pixel buffer without memory leak overhead.

## Controls

| Key            | Action                               |
| -------------- | ------------------------------------ |
| **Spacebar**   | Start Game / Restart after Game Over |
| **Arrow Keys** | Change Direction                     |
| **Escape**     | Exit Application                     |

## Building & Running Locally

### Prerequisites

- [Rust Toolchain](https://www.rust-lang.org/tools/install) (`cargo`, `rustc`)

### Execution

1. **Clone the repository:**

```bash
git clone https://github.com/HardBoss07/rust-snake.git
cd rust-snake
```

2. **Build and run in release mode:**

```bash
cargo run --release
```
