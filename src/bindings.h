#define _BSD_SOURCE
#include <termios.h>

enum { NCCS_ = NCCS };

enum iflags {
  IGNBRK_  = IGNBRK,
  BRKINT_  = BRKINT,
  IGNPAR_  = IGNPAR,
  PARMRK_  = PARMRK,
  INPCK_   = INPCK,
  ISTRIP_  = ISTRIP,
  INLCR_   = INLCR,
  IGNCR_   = IGNCR,
  ICRNL_   = ICRNL,
  IUCLC_   = IUCLC,
  IXON_    = IXON,
  IXANY_   = IXANY,
  IXOFF_   = IXOFF,
  IMAXBEL_ = IMAXBEL,
  IUTF8_   = IUTF8
};

enum oflags {
  OPOST_  = OPOST,
  OLCUC_  = OLCUC,
  ONLCR_  = ONLCR,
  OCRNL_  = OCRNL,
  ONOCR_  = ONOCR,
  ONLRET_ = ONLRET,
  OFILL_  = OFILL,
  OFDEL_  = OFDEL,

  NLDLY_  = NLDLY,
  NL0_    = NL0,
  NL1_    = NL1,

  CRDLY_  = CRDLY,
  CR0_    = CR0,
  CR1_    = CR1,
  CR2_    = CR2,
  CR3_    = CR3,

  TABDLY_ = TABDLY,
  TAB0_   = TAB0,
  TAB1_   = TAB1,
  TAB2_   = TAB2,
  TAB3_   = TAB3,

  BSDLY_  = BSDLY,
  BS0_    = BS0,
  BS1_    = BS1,

  VTDLY_  = VTDLY,
  VT0_    = VT0,
  VT1_    = VT1,

  FFDLY_  = FFDLY,
  FF0_    = FF0,
  FF1_    = FF1,
};

enum cflags {
  CBAUD_   = CBAUD,
  CBAUDEX_ = CBAUDEX,

  CSIZE_   = CSIZE,
  CS5_     = CS5,
  CS6_     = CS6,
  CS7_     = CS7,
  CS8_     = CS8,

  CSTOPB_  = CSTOPB,
  CREAD_   = CREAD,
  PARENB_  = PARENB,
  PARODD_  = PARODD,
  HUPCL_   = HUPCL,
  CLOCAL_  = CLOCAL,
//LOBLK_   = LOBLK,
  CIBAUD_  = CIBAUD,
  CMSPAR_  = CMSPAR,
  CRTSCTS_ = CRTSCTS,
};

enum lflags {
  ISIG_    = ISIG,
  ICANON_  = ICANON,
  XCASE_   = XCASE,
  ECHO_    = ECHO,
  ECHOE_   = ECHOE,
  ECHOK_   = ECHOK,
  ECHONL_  = ECHONL,
  ECHOCTL_ = ECHOCTL,
  ECHOPRT_ = ECHOPRT,
  ECHOKE_  = ECHOKE,
//DEFECHO_ = DEFECHO,
  FLUSHO_  = FLUSHO,
  NOFLSH_  = NOFLSH,
  TOSTOP_  = TOSTOP,
  PENDIN_  = PENDIN,
  IEXTEN_  = IEXTEN,
};

enum {
  VDISCARD_ = VDISCARD,
//VDSUSP_   = VDSUSP,
  VEOF_     = VEOF,
  VEOL_     = VEOL,
  VEOL2_    = VEOL2,
  VERASE_   = VERASE,
  VINTR_    = VINTR,
  VKILL_    = VKILL,
  VLNEXT_   = VLNEXT,
  VMIN_     = VMIN,
  VQUIT_    = VQUIT,
  VREPRINT_ = VREPRINT,
  VSTART_   = VSTART,
//VSTATUS_  = VSTATUS,
  VSTOP_    = VSTOP,
  VSUSP_    = VSUSP,
//VSWTCH_   = VSWTCH,
  VTIME_    = VTIME,
  VWERASE_  = VWERASE,
};

enum when {
  TCSANOW_   = TCSANOW,
  TCSADRAIN_ = TCSADRAIN,
  TCSAFLUSH_ = TCSAFLUSH,
};

enum baud {
  B0_       = B0,
  B50_      = B50,
  B75_      = B75,
  B110_     = B110,
  B134_     = B134,
  B150_     = B150,
  B200_     = B200,
  B300_     = B300,
  B600_     = B600,
  B1200_    = B1200,
  B1800_    = B1800,
  B2400_    = B2400,
  B4800_    = B4800,
  B9600_    = B9600,
  B19200_   = B19200,
  B38400_   = B38400,
  B57600_   = B57600,
  B115200_  = B115200,
  B230400_  = B230400,
  B460800_  = B460800,
  B500000_  = B500000,
  B576000_  = B576000,
  B921600_  = B921600,
  B1000000_ = B1000000,
  B1152000_ = B1152000,
  B1500000_ = B1500000,
  B2000000_ = B2000000,
  B2500000_ = B2500000,
  B3000000_ = B3000000,
  B3500000_ = B3500000,
  B4000000_ = B4000000,
};
