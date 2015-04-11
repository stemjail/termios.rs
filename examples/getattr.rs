extern crate termios;

use termios::Termio;
use std::fs;

fn main() {
    let tty = fs::File::open("/dev/tty").unwrap();
    let termios = tty.tcgetattr().unwrap();
    println!("{:?}", termios)
}
