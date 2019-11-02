use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  fn free(__ptr: *mut libc::c_void);
  #[no_mangle]
  fn fsync(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
  #[no_mangle]
  static mut stderr: *mut _IO_FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
  #[no_mangle]
  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
  #[no_mangle]
  fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
  #[no_mangle]
  fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  fn monotonic_us() -> libc::c_ulonglong;
  #[no_mangle]
  fn xmalloc(size: size_t) -> *mut libc::c_void;
  #[no_mangle]
  fn xmove_fd(_: libc::c_int, _: libc::c_int);
  #[no_mangle]
  fn signal_SA_RESTART_empty_mask(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
  );
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn xlseek(fd: libc::c_int, offset: off_t, whence: libc::c_int) -> off_t;
  #[no_mangle]
  fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
  #[no_mangle]
  fn make_human_readable_str(
    size: libc::c_ulonglong,
    block_size: libc::c_ulong,
    display_unit: libc::c_ulong,
  ) -> *const libc::c_char;
  #[no_mangle]
  static cwbkMG_suffixes: [suffix_mult; 0];
  #[no_mangle]
  fn xatoull_range_sfx(
    str: *const libc::c_char,
    l: libc::c_ulonglong,
    u: libc::c_ulonglong,
    sfx: *const suffix_mult,
  ) -> libc::c_ulonglong;
  #[no_mangle]
  fn xatoull_sfx(str: *const libc::c_char, sfx: *const suffix_mult) -> libc::c_ulonglong;
  #[no_mangle]
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_simple_perror_msg(s: *const libc::c_char);
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn index_in_strings(strings: *const libc::c_char, key: *const libc::c_char) -> libc::c_int;
  #[no_mangle]
  static bb_msg_invalid_arg_to: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_standard_input: [libc::c_char; 0];
  #[no_mangle]
  static bb_msg_standard_output: [libc::c_char; 0];
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint16_t = libc::c_ushort;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type smallint = libc::c_schar;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
  pub tv_sec: __time_t,
  pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
  pub st_dev: __dev_t,
  pub st_ino: __ino_t,
  pub st_nlink: __nlink_t,
  pub st_mode: __mode_t,
  pub st_uid: __uid_t,
  pub st_gid: __gid_t,
  pub __pad0: libc::c_int,
  pub st_rdev: __dev_t,
  pub st_size: __off_t,
  pub st_blksize: __blksize_t,
  pub st_blocks: __blkcnt_t,
  pub st_atim: timespec,
  pub st_mtim: timespec,
  pub st_ctim: timespec,
  pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct suffix_mult {
  pub suffix: [libc::c_char; 4],
  pub mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub out_full: off_t,
  pub out_part: off_t,
  pub in_full: off_t,
  pub in_part: off_t,
  pub total_bytes: libc::c_ulonglong,
  pub begin_time_us: libc::c_ulonglong,
  pub flags: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const ofd: C2RustUnnamed = 1;
pub const ifd: C2RustUnnamed = 0;
/* we have to zero it out because of NOEXEC */
pub type C2RustUnnamed_0 = libc::c_uint;
pub const FLAG_STATUS_NOXFER: C2RustUnnamed_0 = 4096;
pub const FLAG_STATUS_NONE: C2RustUnnamed_0 = 2048;
pub const FLAG_COUNT: C2RustUnnamed_0 = 1024;
/* end of output flags */
pub const FLAG_TWOBUFS: C2RustUnnamed_0 = 512;
pub const FLAG_APPEND: C2RustUnnamed_0 = 256;
pub const FLAG_SEEK_BYTES: C2RustUnnamed_0 = 128;
/* end of input flags */
/* start of output flags */
pub const FLAG_OFLAG_SHIFT: C2RustUnnamed_0 = 7;
pub const FLAG_FULLBLOCK: C2RustUnnamed_0 = 64;
pub const FLAG_SKIP_BYTES: C2RustUnnamed_0 = 32;
/* end of conv flags */
/* start of input flags */
pub const FLAG_IFLAG_SHIFT: C2RustUnnamed_0 = 5;
pub const FLAG_SWAB: C2RustUnnamed_0 = 16;
pub const FLAG_FSYNC: C2RustUnnamed_0 = 8;
pub const FLAG_NOERROR: C2RustUnnamed_0 = 4;
pub const FLAG_SYNC: C2RustUnnamed_0 = 2;
/* Must be in the same order as OP_conv_XXX! */
/* (see "flags |= (1 << what)" below) */
pub const FLAG_NOTRUNC: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
  pub ocount: size_t,
  pub prev_read_size: ssize_t,
  pub count: off_t,
  pub seek: off_t,
  pub skip: off_t,
  pub infile: *const libc::c_char,
  pub outfile: *const libc::c_char,
}
pub const OP_status: C2RustUnnamed_2 = 6;
pub const OP_of: C2RustUnnamed_2 = 5;
pub const OP_if: C2RustUnnamed_2 = 4;
pub const OP_skip: C2RustUnnamed_2 = 3;
pub const OP_seek: C2RustUnnamed_2 = 2;
pub const OP_count: C2RustUnnamed_2 = 1;
pub const OP_bs: C2RustUnnamed_2 = 0;
pub const OP_oflag: C2RustUnnamed_2 = 11;
pub const OP_iflag: C2RustUnnamed_2 = 10;
pub const OP_conv: C2RustUnnamed_2 = 9;
pub const OP_obs: C2RustUnnamed_2 = 8;
pub const OP_ibs: C2RustUnnamed_2 = 7;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const OP_oflag_seek_bytes: C2RustUnnamed_2 = 7;
pub const OP_iflag_fullblock: C2RustUnnamed_2 = 6;
/* Unimplemented conv=XXX: */
//nocreat       do not create the output file
//excl          fail if the output file already exists
//fdatasync     physically write output file data before finishing
//lcase         change upper case to lower case
//ucase         change lower case to upper case
//block         pad newline-terminated records with spaces to cbs-size
//unblock       replace trailing spaces in cbs-size records with newline
//ascii         from EBCDIC to ASCII
//ebcdic        from ASCII to EBCDIC
//ibm           from ASCII to alternate EBCDIC
/* Partially implemented: */
//swab          swap every pair of input bytes: will abort on non-even reads
pub const OP_iflag_skip_bytes: C2RustUnnamed_2 = 5;
pub const OP_conv_swab: C2RustUnnamed_2 = 4;
pub const OP_conv_fsync: C2RustUnnamed_2 = 3;
pub const OP_conv_noerror: C2RustUnnamed_2 = 2;
pub const OP_conv_sync: C2RustUnnamed_2 = 1;
/* Must be in the same order as FLAG_XXX! */
pub const OP_conv_notrunc: C2RustUnnamed_2 = 0;
#[inline(always)]
unsafe extern "C" fn xatoul_range_sfx(
  mut str: *const libc::c_char,
  mut l: libc::c_ulong,
  mut u: libc::c_ulong,
  mut sfx: *const suffix_mult,
) -> libc::c_ulong {
  return xatoull_range_sfx(str, l as libc::c_ulonglong, u as libc::c_ulonglong, sfx)
    as libc::c_ulong; /* before fprintf */
}
unsafe extern "C" fn dd_output_status(mut cur_signal: libc::c_int) {
  let mut seconds: libc::c_double = 0.;
  let mut bytes_sec: libc::c_ulonglong = 0;
  let mut now_us: libc::c_ulonglong = monotonic_us();
  /* Deliberately using %u, not %d */
  fprintf(
    stderr,
    b"%lu+%lu records in\n%lu+%lu records out\n\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_full,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_part,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_full,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_part,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags & FLAG_STATUS_NOXFER as libc::c_int
    != 0
  {
    /* status=noxfer active? */
    return;
  }
  //TODO: should status=none make dd stop reacting to USR1 entirely?
  //So far we react to it (we print the stats),
  //status=none only suppresses final, non-USR1 generated status message.
  fprintf(
    stderr,
    b"%llu bytes (%sB) copied, \x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).total_bytes,
    make_human_readable_str(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).total_bytes,
      1i32 as libc::c_ulong,
      0i32 as libc::c_ulong,
    ),
  );
  /* Corner cases:
   * ./busybox dd </dev/null >/dev/null
   * ./busybox dd bs=1M count=2000 </dev/zero >/dev/null
   * (echo DONE) | ./busybox dd >/dev/null
   * (sleep 1; echo DONE) | ./busybox dd >/dev/null
   */
  seconds = now_us.wrapping_sub((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).begin_time_us)
    as libc::c_double
    / 1000000.0f64;
  bytes_sec = ((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).total_bytes as libc::c_double
    / seconds) as libc::c_ulonglong;
  fprintf(
    stderr,
    b"%f seconds, %sB/s\n\x00" as *const u8 as *const libc::c_char,
    seconds,
    make_human_readable_str(bytes_sec, 1i32 as libc::c_ulong, 0i32 as libc::c_ulong),
  );
}
unsafe extern "C" fn write_and_stats(
  mut buf: *const libc::c_void,
  mut len: size_t,
  mut obs: size_t,
  mut filename: *const libc::c_char,
) -> bool {
  let mut n: ssize_t = 0;
  n = full_write(ofd as libc::c_int, buf, len);
  if n > 0i32 as libc::c_long {
    let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).total_bytes;
    *fresh0 = (*fresh0).wrapping_add(n as libc::c_ulonglong)
  }
  if n as size_t == obs {
    let ref mut fresh1 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_full;
    *fresh1 += 1;
    return 0i32 != 0;
  }
  if n as size_t == len {
    let ref mut fresh2 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).out_part;
    *fresh2 += 1;
    return 0i32 != 0;
  }
  /* n is < len (and possibly is -1).
   * Even if n >= 0, errno is usually set correctly.
   * For example, if writing to block device and getting ENOSPC,
   * full_write() first sees a short write, then tries to write
   * the remainder and gets errno set to ENOSPC.
   * It returns n > 0 (the amount which it did write).
   */
  bb_perror_msg(
    b"error writing \'%s\'\x00" as *const u8 as *const libc::c_char,
    filename,
  );
  return 1i32 != 0;
}
unsafe extern "C" fn parse_comma_flags(
  mut val: *mut libc::c_char,
  mut words: *const libc::c_char,
  mut error_in: *const libc::c_char,
) -> libc::c_int {
  let mut flags: libc::c_int = 0i32;
  loop {
    let mut n: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    /* skip this keyword and ',' */
    /* find ',', replace them with NUL so we can use val for
     * index_in_strings() without copying.
     * We rely on val being non-null, else strchr would fault.
     */
    arg = strchr(val, ',' as i32); /* to preserve ps listing */
    if !arg.is_null() {
      *arg = '\u{0}' as i32 as libc::c_char
    }
    n = index_in_strings(words, val);
    if n < 0i32 {
      bb_error_msg_and_die(bb_msg_invalid_arg_to.as_ptr(), val, error_in);
    }
    flags |= 1i32 << n;
    if arg.is_null() {
      break;
    }
    *arg = ',' as i32 as libc::c_char;
    val = arg.offset(1)
  }
  return flags;
}
#[no_mangle]
pub unsafe extern "C" fn dd_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  static mut keywords: [libc::c_char; 58] = [
    98, 115, 0, 99, 111, 117, 110, 116, 0, 115, 101, 101, 107, 0, 115, 107, 105, 112, 0, 105, 102,
    0, 111, 102, 0, 115, 116, 97, 116, 117, 115, 0, 105, 98, 115, 0, 111, 98, 115, 0, 99, 111, 110,
    118, 0, 105, 102, 108, 97, 103, 0, 111, 102, 108, 97, 103, 0, 0,
  ];
  static mut conv_words: [libc::c_char; 33] = [
    110, 111, 116, 114, 117, 110, 99, 0, 115, 121, 110, 99, 0, 110, 111, 101, 114, 114, 111, 114,
    0, 102, 115, 121, 110, 99, 0, 115, 119, 97, 98, 0, 0,
  ];
  static mut iflag_words: [libc::c_char; 22] = [
    115, 107, 105, 112, 95, 98, 121, 116, 101, 115, 0, 102, 117, 108, 108, 98, 108, 111, 99, 107,
    0, 0,
  ];
  static mut oflag_words: [libc::c_char; 19] = [
    115, 101, 101, 107, 95, 98, 121, 116, 101, 115, 0, 97, 112, 112, 101, 110, 100, 0, 0,
  ];
  static mut status_words: [libc::c_char; 13] =
    [110, 111, 110, 101, 0, 110, 111, 120, 102, 101, 114, 0, 0];
  let mut exitcode: smallint = 1i32 as smallint;
  let mut i: libc::c_int = 0;
  let mut ibs: size_t = 512i32 as size_t;
  let mut ibuf: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut obs: size_t = 512i32 as size_t;
  let mut obuf: *mut libc::c_char = 0 as *mut libc::c_char;
  /* These are all zeroed at once! */
  let mut Z: C2RustUnnamed_1 = C2RustUnnamed_1 {
    ocount: 0,
    prev_read_size: 0,
    count: 0,
    seek: 0,
    skip: 0,
    infile: 0 as *const libc::c_char,
    outfile: 0 as *const libc::c_char,
  };
  memset(
    &mut Z as *mut C2RustUnnamed_1 as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
  );
  memset(
    bb_common_bufsiz1.as_mut_ptr() as *mut globals as *mut libc::c_void,
    0i32,
    ::std::mem::size_of::<globals>() as libc::c_ulong,
  );
  //fflush_all(); - is this needed because of NOEXEC?
  i = 1i32; /* end of "for (argv[i])" */
  while !(*argv.offset(i as isize)).is_null() {
    let mut what: libc::c_int = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut libc::c_char = *argv.offset(i as isize);
    /* "dd --". NB: coreutils 6.9 will complain if they see
     * more than one of them. We wouldn't. */
    if !(*arg.offset(0) as libc::c_int == '-' as i32
      && *arg.offset(1) as libc::c_int == '-' as i32
      && *arg.offset(2) as libc::c_int == '\u{0}' as i32)
    {
      val = strchr(arg, '=' as i32);
      if val.is_null() {
        bb_show_usage();
      }
      *val = '\u{0}' as i32 as libc::c_char;
      what = index_in_strings(keywords.as_ptr(), arg);
      if what < 0i32 {
        bb_show_usage();
      }
      /* *val = '='; - to preserve ps listing? */
      val = val.offset(1);
      if what == OP_ibs as libc::c_int {
        /* Must fit into positive ssize_t */
        ibs = xatoul_range_sfx(
          val,
          1i32 as libc::c_ulong,
          (-1i64 as size_t).wrapping_div(2i32 as libc::c_ulong),
          cwbkMG_suffixes.as_ptr(),
        )
        /*continue;*/
      }
      if what == OP_obs as libc::c_int {
        obs = xatoul_range_sfx(
          val,
          1i32 as libc::c_ulong,
          (-1i64 as size_t).wrapping_div(2i32 as libc::c_ulong),
          cwbkMG_suffixes.as_ptr(),
        )
        /*continue;*/
      }
      if what == OP_conv as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |= parse_comma_flags(
          val,
          conv_words.as_ptr(),
          b"conv\x00" as *const u8 as *const libc::c_char,
        )
        /*continue;*/
      }
      if what == OP_iflag as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |= parse_comma_flags(
          val,
          iflag_words.as_ptr(),
          b"iflag\x00" as *const u8 as *const libc::c_char,
        ) << FLAG_IFLAG_SHIFT
          as libc::c_int
        /*continue;*/
      }
      if what == OP_oflag as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |= parse_comma_flags(
          val,
          oflag_words.as_ptr(),
          b"oflag\x00" as *const u8 as *const libc::c_char,
        ) << FLAG_OFLAG_SHIFT
          as libc::c_int
        /*continue;*/
      }
      if what == OP_bs as libc::c_int {
        ibs = xatoul_range_sfx(
          val,
          1i32 as libc::c_ulong,
          (-1i64 as size_t).wrapping_div(2i32 as libc::c_ulong),
          cwbkMG_suffixes.as_ptr(),
        );
        obs = ibs
        /*continue;*/
      }
      /* These can be large: */
      if what == OP_count as libc::c_int {
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |= FLAG_COUNT as libc::c_int;
        Z.count = xatoull_sfx(val, cwbkMG_suffixes.as_ptr()) as off_t
        /*continue;*/
      }
      if what == OP_seek as libc::c_int {
        Z.seek = xatoull_sfx(val, cwbkMG_suffixes.as_ptr()) as off_t
        /*continue;*/
      }
      if what == OP_skip as libc::c_int {
        Z.skip = xatoull_sfx(val, cwbkMG_suffixes.as_ptr()) as off_t
        /*continue;*/
      }
      if what == OP_if as libc::c_int {
        Z.infile = val
        /*continue;*/
      }
      if what == OP_of as libc::c_int {
        Z.outfile = val
        /*continue;*/
      }
      if what == OP_status as libc::c_int {
        let mut n: libc::c_int = 0;
        n = index_in_strings(status_words.as_ptr(), val);
        if n < 0i32 {
          bb_error_msg_and_die(
            bb_msg_invalid_arg_to.as_ptr(),
            val,
            b"status\x00" as *const u8 as *const libc::c_char,
          );
        }
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |=
          (FLAG_STATUS_NONE as libc::c_int) << n
        /*continue;*/
      }
    }
    i += 1
  }
  //XXX:FIXME for huge ibs or obs, malloc'ing them isn't the brightest idea ever
  ibuf = xmalloc(ibs) as *mut libc::c_char;
  obuf = ibuf;
  if ibs != obs {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags |= FLAG_TWOBUFS as libc::c_int;
    obuf = xmalloc(obs) as *mut libc::c_char
  }
  signal_SA_RESTART_empty_mask(
    10i32,
    Some(dd_output_status as unsafe extern "C" fn(_: libc::c_int) -> ()),
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).begin_time_us = monotonic_us();
  if !Z.infile.is_null() {
    xmove_fd(xopen(Z.infile, 0i32), ifd as libc::c_int);
  } else {
    Z.infile = bb_msg_standard_input.as_ptr()
  }
  if !Z.outfile.is_null() {
    let mut oflag: libc::c_int = 0o1i32 | 0o100i32;
    if Z.seek == 0
      && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags & FLAG_NOTRUNC as libc::c_int
        == 0
    {
      oflag |= 0o1000i32
    }
    if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags & FLAG_APPEND as libc::c_int != 0 {
      oflag |= 0o2000i32
    }
    xmove_fd(xopen(Z.outfile, oflag), ofd as libc::c_int);
    if Z.seek != 0
      && (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags & FLAG_NOTRUNC as libc::c_int
        == 0
    {
      let mut blocksz: size_t = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
        & FLAG_SEEK_BYTES as libc::c_int
        != 0
      {
        1i32 as libc::c_ulong
      } else {
        obs
      };
      if ftruncate(
        ofd as libc::c_int,
        (Z.seek as libc::c_ulong).wrapping_mul(blocksz) as __off64_t,
      ) < 0i32
      {
        let mut st: stat = stat {
          st_dev: 0,
          st_ino: 0,
          st_nlink: 0,
          st_mode: 0,
          st_uid: 0,
          st_gid: 0,
          __pad0: 0,
          st_rdev: 0,
          st_size: 0,
          st_blksize: 0,
          st_blocks: 0,
          st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
          },
          st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
          },
          st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
          },
          __glibc_reserved: [0; 3],
        };
        if fstat(ofd as libc::c_int, &mut st) < 0i32
          || st.st_mode & 0o170000i32 as libc::c_uint == 0o100000i32 as libc::c_uint
          || st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
        {
          current_block = 17215991467164075883;
        } else {
          current_block = 1852451392920375136;
        }
      } else {
        current_block = 1852451392920375136;
      }
    } else {
      current_block = 1852451392920375136;
    }
  } else {
    Z.outfile = bb_msg_standard_output.as_ptr();
    current_block = 1852451392920375136;
  }
  match current_block {
    1852451392920375136 => {
      if Z.skip != 0 {
        let mut blocksz_0: size_t = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
          & FLAG_SKIP_BYTES as libc::c_int
          != 0
        {
          1i32 as libc::c_ulong
        } else {
          ibs
        };
        if lseek(
          ifd as libc::c_int,
          (Z.skip as libc::c_ulong).wrapping_mul(blocksz_0) as __off64_t,
          1i32,
        ) < 0i32 as libc::c_long
        {
          loop {
            let mut n_0: ssize_t = 0;
            if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
              & FLAG_FULLBLOCK as libc::c_int
              != 0
            {
              n_0 = full_read(ifd as libc::c_int, ibuf as *mut libc::c_void, blocksz_0)
            } else {
              n_0 = safe_read(ifd as libc::c_int, ibuf as *mut libc::c_void, blocksz_0)
            }
            if n_0 < 0i32 as libc::c_long {
              current_block = 5737263145267917659;
              break;
            }
            if n_0 == 0i32 as libc::c_long {
              current_block = 1658462350791934405;
              break;
            }
            Z.skip -= 1;
            if !(Z.skip != 0i32 as libc::c_long) {
              current_block = 1658462350791934405;
              break;
            }
          }
        } else {
          current_block = 1658462350791934405;
        }
      } else {
        current_block = 1658462350791934405;
      }
      match current_block {
        1658462350791934405 => {
          if Z.seek != 0 {
            let mut blocksz_1: size_t = if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
              & FLAG_SEEK_BYTES as libc::c_int
              != 0
            {
              1i32 as libc::c_ulong
            } else {
              obs
            };
            if lseek(
              ofd as libc::c_int,
              (Z.seek as libc::c_ulong).wrapping_mul(blocksz_1) as __off64_t,
              1i32,
            ) < 0i32 as libc::c_long
            {
              current_block = 17215991467164075883;
            } else {
              current_block = 17372050596571538954;
            }
          } else {
            current_block = 17372050596571538954;
          }
          match current_block {
            17215991467164075883 => {}
            _ => {
              's_520: loop {
                if !((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                  & FLAG_COUNT as libc::c_int
                  == 0
                  || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_full
                    + (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_part
                    != Z.count)
                {
                  current_block = 9354678635443812511;
                  break;
                }
                let mut n_1: ssize_t = 0;
                if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                  & FLAG_FULLBLOCK as libc::c_int
                  != 0
                {
                  n_1 = full_read(ifd as libc::c_int, ibuf as *mut libc::c_void, ibs)
                } else {
                  n_1 = safe_read(ifd as libc::c_int, ibuf as *mut libc::c_void, ibs)
                }
                if n_1 == 0i32 as libc::c_long {
                  current_block = 9354678635443812511;
                  break;
                }
                if n_1 < 0i32 as libc::c_long {
                  /* "Bad block" */
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                    & FLAG_NOERROR as libc::c_int
                    == 0
                  {
                    current_block = 5737263145267917659;
                    break;
                  }
                  bb_simple_perror_msg(Z.infile);
                  /* GNU dd with conv=noerror skips over bad blocks */
                  xlseek(ifd as libc::c_int, ibs as off_t, 1i32);
                  /* conv=noerror,sync writes NULs,
                   * conv=noerror just ignores input bad blocks */
                  n_1 = 0i32 as ssize_t
                }
                if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                  & FLAG_SWAB as libc::c_int
                  != 0
                {
                  let mut p16: *mut uint16_t = 0 as *mut uint16_t;
                  let mut n2: ssize_t = 0;
                  /* Our code allows only last read to be odd-sized */
                  if Z.prev_read_size & 1i32 as libc::c_long != 0 {
                    bb_error_msg_and_die(
                      b"can\'t swab %lu byte buffer\x00" as *const u8 as *const libc::c_char,
                      Z.prev_read_size as libc::c_ulong,
                    );
                  }
                  Z.prev_read_size = n_1;
                  /* If n is odd, last byte is not swapped:
                   *  echo -n "qwe" | dd conv=swab
                   * prints "wqe".
                   */
                  p16 = ibuf as *mut libc::c_void as *mut uint16_t;
                  n2 = n_1 >> 1i32;
                  loop {
                    n2 -= 1;
                    if !(n2 >= 0i32 as libc::c_long) {
                      break;
                    }
                    *p16 = ({
                      let mut __v: libc::c_ushort = 0;
                      let mut __x: libc::c_ushort = *p16;
                      if 0 != 0 {
                        __v = (__x as libc::c_int >> 8i32 & 0xffi32
                          | (__x as libc::c_int & 0xffi32) << 8i32)
                          as libc::c_ushort
                      } else {
                        let fresh3 = &mut __v;
                        let fresh4;
                        let fresh5 = __x;
                        asm!("rorw $$8, ${0:w}"
                                                              : "=r" (fresh4)
                                                              : "0"
                                                              (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5))
                                                              : "cc");
                        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                      }
                      __v
                    });
                    p16 = p16.offset(1)
                  }
                }
                if n_1 as size_t == ibs {
                  let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_full;
                  *fresh6 += 1
                } else {
                  let ref mut fresh7 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).in_part;
                  *fresh7 += 1;
                  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                    & FLAG_SYNC as libc::c_int
                    != 0
                  {
                    memset(
                      ibuf.offset(n_1 as isize) as *mut libc::c_void,
                      0i32,
                      ibs.wrapping_sub(n_1 as libc::c_ulong),
                    );
                    n_1 = ibs as ssize_t
                  }
                }
                if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                  & FLAG_TWOBUFS as libc::c_int
                  != 0
                {
                  let mut tmp: *mut libc::c_char = ibuf;
                  while n_1 != 0 {
                    let mut d: size_t = obs.wrapping_sub(Z.ocount);
                    if d > n_1 as size_t {
                      d = n_1 as size_t
                    }
                    memcpy(
                      obuf.offset(Z.ocount as isize) as *mut libc::c_void,
                      tmp as *const libc::c_void,
                      d,
                    );
                    n_1 = (n_1 as libc::c_ulong).wrapping_sub(d) as ssize_t as ssize_t;
                    tmp = tmp.offset(d as isize);
                    Z.ocount = (Z.ocount as libc::c_ulong).wrapping_add(d) as size_t as size_t;
                    if !(Z.ocount == obs) {
                      continue;
                    }
                    if write_and_stats(obuf as *const libc::c_void, obs, obs, Z.outfile) {
                      current_block = 14463043422689283256;
                      break 's_520;
                    }
                    Z.ocount = 0i32 as size_t
                  }
                } else if write_and_stats(
                  ibuf as *const libc::c_void,
                  n_1 as size_t,
                  obs,
                  Z.outfile,
                ) {
                  current_block = 14463043422689283256;
                  break;
                }
              }
              match current_block {
                5737263145267917659 => {}
                _ => {
                  match current_block {
                    9354678635443812511 => {
                      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                        & FLAG_FSYNC as libc::c_int
                        != 0
                      {
                        if fsync(ofd as libc::c_int) < 0i32 {
                          current_block = 17215991467164075883;
                        } else {
                          current_block = 14358540534591340610;
                        }
                      } else {
                        current_block = 14358540534591340610;
                      }
                      match current_block {
                        17215991467164075883 => {}
                        _ => {
                          if Z.ocount != 0i32 as libc::c_ulong {
                            if write_and_stats(
                              obuf as *const libc::c_void,
                              Z.ocount,
                              obs,
                              Z.outfile,
                            ) {
                              current_block = 14463043422689283256;
                            } else {
                              current_block = 8444733628337052024;
                            }
                          } else {
                            current_block = 8444733628337052024;
                          }
                          match current_block {
                            14463043422689283256 => {}
                            _ => {
                              if close(ifd as libc::c_int) < 0i32 {
                                current_block = 5737263145267917659;
                              } else if close(ofd as libc::c_int) < 0i32 {
                                current_block = 17215991467164075883;
                              } else {
                                exitcode = 0i32 as smallint;
                                current_block = 14463043422689283256;
                              }
                            }
                          }
                        }
                      }
                    }
                    _ => {}
                  }
                  match current_block {
                    17215991467164075883 => {}
                    5737263145267917659 => {}
                    _ => {
                      if 1i32 == 0
                        || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).flags
                          & FLAG_STATUS_NONE as libc::c_int
                          == 0
                      {
                        dd_output_status(0i32);
                      }
                      return exitcode as libc::c_int;
                    }
                  }
                }
              }
            }
          }
        }
        _ => {}
      }
      match current_block {
        17215991467164075883 => {}
        _ => {
          bb_simple_perror_msg_and_die(Z.infile);
        }
      }
    }
    _ => {}
  }
  bb_simple_perror_msg_and_die(Z.outfile);
}