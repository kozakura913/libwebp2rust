use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn VP8DspInit();
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
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type VP8Idct = Option::<
    unsafe extern "C" fn(*const uint8_t, *const int16_t, *mut uint8_t, libc::c_int) -> (),
>;
pub type VP8Fdct = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut int16_t) -> (),
>;
pub type VP8WHT = Option::<unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ()>;
pub type VP8IntraPreds = Option::<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, *const uint8_t) -> (),
>;
pub type VP8Intra4Preds = Option::<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t) -> (),
>;
pub type VP8Metric = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> libc::c_int,
>;
pub type VP8WMetric = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *const uint16_t) -> libc::c_int,
>;
pub type VP8MeanMetric = Option::<
    unsafe extern "C" fn(*const uint8_t, *mut uint32_t) -> (),
>;
pub type VP8BlockCopy = Option::<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Matrix {
    pub q_: [uint16_t; 16],
    pub iq_: [uint16_t; 16],
    pub bias_: [uint32_t; 16],
    pub zthresh_: [uint32_t; 16],
    pub sharpen_: [uint16_t; 16],
}
pub type VP8QuantizeBlock = Option::<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> libc::c_int,
>;
pub type VP8Quantize2Blocks = Option::<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> libc::c_int,
>;
pub type VP8QuantizeBlockWHT = Option::<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Histogram {
    pub max_value: libc::c_int,
    pub last_non_zero: libc::c_int,
}
pub type VP8CHisto = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        *mut VP8Histogram,
    ) -> (),
>;
pub const MAX_LEVEL: C2RustUnnamed_0 = 2047;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MAX_VARIABLE_LEVEL: C2RustUnnamed_0 = 67;
pub const MAX_LF_LEVELS: C2RustUnnamed_0 = 64;
#[inline]
unsafe extern "C" fn WebPUint32ToMem(ptr: *mut uint8_t, mut val: uint32_t) {
    memcpy(
        ptr as *mut libc::c_void,
        &mut val as *mut uint32_t as *const libc::c_void,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn QUANTDIV(
    mut n: uint32_t,
    mut iQ: uint32_t,
    mut B: uint32_t,
) -> libc::c_int {
    return (n.wrapping_mul(iQ).wrapping_add(B) >> 17 as libc::c_int) as libc::c_int;
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
#[inline]
unsafe extern "C" fn clip_max(mut v: libc::c_int, mut max: libc::c_int) -> libc::c_int {
    return if v > max { max } else { v };
}
#[no_mangle]
pub static mut VP8DspScan: [libc::c_int; 24] = [
    0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    8 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    12 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
    4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
    8 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
    12 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
    0 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int,
    4 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int,
    8 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int,
    12 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int,
    0 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int,
    4 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int,
    8 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int,
    12 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int,
    0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
    4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
    8 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    12 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int,
    8 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
    12 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn VP8SetHistogramData(
    mut distribution: *const libc::c_int,
    histo: *mut VP8Histogram,
) {
    let mut max_value: libc::c_int = 0 as libc::c_int;
    let mut last_non_zero: libc::c_int = 1 as libc::c_int;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k <= 31 as libc::c_int {
        let value: libc::c_int = *distribution.offset(k as isize);
        if value > 0 as libc::c_int {
            if value > max_value {
                max_value = value;
            }
            last_non_zero = k;
        }
        k += 1;
        k;
    }
    (*histo).max_value = max_value;
    (*histo).last_non_zero = last_non_zero;
}
unsafe extern "C" fn CollectHistogram_C(
    mut ref_0: *const uint8_t,
    mut pred: *const uint8_t,
    mut start_block: libc::c_int,
    mut end_block: libc::c_int,
    histo: *mut VP8Histogram,
) {
    let mut j: libc::c_int = 0;
    let mut distribution: [libc::c_int; 32] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    j = start_block;
    while j < end_block {
        let mut k: libc::c_int = 0;
        let mut out: [int16_t; 16] = [0; 16];
        VP8FTransform
            .expect(
                "non-null function pointer",
            )(
            ref_0.offset(VP8DspScan[j as usize] as isize),
            pred.offset(VP8DspScan[j as usize] as isize),
            out.as_mut_ptr(),
        );
        k = 0 as libc::c_int;
        while k < 16 as libc::c_int {
            let v: libc::c_int = abs(out[k as usize] as libc::c_int) >> 3 as libc::c_int;
            let clipped_value: libc::c_int = clip_max(v, 31 as libc::c_int);
            distribution[clipped_value as usize] += 1;
            distribution[clipped_value as usize];
            k += 1;
            k;
        }
        j += 1;
        j;
    }
    VP8SetHistogramData(distribution.as_mut_ptr() as *const libc::c_int, histo);
}
static mut clip1: [uint8_t; 766] = [0; 766];
static mut tables_ok: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn InitTables() {
    if tables_ok == 0 {
        let mut i: libc::c_int = 0;
        i = -(255 as libc::c_int);
        while i <= 255 as libc::c_int + 255 as libc::c_int {
            clip1[(255 as libc::c_int + i) as usize] = clip_8b(i);
            i += 1;
            i;
        }
        ::core::ptr::write_volatile(
            &mut tables_ok as *mut libc::c_int,
            1 as libc::c_int,
        );
    }
}
#[inline]
unsafe extern "C" fn ITransformOne(
    mut ref_0: *const uint8_t,
    mut in_0: *const int16_t,
    mut dst: *mut uint8_t,
) {
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
                (0 as libc::c_int + i * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *ref_0.offset((0 as libc::c_int + i * 32 as libc::c_int) as isize)
                as libc::c_int + (a_0 + d_0 >> 3 as libc::c_int),
        );
        *dst
            .offset(
                (1 as libc::c_int + i * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *ref_0.offset((1 as libc::c_int + i * 32 as libc::c_int) as isize)
                as libc::c_int + (b_0 + c_0 >> 3 as libc::c_int),
        );
        *dst
            .offset(
                (2 as libc::c_int + i * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *ref_0.offset((2 as libc::c_int + i * 32 as libc::c_int) as isize)
                as libc::c_int + (b_0 - c_0 >> 3 as libc::c_int),
        );
        *dst
            .offset(
                (3 as libc::c_int + i * 32 as libc::c_int) as isize,
            ) = clip_8b(
            *ref_0.offset((3 as libc::c_int + i * 32 as libc::c_int) as isize)
                as libc::c_int + (a_0 - d_0 >> 3 as libc::c_int),
        );
        tmp = tmp.offset(1);
        tmp;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ITransform_C(
    mut ref_0: *const uint8_t,
    mut in_0: *const int16_t,
    mut dst: *mut uint8_t,
    mut do_two: libc::c_int,
) {
    ITransformOne(ref_0, in_0, dst);
    if do_two != 0 {
        ITransformOne(
            ref_0.offset(4 as libc::c_int as isize),
            in_0.offset(16 as libc::c_int as isize),
            dst.offset(4 as libc::c_int as isize),
        );
    }
}
unsafe extern "C" fn FTransform_C(
    mut src: *const uint8_t,
    mut ref_0: *const uint8_t,
    mut out: *mut int16_t,
) {
    let mut i: libc::c_int = 0;
    let mut tmp: [libc::c_int; 16] = [0; 16];
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let d0: libc::c_int = *src.offset(0 as libc::c_int as isize) as libc::c_int
            - *ref_0.offset(0 as libc::c_int as isize) as libc::c_int;
        let d1: libc::c_int = *src.offset(1 as libc::c_int as isize) as libc::c_int
            - *ref_0.offset(1 as libc::c_int as isize) as libc::c_int;
        let d2: libc::c_int = *src.offset(2 as libc::c_int as isize) as libc::c_int
            - *ref_0.offset(2 as libc::c_int as isize) as libc::c_int;
        let d3: libc::c_int = *src.offset(3 as libc::c_int as isize) as libc::c_int
            - *ref_0.offset(3 as libc::c_int as isize) as libc::c_int;
        let a0: libc::c_int = d0 + d3;
        let a1: libc::c_int = d1 + d2;
        let a2: libc::c_int = d1 - d2;
        let a3: libc::c_int = d0 - d3;
        tmp[(0 as libc::c_int + i * 4 as libc::c_int)
            as usize] = (a0 + a1) * 8 as libc::c_int;
        tmp[(1 as libc::c_int + i * 4 as libc::c_int)
            as usize] = a2 * 2217 as libc::c_int + a3 * 5352 as libc::c_int
            + 1812 as libc::c_int >> 9 as libc::c_int;
        tmp[(2 as libc::c_int + i * 4 as libc::c_int)
            as usize] = (a0 - a1) * 8 as libc::c_int;
        tmp[(3 as libc::c_int + i * 4 as libc::c_int)
            as usize] = a3 * 2217 as libc::c_int - a2 * 5352 as libc::c_int
            + 937 as libc::c_int >> 9 as libc::c_int;
        i += 1;
        i;
        src = src.offset(32 as libc::c_int as isize);
        ref_0 = ref_0.offset(32 as libc::c_int as isize);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let a0_0: libc::c_int = tmp[(0 as libc::c_int + i) as usize]
            + tmp[(12 as libc::c_int + i) as usize];
        let a1_0: libc::c_int = tmp[(4 as libc::c_int + i) as usize]
            + tmp[(8 as libc::c_int + i) as usize];
        let a2_0: libc::c_int = tmp[(4 as libc::c_int + i) as usize]
            - tmp[(8 as libc::c_int + i) as usize];
        let a3_0: libc::c_int = tmp[(0 as libc::c_int + i) as usize]
            - tmp[(12 as libc::c_int + i) as usize];
        *out
            .offset(
                (0 as libc::c_int + i) as isize,
            ) = (a0_0 + a1_0 + 7 as libc::c_int >> 4 as libc::c_int) as int16_t;
        *out
            .offset(
                (4 as libc::c_int + i) as isize,
            ) = ((a2_0 * 2217 as libc::c_int + a3_0 * 5352 as libc::c_int
            + 12000 as libc::c_int >> 16 as libc::c_int)
            + (a3_0 != 0 as libc::c_int) as libc::c_int) as int16_t;
        *out
            .offset(
                (8 as libc::c_int + i) as isize,
            ) = (a0_0 - a1_0 + 7 as libc::c_int >> 4 as libc::c_int) as int16_t;
        *out
            .offset(
                (12 as libc::c_int + i) as isize,
            ) = (a3_0 * 2217 as libc::c_int - a2_0 * 5352 as libc::c_int
            + 51000 as libc::c_int >> 16 as libc::c_int) as int16_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn FTransform2_C(
    mut src: *const uint8_t,
    mut ref_0: *const uint8_t,
    mut out: *mut int16_t,
) {
    VP8FTransform.expect("non-null function pointer")(src, ref_0, out);
    VP8FTransform
        .expect(
            "non-null function pointer",
        )(
        src.offset(4 as libc::c_int as isize),
        ref_0.offset(4 as libc::c_int as isize),
        out.offset(16 as libc::c_int as isize),
    );
}
unsafe extern "C" fn FTransformWHT_C(mut in_0: *const int16_t, mut out: *mut int16_t) {
    let mut tmp: [int32_t; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let a0: libc::c_int = *in_0
            .offset((0 as libc::c_int * 16 as libc::c_int) as isize) as libc::c_int
            + *in_0.offset((2 as libc::c_int * 16 as libc::c_int) as isize)
                as libc::c_int;
        let a1: libc::c_int = *in_0
            .offset((1 as libc::c_int * 16 as libc::c_int) as isize) as libc::c_int
            + *in_0.offset((3 as libc::c_int * 16 as libc::c_int) as isize)
                as libc::c_int;
        let a2: libc::c_int = *in_0
            .offset((1 as libc::c_int * 16 as libc::c_int) as isize) as libc::c_int
            - *in_0.offset((3 as libc::c_int * 16 as libc::c_int) as isize)
                as libc::c_int;
        let a3: libc::c_int = *in_0
            .offset((0 as libc::c_int * 16 as libc::c_int) as isize) as libc::c_int
            - *in_0.offset((2 as libc::c_int * 16 as libc::c_int) as isize)
                as libc::c_int;
        tmp[(0 as libc::c_int + i * 4 as libc::c_int) as usize] = a0 + a1;
        tmp[(1 as libc::c_int + i * 4 as libc::c_int) as usize] = a3 + a2;
        tmp[(2 as libc::c_int + i * 4 as libc::c_int) as usize] = a3 - a2;
        tmp[(3 as libc::c_int + i * 4 as libc::c_int) as usize] = a0 - a1;
        i += 1;
        i;
        in_0 = in_0.offset(64 as libc::c_int as isize);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let a0_0: libc::c_int = tmp[(0 as libc::c_int + i) as usize]
            + tmp[(8 as libc::c_int + i) as usize];
        let a1_0: libc::c_int = tmp[(4 as libc::c_int + i) as usize]
            + tmp[(12 as libc::c_int + i) as usize];
        let a2_0: libc::c_int = tmp[(4 as libc::c_int + i) as usize]
            - tmp[(12 as libc::c_int + i) as usize];
        let a3_0: libc::c_int = tmp[(0 as libc::c_int + i) as usize]
            - tmp[(8 as libc::c_int + i) as usize];
        let b0: libc::c_int = a0_0 + a1_0;
        let b1: libc::c_int = a3_0 + a2_0;
        let b2: libc::c_int = a3_0 - a2_0;
        let b3: libc::c_int = a0_0 - a1_0;
        *out
            .offset(
                (0 as libc::c_int + i) as isize,
            ) = (b0 >> 1 as libc::c_int) as int16_t;
        *out
            .offset(
                (4 as libc::c_int + i) as isize,
            ) = (b1 >> 1 as libc::c_int) as int16_t;
        *out
            .offset(
                (8 as libc::c_int + i) as isize,
            ) = (b2 >> 1 as libc::c_int) as int16_t;
        *out
            .offset(
                (12 as libc::c_int + i) as isize,
            ) = (b3 >> 1 as libc::c_int) as int16_t;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn Fill(
    mut dst: *mut uint8_t,
    mut value: libc::c_int,
    mut size: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < size {
        memset(
            dst.offset((j * 32 as libc::c_int) as isize) as *mut libc::c_void,
            value,
            size as libc::c_ulong,
        );
        j += 1;
        j;
    }
}
#[inline]
unsafe extern "C" fn VerticalPred(
    mut dst: *mut uint8_t,
    mut top: *const uint8_t,
    mut size: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    if !top.is_null() {
        j = 0 as libc::c_int;
        while j < size {
            memcpy(
                dst.offset((j * 32 as libc::c_int) as isize) as *mut libc::c_void,
                top as *const libc::c_void,
                size as libc::c_ulong,
            );
            j += 1;
            j;
        }
    } else {
        Fill(dst, 127 as libc::c_int, size);
    };
}
#[inline]
unsafe extern "C" fn HorizontalPred(
    mut dst: *mut uint8_t,
    mut left: *const uint8_t,
    mut size: libc::c_int,
) {
    if !left.is_null() {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < size {
            memset(
                dst.offset((j * 32 as libc::c_int) as isize) as *mut libc::c_void,
                *left.offset(j as isize) as libc::c_int,
                size as libc::c_ulong,
            );
            j += 1;
            j;
        }
    } else {
        Fill(dst, 129 as libc::c_int, size);
    };
}
#[inline]
unsafe extern "C" fn TrueMotion(
    mut dst: *mut uint8_t,
    mut left: *const uint8_t,
    mut top: *const uint8_t,
    mut size: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    if !left.is_null() {
        if !top.is_null() {
            let clip: *const uint8_t = clip1
                .as_mut_ptr()
                .offset(255 as libc::c_int as isize)
                .offset(
                    -(*left.offset(-(1 as libc::c_int) as isize) as libc::c_int as isize),
                );
            y = 0 as libc::c_int;
            while y < size {
                let clip_table: *const uint8_t = clip
                    .offset(*left.offset(y as isize) as libc::c_int as isize);
                let mut x: libc::c_int = 0;
                x = 0 as libc::c_int;
                while x < size {
                    *dst
                        .offset(
                            x as isize,
                        ) = *clip_table.offset(*top.offset(x as isize) as isize);
                    x += 1;
                    x;
                }
                dst = dst.offset(32 as libc::c_int as isize);
                y += 1;
                y;
            }
        } else {
            HorizontalPred(dst, left, size);
        }
    } else if !top.is_null() {
        VerticalPred(dst, top, size);
    } else {
        Fill(dst, 129 as libc::c_int, size);
    };
}
#[inline]
unsafe extern "C" fn DCMode(
    mut dst: *mut uint8_t,
    mut left: *const uint8_t,
    mut top: *const uint8_t,
    mut size: libc::c_int,
    mut round: libc::c_int,
    mut shift: libc::c_int,
) {
    let mut DC: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    if !top.is_null() {
        j = 0 as libc::c_int;
        while j < size {
            DC += *top.offset(j as isize) as libc::c_int;
            j += 1;
            j;
        }
        if !left.is_null() {
            j = 0 as libc::c_int;
            while j < size {
                DC += *left.offset(j as isize) as libc::c_int;
                j += 1;
                j;
            }
        } else {
            DC += DC;
        }
        DC = DC + round >> shift;
    } else if !left.is_null() {
        j = 0 as libc::c_int;
        while j < size {
            DC += *left.offset(j as isize) as libc::c_int;
            j += 1;
            j;
        }
        DC += DC;
        DC = DC + round >> shift;
    } else {
        DC = 0x80 as libc::c_int;
    }
    Fill(dst, DC, size);
}
unsafe extern "C" fn IntraChromaPreds_C(
    mut dst: *mut uint8_t,
    mut left: *const uint8_t,
    mut top: *const uint8_t,
) {
    DCMode(
        dst.offset((2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int) as isize),
        left,
        top,
        8 as libc::c_int,
        8 as libc::c_int,
        4 as libc::c_int,
    );
    VerticalPred(
        dst
            .offset(
                (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 8 as libc::c_int * 32 as libc::c_int) as isize,
            ),
        top,
        8 as libc::c_int,
    );
    HorizontalPred(
        dst
            .offset(
                (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 8 as libc::c_int * 32 as libc::c_int
                    + 1 as libc::c_int * 16 as libc::c_int) as isize,
            ),
        left,
        8 as libc::c_int,
    );
    TrueMotion(
        dst
            .offset(
                (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 1 as libc::c_int * 16 as libc::c_int) as isize,
            ),
        left,
        top,
        8 as libc::c_int,
    );
    dst = dst.offset(8 as libc::c_int as isize);
    if !top.is_null() {
        top = top.offset(8 as libc::c_int as isize);
    }
    if !left.is_null() {
        left = left.offset(16 as libc::c_int as isize);
    }
    DCMode(
        dst.offset((2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int) as isize),
        left,
        top,
        8 as libc::c_int,
        8 as libc::c_int,
        4 as libc::c_int,
    );
    VerticalPred(
        dst
            .offset(
                (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 8 as libc::c_int * 32 as libc::c_int) as isize,
            ),
        top,
        8 as libc::c_int,
    );
    HorizontalPred(
        dst
            .offset(
                (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 8 as libc::c_int * 32 as libc::c_int
                    + 1 as libc::c_int * 16 as libc::c_int) as isize,
            ),
        left,
        8 as libc::c_int,
    );
    TrueMotion(
        dst
            .offset(
                (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 1 as libc::c_int * 16 as libc::c_int) as isize,
            ),
        left,
        top,
        8 as libc::c_int,
    );
}
unsafe extern "C" fn Intra16Preds_C(
    mut dst: *mut uint8_t,
    mut left: *const uint8_t,
    mut top: *const uint8_t,
) {
    DCMode(
        dst.offset((0 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int) as isize),
        left,
        top,
        16 as libc::c_int,
        16 as libc::c_int,
        5 as libc::c_int,
    );
    VerticalPred(
        dst.offset((1 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int) as isize),
        top,
        16 as libc::c_int,
    );
    HorizontalPred(
        dst
            .offset(
                (1 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 16 as libc::c_int) as isize,
            ),
        left,
        16 as libc::c_int,
    );
    TrueMotion(
        dst
            .offset(
                (0 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 16 as libc::c_int) as isize,
            ),
        left,
        top,
        16 as libc::c_int,
    );
}
unsafe extern "C" fn VE4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
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
            4 as libc::c_int as libc::c_ulong,
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn HE4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let X: libc::c_int = *top.offset(-(1 as libc::c_int) as isize) as libc::c_int;
    let I: libc::c_int = *top.offset(-(2 as libc::c_int) as isize) as libc::c_int;
    let J: libc::c_int = *top.offset(-(3 as libc::c_int) as isize) as libc::c_int;
    let K: libc::c_int = *top.offset(-(4 as libc::c_int) as isize) as libc::c_int;
    let L: libc::c_int = *top.offset(-(5 as libc::c_int) as isize) as libc::c_int;
    WebPUint32ToMem(
        dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (X + 2 as libc::c_int * I + J + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
    WebPUint32ToMem(
        dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (I + 2 as libc::c_int * J + K + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
    WebPUint32ToMem(
        dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (J + 2 as libc::c_int * K + L + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
    WebPUint32ToMem(
        dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize),
        (0x1010101 as libc::c_uint)
            .wrapping_mul(
                (K + 2 as libc::c_int * L + L + 2 as libc::c_int >> 2 as libc::c_int)
                    as uint8_t as libc::c_uint,
            ),
    );
}
unsafe extern "C" fn DC4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let mut dc: uint32_t = 4 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        dc = (dc as libc::c_uint)
            .wrapping_add(
                (*top.offset(i as isize) as libc::c_int
                    + *top.offset((-(5 as libc::c_int) + i) as isize) as libc::c_int)
                    as libc::c_uint,
            ) as uint32_t as uint32_t;
        i += 1;
        i;
    }
    Fill(dst, (dc >> 3 as libc::c_int) as libc::c_int, 4 as libc::c_int);
}
unsafe extern "C" fn RD4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let X: libc::c_int = *top.offset(-(1 as libc::c_int) as isize) as libc::c_int;
    let I: libc::c_int = *top.offset(-(2 as libc::c_int) as isize) as libc::c_int;
    let J: libc::c_int = *top.offset(-(3 as libc::c_int) as isize) as libc::c_int;
    let K: libc::c_int = *top.offset(-(4 as libc::c_int) as isize) as libc::c_int;
    let L: libc::c_int = *top.offset(-(5 as libc::c_int) as isize) as libc::c_int;
    let A: libc::c_int = *top.offset(0 as libc::c_int as isize) as libc::c_int;
    let B: libc::c_int = *top.offset(1 as libc::c_int as isize) as libc::c_int;
    let C: libc::c_int = *top.offset(2 as libc::c_int as isize) as libc::c_int;
    let D: libc::c_int = *top.offset(3 as libc::c_int as isize) as libc::c_int;
    *dst
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (J + 2 as libc::c_int * K + L + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh0 = *dst
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh0 = (I + 2 as libc::c_int * J + K + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh0;
    let ref mut fresh1 = *dst
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh1 = (X + 2 as libc::c_int * I + J + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh2 = *dst
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh2 = *fresh1;
    *dst
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh2;
    let ref mut fresh3 = *dst
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh3 = (A + 2 as libc::c_int * X + I + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh4 = *dst
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh4 = *fresh3;
    let ref mut fresh5 = *dst
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh5 = *fresh4;
    *dst
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh5;
    let ref mut fresh6 = *dst
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh6 = (B + 2 as libc::c_int * A + X + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    let ref mut fresh7 = *dst
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh7 = *fresh6;
    *dst
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh7;
    let ref mut fresh8 = *dst
        .offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh8 = (C + 2 as libc::c_int * B + A + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
    *dst
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh8;
    *dst
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (D + 2 as libc::c_int * C + B + 2 as libc::c_int >> 2 as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn LD4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let A: libc::c_int = *top.offset(0 as libc::c_int as isize) as libc::c_int;
    let B: libc::c_int = *top.offset(1 as libc::c_int as isize) as libc::c_int;
    let C: libc::c_int = *top.offset(2 as libc::c_int as isize) as libc::c_int;
    let D: libc::c_int = *top.offset(3 as libc::c_int as isize) as libc::c_int;
    let E: libc::c_int = *top.offset(4 as libc::c_int as isize) as libc::c_int;
    let F: libc::c_int = *top.offset(5 as libc::c_int as isize) as libc::c_int;
    let G: libc::c_int = *top.offset(6 as libc::c_int as isize) as libc::c_int;
    let H: libc::c_int = *top.offset(7 as libc::c_int as isize) as libc::c_int;
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
unsafe extern "C" fn VR4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let X: libc::c_int = *top.offset(-(1 as libc::c_int) as isize) as libc::c_int;
    let I: libc::c_int = *top.offset(-(2 as libc::c_int) as isize) as libc::c_int;
    let J: libc::c_int = *top.offset(-(3 as libc::c_int) as isize) as libc::c_int;
    let K: libc::c_int = *top.offset(-(4 as libc::c_int) as isize) as libc::c_int;
    let A: libc::c_int = *top.offset(0 as libc::c_int as isize) as libc::c_int;
    let B: libc::c_int = *top.offset(1 as libc::c_int as isize) as libc::c_int;
    let C: libc::c_int = *top.offset(2 as libc::c_int as isize) as libc::c_int;
    let D: libc::c_int = *top.offset(3 as libc::c_int as isize) as libc::c_int;
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
unsafe extern "C" fn VL4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let A: libc::c_int = *top.offset(0 as libc::c_int as isize) as libc::c_int;
    let B: libc::c_int = *top.offset(1 as libc::c_int as isize) as libc::c_int;
    let C: libc::c_int = *top.offset(2 as libc::c_int as isize) as libc::c_int;
    let D: libc::c_int = *top.offset(3 as libc::c_int as isize) as libc::c_int;
    let E: libc::c_int = *top.offset(4 as libc::c_int as isize) as libc::c_int;
    let F: libc::c_int = *top.offset(5 as libc::c_int as isize) as libc::c_int;
    let G: libc::c_int = *top.offset(6 as libc::c_int as isize) as libc::c_int;
    let H: libc::c_int = *top.offset(7 as libc::c_int as isize) as libc::c_int;
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
unsafe extern "C" fn HU4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let I: libc::c_int = *top.offset(-(2 as libc::c_int) as isize) as libc::c_int;
    let J: libc::c_int = *top.offset(-(3 as libc::c_int) as isize) as libc::c_int;
    let K: libc::c_int = *top.offset(-(4 as libc::c_int) as isize) as libc::c_int;
    let L: libc::c_int = *top.offset(-(5 as libc::c_int) as isize) as libc::c_int;
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
unsafe extern "C" fn HD4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let X: libc::c_int = *top.offset(-(1 as libc::c_int) as isize) as libc::c_int;
    let I: libc::c_int = *top.offset(-(2 as libc::c_int) as isize) as libc::c_int;
    let J: libc::c_int = *top.offset(-(3 as libc::c_int) as isize) as libc::c_int;
    let K: libc::c_int = *top.offset(-(4 as libc::c_int) as isize) as libc::c_int;
    let L: libc::c_int = *top.offset(-(5 as libc::c_int) as isize) as libc::c_int;
    let A: libc::c_int = *top.offset(0 as libc::c_int as isize) as libc::c_int;
    let B: libc::c_int = *top.offset(1 as libc::c_int as isize) as libc::c_int;
    let C: libc::c_int = *top.offset(2 as libc::c_int as isize) as libc::c_int;
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
unsafe extern "C" fn TM4(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let clip: *const uint8_t = clip1
        .as_mut_ptr()
        .offset(255 as libc::c_int as isize)
        .offset(-(*top.offset(-(1 as libc::c_int) as isize) as libc::c_int as isize));
    y = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        let clip_table: *const uint8_t = clip
            .offset(
                *top.offset((-(2 as libc::c_int) - y) as isize) as libc::c_int as isize,
            );
        x = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            *dst
                .offset(
                    x as isize,
                ) = *clip_table.offset(*top.offset(x as isize) as isize);
            x += 1;
            x;
        }
        dst = dst.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn Intra4Preds_C(mut dst: *mut uint8_t, mut top: *const uint8_t) {
    DC4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int) as isize,
            ),
        top,
    );
    TM4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int + 4 as libc::c_int) as isize,
            ),
        top,
    );
    VE4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int + 8 as libc::c_int) as isize,
            ),
        top,
    );
    HE4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int + 12 as libc::c_int) as isize,
            ),
        top,
    );
    RD4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int + 16 as libc::c_int) as isize,
            ),
        top,
    );
    VR4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int + 20 as libc::c_int) as isize,
            ),
        top,
    );
    LD4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int + 24 as libc::c_int) as isize,
            ),
        top,
    );
    VL4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 0 as libc::c_int + 28 as libc::c_int) as isize,
            ),
        top,
    );
    HD4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 4 as libc::c_int * 32 as libc::c_int) as isize,
            ),
        top,
    );
    HU4(
        dst
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 4 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize,
            ),
        top,
    );
}
#[inline]
unsafe extern "C" fn GetSSE(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < h {
        x = 0 as libc::c_int;
        while x < w {
            let diff: libc::c_int = *a.offset(x as isize) as libc::c_int
                - *b.offset(x as isize) as libc::c_int;
            count += diff * diff;
            x += 1;
            x;
        }
        a = a.offset(32 as libc::c_int as isize);
        b = b.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
    return count;
}
unsafe extern "C" fn SSE16x16_C(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
) -> libc::c_int {
    return GetSSE(a, b, 16 as libc::c_int, 16 as libc::c_int);
}
unsafe extern "C" fn SSE16x8_C(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
) -> libc::c_int {
    return GetSSE(a, b, 16 as libc::c_int, 8 as libc::c_int);
}
unsafe extern "C" fn SSE8x8_C(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
) -> libc::c_int {
    return GetSSE(a, b, 8 as libc::c_int, 8 as libc::c_int);
}
unsafe extern "C" fn SSE4x4_C(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
) -> libc::c_int {
    return GetSSE(a, b, 4 as libc::c_int, 4 as libc::c_int);
}
unsafe extern "C" fn Mean16x4_C(mut ref_0: *const uint8_t, mut dc: *mut uint32_t) {
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        let mut avg: uint32_t = 0 as libc::c_int as uint32_t;
        y = 0 as libc::c_int;
        while y < 4 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 4 as libc::c_int {
                avg = (avg as libc::c_uint)
                    .wrapping_add(
                        *ref_0.offset((x + y * 32 as libc::c_int) as isize)
                            as libc::c_uint,
                    ) as uint32_t as uint32_t;
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        *dc.offset(k as isize) = avg;
        ref_0 = ref_0.offset(4 as libc::c_int as isize);
        k += 1;
        k;
    }
}
unsafe extern "C" fn TTransform(
    mut in_0: *const uint8_t,
    mut w: *const uint16_t,
) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut tmp: [libc::c_int; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let a0: libc::c_int = *in_0.offset(0 as libc::c_int as isize) as libc::c_int
            + *in_0.offset(2 as libc::c_int as isize) as libc::c_int;
        let a1: libc::c_int = *in_0.offset(1 as libc::c_int as isize) as libc::c_int
            + *in_0.offset(3 as libc::c_int as isize) as libc::c_int;
        let a2: libc::c_int = *in_0.offset(1 as libc::c_int as isize) as libc::c_int
            - *in_0.offset(3 as libc::c_int as isize) as libc::c_int;
        let a3: libc::c_int = *in_0.offset(0 as libc::c_int as isize) as libc::c_int
            - *in_0.offset(2 as libc::c_int as isize) as libc::c_int;
        tmp[(0 as libc::c_int + i * 4 as libc::c_int) as usize] = a0 + a1;
        tmp[(1 as libc::c_int + i * 4 as libc::c_int) as usize] = a3 + a2;
        tmp[(2 as libc::c_int + i * 4 as libc::c_int) as usize] = a3 - a2;
        tmp[(3 as libc::c_int + i * 4 as libc::c_int) as usize] = a0 - a1;
        i += 1;
        i;
        in_0 = in_0.offset(32 as libc::c_int as isize);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let a0_0: libc::c_int = tmp[(0 as libc::c_int + i) as usize]
            + tmp[(8 as libc::c_int + i) as usize];
        let a1_0: libc::c_int = tmp[(4 as libc::c_int + i) as usize]
            + tmp[(12 as libc::c_int + i) as usize];
        let a2_0: libc::c_int = tmp[(4 as libc::c_int + i) as usize]
            - tmp[(12 as libc::c_int + i) as usize];
        let a3_0: libc::c_int = tmp[(0 as libc::c_int + i) as usize]
            - tmp[(8 as libc::c_int + i) as usize];
        let b0: libc::c_int = a0_0 + a1_0;
        let b1: libc::c_int = a3_0 + a2_0;
        let b2: libc::c_int = a3_0 - a2_0;
        let b3: libc::c_int = a0_0 - a1_0;
        sum += *w.offset(0 as libc::c_int as isize) as libc::c_int * abs(b0);
        sum += *w.offset(4 as libc::c_int as isize) as libc::c_int * abs(b1);
        sum += *w.offset(8 as libc::c_int as isize) as libc::c_int * abs(b2);
        sum += *w.offset(12 as libc::c_int as isize) as libc::c_int * abs(b3);
        i += 1;
        i;
        w = w.offset(1);
        w;
    }
    return sum;
}
unsafe extern "C" fn Disto4x4_C(
    a: *const uint8_t,
    b: *const uint8_t,
    w: *const uint16_t,
) -> libc::c_int {
    let sum1: libc::c_int = TTransform(a, w);
    let sum2: libc::c_int = TTransform(b, w);
    return abs(sum2 - sum1) >> 5 as libc::c_int;
}
unsafe extern "C" fn Disto16x16_C(
    a: *const uint8_t,
    b: *const uint8_t,
    w: *const uint16_t,
) -> libc::c_int {
    let mut D: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < 16 as libc::c_int * 32 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            D
                += Disto4x4_C(
                    a.offset(x as isize).offset(y as isize),
                    b.offset(x as isize).offset(y as isize),
                    w,
                );
            x += 4 as libc::c_int;
        }
        y += 4 as libc::c_int * 32 as libc::c_int;
    }
    return D;
}
static mut kZigzag: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
unsafe extern "C" fn QuantizeBlock_C(
    mut in_0: *mut int16_t,
    mut out: *mut int16_t,
    mtx: *const VP8Matrix,
) -> libc::c_int {
    let mut last: libc::c_int = -(1 as libc::c_int);
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < 16 as libc::c_int {
        let j: libc::c_int = kZigzag[n as usize] as libc::c_int;
        let sign: libc::c_int = ((*in_0.offset(j as isize) as libc::c_int)
            < 0 as libc::c_int) as libc::c_int;
        let coeff: uint32_t = ((if sign != 0 {
            -(*in_0.offset(j as isize) as libc::c_int)
        } else {
            *in_0.offset(j as isize) as libc::c_int
        }) + (*mtx).sharpen_[j as usize] as libc::c_int) as uint32_t;
        if coeff > (*mtx).zthresh_[j as usize] {
            let Q: uint32_t = (*mtx).q_[j as usize] as uint32_t;
            let iQ: uint32_t = (*mtx).iq_[j as usize] as uint32_t;
            let B: uint32_t = (*mtx).bias_[j as usize];
            let mut level: libc::c_int = QUANTDIV(coeff, iQ, B);
            if level > MAX_LEVEL as libc::c_int {
                level = MAX_LEVEL as libc::c_int;
            }
            if sign != 0 {
                level = -level;
            }
            *in_0.offset(j as isize) = (level * Q as libc::c_int) as int16_t;
            *out.offset(n as isize) = level as int16_t;
            if level != 0 {
                last = n;
            }
        } else {
            *out.offset(n as isize) = 0 as libc::c_int as int16_t;
            *in_0.offset(j as isize) = 0 as libc::c_int as int16_t;
        }
        n += 1;
        n;
    }
    return (last >= 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn Quantize2Blocks_C(
    mut in_0: *mut int16_t,
    mut out: *mut int16_t,
    mtx: *const VP8Matrix,
) -> libc::c_int {
    let mut nz: libc::c_int = 0;
    nz = VP8EncQuantizeBlock
        .expect(
            "non-null function pointer",
        )(
        in_0.offset((0 as libc::c_int * 16 as libc::c_int) as isize),
        out.offset((0 as libc::c_int * 16 as libc::c_int) as isize),
        mtx,
    ) << 0 as libc::c_int;
    nz
        |= VP8EncQuantizeBlock
            .expect(
                "non-null function pointer",
            )(
            in_0.offset((1 as libc::c_int * 16 as libc::c_int) as isize),
            out.offset((1 as libc::c_int * 16 as libc::c_int) as isize),
            mtx,
        ) << 1 as libc::c_int;
    return nz;
}
#[inline]
unsafe extern "C" fn Copy(
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < h {
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void, w as libc::c_ulong);
        src = src.offset(32 as libc::c_int as isize);
        dst = dst.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn Copy4x4_C(mut src: *const uint8_t, mut dst: *mut uint8_t) {
    Copy(src, dst, 4 as libc::c_int, 4 as libc::c_int);
}
unsafe extern "C" fn Copy16x8_C(mut src: *const uint8_t, mut dst: *mut uint8_t) {
    Copy(src, dst, 16 as libc::c_int, 8 as libc::c_int);
}
#[no_mangle]
pub static mut VP8CollectHistogram: VP8CHisto = None;
#[no_mangle]
pub static mut VP8ITransform: VP8Idct = None;
#[no_mangle]
pub static mut VP8FTransform: VP8Fdct = None;
#[no_mangle]
pub static mut VP8FTransform2: VP8Fdct = None;
#[no_mangle]
pub static mut VP8FTransformWHT: VP8WHT = None;
#[no_mangle]
pub static mut VP8EncPredLuma4: VP8Intra4Preds = None;
#[no_mangle]
pub static mut VP8EncPredLuma16: VP8IntraPreds = None;
#[no_mangle]
pub static mut VP8EncPredChroma8: VP8IntraPreds = None;
#[no_mangle]
pub static mut VP8SSE16x16: VP8Metric = None;
#[no_mangle]
pub static mut VP8SSE8x8: VP8Metric = None;
#[no_mangle]
pub static mut VP8SSE16x8: VP8Metric = None;
#[no_mangle]
pub static mut VP8SSE4x4: VP8Metric = None;
#[no_mangle]
pub static mut VP8TDisto4x4: VP8WMetric = None;
#[no_mangle]
pub static mut VP8TDisto16x16: VP8WMetric = None;
#[no_mangle]
pub static mut VP8Mean16x4: VP8MeanMetric = None;
#[no_mangle]
pub static mut VP8EncQuantizeBlock: VP8QuantizeBlock = None;
#[no_mangle]
pub static mut VP8EncQuantize2Blocks: VP8Quantize2Blocks = None;
#[no_mangle]
pub static mut VP8EncQuantizeBlockWHT: VP8QuantizeBlockWHT = None;
#[no_mangle]
pub static mut VP8Copy4x4: VP8BlockCopy = None;
#[no_mangle]
pub static mut VP8Copy16x8: VP8BlockCopy = None;
unsafe extern "C" fn VP8EncDspInit_body() {
    VP8DspInit();
    InitTables();
    VP8ITransform = Some(
        ITransform_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const int16_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    VP8FTransform = Some(
        FTransform_C
            as unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut int16_t) -> (),
    );
    VP8FTransformWHT = Some(
        FTransformWHT_C as unsafe extern "C" fn(*const int16_t, *mut int16_t) -> (),
    );
    VP8TDisto4x4 = Some(
        Disto4x4_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint16_t,
            ) -> libc::c_int,
    );
    VP8TDisto16x16 = Some(
        Disto16x16_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint16_t,
            ) -> libc::c_int,
    );
    VP8CollectHistogram = Some(
        CollectHistogram_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                *mut VP8Histogram,
            ) -> (),
    );
    VP8SSE16x16 = Some(
        SSE16x16_C as unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> libc::c_int,
    );
    VP8SSE16x8 = Some(
        SSE16x8_C as unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> libc::c_int,
    );
    VP8SSE8x8 = Some(
        SSE8x8_C as unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> libc::c_int,
    );
    VP8SSE4x4 = Some(
        SSE4x4_C as unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> libc::c_int,
    );
    VP8EncQuantizeBlock = Some(
        QuantizeBlock_C
            as unsafe extern "C" fn(
                *mut int16_t,
                *mut int16_t,
                *const VP8Matrix,
            ) -> libc::c_int,
    );
    VP8EncQuantize2Blocks = Some(
        Quantize2Blocks_C
            as unsafe extern "C" fn(
                *mut int16_t,
                *mut int16_t,
                *const VP8Matrix,
            ) -> libc::c_int,
    );
    VP8FTransform2 = Some(
        FTransform2_C
            as unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut int16_t) -> (),
    );
    VP8EncPredLuma4 = Some(
        Intra4Preds_C as unsafe extern "C" fn(*mut uint8_t, *const uint8_t) -> (),
    );
    VP8EncPredLuma16 = Some(
        Intra16Preds_C
            as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, *const uint8_t) -> (),
    );
    VP8EncPredChroma8 = Some(
        IntraChromaPreds_C
            as unsafe extern "C" fn(*mut uint8_t, *const uint8_t, *const uint8_t) -> (),
    );
    VP8Mean16x4 = Some(
        Mean16x4_C as unsafe extern "C" fn(*const uint8_t, *mut uint32_t) -> (),
    );
    VP8EncQuantizeBlockWHT = Some(
        QuantizeBlock_C
            as unsafe extern "C" fn(
                *mut int16_t,
                *mut int16_t,
                *const VP8Matrix,
            ) -> libc::c_int,
    );
    VP8Copy4x4 = Some(
        Copy4x4_C as unsafe extern "C" fn(*const uint8_t, *mut uint8_t) -> (),
    );
    VP8Copy16x8 = Some(
        Copy16x8_C as unsafe extern "C" fn(*const uint8_t, *mut uint8_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncDspInit() {
    static mut VP8EncDspInit_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut VP8EncDspInit_body_lock) != 0) {
        VP8EncDspInit_body();
        pthread_mutex_unlock(&mut VP8EncDspInit_body_lock);
    }
}
