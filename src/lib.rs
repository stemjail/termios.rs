#![feature(globs)]
extern crate libc;
extern crate native;

use native::io::FileDesc;
use std::io::{IoResult, IoError};
pub use types::*;
use std::mem::zeroed;
mod types;
mod bindings;

trait Termio {
  fn tcgetattr(&self) -> IoResult<Termios>;
  fn tcsetattr(&mut self, when: When, termios: Termios) -> IoResult<()>;
}

impl Termio for FileDesc {
  fn tcgetattr(&self) -> IoResult<Termios> {
    let fd = self.fd();
    let mut termios = unsafe { zeroed() };

    if unsafe { bindings::tcgetattr(fd, &mut termios) } < 0 {
      return Err(IoError::last_error());
    }

    Ok(termios)
  }

  fn tcsetattr(&mut self, when: When, termios: Termios) -> IoResult<()> {
   let fd = self.fd();

   if unsafe { bindings::tcsetattr(fd, when, &termios) } < 0 {
     return Err(IoError::last_error());
   }

   Ok(())
  }
}
