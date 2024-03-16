use ::libc;

use crate::src::utils::types::WEBP_CSP_MODE;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut WebPApplyAlphaMultiply: Option::<
        unsafe extern "C" fn(
            *mut uint8_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >;
    static mut WebPApplyAlphaMultiply4444: Option::<
        unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type VP8LImageTransformType = libc::c_uint;
pub const COLOR_INDEXING_TRANSFORM: VP8LImageTransformType = 3;
pub const SUBTRACT_GREEN_TRANSFORM: VP8LImageTransformType = 2;
pub const CROSS_COLOR_TRANSFORM: VP8LImageTransformType = 1;
pub const PREDICTOR_TRANSFORM: VP8LImageTransformType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LTransform {
    pub type_: VP8LImageTransformType,
    pub bits_: libc::c_int,
    pub xsize_: libc::c_int,
    pub ysize_: libc::c_int,
    pub data_: *mut uint32_t,
}
pub type VP8LPredictorFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
>;
pub type VP8LPredictorAddSubFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        libc::c_int,
        *mut uint32_t,
    ) -> (),
>;
pub type VP8LProcessDecBlueAndRedFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint32_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMultipliers {
    pub green_to_red_: uint8_t,
    pub green_to_blue_: uint8_t,
    pub red_to_blue_: uint8_t,
}
pub type VP8LTransformColorInverseFunc = Option::<
    unsafe extern "C" fn(
        *const VP8LMultipliers,
        *const uint32_t,
        libc::c_int,
        *mut uint32_t,
    ) -> (),
>;
pub type VP8LMapARGBFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        *mut uint32_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type VP8LConvertFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint8_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub w: uint16_t,
    pub b: [uint8_t; 2],
}
pub type VP8LMapAlphaFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint32_t,
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[inline]
unsafe extern "C" fn BSwap32(mut x: uint32_t) -> uint32_t {
    return x.swap_bytes();
}
#[inline]
unsafe extern "C" fn WebPUint32ToMem(ptr: *mut uint8_t, mut val: uint32_t) {
    memcpy(
        ptr as *mut libc::c_void,
        &mut val as *mut uint32_t as *const libc::c_void,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn VP8GetARGBIndex(mut idx: uint32_t) -> uint32_t {
    return idx >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn VP8GetAlphaIndex(mut idx: uint8_t) -> uint8_t {
    return idx;
}
#[inline]
unsafe extern "C" fn VP8GetARGBValue(mut val: uint32_t) -> uint32_t {
    return val;
}
#[inline]
unsafe extern "C" fn VP8GetAlphaValue(mut val: uint32_t) -> uint8_t {
    return (val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8LSubSampleSize(
    mut size: uint32_t,
    mut sampling_bits: uint32_t,
) -> uint32_t {
    return size
        .wrapping_add(((1 as libc::c_int) << sampling_bits) as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) >> sampling_bits;
}
#[inline]
unsafe extern "C" fn VP8LAddPixels(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let alpha_and_green: uint32_t = (a & 0xff00ff00 as libc::c_uint)
        .wrapping_add(b & 0xff00ff00 as libc::c_uint);
    let red_and_blue: uint32_t = (a & 0xff00ff as libc::c_uint)
        .wrapping_add(b & 0xff00ff as libc::c_uint);
    return alpha_and_green & 0xff00ff00 as libc::c_uint
        | red_and_blue & 0xff00ff as libc::c_uint;
}
#[inline]
unsafe extern "C" fn Average2(mut a0: uint32_t, mut a1: uint32_t) -> uint32_t {
    return (((a0 ^ a1) & 0xfefefefe as libc::c_uint) >> 1 as libc::c_int)
        .wrapping_add(a0 & a1);
}
#[inline]
unsafe extern "C" fn Average3(
    mut a0: uint32_t,
    mut a1: uint32_t,
    mut a2: uint32_t,
) -> uint32_t {
    return Average2(Average2(a0, a2), a1);
}
#[inline]
unsafe extern "C" fn Average4(
    mut a0: uint32_t,
    mut a1: uint32_t,
    mut a2: uint32_t,
    mut a3: uint32_t,
) -> uint32_t {
    return Average2(Average2(a0, a1), Average2(a2, a3));
}
#[inline]
unsafe extern "C" fn Clip255(mut a: uint32_t) -> uint32_t {
    if a < 256 as libc::c_int as libc::c_uint {
        return a;
    }
    return !a >> 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn AddSubtractComponentFull(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    return Clip255((a + b - c) as uint32_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ClampedAddSubtractFull(
    mut c0: uint32_t,
    mut c1: uint32_t,
    mut c2: uint32_t,
) -> uint32_t {
    let a: libc::c_int = AddSubtractComponentFull(
        (c0 >> 24 as libc::c_int) as libc::c_int,
        (c1 >> 24 as libc::c_int) as libc::c_int,
        (c2 >> 24 as libc::c_int) as libc::c_int,
    );
    let r: libc::c_int = AddSubtractComponentFull(
        (c0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
    );
    let g: libc::c_int = AddSubtractComponentFull(
        (c0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
    );
    let b: libc::c_int = AddSubtractComponentFull(
        (c0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c2 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
    );
    return (a as uint32_t) << 24 as libc::c_int
        | (r << 16 as libc::c_int) as libc::c_uint
        | (g << 8 as libc::c_int) as libc::c_uint | b as libc::c_uint;
}
#[inline]
unsafe extern "C" fn AddSubtractComponentHalf(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return Clip255((a + (a - b) / 2 as libc::c_int) as uint32_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ClampedAddSubtractHalf(
    mut c0: uint32_t,
    mut c1: uint32_t,
    mut c2: uint32_t,
) -> uint32_t {
    let ave: uint32_t = Average2(c0, c1);
    let a: libc::c_int = AddSubtractComponentHalf(
        (ave >> 24 as libc::c_int) as libc::c_int,
        (c2 >> 24 as libc::c_int) as libc::c_int,
    );
    let r: libc::c_int = AddSubtractComponentHalf(
        (ave >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
    );
    let g: libc::c_int = AddSubtractComponentHalf(
        (ave >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
    );
    let b: libc::c_int = AddSubtractComponentHalf(
        (ave >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (c2 >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
    );
    return (a as uint32_t) << 24 as libc::c_int
        | (r << 16 as libc::c_int) as libc::c_uint
        | (g << 8 as libc::c_int) as libc::c_uint | b as libc::c_uint;
}
#[inline]
unsafe extern "C" fn Sub3(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let pb: libc::c_int = b - c;
    let pa: libc::c_int = a - c;
    return abs(pb) - abs(pa);
}
#[inline]
unsafe extern "C" fn Select(
    mut a: uint32_t,
    mut b: uint32_t,
    mut c: uint32_t,
) -> uint32_t {
    let pa_minus_pb: libc::c_int = Sub3(
        (a >> 24 as libc::c_int) as libc::c_int,
        (b >> 24 as libc::c_int) as libc::c_int,
        (c >> 24 as libc::c_int) as libc::c_int,
    )
        + Sub3(
            (a >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_int,
            (b >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_int,
            (c >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        )
        + Sub3(
            (a >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (b >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (c >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        )
        + Sub3(
            (a & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (b & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (c & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        );
    return if pa_minus_pb <= 0 as libc::c_int { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor0_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    return 0xff000000 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor1_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    return *left;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor2_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    return *top.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor3_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    return *top.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor4_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    return *top.offset(-(1 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor5_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Average3(
        *left,
        *top.offset(0 as libc::c_int as isize),
        *top.offset(1 as libc::c_int as isize),
    );
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor6_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Average2(*left, *top.offset(-(1 as libc::c_int) as isize));
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor7_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Average2(*left, *top.offset(0 as libc::c_int as isize));
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor8_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Average2(
        *top.offset(-(1 as libc::c_int) as isize),
        *top.offset(0 as libc::c_int as isize),
    );
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor9_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Average2(
        *top.offset(0 as libc::c_int as isize),
        *top.offset(1 as libc::c_int as isize),
    );
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor10_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Average4(
        *left,
        *top.offset(-(1 as libc::c_int) as isize),
        *top.offset(0 as libc::c_int as isize),
        *top.offset(1 as libc::c_int as isize),
    );
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor11_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = Select(
        *top.offset(0 as libc::c_int as isize),
        *left,
        *top.offset(-(1 as libc::c_int) as isize),
    );
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor12_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = ClampedAddSubtractFull(
        *left,
        *top.offset(0 as libc::c_int as isize),
        *top.offset(-(1 as libc::c_int) as isize),
    );
    return pred;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPredictor13_C(
    left: *const uint32_t,
    top: *const uint32_t,
) -> uint32_t {
    let pred: uint32_t = ClampedAddSubtractHalf(
        *left,
        *top.offset(0 as libc::c_int as isize),
        *top.offset(-(1 as libc::c_int) as isize),
    );
    return pred;
}
unsafe extern "C" fn PredictorAdd0_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        *out
            .offset(
                x as isize,
            ) = VP8LAddPixels(*in_0.offset(x as isize), 0xff000000 as libc::c_uint);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd1_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut i: libc::c_int = 0;
    let mut left: uint32_t = *out.offset(-(1 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < num_pixels {
        left = VP8LAddPixels(*in_0.offset(i as isize), left);
        *out.offset(i as isize) = left;
        i += 1;
        i;
    }
}
unsafe extern "C" fn PredictorAdd2_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor2_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd3_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor3_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd4_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor4_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd5_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor5_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd6_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor6_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd7_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor7_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd8_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor8_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd9_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor9_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd10_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor10_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd11_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor11_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd12_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor12_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorAdd13_C(
    mut in_0: *const uint32_t,
    mut upper: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < num_pixels {
        let pred: uint32_t = VP8LPredictor13_C(
            &mut *out.offset((x - 1 as libc::c_int) as isize),
            upper.offset(x as isize),
        );
        *out.offset(x as isize) = VP8LAddPixels(*in_0.offset(x as isize), pred);
        x += 1;
        x;
    }
}
unsafe extern "C" fn PredictorInverseTransform_C(
    transform: *const VP8LTransform,
    mut y_start: libc::c_int,
    mut y_end: libc::c_int,
    mut in_0: *const uint32_t,
    mut out: *mut uint32_t,
) {
    let width: libc::c_int = (*transform).xsize_;
    if y_start == 0 as libc::c_int {
        PredictorAdd0_C(in_0, 0 as *const uint32_t, 1 as libc::c_int, out);
        PredictorAdd1_C(
            in_0.offset(1 as libc::c_int as isize),
            0 as *const uint32_t,
            width - 1 as libc::c_int,
            out.offset(1 as libc::c_int as isize),
        );
        in_0 = in_0.offset(width as isize);
        out = out.offset(width as isize);
        y_start += 1;
        y_start;
    }
    let mut y: libc::c_int = y_start;
    let tile_width: libc::c_int = (1 as libc::c_int) << (*transform).bits_;
    let mask: libc::c_int = tile_width - 1 as libc::c_int;
    let tiles_per_row: libc::c_int = VP8LSubSampleSize(
        width as uint32_t,
        (*transform).bits_ as uint32_t,
    ) as libc::c_int;
    let mut pred_mode_base: *const uint32_t = ((*transform).data_)
        .offset(((y >> (*transform).bits_) * tiles_per_row) as isize);
    while y < y_end {
        let mut pred_mode_src: *const uint32_t = pred_mode_base;
        let mut x: libc::c_int = 1 as libc::c_int;
        PredictorAdd2_C(in_0, out.offset(-(width as isize)), 1 as libc::c_int, out);
        while x < width {
            let fresh0 = pred_mode_src;
            pred_mode_src = pred_mode_src.offset(1);
            let pred_func: VP8LPredictorAddSubFunc = VP8LPredictorsAdd[(*fresh0
                >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as usize];
            let mut x_end: libc::c_int = (x & !mask) + tile_width;
            if x_end > width {
                x_end = width;
            }
            pred_func
                .expect(
                    "non-null function pointer",
                )(
                in_0.offset(x as isize),
                out.offset(x as isize).offset(-(width as isize)),
                x_end - x,
                out.offset(x as isize),
            );
            x = x_end;
        }
        in_0 = in_0.offset(width as isize);
        out = out.offset(width as isize);
        y += 1;
        y;
        if y & mask == 0 as libc::c_int {
            pred_mode_base = pred_mode_base.offset(tiles_per_row as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LAddGreenToBlueAndRed_C(
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint32_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        let argb: uint32_t = *src.offset(i as isize);
        let green: uint32_t = argb >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint;
        let mut red_blue: uint32_t = argb & 0xff00ff as libc::c_uint;
        red_blue = (red_blue as libc::c_uint)
            .wrapping_add(green << 16 as libc::c_int | green) as uint32_t as uint32_t;
        red_blue &= 0xff00ff as libc::c_uint;
        *dst.offset(i as isize) = argb & 0xff00ff00 as libc::c_uint | red_blue;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn ColorTransformDelta(
    mut color_pred: int8_t,
    mut color: int8_t,
) -> libc::c_int {
    return color_pred as libc::c_int * color as libc::c_int >> 5 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ColorCodeToMultipliers(
    mut color_code: uint32_t,
    m: *mut VP8LMultipliers,
) {
    (*m)
        .green_to_red_ = (color_code >> 0 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    (*m)
        .green_to_blue_ = (color_code >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    (*m)
        .red_to_blue_ = (color_code >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LTransformColorInverse_C(
    m: *const VP8LMultipliers,
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint32_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        let argb: uint32_t = *src.offset(i as isize);
        let green: int8_t = (argb >> 8 as libc::c_int) as int8_t;
        let red: uint32_t = argb >> 16 as libc::c_int;
        let mut new_red: libc::c_int = (red & 0xff as libc::c_int as libc::c_uint)
            as libc::c_int;
        let mut new_blue: libc::c_int = (argb & 0xff as libc::c_int as libc::c_uint)
            as libc::c_int;
        new_red += ColorTransformDelta((*m).green_to_red_ as int8_t, green);
        new_red &= 0xff as libc::c_int;
        new_blue += ColorTransformDelta((*m).green_to_blue_ as int8_t, green);
        new_blue += ColorTransformDelta((*m).red_to_blue_ as int8_t, new_red as int8_t);
        new_blue &= 0xff as libc::c_int;
        *dst
            .offset(
                i as isize,
            ) = argb & 0xff00ff00 as libc::c_uint
            | (new_red << 16 as libc::c_int) as libc::c_uint | new_blue as libc::c_uint;
        i += 1;
        i;
    }
}
unsafe extern "C" fn ColorSpaceInverseTransform_C(
    transform: *const VP8LTransform,
    mut y_start: libc::c_int,
    mut y_end: libc::c_int,
    mut src: *const uint32_t,
    mut dst: *mut uint32_t,
) {
    let width: libc::c_int = (*transform).xsize_;
    let tile_width: libc::c_int = (1 as libc::c_int) << (*transform).bits_;
    let mask: libc::c_int = tile_width - 1 as libc::c_int;
    let safe_width: libc::c_int = width & !mask;
    let remaining_width: libc::c_int = width - safe_width;
    let tiles_per_row: libc::c_int = VP8LSubSampleSize(
        width as uint32_t,
        (*transform).bits_ as uint32_t,
    ) as libc::c_int;
    let mut y: libc::c_int = y_start;
    let mut pred_row: *const uint32_t = ((*transform).data_)
        .offset(((y >> (*transform).bits_) * tiles_per_row) as isize);
    while y < y_end {
        let mut pred: *const uint32_t = pred_row;
        let mut m: VP8LMultipliers = {
            let mut init = VP8LMultipliers {
                green_to_red_: 0 as libc::c_int as uint8_t,
                green_to_blue_: 0 as libc::c_int as uint8_t,
                red_to_blue_: 0 as libc::c_int as uint8_t,
            };
            init
        };
        let src_safe_end: *const uint32_t = src.offset(safe_width as isize);
        let src_end: *const uint32_t = src.offset(width as isize);
        while src < src_safe_end {
            let fresh1 = pred;
            pred = pred.offset(1);
            ColorCodeToMultipliers(*fresh1, &mut m);
            VP8LTransformColorInverse
                .expect("non-null function pointer")(&mut m, src, tile_width, dst);
            src = src.offset(tile_width as isize);
            dst = dst.offset(tile_width as isize);
        }
        if src < src_end {
            let fresh2 = pred;
            pred = pred.offset(1);
            ColorCodeToMultipliers(*fresh2, &mut m);
            VP8LTransformColorInverse
                .expect("non-null function pointer")(&mut m, src, remaining_width, dst);
            src = src.offset(remaining_width as isize);
            dst = dst.offset(remaining_width as isize);
        }
        y += 1;
        y;
        if y & mask == 0 as libc::c_int {
            pred_row = pred_row.offset(tiles_per_row as isize);
        }
    }
}
unsafe extern "C" fn MapARGB_C(
    mut src: *const uint32_t,
    color_map: *const uint32_t,
    mut dst: *mut uint32_t,
    mut y_start: libc::c_int,
    mut y_end: libc::c_int,
    mut width: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    y = y_start;
    while y < y_end {
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < width {
            let fresh3 = src;
            src = src.offset(1);
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = VP8GetARGBValue(
                *color_map.offset(VP8GetARGBIndex(*fresh3) as isize),
            );
            x += 1;
            x;
        }
        y += 1;
        y;
    }
}
unsafe extern "C" fn ColorIndexInverseTransform_C(
    transform: *const VP8LTransform,
    mut y_start: libc::c_int,
    mut y_end: libc::c_int,
    mut src: *const uint32_t,
    mut dst: *mut uint32_t,
) {
    let mut y: libc::c_int = 0;
    let bits_per_pixel: libc::c_int = 8 as libc::c_int >> (*transform).bits_;
    let width: libc::c_int = (*transform).xsize_;
    let color_map: *const uint32_t = (*transform).data_;
    if bits_per_pixel < 8 as libc::c_int {
        let pixels_per_byte: libc::c_int = (1 as libc::c_int) << (*transform).bits_;
        let count_mask: libc::c_int = pixels_per_byte - 1 as libc::c_int;
        let bit_mask: uint32_t = (((1 as libc::c_int) << bits_per_pixel)
            - 1 as libc::c_int) as uint32_t;
        y = y_start;
        while y < y_end {
            let mut packed_pixels: uint32_t = 0 as libc::c_int as uint32_t;
            let mut x: libc::c_int = 0;
            x = 0 as libc::c_int;
            while x < width {
                if x & count_mask == 0 as libc::c_int {
                    let fresh5 = src;
                    src = src.offset(1);
                    packed_pixels = VP8GetARGBIndex(*fresh5);
                }
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = VP8GetARGBValue(
                    *color_map.offset((packed_pixels & bit_mask) as isize),
                );
                packed_pixels >>= bits_per_pixel;
                x += 1;
                x;
            }
            y += 1;
            y;
        }
    } else {
        VP8LMapColor32b
            .expect(
                "non-null function pointer",
            )(src, color_map, dst, y_start, y_end, width);
    };
}
unsafe extern "C" fn MapAlpha_C(
    mut src: *const uint8_t,
    color_map: *const uint32_t,
    mut dst: *mut uint8_t,
    mut y_start: libc::c_int,
    mut y_end: libc::c_int,
    mut width: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    y = y_start;
    while y < y_end {
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < width {
            let fresh7 = src;
            src = src.offset(1);
            let fresh8 = dst;
            dst = dst.offset(1);
            *fresh8 = VP8GetAlphaValue(
                *color_map.offset(VP8GetAlphaIndex(*fresh7) as isize),
            );
            x += 1;
            x;
        }
        y += 1;
        y;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorIndexInverseTransformAlpha(
    transform: *const VP8LTransform,
    mut y_start: libc::c_int,
    mut y_end: libc::c_int,
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
) {
    let mut y: libc::c_int = 0;
    let bits_per_pixel: libc::c_int = 8 as libc::c_int >> (*transform).bits_;
    let width: libc::c_int = (*transform).xsize_;
    let color_map: *const uint32_t = (*transform).data_;
    if bits_per_pixel < 8 as libc::c_int {
        let pixels_per_byte: libc::c_int = (1 as libc::c_int) << (*transform).bits_;
        let count_mask: libc::c_int = pixels_per_byte - 1 as libc::c_int;
        let bit_mask: uint32_t = (((1 as libc::c_int) << bits_per_pixel)
            - 1 as libc::c_int) as uint32_t;
        y = y_start;
        while y < y_end {
            let mut packed_pixels: uint32_t = 0 as libc::c_int as uint32_t;
            let mut x: libc::c_int = 0;
            x = 0 as libc::c_int;
            while x < width {
                if x & count_mask == 0 as libc::c_int {
                    let fresh9 = src;
                    src = src.offset(1);
                    packed_pixels = VP8GetAlphaIndex(*fresh9) as uint32_t;
                }
                let fresh10 = dst;
                dst = dst.offset(1);
                *fresh10 = VP8GetAlphaValue(
                    *color_map.offset((packed_pixels & bit_mask) as isize),
                );
                packed_pixels >>= bits_per_pixel;
                x += 1;
                x;
            }
            y += 1;
            y;
        }
    } else {
        VP8LMapColor8b
            .expect(
                "non-null function pointer",
            )(src, color_map, dst, y_start, y_end, width);
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LInverseTransform(
    transform: *const VP8LTransform,
    mut row_start: libc::c_int,
    mut row_end: libc::c_int,
    in_0: *const uint32_t,
    out: *mut uint32_t,
) {
    let width: libc::c_int = (*transform).xsize_;
    match (*transform).type_ as libc::c_uint {
        2 => {
            VP8LAddGreenToBlueAndRed
                .expect(
                    "non-null function pointer",
                )(in_0, (row_end - row_start) * width, out);
        }
        0 => {
            PredictorInverseTransform_C(transform, row_start, row_end, in_0, out);
            if row_end != (*transform).ysize_ {
                memcpy(
                    out.offset(-(width as isize)) as *mut libc::c_void,
                    out
                        .offset(
                            ((row_end - row_start - 1 as libc::c_int) * width) as isize,
                        ) as *const libc::c_void,
                    (width as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
            }
        }
        1 => {
            ColorSpaceInverseTransform_C(transform, row_start, row_end, in_0, out);
        }
        3 => {
            if in_0 == out as *const uint32_t && (*transform).bits_ > 0 as libc::c_int {
                let out_stride: libc::c_int = (row_end - row_start) * width;
                let in_stride: libc::c_int = ((row_end - row_start) as libc::c_uint)
                    .wrapping_mul(
                        VP8LSubSampleSize(
                            (*transform).xsize_ as uint32_t,
                            (*transform).bits_ as uint32_t,
                        ),
                    ) as libc::c_int;
                let src: *mut uint32_t = out
                    .offset(out_stride as isize)
                    .offset(-(in_stride as isize));
                memmove(
                    src as *mut libc::c_void,
                    out as *const libc::c_void,
                    (in_stride as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ),
                );
                ColorIndexInverseTransform_C(transform, row_start, row_end, src, out);
            } else {
                ColorIndexInverseTransform_C(transform, row_start, row_end, in_0, out);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn is_big_endian() -> libc::c_int {
    static mut tmp: C2RustUnnamed_0 = C2RustUnnamed_0 {
        w: 1 as libc::c_int as uint16_t,
    };
    return (tmp.b[0 as libc::c_int as usize] as libc::c_int != 1 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGB_C(
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh11 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh11;
        let fresh12 = dst;
        dst = dst.offset(1);
        *fresh12 = (argb >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = (argb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        let fresh14 = dst;
        dst = dst.offset(1);
        *fresh14 = (argb >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGBA_C(
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh15 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh15;
        let fresh16 = dst;
        dst = dst.offset(1);
        *fresh16 = (argb >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        let fresh17 = dst;
        dst = dst.offset(1);
        *fresh17 = (argb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        let fresh18 = dst;
        dst = dst.offset(1);
        *fresh18 = (argb >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        let fresh19 = dst;
        dst = dst.offset(1);
        *fresh19 = (argb >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGBA4444_C(
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh20 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh20;
        let rg: uint8_t = (argb >> 16 as libc::c_int
            & 0xf0 as libc::c_int as libc::c_uint
            | argb >> 12 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as uint8_t;
        let ba: uint8_t = (argb >> 0 as libc::c_int & 0xf0 as libc::c_int as libc::c_uint
            | argb >> 28 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as uint8_t;
        let fresh21 = dst;
        dst = dst.offset(1);
        *fresh21 = rg;
        let fresh22 = dst;
        dst = dst.offset(1);
        *fresh22 = ba;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToRGB565_C(
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh23 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh23;
        let rg: uint8_t = (argb >> 16 as libc::c_int
            & 0xf8 as libc::c_int as libc::c_uint
            | argb >> 13 as libc::c_int & 0x7 as libc::c_int as libc::c_uint) as uint8_t;
        let gb: uint8_t = (argb >> 5 as libc::c_int & 0xe0 as libc::c_int as libc::c_uint
            | argb >> 3 as libc::c_int & 0x1f as libc::c_int as libc::c_uint) as uint8_t;
        let fresh24 = dst;
        dst = dst.offset(1);
        *fresh24 = rg;
        let fresh25 = dst;
        dst = dst.offset(1);
        *fresh25 = gb;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertBGRAToBGR_C(
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint8_t,
) {
    let src_end: *const uint32_t = src.offset(num_pixels as isize);
    while src < src_end {
        let fresh26 = src;
        src = src.offset(1);
        let argb: uint32_t = *fresh26;
        let fresh27 = dst;
        dst = dst.offset(1);
        *fresh27 = (argb >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        let fresh28 = dst;
        dst = dst.offset(1);
        *fresh28 = (argb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        let fresh29 = dst;
        dst = dst.offset(1);
        *fresh29 = (argb >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
    }
}
unsafe extern "C" fn CopyOrSwap(
    mut src: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut dst: *mut uint8_t,
    mut swap_on_big_endian: libc::c_int,
) {
    if is_big_endian() == swap_on_big_endian {
        let src_end: *const uint32_t = src.offset(num_pixels as isize);
        while src < src_end {
            let fresh30 = src;
            src = src.offset(1);
            let argb: uint32_t = *fresh30;
            WebPUint32ToMem(dst, BSwap32(argb));
            dst = dst
                .offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize);
        }
    } else {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (num_pixels as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LConvertFromBGRA(
    in_data: *const uint32_t,
    mut num_pixels: libc::c_int,
    mut out_colorspace: WEBP_CSP_MODE,
    rgba: *mut uint8_t,
) {
    match out_colorspace as libc::c_uint {
        0 => {
            VP8LConvertBGRAToRGB
                .expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        1 => {
            VP8LConvertBGRAToRGBA
                .expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        7 => {
            VP8LConvertBGRAToRGBA
                .expect("non-null function pointer")(in_data, num_pixels, rgba);
            WebPApplyAlphaMultiply
                .expect(
                    "non-null function pointer",
                )(
                rgba,
                0 as libc::c_int,
                num_pixels,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        2 => {
            VP8LConvertBGRAToBGR
                .expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        3 => {
            CopyOrSwap(in_data, num_pixels, rgba, 1 as libc::c_int);
        }
        8 => {
            CopyOrSwap(in_data, num_pixels, rgba, 1 as libc::c_int);
            WebPApplyAlphaMultiply
                .expect(
                    "non-null function pointer",
                )(
                rgba,
                0 as libc::c_int,
                num_pixels,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        4 => {
            CopyOrSwap(in_data, num_pixels, rgba, 0 as libc::c_int);
        }
        9 => {
            CopyOrSwap(in_data, num_pixels, rgba, 0 as libc::c_int);
            WebPApplyAlphaMultiply
                .expect(
                    "non-null function pointer",
                )(
                rgba,
                1 as libc::c_int,
                num_pixels,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        5 => {
            VP8LConvertBGRAToRGBA4444
                .expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        10 => {
            VP8LConvertBGRAToRGBA4444
                .expect("non-null function pointer")(in_data, num_pixels, rgba);
            WebPApplyAlphaMultiply4444
                .expect(
                    "non-null function pointer",
                )(rgba, num_pixels, 1 as libc::c_int, 0 as libc::c_int);
        }
        6 => {
            VP8LConvertBGRAToRGB565
                .expect("non-null function pointer")(in_data, num_pixels, rgba);
        }
        _ => {}
    };
}
#[no_mangle]
pub static mut VP8LAddGreenToBlueAndRed: VP8LProcessDecBlueAndRedFunc = None;
#[no_mangle]
pub static mut VP8LPredictorsAdd: [VP8LPredictorAddSubFunc; 16] = [None; 16];
#[no_mangle]
pub static mut VP8LPredictors: [VP8LPredictorFunc; 16] = [None; 16];
#[no_mangle]
pub static mut VP8LPredictorsAdd_C: [VP8LPredictorAddSubFunc; 16] = [None; 16];
#[no_mangle]
pub static mut VP8LTransformColorInverse: VP8LTransformColorInverseFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGB: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGBA: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGBA4444: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToRGB565: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LConvertBGRAToBGR: VP8LConvertFunc = None;
#[no_mangle]
pub static mut VP8LMapColor32b: VP8LMapARGBFunc = None;
#[no_mangle]
pub static mut VP8LMapColor8b: VP8LMapAlphaFunc = None;
unsafe extern "C" fn VP8LDspInit_body() {
    VP8LPredictors[0 as libc::c_int
        as usize] = Some(
        VP8LPredictor0_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[1 as libc::c_int
        as usize] = Some(
        VP8LPredictor1_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[2 as libc::c_int
        as usize] = Some(
        VP8LPredictor2_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[3 as libc::c_int
        as usize] = Some(
        VP8LPredictor3_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[4 as libc::c_int
        as usize] = Some(
        VP8LPredictor4_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[5 as libc::c_int
        as usize] = Some(
        VP8LPredictor5_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[6 as libc::c_int
        as usize] = Some(
        VP8LPredictor6_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[7 as libc::c_int
        as usize] = Some(
        VP8LPredictor7_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[8 as libc::c_int
        as usize] = Some(
        VP8LPredictor8_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[9 as libc::c_int
        as usize] = Some(
        VP8LPredictor9_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[10 as libc::c_int
        as usize] = Some(
        VP8LPredictor10_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[11 as libc::c_int
        as usize] = Some(
        VP8LPredictor11_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[12 as libc::c_int
        as usize] = Some(
        VP8LPredictor12_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[13 as libc::c_int
        as usize] = Some(
        VP8LPredictor13_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[14 as libc::c_int
        as usize] = Some(
        VP8LPredictor0_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictors[15 as libc::c_int
        as usize] = Some(
        VP8LPredictor0_C
            as unsafe extern "C" fn(*const uint32_t, *const uint32_t) -> uint32_t,
    );
    VP8LPredictorsAdd[0 as libc::c_int
        as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[1 as libc::c_int
        as usize] = Some(
        PredictorAdd1_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[2 as libc::c_int
        as usize] = Some(
        PredictorAdd2_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[3 as libc::c_int
        as usize] = Some(
        PredictorAdd3_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[4 as libc::c_int
        as usize] = Some(
        PredictorAdd4_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[5 as libc::c_int
        as usize] = Some(
        PredictorAdd5_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[6 as libc::c_int
        as usize] = Some(
        PredictorAdd6_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[7 as libc::c_int
        as usize] = Some(
        PredictorAdd7_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[8 as libc::c_int
        as usize] = Some(
        PredictorAdd8_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[9 as libc::c_int
        as usize] = Some(
        PredictorAdd9_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[10 as libc::c_int
        as usize] = Some(
        PredictorAdd10_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[11 as libc::c_int
        as usize] = Some(
        PredictorAdd11_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[12 as libc::c_int
        as usize] = Some(
        PredictorAdd12_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[13 as libc::c_int
        as usize] = Some(
        PredictorAdd13_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[14 as libc::c_int
        as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd[15 as libc::c_int
        as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[0 as libc::c_int
        as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[1 as libc::c_int
        as usize] = Some(
        PredictorAdd1_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[2 as libc::c_int
        as usize] = Some(
        PredictorAdd2_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[3 as libc::c_int
        as usize] = Some(
        PredictorAdd3_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[4 as libc::c_int
        as usize] = Some(
        PredictorAdd4_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[5 as libc::c_int
        as usize] = Some(
        PredictorAdd5_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[6 as libc::c_int
        as usize] = Some(
        PredictorAdd6_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[7 as libc::c_int
        as usize] = Some(
        PredictorAdd7_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[8 as libc::c_int
        as usize] = Some(
        PredictorAdd8_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[9 as libc::c_int
        as usize] = Some(
        PredictorAdd9_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[10 as libc::c_int
        as usize] = Some(
        PredictorAdd10_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[11 as libc::c_int
        as usize] = Some(
        PredictorAdd11_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[12 as libc::c_int
        as usize] = Some(
        PredictorAdd12_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[13 as libc::c_int
        as usize] = Some(
        PredictorAdd13_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[14 as libc::c_int
        as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LPredictorsAdd_C[15 as libc::c_int
        as usize] = Some(
        PredictorAdd0_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LAddGreenToBlueAndRed = Some(
        VP8LAddGreenToBlueAndRed_C
            as unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint32_t) -> (),
    );
    VP8LTransformColorInverse = Some(
        VP8LTransformColorInverse_C
            as unsafe extern "C" fn(
                *const VP8LMultipliers,
                *const uint32_t,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    VP8LConvertBGRAToRGBA = Some(
        VP8LConvertBGRAToRGBA_C
            as unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint8_t) -> (),
    );
    VP8LConvertBGRAToRGB = Some(
        VP8LConvertBGRAToRGB_C
            as unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint8_t) -> (),
    );
    VP8LConvertBGRAToBGR = Some(
        VP8LConvertBGRAToBGR_C
            as unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint8_t) -> (),
    );
    VP8LConvertBGRAToRGBA4444 = Some(
        VP8LConvertBGRAToRGBA4444_C
            as unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint8_t) -> (),
    );
    VP8LConvertBGRAToRGB565 = Some(
        VP8LConvertBGRAToRGB565_C
            as unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint8_t) -> (),
    );
    VP8LMapColor32b = Some(
        MapARGB_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *const uint32_t,
                *mut uint32_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    VP8LMapColor8b = Some(
        MapAlpha_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint32_t,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDspInit() {
    static mut VP8LDspInit_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut VP8LDspInit_body_lock) != 0) {
        VP8LDspInit_body();
        pthread_mutex_unlock(&mut VP8LDspInit_body_lock);
    }
}
