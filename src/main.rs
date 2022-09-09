mod sdl_app;

extern crate sdl2;

use std::f32::consts::PI;
use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use crate::sdl_app::SDLApp;

const N: usize = 250;
const SCREEN_WIDTH: usize = 750;
const SCREEN_HEIGHT: usize = 750;
const ATOM_SPREAD: usize = 2;
const CELL_TYPES: usize = 10;

const CELLS: usize = N * N;
const CELL_WIDTH: usize = SCREEN_WIDTH / N;
const CELL_HEIGHT: usize = SCREEN_HEIGHT / N;

type Cell = u8;
type Board<T = Cell> = [T; CELLS];

static mut BOARD: Board = [0; CELLS];
static mut TICK: u64 = 0;

fn main() {
    let mut rng = rand::thread_rng();

    let mut app = SDLApp::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, "Atoms");
    app.event_handler = event_handler;

    let board = unsafe { &mut BOARD };
    for r in 0..N {
        for c in 0..N {
            *at(board, r, c) = if rng.gen::<u8>() % 3 == 0 {
                gen_cell(&mut rng)
            } else {
                0
            };
        }
    }

    let mut colors: [Color; CELL_TYPES] = [Color::BLACK; CELL_TYPES];
    for x in colors.iter_mut() {
        *x = Color::RGB(
            rng.gen::<u8>() % 230 + 25,
            rng.gen::<u8>() % 230 + 25,
            rng.gen::<u8>() % 230 + 25,
        );
    }

    while app.cycle() {
        unsafe { TICK += 1; }
        update_board(board);

        app.canvas.set_draw_color(Color::RGB(15, 15, 15));
        app.canvas.clear();

        draw_grid(&mut app);
        draw_cells(&mut app, board, &colors);
    }
}

fn at<T>(board: &mut Board<T>, r: usize, c: usize) -> &mut T {
    &mut board[r * N + c]
}

fn gen_cell(rng: &mut ThreadRng) -> Cell {
    (rng.gen::<usize>() % CELL_TYPES) as u8
}

fn event_handler(event: &Event) -> bool {
    if let (close, /*handled*/ true) = SDLApp::default_event_handler(event) {
        return close;
    }

    let board = unsafe { &mut BOARD };
    match event {
        Event::MouseMotion { x, y, .. } => {
            let mut rng = rand::thread_rng();
            let r = (y / CELL_HEIGHT as i32) as usize;
            let c = (x / CELL_WIDTH as i32) as usize;
            *at(board, r, c) = gen_cell(&mut rng);
        }
        Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
            update_board(board);
        }
        _ => {}
    };

    false
}


fn update_board(board: &mut Board) {
    let mut changes: Board<bool> = [false; CELLS];

    let t1 = 25.0 * f32::cos(0.11 * PI * unsafe { TICK } as f32) / 2.0;
    let t2 = 75.0 * f32::sin(0.03 * PI * unsafe { TICK } as f32) / 7.0;
    let t3 = t1 + t2;
    *at(board, 10, (1.0 * (N as f32) / 2.0 + t1) as usize % N) = ((-1 % CELL_TYPES as i32 + CELL_TYPES as i32) % CELL_TYPES as i32) as Cell;
    *at(board, 25, (4.0 * (N as f32) / 5.0 + t2) as usize % N) = ((-3 % CELL_TYPES as i32 + CELL_TYPES as i32) % CELL_TYPES as i32) as Cell;
    *at(board, 0, (1.0 * (N as f32) / 5.0 + t3) as usize % N) = ((-5 % CELL_TYPES as i32 + CELL_TYPES as i32) % CELL_TYPES as i32) as Cell;

    for r in (0..N).rev() {
        for c in (0..N) {
            if r < N - 1 && *at(board, r, c) != 0 {
                for d in 0..ATOM_SPREAD {
                    if c < N - d && *at(board, r + 1, c + d) < *at(board, r, c) {
                        if !*at(&mut changes, r + 1, c + d) && !*at(&mut changes, r, c) && true {
                            let x = *at(board, r + 1, c + d);
                            *at(board, r + 1, c + d) = *at(board, r, c);
                            *at(board, r, c) = x;
                            *at(&mut changes, r + 1, c + d) = true;
                            *at(&mut changes, r, c) = true;
                        }
                        break;
                    } else if c >= d && *at(board, r + 1, c - d) < *at(board, r, c) {
                        if !*at(&mut changes, r + 1, c - d) && !*at(&mut changes, r, c) && true {
                            let x = *at(board, r + 1, c - d);
                            *at(board, r + 1, c - d) = *at(board, r, c);
                            *at(board, r, c) = x;
                            *at(&mut changes, r + 1, c - d) = true;
                            *at(&mut changes, r, c) = true;
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn draw_cells(app: &mut SDLApp, board: &mut Board, colors: &[Color; CELL_TYPES]) {
    for r in 0..N {
        for c in 0..N {
            let cell = *at(board, r, c);
            if cell != 0 {
                // app.canvas.set_draw_color(Color::RGB(255 - cell, 25, cell));
                app.canvas.set_draw_color(colors[cell as usize]);
                app.canvas.fill_rect(Rect::new(
                    (c * CELL_WIDTH) as i32,
                    (r * CELL_HEIGHT) as i32,
                    CELL_WIDTH as u32,
                    CELL_HEIGHT as u32,
                )).expect("Cannot draw");
            }
        }
    }
}

fn draw_grid(app: &mut SDLApp) {
    app.canvas.set_draw_color(Color::RGB(50, 50, 50));
    for c in 1..N {
        app.canvas.draw_line(
            Point::new((c * CELL_WIDTH) as i32, 0),
            Point::new((c * CELL_WIDTH) as i32, SCREEN_HEIGHT as i32),
        ).expect("Cannot draw");
    }
    for r in 1..N {
        app.canvas.draw_line(
            Point::new(0, (r * CELL_HEIGHT) as i32),
            Point::new(SCREEN_WIDTH as i32, (r * CELL_HEIGHT) as i32),
        ).expect("Cannot draw");
    }
}
