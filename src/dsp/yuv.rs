use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type WebPSamplerRowFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub const YUV_FIX2: C2RustUnnamed_0 = 6;
pub const YUV_MASK2: C2RustUnnamed_0 = 16383;
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const MODE_Argb: WEBP_CSP_MODE = 9;
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub const MODE_RGB: WEBP_CSP_MODE = 0;
pub const YUV_HALF: C2RustUnnamed_0 = 32768;
pub const YUV_FIX: C2RustUnnamed_0 = 16;
pub type WEBP_CSP_MODE = libc::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn VP8YuvToRgba4444(
    mut y: libc::c_int,
    mut u: libc::c_int,
    mut v: libc::c_int,
    argb: *mut uint8_t,
) {
    let r: libc::c_int = VP8YUVToR(y, v);
    let g: libc::c_int = VP8YUVToG(y, u, v);
    let b: libc::c_int = VP8YUVToB(y, u);
    let rg: libc::c_int = r & 0xf0 as libc::c_int | g >> 4 as libc::c_int;
    let ba: libc::c_int = b & 0xf0 as libc::c_int | 0xf as libc::c_int;
    *argb.offset(0 as libc::c_int as isize) = rg as uint8_t;
    *argb.offset(1 as libc::c_int as isize) = ba as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YUVToB(mut y: libc::c_int, mut u: libc::c_int) -> libc::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as libc::c_int) + MultHi(u, 33050 as libc::c_int)
            - 17685 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn MultHi(mut v: libc::c_int, mut coeff: libc::c_int) -> libc::c_int {
    return v * coeff >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8Clip8(mut v: libc::c_int) -> libc::c_int {
    return if v & !(YUV_MASK2 as libc::c_int) == 0 as libc::c_int {
        v >> YUV_FIX2 as libc::c_int
    } else if v < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        255 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8YUVToG(
    mut y: libc::c_int,
    mut u: libc::c_int,
    mut v: libc::c_int,
) -> libc::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as libc::c_int) - MultHi(u, 6419 as libc::c_int)
            - MultHi(v, 13320 as libc::c_int) + 8708 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn VP8YUVToR(mut y: libc::c_int, mut v: libc::c_int) -> libc::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as libc::c_int) + MultHi(v, 26149 as libc::c_int)
            - 14234 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn VP8YuvToArgb(
    mut y: uint8_t,
    mut u: uint8_t,
    mut v: uint8_t,
    argb: *mut uint8_t,
) {
    *argb.offset(0 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
    VP8YuvToRgb(
        y as libc::c_int,
        u as libc::c_int,
        v as libc::c_int,
        argb.offset(1 as libc::c_int as isize),
    );
}
#[inline]
unsafe extern "C" fn VP8YuvToRgb(
    mut y: libc::c_int,
    mut u: libc::c_int,
    mut v: libc::c_int,
    rgb: *mut uint8_t,
) {
    *rgb.offset(0 as libc::c_int as isize) = VP8YUVToR(y, v) as uint8_t;
    *rgb.offset(1 as libc::c_int as isize) = VP8YUVToG(y, u, v) as uint8_t;
    *rgb.offset(2 as libc::c_int as isize) = VP8YUVToB(y, u) as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToBgra(
    mut y: uint8_t,
    mut u: uint8_t,
    mut v: uint8_t,
    bgra: *mut uint8_t,
) {
    VP8YuvToBgr(y as libc::c_int, u as libc::c_int, v as libc::c_int, bgra);
    *bgra.offset(3 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToBgr(
    mut y: libc::c_int,
    mut u: libc::c_int,
    mut v: libc::c_int,
    bgr: *mut uint8_t,
) {
    *bgr.offset(0 as libc::c_int as isize) = VP8YUVToB(y, u) as uint8_t;
    *bgr.offset(1 as libc::c_int as isize) = VP8YUVToG(y, u, v) as uint8_t;
    *bgr.offset(2 as libc::c_int as isize) = VP8YUVToR(y, v) as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToRgba(
    mut y: uint8_t,
    mut u: uint8_t,
    mut v: uint8_t,
    rgba: *mut uint8_t,
) {
    VP8YuvToRgb(y as libc::c_int, u as libc::c_int, v as libc::c_int, rgba);
    *rgba.offset(3 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8YuvToRgb565(
    mut y: libc::c_int,
    mut u: libc::c_int,
    mut v: libc::c_int,
    rgb: *mut uint8_t,
) {
    let r: libc::c_int = VP8YUVToR(y, v);
    let g: libc::c_int = VP8YUVToG(y, u, v);
    let b: libc::c_int = VP8YUVToB(y, u);
    let rg: libc::c_int = r & 0xf8 as libc::c_int | g >> 5 as libc::c_int;
    let gb: libc::c_int = g << 3 as libc::c_int & 0xe0 as libc::c_int
        | b >> 3 as libc::c_int;
    *rgb.offset(0 as libc::c_int as isize) = rg as uint8_t;
    *rgb.offset(1 as libc::c_int as isize) = gb as uint8_t;
}
#[inline]
unsafe extern "C" fn VP8RGBToV(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    mut rounding: libc::c_int,
) -> libc::c_int {
    let v: libc::c_int = 28800 as libc::c_int * r - 24116 as libc::c_int * g
        - 4684 as libc::c_int * b;
    return VP8ClipUV(v, rounding);
}
#[inline]
unsafe extern "C" fn VP8ClipUV(
    mut uv: libc::c_int,
    mut rounding: libc::c_int,
) -> libc::c_int {
    uv = uv + rounding
        + ((128 as libc::c_int) << YUV_FIX as libc::c_int + 2 as libc::c_int)
        >> YUV_FIX as libc::c_int + 2 as libc::c_int;
    return if uv & !(0xff as libc::c_int) == 0 as libc::c_int {
        uv
    } else if uv < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        255 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8RGBToU(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    mut rounding: libc::c_int,
) -> libc::c_int {
    let u: libc::c_int = -(9719 as libc::c_int) * r - 19081 as libc::c_int * g
        + 28800 as libc::c_int * b;
    return VP8ClipUV(u, rounding);
}
#[inline]
unsafe extern "C" fn VP8RGBToY(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    mut rounding: libc::c_int,
) -> libc::c_int {
    let luma: libc::c_int = 16839 as libc::c_int * r + 33059 as libc::c_int * g
        + 6420 as libc::c_int * b;
    return luma + rounding + ((16 as libc::c_int) << YUV_FIX as libc::c_int)
        >> YUV_FIX as libc::c_int;
}
unsafe extern "C" fn YuvToRgbRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let end: *const uint8_t = dst
        .offset(((len & !(1 as libc::c_int)) * 3 as libc::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgb(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
        VP8YuvToRgb(
            *y.offset(1 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst.offset(3 as libc::c_int as isize),
        );
        y = y.offset(2 as libc::c_int as isize);
        u = u.offset(1);
        u;
        v = v.offset(1);
        v;
        dst = dst.offset((2 as libc::c_int * 3 as libc::c_int) as isize);
    }
    if len & 1 as libc::c_int != 0 {
        VP8YuvToRgb(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
    }
}
unsafe extern "C" fn YuvToBgrRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let end: *const uint8_t = dst
        .offset(((len & !(1 as libc::c_int)) * 3 as libc::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToBgr(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
        VP8YuvToBgr(
            *y.offset(1 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst.offset(3 as libc::c_int as isize),
        );
        y = y.offset(2 as libc::c_int as isize);
        u = u.offset(1);
        u;
        v = v.offset(1);
        v;
        dst = dst.offset((2 as libc::c_int * 3 as libc::c_int) as isize);
    }
    if len & 1 as libc::c_int != 0 {
        VP8YuvToBgr(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
    }
}
unsafe extern "C" fn YuvToRgbaRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let end: *const uint8_t = dst
        .offset(((len & !(1 as libc::c_int)) * 4 as libc::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgba(
            *y.offset(0 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst,
        );
        VP8YuvToRgba(
            *y.offset(1 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst.offset(4 as libc::c_int as isize),
        );
        y = y.offset(2 as libc::c_int as isize);
        u = u.offset(1);
        u;
        v = v.offset(1);
        v;
        dst = dst.offset((2 as libc::c_int * 4 as libc::c_int) as isize);
    }
    if len & 1 as libc::c_int != 0 {
        VP8YuvToRgba(
            *y.offset(0 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst,
        );
    }
}
unsafe extern "C" fn YuvToBgraRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let end: *const uint8_t = dst
        .offset(((len & !(1 as libc::c_int)) * 4 as libc::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToBgra(
            *y.offset(0 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst,
        );
        VP8YuvToBgra(
            *y.offset(1 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst.offset(4 as libc::c_int as isize),
        );
        y = y.offset(2 as libc::c_int as isize);
        u = u.offset(1);
        u;
        v = v.offset(1);
        v;
        dst = dst.offset((2 as libc::c_int * 4 as libc::c_int) as isize);
    }
    if len & 1 as libc::c_int != 0 {
        VP8YuvToBgra(
            *y.offset(0 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst,
        );
    }
}
unsafe extern "C" fn YuvToArgbRow(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let end: *const uint8_t = dst
        .offset(((len & !(1 as libc::c_int)) * 4 as libc::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToArgb(
            *y.offset(0 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst,
        );
        VP8YuvToArgb(
            *y.offset(1 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst.offset(4 as libc::c_int as isize),
        );
        y = y.offset(2 as libc::c_int as isize);
        u = u.offset(1);
        u;
        v = v.offset(1);
        v;
        dst = dst.offset((2 as libc::c_int * 4 as libc::c_int) as isize);
    }
    if len & 1 as libc::c_int != 0 {
        VP8YuvToArgb(
            *y.offset(0 as libc::c_int as isize),
            *u.offset(0 as libc::c_int as isize),
            *v.offset(0 as libc::c_int as isize),
            dst,
        );
    }
}
unsafe extern "C" fn YuvToRgba4444Row(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let end: *const uint8_t = dst
        .offset(((len & !(1 as libc::c_int)) * 2 as libc::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgba4444(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
        VP8YuvToRgba4444(
            *y.offset(1 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst.offset(2 as libc::c_int as isize),
        );
        y = y.offset(2 as libc::c_int as isize);
        u = u.offset(1);
        u;
        v = v.offset(1);
        v;
        dst = dst.offset((2 as libc::c_int * 2 as libc::c_int) as isize);
    }
    if len & 1 as libc::c_int != 0 {
        VP8YuvToRgba4444(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
    }
}
unsafe extern "C" fn YuvToRgb565Row(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let end: *const uint8_t = dst
        .offset(((len & !(1 as libc::c_int)) * 2 as libc::c_int) as isize);
    while dst != end as *mut uint8_t {
        VP8YuvToRgb565(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
        VP8YuvToRgb565(
            *y.offset(1 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst.offset(2 as libc::c_int as isize),
        );
        y = y.offset(2 as libc::c_int as isize);
        u = u.offset(1);
        u;
        v = v.offset(1);
        v;
        dst = dst.offset((2 as libc::c_int * 2 as libc::c_int) as isize);
    }
    if len & 1 as libc::c_int != 0 {
        VP8YuvToRgb565(
            *y.offset(0 as libc::c_int as isize) as libc::c_int,
            *u.offset(0 as libc::c_int as isize) as libc::c_int,
            *v.offset(0 as libc::c_int as isize) as libc::c_int,
            dst,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPSamplerProcessPlane(
    mut y: *const uint8_t,
    mut y_stride: libc::c_int,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut uv_stride: libc::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut func: WebPSamplerRowFunc,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < height {
        func.expect("non-null function pointer")(y, u, v, dst, width);
        y = y.offset(y_stride as isize);
        if j & 1 as libc::c_int != 0 {
            u = u.offset(uv_stride as isize);
            v = v.offset(uv_stride as isize);
        }
        dst = dst.offset(dst_stride as isize);
        j += 1;
        j;
    }
}
#[no_mangle]
pub static mut WebPSamplers: [WebPSamplerRowFunc; 13] = [None; 13];
unsafe extern "C" fn WebPInitSamplers_body() {
    WebPSamplers[MODE_RGB as libc::c_int
        as usize] = Some(
        YuvToRgbRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_RGBA as libc::c_int
        as usize] = Some(
        YuvToRgbaRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_BGR as libc::c_int
        as usize] = Some(
        YuvToBgrRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_BGRA as libc::c_int
        as usize] = Some(
        YuvToBgraRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_ARGB as libc::c_int
        as usize] = Some(
        YuvToArgbRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_RGBA_4444 as libc::c_int
        as usize] = Some(
        YuvToRgba4444Row
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_RGB_565 as libc::c_int
        as usize] = Some(
        YuvToRgb565Row
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_rgbA as libc::c_int
        as usize] = Some(
        YuvToRgbaRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_bgrA as libc::c_int
        as usize] = Some(
        YuvToBgraRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_Argb as libc::c_int
        as usize] = Some(
        YuvToArgbRow
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPSamplers[MODE_rgbA_4444 as libc::c_int
        as usize] = Some(
        YuvToRgba4444Row
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitSamplers() {
    static mut WebPInitSamplers_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut WebPInitSamplers_body_lock) != 0) {
        WebPInitSamplers_body();
        pthread_mutex_unlock(&mut WebPInitSamplers_body_lock);
    }
}
unsafe extern "C" fn ConvertARGBToY_C(
    mut argb: *const uint32_t,
    mut y: *mut uint8_t,
    mut width: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < width {
        let p: uint32_t = *argb.offset(i as isize);
        *y
            .offset(
                i as isize,
            ) = VP8RGBToY(
            (p >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_int,
            (p >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (p >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            YUV_HALF as libc::c_int,
        ) as uint8_t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPConvertARGBToUV_C(
    mut argb: *const uint32_t,
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut src_width: libc::c_int,
    mut do_store: libc::c_int,
) {
    let uv_width: libc::c_int = src_width >> 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < uv_width {
        let v0: uint32_t = *argb
            .offset((2 as libc::c_int * i + 0 as libc::c_int) as isize);
        let v1: uint32_t = *argb
            .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
        let r: libc::c_int = (v0 >> 15 as libc::c_int
            & 0x1fe as libc::c_int as libc::c_uint)
            .wrapping_add(v1 >> 15 as libc::c_int & 0x1fe as libc::c_int as libc::c_uint)
            as libc::c_int;
        let g: libc::c_int = (v0 >> 7 as libc::c_int
            & 0x1fe as libc::c_int as libc::c_uint)
            .wrapping_add(v1 >> 7 as libc::c_int & 0x1fe as libc::c_int as libc::c_uint)
            as libc::c_int;
        let b: libc::c_int = (v0 << 1 as libc::c_int
            & 0x1fe as libc::c_int as libc::c_uint)
            .wrapping_add(v1 << 1 as libc::c_int & 0x1fe as libc::c_int as libc::c_uint)
            as libc::c_int;
        let tmp_u: libc::c_int = VP8RGBToU(
            r,
            g,
            b,
            (YUV_HALF as libc::c_int) << 2 as libc::c_int,
        );
        let tmp_v: libc::c_int = VP8RGBToV(
            r,
            g,
            b,
            (YUV_HALF as libc::c_int) << 2 as libc::c_int,
        );
        if do_store != 0 {
            *u.offset(i as isize) = tmp_u as uint8_t;
            *v.offset(i as isize) = tmp_v as uint8_t;
        } else {
            *u
                .offset(
                    i as isize,
                ) = (*u.offset(i as isize) as libc::c_int + tmp_u + 1 as libc::c_int
                >> 1 as libc::c_int) as uint8_t;
            *v
                .offset(
                    i as isize,
                ) = (*v.offset(i as isize) as libc::c_int + tmp_v + 1 as libc::c_int
                >> 1 as libc::c_int) as uint8_t;
        }
        i += 1;
        i;
    }
    if src_width & 1 as libc::c_int != 0 {
        let v0_0: uint32_t = *argb
            .offset((2 as libc::c_int * i + 0 as libc::c_int) as isize);
        let r_0: libc::c_int = (v0_0 >> 14 as libc::c_int
            & 0x3fc as libc::c_int as libc::c_uint) as libc::c_int;
        let g_0: libc::c_int = (v0_0 >> 6 as libc::c_int
            & 0x3fc as libc::c_int as libc::c_uint) as libc::c_int;
        let b_0: libc::c_int = (v0_0 << 2 as libc::c_int
            & 0x3fc as libc::c_int as libc::c_uint) as libc::c_int;
        let tmp_u_0: libc::c_int = VP8RGBToU(
            r_0,
            g_0,
            b_0,
            (YUV_HALF as libc::c_int) << 2 as libc::c_int,
        );
        let tmp_v_0: libc::c_int = VP8RGBToV(
            r_0,
            g_0,
            b_0,
            (YUV_HALF as libc::c_int) << 2 as libc::c_int,
        );
        if do_store != 0 {
            *u.offset(i as isize) = tmp_u_0 as uint8_t;
            *v.offset(i as isize) = tmp_v_0 as uint8_t;
        } else {
            *u
                .offset(
                    i as isize,
                ) = (*u.offset(i as isize) as libc::c_int + tmp_u_0 + 1 as libc::c_int
                >> 1 as libc::c_int) as uint8_t;
            *v
                .offset(
                    i as isize,
                ) = (*v.offset(i as isize) as libc::c_int + tmp_v_0 + 1 as libc::c_int
                >> 1 as libc::c_int) as uint8_t;
        }
    }
}
unsafe extern "C" fn ConvertRGB24ToY_C(
    mut rgb: *const uint8_t,
    mut y: *mut uint8_t,
    mut width: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < width {
        *y
            .offset(
                i as isize,
            ) = VP8RGBToY(
            *rgb.offset(0 as libc::c_int as isize) as libc::c_int,
            *rgb.offset(1 as libc::c_int as isize) as libc::c_int,
            *rgb.offset(2 as libc::c_int as isize) as libc::c_int,
            YUV_HALF as libc::c_int,
        ) as uint8_t;
        i += 1;
        i;
        rgb = rgb.offset(3 as libc::c_int as isize);
    }
}
unsafe extern "C" fn ConvertBGR24ToY_C(
    mut bgr: *const uint8_t,
    mut y: *mut uint8_t,
    mut width: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < width {
        *y
            .offset(
                i as isize,
            ) = VP8RGBToY(
            *bgr.offset(2 as libc::c_int as isize) as libc::c_int,
            *bgr.offset(1 as libc::c_int as isize) as libc::c_int,
            *bgr.offset(0 as libc::c_int as isize) as libc::c_int,
            YUV_HALF as libc::c_int,
        ) as uint8_t;
        i += 1;
        i;
        bgr = bgr.offset(3 as libc::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPConvertRGBA32ToUV_C(
    mut rgb: *const uint16_t,
    mut u: *mut uint8_t,
    mut v: *mut uint8_t,
    mut width: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < width {
        let r: libc::c_int = *rgb.offset(0 as libc::c_int as isize) as libc::c_int;
        let g: libc::c_int = *rgb.offset(1 as libc::c_int as isize) as libc::c_int;
        let b: libc::c_int = *rgb.offset(2 as libc::c_int as isize) as libc::c_int;
        *u
            .offset(
                i as isize,
            ) = VP8RGBToU(r, g, b, (YUV_HALF as libc::c_int) << 2 as libc::c_int)
            as uint8_t;
        *v
            .offset(
                i as isize,
            ) = VP8RGBToV(r, g, b, (YUV_HALF as libc::c_int) << 2 as libc::c_int)
            as uint8_t;
        i += 1 as libc::c_int;
        rgb = rgb.offset(4 as libc::c_int as isize);
    }
}
#[no_mangle]
pub static mut WebPConvertRGB24ToY: Option::<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertBGR24ToY: Option::<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertRGBA32ToUV: Option::<
    unsafe extern "C" fn(*const uint16_t, *mut uint8_t, *mut uint8_t, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertARGBToY: Option::<
    unsafe extern "C" fn(*const uint32_t, *mut uint8_t, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPConvertARGBToUV: Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        *mut uint8_t,
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn WebPInitConvertARGBToYUV() {
    static mut WebPInitConvertARGBToYUV_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut WebPInitConvertARGBToYUV_body_lock) != 0) {
        WebPInitConvertARGBToYUV_body();
        pthread_mutex_unlock(&mut WebPInitConvertARGBToYUV_body_lock);
    }
}
unsafe extern "C" fn WebPInitConvertARGBToYUV_body() {
    WebPConvertARGBToY = Some(
        ConvertARGBToY_C
            as unsafe extern "C" fn(*const uint32_t, *mut uint8_t, libc::c_int) -> (),
    );
    WebPConvertARGBToUV = Some(
        WebPConvertARGBToUV_C
            as unsafe extern "C" fn(
                *const uint32_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    WebPConvertRGB24ToY = Some(
        ConvertRGB24ToY_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
    );
    WebPConvertBGR24ToY = Some(
        ConvertBGR24ToY_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
    );
    WebPConvertRGBA32ToUV = Some(
        WebPConvertRGBA32ToUV_C
            as unsafe extern "C" fn(
                *const uint16_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
}
