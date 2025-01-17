use crate::libbb::appletlib::applet_name;
use crate::libbb::ptr_to_globals::bb_errno;
use crate::libbb::skip_whitespace::skip_whitespace;
use c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use libc;
use libc::close;
use libc::closedir;
use libc::fclose;
use libc::fprintf;
use libc::free;
use libc::getenv;
use libc::open;
use libc::printf;
use libc::puts;
use libc::readdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
extern "C" {

  #[no_mangle]
  fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;

  #[no_mangle]
  fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn vfork() -> libc::c_int;

  #[no_mangle]
  fn _exit(_: libc::c_int) -> !;

  #[no_mangle]
  fn execle(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;

  #[no_mangle]
  fn usleep(__useconds: useconds_t) -> libc::c_int;

  #[no_mangle]
  fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;

  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;

  #[no_mangle]
  fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;

  #[no_mangle]
  static mut option_mask32: u32;

  #[no_mangle]
  static mut bb_common_bufsiz1: [libc::c_char; 0];

  #[no_mangle]
  fn fnmatch(
    __pattern: *const libc::c_char,
    __name: *const libc::c_char,
    __flags: libc::c_int,
  ) -> libc::c_int;
}

use crate::librb::size_t;
use libc::pid_t;
use libc::useconds_t;

use libc::in_addr;
pub type in_addr_t = u32;
use crate::libbb::llist::llist_t;
use crate::librb::fd_pair;
use crate::librb::smallint;
use libc::dirent;
use libc::DIR;
use libc::FILE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
  pub my_environ: *mut *mut libc::c_char,
  pub startup_PATH: *const libc::c_char,
  pub shell: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct interface_defn_t {
  pub address_family: *const address_family_t,
  pub method: *const method_t,
  pub iface: *mut libc::c_char,
  pub n_options: libc::c_int,
  pub option: *mut variable_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct variable_t {
  pub name: *mut libc::c_char,
  pub value: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct method_t {
  pub name: *const libc::c_char,
  pub up: Option<
    unsafe fn(
      _: *mut interface_defn_t,
      _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
    ) -> libc::c_int,
  >,
  pub down: Option<
    unsafe fn(
      _: *mut interface_defn_t,
      _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
    ) -> libc::c_int,
  >,
}
pub type execfn = unsafe fn(_: *mut libc::c_char) -> libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct address_family_t {
  pub name: *const libc::c_char,
  pub n_methods: libc::c_int,
  pub method: *const method_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapping_defn_t {
  pub next: *mut mapping_defn_t,
  pub max_matches: libc::c_int,
  pub n_matches: libc::c_int,
  pub match_0: *mut *mut libc::c_char,
  pub script: *mut libc::c_char,
  pub n_mappings: libc::c_int,
  pub mapping: *mut *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct interfaces_file_t {
  pub autointerfaces: *mut llist_t,
  pub ifaces: *mut llist_t,
  pub mappings: *mut mapping_defn_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_no_mappings: C2RustUnnamed = 16;
pub const OPT_force: C2RustUnnamed = 8;
pub const OPT_verbose: C2RustUnnamed = 4;
pub const OPT_no_act: C2RustUnnamed = 2;
pub const OPT_do_all: C2RustUnnamed = 1;
pub const NONE: C2RustUnnamed_0 = 0;
pub const MAPPING: C2RustUnnamed_0 = 2;
pub const IFACE: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline(always)]
unsafe fn bb_ascii_isalnum(mut a: libc::c_uchar) -> libc::c_int {
  let mut b: libc::c_uchar = (a as libc::c_int - '0' as i32) as libc::c_uchar;
  if b as libc::c_int <= 9i32 {
    return (b as libc::c_int <= 9i32) as libc::c_int;
  }
  b = ((a as libc::c_int | 0x20i32) - 'a' as i32) as libc::c_uchar;
  return (b as libc::c_int <= 'z' as i32 - 'a' as i32) as libc::c_int;
}
static mut keywords_up_down: [libc::c_char; 26] = [
  117, 112, 0, 100, 111, 119, 110, 0, 112, 114, 101, 45, 117, 112, 0, 112, 111, 115, 116, 45, 100,
  111, 119, 110, 0, 0,
];
unsafe fn addstr(
  mut bufp: *mut *mut libc::c_char,
  mut str: *const libc::c_char,
  mut str_length: size_t,
) {
  /* xasprintf trick will be smaller, but we are often
   * called with str_length == 1 - don't want to have
   * THAT much of malloc/freeing! */
  let mut buf: *mut libc::c_char = *bufp;
  let mut len: libc::c_int = if !buf.is_null() {
    strlen(buf)
  } else {
    0 as libc::c_ulong
  } as libc::c_int;
  str_length = str_length.wrapping_add(1);
  buf = crate::libbb::xfuncs_printf::xrealloc(
    buf as *mut libc::c_void,
    (len as libc::c_ulong).wrapping_add(str_length),
  ) as *mut libc::c_char;
  /* copies at most str_length-1 chars! */
  crate::libbb::safe_strncpy::safe_strncpy(buf.offset(len as isize), str, str_length);
  *bufp = buf;
}
unsafe fn strncmpz(
  mut l: *const libc::c_char,
  mut r: *const libc::c_char,
  mut llen: size_t,
) -> libc::c_int {
  let mut i: libc::c_int = strncmp(l, r, llen);
  if i == 0 {
    return -(*r.offset(llen as isize) as libc::c_uchar as libc::c_int);
  }
  return i;
}
unsafe fn get_var(
  mut id: *const libc::c_char,
  mut idlen: size_t,
  mut ifd: *mut interface_defn_t,
) -> *mut libc::c_char {
  let mut i: libc::c_int = 0;
  if strncmpz(id, b"iface\x00" as *const u8 as *const libc::c_char, idlen) == 0 {
    // ubuntu's ifup doesn't do this:
    //static char *label_buf;
    //char *result;
    //free(label_buf);
    //label_buf = xstrdup(ifd->iface);
    // Remove virtual iface suffix
    //result = strchrnul(label_buf, ':');
    //*result = '\0';
    //return label_buf;
    return (*ifd).iface;
  }
  if strncmpz(id, b"label\x00" as *const u8 as *const libc::c_char, idlen) == 0 {
    return (*ifd).iface;
  }
  i = 0;
  while i < (*ifd).n_options {
    if strncmpz(id, (*(*ifd).option.offset(i as isize)).name, idlen) == 0 {
      return (*(*ifd).option.offset(i as isize)).value;
    }
    i += 1
  }
  return std::ptr::null_mut::<libc::c_char>();
}
unsafe fn count_netmask_bits(mut dotted_quad: *const libc::c_char) -> libc::c_int {
  //	int result;
  //	unsigned a, b, c, d;
  //	/* Found a netmask...  Check if it is dotted quad */
  //	if (sscanf(dotted_quad, "%u.%u.%u.%u", &a, &b, &c, &d) != 4)
  //		return -1;
  //	if ((a|b|c|d) >> 8)
  //		return -1; /* one of numbers is >= 256 */
  //	d |= (a << 24) | (b << 16) | (c << 8); /* IP */
  //	d = ~d; /* 11110000 -> 00001111 */
  /* Shorter version */
  let mut result: libc::c_int = 0; /* malformed dotted IP */
  let mut ip: in_addr = in_addr { s_addr: 0 }; /* IP in host order */
  let mut d: libc::c_uint = 0; /* 11110000 -> 00001111 */
  if inet_aton(dotted_quad, &mut ip) == 0 {
    return -1i32;
  } /* no it is not */
  d = {
    let mut __v: libc::c_uint = 0;
    let mut __x: libc::c_uint = ip.s_addr;
    if false {
      __v = (__x & 0xff000000u32) >> 24i32
        | (__x & 0xff0000i32 as libc::c_uint) >> 8i32
        | (__x & 0xff00i32 as libc::c_uint) << 8i32
        | (__x & 0xffi32 as libc::c_uint) << 24i32
    } else {
      let fresh0 = &mut __v;
      let fresh1;
      let fresh2 = __x;
      asm!("bswap $0" : "=r" (fresh1) : "0"
     (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
      c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    }
    __v
  };
  d = !d;
  if d & d.wrapping_add(1i32 as libc::c_uint) != 0 {
    /* check that it is in 00001111 form */
    return -1i32;
  }
  result = 32i32;
  while d != 0 {
    d >>= 1i32;
    result -= 1
  }
  return result;
}
unsafe fn parse(
  mut command: *const libc::c_char,
  mut ifd: *mut interface_defn_t,
) -> *mut libc::c_char {
  let mut old_pos: [size_t; 10] = [0i32 as size_t, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let mut okay: [smallint; 10] = [1i32 as smallint, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let mut opt_depth: libc::c_int = 1i32;
  let mut result: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  while *command != 0 {
    let mut current_block_42: u64;
    match *command as libc::c_int {
      92 => {
        if *command.offset(1) != 0 {
          command = command.offset(1)
        }
        addstr(&mut result, command, 1i32 as size_t);
        command = command.offset(1)
      }
      91 => {
        if *command.offset(1) as libc::c_int == '[' as i32 && opt_depth < 10i32 {
          old_pos[opt_depth as usize] = if !result.is_null() {
            strlen(result)
          } else {
            0 as libc::c_ulong
          };
          okay[opt_depth as usize] = 1i32 as smallint;
          opt_depth += 1;
          command = command.offset(2)
        } else {
          addstr(&mut result, command, 1i32 as size_t);
          command = command.offset(1)
        }
      }
      93 => {
        if *command.offset(1) as libc::c_int == ']' as i32 && opt_depth > 1i32 {
          opt_depth -= 1;
          if okay[opt_depth as usize] == 0 {
            *result.offset(old_pos[opt_depth as usize] as isize) = '\u{0}' as i32 as libc::c_char
          }
          command = command.offset(2)
        } else {
          addstr(&mut result, command, 1i32 as size_t);
          command = command.offset(1)
        }
      }
      37 => {
        let mut nextpercent: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        let mut varvalue: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        command = command.offset(1);
        nextpercent = strchr(command, '%' as i32);
        if nextpercent.is_null() {
          /* Unterminated %var% */
          free(result as *mut libc::c_void);
          return std::ptr::null_mut::<libc::c_char>();
        }
        varvalue = get_var(
          command,
          nextpercent.wrapping_offset_from(command) as libc::c_long as size_t,
          ifd,
        );
        if !varvalue.is_null() {
          /* "hwaddress <class> <address>":
           * unlike ifconfig, ip doesnt want <class>
           * (usually "ether" keyword). Skip it. */
          if !crate::libbb::compare_string_array::is_prefixed_with(
            command,
            b"hwaddress\x00" as *const u8 as *const libc::c_char,
          )
          .is_null()
          {
            varvalue = skip_whitespace(crate::libbb::skip_whitespace::skip_non_whitespace(varvalue))
          }
          addstr(&mut result, varvalue, strlen(varvalue));
          current_block_42 = 5892776923941496671;
        } else {
          /* Sigh...  Add a special case for 'ip' to convert from
           * dotted quad to bit count style netmasks.  */
          if !crate::libbb::compare_string_array::is_prefixed_with(
            command,
            b"bnmask\x00" as *const u8 as *const libc::c_char,
          )
          .is_null()
          {
            let mut res: libc::c_uint = 0;
            varvalue = get_var(
              b"netmask\x00" as *const u8 as *const libc::c_char,
              7i32 as size_t,
              ifd,
            );
            if !varvalue.is_null() {
              res = count_netmask_bits(varvalue) as libc::c_uint;
              if res > 0 as libc::c_uint {
                let mut argument: *const libc::c_char = crate::libbb::xfuncs::utoa(res);
                addstr(&mut result, argument, strlen(argument));
                command = nextpercent.offset(1);
                current_block_42 = 9353995356876505083;
              } else {
                current_block_42 = 5141539773904409130;
              }
            } else {
              current_block_42 = 5141539773904409130;
            }
          } else {
            current_block_42 = 5141539773904409130;
          }
          match current_block_42 {
            9353995356876505083 => {}
            _ => {
              okay[(opt_depth - 1i32) as usize] = 0 as smallint;
              current_block_42 = 5892776923941496671;
            }
          }
        }
        match current_block_42 {
          9353995356876505083 => {}
          _ => command = nextpercent.offset(1),
        }
      }
      _ => {
        addstr(&mut result, command, 1i32 as size_t);
        command = command.offset(1)
      }
    }
  }
  if opt_depth > 1i32 {
    /* Unbalanced bracket */
    free(result as *mut libc::c_void);
    return std::ptr::null_mut::<libc::c_char>();
  }
  if okay[0] == 0 {
    /* Undefined variable and we aren't in a bracket */
    free(result as *mut libc::c_void);
    return std::ptr::null_mut::<libc::c_char>();
  }
  return result;
}
/* execute() returns 1 for success and 0 for failure */
unsafe fn execute(
  mut command: *const libc::c_char,
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut out: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut ret: libc::c_int = 0;
  out = parse(command, ifd);
  if out.is_null() {
    /* parse error? */
    return 0;
  }
  /* out == "": parsed ok but not all needed variables known, skip */
  ret = if *out.offset(0) as libc::c_int != 0 {
    Some(exec.expect("non-null function pointer")).expect("non-null function pointer")(out)
  } else {
    1i32
  };
  free(out as *mut libc::c_void);
  if ret != 1i32 {
    return 0;
  }
  return 1i32;
}
/* FEATURE_IFUPDOWN_IPV4 || FEATURE_IFUPDOWN_IPV6 */
unsafe fn loopback_up6(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  result = execute(
    b"ip addr add ::1 dev %iface%\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"ip link set %iface% up\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  return if result == 2i32 { 2i32 } else { 0 };
}
unsafe fn loopback_down6(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return execute(
    b"ip link set %iface% down\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
}
unsafe fn manual_up_down6(
  mut _ifd: *mut interface_defn_t,
  mut _exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return 1i32;
}
unsafe fn static_up6(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  result = execute(
    b"ip addr add %address%/%netmask% dev %iface%[[ label %label%]]\x00" as *const u8
      as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"ip link set[[ mtu %mtu%]][[ addr %hwaddress%]] %iface% up\x00" as *const u8
      as *const libc::c_char,
    ifd,
    exec,
  );
  /* Reportedly, IPv6 needs "dev %iface%", but IPv4 does not: */
  result += execute(
    b"[[ip route add ::/0 via %gateway% dev %iface%]][[ metric %metric%]]\x00" as *const u8
      as *const libc::c_char,
    ifd,
    exec,
  ); /* already gone */
  return if result == 3i32 { 3i32 } else { 0 };
}
unsafe fn static_down6(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  if if_nametoindex((*ifd).iface) == 0 {
    return 1i32;
  }
  return execute(
    b"ip link set %iface% down\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
}
unsafe fn v4tunnel_up(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  result = execute(
    b"ip tunnel add %iface% mode sit remote %endpoint%[[ local %local%]][[ ttl %ttl%]]\x00"
      as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"ip link set %iface% up\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"ip addr add %address%/%netmask% dev %iface%\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  /* Reportedly, IPv6 needs "dev %iface%", but IPv4 does not: */
  result += execute(
    b"[[ip route add ::/0 via %gateway% dev %iface%]]\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  return if result == 4i32 { 4i32 } else { 0 };
}
unsafe fn v4tunnel_down(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return execute(
    b"ip tunnel del %iface%\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
}
static mut methods6: [method_t; 4] = [
  {
    let mut init = method_t {
      name: b"v4tunnel\x00" as *const u8 as *const libc::c_char,
      up: Some(
        v4tunnel_up
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        v4tunnel_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"static\x00" as *const u8 as *const libc::c_char,
      up: Some(
        static_up6
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        static_down6
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"manual\x00" as *const u8 as *const libc::c_char,
      up: Some(
        manual_up_down6
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        manual_up_down6
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"loopback\x00" as *const u8 as *const libc::c_char,
      up: Some(
        loopback_up6
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        loopback_down6
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
];

// Initialized in run_static_initializers
static mut addr_inet6: address_family_t = address_family_t {
  name: 0 as *const libc::c_char,
  n_methods: 0,
  method: 0 as *const method_t,
};
/* FEATURE_IFUPDOWN_IPV6 */
unsafe fn loopback_up(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0; /* already gone */
  result = execute(
    b"ip addr add 127.0.0.1/8 dev %iface%\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"ip link set %iface% up\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  return if result == 2i32 { 2i32 } else { 0 };
}
unsafe fn loopback_down(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  result = execute(
    b"ip addr flush dev %iface%\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"ip link set %iface% down\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  return if result == 2i32 { 2i32 } else { 0 };
}
unsafe fn static_up(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  result =
        execute(b"ip addr add %address%/%bnmask%[[ broadcast %broadcast%]] dev %iface%[[ peer %pointopoint%]][[ label %label%]]\x00"
                    as *const u8 as *const libc::c_char, ifd, exec);
  result += execute(
    b"ip link set[[ mtu %mtu%]][[ addr %hwaddress%]] %iface% up\x00" as *const u8
      as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"[[ip route add default via %gateway% dev %iface%[[ metric %metric%]]]]\x00" as *const u8
      as *const libc::c_char,
    ifd,
    exec,
  );
  return if result == 3i32 { 3i32 } else { 0 };
}
unsafe fn static_down(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  if if_nametoindex((*ifd).iface) == 0 {
    return 2i32;
  }
  /* Optional "label LBL" is necessary if interface is an alias (eth0:0),
   * otherwise "ip addr flush dev eth0:0" flushes all addresses on eth0.
   */
  result = execute(
    b"ip addr flush dev %iface%[[ label %label%]]\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  result += execute(
    b"ip link set %iface% down\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  return if result == 2i32 { 2i32 } else { 0 };
}
/* FEATURE_IFUPDOWN_EXTERNAL_DHCPC */
unsafe fn dhcp_up(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  /* ip doesn't up iface when it configures it (unlike ifconfig) */
  if execute(
    b"ip link set[[ addr %hwaddress%]] %iface% up\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  ) == 0
  {
    return 0;
  }
  return execute(b"udhcpc -R -n -p /var/run/udhcpc.%iface%.pid -i %iface%[[ -x hostname:%hostname%]][[ -c %client%]][[ -s %script%]][[ %udhcpc_opts%]]\x00"
                       as *const u8 as *const libc::c_char, ifd, exec);
}
unsafe fn dhcp_down(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  let mut result: libc::c_int = 0;
  result = execute(
    b"test -f /var/run/udhcpc.%iface%.pid && kill `cat /var/run/udhcpc.%iface%.pid` 2>/dev/null\x00"
      as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
  /* Also bring the hardware interface down since
  killing the dhcp client alone doesn't do it.
  This enables consecutive ifup->ifdown->ifup */
  /* Sleep a bit, otherwise static_down tries to bring down interface too soon,
  and it may come back up because udhcpc is still shutting down */
  usleep(100000i32 as useconds_t);
  result += static_down(ifd, exec);
  return if result == 3i32 { 3i32 } else { 0 };
}
unsafe fn manual_up_down(
  mut _ifd: *mut interface_defn_t,
  mut _exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return 1i32;
}
unsafe fn bootp_up(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return execute(b"bootpc[[ --bootfile %bootfile%]] --dev %iface%[[ --server %server%]][[ --hwaddr %hwaddr%]] --returniffail --serverbcast\x00"
                       as *const u8 as *const libc::c_char, ifd, exec);
}
unsafe fn ppp_up(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return execute(
    b"pon[[ %provider%]]\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
}
unsafe fn ppp_down(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return execute(
    b"poff[[ %provider%]]\x00" as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
}
unsafe fn wvdial_up(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return execute(
    b"start-stop-daemon --start -x wvdial -p /var/run/wvdial.%iface% -b -m --[[ %provider%]]\x00"
      as *const u8 as *const libc::c_char,
    ifd,
    exec,
  );
}
unsafe fn wvdial_down(
  mut ifd: *mut interface_defn_t,
  mut exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return execute(
    b"start-stop-daemon --stop -x wvdial -p /var/run/wvdial.%iface% -s 2\x00" as *const u8
      as *const libc::c_char,
    ifd,
    exec,
  );
}

static mut methods: [method_t; 7] = [
  {
    let mut init = method_t {
      name: b"manual\x00" as *const u8 as *const libc::c_char,
      up: Some(
        manual_up_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        manual_up_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"wvdial\x00" as *const u8 as *const libc::c_char,
      up: Some(
        wvdial_up
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        wvdial_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"ppp\x00" as *const u8 as *const libc::c_char,
      up: Some(
        ppp_up
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        ppp_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"static\x00" as *const u8 as *const libc::c_char,
      up: Some(
        static_up
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        static_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"bootp\x00" as *const u8 as *const libc::c_char,
      up: Some(
        bootp_up
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        static_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"dhcp\x00" as *const u8 as *const libc::c_char,
      up: Some(
        dhcp_up
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        dhcp_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
  {
    let mut init = method_t {
      name: b"loopback\x00" as *const u8 as *const libc::c_char,
      up: Some(
        loopback_up
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
      down: Some(
        loopback_down
          as unsafe fn(
            _: *mut interface_defn_t,
            _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
          ) -> libc::c_int,
      ),
    };
    init
  },
];

// Initialized in run_static_initializers
static mut addr_inet: address_family_t = address_family_t {
  name: 0 as *const libc::c_char,
  n_methods: 0,
  method: 0 as *const method_t,
};

/* FEATURE_IFUPDOWN_IPV4 */
unsafe fn link_up_down(
  mut _ifd: *mut interface_defn_t,
  mut _exec: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
) -> libc::c_int {
  return 1i32;
}

static mut link_methods: [method_t; 1] = [{
  let mut init = method_t {
    name: b"none\x00" as *const u8 as *const libc::c_char,
    up: Some(
      link_up_down
        as unsafe fn(
          _: *mut interface_defn_t,
          _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
        ) -> libc::c_int,
    ),
    down: Some(
      link_up_down
        as unsafe fn(
          _: *mut interface_defn_t,
          _: Option<unsafe fn(_: *mut libc::c_char) -> libc::c_int>,
        ) -> libc::c_int,
    ),
  };
  init
}];

// Initialized in run_static_initializers
static mut addr_link: address_family_t = address_family_t {
  name: 0 as *const libc::c_char,
  n_methods: 0,
  method: 0 as *const method_t,
};
/* Returns pointer to the next word, or NULL.
 * In 1st case, advances *buf to the word after this one.
 */
unsafe fn next_word(mut buf: *mut *mut libc::c_char) -> *mut libc::c_char {
  let mut length: libc::c_uint = 0;
  let mut word: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  /* Skip over leading whitespace */
  word = skip_whitespace(*buf);
  /* Stop on EOL */
  if *word as libc::c_int == '\u{0}' as i32 {
    return std::ptr::null_mut::<libc::c_char>();
  }
  /* Find the length of this word (can't be 0) */
  length = strcspn(word, b" \t\n\x00" as *const u8 as *const libc::c_char) as libc::c_uint;
  /* Unless we are already at NUL, store NUL and advance */
  if *word.offset(length as isize) as libc::c_int != '\u{0}' as i32 {
    let fresh3 = length;
    length = length.wrapping_add(1);
    *word.offset(fresh3 as isize) = '\u{0}' as i32 as libc::c_char
  }
  *buf = skip_whitespace(word.offset(length as isize));
  return word;
}
unsafe fn get_address_family(
  mut af: *const *const address_family_t,
  mut name: *mut libc::c_char,
) -> *const address_family_t {
  let mut i: libc::c_int = 0;
  if name.is_null() {
    return std::ptr::null();
  }
  i = 0;
  while !(*af.offset(i as isize)).is_null() {
    if strcmp((**af.offset(i as isize)).name, name) == 0 {
      return *af.offset(i as isize);
    }
    i += 1
  }
  return std::ptr::null();
}
unsafe fn get_method(
  mut af: *const address_family_t,
  mut name: *mut libc::c_char,
) -> *const method_t {
  let mut i: libc::c_int = 0;
  if name.is_null() {
    return std::ptr::null();
  }
  /* TODO: use index_in_str_array() */
  i = 0;
  while i < (*af).n_methods {
    if strcmp((*(*af).method.offset(i as isize)).name, name) == 0 {
      return &*(*af).method.offset(i as isize) as *const method_t;
    }
    i += 1
  }
  return std::ptr::null();
}
unsafe fn read_interfaces(
  mut filename: *const libc::c_char,
  mut defn: *mut interfaces_file_t,
) -> *mut interfaces_file_t {
  /* Let's try to be compatible.
   *
   * "man 5 interfaces" says:
   * Lines starting with "#" are ignored. Note that end-of-line
   * comments are NOT supported, comments must be on a line of their own.
   * A line may be extended across multiple lines by making
   * the last character a backslash.
   *
   * Seen elsewhere in example config file:
   * A first non-blank "#" character makes the rest of the line
   * be ignored. Blank lines are ignored. Lines may be indented freely.
   * A "\" character at the very end of the line indicates the next line
   * should be treated as a continuation of the current one.
   *
   * Lines  beginning with "source" are used to include stanzas from
   * other files, so configuration can be split into many files.
   * The word "source" is followed by the path of file to be sourced.
   */
  let mut currmap: *mut mapping_defn_t = std::ptr::null_mut(); /* while (fgets) */
  let mut currif: *mut interface_defn_t = std::ptr::null_mut();
  let mut f: *mut FILE = std::ptr::null_mut();
  let mut buf: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut first_word: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut rest_of_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut currently_processing: C2RustUnnamed_0 = NONE;
  if defn.is_null() {
    defn = crate::libbb::xfuncs_printf::xzalloc(
      ::std::mem::size_of::<interfaces_file_t>() as libc::c_ulong
    ) as *mut interfaces_file_t
  }
  f = crate::libbb::wfopen::xfopen_for_read(filename);
  loop {
    buf = crate::libbb::get_line_from_file::xmalloc_fgetline(f);
    if buf.is_null() {
      break;
    }
    /* Trailing "\" concatenates lines */
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    loop {
      p = crate::libbb::last_char_is::last_char_is(buf, '\\' as i32);
      if p.is_null() {
        break;
      }
      *p = '\u{0}' as i32 as libc::c_char;
      rest_of_line = crate::libbb::get_line_from_file::xmalloc_fgetline(f);
      if rest_of_line.is_null() {
        break;
      }
      p = crate::libbb::xfuncs_printf::xasprintf(
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        buf,
        rest_of_line,
      );
      free(buf as *mut libc::c_void);
      free(rest_of_line as *mut libc::c_void);
      buf = p
    }
    rest_of_line = buf;
    first_word = next_word(&mut rest_of_line);
    if first_word.is_null() || *first_word as libc::c_int == '#' as i32 {
      free(buf as *mut libc::c_void);
    /* blank/comment line */
    } else {
      if strcmp(
        first_word,
        b"mapping\x00" as *const u8 as *const libc::c_char,
      ) == 0
      {
        currmap = crate::libbb::xfuncs_printf::xzalloc(
          ::std::mem::size_of::<mapping_defn_t>() as libc::c_ulong
        ) as *mut mapping_defn_t;
        loop {
          first_word = next_word(&mut rest_of_line);
          if first_word.is_null() {
            break;
          }
          (*currmap).match_0 = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
            (*currmap).match_0 as *mut libc::c_void,
            ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
              .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
            (*currmap).n_matches,
          ) as *mut *mut libc::c_char;
          let fresh4 = (*currmap).n_matches;
          (*currmap).n_matches = (*currmap).n_matches + 1;
          let ref mut fresh5 = *(*currmap).match_0.offset(fresh4 as isize);
          *fresh5 = crate::libbb::xfuncs_printf::xstrdup(first_word)
        }
        /*currmap->n_mappings = 0;*/
        /*currmap->mapping = NULL;*/
        /*currmap->script = NULL;*/
        let mut where_0: *mut *mut mapping_defn_t = &mut (*defn).mappings;
        while !(*where_0).is_null() {
          where_0 = &mut (**where_0).next
        }
        *where_0 = currmap;
        currently_processing = MAPPING
      /*currmap->next = NULL;*/
      } else if strcmp(first_word, b"iface\x00" as *const u8 as *const libc::c_char) == 0 {
        static mut addr_fams: [*const address_family_t; 4] = unsafe {
          [
            &addr_inet as *const address_family_t,
            &addr_inet6 as *const address_family_t,
            &addr_link as *const address_family_t,
            0 as *const address_family_t,
          ]
        };
        let mut iface_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        let mut address_family_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        let mut method_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        currif = crate::libbb::xfuncs_printf::xzalloc(
          ::std::mem::size_of::<interface_defn_t>() as libc::c_ulong
        ) as *mut interface_defn_t;
        iface_name = next_word(&mut rest_of_line);
        address_family_name = next_word(&mut rest_of_line);
        method_name = next_word(&mut rest_of_line);
        if method_name.is_null() {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"too few parameters for line \"%s\"\x00" as *const u8 as *const libc::c_char,
            buf,
          );
        }
        /* ship any trailing whitespace */
        rest_of_line = skip_whitespace(rest_of_line);
        if *rest_of_line.offset(0) as libc::c_int != '\u{0}' as i32 {
          /* && rest_of_line[0] != '#' */
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"too many parameters \"%s\"\x00" as *const u8 as *const libc::c_char,
            buf,
          );
        }
        (*currif).iface = crate::libbb::xfuncs_printf::xstrdup(iface_name);
        (*currif).address_family = get_address_family(addr_fams.as_ptr(), address_family_name);
        if (*currif).address_family.is_null() {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"unknown address type \"%s\"\x00" as *const u8 as *const libc::c_char,
            address_family_name,
          );
        }
        (*currif).method = get_method((*currif).address_family, method_name);
        if (*currif).method.is_null() {
          crate::libbb::verror_msg::bb_error_msg_and_die(
            b"unknown method \"%s\"\x00" as *const u8 as *const libc::c_char,
            method_name,
          );
        }
        crate::libbb::llist::llist_add_to_end(
          &mut (*defn).ifaces,
          currif as *mut libc::c_char as *mut libc::c_void,
        );
        currently_processing = IFACE
      } else if strcmp(first_word, b"auto\x00" as *const u8 as *const libc::c_char) == 0 {
        loop {
          first_word = next_word(&mut rest_of_line);
          if first_word.is_null() {
            break;
          }
          /* Check the interface isnt already listed */
          if !crate::libbb::llist::llist_find_str((*defn).autointerfaces, first_word).is_null() {
            crate::libbb::perror_msg::bb_perror_msg_and_die(
              b"interface declared auto twice \"%s\"\x00" as *const u8 as *const libc::c_char,
              buf,
            );
          }
          /* Add the interface to the list */
          crate::libbb::llist::llist_add_to_end(
            &mut (*defn).autointerfaces,
            crate::libbb::xfuncs_printf::xstrdup(first_word) as *mut libc::c_void,
          ); /* "down" */
        } /* "up" */
        currently_processing = NONE
      } else if strcmp(
        first_word,
        b"source\x00" as *const u8 as *const libc::c_char,
      ) == 0
      {
        read_interfaces(next_word(&mut rest_of_line), defn);
      } else if !crate::libbb::compare_string_array::is_prefixed_with(
        first_word,
        b"source-dir\x00" as *const u8 as *const libc::c_char,
      )
      .is_null()
      {
        let mut dirpath: *const libc::c_char = std::ptr::null();
        let mut dir: *mut DIR = std::ptr::null_mut();
        let mut entry: *mut dirent = std::ptr::null_mut();
        dirpath = next_word(&mut rest_of_line);
        dir = crate::libbb::xfuncs_printf::xopendir(dirpath);
        loop {
          entry = readdir(dir);
          if entry.is_null() {
            break;
          }
          let mut path: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
          if (*entry).d_name[0] as libc::c_int == '.' as i32 {
            continue;
          }
          path =
            crate::libbb::concat_path_file::concat_path_file(dirpath, (*entry).d_name.as_mut_ptr());
          read_interfaces(path, defn);
          free(path as *mut libc::c_void);
        }
        closedir(dir);
      } else {
        match currently_processing as libc::c_uint {
          1 => {
            if *rest_of_line.offset(0) as libc::c_int == '\u{0}' as i32 {
              crate::libbb::verror_msg::bb_error_msg_and_die(
                b"option with empty value \"%s\"\x00" as *const u8 as *const libc::c_char,
                buf,
              );
            }
            if strcmp(
              first_word,
              b"post-up\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
              first_word = first_word.offset(5)
            } else if strcmp(
              first_word,
              b"pre-down\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
              first_word = first_word.offset(4)
            }
            /* If not one of "up", "down",... words... */
            if crate::libbb::compare_string_array::index_in_strings(
              keywords_up_down.as_ptr(),
              first_word,
            ) < 0
            {
              let mut i: libc::c_int = 0;
              i = 0;
              while i < (*currif).n_options {
                if strcmp((*(*currif).option.offset(i as isize)).name, first_word) == 0 {
                  crate::libbb::verror_msg::bb_error_msg_and_die(
                    b"duplicate option \"%s\"\x00" as *const u8 as *const libc::c_char,
                    buf,
                  );
                }
                i += 1
              }
            }
            (*currif).option = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
              (*currif).option as *mut libc::c_void,
              ((::std::mem::size_of::<variable_t>() as libc::c_ulong) << 8i32)
                .wrapping_add(4i32 as libc::c_ulong) as libc::c_uint,
              (*currif).n_options,
            ) as *mut variable_t;
            let ref mut fresh6 = (*(*currif).option.offset((*currif).n_options as isize)).name;
            *fresh6 = crate::libbb::xfuncs_printf::xstrdup(first_word);
            let ref mut fresh7 = (*(*currif).option.offset((*currif).n_options as isize)).value;
            *fresh7 = crate::libbb::xfuncs_printf::xstrdup(rest_of_line);
            (*currif).n_options += 1
          }
          2 => {
            if strcmp(
              first_word,
              b"script\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
              if !(*currmap).script.is_null() {
                crate::libbb::verror_msg::bb_error_msg_and_die(
                  b"duplicate script in mapping \"%s\"\x00" as *const u8 as *const libc::c_char,
                  buf,
                );
              }
              (*currmap).script = crate::libbb::xfuncs_printf::xstrdup(next_word(&mut rest_of_line))
            } else if strcmp(first_word, b"map\x00" as *const u8 as *const libc::c_char) == 0 {
              (*currmap).mapping = crate::libbb::xrealloc_vector::xrealloc_vector_helper(
                (*currmap).mapping as *mut libc::c_void,
                ((::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) << 8i32)
                  .wrapping_add(2i32 as libc::c_ulong) as libc::c_uint,
                (*currmap).n_mappings,
              ) as *mut *mut libc::c_char;
              let ref mut fresh8 = *(*currmap).mapping.offset((*currmap).n_mappings as isize);
              *fresh8 = crate::libbb::xfuncs_printf::xstrdup(next_word(&mut rest_of_line));
              (*currmap).n_mappings += 1
            } else {
              crate::libbb::verror_msg::bb_error_msg_and_die(
                b"misplaced option \"%s\"\x00" as *const u8 as *const libc::c_char,
                buf,
              );
            }
          }
          0 | _ => {
            crate::libbb::verror_msg::bb_error_msg_and_die(
              b"misplaced option \"%s\"\x00" as *const u8 as *const libc::c_char,
              buf,
            );
          }
        }
      }
      free(buf as *mut libc::c_void);
    }
  }
  if ferror_unlocked(f) != 0 {
    /* ferror does NOT set errno! */
    crate::libbb::verror_msg::bb_error_msg_and_die(
      b"%s: I/O error\x00" as *const u8 as *const libc::c_char,
      filename,
    );
  }
  fclose(f);
  return defn;
}
unsafe fn setlocalenv(
  mut format: *const libc::c_char,
  mut name: *const libc::c_char,
  mut value: *const libc::c_char,
) -> *mut libc::c_char {
  let mut result: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut dst: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut src: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut c: libc::c_char = 0;
  result = crate::libbb::xfuncs_printf::xasprintf(format, name, value);
  src = result;
  dst = src;
  loop {
    c = *src;
    if !(c as libc::c_int != '=' as i32 && c as libc::c_int != 0) {
      break;
    }
    if c as libc::c_int == '-' as i32 {
      c = '_' as i32 as libc::c_char
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
      c = (c as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char
    }
    if bb_ascii_isalnum(c as libc::c_uchar) != 0 || c as libc::c_int == '_' as i32 {
      let fresh9 = dst;
      dst = dst.offset(1);
      *fresh9 = c
    }
    src = src.offset(1)
  }
  crate::libbb::safe_strncpy::overlapping_strcpy(dst, src);
  return result;
}
unsafe fn set_environ(
  mut iface: *mut interface_defn_t,
  mut mode: *const libc::c_char,
  mut opt: *const libc::c_char,
) {
  let mut i: libc::c_int = 0;
  let mut pp: *mut *mut libc::c_char = std::ptr::null_mut();
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .my_environ
    .is_null()
  {
    pp = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).my_environ;
    while !(*pp).is_null() {
      free(*pp as *mut libc::c_void);
      pp = pp.offset(1)
    }
    free((*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).my_environ as *mut libc::c_void);
  }
  /* note: last element will stay NULL: */
  let ref mut fresh10 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).my_environ;
  *fresh10 = crate::libbb::xfuncs_printf::xzalloc(
    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      .wrapping_mul(((*iface).n_options + 7i32) as libc::c_ulong),
  ) as *mut *mut libc::c_char;
  pp = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).my_environ;
  i = 0;
  while i < (*iface).n_options {
    if !(crate::libbb::compare_string_array::index_in_strings(
      keywords_up_down.as_ptr(),
      (*(*iface).option.offset(i as isize)).name,
    ) >= 0)
    {
      let fresh11 = pp;
      pp = pp.offset(1);
      *fresh11 = setlocalenv(
        b"IF_%s=%s\x00" as *const u8 as *const libc::c_char,
        (*(*iface).option.offset(i as isize)).name,
        (*(*iface).option.offset(i as isize)).value,
      )
    }
    i += 1
  }
  let fresh12 = pp;
  pp = pp.offset(1);
  *fresh12 = setlocalenv(
    b"%s=%s\x00" as *const u8 as *const libc::c_char,
    b"IFACE\x00" as *const u8 as *const libc::c_char,
    (*iface).iface,
  );
  let fresh13 = pp;
  pp = pp.offset(1);
  *fresh13 = setlocalenv(
    b"%s=%s\x00" as *const u8 as *const libc::c_char,
    b"ADDRFAM\x00" as *const u8 as *const libc::c_char,
    (*(*iface).address_family).name,
  );
  let fresh14 = pp;
  pp = pp.offset(1);
  *fresh14 = setlocalenv(
    b"%s=%s\x00" as *const u8 as *const libc::c_char,
    b"METHOD\x00" as *const u8 as *const libc::c_char,
    (*(*iface).method).name,
  );
  let fresh15 = pp;
  pp = pp.offset(1);
  *fresh15 = setlocalenv(
    b"%s=%s\x00" as *const u8 as *const libc::c_char,
    b"MODE\x00" as *const u8 as *const libc::c_char,
    mode,
  );
  let fresh16 = pp;
  pp = pp.offset(1);
  *fresh16 = setlocalenv(
    b"%s=%s\x00" as *const u8 as *const libc::c_char,
    b"PHASE\x00" as *const u8 as *const libc::c_char,
    opt,
  );
  if !(*(bb_common_bufsiz1.as_mut_ptr() as *mut globals))
    .startup_PATH
    .is_null()
  {
    let fresh17 = pp;
    pp = pp.offset(1);
    *fresh17 = setlocalenv(
      b"%s=%s\x00" as *const u8 as *const libc::c_char,
      b"PATH\x00" as *const u8 as *const libc::c_char,
      (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).startup_PATH,
    )
  };
}
unsafe fn doit(mut str: *mut libc::c_char) -> libc::c_int {
  if option_mask32 & (OPT_no_act as libc::c_int | OPT_verbose as libc::c_int) as libc::c_uint != 0 {
    puts(str);
  }
  if option_mask32 & OPT_no_act as libc::c_int as libc::c_uint == 0 {
    let mut child: pid_t = 0;
    let mut status: libc::c_int = 0;
    crate::libbb::xfuncs_printf::fflush_all();
    child = vfork();
    if child < 0 {
      /* failure */
      return 0;
    }
    if child == 0 {
      /* child */
      execle(
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shell,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shell,
        b"-c\x00" as *const u8 as *const libc::c_char,
        str,
        0 as *mut libc::c_void as *mut libc::c_char,
        (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).my_environ,
      );
      _exit(127i32);
    }
    crate::libbb::xfuncs::safe_waitpid(child, &mut status, 0);
    if !(status & 0x7fi32 == 0) || (status & 0xff00i32) >> 8i32 != 0 {
      return 0;
    }
  }
  return 1i32;
}
unsafe fn execute_all(mut ifd: *mut interface_defn_t, mut opt: *const libc::c_char) -> libc::c_int {
  /* 'opt' is always short, the longest value is "post-down".
   * Can use on-stack buffer instead of xasprintf'ed one.
   */
  let mut buf: [libc::c_char; 49] = [0; 49];
  let mut i: libc::c_int = 0;
  i = 0;
  while i < (*ifd).n_options {
    if strcmp((*(*ifd).option.offset(i as isize)).name, opt) == 0 {
      if doit((*(*ifd).option.offset(i as isize)).value) == 0 {
        return 0;
      }
    }
    i += 1
  }
  /*paranoia:*/
  /* Tested on Debian Squeeze: "standard" ifup runs this without
   * checking that directory exists. If it doesn't, run-parts
   * complains, and this message _is_ annoyingly visible.
   * Don't "fix" this (unless newer Debian does).
   */
  sprintf(
    buf.as_mut_ptr(),
    b"run-parts /etc/network/if-%s.d\x00" as *const u8 as *const libc::c_char,
    opt,
  );
  return doit(buf.as_mut_ptr());
}
unsafe fn check(mut str: *mut libc::c_char) -> libc::c_int {
  return (str != std::ptr::null_mut()) as libc::c_int;
}
unsafe fn iface_up(mut iface: *mut interface_defn_t) -> libc::c_int {
  if (*(*iface).method).up.expect("non-null function pointer")(
    iface,
    Some(check as unsafe fn(_: *mut libc::c_char) -> libc::c_int),
  ) == 0
  {
    return -1i32;
  }
  set_environ(
    iface,
    b"start\x00" as *const u8 as *const libc::c_char,
    b"pre-up\x00" as *const u8 as *const libc::c_char,
  );
  if execute_all(iface, b"pre-up\x00" as *const u8 as *const libc::c_char) == 0 {
    return 0;
  }
  if (*(*iface).method).up.expect("non-null function pointer")(
    iface,
    Some(doit as unsafe fn(_: *mut libc::c_char) -> libc::c_int),
  ) == 0
  {
    return 0;
  }
  set_environ(
    iface,
    b"start\x00" as *const u8 as *const libc::c_char,
    b"post-up\x00" as *const u8 as *const libc::c_char,
  );
  if execute_all(iface, b"up\x00" as *const u8 as *const libc::c_char) == 0 {
    return 0;
  }
  return 1i32;
}
unsafe fn iface_down(mut iface: *mut interface_defn_t) -> libc::c_int {
  if (*(*iface).method).down.expect("non-null function pointer")(
    iface,
    Some(check as unsafe fn(_: *mut libc::c_char) -> libc::c_int),
  ) == 0
  {
    return -1i32;
  }
  set_environ(
    iface,
    b"stop\x00" as *const u8 as *const libc::c_char,
    b"pre-down\x00" as *const u8 as *const libc::c_char,
  );
  if execute_all(iface, b"down\x00" as *const u8 as *const libc::c_char) == 0 {
    return 0;
  }
  if (*(*iface).method).down.expect("non-null function pointer")(
    iface,
    Some(doit as unsafe fn(_: *mut libc::c_char) -> libc::c_int),
  ) == 0
  {
    return 0;
  }
  set_environ(
    iface,
    b"stop\x00" as *const u8 as *const libc::c_char,
    b"post-down\x00" as *const u8 as *const libc::c_char,
  );
  if execute_all(iface, b"post-down\x00" as *const u8 as *const libc::c_char) == 0 {
    return 0;
  }
  return 1i32;
}
unsafe fn popen2(
  mut in_0: *mut *mut FILE,
  mut out: *mut *mut FILE,
  mut command: *mut libc::c_char,
  mut param: *mut libc::c_char,
) -> libc::c_int {
  let mut argv: [*mut libc::c_char; 3] = [command, param, std::ptr::null_mut::<libc::c_char>()];
  let mut infd: fd_pair = fd_pair { rd: 0, wr: 0 };
  let mut outfd: fd_pair = fd_pair { rd: 0, wr: 0 };
  let mut pid: pid_t = 0;
  crate::libbb::xfuncs_printf::xpipe(&mut infd.rd);
  crate::libbb::xfuncs_printf::xpipe(&mut outfd.rd);
  crate::libbb::xfuncs_printf::fflush_all();
  pid = {
    let mut bb__xvfork_pid: pid_t = vfork();
    if bb__xvfork_pid < 0 {
      crate::libbb::perror_msg::bb_simple_perror_msg_and_die(
        b"vfork\x00" as *const u8 as *const libc::c_char,
      );
    }
    bb__xvfork_pid
  };
  if pid == 0 {
    /* Child */
    /* NB: close _first_, then move fds! */
    close(infd.wr);
    close(outfd.rd);
    crate::libbb::xfuncs_printf::xmove_fd(infd.rd, 0);
    crate::libbb::xfuncs_printf::xmove_fd(outfd.wr, 1i32);
    crate::libbb::executable::BB_EXECVP_or_die(argv.as_mut_ptr());
  }
  /* parent */
  close(infd.rd);
  close(outfd.wr);
  *in_0 = crate::libbb::wfopen::xfdopen_for_write(infd.wr);
  *out = crate::libbb::wfopen::xfdopen_for_read(outfd.rd);
  return pid;
}
unsafe fn run_mapping(
  mut physical: *mut libc::c_char,
  mut map: *mut mapping_defn_t,
) -> *mut libc::c_char {
  let mut in_0: *mut FILE = std::ptr::null_mut();
  let mut out: *mut FILE = std::ptr::null_mut();
  let mut i: libc::c_int = 0;
  let mut status: libc::c_int = 0;
  let mut pid: pid_t = 0;
  let mut logical: *mut libc::c_char = crate::libbb::xfuncs_printf::xstrdup(physical);
  /* Run the mapping script. Never fails. */
  pid = popen2(&mut in_0, &mut out, (*map).script, physical);
  /* Write mappings to stdin of mapping script. */
  i = 0;
  while i < (*map).n_mappings {
    fprintf(
      in_0,
      b"%s\n\x00" as *const u8 as *const libc::c_char,
      *(*map).mapping.offset(i as isize),
    );
    i += 1
  }
  fclose(in_0);
  crate::libbb::xfuncs::safe_waitpid(pid, &mut status, 0);
  if status & 0x7fi32 == 0 && (status & 0xff00i32) >> 8i32 == 0 {
    /* If the mapping script exited successfully, try to
     * grab a line of output and use that as the name of the
     * logical interface. */
    let mut new_logical: *mut libc::c_char =
      crate::libbb::get_line_from_file::xmalloc_fgetline(out);
    if !new_logical.is_null() {
      /* If we are able to read a line of output from the script,
       * remove any trailing whitespace and use this value
       * as the name of the logical interface. */
      let mut pch: *mut libc::c_char = new_logical.offset(strlen(new_logical) as isize).offset(-1);
      while pch >= new_logical
        && ({
          let mut bb__isspace: libc::c_uchar = (*pch as libc::c_int - 9i32) as libc::c_uchar;
          (bb__isspace as libc::c_int == ' ' as i32 - 9i32
            || bb__isspace as libc::c_int <= 13i32 - 9i32) as libc::c_int
        }) != 0
      {
        let fresh18 = pch;
        pch = pch.offset(-1);
        *fresh18 = '\u{0}' as i32 as libc::c_char
      }
      free(logical as *mut libc::c_void);
      logical = new_logical
    }
  }
  fclose(out);
  return logical;
}
/* FEATURE_IFUPDOWN_MAPPING */
unsafe fn find_iface_state(
  mut state_list: *mut llist_t,
  mut iface: *const libc::c_char,
) -> *mut llist_t {
  let mut search: *mut llist_t = state_list;
  while !search.is_null() {
    let mut after_iface: *mut libc::c_char =
      crate::libbb::compare_string_array::is_prefixed_with((*search).data, iface);
    if !after_iface.is_null() && *after_iface as libc::c_int == '=' as i32 {
      return search;
    }
    search = (*search).link
  }
  return std::ptr::null_mut();
}
/* read the previous state from the state file */
unsafe fn read_iface_state() -> *mut llist_t {
  let mut state_list: *mut llist_t = std::ptr::null_mut();
  let mut state_fp: *mut FILE = crate::libbb::wfopen::fopen_for_read(
    b"/var/run/ifstate\x00" as *const u8 as *const libc::c_char,
  );
  if !state_fp.is_null() {
    let mut start: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut end_ptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    loop {
      start = crate::libbb::get_line_from_file::xmalloc_fgets(state_fp);
      if start.is_null() {
        break;
      }
      /* We should only need to check for a single character */
      end_ptr =
        start.offset(strcspn(start, b" \t\n\x00" as *const u8 as *const libc::c_char) as isize);
      *end_ptr = '\u{0}' as i32 as libc::c_char;
      crate::libbb::llist::llist_add_to(&mut state_list, start as *mut libc::c_void);
    }
    fclose(state_fp);
  }
  return state_list;
}
/* read the previous state from the state file */
unsafe fn open_new_state_file() -> *mut FILE {
  let mut fd: libc::c_int = 0;
  let mut flags: libc::c_int = 0;
  let mut cnt: libc::c_int = 0;
  cnt = 0;
  flags = 0o1i32 | 0o100i32 | 0o200i32;
  loop {
    fd = open(
      b"/var/run/ifstate.new\x00" as *const u8 as *const libc::c_char,
      flags,
      0o666i32,
    );
    if fd >= 0 {
      break;
    }
    if *bb_errno != 17i32 || flags == 0o1i32 | 0o100i32 | 0o1000i32 {
      crate::libbb::perror_msg::bb_perror_msg_and_die(
        b"can\'t open \'%s\'\x00" as *const u8 as *const libc::c_char,
        b"/var/run/ifstate.new\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* Someone else created the .new file */
    if cnt > 30i32 * 1000i32 {
      /* Waited for 30*30/2 = 450 milliseconds, still EEXIST.
       * Assuming a stale file, rewriting it.
       */
      flags = 0o1i32 | 0o100i32 | 0o1000i32
    } else {
      usleep(cnt as useconds_t);
      cnt += 1000i32
    }
  }
  return crate::libbb::wfopen::xfdopen_for_write(fd);
}
pub unsafe fn ifupdown_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut current_block: u64;
  let mut cmds: Option<unsafe fn(_: *mut interface_defn_t) -> libc::c_int> = None;
  let mut defn: *mut interfaces_file_t = std::ptr::null_mut();
  let mut target_list: *mut llist_t = std::ptr::null_mut();
  let mut interfaces: *const libc::c_char =
    b"/etc/network/interfaces\x00" as *const u8 as *const libc::c_char;
  let mut any_failures: bool = 0 != 0;
  let ref mut fresh19 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).startup_PATH;
  *fresh19 = getenv(b"PATH\x00" as *const u8 as *const libc::c_char);
  let ref mut fresh20 = (*(bb_common_bufsiz1.as_mut_ptr() as *mut globals)).shell;
  *fresh20 = crate::libbb::xfuncs_printf::xstrdup(crate::libbb::get_shell_name::get_shell_name());
  if 1i32 != 0 && (1i32 == 0 || *applet_name.offset(2) as libc::c_int == 'u' as i32) {
    /* ifup command */
    cmds = Some(iface_up as unsafe fn(_: *mut interface_defn_t) -> libc::c_int)
  } else {
    cmds = Some(iface_down as unsafe fn(_: *mut interface_defn_t) -> libc::c_int)
  }
  crate::libbb::getopt32::getopt32(
    argv,
    b"anvfmi:\x00" as *const u8 as *const libc::c_char,
    &mut interfaces as *mut *const libc::c_char,
  );
  argv = argv.offset(optind as isize);
  if !(*argv.offset(0)).is_null() {
    if option_mask32 & OPT_do_all as libc::c_int as libc::c_uint != 0 {
      crate::libbb::appletlib::bb_show_usage();
    }
  } else if option_mask32 & OPT_do_all as libc::c_int as libc::c_uint == 0 {
    crate::libbb::appletlib::bb_show_usage();
  }
  defn = read_interfaces(interfaces, 0 as *mut interfaces_file_t);
  /* Create a list of interfaces to work on */
  if option_mask32 & OPT_do_all as libc::c_int as libc::c_uint != 0 {
    target_list = (*defn).autointerfaces
  } else {
    crate::libbb::llist::llist_add_to_end(&mut target_list, *argv.offset(0) as *mut libc::c_void);
  }
  /* Update the interfaces */
  while !target_list.is_null() {
    let mut iface_list: *mut llist_t = std::ptr::null_mut();
    let mut currif: *mut interface_defn_t = std::ptr::null_mut();
    let mut iface: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut liface: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut pch: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut okay: bool = 0 != 0;
    let mut cmds_ret: libc::c_int = 0;
    let mut curr_failure: bool = 0 != 0;
    iface = crate::libbb::xfuncs_printf::xstrdup((*target_list).data);
    target_list = (*target_list).link;
    pch = strchr(iface, '=' as i32);
    if !pch.is_null() {
      *pch = '\u{0}' as i32 as libc::c_char;
      liface = crate::libbb::xfuncs_printf::xstrdup(pch.offset(1))
    } else {
      liface = crate::libbb::xfuncs_printf::xstrdup(iface)
    }
    if option_mask32 & OPT_force as libc::c_int as libc::c_uint == 0 {
      let mut state_list: *mut llist_t = read_iface_state();
      let mut iface_state: *const llist_t = find_iface_state(state_list, iface);
      if cmds == Some(iface_up as unsafe fn(_: *mut interface_defn_t) -> libc::c_int) {
        /* ifup */
        if !iface_state.is_null() {
          crate::libbb::verror_msg::bb_error_msg(
            b"interface %s already configured\x00" as *const u8 as *const libc::c_char,
            iface,
          );
          current_block = 12709013627096618709;
        } else {
          current_block = 2989495919056355252;
        }
      } else if iface_state.is_null() {
        crate::libbb::verror_msg::bb_error_msg(
          b"interface %s not configured\x00" as *const u8 as *const libc::c_char,
          iface,
        );
        current_block = 12709013627096618709;
      } else {
        current_block = 2989495919056355252;
      }
      match current_block {
        12709013627096618709 => {}
        _ => {
          crate::libbb::llist::llist_free(state_list);
          current_block = 13678349939556791712;
        }
      }
    } else {
      current_block = 13678349939556791712;
    }
    match current_block {
      13678349939556791712 => {
        if cmds == Some(iface_up as unsafe fn(_: *mut interface_defn_t) -> libc::c_int)
          && option_mask32 & OPT_no_mappings as libc::c_int as libc::c_uint == 0
        {
          let mut currmap: *mut mapping_defn_t = std::ptr::null_mut();
          currmap = (*defn).mappings;
          while !currmap.is_null() {
            let mut i: libc::c_int = 0;
            i = 0;
            while i < (*currmap).n_matches {
              if fnmatch(*(*currmap).match_0.offset(i as isize), liface, 0) != 0 {
                i += 1
              } else {
                if option_mask32 & OPT_verbose as libc::c_int as libc::c_uint != 0 {
                  printf(
                    b"Running mapping script %s on %s\n\x00" as *const u8 as *const libc::c_char,
                    (*currmap).script,
                    liface,
                  );
                }
                liface = run_mapping(iface, currmap);
                break;
              }
            }
            currmap = (*currmap).next
          }
        }
        iface_list = (*defn).ifaces;
        while !iface_list.is_null() {
          currif = (*iface_list).data as *mut interface_defn_t;
          if strcmp(liface, (*currif).iface) == 0 {
            let mut oldiface: *mut libc::c_char = (*currif).iface;
            okay = 1i32 != 0;
            (*currif).iface = iface;
            /* ifdown */
            /* Call the cmds function pointer, does either iface_up() or iface_down() */
            cmds_ret = cmds.expect("non-null function pointer")(currif);
            if cmds_ret == -1i32 {
              crate::libbb::verror_msg::bb_error_msg(
                b"don\'t have all variables for %s/%s\x00" as *const u8 as *const libc::c_char,
                liface,
                (*(*currif).address_family).name,
              );
              curr_failure = 1i32 != 0;
              any_failures = curr_failure
            } else if cmds_ret == 0 {
              curr_failure = 1i32 != 0;
              any_failures = curr_failure
            }
            (*currif).iface = oldiface
          }
          iface_list = (*iface_list).link
        }
        if option_mask32 & OPT_verbose as libc::c_int as libc::c_uint != 0 {
          crate::libbb::xfuncs_printf::bb_putchar('\n' as i32);
        }
        if !okay && option_mask32 & OPT_force as libc::c_int as libc::c_uint == 0 {
          crate::libbb::verror_msg::bb_error_msg(
            b"ignoring unknown interface %s\x00" as *const u8 as *const libc::c_char,
            liface,
          );
          any_failures = 1i32 != 0
        } else if option_mask32 & OPT_no_act as libc::c_int as libc::c_uint == 0 {
          /* update the state file */
          let mut new_state_fp: *mut FILE = open_new_state_file();
          let mut state: *mut llist_t = std::ptr::null_mut();
          let mut state_list_0: *mut llist_t = read_iface_state();
          let mut iface_state_0: *mut llist_t = find_iface_state(state_list_0, iface);
          if cmds == Some(iface_up as unsafe fn(_: *mut interface_defn_t) -> libc::c_int)
            && !curr_failure
          {
            let mut newiface: *mut libc::c_char = crate::libbb::xfuncs_printf::xasprintf(
              b"%s=%s\x00" as *const u8 as *const libc::c_char,
              iface,
              liface,
            );
            if iface_state_0.is_null() {
              crate::libbb::llist::llist_add_to_end(
                &mut state_list_0,
                newiface as *mut libc::c_void,
              );
            } else {
              free((*iface_state_0).data as *mut libc::c_void);
              (*iface_state_0).data = newiface
            }
          } else {
            /* Remove an interface from state_list */
            crate::libbb::llist::llist_unlink(&mut state_list_0, iface_state_0);
            free(crate::libbb::llist::llist_pop(&mut iface_state_0));
          }
          /* Actually write the new state */
          state = state_list_0;
          while !state.is_null() {
            if !(*state).data.is_null() {
              fprintf(
                new_state_fp,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                (*state).data,
              );
            }
            state = (*state).link
          }
          fclose(new_state_fp);
          crate::libbb::xfuncs_printf::xrename(
            b"/var/run/ifstate.new\x00" as *const u8 as *const libc::c_char,
            b"/var/run/ifstate\x00" as *const u8 as *const libc::c_char,
          );
          crate::libbb::llist::llist_free(state_list_0);
        }
      }
      _ => {}
    }
    free(iface as *mut libc::c_void);
    free(liface as *mut libc::c_void);
  }
  return any_failures as libc::c_int;
}
unsafe fn run_static_initializers() {
  addr_inet6 = {
    let mut init = address_family_t {
      name: b"inet6\x00" as *const u8 as *const libc::c_char,
      n_methods: (::std::mem::size_of::<[method_t; 4]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<method_t>() as libc::c_ulong)
        as libc::c_uint as libc::c_int,
      method: methods6.as_ptr(),
    };
    init
  };
  addr_inet = {
    let mut init = address_family_t {
      name: b"inet\x00" as *const u8 as *const libc::c_char,
      n_methods: (::std::mem::size_of::<[method_t; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<method_t>() as libc::c_ulong)
        as libc::c_uint as libc::c_int,
      method: methods.as_ptr(),
    };
    init
  };
  addr_link = {
    let mut init = address_family_t {
      name: b"link\x00" as *const u8 as *const libc::c_char,
      n_methods: (::std::mem::size_of::<[method_t; 1]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<method_t>() as libc::c_ulong)
        as libc::c_uint as libc::c_int,
      method: link_methods.as_ptr(),
    };
    init
  }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe fn(); 1] = [run_static_initializers];
