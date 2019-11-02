use libc;
extern "C" {
  #[no_mangle]
  fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
  #[no_mangle]
  static bb_errno: *mut libc::c_int;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
/* vi: set sw=4 ts=4: */
/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
pub type stat_func =
  Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int>;
/* vi: set sw=4 ts=4: */
/*
 * coreutils utility routine
 *
 * Copyright (C) 2003  Manuel Novoa III  <mjn3@codepoet.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */
#[no_mangle]
pub unsafe extern "C" fn cp_mv_stat2(
  mut fn_0: *const libc::c_char,
  mut fn_stat: *mut stat,
  mut sf: stat_func,
) -> libc::c_int {
  if sf.expect("non-null function pointer")(fn_0, fn_stat) < 0i32 {
    if *bb_errno != 2i32 {
      bb_perror_msg(
        b"can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
        fn_0,
      );
      return -1i32;
    }
    return 0i32;
  }
  if (*fn_stat).st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint {
    return 3i32;
  }
  return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn cp_mv_stat(
  mut fn_0: *const libc::c_char,
  mut fn_stat: *mut stat,
) -> libc::c_int {
  return cp_mv_stat2(
    fn_0,
    fn_stat,
    Some(stat as unsafe extern "C" fn(_: *const libc::c_char, _: *mut stat) -> libc::c_int),
  );
}