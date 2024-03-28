cfg_if! {
    if #[cfg(not(target_arch = "x86_64"))] {
        compile_error!("Aero is not ported to this architecture");
    }
}

// Basic data types
pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = ::c_int;

// options/posix/include/sys/resource.h
pub type rlim_t = ::c_ulong;

// abis/mlibc/mode_t.h
pub type mode_t = ::c_int;

// options/posix/include/bits/posix/socklen_t.h
pub type socklen_t = ::c_ulong;

// options/internal/include/bits/off_t.h
pub type off_t = ::c_long;

// options/ansi/include/time.h
pub type clock_t = ::c_ulong;

// options/ansi/include/bits/ansi/clockid_t.h
pub type clockid_t = ::c_long;

// options/ansi/include/bits/ansi/time_t.h
pub type time_t = ::c_long;

// options/posix/include/bits/posix/suseconds_t.h
pub type suseconds_t = ::c_long;

// abis/mlibc/dev_t.h
pub type dev_t = ::c_ulong;

// options/posix/include/bits/posix/fsblkcnt_t.h
pub type fsblkcnt_t = ::c_uint;

// options/posix/include/bits/posix/fsfilcnt_t.h
pub type fsfilcnt_t = ::c_uint;

// abis/mlibc/signal.h
pub type sigset_t = ::c_long;

// abis/mlibc/termios.h
pub type speed_t = ::c_uint;
pub type tcflag_t = ::c_uint;

// abis/mlibc/ino_t.h
pub type ino_t = ::c_long;

// abis/mlibc/blksize_t.h
pub type blksize_t = ::c_long;

// abis/mlibc/blkcnt_t.h
pub type blkcnt_t = ::c_long;

// abis/mlibc/nlink_t.h
pub type nlink_t = ::c_int;

// options/posix/include/bits/posix/in_addr_t.h
pub type in_addr_t = u32;

// options/posix/include/bits/posix/in_port_t.h
pub type in_port_t = u16;

// abis/mlibc/socket.h
pub type sa_family_t = ::c_int;

// options/linux/include/sys/poll.h
pub type nfds_t = ::size_t;

// options/posix/include/bits/posix/pthread_t.h
pub type pthread_t = *mut __mlibc_thread_data;

// options/posix/include/bits/posix/pthread.h
pub type pthread_key_t = usize; // TODO: This is a big hack

s! {
    // options/ansi/include/time.h
    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
        pub tm_gmtoff: ::c_long,
        pub tm_zone: *const ::c_char,
    }

    // abis/mlibc/signal.h
    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_errno: ::c_int,
        pub si_pid: ::pid_t,
        pub si_uid: ::uid_t,
        pub si_addr: *mut ::c_void,
        pub si_status: ::c_int,
        pub si_value: sigval,
    }
    pub struct sigaction {
        pub sa_handler: ::Option<extern fn(::c_int)>,
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
        pub sa_sigaction: ::sighandler_t,
    }

    // abis/mlibc/termios.h
    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; NCCS],
        pub ibaud: speed_t,
        pub obaud: speed_t,
    }

    // options/posix/include/sys/statvfs.h
    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    // options/posix/include/dirent.h
    pub struct dirent {
        pub d_ino: ino_t,
        pub d_off: off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 1024],
    }

    // options/ansi/include/bits/ansi/timespec.h
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: ::c_long,
    }

    // options/posix/include/sys/un.h
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108],
    }

    // abis/mlibc/in.h
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8], // std relies on this being public
    }
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    // options/posix/include/dlfcn.h
    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    // abis/mlibc/socket.h
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __padding: [u8; 128 - ::mem::size_of::<sa_family_t>()],
    }

    // abis/mlibc/stat.h
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
    }

    // options/posix/include/pwd.h
    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    // options/posix/include/sys/socket.h
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    // options/posix/include/bits/posix/pthread_t.h
    pub struct __mlibc_thread_data {}

    // options/posix/include/sched.h
    pub struct sched_param {
        pub sched_priority: ::c_int,
    }

    pub struct __mlibc_cpu_set {
        pub __bits: [::c_ulong; 128 / core::mem::size_of::<::c_long>()],
    }

    // options/posix/include/bits/posix/pthread.h
    pub struct pthread_attr_t {
        pub __mlibc_guardsize: ::size_t,
        pub __mlibc_stacksize: ::size_t,
        pub __mlibc_stackaddr: *mut ::c_void,
        pub __mlibc_detachstate: ::c_int,
        pub __mlibc_scope: ::c_int,
        pub __mlibc_inheritsched: ::c_int,
        pub __mlibc_schedparam: sched_param,
        pub __mlibc_schedpolicy: ::c_int,
        pub __mlibc_cpuset: *mut __mlibc_cpu_set,
        pub __mlibc_cpusetsize: ::size_t,
        pub __mlibc_sigmask: sigset_t,
        pub __mlibc_sigmaskset: ::c_int,
    }
    pub struct pthread_cond_t {
        pub __mlibc_seq: ::c_uint,
        pub __mlibc_flags: ::c_uint,
        pub __mlibc_clock: clockid_t,
    }
    pub struct pthread_condattr_t {
        pub __mlibc_pshared: ::c_int,
        pub __mlibc_clock: clockid_t,
    }
    pub struct pthread_mutex_t {
        pub __mlibc_state: ::c_uint,
        pub __mlibc_recursion: ::c_uint,
        pub __mlibc_flags: ::c_uint,
        pub __mlibc_prioceiling: ::c_int,
    }
    pub struct pthread_mutexattr_t {
        pub __mlibc_type: ::c_int,
        pub __mlibc_robust: ::c_int,
        pub __mlibc_protocol: ::c_int,
        pub __mlibc_pshared: ::c_int,
        pub __mlibc_prioceiling: ::c_int,
    }
    pub struct pthread_rwlock_t {
        pub __mlibc_m: ::c_uint,
        pub __mlibc_rc: ::c_uint,
        __mlibc_padding: ::c_uint,
    }
    pub struct pthread_rwlockattr_t {
        pub __mlibc_align: ::c_int,
    }

    // options/posix/include/netdb.h
    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut ::c_char,
        pub ai_next: *mut addrinfo,
    }

    // options/ansi/include/locale.h
    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }

    // options/posix/include/semaphore.h
    pub struct sem_t {
        pub __mlibc_count: ::c_uint,
    }

    // abis/linux/signal.h
    pub struct sigval {
        // Actually a union of an int and a void*
        pub sival_ptr: *mut ::c_void,
    }

    // options/posix/include/bits/posix/fd_set.h
    pub struct fd_set {
        pub fds_bits: [c_char; 128],
    }

    pub struct cmsghdr {
        // FIXME: mio needs this to be a size_t for some reason
        // pub cmsg_len: ::socklen_t,
        pub cmsg_len: ::size_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::size_t, // nix assumes this is a size_t
        pub msg_flags: ::c_int,
    }

    // options/linux-headers/include/linux/if_packet.h
    pub struct sockaddr_ll {
        pub sll_family: ::c_ushort,
        pub sll_protocol: ::c_ushort,
        pub sll_ifindex: ::c_int,
        pub sll_hatype: ::c_ushort,
        pub sll_pkttype: ::c_uchar,
        pub sll_halen: ::c_uchar,
        pub sll_addr: [::c_uchar; 8]
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }

    pub struct __c_anonymous_ifru_map {
        pub mem_start: ::c_ulong,
        pub mem_end: ::c_ulong,
        pub base_addr: ::c_ushort,
        pub irq: ::c_uchar,
        pub dma: ::c_uchar,
        pub port: ::c_uchar,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl ::fmt::Debug for epoll_event {
            fn fmt(&self, f: &mut ::fmt::Formatter<'_>) -> ::fmt::Result {
                let events = self.events;
                let u64 = self.u64;
                f.debug_struct("epoll_event")
                    .field("events", &events)
                    .field("u64", &u64)
                    .finish()
            }
        }
        impl PartialEq for epoll_event {
            fn eq(&self, other: &epoll_event) -> bool {
                self.events == other.events
                    && self.u64 == other.u64
            }
        }
        impl Eq for epoll_event {}
        impl ::hash::Hash for epoll_event {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                let events = self.events;
                let u64 = self.u64;
                events.hash(state);
                u64.hash(state);
            }
        }

        #[cfg(libc_union)]
        impl ::fmt::Debug for __c_anonymous_ifr_ifru {
            fn fmt(&self, f: &mut ::fmt::Formatter<'_>) -> ::fmt::Result {
                f.debug_struct("ifr_ifru")
                    .field("ifru_addr", unsafe { &self.ifru_addr })
                    .field("ifru_dstaddr", unsafe { &self.ifru_dstaddr })
                    .field("ifru_broadaddr", unsafe { &self.ifru_broadaddr })
                    .field("ifru_netmask", unsafe { &self.ifru_netmask })
                    .field("ifru_hwaddr", unsafe { &self.ifru_hwaddr })
                    .field("ifru_flags", unsafe { &self.ifru_flags })
                    .field("ifru_ifindex", unsafe { &self.ifru_ifindex })
                    .field("ifru_metric", unsafe { &self.ifru_metric })
                    .field("ifru_mtu", unsafe { &self.ifru_mtu })
                    .field("ifru_map", unsafe { &self.ifru_map })
                    .field("ifru_slave", unsafe { &self.ifru_slave })
                    .field("ifru_newname", unsafe { &self.ifru_newname })
                    .field("ifru_data", unsafe { &self.ifru_data })
                    .finish()
            }
        }

        impl ::fmt::Debug for ifreq {
            fn fmt(&self, f: &mut ::fmt::Formatter<'_>) -> ::fmt::Result {
                f.debug_struct("ifreq")
                    .field("ifr_name", &self.ifr_name)
                    .field("ifr_ifru", &self.ifr_ifru)
                    .finish()
            }
        }
    }
}

s_no_extra_traits! {
    // options/linux/include/sys/epoll.h
    #[cfg_attr(target_arch = "x86_64", repr(packed))]
    pub struct epoll_event {
        pub events: u32,
        pub u64: u64,
    }

    // options/posix/include/net/if.h
    #[cfg(libc_union)]
    pub union __c_anonymous_ifr_ifru {
        pub ifru_addr: ::sockaddr,
        pub ifru_dstaddr: ::sockaddr,
        pub ifru_broadaddr: ::sockaddr,
        pub ifru_netmask: ::sockaddr,
        pub ifru_hwaddr: ::sockaddr,
        pub ifru_flags: ::c_short,
        pub ifru_ifindex: ::c_int,
        pub ifru_metric: ::c_int,
        pub ifru_mtu: ::c_int,
        pub ifru_map: __c_anonymous_ifru_map,
        pub ifru_slave: [::c_char; ::IFNAMSIZ],
        pub ifru_newname: [::c_char; ::IFNAMSIZ],
        pub ifru_data: *mut ::c_char,
    }

    pub struct ifreq {
        /// if name, e.g. "en0"
        pub ifr_name: [::c_char; ::IFNAMSIZ],
        #[cfg(libc_union)]
        pub ifr_ifru: __c_anonymous_ifr_ifru,
        #[cfg(not(libc_union))]
        pub ifr_ifru: ::sockaddr,
    }
}

// options/posix/include/sys/wait.h
f! {
    pub fn CMSG_FIRSTHDR(_mhdr: *const msghdr) -> *mut cmsghdr {
        core::unimplemented!()
    }

    pub fn CMSG_NXTHDR(_mhdr: *const ::msghdr, _cmsg: *const ::cmsghdr) -> *mut ::cmsghdr {
        core::unimplemented!()
    }

    pub {const} fn CMSG_LEN(_length: ::c_uint) -> ::c_uint {
        core::unimplemented!()
    }

    pub fn CMSG_DATA(_cmsg: *const cmsghdr) -> *mut ::c_uchar {
        core::unimplemented!()
    }

    pub {const} fn CMSG_SPACE(_len: ::c_uint) -> ::c_uint {
        core::unimplemented!()
    }
}

safe_f! {
    pub {const} fn WCOREDUMP(x: ::c_int) -> bool {
        x & WCOREFLAG != 0
    }
    pub {const} fn WEXITSTATUS(x: ::c_int) -> ::c_int {
        x & 0xFF
    }
    pub {const} fn WIFCONTINUED(x: ::c_int) -> bool {
        x & 0x100 != 0
    }
    pub {const} fn WIFEXITED(x: ::c_int) -> bool {
        x & 0x200 != 0
    }
    pub {const} fn WIFSIGNALED(x: ::c_int) -> bool {
        x & 0x400 != 0
    }
    pub {const} fn WIFSTOPPED(x: ::c_int) -> bool {
        x & 0x800 != 0
    }
    pub {const} fn WSTOPSIG(x: ::c_int) -> ::c_int {
        (x & 0xFF_0000) >> 16
    }
    pub {const} fn WTERMSIG(x: ::c_int) -> ::c_int {
        (x & 0xFF00_0000) >> 24
    }
}

// abis/linux/termios.h - indices for the `c_cc` array in struct termios
pub const NCCS: ::size_t = 32;
pub const VINTR: ::c_int = 0;
pub const VQUIT: ::c_int = 1;
pub const VERASE: ::c_int = 2;
pub const VKILL: ::c_int = 3;
pub const VEOF: ::size_t = 4;
pub const VTIME: ::c_int = 5;
pub const VMIN: ::c_int = 6;
pub const VSWTC: ::c_int = 7;
pub const VSTART: ::c_int = 8;
pub const VSTOP: ::c_int = 9;
pub const VSUSP: ::c_int = 10;
pub const VEOL: ::c_int = 11;
pub const VREPRINT: ::c_int = 12;
pub const VDISCARD: ::c_int = 13;
pub const VWERASE: ::c_int = 14;
pub const VLNEXT: ::c_int = 15;
pub const VEOL2: ::c_int = 16;

// abis/linux/termios.h - bitwise flags for c_iflag in struct termios
pub const BRKINT: ::c_uint = 0o000002;
pub const ICRNL: ::c_uint = 0o000400;
pub const IGNBRK: ::c_uint = 0o000001;
pub const IGNCR: ::c_uint = 0o000200;
pub const IGNPAR: ::c_uint = 0o000004;
pub const INLCR: ::c_uint = 0o000100;
pub const INPCK: ::c_uint = 0o000020;
pub const ISTRIP: ::c_uint = 0o000040;
pub const IXANY: ::c_uint = 0o004000;
pub const IXOFF: ::c_uint = 0o010000;
pub const IXON: ::c_uint = 0o002000;
pub const PARMRK: ::c_uint = 0o000010;

// abis/linux/termios.h - bitwise flags for c_oflag in struct termios
pub const OPOST: ::c_uint = 0o000001;
pub const ONLCR: ::c_int = 0o000004;
pub const OCRNL: ::c_int = 0o000010;
pub const ONOCR: ::c_int = 0o000020;
pub const ONLRET: ::c_int = 0o000040;
pub const OFDEL: ::c_int = 0o000200;
pub const OFILL: ::c_int = 0o000100;

pub const NLDLY: ::c_int = 0o000400;
pub const NL0: ::c_int = 0o000000;
pub const NL1: ::c_int = 0o000400;

pub const CRDLY: ::c_int = 0o003000;
pub const CR0: ::c_int = 0o000000;
pub const CR1: ::c_int = 0o001000;
pub const CR2: ::c_int = 0o002000;
pub const CR3: ::c_int = 0o003000;

pub const TABDLY: ::c_int = 0o014000;
pub const TAB0: ::c_int = 0o000000;
pub const TAB1: ::c_int = 0o004000;
pub const TAB2: ::c_int = 0o010000;
pub const TAB3: ::c_int = 0o014000;

pub const XTABS: ::c_int = 0o014000;
pub const BSDLY: ::c_int = 0o020000;
pub const BS0: ::c_int = 0o000000;
pub const BS1: ::c_int = 0o020000;

pub const VTDLY: ::c_int = 0o040000;
pub const VT0: ::c_int = 0o000000;
pub const VT1: ::c_int = 0o040000;

pub const FFDLY: ::c_int = 0o100000;
pub const FF0: ::c_int = 0o000000;
pub const FF1: ::c_int = 0o100000;

// abis/linux/termios.h - bitwise constants for c_cflag in struct termios
pub const CSIZE: ::c_int = 0o000060;
pub const CS5: ::c_uint = 0o000000;
pub const CS6: ::c_int = 0o000020;
pub const CS7: ::c_int = 0o000040;
pub const CS8: ::c_int = 0o000060;

pub const CSTOPB: ::c_int = 0o000100;
pub const CREAD: ::c_int = 0o000200;
pub const PARENB: ::c_int = 0o000400;
pub const PARODD: ::c_int = 0o001000;
pub const HUPCL: ::c_int = 0o002000;
pub const CLOCAL: ::c_int = 0o004000;

// abis/linux/termios.h - bitwise constants for c_lflag in struct termios
pub const ECHO: ::c_uint = 0o000010;
pub const ECHOE: ::c_int = 0o000020;
pub const ECHOK: ::c_int = 0o000040;
pub const ECHONL: ::c_int = 0o000100;
pub const ICANON: ::c_int = 0o000002;
pub const IEXTEN: ::c_int = 0o100000;
pub const ISIG: ::c_int = 0o000001;
pub const NOFLSH: ::c_int = 0o000200;
pub const TOSTOP: ::c_int = 0o000400;
pub const ECHOPRT: ::c_int = 0o002000;

// abis/mlibc/vm-flags.h
pub const MAP_ANON: ::c_int = 8;
pub const MAP_PRIVATE: ::c_int = 1;
pub const MAP_SHARED: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_NONE: ::c_int = 0;
pub const MAP_FIXED: ::c_int = 2;
pub const MAP_NORESERVE: ::c_int = 0x10;

// options/posix/include/sys/mman.h
pub const MAP_FILE: ::c_int = 0;
pub const MAP_FAILED: *mut ::c_void = usize::MAX as *mut ::c_void;
pub const MS_ASYNC: ::c_int = 1;
pub const MS_SYNC: ::c_int = 2;
pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_RANDOM: ::c_int = 1;
pub const MADV_SEQUENTIAL: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;
pub const MADV_FREE: ::c_int = 8;
pub const MS_INVALIDATE: ::c_int = 4;
pub const MCL_CURRENT: ::c_int = 1;
pub const MCL_FUTURE: ::c_int = 2;

// options/ansi/include/time.h
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_REALTIME: clockid_t = 0;

// options/posix/include/netdb.h
pub const EAI_SYSTEM: ::c_int = 9;

// abis/mlibc/in.h
pub const IPV6_JOIN_GROUP: ::c_int = 1;
pub const IPV6_LEAVE_GROUP: ::c_int = 2;
pub const IPV6_MULTICAST_HOPS: ::c_int = 3;
pub const IPV6_MULTICAST_IF: ::c_int = 4;
pub const IPV6_MULTICAST_LOOP: ::c_int = 5;
pub const IPV6_UNICAST_HOPS: ::c_int = 6;
pub const IPV6_V6ONLY: ::c_int = 7;
pub const IPV6_PMTUDISC_DONT: ::c_int = 8;
pub const IPV6_PMTUDISC_DO: ::c_int = 9;
pub const IPV6_MTU_DISCOVER: ::c_int = 23;
pub const IPV6_RECVERR: ::c_int = 25;
pub const IPV6_RECVPKTINFO: ::c_int = 49;
pub const IPV6_PKTINFO: ::c_int = 50;
pub const IPV6_RECVHOPLIMIT: ::c_int = 51;
pub const IPV6_HOPLIMIT: ::c_int = 52;
pub const IPV6_TCLASS: ::c_int = 67;
pub const IP_TOS: ::c_int = 1;
pub const IP_TTL: ::c_int = 2;
pub const IP_OPTIONS: ::c_int = 4;
pub const IP_PKTINFO: ::c_int = 8;
pub const IP_MTU_DISCOVER: ::c_int = 10;
pub const IP_RECVERR: ::c_int = 11;
pub const IP_RECVTTL: ::c_int = 12;

pub const IP_DEFAULT_MULTICAST_TTL: ::c_int = 1;
pub const IP_MULTICAST_IF: ::c_int = 32;
pub const IP_MULTICAST_TTL: ::c_int = 33;
pub const IP_MULTICAST_LOOP: ::c_int = 34;
pub const IP_ADD_MEMBERSHIP: ::c_int = 35;
pub const IP_DROP_MEMBERSHIP: ::c_int = 36;
pub const IP_UNBLOCK_SOURCE: ::c_int = 37;
pub const IP_BLOCK_SOURCE: ::c_int = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: ::c_int = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: ::c_int = 40;
pub const MCAST_JOIN_SOURCE_GROUP: ::c_int = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: ::c_int = 47;

pub const IPV6_ADD_MEMBERSHIP: ::c_int = IPV6_JOIN_GROUP;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = IPV6_LEAVE_GROUP;

// FIXME: not defined in mlibc
pub const TCP_KEEPIDLE: ::c_int = 4;

// abis/linux/signal.h
pub const SIGABRT: ::c_int = 6;
pub const SIGFPE: ::c_int = 8;
pub const SIGILL: ::c_int = 4;
pub const SIGINT: ::c_int = 2;
pub const SIGSEGV: ::c_int = 11;
pub const SIGTERM: ::c_int = 15;
pub const SIGPROF: ::c_int = 27;
pub const SIGIO: ::c_int = 29;
pub const SIGPWR: ::c_int = 30;
pub const SIGRTMIN: ::c_int = 35;
pub const SIGRTMAX: ::c_int = 64;
pub const SIGHUP: ::c_int = 1;
pub const SIGQUIT: ::c_int = 3;
pub const SIGTRAP: ::c_int = 5;
pub const SIGIOT: ::c_int = SIGABRT;
pub const SIGBUS: ::c_int = 7;
pub const SIGKILL: ::c_int = 9;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGCHLD: ::c_int = 17;
pub const SIGCONT: ::c_int = 18;
pub const SIGSTOP: ::c_int = 19;
pub const SIGTSTP: ::c_int = 20;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGURG: ::c_int = 23;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGWINCH: ::c_int = 28;
pub const SIGPOLL: ::c_int = 29;
pub const SIGSYS: ::c_int = 31;
pub const SIGUNUSED: ::c_int = SIGSYS;
pub const SIGCANCEL: ::c_int = 32;

pub const SA_SIGINFO: ::c_int = 4;
pub const SA_RESTART: ::c_int = 0x10000000;

pub const SIG_DFL: ::sighandler_t = 0 as ::sighandler_t;
pub const SIG_IGN: ::sighandler_t = 1 as ::sighandler_t;
pub const SIG_ERR: ::sighandler_t = !0 as ::sighandler_t;

// abis/mlibc/signal.h - constants for sigprocmask()
pub const SIG_BLOCK: ::c_int = 0;
pub const SIG_UNBLOCK: ::c_int = 1;
pub const SIG_SETMASK: ::c_int = 2;

// abis/mlibc/termios.h
pub const B0: ::c_uint = 0;
pub const B50: ::c_uint = 1;
pub const B75: ::c_uint = 2;
pub const B110: ::c_uint = 3;
pub const B134: ::c_uint = 4;
pub const B150: ::c_uint = 5;
pub const B200: ::c_uint = 6;
pub const B300: ::c_uint = 7;
pub const B600: ::c_uint = 8;
pub const B1200: ::c_uint = 9;
pub const B1800: ::c_uint = 10;
pub const B2400: ::c_uint = 11;
pub const B4800: ::c_uint = 12;
pub const B9600: ::c_uint = 13;
pub const B19200: ::c_uint = 14;
pub const B38400: ::c_uint = 15;
pub const B57600: ::c_uint = 16;
pub const B115200: ::c_uint = 17;
pub const B230400: ::c_uint = 18;
pub const TCIFLUSH: ::c_int = 1;
pub const TCIOFF: ::c_int = 1;
pub const TCIOFLUSH: ::c_int = 2;
pub const TCION: ::c_int = 2;
pub const TCOFLUSH: ::c_int = 3;
pub const TCOOFF: ::c_int = 3;
pub const TCOON: ::c_int = 4;
pub const TCSADRAIN: ::c_int = 2;
pub const TCSAFLUSH: ::c_int = 3;
pub const TCSANOW: ::c_int = 1;
pub const TIOCSCTTY: ::c_ulong = 0x540e;
pub const TIOCSWINSZ: ::c_ulong = 0x5414;

// options/posix/include/termios.h
pub const TIOCGWINSZ: ::c_ulong = 0x5413;

// options/ansi/include/locale.h
pub const LC_CTYPE: ::c_int = 3;

// options/ansi/include/stdlib.h
pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;

// options/posix/include/dlfcn.h
pub const RTLD_DEFAULT: *mut ::c_void = 0 as *mut ::c_void;
pub const RTLD_LAZY: ::c_int = 32;

// options/posix/include/unistd.h
pub const F_OK: ::c_int = 1;
pub const R_OK: ::c_int = 2;
pub const STDERR_FILENO: ::c_int = 2;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const W_OK: ::c_int = 4;
pub const X_OK: ::c_int = 8;
pub const _PC_NAME_MAX: ::c_int = 3;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 7;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 1;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 6;
pub const _SC_NGROUPS_MAX: ::c_int = 10;
pub const _SC_PAGESIZE: ::c_int = _SC_PAGE_SIZE;
pub const _SC_PAGE_SIZE: ::c_int = 3;

// abis/mlibc/socket.h
pub const AF_APPLETALK: ::c_int = PF_APPLETALK;
pub const AF_BLUETOOTH: ::c_int = PF_BLUETOOTH;
pub const AF_DECnet: ::c_int = PF_DECnet;
pub const AF_INET6: ::c_int = PF_INET6;
pub const AF_INET: ::c_int = PF_INET;
pub const AF_IPX: ::c_int = PF_IPX;
pub const AF_ISDN: ::c_int = PF_ISDN;
pub const AF_PACKET: ::c_int = PF_PACKET;
pub const AF_SNA: ::c_int = PF_SNA;
pub const AF_UNIX: ::c_int = 3;
pub const MSG_PEEK: ::c_int = 0x20;
pub const MSG_TRUNC: ::c_int = 0x40;
pub const AF_UNSPEC: ::c_int = PF_UNSPEC;
pub const MSG_CTRUNC: ::c_int = 1;
pub const MSG_CMSG_CLOEXEC: ::c_int = 0x2000;
pub const MSG_EOR: ::c_int = 4;
pub const MSG_OOB: ::c_int = 8;
pub const MSG_WAITALL: ::c_int = 0x80;
pub const PF_APPLETALK: ::c_int = 7;
pub const PF_BLUETOOTH: ::c_int = 8;
pub const PF_DECnet: ::c_int = 9;
pub const MSG_DONTWAIT: ::c_int = 0x1000;
pub const PF_INET6: ::c_int = 2;
pub const PF_INET: ::c_int = 1;
pub const PF_UNIX: ::c_int = 3;
pub const PF_UNSPEC: ::c_int = 4;
pub const SCM_RIGHTS: ::c_int = 1;
pub const SCM_TIMESTAMP: ::c_int = SO_TIMESTAMP;
pub const SHUT_RD: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;
pub const SHUT_WR: ::c_int = 3;
pub const SOCK_DGRAM: ::c_int = 1;
pub const SOCK_RAW: ::c_int = 2;
pub const SOCK_RDM: ::c_int = 0x40000;
pub const SOCK_SEQPACKET: ::c_int = 3;
pub const PF_IPX: ::c_int = 10;
pub const PF_ISDN: ::c_int = 11;
pub const PF_PACKET: ::c_int = 13;
pub const PF_SNA: ::c_int = 12;
pub const SOCK_STREAM: ::c_int = 4;
pub const SOL_SOCKET: ::c_int = 1;
pub const SO_ACCEPTCONN: ::c_int = 1;
pub const SO_BROADCAST: ::c_int = 2;
pub const SO_DEBUG: ::c_int = 3;
pub const SO_DONTROUTE: ::c_int = 4;
pub const SO_ERROR: ::c_int = 5;
pub const SO_KEEPALIVE: ::c_int = 6;
pub const SO_LINGER: ::c_int = 7;
pub const SO_OOBINLINE: ::c_int = 8;
pub const SO_RCVBUF: ::c_int = 9;
pub const SO_RCVLOWAT: ::c_int = 10;
pub const SO_RCVTIMEO: ::c_int = 11;
pub const SO_REUSEADDR: ::c_int = 12;
pub const SO_SNDBUF: ::c_int = 13;
pub const SO_SNDLOWAT: ::c_int = 14;
pub const SO_SNDTIMEO: ::c_int = 15;
pub const SO_TYPE: ::c_int = 16;
pub const SO_SNDBUFFORCE: ::c_int = 17;
pub const TCP_KEEPCNT: ::c_int = 6;
pub const TCP_KEEPINTVL: ::c_int = 5;
pub const SO_PEERCRED: ::c_int = 18;
pub const SO_ATTACH_FILTER: ::c_int = 19;
pub const SO_PASSCRED: ::c_int = 20;
pub const SO_RCVBUFFORCE: ::c_int = 21;
pub const SO_DETACH_FILTER: ::c_int = 22;
pub const SO_PROTOCOL: ::c_int = 23;
pub const SO_REUSEPORT: ::c_int = 24;
pub const SO_TIMESTAMP: ::c_int = 25;

// options/posix/include/sys/file.h
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_SH: ::c_int = 1;
pub const LOCK_UN: ::c_int = 8;

// abis/mlibc/errno.h
pub const EDOM: ::c_int = 1;
pub const EILSEQ: ::c_int = 2;
pub const ERANGE: ::c_int = 3;
pub const E2BIG: ::c_int = 1001;
pub const EACCES: ::c_int = 1002;
pub const EADDRINUSE: ::c_int = 1003;
pub const EADDRNOTAVAIL: ::c_int = 1004;
pub const EAFNOSUPPORT: ::c_int = 1005;
pub const EAGAIN: ::c_int = 1006;
pub const EALREADY: ::c_int = 1007;
pub const EBADF: ::c_int = 1008;
pub const EBADMSG: ::c_int = 1009;
pub const EBUSY: ::c_int = 1010;
pub const ECANCELED: ::c_int = 1011;
pub const ECHILD: ::c_int = 1012;
pub const ECONNABORTED: ::c_int = 1013;
pub const ECONNREFUSED: ::c_int = 1014;
pub const ECONNRESET: ::c_int = 1015;
pub const EDEADLK: ::c_int = 1016;
pub const EDESTADDRREQ: ::c_int = 1017;
pub const EDQUOT: ::c_int = 1018;
pub const EEXIST: ::c_int = 1019;
pub const EFAULT: ::c_int = 1020;
pub const EFBIG: ::c_int = 1021;
pub const EHOSTUNREACH: ::c_int = 1022;
pub const EIDRM: ::c_int = 1023;
pub const EINPROGRESS: ::c_int = 1024;
pub const EINTR: ::c_int = 1025;
pub const EINVAL: ::c_int = 1026;
pub const EIO: ::c_int = 1027;
pub const EISCONN: ::c_int = 1028;
pub const EISDIR: ::c_int = 1029;
pub const ELOOP: ::c_int = 1030;
pub const EMFILE: ::c_int = 1031;
pub const EMLINK: ::c_int = 1032;
pub const EMSGSIZE: ::c_int = 1034;
pub const EMULTIHOP: ::c_int = 1035;
pub const ENAMETOOLONG: ::c_int = 1036;
pub const ENETDOWN: ::c_int = 1037;
pub const ENETRESET: ::c_int = 1038;
pub const ENETUNREACH: ::c_int = 1039;
pub const ENFILE: ::c_int = 1040;
pub const ENOBUFS: ::c_int = 1041;
pub const ENODEV: ::c_int = 1042;
pub const ENOENT: ::c_int = 1043;
pub const ENOEXEC: ::c_int = 1044;
pub const ENOLCK: ::c_int = 1045;
pub const ENOLINK: ::c_int = 1046;
pub const ENOMEM: ::c_int = 1047;
pub const ENOMSG: ::c_int = 1048;
pub const ENOPROTOOPT: ::c_int = 1049;
pub const ENOSPC: ::c_int = 1050;
pub const ENOSYS: ::c_int = 1051;
pub const ENOTCONN: ::c_int = 1052;
pub const ENOTDIR: ::c_int = 1053;
pub const ENOTEMPTY: ::c_int = 1054;
pub const ENOTRECOVERABLE: ::c_int = 1055;
pub const ENOTSOCK: ::c_int = 1056;
pub const ENOTSUP: ::c_int = 1057;
pub const ENOTTY: ::c_int = 1058;
pub const ENXIO: ::c_int = 1059;
pub const EOPNOTSUPP: ::c_int = 1060;
pub const EOVERFLOW: ::c_int = 1061;
pub const EOWNERDEAD: ::c_int = 1062;
pub const EPERM: ::c_int = 1063;
pub const EPIPE: ::c_int = 1064;
pub const EPROTO: ::c_int = 1065;
pub const EPROTONOSUPPORT: ::c_int = 1066;
pub const EPROTOTYPE: ::c_int = 1067;
pub const EROFS: ::c_int = 1068;
pub const ESPIPE: ::c_int = 1069;
pub const ESRCH: ::c_int = 1070;
pub const ESTALE: ::c_int = 1071;
pub const ETIMEDOUT: ::c_int = 1072;
pub const ETXTBSY: ::c_int = 1073;
pub const EWOULDBLOCK: ::c_int = EAGAIN;
pub const EXDEV: ::c_int = 1075;
pub const ENODATA: ::c_int = 1076;
pub const ETIME: ::c_int = 1077;
pub const ENOKEY: ::c_int = 1078;
pub const ESHUTDOWN: ::c_int = 1079;
pub const EHOSTDOWN: ::c_int = 1080;
pub const EBADFD: ::c_int = 1081;
pub const ENOMEDIUM: ::c_int = 1082;
pub const ENOTBLK: ::c_int = 1083;

// options/posix/include/fcntl.h
pub const AT_FDCWD: ::c_int = -100;
pub const F_DUPFD_CLOEXEC: ::c_int = 2;
pub const AT_REMOVEDIR: ::c_int = 8;
pub const F_GETFD: ::c_int = 3;
pub const F_SETFD: ::c_int = 4;
pub const AT_SYMLINK_FOLLOW: ::c_int = 2;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 4;
pub const F_DUPFD: ::c_int = 1;
pub const F_GETFL: ::c_int = 5;
pub const F_SETFL: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const O_DIRECTORY: ::c_int = 0x00020;
pub const O_NOFOLLOW: ::c_int = 0x100;
pub const O_ASYNC: ::c_int = 0x40000;
pub const O_NDELAY: ::c_int = 0x400;
pub const O_NOCTTY: ::c_int = 0x80;
pub const O_SYNC: ::c_int = 0x2000;
pub const F_GETOWN: ::c_int = 10;
pub const F_SETOWN: ::c_int = 11;
pub const O_ACCMODE: ::c_int = 7;
pub const O_APPEND: ::c_int = 8;
pub const O_CLOEXEC: ::c_int = 0x4000;
pub const O_CREAT: ::c_int = 0x10;
pub const O_EXCL: ::c_int = 0x40;
pub const O_NONBLOCK: ::c_int = 0x400;
pub const O_RDONLY: ::c_int = 2;
pub const O_RDWR: ::c_int = 3;
pub const O_TRUNC: ::c_int = 0x200;
pub const O_WRONLY: ::c_int = 5;

// options/mlibc/seek-whence.h
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const SEEK_SET: ::c_int = 3;

// options/posix/include/netinet/tcp.h
pub const TCP_NODELAY: ::c_int = 1;

// abis/mlibc/stat.h
pub const S_IFBLK: mode_t = 0x6000;
pub const S_IFCHR: mode_t = 0x2000;
pub const S_IFDIR: mode_t = 0x4000;
pub const S_IFIFO: mode_t = 0x1000;
pub const S_IFLNK: mode_t = 0xA000;
pub const S_IFMT: mode_t = 0xF000;
pub const S_IFREG: mode_t = 0x8000;
pub const S_IFSOCK: mode_t = 0xC000;
pub const S_IRGRP: mode_t = 0o40;
pub const S_IROTH: mode_t = 0o4;
pub const S_IRUSR: mode_t = 0o400;
pub const S_IRWXG: mode_t = 0o70;
pub const S_IRWXO: mode_t = 0o7;
pub const S_IRWXU: mode_t = 0o700;
pub const S_IWGRP: mode_t = 0o20;
pub const S_IWOTH: mode_t = 0o2;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXGRP: mode_t = 0o10;
pub const S_IXOTH: mode_t = 0o1;
pub const S_IXUSR: mode_t = 0o100;

// Used by utimensat() and friends
pub const UTIME_NOW: c_long = (1 << 30) - 1;
pub const UTIME_OMIT: c_long = (1 << 30) - 2;

// options/posix/include/sys/wait.h
pub const WCOREFLAG: ::c_int = 0x80;
pub const WNOHANG: ::c_int = 2;

// options/linux/include/sys/poll.h
pub const POLLHUP: ::c_short = 8;
pub const POLLIN: ::c_short = 1;
pub const POLLNVAL: ::c_short = 0x40;
pub const POLLOUT: ::c_short = 2;

// options/linux/include/sys/epoll.h
pub const EPOLLERR: ::c_int = 8;
pub const EPOLLET: ::c_int = 1 << 31;
pub const EPOLLHUP: ::c_int = 0x10;
pub const EPOLLIN: ::c_int = 1;
pub const EPOLLMSG: ::c_int = 0x400;
pub const EPOLLONESHOT: ::c_int = 1 << 30;
pub const EPOLLOUT: ::c_int = 4;
pub const EPOLLPRI: ::c_int = 2;
pub const EPOLLRDBAND: ::c_int = 0x80;
pub const EPOLLRDHUP: ::c_int = 0x2000;
pub const EPOLLRDNORM: ::c_int = 0x40;
pub const EPOLLWRBAND: ::c_int = 0x200;
pub const EPOLLWRNORM: ::c_int = 0x100;
pub const EPOLLWAKEUP: ::c_int = 1 << 29;
pub const EPOLL_CLOEXEC: ::c_int = 1;
pub const EPOLL_CTL_ADD: ::c_int = 1;
pub const EPOLL_CTL_DEL: ::c_int = 2;
pub const EPOLL_CTL_MOD: ::c_int = 3;

// options/linux/include/sys/eventfd.h
pub const EFD_CLOEXEC: ::c_int = O_CLOEXEC;
pub const EFD_NONBLOCK: ::c_int = O_NONBLOCK;

// options/glibc/include/sys/ioctl.h
pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

// options/ansi/include/limits.h
pub const PTHREAD_STACK_MIN: ::size_t = 16384;
pub const PATH_MAX: ::size_t = 4096;

// abis/linux/ioctls.h
pub const SIOCGIFHWADDR: ::c_ulong = 0x8927;
pub const SIOCSIFADDR: ::c_ulong = 0x8916;
pub const SIOCSIFNETMASK: ::c_ulong = 0x891c;

// options/posix/include/pthread.h
align_const! {
    pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
        __mlibc_state: 0,
        __mlibc_recursion: 0,
        __mlibc_flags: 0,
        __mlibc_prioceiling: 0,
    };
    pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
        __mlibc_seq: 0,
        __mlibc_flags: 0,
        __mlibc_clock: 0,
    };
    pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
        __mlibc_m: 0,
        __mlibc_rc: 0,
        __mlibc_padding: 0,
    };
}

pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 1;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_MUTEX_STALLED: ::c_int = 0;
pub const PTHREAD_MUTEX_ROBUST: ::c_int = 1;
pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 0;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 1;

extern "C" {
    pub fn __errno_location() -> *mut ::c_int;
    pub fn acct(filename: *const ::c_char) -> ::c_int;
    pub fn bind(socket: ::c_int, address: *const ::sockaddr, address_len: ::socklen_t) -> ::c_int;
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_settime(clk_id: clockid_t, tp: *const ::timespec) -> ::c_int;
    pub fn endpwent();
    pub fn epoll_create(size: ::c_int) -> ::c_int;
    pub fn epoll_create1(flags: ::c_int) -> ::c_int;
    pub fn epoll_ctl(epfd: ::c_int, op: ::c_int, fd: ::c_int, event: *mut ::epoll_event)
        -> ::c_int;
    pub fn epoll_wait(
        epfd: ::c_int,
        events: *mut ::epoll_event,
        maxevents: ::c_int,
        timeout: ::c_int,
    ) -> ::c_int;
    pub fn eventfd(init: ::c_uint, flags: ::c_int) -> ::c_int;
    pub fn forkpty(
        amaster: *mut ::c_int,
        name: *mut ::c_char,
        termp: *mut termios,
        winp: *mut ::winsize,
    ) -> ::pid_t;
    pub fn futimes(file: ::c_int, times: *const ::timeval) -> ::c_int;
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
    pub fn getpwent() -> *mut passwd;
    pub fn openpty(
        amaster: *mut ::c_int,
        aslave: *mut ::c_int,
        name: *mut ::c_char,
        termp: *const termios,
        winp: *const ::winsize,
    ) -> ::c_int;
    pub fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int;
    pub fn getgrgid_r(
        gid: ::gid_t,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn getgrnam_r(
        name: *const ::c_char,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn getgrouplist(
        user: *const ::c_char,
        group: ::gid_t,
        groups: *mut ::gid_t,
        ngroups: *mut ::c_int,
    ) -> ::c_int;
    pub fn getpwnam_r(
        name: *const ::c_char,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn getpwuid_r(
        uid: ::uid_t,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn initgroups(user: *const ::c_char, group: ::gid_t) -> ::c_int;
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    pub fn lutimes(file: *const ::c_char, times: *const ::timeval) -> ::c_int;
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int;
    pub fn sethostname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn shm_open(name: *const c_char, oflag: ::c_int, mode: mode_t) -> ::c_int;
    pub fn shm_unlink(name: *const ::c_char) -> ::c_int;
    pub fn madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int) -> ::c_int;
    pub fn mkfifoat(dirfd: ::c_int, pathname: *const ::c_char, mode: ::mode_t) -> ::c_int;
    pub fn utimensat(
        dirfd: ::c_int,
        path: *const ::c_char,
        times: *const ::timespec,
        flag: ::c_int,
    ) -> ::c_int;
    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock_id: clockid_t)
        -> ::c_int;
    pub fn pthread_create(
        thread: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;
    pub fn pthread_setname_np(t: ::pthread_t, name: *const ::c_char) -> ::c_int;
    pub fn pthread_sigmask(how: ::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> ::c_int;
    pub fn readv(fd: ::c_int, iov: *const ::iovec, count: ::c_int) -> ::ssize_t;
    pub fn recvfrom(
        socket: ::c_int,
        buf: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::ssize_t;
    pub fn recvmsg(fd: ::c_int, msg: *mut ::msghdr, flags: ::c_int) -> ::ssize_t;
    pub fn sendmsg(fd: ::c_int, msg: *const ::msghdr, flags: ::c_int) -> ::ssize_t;
    pub fn setgroups(ngroups: ::c_int, ptr: *const ::gid_t) -> ::c_int;
    pub fn setpwent();
    pub fn writev(fd: ::c_int, iov: *const ::iovec, count: ::c_int) -> ::ssize_t;

    // Non standard extensions, also found on modern BSD's.
    //
    // options/posix/include/sys/uio.h
    pub fn preadv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int, offset: ::off_t) -> ::ssize_t;
    pub fn pwritev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int, offset: ::off_t)
        -> ::ssize_t;
}
