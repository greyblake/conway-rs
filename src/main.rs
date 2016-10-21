extern crate rand;
extern crate clap;
extern crate termion;

use std::thread;

mod board;
mod config;

use board::Board;
use config::Config;

fn main() {
    let matches = clap::App::new("conway")
       .author("Sergey Potapov (https://github.com/greyblake)")
       .version("1.0")
       .about("Conway's Game Of Life.")
       .arg(clap::Arg::with_name("width")
           .short("w")
           .long("width")
           .value_name("WIDTH")
           .help("Set width of the board")
           .takes_value(true))
       .arg(clap::Arg::with_name("height")
           .long("height")
           .value_name("HEIGHT")
           .help("Set height of the board")
           .takes_value(true))
       .arg(clap::Arg::with_name("delay")
           .short("d")
           .long("delay")
           .value_name("MILLISECONDS")
           .help("Delay between the frames in milliseconds")
           .takes_value(true))
       .arg(clap::Arg::with_name("alive")
           .short("a")
           .long("alive")
           .value_name("CHAR")
           .help("Char to display alive cells")
           .takes_value(true))
       .arg(clap::Arg::with_name("dead")
           .long("dead")
           .value_name("CHAR")
           .help("Char to display dead cells")
           .takes_value(true))
       .get_matches();

    let config = Config::from_matches(matches);

    let mut board = Board::new(config.width, config.height);
    board.randomize();

    loop {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(3, 3));
        board_render(&board, config.alive, config.dead);
        board = board.next();
        thread::sleep(config.delay);
    }
}

fn board_render(board : &Board, alive : char, dead : char) {
    let mut output = String::with_capacity(board.width * (board.height + 1));
    for row in board.matrix.iter() {
        for cell in row.iter() {
            let ch = match *cell {
                true => alive,
                false => dead,
            };
            output.push(ch);
        }
        output.push_str("\n  ");
    }
    print!("{}", output);
}
