#![feature(globs)]
#![feature(unsafe_destructor)]
extern crate libc;
extern crate native;

use native::io::FileDesc;
use std::io::{IoResult, IoError};
pub use types::*;
use std::mem::{zeroed, transmute};
#[allow(visible_private_types)]
mod types;
#[allow(non_camel_case_types, dead_code)]
mod bindings;

pub trait Termio {
  fn tcgetattr(&self) -> IoResult<Termios>;
  fn tcsetattr(&self, when: When, termios: Termios) -> IoResult<()>;
  fn tcsetattr_auto(&self, termios: Termios) -> IoResult<TermioHandle>;
}

pub struct TermioHandle<'a>(&'a Termio, Termios);

impl Termio for FileDesc {
  fn tcgetattr(&self) -> IoResult<Termios> {
    let fd = self.fd();
    let mut termios = unsafe { zeroed() };

    if unsafe { bindings::tcgetattr(fd, transmute(&mut termios)) } < 0 {
      return Err(IoError::last_error());
    }

    Ok(termios)
  }

  fn tcsetattr(&self, when: When, termios: Termios) -> IoResult<()> {
   let fd = self.fd();

   if unsafe { bindings::tcsetattr(fd, when as i32, transmute(&termios)) } < 0 {
     return Err(IoError::last_error());
   }

   Ok(())
  }

  fn tcsetattr_auto(&self, termios: Termios) -> IoResult<TermioHandle> {
    try!(self.tcsetattr(TCSANOW, termios));
    Ok(TermioHandle(self, termios))
  }
}

#[unsafe_destructor]
impl<'a> Drop for TermioHandle<'a> {
  fn drop(&mut self) {
    let &TermioHandle(termio, termios) = self;
    termio.tcsetattr(TCSANOW, termios).unwrap();
  }
}
