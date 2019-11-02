use libc;
extern "C" {
  #[no_mangle]
  fn atof(__nptr: *const libc::c_char) -> libc::c_double;
  #[no_mangle]
  fn usleep(__useconds: __useconds_t) -> libc::c_int;
  #[no_mangle]
  fn fclose(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn bb_copyfd_exact_size(fd1: libc::c_int, fd2: libc::c_int, size: off_t);
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xfopen_for_read(path: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn bb_show_usage() -> !;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
  pub _flags: libc::c_int,
  pub _IO_read_ptr: *mut libc::c_char,
  pub _IO_read_end: *mut libc::c_char,
  pub _IO_read_base: *mut libc::c_char,
  pub _IO_write_base: *mut libc::c_char,
  pub _IO_write_ptr: *mut libc::c_char,
  pub _IO_write_end: *mut libc::c_char,
  pub _IO_buf_base: *mut libc::c_char,
  pub _IO_buf_end: *mut libc::c_char,
  pub _IO_save_base: *mut libc::c_char,
  pub _IO_backup_base: *mut libc::c_char,
  pub _IO_save_end: *mut libc::c_char,
  pub _markers: *mut _IO_marker,
  pub _chain: *mut _IO_FILE,
  pub _fileno: libc::c_int,
  pub _flags2: libc::c_int,
  pub _old_offset: __off_t,
  pub _cur_column: libc::c_ushort,
  pub _vtable_offset: libc::c_schar,
  pub _shortbuf: [libc::c_char; 1],
  pub _lock: *mut libc::c_void,
  pub _offset: __off64_t,
  pub __pad1: *mut libc::c_void,
  pub __pad2: *mut libc::c_void,
  pub __pad3: *mut libc::c_void,
  pub __pad4: *mut libc::c_void,
  pub __pad5: size_t,
  pub _mode: libc::c_int,
  pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub _next: *mut _IO_marker,
  pub _sbuf: *mut _IO_FILE,
  pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
/* vi: set sw=4 ts=4: */
/*
 * scriptreplay - play back typescripts, using timing information
 *
 * pascal.bellard@ads-lu.com
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config SCRIPTREPLAY
//config:	bool "scriptreplay (2.4 kb)"
//config:	default y
//config:	help
//config:	This program replays a typescript, using timing information
//config:	given by script -t.
//applet:IF_SCRIPTREPLAY(APPLET(scriptreplay, BB_DIR_BIN, BB_SUID_DROP))
//kbuild:lib-$(CONFIG_SCRIPTREPLAY) += scriptreplay.o
//usage:#define scriptreplay_trivial_usage
//usage:       "TIMINGFILE [TYPESCRIPT [DIVISOR]]"
//usage:#define scriptreplay_full_usage "\n\n"
//usage:       "Play back typescripts, using timing information"
#[no_mangle]
pub unsafe extern "C" fn scriptreplay_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut script: *const libc::c_char = b"typescript\x00" as *const u8 as *const libc::c_char;
  let mut delay: libc::c_double = 0.;
  let mut factor: libc::c_double = 1000000.0f64;
  let mut fd: libc::c_int = 0;
  let mut count: libc::c_ulong = 0;
  let mut tfp: *mut FILE = 0 as *mut FILE;
  if (*argv.offset(1)).is_null() {
    bb_show_usage();
  }
  if !(*argv.offset(2)).is_null() {
    script = *argv.offset(2);
    if !(*argv.offset(3)).is_null() {
      factor /= atof(*argv.offset(3))
    }
  }
  tfp = xfopen_for_read(*argv.offset(1));
  fd = xopen(script, 0i32);
  while fscanf(
    tfp,
    b"%lf %lu\n\x00" as *const u8 as *const libc::c_char,
    &mut delay as *mut libc::c_double,
    &mut count as *mut libc::c_ulong,
  ) == 2i32
  {
    usleep((delay * factor) as __useconds_t);
    bb_copyfd_exact_size(fd, 1i32, count as off_t);
  }
  return 0i32;
}