use std::fmt;
use bindings::*;

pub use self::ControlCharacter::*;
pub use self::Speed::*;
pub use self::When::*;

#[derive(Copy, Clone)]
pub struct Termios {
  pub input_flags: InputFlags,
  pub output_flags: OutputFlags,
  pub control_flags: ControlFlags,
  pub local_flags: LocalFlags,
  _line: u8, // line discipline (unused on POSIX)
  pub control_chars: [u8; NCCS_ as usize],
  pub input_speed: Speed,
  pub output_speed: Speed,
}

impl fmt::Debug for Termios {
  #[allow(unused_must_use, unused_assignments)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "Termios {{ ");
    write!(fmt, "input_flags: {:?}, ", self.input_flags);
    write!(fmt, "output_flags: {:?}, ", self.output_flags);
    write!(fmt, "control_flags: {:?}, ", self.control_flags);
    write!(fmt, "local_flags: {:?}", self.local_flags);

    write!(fmt, "control_chars: {{ ");
    let ref c = self.control_chars;
    write!(fmt, "VINTR: {:?}, ", c[VINTR as usize]);
    write!(fmt, "VQUIT: {:?}, ", c[VQUIT as usize]);
    write!(fmt, "VERASE: {:?}, ", c[VERASE as usize]);
    write!(fmt, "VKILL: {:?}, ", c[VKILL as usize]);
    write!(fmt, "VEOF: {:?}, ", c[VEOF as usize]);
    write!(fmt, "VTIME: {:?}, ", c[VTIME as usize]);
    write!(fmt, "VMIN: {:?}, ", c[VMIN as usize]);
    write!(fmt, "VSTART: {:?}, ", c[VSTART as usize]);
    write!(fmt, "VSTOP: {:?}, ", c[VSTOP as usize]);
    write!(fmt, "VSUSP: {:?}, ", c[VSUSP as usize]);
    write!(fmt, "VEOL: {:?}, ", c[VEOL as usize]);
    write!(fmt, "VREPRINT: {:?}, ", c[VREPRINT as usize]);
    write!(fmt, "VDISCARD: {:?}, ", c[VDISCARD as usize]);
    write!(fmt, "VWERASE: {:?}, ", c[VWERASE as usize]);
    write!(fmt, "VLNEXT: {:?}, ", c[VLNEXT as usize]);
    write!(fmt, "VEOL2: {:?} }}", c[VEOL2 as usize]);

    write!(fmt, " }}");

    Ok(())
  }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ControlCharacter {
  VINTR    = VINTR_,
  VQUIT    = VQUIT_,
  VERASE   = VERASE_,
  VKILL    = VKILL_,
  VEOF     = VEOF_,
  VTIME    = VTIME_,
  VMIN     = VMIN_,
//VSWTC    = VSWTC_,
  VSTART   = VSTART_,
  VSTOP    = VSTOP_,
  VSUSP    = VSUSP_,
  VEOL     = VEOL_,
  VREPRINT = VREPRINT_,
  VDISCARD = VDISCARD_,
  VWERASE  = VWERASE_,
  VLNEXT   = VLNEXT_,
  VEOL2    = VEOL2_,
}

bitflags!{
  flags InputFlags: Enum_iflags {
    // Ignore BREAK condition on input.
    const IGNBRK  = IGNBRK_,
    // If  IGNBRK is set, a BREAK is ignored. If it is not set but BRKINT is set, then a BREAK
    // causes the input and output queues to be flushed, and if the terminal is the controlling
    // terminal of a foreground process group, it will cause a SIGINT to be sent to this foreground
    // process group. When neither IGNBRK nor BRKINT are set, a BREAK reads as a null byte ('\0'),
    // except when PARMRK is set, in which case it reads as the sequence \377 \0 \0.
    const BRKINT  = BRKINT_,
    // Ignore framing errors and parity errors.
    const IGNPAR  = IGNPAR_,
    // If IGNPAR is not set, prefix a character with a parity error or framing error with \377 \0.
    // If neither IGNPAR nor PARMRK is set, read a character with a parity error or framing error
    // as \0.
    const PARMRK  = PARMRK_,
    // Enable input parity checking.
    const INPCK   = INPCK_,
    // Strip off eighth bit.
    const ISTRIP  = ISTRIP_,
    // Translate NL to CR on input.
    const INLCR   = INLCR_,
    // Ignore carriage return on input.
    const IGNCR   = IGNCR_,
    // Translate carriage return to newline on input (unless IGNCR is set).
    const ICRNL   = ICRNL_,
    // (not in POSIX) Map uppercase characters to lowercase on input.
    const IUCLC   = IUCLC_,
    // Enable XON/XOFF flow control on output.
    const IXON    = IXON_,
    // (XSI) Typing any character will restart stopped output.  (The default is to allow just the
    // START character to restart output.)
    const IXANY   = IXANY_,
    // Enable XON/XOFF flow control on input.
    const IXOFF   = IXOFF_,
    // (not in POSIX) Ring bell when input queue is full.  Linux does not implement this bit, and
    // acts as if it is always set.
    const IMAXBEL = IMAXBEL_,
    // (since Linux 2.6.4) (not in POSIX) Input is UTF8; this allows character-erase to be
    // correctly performed in cooked mode.
    const IUTF8   = IUTF8_
  }
}


impl fmt::Debug for InputFlags {
  #[allow(unused_must_use, unused_assignments)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
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
  flags OutputFlags: Enum_oflags {
    // Enable implementation-defined output processing.
    const OPOST  = OPOST_,
    // (not in POSIX) Map lowercase characters to uppercase on output.
    const OLCUC  = OLCUC_,
    // (XSI) Map NL to CR-NL on output.
    const ONLCR  = ONLCR_,
    // Map CR to NL on output.
    const OCRNL  = OCRNL_,
    // Don't output CR at column 0.
    const ONOCR  = ONOCR_,
    // Don't output CR.
    const ONLRET = ONLRET_,

    // Send fill characters for a delay, rather than using a timed delay.
    const OFILL  = OFILL_,
    // Fill character is ASCII DEL (0177).  If unset, fill character is ASCII NUL ('\0').  (Not implemented on Linux.)
    const OFDEL  = OFDEL_,

    // newline delay
    const NL0    = NL0_,
    const NL1    = NL1_,

    // carriage return delay
    const CR0    = CR0_,
    const CR1    = CR1_,
    const CR2    = CR2_,
    const CR3    = CR3_,

    // tab delay
    const TAB0   = TAB0_,
    const TAB1   = TAB1_,
    const TAB2   = TAB2_,
    const TAB3   = TAB3_,

    // backspace delay
    const BS0    = BS0_,
    const BS1    = BS1_,

    // form feed delay
    const FF0    = FF0_,
    const FF1    = FF1_,

    // vertical tab delay
    const VT0    = VT0_,
    const VT1    = VT1_
  }
}

impl fmt::Debug for OutputFlags {
  #[allow(unused_must_use, unused_assignments)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
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
  flags ControlFlags: Enum_cflags {
    // character size
    const CS5 = CS5_,
    const CS6 = CS6_,
    const CS7 = CS7_,
    const CS8 = CS8_,

    // Set two stop bits, rather than one.
    const CSTOPB = CSTOPB_,
    // Enable receiver.
    const CREAD = CREAD_,
    // Enable parity generation on output and parity checking for input.
    const PARENB = PARENB_,
    // If set, then parity for input and output is odd; otherwise even parity is used.
    const PARODD = PARODD_,
    // Lower modem control lines after last process closes the device (hang up).
    const HUPCL = HUPCL_,
    // Ignore modem control lines.
    const CLOCAL = CLOCAL_,

    // baud
    const CBAUD = CBAUD_,
    // extended baud
    const CBAUDEX = CBAUDEX_,

    // (not  in  POSIX) Mask for input speeds.  The values for the CIBAUD bits are the same as the
    // values for the CBAUD bits, shifted left IBSHIFT bits. (Not implemented on Linux.)
    const CIBAUD = CIBAUD_,

    // (not in POSIX) Use "stick" (mark/space) parity (supported on certain serial devices): if
    // PARODD is set, the parity bit is always 1; if PARODD is not set, then the parity  bit  is
    // always  0.
    const CMSPAR = CMSPAR_,
    // (not in POSIX) Enable RTS/CTS (hardware) flow control.
    const CRTSCTS = CRTSCTS_
  }
}

impl fmt::Debug for ControlFlags {
  #[allow(unused_must_use, unused_assignments)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
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
  flags LocalFlags: Enum_lflags {
    // When any of the characters INTR, QUIT, SUSP, or DSUSP are received, generate the
    // corresponding signal.
    const ISIG = ISIG_,
    // Enable canonical mode
    const ICANON = ICANON_,
    // (not  in  POSIX; not supported under Linux) If ICANON is also set, terminal is uppercase
    // only.  Input is converted to lowercase, except for characters preceded by \.  On output,
    // uppercase characters are preceded by \ and lowercase characters are converted to uppercase
    const XCASE = XCASE_,
    // Echo input characters.
    const ECHO = ECHO_,
    // If ICANON is also set, the ERASE character erases the preceding input character, and WERASE
    // erases the preceding word.
    const ECHOE = ECHOE_,
    // If ICANON is also set, the KILL character erases the current line.
    const ECHOK = ECHOK_,
    // If ICANON is also set, echo the NL character even if ECHO is not set.
    const ECHONL = ECHONL_,
    // Disable flushing the input and output queues when generating signals for the INT, QUIT, and
    // SUSP characters.
    const NOFLSH = NOFLSH_,
    // Send the SIGTTOU signal to the process group of a background process which tries to write to
    // its controlling terminal.
    const TOSTOP = TOSTOP_,
    // (not in POSIX) If ECHO is also set, terminal special characters other than TAB, NL, START,
    // and STOP are echoed as ^X, where X is the character with ASCII code 0x40 greater than the
    // special character. For example, character 0x08 (BS) is echoed as ^H.
    const ECHOCTL = ECHOCTL_,
    // (not in POSIX) If ICANON and ECHO are also set, characters are printed as they are being
    // erased.
    const ECHOPRT = ECHOPRT_,
    // (not in POSIX) If ICANON is also set, KILL is echoed by erasing each character on the line,
    // as specified by ECHOE and ECHOPRT.
    const ECHOKE = ECHOKE_,
    // (not in POSIX; not supported under Linux) Output is being flushed.  This flag is toggled by
    // typing the DISCARD character.
    const FLUSHO = FLUSHO_,
    // (not in POSIX; not supported under Linux) All characters in the input queue are reprinted
    // when the next character is read.  (bash(1) handles typeahead this way.)
    const PENDIN = PENDIN_,
    // Enable  implementation-defined  input  processing.   This  flag, as well as ICANON must be
    // enabled for the special characters EOL2, LNEXT, REPRINT, WERASE to be interpreted, and for
    // the IUCLC flag to be effective.
    const IEXTEN = IEXTEN_

    /* const EXTPROC = EXTPROC_, */
  }
}

impl fmt::Debug for LocalFlags {
  #[allow(unused_must_use, unused_assignments)]
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Speed {
  B0       = B0_,
  B50      = B50_,
  B75      = B75_,
  B110     = B110_,
  B134     = B134_,
  B150     = B150_,
  B200     = B200_,
  B300     = B300_,
  B600     = B600_,
  B1200    = B1200_,
  B1800    = B1800_,
  B2400    = B2400_,
  B4800    = B4800_,
  B9600    = B9600_,
  B19200   = B19200_,
  B38400   = B38400_,
  B57600   = B57600_,
  B115200  = B115200_,
  B230400  = B230400_,
  B460800  = B460800_,
  B500000  = B500000_,
  B576000  = B576000_,
  B921600  = B921600_,
  B1000000 = B1000000_,
  B1152000 = B1152000_,
  B1500000 = B1500000_,
  B2000000 = B2000000_,
  B2500000 = B2500000_,
  B3000000 = B3000000_,
  B3500000 = B3500000_,
  B4000000 = B4000000_,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum When {
  TCSANOW   = TCSANOW_,
  TCSADRAIN = TCSADRAIN_,
  TCSAFLUSH = TCSAFLUSH_,
}
