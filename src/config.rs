extern crate clap;
use std::process::exit;
use std::time;

#[derive(Debug)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub delay: time::Duration,
    pub alive: char,
    pub dead: char
}

const DEFAULT_WIDTH : u32 = 80;
const DEFAULT_HEIGHT : u32 = 20;
const DEFAULT_DELAY : u64 = 250;
const DEFAULT_ALIVE_CHAR : char = '█';
const DEFAULT_DEAD_CHAR : char = '░';

impl Config {
    pub fn from_matches(matches : clap::ArgMatches) -> Config {
        Config {
            width: get_width(&matches) as usize,
            height: get_height(&matches) as usize,
            delay: get_delay(&matches),
            alive: get_alive_char(&matches),
            dead: get_dead_char(&matches)
        }
    }

}

fn get_width(matches : &clap::ArgMatches) -> u32 {
    match matches.value_of("width") {
        None => DEFAULT_WIDTH,
        Some(str_val) => {
            match str_val.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Invalid width option. Must be an integer, given `{}`.", str_val);
                    exit(1);
                }
            }
        }
    }
}

fn get_height(matches : &clap::ArgMatches) -> u32 {
    match matches.value_of("height") {
        None => DEFAULT_HEIGHT,
        Some(str_val) => {
            match str_val.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Invalid height option. Must be an integer, given `{}`.", str_val);
                    exit(1);
                }
            }
        }
    }
}

fn get_delay(matches : &clap::ArgMatches) -> time::Duration {
    let number = match matches.value_of("delay") {
        None => DEFAULT_DELAY,
        Some(str_val) => {
            match str_val.parse::<u64>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Invalid delay option. Must be an integer, given `{}`.", str_val);
                    exit(1);
                }
            }
        }
    };
    time::Duration::from_millis(number)
}

fn get_alive_char(matches : &clap::ArgMatches) -> char {
    match matches.value_of("alive") {
        None => DEFAULT_ALIVE_CHAR,
        Some(str_val) => {
            str_val.chars().nth(0).unwrap_or(DEFAULT_ALIVE_CHAR)
        }
    }
}

fn get_dead_char(matches : &clap::ArgMatches) -> char {
    match matches.value_of("dead") {
        None => DEFAULT_DEAD_CHAR,
        Some(str_val) => {
            str_val.chars().nth(0).unwrap_or(DEFAULT_DEAD_CHAR)
        }
    }
}
