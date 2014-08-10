use libc::c_int;
use types::{Termios, When};
extern {
  pub fn tcgetattr(fd: c_int, termios: &mut Termios) -> c_int;
  pub fn tcsetattr(fd: c_int, optional_actions: When, termios: &Termios) -> c_int;
}
