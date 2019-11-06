pub mod signal;

// These bits all seem to be from c2rust...
// See https://pubs.opengroup.org/onlinepubs/7908799/xsh/systypes.h.html.
pub type __blkcnt_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type ino_t = __ino64_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type smallint = libc::c_schar;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type uoff_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;

pub type __compar_fn_t =
  Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut libc::FILE,
  pub _pos: libc::c_int,
}

pub type _IO_lock_t = ();

// See http://man7.org/linux/man-pages/man3/getpwnam.3.html.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut libc::c_char,
  pub pw_passwd: *mut libc::c_char,
  pub pw_uid: libc::uid_t,
  pub pw_gid: libc::gid_t,
  pub pw_gecos: *mut libc::c_char,
  pub pw_dir: *mut libc::c_char,
  pub pw_shell: *mut libc::c_char,
}

// See https://www.mkssoftware.com/docs/man5/struct_group.5.asp.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
  pub gr_name: *mut libc::c_char,
  pub gr_passwd: *mut libc::c_char,
  pub gr_gid: libc::gid_t,
  pub gr_mem: *mut *mut libc::c_char,
}

// See
//  * https://www.gnu.org/software/libc/manual/html_node/Mode-Data-Types.html
//  * http://man7.org/linux/man-pages/man3/termios.3.html
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
  pub c_iflag: tcflag_t,
  pub c_oflag: tcflag_t,
  pub c_cflag: tcflag_t,
  pub c_lflag: tcflag_t,
  pub c_line: cc_t,
  pub c_cc: [cc_t; 32],
  pub c_ispeed: speed_t,
  pub c_ospeed: speed_t,
}

// See http://www.delorie.com/djgpp/doc/libc/libc_495.html.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
  pub ws_row: libc::c_ushort,
  pub ws_col: libc::c_ushort,
  pub ws_xpixel: libc::c_ushort,
  pub ws_ypixel: libc::c_ushort,
}

// ... end c2rust stuff

// Defined in libbb.h
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bb_uidgid_t {
  pub uid: libc::uid_t,
  pub gid: libc::gid_t,
}

// TODO: probably not as readable as a rust tuple.
/* In this form code with pipes is much more readable */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_pair {
  pub rd: libc::c_int,
  pub wr: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx_t {
  pub wbuffer: [u8; 64], /* always correctly aligned for uint64_t */
  pub process_block: Option<unsafe extern "C" fn(_: *mut md5_ctx_t) -> ()>,
  pub total64: u64,   /* must be directly before hash[] */
  pub hash: [u32; 8], /* 4 elements for md5, 5 for sha1, 8 for sha256 */
}
pub type sha1_ctx_t = md5_ctx_t;
pub type sha256_ctx_t = md5_ctx_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx_t {
  pub total64: [u64; 2], /* must be directly before hash[] */
  pub hash: [u64; 8],
  pub wbuffer: [u8; 128], /* always correctly aligned for uint64_t */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_ctx_t {
  pub state: [u64; 25],
  pub bytes_queued: libc::c_uint,
  pub input_block_bytes: libc::c_uint,
}
// ... end libbb.h stuff
