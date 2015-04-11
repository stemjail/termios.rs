extern crate termios;

use termios::{Termio, TCSANOW, ECHO};
use std::io;
use std::io::Write;
use std::fs;

fn main() {
    let tty = fs::File::open("/dev/tty").unwrap();
    let termios = tty.tcgetattr().unwrap();
    let mut new_termios = termios.clone();
    new_termios.local_flags.remove(ECHO);
    tty.tcsetattr(TCSANOW, &new_termios).unwrap();

    let stdout = io::stdout();
    let mut reader = io::stdin();
    loop {
        let mut input = String::new();
        print!("Password: ");
        stdout.lock().flush().unwrap();
        reader.read_line(&mut input).unwrap();
        println!("");
        if input == "sesame\n" {
            break
        }
        println!("access denied");
        std::thread::sleep_ms(2_000);
    }
    println!("access granted");

    // restore terminal
    tty.tcsetattr(TCSANOW, &termios).unwrap();
}
