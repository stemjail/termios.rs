#![feature(negate_unsigned)]

extern crate libc;
#[macro_use]
extern crate bitflags;

use std::os::unix::prelude::*;
use std::io::Result as IoResult;
use std::io::Error as IoError;
pub use types::*;
use std::mem::{zeroed, transmute};
mod types;

#[allow(non_camel_case_types, dead_code)]
mod bindings;

pub trait Termio {
  fn tcgetattr(&self) -> IoResult<Termios>;
  fn tcsetattr(&self, when: When, termios: &Termios) -> IoResult<()>;
  fn tcsetattr_auto(&self, termios: &Termios) -> IoResult<TermioHandle>;
}

pub struct TermioHandle(RawFd, Termios);

impl<T> Termio for T where T: AsRawFd {
  fn tcgetattr(&self) -> IoResult<Termios> {
    let fd = self.as_raw_fd();
    let mut termios = unsafe { zeroed() };

    if unsafe { bindings::tcgetattr(fd, transmute(&mut termios)) } < 0 {
      return Err(IoError::last_os_error());
    }

    Ok(termios)
  }

  fn tcsetattr(&self, when: When, termios: &Termios) -> IoResult<()> {
   let fd = self.as_raw_fd();

   if unsafe { bindings::tcsetattr(fd, when as i32, transmute(termios)) } < 0 {
     return Err(IoError::last_os_error());
   }

   Ok(())
  }

  fn tcsetattr_auto(&self, termios: &Termios) -> IoResult<TermioHandle> {
    try!(self.tcsetattr(TCSANOW, termios));
    Ok(TermioHandle(self.as_raw_fd(), *termios))
  }
}

impl Drop for TermioHandle {
  fn drop(&mut self) {
    self.tcsetattr(TCSANOW, &self.1).unwrap();
  }
}

impl AsRawFd for TermioHandle {
  fn as_raw_fd(&self) -> RawFd { self.0 }
}
