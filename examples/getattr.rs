extern crate termios;
extern crate native;
use termios::Termio;
use native::io::FileDesc;
fn main() {
  let fd = FileDesc::new(0, false);
  let termios = fd.tcgetattr().unwrap();
  println!("{}", termios)
}
