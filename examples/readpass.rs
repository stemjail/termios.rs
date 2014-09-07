extern crate termios;
extern crate native;
use termios::{Termio, TCSANOW, ECHO};
use native::io::FileDesc;
use std::io;
use std::io::timer::sleep;
use std::time::duration::Duration;

fn main() {
  let fd = FileDesc::new(0, false);
  let mut termios = fd.tcgetattr().unwrap();
  termios.local_flags.remove(ECHO);
  fd.tcsetattr(TCSANOW, &termios).unwrap();

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
  println!("access granted")
}
