use std::fmt;

#[deriving(Show)]
pub struct Termios {
  pub input_flags: InputFlags,
  pub output_flags: OutputFlags,
  pub control_flags: ControlFlags,
  pub local_flags: LocalFlags,
  _line: u8, // line discipline (unused on POSIX)
  pub control_chars: ControlCharacters,
  pub input_speed: Speed,
  pub output_speed: Speed,
}

#[deriving(Show)]
#[packed]
#[allow(uppercase_variables)]
pub struct ControlCharacters {
  pub VINTR    : u8,
  pub VQUIT    : u8,
  pub VERASE   : u8,
  pub VKILL    : u8,
  pub VEOF     : u8,
  pub VTIME    : u8,
  pub VMIN     : u8,
  pub VSWTC    : u8,
  pub VSTART   : u8,
  pub VSTOP    : u8,
  pub VSUSP    : u8,
  pub VEOL     : u8,
  pub VREPRINT : u8,
  pub VDISCARD : u8,
  pub VWERASE  : u8,
  pub VLNEXT   : u8,
  pub VEOL2    : u8,
}

bitflags!{
  flags InputFlags: libc::c_ulong {
    // Ignore BREAK condition on input.
    static IGNBRK  = 0x1,
    // If  IGNBRK is set, a BREAK is ignored. If it is not set but BRKINT is set, then a BREAK
    // causes the input and output queues to be flushed, and if the terminal is the controlling
    // terminal of a foreground process group, it will cause a SIGINT to be sent to this foreground
    // process group. When neither IGNBRK nor BRKINT are set, a BREAK reads as a null byte ('\0'),
    // except when PARMRK is set, in which case it reads as the sequence \377 \0 \0.
    static BRKINT  = 0x2,
    // Ignore framing errors and parity errors.
    static IGNPAR  = 0x4,
    // If IGNPAR is not set, prefix a character with a parity error or framing error with \377 \0.
    // If neither IGNPAR nor PARMRK is set, read a character with a parity error or framing error
    // as \0.
    static PARMRK  = 0x8,
    // Enable input parity checking.
    static INPCK   = 0x10,
    // Strip off eighth bit.
    static ISTRIP  = 0x20,
    // Translate NL to CR on input.
    static INLCR   = 0x40,
    // Ignore carriage return on input.
    static IGNCR   = 0x80,
    // Translate carriage return to newline on input (unless IGNCR is set).
    static ICRNL   = 0x100,
    // (not in POSIX) Map uppercase characters to lowercase on input.
    static IUCLC   = 0x200,
    // Enable XON/XOFF flow control on output.
    static IXON    = 0x400,
    // (XSI) Typing any character will restart stopped output.  (The default is to allow just the
    // START character to restart output.)
    static IXANY   = 0x800,
    // Enable XON/XOFF flow control on input.
    static IXOFF   = 0x1000,
    // (not in POSIX) Ring bell when input queue is full.  Linux does not implement this bit, and
    // acts as if it is always set.
    static IMAXBEL = 0x2000,
    // (since Linux 2.6.4) (not in POSIX) Input is UTF8; this allows character-erase to be
    // correctly performed in cooked mode.
    static IUTF8   = 0x4000
  }
}


impl fmt::Show for InputFlags {
  #[allow(unused_must_use, dead_assignment)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
    let mut first = true;
    fmt.write_str("InputFlags {");

    if self.contains(IGNBRK) {
      if first { first = false; } else { fmt.write_str(", "); }
      fmt.write_str("IGNBRK");
    }

    if self.contains(BRKINT) {
      if first { first = false; } else { fmt.write_str(", "); }
      fmt.write_str("BRKINT");
    }

    if self.contains(IGNPAR) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IGNPAR");
    }

    if self.contains(PARMRK) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("PARMRK");
    }

    if self.contains(INPCK) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("INPCK");
    }

    if self.contains(ISTRIP) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ISTRIP");
    }

    if self.contains(INLCR) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("INLCR");
    }

    if self.contains(IGNCR) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IGNCR");
    }

    if self.contains(ICRNL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ICRNL");
    }

    if self.contains(IUCLC) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IUCLC");
    }

    if self.contains(IXON) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IXON");
    }

    if self.contains(IXANY) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IXANY");
    }

    if self.contains(IXOFF) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IXOFF");
    }

    if self.contains(IMAXBEL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IMAXBEL");
    }

    if self.contains(IUTF8) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IUTF8");
    }

    fmt.write_str("}");
    Ok(())
  }
}

bitflags!{
  flags OutputFlags: libc::c_ulong {
    // Enable implementation-defined output processing.
    static OPOST  = 0x1,
    // (not in POSIX) Map lowercase characters to uppercase on output.
    static OLCUC  = 0x2,
    // (XSI) Map NL to CR-NL on output.
    static ONLCR  = 0x4,
    // Map CR to NL on output.
    static OCRNL  = 0x8,
    // Don't output CR at column 0.
    static ONOCR  = 0x10,
    // Don't output CR.
    static ONLRET = 0x20,

    // Send fill characters for a delay, rather than using a timed delay.
    static OFILL  = 0x40,
    // Fill character is ASCII DEL (0177).  If unset, fill character is ASCII NUL ('\0').  (Not implemented on Linux.)
    static OFDEL  = 0x80,

    // newline delay
    static NL0    = 0x0,
    static NL1    = 0x100,

    // carriage return delay
    static CR0    = 0x0,
    static CR1    = 0x200,
    static CR2    = 0x400,
    static CR3    = 0x600,

    // tab delay
    static TAB0   = 0x0,
    static TAB1   = 0x800,
    static TAB2   = 0x1000,
    static TAB3   = 0x1800,

    // backspace delay
    static BS0    = 0x0,
    static BS1    = 0x2000,

    // form feed delay
    static FF0    = 0x0,
    static FF1    = 0x8000,

    // vertical tab delay
    static VT0    = 0x0,
    static VT1    = 0x4000
  }
}

impl fmt::Show for OutputFlags {
  #[allow(unused_must_use, dead_assignment)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
    let mut first = true;
    fmt.write_str("OutputFlags {");

    if self.contains(OPOST) {
      if first { first = false; } else { fmt.write_str(", "); }
      fmt.write_str("OPOST");
    }

    if self.contains(OLCUC) {
      if first { first = false; } else { fmt.write_str(", "); }
      fmt.write_str("OLCUC");
    }

    if self.contains(ONLCR) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ONLCR");
    }

    if self.contains(OCRNL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("OCRNL");
    }

    if self.contains(ONOCR) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ONOCR");
    }

    if self.contains(ONLRET) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ONLRET");
    }

    if self.contains(OFILL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("OFILL");
    }

    if first { first = false } else { fmt.write_str(", "); }
    fmt.write_str(if self.contains(NL1) { "NL1" } else { "NL0" });

    {
      let mut cr = false;

      if self.contains(CR1) {
        if first { first = false } else { fmt.write_str(", "); }
        cr = true;
        fmt.write_str("CR1");
      }

      if self.contains(CR2) {
        if first { first = false } else { fmt.write_str(", "); }
        cr = true;
        fmt.write_str("CR2");
      }

      if self.contains(CR3) {
        if first { first = false } else { fmt.write_str(", "); }
        cr = true;
        fmt.write_str("CR3");
      }

      if !cr {
        if first { first = false } else { fmt.write_str(", "); }
        fmt.write_str("CR0");
      }
    }


    {
      let mut tab = false;

      if self.contains(TAB1) {
        if first { first = false } else { fmt.write_str(", "); }
        tab = true;
        fmt.write_str("TAB1");
      }

      if self.contains(TAB2) {
        if first { first = false } else { fmt.write_str(", "); }
        tab = true;
        fmt.write_str("TAB2");
      }

      if self.contains(TAB3) {
        if first { first = false } else { fmt.write_str(", "); }
        tab = true;
        fmt.write_str("TAB3");
      }

      if !tab {
        if first { first = false } else { fmt.write_str(", "); }
        fmt.write_str("TAB0");
      }
    }

    if first { first = false } else { fmt.write_str(", "); }
    fmt.write_str(if self.contains(BS1) { "BS1" } else { "BS0" } );

    if first { first = false } else { fmt.write_str(", "); }
    fmt.write_str(if self.contains(FF1) { "FF1" } else { "FF0" } );

    if first { first = false } else { fmt.write_str(", "); }
    fmt.write_str(if self.contains(VT1) { "VT1" } else { "VT0" } );

    fmt.write_str("}");
    Ok(())
  }
}


bitflags!{
  flags ControlFlags: libc::c_ulong {
    // character size
    static CS5 = 0x0,
    static CS6 = 0x10,
    static CS7 = 0x20,
    static CS8 = 0x30,

    // Set two stop bits, rather than one.
    static CSTOPB = 0x40,
    // Enable receiver.
    static CREAD = 0x80,
    // Enable parity generation on output and parity checking for input.
    static PARENB = 0x100,
    // If set, then parity for input and output is odd; otherwise even parity is used.
    static PARODD = 0x200,
    // Lower modem control lines after last process closes the device (hang up).
    static HUPCL = 0x400,
    // Ignore modem control lines.
    static CLOCAL = 0x800,

    // baud
    static CBAUD = 0x100f,
    // extended baud
    static CBAUDEX = 0x1000,

    // (not  in  POSIX) Mask for input speeds.  The values for the CIBAUD bits are the same as the
    // values for the CBAUD bits, shifted left IBSHIFT bits. (Not implemented on Linux.)
    static CIBAUD = 0x100f0000,

    // (not in POSIX) Use "stick" (mark/space) parity (supported on certain serial devices): if
    // PARODD is set, the parity bit is always 1; if PARODD is not set, then the parity  bit  is
    // always  0.
    static CMSPAR = 0x40000000,
    // (not in POSIX) Enable RTS/CTS (hardware) flow control.
    static CRTSCTS = 0x80000000
  }
}

impl fmt::Show for ControlFlags {
  #[allow(unused_must_use, dead_assignment)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
    let mut first = true;
    fmt.write_str("ControlFlags {");

    {
      let mut cs = false;

      if self.contains(CS6) {
        if first { first = false } else { fmt.write_str(", "); }
        cs = true;
        fmt.write_str("CS6");
      }

      if self.contains(CS7) {
        if first { first = false } else { fmt.write_str(", "); }
        cs = true;
        fmt.write_str("CS7");
      }

      if self.contains(CS8) {
        if first { first = false } else { fmt.write_str(", "); }
        cs = true;
        fmt.write_str("CS8");
      }

      if !cs {
        if first { first = false } else { fmt.write_str(", "); }
        fmt.write_str("CS5");
      }
    }

    if self.contains(CSTOPB) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("CSTOPB");
    }

    if self.contains(CREAD) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("CREAD");
    }

    if self.contains(PARENB) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("PARENB");
    }

    if self.contains(PARODD) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("PARODD");
    }

    if self.contains(HUPCL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("HUPCL");
    }

    if self.contains(CLOCAL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("CLOCAL");
    }

    // CBAUD, CBAUDEX, CIBAUD?

   if self.contains(CMSPAR) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("CMSPAR");
    }

    if self.contains(CRTSCTS) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("CRTSCTS");
    }

    fmt.write_str("}");
    Ok(())
  }
}

bitflags!{
  flags LocalFlags: libc::c_ulong {
    // When any of the characters INTR, QUIT, SUSP, or DSUSP are received, generate the
    // corresponding signal.
    static ISIG = 0x1,
    // Enable canonical mode
    static ICANON = 0x2,
    // (not  in  POSIX; not supported under Linux) If ICANON is also set, terminal is uppercase
    // only.  Input is converted to lowercase, except for characters preceded by \.  On output,
    // uppercase characters are preceded by \ and lowercase characters are converted to uppercase
    static XCASE = 0x4,
    // Echo input characters.
    static ECHO = 0x8,
    // If ICANON is also set, the ERASE character erases the preceding input character, and WERASE
    // erases the preceding word.
    static ECHOE = 0x10,
    // If ICANON is also set, the KILL character erases the current line.
    static ECHOK = 0x20,
    // If ICANON is also set, echo the NL character even if ECHO is not set.
    static ECHONL = 0x40,
    // Disable flushing the input and output queues when generating signals for the INT, QUIT, and
    // SUSP characters.
    static NOFLSH = 0x80,
    // Send the SIGTTOU signal to the process group of a background process which tries to write to
    // its controlling terminal.
    static TOSTOP = 0x100,
    // (not in POSIX) If ECHO is also set, terminal special characters other than TAB, NL, START,
    // and STOP are echoed as ^X, where X is the character with ASCII code 0x40 greater than the
    // special character. For example, character 0x08 (BS) is echoed as ^H.
    static ECHOCTL = 0x200,
    // (not in POSIX) If ICANON and ECHO are also set, characters are printed as they are being
    // erased.
    static ECHOPRT = 0x400,
    // (not in POSIX) If ICANON is also set, KILL is echoed by erasing each character on the line,
    // as specified by ECHOE and ECHOPRT.
    static ECHOKE = 0x800,
    // (not in POSIX; not supported under Linux) Output is being flushed.  This flag is toggled by
    // typing the DISCARD character.
    static FLUSHO = 0x1000,
    // (not in POSIX; not supported under Linux) All characters in the input queue are reprinted
    // when the next character is read.  (bash(1) handles typeahead this way.)
    static PENDIN = 0x4000,
    // Enable  implementation-defined  input  processing.   This  flag, as well as ICANON must be
    // enabled for the special characters EOL2, LNEXT, REPRINT, WERASE to be interpreted, and for
    // the IUCLC flag to be effective.
    static IEXTEN = 0x8000
    // wtf?
    /* static EXTPROC = 0x10000, */
  }
}

impl fmt::Show for LocalFlags {
  #[allow(unused_must_use, dead_assignment)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
    let mut first = true;
    fmt.write_str("LocalFlags {");

    if self.contains(ISIG) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ISIG");
    }

    if self.contains(ICANON) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ICANON");
    }

    if self.contains(XCASE) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("XCASE");
    }

    if self.contains(ECHO) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ECHO");
    }

    if self.contains(ECHOE) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ECHOE");
    }

    if self.contains(ECHOK) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ECHOK");
    }

    if self.contains(ECHONL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ECHONL");
    }

    if self.contains(NOFLSH) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("NOFLSH");
    }

    if self.contains(TOSTOP) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("TOSTOP");
    }

    if self.contains(ECHOCTL) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ECHOCTL");
    }

    if self.contains(ECHOPRT) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ECHOPRT");
    }

    if self.contains(ECHOKE) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("ECHOKE");
    }

    if self.contains(FLUSHO) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("FLUSHO");
    }

    if self.contains(PENDIN) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("PENDIN");
    }

    if self.contains(IEXTEN) {
      if first { first = false } else { fmt.write_str(", "); }
      fmt.write_str("IEXTEN");
    }

    fmt.write_str("}");
    Ok(())
  }
}

#[deriving(Show)]
#[repr(uint)]
pub enum Speed {
  B0       = 0x0,
  B50      = 0x1,
  B75      = 0x2,
  B110     = 0x3,
  B134     = 0x4,
  B150     = 0x5,
  B200     = 0x6,
  B300     = 0x7,
  B600     = 0x8,
  B1200    = 0x9,
  B1800    = 0xa,
  B2400    = 0xb,
  B4800    = 0xc,
  B9600    = 0xd,
  B19200   = 0xe,
  B38400   = 0xf,
  B57600   = 0x1001,
  B115200  = 0x1002,
  B230400  = 0x1003,
  B460800  = 0x1004,
  B500000  = 0x1005,
  B576000  = 0x1006,
  B921600  = 0x1007,
  B1000000 = 0x1008,
  B1152000 = 0x1009,
  B1500000 = 0x100a,
  B2000000 = 0x100b,
  B2500000 = 0x100c,
  B3000000 = 0x100d,
  B3500000 = 0x100e,
  B4000000 = 0x100f,
}

#[repr(C)]
pub enum When {
  TCSANOW = 0x0,
  TCSADRAIN = 0x1,
  TCSAFLUSH = 0x2,
}
