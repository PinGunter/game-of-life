use std::vec;

use raylib::prelude::*;

const WINDOW_SIZE: usize = 1000;
const GRID_SIZE: usize = 25;
const SQUARE_SIZE: usize = WINDOW_SIZE / GRID_SIZE;

fn draw_square(
    d: &mut RaylibDrawHandle<'_>,
    x: usize,
    y: usize,
    size: usize,
    color: Color,
    fill: bool,
) {
    if fill {
        d.draw_rectangle(x as i32, y as i32, size as i32, size as i32, color);
    } else {
        d.draw_rectangle_lines(x as i32, y as i32, size as i32, size as i32, color);
    }
}

fn simulate(grid: &mut [[bool; GRID_SIZE]; GRID_SIZE]) {
    let grid_size = grid.len();
    let mut next_step = vec![vec![false; grid_size]; grid_size];

    for i in 0..grid_size {
        for j in 0..grid_size {
            let mut n_living = 0;

            for ni in -1..=1 {
                for nj in -1..=1 {
                    if !(ni == 0 && nj == 0) {
                        let idx = (i as isize + ni).clamp(0, grid_size as isize - 1) as usize;
                        let jdx = (j as isize + nj).clamp(0, grid_size as isize - 1) as usize;

                        if grid[idx][jdx] {
                            n_living += 1;
                        }
                    }
                }
            }

            next_step[i][j] = match (grid[i][j], n_living) {
                (true, 2..=3) | (false, 3) => true,
                _ => false,
            };
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i as usize].len() {
            grid[i][j] = next_step[i][j];
        }
    }
}

fn main() {
    assert!((WINDOW_SIZE as f64 / GRID_SIZE as f64) == SQUARE_SIZE as f64);
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE as i32, WINDOW_SIZE as i32)
        .title("Game of Life")
        .build();

    let mut cell_grid = [[false; GRID_SIZE]; GRID_SIZE];

    let mut current_time: f64;
    let mut end_time: f64 = rl.get_time();
    let mut remaining_time: f64;

    let mut simulating: bool = false;

    let next_iteration = 1.0;
    while !rl.window_should_close() {
        current_time = rl.get_time();
        remaining_time = end_time - current_time;

        // event polling
        let mouse_position = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            cell_grid[mouse_position.y as usize / SQUARE_SIZE]
                [mouse_position.x as usize / SQUARE_SIZE] = !cell_grid
                [mouse_position.y as usize / SQUARE_SIZE][mouse_position.x as usize / SQUARE_SIZE];
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            simulating = !simulating;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            cell_grid = [[false; GRID_SIZE]; GRID_SIZE];
        }

        if simulating && remaining_time <= 0.0 {
            simulate(&mut cell_grid);
            end_time = rl.get_time() + next_iteration;
        }

        // Start rendering
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // draw grid
        let mut opacity: f32;
        for row in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                opacity = if mouse_position.x as usize / SQUARE_SIZE == column
                    && mouse_position.y as usize / SQUARE_SIZE == row
                {
                    1.0
                } else {
                    0.3
                };
                let fill = cell_grid[row][column];
                draw_square(
                    &mut d,
                    column * SQUARE_SIZE,
                    row * SQUARE_SIZE,
                    SQUARE_SIZE,
                    Color::WHITE.alpha(if fill { 1.0 } else { opacity }),
                    fill,
                );
            }
        }
        if simulating {
            d.draw_text("Simulating", 890, 940, 20, Color::ORANGE);
        }
    }
}
