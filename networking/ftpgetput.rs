use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
extern "C" {
  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  fn fflush(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn fgets_unlocked(
    __s: *mut libc::c_char,
    __n: libc::c_int,
    __stream: *mut FILE,
  ) -> *mut libc::c_char;
  #[no_mangle]
  fn close(__fd: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  /* bb_copyfd_XX print read/write errors and return -1 if they occur */
  #[no_mangle]
  fn bb_copyfd_eof(fd1: libc::c_int, fd2: libc::c_int) -> off_t;
  #[no_mangle]
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  /* NB: returns port in host byte order */
  #[no_mangle]
  fn bb_lookup_port(
    port: *const libc::c_char,
    protocol: *const libc::c_char,
    default_port: libc::c_uint,
  ) -> libc::c_uint;
  /* Connect to peer identified by lsa */
  #[no_mangle]
  fn xconnect_stream(lsa: *const len_and_sockaddr) -> libc::c_int;
  /* Version which dies on error */
  #[no_mangle]
  fn xhost2sockaddr(host: *const libc::c_char, port: libc::c_int) -> *mut len_and_sockaddr;
  /* Assign sin[6]_port member if the socket is an AF_INET[6] one,
   * otherwise no-op. Useful for ftp.
   * NB: does NOT do htons() internally, just direct assignment. */
  #[no_mangle]
  fn set_nport(sa: *mut sockaddr, port: libc::c_uint);
  /* inet_[ap]ton on steroids */
  #[no_mangle]
  fn xmalloc_sockaddr2dotted(sa: *const sockaddr) -> *mut libc::c_char;
  #[no_mangle]
  fn parse_pasv_epsv(buf_0: *mut libc::c_char) -> libc::c_int;
  #[no_mangle]
  fn xatou(str: *const libc::c_char) -> libc::c_uint;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
  #[no_mangle]
  fn bb_error_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
  #[no_mangle]
  fn bb_simple_perror_msg_and_die(s: *const libc::c_char) -> !;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;
  #[no_mangle]
  static mut applet_name: *const libc::c_char;
  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
pub type socklen_t = __socklen_t;
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
  pub sa_family: sa_family_t,
  pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
  pub sin6_family: sa_family_t,
  pub sin6_port: in_port_t,
  pub sin6_flowinfo: uint32_t,
  pub sin6_addr: in6_addr,
  pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
  pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
  pub __u6_addr8: [uint8_t; 16],
  pub __u6_addr16: [uint16_t; 8],
  pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
  pub sin_family: sa_family_t,
  pub sin_port: in_port_t,
  pub sin_addr: in_addr,
  pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
  pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
pub struct len_and_sockaddr {
  pub len: socklen_t,
  pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
  pub sa: sockaddr,
  pub sin: sockaddr_in,
  pub sin6: sockaddr_in6,
}
//extern const int const_int_1;
/* This struct is deliberately not defined. */
/* See docs/keep_data_small.txt */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globals {
  pub user: *const libc::c_char,
  pub password: *const libc::c_char,
  pub lsa: *mut len_and_sockaddr,
  pub control_stream: *mut FILE,
  pub verbose_flag: libc::c_int,
  pub do_continue: libc::c_int,
  pub buf: [libc::c_char; 4],
}
/* actually [BUFSZ] */
pub type C2RustUnnamed_1 = libc::c_uint;
pub const BUFSZ: C2RustUnnamed_1 = 984;
unsafe extern "C" fn ftp_die(mut msg: *const libc::c_char) -> ! {
  let mut cp: *mut libc::c_char = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .buf
    .as_mut_ptr(); /* buf holds peer's response */
  /* Guard against garbage from remote server */
  while *cp as libc::c_int >= ' ' as i32 && (*cp as libc::c_int) < '\u{7f}' as i32 {
    cp = cp.offset(1)
  } /* for ftp_die */
  *cp = '\u{0}' as i32 as libc::c_char;
  bb_error_msg_and_die(
    b"unexpected server response%s%s: %s\x00" as *const u8 as *const libc::c_char,
    if !msg.is_null() {
      b" to \x00" as *const u8 as *const libc::c_char
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    if !msg.is_null() {
      msg
    } else {
      b"\x00" as *const u8 as *const libc::c_char
    },
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .buf
      .as_mut_ptr(),
  );
}
unsafe extern "C" fn ftpcmd(
  mut s1: *const libc::c_char,
  mut s2: *const libc::c_char,
) -> libc::c_int {
  let mut n: libc::c_uint = 0;
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose_flag != 0 {
    bb_error_msg(b"cmd %s %s\x00" as *const u8 as *const libc::c_char, s1, s2);
  }
  if !s1.is_null() {
    fprintf(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).control_stream,
      if !s2.is_null() {
        b"%s %s\r\n\x00" as *const u8 as *const libc::c_char
      } else {
        (b"%s %s\r\n\x00" as *const u8 as *const libc::c_char).offset(3)
      },
      s1,
      s2,
    );
    fflush((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).control_stream);
  }
  loop {
    strcpy(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .buf
        .as_mut_ptr(),
      b"EOF\x00" as *const u8 as *const libc::c_char,
    );
    if fgets_unlocked(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .buf
        .as_mut_ptr(),
      BUFSZ as libc::c_int - 2i32,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).control_stream,
    )
    .is_null()
    {
      ftp_die(0 as *const libc::c_char);
    }
    if !(!(((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[0] as libc::c_int - '0' as i32)
      as libc::c_uchar as libc::c_int
      <= 9i32)
      || (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[3] as libc::c_int != ' ' as i32)
    {
      break;
    }
  }
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[3] = '\u{0}' as i32 as libc::c_char;
  n = xatou(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .buf
      .as_mut_ptr(),
  );
  (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).buf[3] = ' ' as i32 as libc::c_char;
  return n as libc::c_int;
}
unsafe extern "C" fn ftp_login() {
  /* Connect to the command socket */
  let ref mut fresh0 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).control_stream;
  *fresh0 = fdopen(
    xconnect_stream((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lsa),
    b"r+\x00" as *const u8 as *const libc::c_char,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .control_stream
    .is_null()
  {
    /* fdopen failed - extremely unlikely */
    bb_perror_nomsg_and_die();
  }
  if ftpcmd(0 as *const libc::c_char, 0 as *const libc::c_char) != 220i32 {
    ftp_die(0 as *const libc::c_char);
  }
  /*  Login to the server */
  match ftpcmd(
    b"USER\x00" as *const u8 as *const libc::c_char,
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).user,
  ) {
    230 => {}
    331 => {
      if ftpcmd(
        b"PASS\x00" as *const u8 as *const libc::c_char,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).password,
      ) != 230i32
      {
        ftp_die(b"PASS\x00" as *const u8 as *const libc::c_char);
      }
    }
    _ => {
      ftp_die(b"USER\x00" as *const u8 as *const libc::c_char);
    }
  }
  ftpcmd(
    b"TYPE I\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
  );
}
unsafe extern "C" fn xconnect_ftpdata() -> libc::c_int {
  let mut port_num: libc::c_int = 0;
  if !(1i32 != 0
    && ftpcmd(
      b"EPSV\x00" as *const u8 as *const libc::c_char,
      0 as *const libc::c_char,
    ) == 229i32)
  {
    if ftpcmd(
      b"PASV\x00" as *const u8 as *const libc::c_char,
      0 as *const libc::c_char,
    ) != 227i32
    {
      ftp_die(b"PASV\x00" as *const u8 as *const libc::c_char);
    }
  }
  port_num = parse_pasv_epsv(
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
      .buf
      .as_mut_ptr(),
  );
  if port_num < 0i32 {
    ftp_die(b"PASV\x00" as *const u8 as *const libc::c_char);
  }
  set_nport(
    &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lsa)
      .u
      .sa,
    ({
      let mut __v: libc::c_ushort = 0;
      let mut __x: libc::c_ushort = port_num as libc::c_ushort;
      if 0 != 0 {
        __v = (__x as libc::c_int >> 8i32 & 0xffi32 | (__x as libc::c_int & 0xffi32) << 8i32)
          as libc::c_ushort
      } else {
        let fresh1 = &mut __v;
        let fresh2;
        let fresh3 = __x;
        asm!("rorw $$8, ${0:w}" : "=r" (fresh2) : "0"
                            (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh3))
                            : "cc");
        c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
      }
      __v
    }) as libc::c_uint,
  );
  return xconnect_stream((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lsa);
}
unsafe extern "C" fn pump_data_and_QUIT(mut from: libc::c_int, mut to: libc::c_int) -> libc::c_int {
  /* copy the file */
  if bb_copyfd_eof(from, to) == -1i32 as libc::c_long {
    /* error msg is already printed by bb_copyfd_eof */
    return 1i32;
  }
  /* close data connection */
  close(from); /* don't know which one is that, so we close both */
  close(to);
  /* does server confirm that transfer is finished? */
  if ftpcmd(0 as *const libc::c_char, 0 as *const libc::c_char) != 226i32 {
    ftp_die(0 as *const libc::c_char);
  }
  ftpcmd(
    b"QUIT\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
  );
  return 0i32;
}
unsafe extern "C" fn ftp_receive(
  mut local_path: *const libc::c_char,
  mut server_path: *mut libc::c_char,
) -> libc::c_int {
  let mut fd_data: libc::c_int = 0;
  let mut fd_local: libc::c_int = -1i32;
  let mut beg_range: off_t = 0i32 as off_t;
  /* connect to the data socket */
  fd_data = xconnect_ftpdata();
  if ftpcmd(b"SIZE\x00" as *const u8 as *const libc::c_char, server_path) != 213i32 {
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue = 0i32
  }
  if *local_path.offset(0) as libc::c_int == '-' as i32 && *local_path.offset(1) == 0 {
    fd_local = 1i32;
    (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue = 0i32
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue != 0 {
    let mut sbuf: stat = stat {
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
    /* lstat would be wrong here! */
    if stat(local_path, &mut sbuf) < 0i32 {
      bb_simple_perror_msg_and_die(b"stat\x00" as *const u8 as *const libc::c_char);
    }
    if sbuf.st_size > 0i32 as libc::c_long {
      beg_range = sbuf.st_size
    } else {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue = 0i32
    }
  }
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue != 0 {
    sprintf(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .buf
        .as_mut_ptr(),
      b"REST %lu\x00" as *const u8 as *const libc::c_char,
      beg_range,
    );
    if ftpcmd(
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
        .buf
        .as_mut_ptr(),
      0 as *const libc::c_char,
    ) != 350i32
    {
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue = 0i32
    }
  }
  if ftpcmd(b"RETR\x00" as *const u8 as *const libc::c_char, server_path) > 150i32 {
    ftp_die(b"RETR\x00" as *const u8 as *const libc::c_char);
  }
  /* create local file _after_ we know that remote file exists */
  if fd_local == -1i32 {
    fd_local = xopen(
      local_path,
      if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue != 0 {
        (0o2000i32) | 0o1i32
      } else {
        (0o100i32 | 0o1000i32) | 0o1i32
      },
    )
  }
  return pump_data_and_QUIT(fd_data, fd_local);
}
unsafe extern "C" fn ftp_send(
  mut server_path: *const libc::c_char,
  mut local_path: *mut libc::c_char,
) -> libc::c_int {
  let mut fd_data: libc::c_int = 0;
  let mut fd_local: libc::c_int = 0;
  let mut response: libc::c_int = 0;
  /* connect to the data socket */
  fd_data = xconnect_ftpdata();
  /* get the local file */
  fd_local = 0i32;
  if *local_path.offset(0) as libc::c_int != '-' as i32 || *local_path.offset(1) as libc::c_int != 0
  {
    fd_local = xopen(local_path, 0i32)
  }
  response = ftpcmd(b"STOR\x00" as *const u8 as *const libc::c_char, server_path);
  match response {
    125 | 150 => {}
    _ => {
      ftp_die(b"STOR\x00" as *const u8 as *const libc::c_char);
    }
  }
  return pump_data_and_QUIT(fd_local, fd_data);
}
static mut ftpgetput_longopts: [libc::c_char; 51] = [
  99, 111, 110, 116, 105, 110, 117, 101, 0, 1, 99, 118, 101, 114, 98, 111, 115, 101, 0, 0, 118,
  117, 115, 101, 114, 110, 97, 109, 101, 0, 1, 117, 112, 97, 115, 115, 119, 111, 114, 100, 0, 1,
  112, 112, 111, 114, 116, 0, 1, 80, 0,
];
#[no_mangle]
pub unsafe extern "C" fn ftpgetput_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut port: *const libc::c_char = b"ftp\x00" as *const u8 as *const libc::c_char;
  /* socket to ftp server */
  let mut ftp_action: Option<
    unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_char) -> libc::c_int,
  > = Some(
    ftp_send as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_char) -> libc::c_int,
  );
  /* Check to see if the command is ftpget or ftput */
  if *applet_name.offset(3) as libc::c_int == 'g' as i32 {
    ftp_action = Some(
      ftp_receive
        as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_char) -> libc::c_int,
    )
  }
  /* Set default values */
  let ref mut fresh4 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).user;
  *fresh4 = b"anonymous\x00" as *const u8 as *const libc::c_char;
  let ref mut fresh5 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).password;
  *fresh5 = b"busybox\x00" as *const u8 as *const libc::c_char;
  /*
   * Decipher the command line
   */
  /* must have 2 to 3 params; -v and -c count */
  getopt32long(
    argv,
    b"^cvu:p:P:\x00-2:?3:vv:cc\x00" as *const u8 as *const libc::c_char,
    ftpgetput_longopts.as_ptr(),
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).user as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).password as *mut *const libc::c_char,
    &mut port as *mut *const libc::c_char,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose_flag as *mut libc::c_int,
    &mut (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).do_continue as *mut libc::c_int,
  );
  argv = argv.offset(optind as isize);
  /* We want to do exactly _one_ DNS lookup, since some
   * sites (i.e. ftp.us.debian.org) use round-robin DNS
   * and we want to connect to only one IP... */
  let ref mut fresh6 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lsa;
  *fresh6 = xhost2sockaddr(
    *argv.offset(0),
    bb_lookup_port(
      port,
      b"tcp\x00" as *const u8 as *const libc::c_char,
      21i32 as libc::c_uint,
    ) as libc::c_int,
  );
  if (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).verbose_flag != 0 {
    printf(
      b"Connecting to %s (%s)\n\x00" as *const u8 as *const libc::c_char,
      *argv.offset(0),
      xmalloc_sockaddr2dotted(
        &mut (*(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).lsa)
          .u
          .sa,
      ),
    );
  }
  ftp_login();
  return ftp_action.expect("non-null function pointer")(
    *argv.offset(1),
    if !(*argv.offset(2)).is_null() {
      *argv.offset(2)
    } else {
      *argv.offset(1)
    },
  );
}