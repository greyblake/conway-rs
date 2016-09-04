extern crate rand;

use rand::Rng;
use std::{thread, time};

mod board;
use board::Board;

const WIDTH : usize = 20;
const HEIGH : usize = 10;

const SIZE : usize = WIDTH * HEIGH;

type MatrixLine = [bool; WIDTH];
type Matrix = [MatrixLine; HEIGH];


fn matrix_init() -> Matrix {
    let mut matrix : Matrix = [[false; WIDTH]; HEIGH];
    let mut rng = rand::thread_rng();

    for _ in 0..(SIZE / 2) {
        let x = rng.gen_range(0, WIDTH);
        let y = rng.gen_range(0, HEIGH);
        matrix[y][x] = true;
    }

    matrix
}

fn matrix_render(matrix : Matrix) {
    let mut output = String::with_capacity(WIDTH * (HEIGH + 1));
    for row in matrix.iter() {
        for cell in row.iter() {
            let ch = match *cell {
                true => "◉",
                false => ".",
            };
            output.push_str(ch);
        }
        output.push_str("\n");
    }
    print!("{}", output);
}

fn matrix_next(matrix : Matrix) -> Matrix {
    let mut result: Matrix = [[false; WIDTH]; HEIGH];
    for (ri, row) in matrix.iter().enumerate() {
        for (ci, cell) in row.iter().enumerate() {
            let alive_count = matrix_count_cell_neighbors(matrix, ri as i32, ci as i32);
            let is_alive = match (*cell, alive_count) {
                (_   , 3) => true,
                (true, 2) => true,
                (_   , _) => false,
            };
            result[ri][ci] = is_alive;
        }
    }
    result
}

fn cyclical_add(base: i32, a: i32, b: i32) -> i32 {
    let mut res = (a + b) % base;
    if res < 0 { res += base; }
    res
}

#[test]
fn test_cyclical_add() {
    assert_eq!(cyclical_add(10, 9, 1), 0);
    assert_eq!(cyclical_add(10, 0, -1), 9);
    assert_eq!(cyclical_add(5, 2, 4), 1);
}

fn matrix_count_cell_neighbors(matrix: Matrix, ri: i32, ci: i32) -> i32 {
    let mut count = 0;
    for ro in -1..2 {
        for co in -1..2 {
            if ro == 0 && co == 0 { continue; }
            let r = cyclical_add(HEIGH as i32, ri, ro) as usize;
            let c = cyclical_add(WIDTH as i32, ci, co) as usize;
            if matrix[r][c] { count += 1; }
        }
    }
    count
}


fn main() {
    let mut matrix = matrix_init();
    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);

    //loop {
    //    print!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    //    matrix_render(matrix);
    //    matrix = matrix_next(matrix);
    //    thread::sleep(delay);
    //}
    //let board = Board::new();
    let mut b = Board::new(10, 5);
    b.randomize();
    board_render(&b);
}

fn board_render(board : &Board) {
    let mut output = String::with_capacity(WIDTH * (HEIGH + 1));
    for row in board.matrix.iter() {
        for cell in row.iter() {
            let ch = match *cell {
                true => "◉",
                false => ".",
            };
            output.push_str(ch);
        }
        output.push_str("\n");
    }
    print!("{}", output);
}
