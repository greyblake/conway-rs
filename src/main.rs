extern crate rand;
extern crate clap;

use std::{thread, time};

mod board;
use board::Board;

fn main() {
    let matches = clap::App::new("conway")
       .author("Sergey Potapov (https://github.com/greyblake)")
       .version("1.0")
       .about("Conway's game of life.")
       .arg(clap::Arg::with_name("width")
           .short("w")
           .long("width")
           .value_name("WIDTH")
           .help("Set width of the board")
           .takes_value(true))
       .get_matches();

    let width = matches.value_of("width").unwrap_or("20").to_string();
    let width = width.parse::<i32>().unwrap_or(20);

    println!("width: {:?}", width);

    let delay = time::Duration::from_millis(500);
    let mut board = Board::new(width as usize, 34);
    board.randomize();

    loop {
        print!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        board_render(&board);
        board = board.next();
        thread::sleep(delay);
    }
}


fn board_render(board : &Board) {
    let mut output = String::with_capacity(board.width * (board.height + 1));
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
