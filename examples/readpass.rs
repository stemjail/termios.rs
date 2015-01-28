extern crate termios;

use termios::{Termio, TCSANOW, ECHO};
use std::old_io as io;
use std::os::unix::AsRawFd;
use std::path::Path;
use std::old_io::timer::sleep;
use std::time::duration::Duration;

fn main() {
    let mut tty = io::File::open(&Path::new("/dev/tty")).unwrap();
    let termios = tty.tcgetattr().unwrap();
    let mut new_termios = termios.clone();
    new_termios.local_flags.remove(ECHO);
    tty.tcsetattr(TCSANOW, &new_termios).unwrap();

    let mut reader = io::stdin();
    loop {
        print!("Password: ");
        let input = reader.read_line().unwrap();
        println!("");
        if input.as_slice() == "sesame\n" {
            break
        }
        println!("access denied");
        sleep(Duration::seconds(2));
    }
    println!("access granted");

    // restore terminal
    tty.tcsetattr(TCSANOW, &termios).unwrap();
}
