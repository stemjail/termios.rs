extern crate termios;

use termios::Termio;
use std::old_io as io;
use std::os::unix::AsRawFd;
use std::path::Path;

fn main() {
    let mut tty = io::File::open(&Path::new("/dev/tty")).unwrap();
    let termios = tty.tcgetattr().unwrap();
    println!("{:?}", termios)
}
