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
  flags InputFlags: uint {
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

bitflags!{
  flags OutputFlags: uint {
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

bitflags!{
  flags ControlFlags: uint {
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

bitflags!{
  flags LocalFlags: uint {
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
