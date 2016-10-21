# Conway

Conway's Game of Life implemented in Rust.

The project is created for learning purposes.

![Conway's Game of Life Rust implementation](https://github.com/greyblake/conway-rs/raw/master/misc/conway.gif)

## Usage

Build executable:

```
cargo build --release
```

Run it:

```
./target/release/conway
```

## Options

Run with `--help` flag to see all the supported options:

```
OPTIONS:
    -a, --alive <CHAR>            Char to display alive cells
        --dead <CHAR>             Char to display dead cells
    -d, --delay <MILLISECONDS>    Delay between the frames in milliseconds
        --height <HEIGHT>         Set height of the board
    -w, --width <WIDTH>           Set width of the board
```

## Contributors

- [greyblake](https://github.com/greyblake) Potapov Sergey - creator, maintainer




