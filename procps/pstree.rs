use crate::libbb::xfuncs_printf::xmalloc;
use crate::librb::procps_status_t;
use crate::librb::smallint;
use libc;
use libc::pid_t;
use libc::putchar_unlocked;
use libc::sprintf;
use libc::strcmp;
use libc::strcpy;
use libc::uid_t;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static ptr_to_globals: *mut globals;

  #[no_mangle]
  static mut option_mask32: u32;

}

pub type C2RustUnnamed = libc::c_uint;
pub const PSSCAN_TASKS: C2RustUnnamed = 4194304;
pub const PSSCAN_RUIDGID: C2RustUnnamed = 2097152;
pub const PSSCAN_NICE: C2RustUnnamed = 1048576;
pub const PSSCAN_CPU: C2RustUnnamed = 524288;
pub const PSSCAN_START_TIME: C2RustUnnamed = 262144;
pub const PSSCAN_CONTEXT: C2RustUnnamed = 0;
pub const PSSCAN_ARGVN: C2RustUnnamed = 65536;
pub const PSSCAN_SMAPS: C2RustUnnamed = 32768;
pub const PSSCAN_TTY: C2RustUnnamed = 16384;
pub const PSSCAN_UTIME: C2RustUnnamed = 8192;
pub const PSSCAN_STIME: C2RustUnnamed = 4096;
pub const PSSCAN_RSS: C2RustUnnamed = 2048;
pub const PSSCAN_VSZ: C2RustUnnamed = 1024;
pub const PSSCAN_STATE: C2RustUnnamed = 512;
pub const PSSCAN_EXE: C2RustUnnamed = 256;
pub const PSSCAN_ARGV0: C2RustUnnamed = 128;
pub const PSSCAN_COMM: C2RustUnnamed = 32;
pub const PSSCAN_UIDGID: C2RustUnnamed = 16;
pub const PSSCAN_SID: C2RustUnnamed = 8;
pub const PSSCAN_PGID: C2RustUnnamed = 4;
pub const PSSCAN_PPID: C2RustUnnamed = 2;
pub const PSSCAN_PID: C2RustUnnamed = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub cur_x: libc::c_uint,
  pub output_width: libc::c_uint,
  pub capacity: libc::c_uint,
  pub width: *mut libc::c_uint,
  pub more: *mut u8,
  pub list: *mut PROC,
  pub dumped: smallint,
}
pub type PROC = proc_0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_0 {
  pub comm: [libc::c_char; 19],
  pub pid: pid_t,
  pub uid: uid_t,
  pub children: *mut child,
  pub parent: *mut proc_0,
  pub next: *mut proc_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct child {
  pub child: *mut PROC,
  pub next: *mut child,
}
pub type CHILD = child;
#[inline(always)]
unsafe fn not_const_pp(mut p: *const libc::c_void) -> *mut libc::c_void {
  return p as *mut libc::c_void;
}
/* used by dump_by_user */
/*
 * Allocates additional buffer space for width and more as needed.
 * The first call will allocate the first buffer.
 *
 * bufindex  the index that will be used after the call to this function.
 */
unsafe fn ensure_buffer_capacity(mut bufindex: libc::c_int) {
  if bufindex as libc::c_uint >= (*ptr_to_globals).capacity {
    (*ptr_to_globals).capacity = (*ptr_to_globals)
      .capacity
      .wrapping_add(0x100i32 as libc::c_uint);
    (*ptr_to_globals).width = crate::libbb::xfuncs_printf::xrealloc(
      (*ptr_to_globals).width as *mut libc::c_void,
      ((*ptr_to_globals).capacity as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    (*ptr_to_globals).more = crate::libbb::xfuncs_printf::xrealloc(
      (*ptr_to_globals).more as *mut libc::c_void,
      ((*ptr_to_globals).capacity as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<u8>() as libc::c_ulong),
    ) as *mut u8
  };
}
/* NB: this function is never called with "bad" chars
 * (control chars or chars >= 0x7f)
 */
unsafe fn out_char(mut c: libc::c_char) {
  (*ptr_to_globals).cur_x = (*ptr_to_globals).cur_x.wrapping_add(1);
  if (*ptr_to_globals).cur_x > (*ptr_to_globals).output_width {
    return;
  }
  if (*ptr_to_globals).cur_x == (*ptr_to_globals).output_width {
    c = '+' as i32 as libc::c_char
  }
  putchar_unlocked(c as libc::c_int);
}
/* NB: this function is never called with "bad" chars
 * (control chars or chars >= 0x7f)
 */
unsafe fn out_string(mut str: *const libc::c_char) {
  while *str != 0 {
    let fresh0 = str;
    str = str.offset(1);
    out_char(*fresh0);
  }
}
unsafe fn out_newline() {
  putchar_unlocked('\n' as i32);
  (*ptr_to_globals).cur_x = 0 as libc::c_uint;
}
unsafe fn find_proc(mut pid: pid_t) -> *mut PROC {
  let mut walk: *mut PROC = std::ptr::null_mut();
  walk = (*ptr_to_globals).list;
  while !walk.is_null() {
    if (*walk).pid == pid {
      break;
    }
    walk = (*walk).next
  }
  return walk;
}
unsafe fn new_proc(mut comm: *const libc::c_char, mut pid: pid_t, mut uid: uid_t) -> *mut PROC {
  let mut new: *mut PROC =
    crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<PROC>() as libc::c_ulong)
      as *mut PROC;
  strcpy((*new).comm.as_mut_ptr(), comm);
  (*new).pid = pid;
  (*new).uid = uid;
  (*new).next = (*ptr_to_globals).list;
  (*ptr_to_globals).list = new;
  return (*ptr_to_globals).list;
}
unsafe fn add_child(mut parent: *mut PROC, mut child: *mut PROC) {
  let mut new: *mut CHILD = std::ptr::null_mut();
  let mut walk: *mut *mut CHILD = std::ptr::null_mut();
  let mut cmp: libc::c_int = 0;
  new = xmalloc(::std::mem::size_of::<CHILD>() as libc::c_ulong) as *mut CHILD;
  (*new).child = child;
  walk = &mut (*parent).children;
  while !(*walk).is_null() {
    cmp = strcmp(
      (*(**walk).child).comm.as_mut_ptr(),
      (*child).comm.as_mut_ptr(),
    );
    if cmp > 0 {
      break;
    }
    if cmp == 0 && (*(**walk).child).uid > (*child).uid {
      break;
    }
    walk = &mut (**walk).next
  }
  (*new).next = *walk;
  *walk = new;
}
unsafe fn add_proc(mut comm: *const libc::c_char, mut pid: pid_t, mut ppid: pid_t, mut uid: uid_t)
/*, char isthread*/
{
  let mut this: *mut PROC = std::ptr::null_mut();
  let mut parent: *mut PROC = std::ptr::null_mut();
  this = find_proc(pid);
  if this.is_null() {
    this = new_proc(comm, pid, uid)
  } else {
    strcpy((*this).comm.as_mut_ptr(), comm);
    (*this).uid = uid
  }
  if pid == ppid {
    ppid = 0
  }
  //	if (isthread)
  //		this->flags |= PFLAG_THREAD;
  parent = find_proc(ppid);
  if parent.is_null() {
    parent = new_proc(
      b"?\x00" as *const u8 as *const libc::c_char,
      ppid,
      0 as uid_t,
    )
  }
  add_child(parent, this);
  (*this).parent = parent;
}
unsafe fn tree_equal(mut a: *const PROC, mut b: *const PROC) -> libc::c_int {
  let mut walk_a: *const CHILD = std::ptr::null();
  let mut walk_b: *const CHILD = std::ptr::null();
  if strcmp((*a).comm.as_ptr(), (*b).comm.as_ptr()) != 0 {
    return 0;
  }
  if option_mask32 != 0 && (*a).pid != (*b).pid {
    return 0;
  }
  walk_a = (*a).children;
  walk_b = (*b).children;
  while !walk_a.is_null() && !walk_b.is_null() {
    if tree_equal((*walk_a).child, (*walk_b).child) == 0 {
      return 0;
    }
    walk_a = (*walk_a).next;
    walk_b = (*walk_b).next
  }
  return !(!walk_a.is_null() || !walk_b.is_null()) as libc::c_int;
}
unsafe fn out_args(mut mystr: *const libc::c_char) -> libc::c_int {
  let mut here: *const libc::c_char = std::ptr::null();
  let mut strcount: libc::c_int = 0;
  let mut tmpstr: [libc::c_char; 5] = [0; 5];
  here = mystr;
  while *here != 0 {
    if *here as libc::c_int == '\\' as i32 {
      out_string(b"\\\\\x00" as *const u8 as *const libc::c_char);
      strcount += 2i32
    } else if *here as libc::c_int >= ' ' as i32 && (*here as libc::c_int) < 0x7fi32 {
      out_char(*here);
      strcount += 1
    } else {
      sprintf(
        tmpstr.as_mut_ptr(),
        b"\\%03o\x00" as *const u8 as *const libc::c_char,
        *here as libc::c_uchar as libc::c_int,
      );
      out_string(tmpstr.as_mut_ptr());
      strcount += 4i32
    }
    here = here.offset(1)
  }
  return strcount;
}
unsafe fn dump_tree(
  mut current: *mut PROC,
  mut level: libc::c_int,
  mut rep: libc::c_int,
  mut leaf: libc::c_int,
  mut last: libc::c_int,
  mut closing: libc::c_int,
) {
  let mut walk: *mut CHILD = std::ptr::null_mut();
  let mut next: *mut CHILD = std::ptr::null_mut();
  let mut scan: *mut *mut CHILD = std::ptr::null_mut();
  let mut lvl: libc::c_int = 0;
  let mut i: libc::c_int = 0;
  let mut add: libc::c_int = 0;
  let mut offset: libc::c_int = 0;
  let mut count: libc::c_int = 0;
  let mut comm_len: libc::c_int = 0;
  let mut first: libc::c_int = 0;
  let mut tmp: [libc::c_char; 16] = [0; 16];
  if current.is_null() {
    return;
  }
  if leaf == 0 {
    lvl = 0;
    while lvl < level {
      i = (*(*ptr_to_globals).width.offset(lvl as isize)).wrapping_add(1i32 as libc::c_uint)
        as libc::c_int;
      loop {
        i -= 1;
        if !(i >= 0) {
          break;
        }
        out_char(' ' as i32 as libc::c_char);
      }
      if lvl == level - 1i32 {
        if last != 0 {
          out_string(b"`-\x00" as *const u8 as *const libc::c_char);
        } else {
          out_string(b"|-\x00" as *const u8 as *const libc::c_char);
        }
      } else if *(*ptr_to_globals).more.offset((lvl + 1i32) as isize) != 0 {
        out_string(b"| \x00" as *const u8 as *const libc::c_char);
      } else {
        out_string(b"  \x00" as *const u8 as *const libc::c_char);
      }
      lvl += 1
    }
  }
  add = 0;
  if rep > 1i32 {
    add += sprintf(
      tmp.as_mut_ptr(),
      b"%d*[\x00" as *const u8 as *const libc::c_char,
      rep,
    );
    out_string(tmp.as_mut_ptr());
  }
  comm_len = out_args((*current).comm.as_mut_ptr());
  if option_mask32 != 0 {
    /*& OPT_PID*/
    comm_len += sprintf(
      tmp.as_mut_ptr(),
      b"(%d)\x00" as *const u8 as *const libc::c_char,
      (*current).pid,
    );
    out_string(tmp.as_mut_ptr());
  }
  offset = (*ptr_to_globals).cur_x as libc::c_int;
  if (*current).children.is_null() {
    loop {
      let fresh1 = closing;
      closing = closing - 1;
      if !(fresh1 != 0) {
        break;
      }
      out_char(']' as i32 as libc::c_char);
    }
    out_newline();
  }
  ensure_buffer_capacity(level);
  *(*ptr_to_globals).more.offset(level as isize) = (last == 0) as libc::c_int as u8;
  *(*ptr_to_globals).width.offset(level as isize) = (comm_len as libc::c_uint)
    .wrapping_add((*ptr_to_globals).cur_x)
    .wrapping_sub(offset as libc::c_uint)
    .wrapping_add(add as libc::c_uint);
  if (*ptr_to_globals).cur_x >= (*ptr_to_globals).output_width {
    //out_string(first_3); - why? it won't print anything
    //out_char('+');
    out_newline();
    return;
  }
  first = 1i32;
  walk = (*current).children;
  while !walk.is_null() {
    count = 0;
    next = (*walk).next;
    scan = &mut (*walk).next;
    while !(*scan).is_null() {
      if tree_equal((*walk).child, (**scan).child) == 0 {
        scan = &mut (**scan).next
      } else {
        if next == *scan {
          next = (**scan).next
        }
        count += 1;
        *scan = (**scan).next
      }
    }
    if first != 0 {
      out_string(if !next.is_null() {
        b"-+-\x00" as *const u8 as *const libc::c_char
      } else {
        b"---\x00" as *const u8 as *const libc::c_char
      });
      first = 0
    }
    dump_tree(
      (*walk).child,
      level + 1i32,
      count + 1i32,
      (walk == (*current).children) as libc::c_int,
      next.is_null() as libc::c_int,
      closing + (if count != 0 { 1i32 } else { 0 }),
    );
    walk = next
  }
}
unsafe fn dump_by_user(mut current: *mut PROC, mut uid: uid_t) {
  let mut walk: *const CHILD = std::ptr::null();
  if current.is_null() {
    return;
  }
  if (*current).uid == uid {
    if (*ptr_to_globals).dumped != 0 {
      putchar_unlocked('\n' as i32);
    }
    dump_tree(current, 0, 1i32, 1i32, 1i32, 0);
    (*ptr_to_globals).dumped = 1i32 as smallint;
    return;
  }
  walk = (*current).children;
  while !walk.is_null() {
    dump_by_user((*walk).child, uid);
    walk = (*walk).next
  }
}
unsafe fn handle_thread(
  mut comm: *const libc::c_char,
  mut pid: pid_t,
  mut ppid: pid_t,
  mut uid: uid_t,
) {
  let mut threadname: [libc::c_char; 19] = [0; 19];
  sprintf(
    threadname.as_mut_ptr(),
    b"{%.*s}\x00" as *const u8 as *const libc::c_char,
    ::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as libc::c_int - 3i32,
    comm,
  );
  add_proc(threadname.as_mut_ptr(), pid, ppid, uid);
}
unsafe fn mread_proc() {
  let mut p: *mut procps_status_t = std::ptr::null_mut();
  let mut parent: pid_t = 0;
  let mut flags: libc::c_int = PSSCAN_COMM as libc::c_int
    | PSSCAN_PID as libc::c_int
    | PSSCAN_PPID as libc::c_int
    | PSSCAN_UIDGID as libc::c_int
    | PSSCAN_TASKS as libc::c_int;
  loop {
    p = crate::libbb::procps::procps_scan(p, flags);
    if p.is_null() {
      break;
    }
    if (*p).pid != (*p).main_thread_pid {
      handle_thread((*p).comm.as_mut_ptr(), (*p).pid as pid_t, parent, (*p).uid);
    } else {
      add_proc(
        (*p).comm.as_mut_ptr(),
        (*p).pid as pid_t,
        (*p).ppid as pid_t,
        (*p).uid,
      );
      parent = (*p).pid as pid_t
    }
  }
}
pub unsafe fn pstree_main(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
  let mut pid: pid_t = 1i32;
  let mut uid: libc::c_long = 0;
  let ref mut fresh2 = *(not_const_pp(&ptr_to_globals as *const *mut globals as *const libc::c_void)
    as *mut *mut globals);
  *fresh2 = crate::libbb::xfuncs_printf::xzalloc(::std::mem::size_of::<globals>() as libc::c_ulong)
    as *mut globals;
  asm!("" : : : "memory" : "volatile");
  (*ptr_to_globals).output_width = crate::libbb::xfuncs::get_terminal_width(0i32) as libc::c_uint;
  crate::libbb::getopt32::getopt32(argv, b"^p\x00?1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null() {
    if *(*argv.offset(0)).offset(0) as libc::c_int >= '0' as i32
      && *(*argv.offset(0)).offset(0) as libc::c_int <= '9' as i32
    {
      pid = crate::libbb::xatonum::xatoi(*argv.offset(0))
    } else {
      uid = crate::libbb::bb_pwd::xuname2uid(*argv.offset(0))
    }
  }
  mread_proc();
  if uid == 0 {
    dump_tree(find_proc(pid), 0, 1i32, 1i32, 1i32, 0);
  } else {
    dump_by_user(find_proc(1i32), uid as uid_t);
    if (*ptr_to_globals).dumped == 0 {
      crate::libbb::verror_msg::bb_simple_error_msg_and_die(
        b"no processes found\x00" as *const u8 as *const libc::c_char,
      );
    }
  }
  return 0;
}
