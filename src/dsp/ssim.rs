use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8DistoStats {
    pub w: uint32_t,
    pub xm: uint32_t,
    pub ym: uint32_t,
    pub xxm: uint32_t,
    pub xym: uint32_t,
    pub yym: uint32_t,
}
pub type VP8SSIMGetClippedFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_double,
>;
pub type VP8SSIMGetFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        *const uint8_t,
        libc::c_int,
    ) -> libc::c_double,
>;
pub type VP8AccumulateSSEFunc = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, libc::c_int) -> uint32_t,
>;
static mut kWeight: [uint32_t; 7] = [
    1 as libc::c_int as uint32_t,
    2 as libc::c_int as uint32_t,
    3 as libc::c_int as uint32_t,
    4 as libc::c_int as uint32_t,
    3 as libc::c_int as uint32_t,
    2 as libc::c_int as uint32_t,
    1 as libc::c_int as uint32_t,
];
static mut kWeightSum: uint32_t = (16 as libc::c_int * 16 as libc::c_int) as uint32_t;
#[inline]
unsafe extern "C" fn SSIMCalculation(
    stats: *const VP8DistoStats,
    mut N: uint32_t,
) -> libc::c_double {
    let w2: uint32_t = N.wrapping_mul(N);
    let C1: uint32_t = (20 as libc::c_int as libc::c_uint).wrapping_mul(w2);
    let C2: uint32_t = (60 as libc::c_int as libc::c_uint).wrapping_mul(w2);
    let C3: uint32_t = ((8 as libc::c_int * 8 as libc::c_int) as libc::c_uint)
        .wrapping_mul(w2);
    let xmxm: uint64_t = ((*stats).xm as uint64_t)
        .wrapping_mul((*stats).xm as libc::c_ulong);
    let ymym: uint64_t = ((*stats).ym as uint64_t)
        .wrapping_mul((*stats).ym as libc::c_ulong);
    if xmxm.wrapping_add(ymym) >= C3 as libc::c_ulong {
        let xmym: int64_t = (*stats).xm as int64_t * (*stats).ym as libc::c_long;
        let sxy: int64_t = (*stats).xym as int64_t * N as libc::c_long - xmym;
        let sxx: uint64_t = ((*stats).xxm as uint64_t)
            .wrapping_mul(N as libc::c_ulong)
            .wrapping_sub(xmxm);
        let syy: uint64_t = ((*stats).yym as uint64_t)
            .wrapping_mul(N as libc::c_ulong)
            .wrapping_sub(ymym);
        let num_S: uint64_t = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                (if sxy < 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int as libc::c_long
                } else {
                    sxy
                }) as uint64_t,
            )
            .wrapping_add(C2 as libc::c_ulong) >> 8 as libc::c_int;
        let den_S: uint64_t = sxx.wrapping_add(syy).wrapping_add(C2 as libc::c_ulong)
            >> 8 as libc::c_int;
        let fnum: uint64_t = ((2 as libc::c_int as libc::c_long * xmym
            + C1 as libc::c_long) as libc::c_ulong)
            .wrapping_mul(num_S);
        let fden: uint64_t = xmxm
            .wrapping_add(ymym)
            .wrapping_add(C1 as libc::c_ulong)
            .wrapping_mul(den_S);
        let r: libc::c_double = fnum as libc::c_double / fden as libc::c_double;
        return r;
    }
    return 1.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn VP8SSIMFromStats(
    stats: *const VP8DistoStats,
) -> libc::c_double {
    return SSIMCalculation(stats, kWeightSum);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SSIMFromStatsClipped(
    stats: *const VP8DistoStats,
) -> libc::c_double {
    return SSIMCalculation(stats, (*stats).w);
}
unsafe extern "C" fn SSIMGetClipped_C(
    mut src1: *const uint8_t,
    mut stride1: libc::c_int,
    mut src2: *const uint8_t,
    mut stride2: libc::c_int,
    mut xo: libc::c_int,
    mut yo: libc::c_int,
    mut W: libc::c_int,
    mut H: libc::c_int,
) -> libc::c_double {
    let mut stats: VP8DistoStats = {
        let mut init = VP8DistoStats {
            w: 0 as libc::c_int as uint32_t,
            xm: 0 as libc::c_int as uint32_t,
            ym: 0 as libc::c_int as uint32_t,
            xxm: 0 as libc::c_int as uint32_t,
            xym: 0 as libc::c_int as uint32_t,
            yym: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let ymin: libc::c_int = if (yo - 3 as libc::c_int) < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        yo - 3 as libc::c_int
    };
    let ymax: libc::c_int = if yo + 3 as libc::c_int > H - 1 as libc::c_int {
        H - 1 as libc::c_int
    } else {
        yo + 3 as libc::c_int
    };
    let xmin: libc::c_int = if (xo - 3 as libc::c_int) < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        xo - 3 as libc::c_int
    };
    let xmax: libc::c_int = if xo + 3 as libc::c_int > W - 1 as libc::c_int {
        W - 1 as libc::c_int
    } else {
        xo + 3 as libc::c_int
    };
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    src1 = src1.offset((ymin * stride1) as isize);
    src2 = src2.offset((ymin * stride2) as isize);
    y = ymin;
    while y <= ymax {
        x = xmin;
        while x <= xmax {
            let w: uint32_t = (kWeight[(3 as libc::c_int + x - xo) as usize])
                .wrapping_mul(kWeight[(3 as libc::c_int + y - yo) as usize]);
            let s1: uint32_t = *src1.offset(x as isize) as uint32_t;
            let s2: uint32_t = *src2.offset(x as isize) as uint32_t;
            stats.w = (stats.w as libc::c_uint).wrapping_add(w) as uint32_t as uint32_t;
            stats
                .xm = (stats.xm as libc::c_uint).wrapping_add(w.wrapping_mul(s1))
                as uint32_t as uint32_t;
            stats
                .ym = (stats.ym as libc::c_uint).wrapping_add(w.wrapping_mul(s2))
                as uint32_t as uint32_t;
            stats
                .xxm = (stats.xxm as libc::c_uint)
                .wrapping_add(w.wrapping_mul(s1).wrapping_mul(s1)) as uint32_t
                as uint32_t;
            stats
                .xym = (stats.xym as libc::c_uint)
                .wrapping_add(w.wrapping_mul(s1).wrapping_mul(s2)) as uint32_t
                as uint32_t;
            stats
                .yym = (stats.yym as libc::c_uint)
                .wrapping_add(w.wrapping_mul(s2).wrapping_mul(s2)) as uint32_t
                as uint32_t;
            x += 1;
            x;
        }
        y += 1;
        y;
        src1 = src1.offset(stride1 as isize);
        src2 = src2.offset(stride2 as isize);
    }
    return VP8SSIMFromStatsClipped(&mut stats);
}
unsafe extern "C" fn SSIMGet_C(
    mut src1: *const uint8_t,
    mut stride1: libc::c_int,
    mut src2: *const uint8_t,
    mut stride2: libc::c_int,
) -> libc::c_double {
    let mut stats: VP8DistoStats = {
        let mut init = VP8DistoStats {
            w: 0 as libc::c_int as uint32_t,
            xm: 0 as libc::c_int as uint32_t,
            ym: 0 as libc::c_int as uint32_t,
            xxm: 0 as libc::c_int as uint32_t,
            xym: 0 as libc::c_int as uint32_t,
            yym: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y <= 2 as libc::c_int * 3 as libc::c_int {
        x = 0 as libc::c_int;
        while x <= 2 as libc::c_int * 3 as libc::c_int {
            let w: uint32_t = (kWeight[x as usize]).wrapping_mul(kWeight[y as usize]);
            let s1: uint32_t = *src1.offset(x as isize) as uint32_t;
            let s2: uint32_t = *src2.offset(x as isize) as uint32_t;
            stats
                .xm = (stats.xm as libc::c_uint).wrapping_add(w.wrapping_mul(s1))
                as uint32_t as uint32_t;
            stats
                .ym = (stats.ym as libc::c_uint).wrapping_add(w.wrapping_mul(s2))
                as uint32_t as uint32_t;
            stats
                .xxm = (stats.xxm as libc::c_uint)
                .wrapping_add(w.wrapping_mul(s1).wrapping_mul(s1)) as uint32_t
                as uint32_t;
            stats
                .xym = (stats.xym as libc::c_uint)
                .wrapping_add(w.wrapping_mul(s1).wrapping_mul(s2)) as uint32_t
                as uint32_t;
            stats
                .yym = (stats.yym as libc::c_uint)
                .wrapping_add(w.wrapping_mul(s2).wrapping_mul(s2)) as uint32_t
                as uint32_t;
            x += 1;
            x;
        }
        y += 1;
        y;
        src1 = src1.offset(stride1 as isize);
        src2 = src2.offset(stride2 as isize);
    }
    return VP8SSIMFromStats(&mut stats);
}
unsafe extern "C" fn AccumulateSSE_C(
    mut src1: *const uint8_t,
    mut src2: *const uint8_t,
    mut len: libc::c_int,
) -> uint32_t {
    let mut i: libc::c_int = 0;
    let mut sse2: uint32_t = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int;
    while i < len {
        let diff: int32_t = *src1.offset(i as isize) as libc::c_int
            - *src2.offset(i as isize) as libc::c_int;
        sse2 = (sse2 as libc::c_uint).wrapping_add((diff * diff) as libc::c_uint)
            as uint32_t as uint32_t;
        i += 1;
        i;
    }
    return sse2;
}
#[no_mangle]
pub static mut VP8SSIMGet: VP8SSIMGetFunc = None;
#[no_mangle]
pub static mut VP8SSIMGetClipped: VP8SSIMGetClippedFunc = None;
#[no_mangle]
pub static mut VP8AccumulateSSE: VP8AccumulateSSEFunc = None;
unsafe extern "C" fn VP8SSIMDspInit_body() {
    VP8SSIMGetClipped = Some(
        SSIMGetClipped_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> libc::c_double,
    );
    VP8SSIMGet = Some(
        SSIMGet_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                *const uint8_t,
                libc::c_int,
            ) -> libc::c_double,
    );
    VP8AccumulateSSE = Some(
        AccumulateSSE_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                libc::c_int,
            ) -> uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8SSIMDspInit() {
    static mut VP8SSIMDspInit_body_lock: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int as libc::c_short,
                __elision: 0 as libc::c_int as libc::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    if !(pthread_mutex_lock(&mut VP8SSIMDspInit_body_lock) != 0) {
        VP8SSIMDspInit_body();
        pthread_mutex_unlock(&mut VP8SSIMDspInit_body_lock);
    }
}
