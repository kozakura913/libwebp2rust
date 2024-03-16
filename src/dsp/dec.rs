use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static VP8ksclip1: *const int8_t;
    static VP8ksclip2: *const int8_t;
    static VP8kclip1: *const uint8_t;
    static VP8kabs0: *const uint8_t;
    fn VP8InitClipTables();
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type CPUFeature = libc::c_uint;
pub const kMSA: CPUFeature = 9;
pub const kMIPSdspR2: CPUFeature = 8;
pub const kMIPS32: CPUFeature = 7;
pub const kNEON: CPUFeature = 6;
pub const kAVX2: CPUFeature = 5;
pub const kAVX: CPUFeature = 4;
pub const kSSE4_1: CPUFeature = 3;
pub const kSlowSSSE3: CPUFeature = 2;
pub const kSSE3: CPUFeature = 1;
pub const kSSE2: CPUFeature = 0;
pub type VP8DecIdct = Option::<unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ()>;
pub type VP8DecIdct2 = Option::<
    unsafe extern "C" fn(*const int16_t, *mut uint8_t, libc::c_int) -> (),
>;
pub type VP8PredFunc = Option::<unsafe extern "C" fn(*mut uint8_t) -> ()>;
pub type VP8SimpleFilterFunc = Option::<
    unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int) -> (),
>;
pub type VP8LumaFilterFunc = Option::<
    unsafe extern "C" fn(
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type VP8ChromaFilterFunc = Option::<
    unsafe extern "C" fn(
        *mut uint8_t,
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[inline]
unsafe extern "C" fn WebPUint32ToMem(ptr: *mut uint8_t, mut val: uint32_t) {
    memcpy(
        ptr as *mut libc::c_void,
        &mut val as *mut uint32_t as *const libc::c_void,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn clip_8b(mut v: libc::c_int) -> uint8_t {
    return (if v & !(0xff as libc::c_int) == 0 {
        v
    } else if v < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        255 as libc::c_int
    }) as uint8_t;
}
unsafe extern "C" fn TransformOne_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    let mut C: [libc::c_int; 16] = [0; 16];
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    tmp = C.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let a: libc::c_int = *in_0.offset(0 as libc::c_int as isize) as libc::c_int
            + *in_0.offset(8 as libc::c_int as isize) as libc::c_int;
        let b: libc::c_int = *in_0.offset(0 as libc::c_int as isize) as libc::c_int
            - *in_0.offset(8 as libc::c_int as isize) as libc::c_int;
        let c: libc::c_int = (*in_0.offset(4 as libc::c_int as isize) as libc::c_int
            * 35468 as libc::c_int >> 16 as libc::c_int)
            - ((*in_0.offset(12 as libc::c_int as isize) as libc::c_int
                * 20091 as libc::c_int >> 16 as libc::c_int)
                + *in_0.offset(12 as libc::c_int as isize) as libc::c_int);
        let d: libc::c_int = (*in_0.offset(4 as libc::c_int as isize) as libc::c_int
            * 20091 as libc::c_int >> 16 as libc::c_int)
            + *in_0.offset(4 as libc::c_int as isize) as libc::c_int
            + (*in_0.offset(12 as libc::c_int as isize) as libc::c_int
                * 35468 as libc::c_int >> 16 as libc::c_int);
        *tmp.offset(0 as libc::c_int as isize) = a + d;
        *tmp.offset(1 as libc::c_int as isize) = b + c;
        *tmp.offset(2 as libc::c_int as isize) = b - c;
        *tmp.offset(3 as libc::c_int as isize) = a - d;
        tmp = tmp.offset(4 as libc::c_int as isize);
        in_0 = in_0.offset(1);
        in_0;
        i += 1;
        i;
    }
    tmp = C.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let dc: libc::c_int = *tmp.offset(0 as libc::c_int as isize) + 4 as libc::c_int;
        let a_0: libc::c_int = dc + *tmp.offset(8 as libc::c_int as isize);
        let b_0: libc::c_int = dc - *tmp.offset(8 as libc::c_int as isize);
        let c_0: libc::c_int = (*tmp.offset(4 as libc::c_int as isize)
            * 35468 as libc::c_int >> 16 as libc::c_int)
            - ((*tmp.offset(12 as libc::c_int as isize) * 20091 as libc::c_int
                >> 16 as libc::c_int) + *tmp.offset(12 as libc::c_int as isize));
        let d_0: libc::c_int = (*tmp.offset(4 as libc::c_int as isize)
            * 20091 as libc::c_int >> 16 as libc::c_int)
            + *tmp.offset(4 as libc::c_int as isize)
            + (*tmp.offset(12 as libc::c_int as isize) * 35468 as libc::c_int
                >> 16 as libc::c_int);
        *dst
            .offset(
                (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *dst
                .offset(
                    (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
                ) as libc::c_int + (a_0 + d_0 >> 3 as libc::c_int),
        );
        *dst
            .offset(
                (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *dst
                .offset(
                    (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
                ) as libc::c_int + (b_0 + c_0 >> 3 as libc::c_int),
        );
        *dst
            .offset(
                (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *dst
                .offset(
                    (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
                ) as libc::c_int + (b_0 - c_0 >> 3 as libc::c_int),
        );
        *dst
            .offset(
                (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *dst
                .offset(
                    (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
                ) as libc::c_int + (a_0 - d_0 >> 3 as libc::c_int),
        );
        tmp = tmp.offset(1);
        tmp;
        dst = dst.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn TransformAC3_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    let a: libc::c_int = *in_0.offset(0 as libc::c_int as isize) as libc::c_int
        + 4 as libc::c_int;
    let c4: libc::c_int = *in_0.offset(4 as libc::c_int as isize) as libc::c_int
        * 35468 as libc::c_int >> 16 as libc::c_int;
    let d4: libc::c_int = (*in_0.offset(4 as libc::c_int as isize) as libc::c_int
        * 20091 as libc::c_int >> 16 as libc::c_int)
        + *in_0.offset(4 as libc::c_int as isize) as libc::c_int;
    let c1: libc::c_int = *in_0.offset(1 as libc::c_int as isize) as libc::c_int
        * 35468 as libc::c_int >> 16 as libc::c_int;
    let d1: libc::c_int = (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
        * 20091 as libc::c_int >> 16 as libc::c_int)
        + *in_0.offset(1 as libc::c_int as isize) as libc::c_int;
    let DC: libc::c_int = a + d4;
    *dst
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC + d1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC + c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC - c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC - d1 >> 3 as libc::c_int),
    );
    let DC_0: libc::c_int = a + c4;
    *dst
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_0 + d1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_0 + c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_0 - c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_0 - d1 >> 3 as libc::c_int),
    );
    let DC_1: libc::c_int = a - c4;
    *dst
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_1 + d1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_1 + c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_1 - c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_1 - d1 >> 3 as libc::c_int),
    );
    let DC_2: libc::c_int = a - d4;
    *dst
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_2 + d1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_2 + c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_2 - c1 >> 3 as libc::c_int),
    );
    *dst
        .offset(
            (3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = clip_8b(
        *dst.offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
            as libc::c_int + (DC_2 - d1 >> 3 as libc::c_int),
    );
}
unsafe extern "C" fn TransformTwo_C(
    mut in_0: *const int16_t,
    mut dst: *mut uint8_t,
    mut do_two: libc::c_int,
) {
    TransformOne_C(in_0, dst);
    if do_two != 0 {
        TransformOne_C(
            in_0.offset(16 as libc::c_int as isize),
            dst.offset(4 as libc::c_int as isize),
        );
    }
}
unsafe extern "C" fn TransformUV_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    VP8Transform
        .expect(
            "non-null function pointer",
        )(
        in_0.offset((0 as libc::c_int * 16 as libc::c_int) as isize),
        dst,
        1 as libc::c_int,
    );
    VP8Transform
        .expect(
            "non-null function pointer",
        )(
        in_0.offset((2 as libc::c_int * 16 as libc::c_int) as isize),
        dst.offset((4 as libc::c_int * 32 as libc::c_int) as isize),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn TransformDC_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    let DC: libc::c_int = *in_0.offset(0 as libc::c_int as isize) as libc::c_int
        + 4 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *dst
                .offset(
                    (i + j * 32 as libc::c_int) as isize,
                ) = clip_8b(
                *dst.offset((i + j * 32 as libc::c_int) as isize) as libc::c_int
                    + (DC >> 3 as libc::c_int),
            );
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn TransformDCUV_C(mut in_0: *const int16_t, mut dst: *mut uint8_t) {
    if *in_0.offset((0 as libc::c_int * 16 as libc::c_int) as isize) != 0 {
        VP8TransformDC
            .expect(
                "non-null function pointer",
            )(in_0.offset((0 as libc::c_int * 16 as libc::c_int) as isize), dst);
    }
    if *in_0.offset((1 as libc::c_int * 16 as libc::c_int) as isize) != 0 {
        VP8TransformDC
            .expect(
                "non-null function pointer",
            )(
            in_0.offset((1 as libc::c_int * 16 as libc::c_int) as isize),
            dst.offset(4 as libc::c_int as isize),
        );
    }
    if *in_0.offset((2 as libc::c_int * 16 as libc::c_int) as isize) != 0 {
        VP8TransformDC
            .expect(
                "non-null function pointer",
            )(
            in_0.offset((2 as libc::c_int * 16 as libc::c_int) as isize),
            dst.offset((4 as libc::c_int * 32 as libc::c_int) as isize),
        );
    }
    if *in_0.offset((3 as libc::c_int * 16 as libc::c_int) as isize) != 0 {
        VP8TransformDC
            .expect(
                "non-null function pointer",
            )(
            in_0.offset((3 as libc::c_int * 16 as libc::c_int) as isize),
            dst
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize),
        );
    }
}
unsafe extern "C" fn TransformWHT_C(mut in_0: *const int16_t, mut out: *mut int16_t) {
    let mut tmp: [libc::c_int; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let a0: libc::c_int = *in_0.offset((0 as libc::c_int + i) as isize)
            as libc::c_int
            + *in_0.offset((12 as libc::c_int + i) as isize) as libc::c_int;
        let a1: libc::c_int = *in_0.offset((4 as libc::c_int + i) as isize)
            as libc::c_int
            + *in_0.offset((8 as libc::c_int + i) as isize) as libc::c_int;
        let a2: libc::c_int = *in_0.offset((4 as libc::c_int + i) as isize)
            as libc::c_int
            - *in_0.offset((8 as libc::c_int + i) as isize) as libc::c_int;
        let a3: libc::c_int = *in_0.offset((0 as libc::c_int + i) as isize)
            as libc::c_int
            - *in_0.offset((12 as libc::c_int + i) as isize) as libc::c_int;
        tmp[(0 as libc::c_int + i) as usize] = a0 + a1;
        tmp[(8 as libc::c_int + i) as usize] = a0 - a1;
        tmp[(4 as libc::c_int + i) as usize] = a3 + a2;
        tmp[(12 as libc::c_int + i) as usize] = a3 - a2;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let dc: libc::c_int = tmp[(0 as libc::c_int + i * 4 as libc::c_int) as usize]
            + 3 as libc::c_int;
        let a0_0: libc::c_int = dc
            + tmp[(3 as libc::c_int + i * 4 as libc::c_int) as usize];
        let a1_0: libc::c_int = tmp[(1 as libc::c_int + i * 4 as libc::c_int) as usize]
            + tmp[(2 as libc::c_int + i * 4 as libc::c_int) as usize];
        let a2_0: libc::c_int = tmp[(1 as libc::c_int + i * 4 as libc::c_int) as usize]
            - tmp[(2 as libc::c_int + i * 4 as libc::c_int) as usize];
        let a3_0: libc::c_int = dc
            - tmp[(3 as libc::c_int + i * 4 as libc::c_int) as usize];
        *out
            .offset(
                0 as libc::c_int as isize,
            ) = (a0_0 + a1_0 >> 3 as libc::c_int) as int16_t;
        *out
            .offset(
                16 as libc::c_int as isize,
            ) = (a3_0 + a2_0 >> 3 as libc::c_int) as int16_t;
        *out
            .offset(
                32 as libc::c_int as isize,
            ) = (a0_0 - a1_0 >> 3 as libc::c_int) as int16_t;
        *out
            .offset(
                48 as libc::c_int as isize,
            ) = (a3_0 - a2_0 >> 3 as libc::c_int) as int16_t;
        out = out.offset(64 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut VP8TransformWHT: Option::<
    unsafe extern "C" fn(*const int16_t, *mut int16_t) -> (),
> = None;
#[inline]
unsafe extern "C" fn TrueMotion(mut dst: *mut uint8_t, mut size: libc::c_int) {
    let mut top: *const uint8_t = dst.offset(-(32 as libc::c_int as isize));
    let clip0: *const uint8_t = VP8kclip1
        .offset(-(*top.offset(-(1 as libc::c_int) as isize) as libc::c_int as isize));
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < size {
        let clip: *const uint8_t = clip0
            .offset(*dst.offset(-(1 as libc::c_int) as isize) as libc::c_int as isize);
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < size {
            *dst.offset(x as isize) = *clip.offset(*top.offset(x as isize) as isize);
            x += 1;
            x;
        }
        dst = dst.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn TM4_C(mut dst: *mut uint8_t) {
    TrueMotion(dst, 4 as libc::c_int);
}
unsafe extern "C" fn TM8uv_C(mut dst: *mut uint8_t) {
    TrueMotion(dst, 8 as libc::c_int);
}
unsafe extern "C" fn TM16_C(mut dst: *mut uint8_t) {
    TrueMotion(dst, 16 as libc::c_int);
}
unsafe extern "C" fn VE16_C(mut dst: *mut uint8_t) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        memcpy(
            dst.offset((j * 32 as libc::c_int) as isize) as *mut libc::c_void,
            dst.offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        j += 1;
        j;
    }
}
unsafe extern "C" fn HE16_C(mut dst: *mut uint8_t) {
    let mut j: libc::c_int = 0;
    j = 16 as libc::c_int;
    while j > 0 as libc::c_int {
        memset(
            dst as *mut libc::c_void,
            *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int,
            16 as libc::c_int as libc::c_ulong,
        );
        dst = dst.offset(32 as libc::c_int as isize);
        j -= 1;
        j;
    }
}
#[inline]
unsafe extern "C" fn Put16(mut v: libc::c_int, mut dst: *mut uint8_t) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        memset(
            dst.offset((j * 32 as libc::c_int) as isize) as *mut libc::c_void,
            v,
            16 as libc::c_int as libc::c_ulong,
        );
        j += 1;
        j;
    }
}
unsafe extern "C" fn DC16_C(mut dst: *mut uint8_t) {
    let mut DC: libc::c_int = 16 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        DC
            += *dst.offset((-(1 as libc::c_int) + j * 32 as libc::c_int) as isize)
                as libc::c_int
                + *dst.offset((j - 32 as libc::c_int) as isize) as libc::c_int;
        j += 1;
        j;
    }
    Put16(DC >> 5 as libc::c_int, dst);
}
unsafe extern "C" fn DC16NoTop_C(mut dst: *mut uint8_t) {
    let mut DC: libc::c_int = 8 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        DC
            += *dst.offset((-(1 as libc::c_int) + j * 32 as libc::c_int) as isize)
                as libc::c_int;
        j += 1;
        j;
    }
    Put16(DC >> 4 as libc::c_int, dst);
}
unsafe extern "C" fn DC16NoLeft_C(mut dst: *mut uint8_t) {
    let mut DC: libc::c_int = 8 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        DC += *dst.offset((i - 32 as libc::c_int) as isize) as libc::c_int;
        i += 1;
        i;
    }
    Put16(DC >> 4 as libc::c_int, dst);
}
unsafe extern "C" fn DC16NoTopLeft_C(mut dst: *mut uint8_t) {
    Put16(0x80 as libc::c_int, dst);
}
#[no_mangle]
pub static mut VP8PredLuma16: [VP8PredFunc; 7] = [None; 7];
unsafe extern "C" fn VE4_C(mut dst: *mut uint8_t) {
    let mut top: *const uint8_t = dst.offset(-(32 as libc::c_int as isize));
    let vals: [uint8_t; 4] = [
        (*top.offset(-(1 as libc::c_int) as isize) as libc::c_int
            + 2 as libc::c_int * *top.offset(0 as libc::c_int as isize) as libc::c_int
            + *top.offset(1 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int
            >> 2 as libc::c_int) as uint8_t,
        (*top.offset(0 as libc::c_int as isize) as libc::c_int
            + 2 as libc::c_int * *top.offset(1 as libc::c_int as isize) as libc::c_int
            + *top.offset(2 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int
            >> 2 as libc::c_int) as uint8_t,
        (*top.offset(1 as libc::c_int as isize) as libc::c_int
            + 2 as libc::c_int * *top.offset(2 as libc::c_int as isize) as libc::c_int
            + *top.offset(3 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int
            >> 2 as libc::c_int) as uint8_t,
        (*top.offset(2 as libc::c_int as isize) as libc::c_int
            + 2 as libc::c_int * *top.offset(3 as libc::c_int as isize) as libc::c_int
            + *top.offset(4 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int
            >> 2 as libc::c_int) as uint8_t,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        memcpy(
            dst.offset((i * 32 as libc::c_int) as isize) as *mut libc::c_void,
            vals.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn HE4_C(mut dst: *mut uint8_t) {
    let A: libc::c_int = *dst.offset((-(1 as libc::c_int) - 32 as libc::c_int) as isize)
        as libc::c_int;
    let B: libc::c_int = *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int;
    let C: libc::c_int = *dst.offset((-(1 as libc::c_int) + 32 as libc::c_int) as isize)
        as libc::c_int;
    let D: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let E: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    WebPUint32ToMem(
        dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (A + 2 as libc::c_int * B + C + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
    WebPUint32ToMem(
        dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (B + 2 as libc::c_int * C + D + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
    WebPUint32ToMem(
        dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (C + 2 as libc::c_int * D + E + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
    WebPUint32ToMem(
        dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (D + 2 as libc::c_int * E + E + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
}
unsafe extern "C" fn DC4_C(mut dst: *mut uint8_t) {
    let mut dc: uint32_t = 4 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        dc = (dc as libc::c_uint)
            .wrapping_add(
                (*dst.offset((i - 32 as libc::c_int) as isize) as libc::c_int
                    + *dst.offset((-(1 as libc::c_int) + i * 32 as libc::c_int) as isize)
                        as libc::c_int) as libc::c_uint,
            ) as uint32_t as uint32_t;
        i += 1;
        i;
    }
    dc >>= 3 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        memset(
            dst.offset((i * 32 as libc::c_int) as isize) as *mut libc::c_void,
            dc as libc::c_int,
            4 as libc::c_int as libc::c_ulong,
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn RD4_C(mut dst: *mut uint8_t) {
    let I: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let J: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let K: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let L: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let X: libc::c_int = *dst.offset((-(1 as libc::c_int) - 32 as libc::c_int) as isize)
        as libc::c_int;
    let A: libc::c_int = *dst.offset((0 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let B: libc::c_int = *dst.offset((1 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let C: libc::c_int = *dst.offset((2 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let D: libc::c_int = *dst.offset((3 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    *dst
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (J + 2 as libc::c_int * K + L + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh0 = *dst
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh0 = (I + 2 as libc::c_int * J + K + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh0;
    let ref mut fresh1 = *dst
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh1 = (X + 2 as libc::c_int * I + J + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh2 = *dst
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh2 = *fresh1;
    *dst
        .offset(
            (2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh2;
    let ref mut fresh3 = *dst
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh3 = (A + 2 as libc::c_int * X + I + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh4 = *dst
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh4 = *fresh3;
    let ref mut fresh5 = *dst
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh5 = *fresh4;
    *dst
        .offset(
            (3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh5;
    let ref mut fresh6 = *dst
        .offset((1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh6 = (B + 2 as libc::c_int * A + X + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh7 = *dst
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh7 = *fresh6;
    *dst
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh7;
    let ref mut fresh8 = *dst
        .offset((2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh8 = (C + 2 as libc::c_int * B + A + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh8;
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (D + 2 as libc::c_int * C + B + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn LD4_C(mut dst: *mut uint8_t) {
    let A: libc::c_int = *dst.offset((0 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let B: libc::c_int = *dst.offset((1 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let C: libc::c_int = *dst.offset((2 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let D: libc::c_int = *dst.offset((3 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let E: libc::c_int = *dst.offset((4 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let F: libc::c_int = *dst.offset((5 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let G: libc::c_int = *dst.offset((6 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let H: libc::c_int = *dst.offset((7 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    *dst
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (A + 2 as libc::c_int * B + C + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh9 = *dst
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh9 = (B + 2 as libc::c_int * C + D + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh9;
    let ref mut fresh10 = *dst
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh10 = (C + 2 as libc::c_int * D + E + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh11 = *dst
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh11 = *fresh10;
    *dst
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh11;
    let ref mut fresh12 = *dst
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh12 = (D + 2 as libc::c_int * E + F + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh13 = *dst
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh13 = *fresh12;
    let ref mut fresh14 = *dst
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh14 = *fresh13;
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh14;
    let ref mut fresh15 = *dst
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh15 = (E + 2 as libc::c_int * F + G + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh16 = *dst
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh16 = *fresh15;
    *dst
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh16;
    let ref mut fresh17 = *dst
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh17 = (F + 2 as libc::c_int * G + H + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh17;
    *dst
        .offset(
            (3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (G + 2 as libc::c_int * H + H + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn VR4_C(mut dst: *mut uint8_t) {
    let I: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let J: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let K: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let X: libc::c_int = *dst.offset((-(1 as libc::c_int) - 32 as libc::c_int) as isize)
        as libc::c_int;
    let A: libc::c_int = *dst.offset((0 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let B: libc::c_int = *dst.offset((1 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let C: libc::c_int = *dst.offset((2 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let D: libc::c_int = *dst.offset((3 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let ref mut fresh18 = *dst
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh18 = (X + A + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh18;
    let ref mut fresh19 = *dst
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh19 = (A + B + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh19;
    let ref mut fresh20 = *dst
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh20 = (B + C + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh20;
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (C + D + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (K + 2 as libc::c_int * J + I + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (J + 2 as libc::c_int * I + X + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh21 = *dst
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh21 = (I + 2 as libc::c_int * X + A + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh21;
    let ref mut fresh22 = *dst
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh22 = (X + 2 as libc::c_int * A + B + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh22;
    let ref mut fresh23 = *dst
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh23 = (A + 2 as libc::c_int * B + C + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh23;
    *dst
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (B + 2 as libc::c_int * C + D + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn VL4_C(mut dst: *mut uint8_t) {
    let A: libc::c_int = *dst.offset((0 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let B: libc::c_int = *dst.offset((1 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let C: libc::c_int = *dst.offset((2 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let D: libc::c_int = *dst.offset((3 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let E: libc::c_int = *dst.offset((4 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let F: libc::c_int = *dst.offset((5 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let G: libc::c_int = *dst.offset((6 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let H: libc::c_int = *dst.offset((7 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    *dst
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (A + B + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    let ref mut fresh24 = *dst
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh24 = (B + C + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh24;
    let ref mut fresh25 = *dst
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh25 = (C + D + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh25;
    let ref mut fresh26 = *dst
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh26 = (D + E + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh26;
    *dst
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (A + 2 as libc::c_int * B + C + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh27 = *dst
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh27 = (B + 2 as libc::c_int * C + D + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh27;
    let ref mut fresh28 = *dst
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh28 = (C + 2 as libc::c_int * D + E + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh28;
    let ref mut fresh29 = *dst
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh29 = (D + 2 as libc::c_int * E + F + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh29;
    *dst
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (E + 2 as libc::c_int * F + G + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (F + 2 as libc::c_int * G + H + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn HU4_C(mut dst: *mut uint8_t) {
    let I: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let J: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let K: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let L: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    *dst
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (I + J + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    let ref mut fresh30 = *dst
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh30 = (J + K + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh30;
    let ref mut fresh31 = *dst
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh31 = (K + L + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh31;
    *dst
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (I + 2 as libc::c_int * J + K + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh32 = *dst
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh32 = (J + 2 as libc::c_int * K + L + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh32;
    let ref mut fresh33 = *dst
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh33 = (K + 2 as libc::c_int * L + L + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh33;
    let ref mut fresh34 = *dst
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh34 = L as uint8_t;
    let ref mut fresh35 = *dst
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh35 = *fresh34;
    let ref mut fresh36 = *dst
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh36 = *fresh35;
    let ref mut fresh37 = *dst
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh37 = *fresh36;
    let ref mut fresh38 = *dst
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh38 = *fresh37;
    *dst
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh38;
}
unsafe extern "C" fn HD4_C(mut dst: *mut uint8_t) {
    let I: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let J: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let K: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let L: libc::c_int = *dst
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let X: libc::c_int = *dst.offset((-(1 as libc::c_int) - 32 as libc::c_int) as isize)
        as libc::c_int;
    let A: libc::c_int = *dst.offset((0 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let B: libc::c_int = *dst.offset((1 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let C: libc::c_int = *dst.offset((2 as libc::c_int - 32 as libc::c_int) as isize)
        as libc::c_int;
    let ref mut fresh39 = *dst
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh39 = (I + X + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh39;
    let ref mut fresh40 = *dst
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh40 = (J + I + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh40;
    let ref mut fresh41 = *dst
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh41 = (K + J + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh41;
    *dst
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (L + K + 1 as libc::c_int >> 1 as libc::c_int) as uint8_t;
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (A + 2 as libc::c_int * B + C + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (X + 2 as libc::c_int * A + B + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh42 = *dst
        .offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh42 = (I + 2 as libc::c_int * X + A + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh42;
    let ref mut fresh43 = *dst
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh43 = (J + 2 as libc::c_int * I + X + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh43;
    let ref mut fresh44 = *dst
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh44 = (K + 2 as libc::c_int * J + I + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh44;
    *dst
        .offset(
            (1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (L + 2 as libc::c_int * K + J + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
}
#[no_mangle]
pub static mut VP8PredLuma4: [VP8PredFunc; 10] = [None; 10];
unsafe extern "C" fn VE8uv_C(mut dst: *mut uint8_t) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        memcpy(
            dst.offset((j * 32 as libc::c_int) as isize) as *mut libc::c_void,
            dst.offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        j += 1;
        j;
    }
}
unsafe extern "C" fn HE8uv_C(mut dst: *mut uint8_t) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        memset(
            dst as *mut libc::c_void,
            *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int,
            8 as libc::c_int as libc::c_ulong,
        );
        dst = dst.offset(32 as libc::c_int as isize);
        j += 1;
        j;
    }
}
#[inline]
unsafe extern "C" fn Put8x8uv(mut value: uint8_t, mut dst: *mut uint8_t) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        memset(
            dst.offset((j * 32 as libc::c_int) as isize) as *mut libc::c_void,
            value as libc::c_int,
            8 as libc::c_int as libc::c_ulong,
        );
        j += 1;
        j;
    }
}
unsafe extern "C" fn DC8uv_C(mut dst: *mut uint8_t) {
    let mut dc0: libc::c_int = 8 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        dc0
            += *dst.offset((i - 32 as libc::c_int) as isize) as libc::c_int
                + *dst.offset((-(1 as libc::c_int) + i * 32 as libc::c_int) as isize)
                    as libc::c_int;
        i += 1;
        i;
    }
    Put8x8uv((dc0 >> 4 as libc::c_int) as uint8_t, dst);
}
unsafe extern "C" fn DC8uvNoLeft_C(mut dst: *mut uint8_t) {
    let mut dc0: libc::c_int = 4 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        dc0 += *dst.offset((i - 32 as libc::c_int) as isize) as libc::c_int;
        i += 1;
        i;
    }
    Put8x8uv((dc0 >> 3 as libc::c_int) as uint8_t, dst);
}
unsafe extern "C" fn DC8uvNoTop_C(mut dst: *mut uint8_t) {
    let mut dc0: libc::c_int = 4 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        dc0
            += *dst.offset((-(1 as libc::c_int) + i * 32 as libc::c_int) as isize)
                as libc::c_int;
        i += 1;
        i;
    }
    Put8x8uv((dc0 >> 3 as libc::c_int) as uint8_t, dst);
}
unsafe extern "C" fn DC8uvNoTopLeft_C(mut dst: *mut uint8_t) {
    Put8x8uv(0x80 as libc::c_int as uint8_t, dst);
}
#[no_mangle]
pub static mut VP8PredChroma8: [VP8PredFunc; 7] = [None; 7];
#[inline]
unsafe extern "C" fn DoFilter2_C(mut p: *mut uint8_t, mut step: libc::c_int) {
    let p1: libc::c_int = *p.offset((-(2 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p0: libc::c_int = *p.offset(-step as isize) as libc::c_int;
    let q0: libc::c_int = *p.offset(0 as libc::c_int as isize) as libc::c_int;
    let q1: libc::c_int = *p.offset(step as isize) as libc::c_int;
    let a: libc::c_int = 3 as libc::c_int * (q0 - p0)
        + *VP8ksclip1.offset((p1 - q1) as isize) as libc::c_int;
    let a1: libc::c_int = *VP8ksclip2
        .offset((a + 4 as libc::c_int >> 3 as libc::c_int) as isize) as libc::c_int;
    let a2: libc::c_int = *VP8ksclip2
        .offset((a + 3 as libc::c_int >> 3 as libc::c_int) as isize) as libc::c_int;
    *p.offset(-step as isize) = *VP8kclip1.offset((p0 + a2) as isize);
    *p.offset(0 as libc::c_int as isize) = *VP8kclip1.offset((q0 - a1) as isize);
}
#[inline]
unsafe extern "C" fn DoFilter4_C(mut p: *mut uint8_t, mut step: libc::c_int) {
    let p1: libc::c_int = *p.offset((-(2 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p0: libc::c_int = *p.offset(-step as isize) as libc::c_int;
    let q0: libc::c_int = *p.offset(0 as libc::c_int as isize) as libc::c_int;
    let q1: libc::c_int = *p.offset(step as isize) as libc::c_int;
    let a: libc::c_int = 3 as libc::c_int * (q0 - p0);
    let a1: libc::c_int = *VP8ksclip2
        .offset((a + 4 as libc::c_int >> 3 as libc::c_int) as isize) as libc::c_int;
    let a2: libc::c_int = *VP8ksclip2
        .offset((a + 3 as libc::c_int >> 3 as libc::c_int) as isize) as libc::c_int;
    let a3: libc::c_int = a1 + 1 as libc::c_int >> 1 as libc::c_int;
    *p
        .offset(
            (-(2 as libc::c_int) * step) as isize,
        ) = *VP8kclip1.offset((p1 + a3) as isize);
    *p.offset(-step as isize) = *VP8kclip1.offset((p0 + a2) as isize);
    *p.offset(0 as libc::c_int as isize) = *VP8kclip1.offset((q0 - a1) as isize);
    *p.offset(step as isize) = *VP8kclip1.offset((q1 - a3) as isize);
}
#[inline]
unsafe extern "C" fn DoFilter6_C(mut p: *mut uint8_t, mut step: libc::c_int) {
    let p2: libc::c_int = *p.offset((-(3 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p1: libc::c_int = *p.offset((-(2 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p0: libc::c_int = *p.offset(-step as isize) as libc::c_int;
    let q0: libc::c_int = *p.offset(0 as libc::c_int as isize) as libc::c_int;
    let q1: libc::c_int = *p.offset(step as isize) as libc::c_int;
    let q2: libc::c_int = *p.offset((2 as libc::c_int * step) as isize) as libc::c_int;
    let a: libc::c_int = *VP8ksclip1
        .offset(
            (3 as libc::c_int * (q0 - p0)
                + *VP8ksclip1.offset((p1 - q1) as isize) as libc::c_int) as isize,
        ) as libc::c_int;
    let a1: libc::c_int = 27 as libc::c_int * a + 63 as libc::c_int >> 7 as libc::c_int;
    let a2: libc::c_int = 18 as libc::c_int * a + 63 as libc::c_int >> 7 as libc::c_int;
    let a3: libc::c_int = 9 as libc::c_int * a + 63 as libc::c_int >> 7 as libc::c_int;
    *p
        .offset(
            (-(3 as libc::c_int) * step) as isize,
        ) = *VP8kclip1.offset((p2 + a3) as isize);
    *p
        .offset(
            (-(2 as libc::c_int) * step) as isize,
        ) = *VP8kclip1.offset((p1 + a2) as isize);
    *p.offset(-step as isize) = *VP8kclip1.offset((p0 + a1) as isize);
    *p.offset(0 as libc::c_int as isize) = *VP8kclip1.offset((q0 - a1) as isize);
    *p.offset(step as isize) = *VP8kclip1.offset((q1 - a2) as isize);
    *p
        .offset(
            (2 as libc::c_int * step) as isize,
        ) = *VP8kclip1.offset((q2 - a3) as isize);
}
#[inline]
unsafe extern "C" fn Hev(
    mut p: *const uint8_t,
    mut step: libc::c_int,
    mut thresh: libc::c_int,
) -> libc::c_int {
    let p1: libc::c_int = *p.offset((-(2 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p0: libc::c_int = *p.offset(-step as isize) as libc::c_int;
    let q0: libc::c_int = *p.offset(0 as libc::c_int as isize) as libc::c_int;
    let q1: libc::c_int = *p.offset(step as isize) as libc::c_int;
    return (*VP8kabs0.offset((p1 - p0) as isize) as libc::c_int > thresh
        || *VP8kabs0.offset((q1 - q0) as isize) as libc::c_int > thresh) as libc::c_int;
}
#[inline]
unsafe extern "C" fn NeedsFilter_C(
    mut p: *const uint8_t,
    mut step: libc::c_int,
    mut t: libc::c_int,
) -> libc::c_int {
    let p1: libc::c_int = *p.offset((-(2 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p0: libc::c_int = *p.offset(-step as isize) as libc::c_int;
    let q0: libc::c_int = *p.offset(0 as libc::c_int as isize) as libc::c_int;
    let q1: libc::c_int = *p.offset(step as isize) as libc::c_int;
    return (4 as libc::c_int * *VP8kabs0.offset((p0 - q0) as isize) as libc::c_int
        + *VP8kabs0.offset((p1 - q1) as isize) as libc::c_int <= t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn NeedsFilter2_C(
    mut p: *const uint8_t,
    mut step: libc::c_int,
    mut t: libc::c_int,
    mut it: libc::c_int,
) -> libc::c_int {
    let p3: libc::c_int = *p.offset((-(4 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p2: libc::c_int = *p.offset((-(3 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p1: libc::c_int = *p.offset((-(2 as libc::c_int) * step) as isize)
        as libc::c_int;
    let p0: libc::c_int = *p.offset(-step as isize) as libc::c_int;
    let q0: libc::c_int = *p.offset(0 as libc::c_int as isize) as libc::c_int;
    let q1: libc::c_int = *p.offset(step as isize) as libc::c_int;
    let q2: libc::c_int = *p.offset((2 as libc::c_int * step) as isize) as libc::c_int;
    let q3: libc::c_int = *p.offset((3 as libc::c_int * step) as isize) as libc::c_int;
    if 4 as libc::c_int * *VP8kabs0.offset((p0 - q0) as isize) as libc::c_int
        + *VP8kabs0.offset((p1 - q1) as isize) as libc::c_int > t
    {
        return 0 as libc::c_int;
    }
    return (*VP8kabs0.offset((p3 - p2) as isize) as libc::c_int <= it
        && *VP8kabs0.offset((p2 - p1) as isize) as libc::c_int <= it
        && *VP8kabs0.offset((p1 - p0) as isize) as libc::c_int <= it
        && *VP8kabs0.offset((q3 - q2) as isize) as libc::c_int <= it
        && *VP8kabs0.offset((q2 - q1) as isize) as libc::c_int <= it
        && *VP8kabs0.offset((q1 - q0) as isize) as libc::c_int <= it) as libc::c_int;
}
unsafe extern "C" fn SimpleVFilter16_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let thresh2: libc::c_int = 2 as libc::c_int * thresh + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if NeedsFilter_C(p.offset(i as isize), stride, thresh2) != 0 {
            DoFilter2_C(p.offset(i as isize), stride);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn SimpleHFilter16_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let thresh2: libc::c_int = 2 as libc::c_int * thresh + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if NeedsFilter_C(p.offset((i * stride) as isize), 1 as libc::c_int, thresh2) != 0
        {
            DoFilter2_C(p.offset((i * stride) as isize), 1 as libc::c_int);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn SimpleVFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 3 as libc::c_int;
    while k > 0 as libc::c_int {
        p = p.offset((4 as libc::c_int * stride) as isize);
        SimpleVFilter16_C(p, stride, thresh);
        k -= 1;
        k;
    }
}
unsafe extern "C" fn SimpleHFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 3 as libc::c_int;
    while k > 0 as libc::c_int {
        p = p.offset(4 as libc::c_int as isize);
        SimpleHFilter16_C(p, stride, thresh);
        k -= 1;
        k;
    }
}
#[inline]
unsafe extern "C" fn FilterLoop26_C(
    mut p: *mut uint8_t,
    mut hstride: libc::c_int,
    mut vstride: libc::c_int,
    mut size: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    let thresh2: libc::c_int = 2 as libc::c_int * thresh + 1 as libc::c_int;
    loop {
        let fresh45 = size;
        size = size - 1;
        if !(fresh45 > 0 as libc::c_int) {
            break;
        }
        if NeedsFilter2_C(p, hstride, thresh2, ithresh) != 0 {
            if Hev(p, hstride, hev_thresh) != 0 {
                DoFilter2_C(p, hstride);
            } else {
                DoFilter6_C(p, hstride);
            }
        }
        p = p.offset(vstride as isize);
    };
}
#[inline]
unsafe extern "C" fn FilterLoop24_C(
    mut p: *mut uint8_t,
    mut hstride: libc::c_int,
    mut vstride: libc::c_int,
    mut size: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    let thresh2: libc::c_int = 2 as libc::c_int * thresh + 1 as libc::c_int;
    loop {
        let fresh46 = size;
        size = size - 1;
        if !(fresh46 > 0 as libc::c_int) {
            break;
        }
        if NeedsFilter2_C(p, hstride, thresh2, ithresh) != 0 {
            if Hev(p, hstride, hev_thresh) != 0 {
                DoFilter2_C(p, hstride);
            } else {
                DoFilter4_C(p, hstride);
            }
        }
        p = p.offset(vstride as isize);
    };
}
unsafe extern "C" fn VFilter16_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    FilterLoop26_C(
        p,
        stride,
        1 as libc::c_int,
        16 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn HFilter16_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    FilterLoop26_C(
        p,
        1 as libc::c_int,
        stride,
        16 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn VFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 3 as libc::c_int;
    while k > 0 as libc::c_int {
        p = p.offset((4 as libc::c_int * stride) as isize);
        FilterLoop24_C(
            p,
            stride,
            1 as libc::c_int,
            16 as libc::c_int,
            thresh,
            ithresh,
            hev_thresh,
        );
        k -= 1;
        k;
    }
}
unsafe extern "C" fn HFilter16i_C(
    mut p: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 3 as libc::c_int;
    while k > 0 as libc::c_int {
        p = p.offset(4 as libc::c_int as isize);
        FilterLoop24_C(
            p,
            1 as libc::c_int,
            stride,
            16 as libc::c_int,
            thresh,
            ithresh,
            hev_thresh,
        );
        k -= 1;
        k;
    }
}
unsafe extern "C" fn VFilter8_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    FilterLoop26_C(
        u,
        stride,
        1 as libc::c_int,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop26_C(
        v,
        stride,
        1 as libc::c_int,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn HFilter8_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    FilterLoop26_C(
        u,
        1 as libc::c_int,
        stride,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop26_C(
        v,
        1 as libc::c_int,
        stride,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn VFilter8i_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    FilterLoop24_C(
        u.offset((4 as libc::c_int * stride) as isize),
        stride,
        1 as libc::c_int,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop24_C(
        v.offset((4 as libc::c_int * stride) as isize),
        stride,
        1 as libc::c_int,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn HFilter8i_C(
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut stride: libc::c_int,
    mut thresh: libc::c_int,
    mut ithresh: libc::c_int,
    mut hev_thresh: libc::c_int,
) {
    FilterLoop24_C(
        u.offset(4 as libc::c_int as isize),
        1 as libc::c_int,
        stride,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
    FilterLoop24_C(
        v.offset(4 as libc::c_int as isize),
        1 as libc::c_int,
        stride,
        8 as libc::c_int,
        thresh,
        ithresh,
        hev_thresh,
    );
}
unsafe extern "C" fn DitherCombine8x8_C(
    mut dither: *const uint8_t,
    mut dst: *mut uint8_t,
    mut dst_stride: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let delta0: libc::c_int = *dither.offset(i as isize) as libc::c_int
                - ((1 as libc::c_int) << 7 as libc::c_int);
            let delta1: libc::c_int = delta0
                + ((1 as libc::c_int) << 4 as libc::c_int - 1 as libc::c_int)
                >> 4 as libc::c_int;
            *dst
                .offset(
                    i as isize,
                ) = clip_8b(*dst.offset(i as isize) as libc::c_int + delta1);
            i += 1;
            i;
        }
        dst = dst.offset(dst_stride as isize);
        dither = dither.offset(8 as libc::c_int as isize);
        j += 1;
        j;
    }
}
#[no_mangle]
pub static mut VP8Transform: VP8DecIdct2 = None;
#[no_mangle]
pub static mut VP8TransformAC3: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8TransformUV: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8TransformDC: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8TransformDCUV: VP8DecIdct = None;
#[no_mangle]
pub static mut VP8VFilter16: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter16: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8VFilter8: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter8: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8VFilter16i: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter16i: VP8LumaFilterFunc = None;
#[no_mangle]
pub static mut VP8VFilter8i: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8HFilter8i: VP8ChromaFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleVFilter16: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleHFilter16: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleVFilter16i: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8SimpleHFilter16i: VP8SimpleFilterFunc = None;
#[no_mangle]
pub static mut VP8DitherCombine8x8: Option::<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn VP8DspInit() {
    static mut VP8DspInit_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut VP8DspInit_body_lock) != 0) {
        VP8DspInit_body();
        pthread_mutex_unlock(&mut VP8DspInit_body_lock);
    }
}
unsafe extern "C" fn VP8DspInit_body() {
    VP8InitClipTables();
    VP8TransformWHT = Some(
        TransformWHT_C as unsafe extern "C" fn(*const int16_t, *mut int16_t) -> (),
    );
    VP8Transform = Some(
        TransformTwo_C
            as unsafe extern "C" fn(*const int16_t, *mut uint8_t, libc::c_int) -> (),
    );
    VP8TransformDC = Some(
        TransformDC_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> (),
    );
    VP8TransformAC3 = Some(
        TransformAC3_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> (),
    );
    VP8TransformUV = Some(
        TransformUV_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> (),
    );
    VP8TransformDCUV = Some(
        TransformDCUV_C as unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> (),
    );
    VP8VFilter16 = Some(
        VFilter16_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8VFilter16i = Some(
        VFilter16i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8HFilter16 = Some(
        HFilter16_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8VFilter8 = Some(
        VFilter8_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8VFilter8i = Some(
        VFilter8i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8SimpleVFilter16 = Some(
        SimpleVFilter16_C
            as unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int) -> (),
    );
    VP8SimpleHFilter16 = Some(
        SimpleHFilter16_C
            as unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int) -> (),
    );
    VP8SimpleVFilter16i = Some(
        SimpleVFilter16i_C
            as unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int) -> (),
    );
    VP8SimpleHFilter16i = Some(
        SimpleHFilter16i_C
            as unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int) -> (),
    );
    VP8HFilter16i = Some(
        HFilter16i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8HFilter8 = Some(
        HFilter8_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8HFilter8i = Some(
        HFilter8i_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8PredLuma4[0 as libc::c_int
        as usize] = Some(DC4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[1 as libc::c_int
        as usize] = Some(TM4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[2 as libc::c_int
        as usize] = Some(VE4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[4 as libc::c_int
        as usize] = Some(RD4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[6 as libc::c_int
        as usize] = Some(LD4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[3 as libc::c_int
        as usize] = Some(HE4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[5 as libc::c_int
        as usize] = Some(VR4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[7 as libc::c_int
        as usize] = Some(VL4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[8 as libc::c_int
        as usize] = Some(HD4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma4[9 as libc::c_int
        as usize] = Some(HU4_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma16[0 as libc::c_int
        as usize] = Some(DC16_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma16[1 as libc::c_int
        as usize] = Some(TM16_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma16[2 as libc::c_int
        as usize] = Some(VE16_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma16[3 as libc::c_int
        as usize] = Some(HE16_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma16[4 as libc::c_int
        as usize] = Some(DC16NoTop_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma16[5 as libc::c_int
        as usize] = Some(DC16NoLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredLuma16[6 as libc::c_int
        as usize] = Some(DC16NoTopLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredChroma8[0 as libc::c_int
        as usize] = Some(DC8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredChroma8[1 as libc::c_int
        as usize] = Some(TM8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredChroma8[2 as libc::c_int
        as usize] = Some(VE8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredChroma8[3 as libc::c_int
        as usize] = Some(HE8uv_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredChroma8[4 as libc::c_int
        as usize] = Some(DC8uvNoTop_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredChroma8[5 as libc::c_int
        as usize] = Some(DC8uvNoLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8PredChroma8[6 as libc::c_int
        as usize] = Some(DC8uvNoTopLeft_C as unsafe extern "C" fn(*mut uint8_t) -> ());
    VP8DitherCombine8x8 = Some(
        DitherCombine8x8_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
    );
}
