/* automatically generated by rust-bindgen */

pub type __u_char = ::libc::c_uchar;
pub type __u_short = ::libc::c_ushort;
pub type __u_int = ::libc::c_uint;
pub type __u_long = ::libc::c_ulong;
pub type __int8_t = ::libc::c_char;
pub type __uint8_t = ::libc::c_uchar;
pub type __int16_t = ::libc::c_short;
pub type __uint16_t = ::libc::c_ushort;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_long;
pub type __uint64_t = ::libc::c_ulong;
pub type __quad_t = ::libc::c_long;
pub type __u_quad_t = ::libc::c_ulong;
pub type __dev_t = ::libc::c_ulong;
pub type __uid_t = ::libc::c_uint;
pub type __gid_t = ::libc::c_uint;
pub type __ino_t = ::libc::c_ulong;
pub type __ino64_t = ::libc::c_ulong;
pub type __mode_t = ::libc::c_uint;
pub type __nlink_t = ::libc::c_ulong;
pub type __off_t = ::libc::c_long;
pub type __off64_t = ::libc::c_long;
pub type __pid_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub __val: [::libc::c_int; 2usize],
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __fsid_t = Struct_Unnamed1;
pub type __clock_t = ::libc::c_long;
pub type __rlim_t = ::libc::c_ulong;
pub type __rlim64_t = ::libc::c_ulong;
pub type __id_t = ::libc::c_uint;
pub type __time_t = ::libc::c_long;
pub type __useconds_t = ::libc::c_uint;
pub type __suseconds_t = ::libc::c_long;
pub type __daddr_t = ::libc::c_int;
pub type __key_t = ::libc::c_int;
pub type __clockid_t = ::libc::c_int;
pub type __timer_t = *mut ::libc::c_void;
pub type __blksize_t = ::libc::c_long;
pub type __blkcnt_t = ::libc::c_long;
pub type __blkcnt64_t = ::libc::c_long;
pub type __fsblkcnt_t = ::libc::c_ulong;
pub type __fsblkcnt64_t = ::libc::c_ulong;
pub type __fsfilcnt_t = ::libc::c_ulong;
pub type __fsfilcnt64_t = ::libc::c_ulong;
pub type __fsword_t = ::libc::c_long;
pub type __ssize_t = ::libc::c_long;
pub type __syscall_slong_t = ::libc::c_long;
pub type __syscall_ulong_t = ::libc::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::libc::c_char;
pub type __intptr_t = ::libc::c_long;
pub type __socklen_t = ::libc::c_uint;
pub type pid_t = __pid_t;
pub type cc_t = ::libc::c_uchar;
pub type speed_t = ::libc::c_uint;
pub type tcflag_t = ::libc::c_uint;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32usize],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
impl ::std::clone::Clone for Struct_termios {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_termios {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const NCCS_: ::libc::c_uint = 32;
pub type Enum_iflags = ::libc::c_uint;
pub const IGNBRK_: ::libc::c_uint = 1;
pub const BRKINT_: ::libc::c_uint = 2;
pub const IGNPAR_: ::libc::c_uint = 4;
pub const PARMRK_: ::libc::c_uint = 8;
pub const INPCK_: ::libc::c_uint = 16;
pub const ISTRIP_: ::libc::c_uint = 32;
pub const INLCR_: ::libc::c_uint = 64;
pub const IGNCR_: ::libc::c_uint = 128;
pub const ICRNL_: ::libc::c_uint = 256;
pub const IUCLC_: ::libc::c_uint = 512;
pub const IXON_: ::libc::c_uint = 1024;
pub const IXANY_: ::libc::c_uint = 2048;
pub const IXOFF_: ::libc::c_uint = 4096;
pub const IMAXBEL_: ::libc::c_uint = 8192;
pub const IUTF8_: ::libc::c_uint = 16384;
pub type Enum_oflags = ::libc::c_uint;
pub const OPOST_: ::libc::c_uint = 1;
pub const OLCUC_: ::libc::c_uint = 2;
pub const ONLCR_: ::libc::c_uint = 4;
pub const OCRNL_: ::libc::c_uint = 8;
pub const ONOCR_: ::libc::c_uint = 16;
pub const ONLRET_: ::libc::c_uint = 32;
pub const OFILL_: ::libc::c_uint = 64;
pub const OFDEL_: ::libc::c_uint = 128;
pub const NLDLY_: ::libc::c_uint = 256;
pub const NL0_: ::libc::c_uint = 0;
pub const NL1_: ::libc::c_uint = 256;
pub const CRDLY_: ::libc::c_uint = 1536;
pub const CR0_: ::libc::c_uint = 0;
pub const CR1_: ::libc::c_uint = 512;
pub const CR2_: ::libc::c_uint = 1024;
pub const CR3_: ::libc::c_uint = 1536;
pub const TABDLY_: ::libc::c_uint = 6144;
pub const TAB0_: ::libc::c_uint = 0;
pub const TAB1_: ::libc::c_uint = 2048;
pub const TAB2_: ::libc::c_uint = 4096;
pub const TAB3_: ::libc::c_uint = 6144;
pub const BSDLY_: ::libc::c_uint = 8192;
pub const BS0_: ::libc::c_uint = 0;
pub const BS1_: ::libc::c_uint = 8192;
pub const VTDLY_: ::libc::c_uint = 16384;
pub const VT0_: ::libc::c_uint = 0;
pub const VT1_: ::libc::c_uint = 16384;
pub const FFDLY_: ::libc::c_uint = 32768;
pub const FF0_: ::libc::c_uint = 0;
pub const FF1_: ::libc::c_uint = 32768;
pub type Enum_cflags = ::libc::c_uint;
pub const CBAUD_: ::libc::c_uint = 4111;
pub const CBAUDEX_: ::libc::c_uint = 4096;
pub const CSIZE_: ::libc::c_uint = 48;
pub const CS5_: ::libc::c_uint = 0;
pub const CS6_: ::libc::c_uint = 16;
pub const CS7_: ::libc::c_uint = 32;
pub const CS8_: ::libc::c_uint = 48;
pub const CSTOPB_: ::libc::c_uint = 64;
pub const CREAD_: ::libc::c_uint = 128;
pub const PARENB_: ::libc::c_uint = 256;
pub const PARODD_: ::libc::c_uint = 512;
pub const HUPCL_: ::libc::c_uint = 1024;
pub const CLOCAL_: ::libc::c_uint = 2048;
pub const CIBAUD_: ::libc::c_uint = 269418496;
pub const CMSPAR_: ::libc::c_uint = 1073741824;
pub const CRTSCTS_: ::libc::c_uint = -2147483648;
pub type Enum_lflags = ::libc::c_uint;
pub const ISIG_: ::libc::c_uint = 1;
pub const ICANON_: ::libc::c_uint = 2;
pub const XCASE_: ::libc::c_uint = 4;
pub const ECHO_: ::libc::c_uint = 8;
pub const ECHOE_: ::libc::c_uint = 16;
pub const ECHOK_: ::libc::c_uint = 32;
pub const ECHONL_: ::libc::c_uint = 64;
pub const ECHOCTL_: ::libc::c_uint = 512;
pub const ECHOPRT_: ::libc::c_uint = 1024;
pub const ECHOKE_: ::libc::c_uint = 2048;
pub const FLUSHO_: ::libc::c_uint = 4096;
pub const NOFLSH_: ::libc::c_uint = 128;
pub const TOSTOP_: ::libc::c_uint = 256;
pub const PENDIN_: ::libc::c_uint = 16384;
pub const IEXTEN_: ::libc::c_uint = 32768;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const VDISCARD_: ::libc::c_uint = 13;
pub const VEOF_: ::libc::c_uint = 4;
pub const VEOL_: ::libc::c_uint = 11;
pub const VEOL2_: ::libc::c_uint = 16;
pub const VERASE_: ::libc::c_uint = 2;
pub const VINTR_: ::libc::c_uint = 0;
pub const VKILL_: ::libc::c_uint = 3;
pub const VLNEXT_: ::libc::c_uint = 15;
pub const VMIN_: ::libc::c_uint = 6;
pub const VQUIT_: ::libc::c_uint = 1;
pub const VREPRINT_: ::libc::c_uint = 12;
pub const VSTART_: ::libc::c_uint = 8;
pub const VSTOP_: ::libc::c_uint = 9;
pub const VSUSP_: ::libc::c_uint = 10;
pub const VTIME_: ::libc::c_uint = 5;
pub const VWERASE_: ::libc::c_uint = 14;
pub type Enum_when = ::libc::c_uint;
pub const TCSANOW_: ::libc::c_uint = 0;
pub const TCSADRAIN_: ::libc::c_uint = 1;
pub const TCSAFLUSH_: ::libc::c_uint = 2;
pub type Enum_baud = ::libc::c_uint;
pub const B0_: ::libc::c_uint = 0;
pub const B50_: ::libc::c_uint = 1;
pub const B75_: ::libc::c_uint = 2;
pub const B110_: ::libc::c_uint = 3;
pub const B134_: ::libc::c_uint = 4;
pub const B150_: ::libc::c_uint = 5;
pub const B200_: ::libc::c_uint = 6;
pub const B300_: ::libc::c_uint = 7;
pub const B600_: ::libc::c_uint = 8;
pub const B1200_: ::libc::c_uint = 9;
pub const B1800_: ::libc::c_uint = 10;
pub const B2400_: ::libc::c_uint = 11;
pub const B4800_: ::libc::c_uint = 12;
pub const B9600_: ::libc::c_uint = 13;
pub const B19200_: ::libc::c_uint = 14;
pub const B38400_: ::libc::c_uint = 15;
pub const B57600_: ::libc::c_uint = 4097;
pub const B115200_: ::libc::c_uint = 4098;
pub const B230400_: ::libc::c_uint = 4099;
pub const B460800_: ::libc::c_uint = 4100;
pub const B500000_: ::libc::c_uint = 4101;
pub const B576000_: ::libc::c_uint = 4102;
pub const B921600_: ::libc::c_uint = 4103;
pub const B1000000_: ::libc::c_uint = 4104;
pub const B1152000_: ::libc::c_uint = 4105;
pub const B1500000_: ::libc::c_uint = 4106;
pub const B2000000_: ::libc::c_uint = 4107;
pub const B2500000_: ::libc::c_uint = 4108;
pub const B3000000_: ::libc::c_uint = 4109;
pub const B3500000_: ::libc::c_uint = 4110;
pub const B4000000_: ::libc::c_uint = 4111;
extern "C" {
    pub fn cfgetospeed(__termios_p: *const Struct_termios) -> speed_t;
    pub fn cfgetispeed(__termios_p: *const Struct_termios) -> speed_t;
    pub fn cfsetospeed(__termios_p: *mut Struct_termios, __speed: speed_t)
     -> ::libc::c_int;
    pub fn cfsetispeed(__termios_p: *mut Struct_termios, __speed: speed_t)
     -> ::libc::c_int;
    pub fn cfsetspeed(__termios_p: *mut Struct_termios, __speed: speed_t)
     -> ::libc::c_int;
    pub fn tcgetattr(__fd: ::libc::c_int, __termios_p: *mut Struct_termios)
     -> ::libc::c_int;
    pub fn tcsetattr(__fd: ::libc::c_int, __optional_actions: ::libc::c_int,
                     __termios_p: *const Struct_termios) -> ::libc::c_int;
    pub fn cfmakeraw(__termios_p: *mut Struct_termios) -> ();
    pub fn tcsendbreak(__fd: ::libc::c_int, __duration: ::libc::c_int)
     -> ::libc::c_int;
    pub fn tcdrain(__fd: ::libc::c_int) -> ::libc::c_int;
    pub fn tcflush(__fd: ::libc::c_int, __queue_selector: ::libc::c_int)
     -> ::libc::c_int;
    pub fn tcflow(__fd: ::libc::c_int, __action: ::libc::c_int)
     -> ::libc::c_int;
    pub fn tcgetsid(__fd: ::libc::c_int) -> __pid_t;
}
