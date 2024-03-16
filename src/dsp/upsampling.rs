use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
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
pub type WebPUpsampleLinePairFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const YUV_FIX2: C2RustUnnamed_0 = 6;
pub const YUV_MASK2: C2RustUnnamed_0 = 16383;
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const MODE_Argb: WEBP_CSP_MODE = 9;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_RGB: WEBP_CSP_MODE = 0;
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub type WebPYUV444Converter = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type WEBP_CSP_MODE = libc::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const YUV_HALF: C2RustUnnamed_0 = 32768;
pub const YUV_FIX: C2RustUnnamed_0 = 16;
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
unsafe extern "C" fn VP8YUVToR(mut y: libc::c_int, mut v: libc::c_int) -> libc::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as libc::c_int) + MultHi(v, 26149 as libc::c_int)
            - 14234 as libc::c_int,
    );
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
unsafe extern "C" fn MultHi(mut v: libc::c_int, mut coeff: libc::c_int) -> libc::c_int {
    return v * coeff >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8YUVToB(mut y: libc::c_int, mut u: libc::c_int) -> libc::c_int {
    return VP8Clip8(
        MultHi(y, 19077 as libc::c_int) + MultHi(u, 33050 as libc::c_int)
            - 17685 as libc::c_int,
    );
}
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
#[no_mangle]
pub static mut WebPUpsamplers: [WebPUpsampleLinePairFunc; 13] = [None; 13];
unsafe extern "C" fn UpsampleRgbaLinePair_C(
    mut top_y: *const uint8_t,
    mut bottom_y: *const uint8_t,
    mut top_u: *const uint8_t,
    mut top_v: *const uint8_t,
    mut cur_u: *const uint8_t,
    mut cur_v: *const uint8_t,
    mut top_dst: *mut uint8_t,
    mut bottom_dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let last_pixel_pair: libc::c_int = len - 1 as libc::c_int >> 1 as libc::c_int;
    let mut tl_uv: uint32_t = (*top_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*top_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let mut l_uv: uint32_t = (*cur_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*cur_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let uv0: uint32_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(tl_uv)
        .wrapping_add(l_uv)
        .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
    VP8YuvToRgba(
        *top_y.offset(0 as libc::c_int as isize),
        (uv0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        (uv0 >> 16 as libc::c_int) as uint8_t,
        top_dst,
    );
    if !bottom_y.is_null() {
        let uv0_0: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(l_uv)
            .wrapping_add(tl_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgba(
            *bottom_y.offset(0 as libc::c_int as isize),
            (uv0_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_0 >> 16 as libc::c_int) as uint8_t,
            bottom_dst,
        );
    }
    x = 1 as libc::c_int;
    while x <= last_pixel_pair {
        let t_uv: uint32_t = (*top_u.offset(x as isize) as libc::c_int
            | (*top_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let uv: uint32_t = (*cur_u.offset(x as isize) as libc::c_int
            | (*cur_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let avg: uint32_t = tl_uv
            .wrapping_add(t_uv)
            .wrapping_add(l_uv)
            .wrapping_add(uv)
            .wrapping_add(0x80008 as libc::c_uint);
        let diag_12: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(t_uv.wrapping_add(l_uv)),
            ) >> 3 as libc::c_int;
        let diag_03: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(tl_uv.wrapping_add(uv)),
            ) >> 3 as libc::c_int;
        let uv0_1: uint32_t = diag_12.wrapping_add(tl_uv) >> 1 as libc::c_int;
        let uv1: uint32_t = diag_03.wrapping_add(t_uv) >> 1 as libc::c_int;
        VP8YuvToRgba(
            *top_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize),
            (uv0_1 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_1 >> 16 as libc::c_int) as uint8_t,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 1 as libc::c_int) * 4 as libc::c_int)
                        as isize,
                ),
        );
        VP8YuvToRgba(
            *top_y.offset((2 as libc::c_int * x - 0 as libc::c_int) as isize),
            (uv1 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv1 >> 16 as libc::c_int) as uint8_t,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 0 as libc::c_int) * 4 as libc::c_int)
                        as isize,
                ),
        );
        if !bottom_y.is_null() {
            let uv0_2: uint32_t = diag_03.wrapping_add(l_uv) >> 1 as libc::c_int;
            let uv1_0: uint32_t = diag_12.wrapping_add(uv) >> 1 as libc::c_int;
            VP8YuvToRgba(
                *bottom_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize),
                (uv0_2 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv0_2 >> 16 as libc::c_int) as uint8_t,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x - 1 as libc::c_int) * 4 as libc::c_int)
                            as isize,
                    ),
            );
            VP8YuvToRgba(
                *bottom_y.offset((2 as libc::c_int * x + 0 as libc::c_int) as isize),
                (uv1_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv1_0 >> 16 as libc::c_int) as uint8_t,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x + 0 as libc::c_int) * 4 as libc::c_int)
                            as isize,
                    ),
            );
        }
        tl_uv = t_uv;
        l_uv = uv;
        x += 1;
        x;
    }
    if len & 1 as libc::c_int == 0 {
        let uv0_3: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tl_uv)
            .wrapping_add(l_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgba(
            *top_y.offset((len - 1 as libc::c_int) as isize),
            (uv0_3 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_3 >> 16 as libc::c_int) as uint8_t,
            top_dst.offset(((len - 1 as libc::c_int) * 4 as libc::c_int) as isize),
        );
        if !bottom_y.is_null() {
            let uv0_4: uint32_t = (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(l_uv)
                .wrapping_add(tl_uv)
                .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
            VP8YuvToRgba(
                *bottom_y.offset((len - 1 as libc::c_int) as isize),
                (uv0_4 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv0_4 >> 16 as libc::c_int) as uint8_t,
                bottom_dst.offset(((len - 1 as libc::c_int) * 4 as libc::c_int) as isize),
            );
        }
    }
}
unsafe extern "C" fn UpsampleBgraLinePair_C(
    mut top_y: *const uint8_t,
    mut bottom_y: *const uint8_t,
    mut top_u: *const uint8_t,
    mut top_v: *const uint8_t,
    mut cur_u: *const uint8_t,
    mut cur_v: *const uint8_t,
    mut top_dst: *mut uint8_t,
    mut bottom_dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let last_pixel_pair: libc::c_int = len - 1 as libc::c_int >> 1 as libc::c_int;
    let mut tl_uv: uint32_t = (*top_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*top_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let mut l_uv: uint32_t = (*cur_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*cur_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let uv0: uint32_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(tl_uv)
        .wrapping_add(l_uv)
        .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
    VP8YuvToBgra(
        *top_y.offset(0 as libc::c_int as isize),
        (uv0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        (uv0 >> 16 as libc::c_int) as uint8_t,
        top_dst,
    );
    if !bottom_y.is_null() {
        let uv0_0: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(l_uv)
            .wrapping_add(tl_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToBgra(
            *bottom_y.offset(0 as libc::c_int as isize),
            (uv0_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_0 >> 16 as libc::c_int) as uint8_t,
            bottom_dst,
        );
    }
    x = 1 as libc::c_int;
    while x <= last_pixel_pair {
        let t_uv: uint32_t = (*top_u.offset(x as isize) as libc::c_int
            | (*top_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let uv: uint32_t = (*cur_u.offset(x as isize) as libc::c_int
            | (*cur_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let avg: uint32_t = tl_uv
            .wrapping_add(t_uv)
            .wrapping_add(l_uv)
            .wrapping_add(uv)
            .wrapping_add(0x80008 as libc::c_uint);
        let diag_12: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(t_uv.wrapping_add(l_uv)),
            ) >> 3 as libc::c_int;
        let diag_03: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(tl_uv.wrapping_add(uv)),
            ) >> 3 as libc::c_int;
        let uv0_1: uint32_t = diag_12.wrapping_add(tl_uv) >> 1 as libc::c_int;
        let uv1: uint32_t = diag_03.wrapping_add(t_uv) >> 1 as libc::c_int;
        VP8YuvToBgra(
            *top_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize),
            (uv0_1 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_1 >> 16 as libc::c_int) as uint8_t,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 1 as libc::c_int) * 4 as libc::c_int)
                        as isize,
                ),
        );
        VP8YuvToBgra(
            *top_y.offset((2 as libc::c_int * x - 0 as libc::c_int) as isize),
            (uv1 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv1 >> 16 as libc::c_int) as uint8_t,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 0 as libc::c_int) * 4 as libc::c_int)
                        as isize,
                ),
        );
        if !bottom_y.is_null() {
            let uv0_2: uint32_t = diag_03.wrapping_add(l_uv) >> 1 as libc::c_int;
            let uv1_0: uint32_t = diag_12.wrapping_add(uv) >> 1 as libc::c_int;
            VP8YuvToBgra(
                *bottom_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize),
                (uv0_2 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv0_2 >> 16 as libc::c_int) as uint8_t,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x - 1 as libc::c_int) * 4 as libc::c_int)
                            as isize,
                    ),
            );
            VP8YuvToBgra(
                *bottom_y.offset((2 as libc::c_int * x + 0 as libc::c_int) as isize),
                (uv1_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv1_0 >> 16 as libc::c_int) as uint8_t,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x + 0 as libc::c_int) * 4 as libc::c_int)
                            as isize,
                    ),
            );
        }
        tl_uv = t_uv;
        l_uv = uv;
        x += 1;
        x;
    }
    if len & 1 as libc::c_int == 0 {
        let uv0_3: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tl_uv)
            .wrapping_add(l_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToBgra(
            *top_y.offset((len - 1 as libc::c_int) as isize),
            (uv0_3 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_3 >> 16 as libc::c_int) as uint8_t,
            top_dst.offset(((len - 1 as libc::c_int) * 4 as libc::c_int) as isize),
        );
        if !bottom_y.is_null() {
            let uv0_4: uint32_t = (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(l_uv)
                .wrapping_add(tl_uv)
                .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
            VP8YuvToBgra(
                *bottom_y.offset((len - 1 as libc::c_int) as isize),
                (uv0_4 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv0_4 >> 16 as libc::c_int) as uint8_t,
                bottom_dst.offset(((len - 1 as libc::c_int) * 4 as libc::c_int) as isize),
            );
        }
    }
}
unsafe extern "C" fn UpsampleArgbLinePair_C(
    mut top_y: *const uint8_t,
    mut bottom_y: *const uint8_t,
    mut top_u: *const uint8_t,
    mut top_v: *const uint8_t,
    mut cur_u: *const uint8_t,
    mut cur_v: *const uint8_t,
    mut top_dst: *mut uint8_t,
    mut bottom_dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let last_pixel_pair: libc::c_int = len - 1 as libc::c_int >> 1 as libc::c_int;
    let mut tl_uv: uint32_t = (*top_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*top_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let mut l_uv: uint32_t = (*cur_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*cur_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let uv0: uint32_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(tl_uv)
        .wrapping_add(l_uv)
        .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
    VP8YuvToArgb(
        *top_y.offset(0 as libc::c_int as isize),
        (uv0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        (uv0 >> 16 as libc::c_int) as uint8_t,
        top_dst,
    );
    if !bottom_y.is_null() {
        let uv0_0: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(l_uv)
            .wrapping_add(tl_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToArgb(
            *bottom_y.offset(0 as libc::c_int as isize),
            (uv0_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_0 >> 16 as libc::c_int) as uint8_t,
            bottom_dst,
        );
    }
    x = 1 as libc::c_int;
    while x <= last_pixel_pair {
        let t_uv: uint32_t = (*top_u.offset(x as isize) as libc::c_int
            | (*top_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let uv: uint32_t = (*cur_u.offset(x as isize) as libc::c_int
            | (*cur_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let avg: uint32_t = tl_uv
            .wrapping_add(t_uv)
            .wrapping_add(l_uv)
            .wrapping_add(uv)
            .wrapping_add(0x80008 as libc::c_uint);
        let diag_12: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(t_uv.wrapping_add(l_uv)),
            ) >> 3 as libc::c_int;
        let diag_03: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(tl_uv.wrapping_add(uv)),
            ) >> 3 as libc::c_int;
        let uv0_1: uint32_t = diag_12.wrapping_add(tl_uv) >> 1 as libc::c_int;
        let uv1: uint32_t = diag_03.wrapping_add(t_uv) >> 1 as libc::c_int;
        VP8YuvToArgb(
            *top_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize),
            (uv0_1 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_1 >> 16 as libc::c_int) as uint8_t,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 1 as libc::c_int) * 4 as libc::c_int)
                        as isize,
                ),
        );
        VP8YuvToArgb(
            *top_y.offset((2 as libc::c_int * x - 0 as libc::c_int) as isize),
            (uv1 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv1 >> 16 as libc::c_int) as uint8_t,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 0 as libc::c_int) * 4 as libc::c_int)
                        as isize,
                ),
        );
        if !bottom_y.is_null() {
            let uv0_2: uint32_t = diag_03.wrapping_add(l_uv) >> 1 as libc::c_int;
            let uv1_0: uint32_t = diag_12.wrapping_add(uv) >> 1 as libc::c_int;
            VP8YuvToArgb(
                *bottom_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize),
                (uv0_2 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv0_2 >> 16 as libc::c_int) as uint8_t,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x - 1 as libc::c_int) * 4 as libc::c_int)
                            as isize,
                    ),
            );
            VP8YuvToArgb(
                *bottom_y.offset((2 as libc::c_int * x + 0 as libc::c_int) as isize),
                (uv1_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv1_0 >> 16 as libc::c_int) as uint8_t,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x + 0 as libc::c_int) * 4 as libc::c_int)
                            as isize,
                    ),
            );
        }
        tl_uv = t_uv;
        l_uv = uv;
        x += 1;
        x;
    }
    if len & 1 as libc::c_int == 0 {
        let uv0_3: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tl_uv)
            .wrapping_add(l_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToArgb(
            *top_y.offset((len - 1 as libc::c_int) as isize),
            (uv0_3 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            (uv0_3 >> 16 as libc::c_int) as uint8_t,
            top_dst.offset(((len - 1 as libc::c_int) * 4 as libc::c_int) as isize),
        );
        if !bottom_y.is_null() {
            let uv0_4: uint32_t = (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(l_uv)
                .wrapping_add(tl_uv)
                .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
            VP8YuvToArgb(
                *bottom_y.offset((len - 1 as libc::c_int) as isize),
                (uv0_4 & 0xff as libc::c_int as libc::c_uint) as uint8_t,
                (uv0_4 >> 16 as libc::c_int) as uint8_t,
                bottom_dst.offset(((len - 1 as libc::c_int) * 4 as libc::c_int) as isize),
            );
        }
    }
}
unsafe extern "C" fn UpsampleRgbLinePair_C(
    mut top_y: *const uint8_t,
    mut bottom_y: *const uint8_t,
    mut top_u: *const uint8_t,
    mut top_v: *const uint8_t,
    mut cur_u: *const uint8_t,
    mut cur_v: *const uint8_t,
    mut top_dst: *mut uint8_t,
    mut bottom_dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let last_pixel_pair: libc::c_int = len - 1 as libc::c_int >> 1 as libc::c_int;
    let mut tl_uv: uint32_t = (*top_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*top_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let mut l_uv: uint32_t = (*cur_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*cur_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let uv0: uint32_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(tl_uv)
        .wrapping_add(l_uv)
        .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
    VP8YuvToRgb(
        *top_y.offset(0 as libc::c_int as isize) as libc::c_int,
        (uv0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (uv0 >> 16 as libc::c_int) as libc::c_int,
        top_dst,
    );
    if !bottom_y.is_null() {
        let uv0_0: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(l_uv)
            .wrapping_add(tl_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgb(
            *bottom_y.offset(0 as libc::c_int as isize) as libc::c_int,
            (uv0_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_0 >> 16 as libc::c_int) as libc::c_int,
            bottom_dst,
        );
    }
    x = 1 as libc::c_int;
    while x <= last_pixel_pair {
        let t_uv: uint32_t = (*top_u.offset(x as isize) as libc::c_int
            | (*top_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let uv: uint32_t = (*cur_u.offset(x as isize) as libc::c_int
            | (*cur_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let avg: uint32_t = tl_uv
            .wrapping_add(t_uv)
            .wrapping_add(l_uv)
            .wrapping_add(uv)
            .wrapping_add(0x80008 as libc::c_uint);
        let diag_12: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(t_uv.wrapping_add(l_uv)),
            ) >> 3 as libc::c_int;
        let diag_03: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(tl_uv.wrapping_add(uv)),
            ) >> 3 as libc::c_int;
        let uv0_1: uint32_t = diag_12.wrapping_add(tl_uv) >> 1 as libc::c_int;
        let uv1: uint32_t = diag_03.wrapping_add(t_uv) >> 1 as libc::c_int;
        VP8YuvToRgb(
            *top_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                as libc::c_int,
            (uv0_1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 1 as libc::c_int) * 3 as libc::c_int)
                        as isize,
                ),
        );
        VP8YuvToRgb(
            *top_y.offset((2 as libc::c_int * x - 0 as libc::c_int) as isize)
                as libc::c_int,
            (uv1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 0 as libc::c_int) * 3 as libc::c_int)
                        as isize,
                ),
        );
        if !bottom_y.is_null() {
            let uv0_2: uint32_t = diag_03.wrapping_add(l_uv) >> 1 as libc::c_int;
            let uv1_0: uint32_t = diag_12.wrapping_add(uv) >> 1 as libc::c_int;
            VP8YuvToRgb(
                *bottom_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                    as libc::c_int,
                (uv0_2 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_2 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x - 1 as libc::c_int) * 3 as libc::c_int)
                            as isize,
                    ),
            );
            VP8YuvToRgb(
                *bottom_y.offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                    as libc::c_int,
                (uv1_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv1_0 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x + 0 as libc::c_int) * 3 as libc::c_int)
                            as isize,
                    ),
            );
        }
        tl_uv = t_uv;
        l_uv = uv;
        x += 1;
        x;
    }
    if len & 1 as libc::c_int == 0 {
        let uv0_3: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tl_uv)
            .wrapping_add(l_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgb(
            *top_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
            (uv0_3 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_3 >> 16 as libc::c_int) as libc::c_int,
            top_dst.offset(((len - 1 as libc::c_int) * 3 as libc::c_int) as isize),
        );
        if !bottom_y.is_null() {
            let uv0_4: uint32_t = (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(l_uv)
                .wrapping_add(tl_uv)
                .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
            VP8YuvToRgb(
                *bottom_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
                (uv0_4 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_4 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst.offset(((len - 1 as libc::c_int) * 3 as libc::c_int) as isize),
            );
        }
    }
}
unsafe extern "C" fn UpsampleBgrLinePair_C(
    mut top_y: *const uint8_t,
    mut bottom_y: *const uint8_t,
    mut top_u: *const uint8_t,
    mut top_v: *const uint8_t,
    mut cur_u: *const uint8_t,
    mut cur_v: *const uint8_t,
    mut top_dst: *mut uint8_t,
    mut bottom_dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let last_pixel_pair: libc::c_int = len - 1 as libc::c_int >> 1 as libc::c_int;
    let mut tl_uv: uint32_t = (*top_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*top_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let mut l_uv: uint32_t = (*cur_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*cur_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let uv0: uint32_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(tl_uv)
        .wrapping_add(l_uv)
        .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
    VP8YuvToBgr(
        *top_y.offset(0 as libc::c_int as isize) as libc::c_int,
        (uv0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (uv0 >> 16 as libc::c_int) as libc::c_int,
        top_dst,
    );
    if !bottom_y.is_null() {
        let uv0_0: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(l_uv)
            .wrapping_add(tl_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToBgr(
            *bottom_y.offset(0 as libc::c_int as isize) as libc::c_int,
            (uv0_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_0 >> 16 as libc::c_int) as libc::c_int,
            bottom_dst,
        );
    }
    x = 1 as libc::c_int;
    while x <= last_pixel_pair {
        let t_uv: uint32_t = (*top_u.offset(x as isize) as libc::c_int
            | (*top_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let uv: uint32_t = (*cur_u.offset(x as isize) as libc::c_int
            | (*cur_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let avg: uint32_t = tl_uv
            .wrapping_add(t_uv)
            .wrapping_add(l_uv)
            .wrapping_add(uv)
            .wrapping_add(0x80008 as libc::c_uint);
        let diag_12: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(t_uv.wrapping_add(l_uv)),
            ) >> 3 as libc::c_int;
        let diag_03: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(tl_uv.wrapping_add(uv)),
            ) >> 3 as libc::c_int;
        let uv0_1: uint32_t = diag_12.wrapping_add(tl_uv) >> 1 as libc::c_int;
        let uv1: uint32_t = diag_03.wrapping_add(t_uv) >> 1 as libc::c_int;
        VP8YuvToBgr(
            *top_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                as libc::c_int,
            (uv0_1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 1 as libc::c_int) * 3 as libc::c_int)
                        as isize,
                ),
        );
        VP8YuvToBgr(
            *top_y.offset((2 as libc::c_int * x - 0 as libc::c_int) as isize)
                as libc::c_int,
            (uv1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 0 as libc::c_int) * 3 as libc::c_int)
                        as isize,
                ),
        );
        if !bottom_y.is_null() {
            let uv0_2: uint32_t = diag_03.wrapping_add(l_uv) >> 1 as libc::c_int;
            let uv1_0: uint32_t = diag_12.wrapping_add(uv) >> 1 as libc::c_int;
            VP8YuvToBgr(
                *bottom_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                    as libc::c_int,
                (uv0_2 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_2 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x - 1 as libc::c_int) * 3 as libc::c_int)
                            as isize,
                    ),
            );
            VP8YuvToBgr(
                *bottom_y.offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                    as libc::c_int,
                (uv1_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv1_0 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x + 0 as libc::c_int) * 3 as libc::c_int)
                            as isize,
                    ),
            );
        }
        tl_uv = t_uv;
        l_uv = uv;
        x += 1;
        x;
    }
    if len & 1 as libc::c_int == 0 {
        let uv0_3: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tl_uv)
            .wrapping_add(l_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToBgr(
            *top_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
            (uv0_3 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_3 >> 16 as libc::c_int) as libc::c_int,
            top_dst.offset(((len - 1 as libc::c_int) * 3 as libc::c_int) as isize),
        );
        if !bottom_y.is_null() {
            let uv0_4: uint32_t = (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(l_uv)
                .wrapping_add(tl_uv)
                .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
            VP8YuvToBgr(
                *bottom_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
                (uv0_4 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_4 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst.offset(((len - 1 as libc::c_int) * 3 as libc::c_int) as isize),
            );
        }
    }
}
unsafe extern "C" fn UpsampleRgba4444LinePair_C(
    mut top_y: *const uint8_t,
    mut bottom_y: *const uint8_t,
    mut top_u: *const uint8_t,
    mut top_v: *const uint8_t,
    mut cur_u: *const uint8_t,
    mut cur_v: *const uint8_t,
    mut top_dst: *mut uint8_t,
    mut bottom_dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let last_pixel_pair: libc::c_int = len - 1 as libc::c_int >> 1 as libc::c_int;
    let mut tl_uv: uint32_t = (*top_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*top_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let mut l_uv: uint32_t = (*cur_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*cur_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let uv0: uint32_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(tl_uv)
        .wrapping_add(l_uv)
        .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
    VP8YuvToRgba4444(
        *top_y.offset(0 as libc::c_int as isize) as libc::c_int,
        (uv0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (uv0 >> 16 as libc::c_int) as libc::c_int,
        top_dst,
    );
    if !bottom_y.is_null() {
        let uv0_0: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(l_uv)
            .wrapping_add(tl_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgba4444(
            *bottom_y.offset(0 as libc::c_int as isize) as libc::c_int,
            (uv0_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_0 >> 16 as libc::c_int) as libc::c_int,
            bottom_dst,
        );
    }
    x = 1 as libc::c_int;
    while x <= last_pixel_pair {
        let t_uv: uint32_t = (*top_u.offset(x as isize) as libc::c_int
            | (*top_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let uv: uint32_t = (*cur_u.offset(x as isize) as libc::c_int
            | (*cur_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let avg: uint32_t = tl_uv
            .wrapping_add(t_uv)
            .wrapping_add(l_uv)
            .wrapping_add(uv)
            .wrapping_add(0x80008 as libc::c_uint);
        let diag_12: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(t_uv.wrapping_add(l_uv)),
            ) >> 3 as libc::c_int;
        let diag_03: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(tl_uv.wrapping_add(uv)),
            ) >> 3 as libc::c_int;
        let uv0_1: uint32_t = diag_12.wrapping_add(tl_uv) >> 1 as libc::c_int;
        let uv1: uint32_t = diag_03.wrapping_add(t_uv) >> 1 as libc::c_int;
        VP8YuvToRgba4444(
            *top_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                as libc::c_int,
            (uv0_1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 1 as libc::c_int) * 2 as libc::c_int)
                        as isize,
                ),
        );
        VP8YuvToRgba4444(
            *top_y.offset((2 as libc::c_int * x - 0 as libc::c_int) as isize)
                as libc::c_int,
            (uv1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 0 as libc::c_int) * 2 as libc::c_int)
                        as isize,
                ),
        );
        if !bottom_y.is_null() {
            let uv0_2: uint32_t = diag_03.wrapping_add(l_uv) >> 1 as libc::c_int;
            let uv1_0: uint32_t = diag_12.wrapping_add(uv) >> 1 as libc::c_int;
            VP8YuvToRgba4444(
                *bottom_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                    as libc::c_int,
                (uv0_2 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_2 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x - 1 as libc::c_int) * 2 as libc::c_int)
                            as isize,
                    ),
            );
            VP8YuvToRgba4444(
                *bottom_y.offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                    as libc::c_int,
                (uv1_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv1_0 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x + 0 as libc::c_int) * 2 as libc::c_int)
                            as isize,
                    ),
            );
        }
        tl_uv = t_uv;
        l_uv = uv;
        x += 1;
        x;
    }
    if len & 1 as libc::c_int == 0 {
        let uv0_3: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tl_uv)
            .wrapping_add(l_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgba4444(
            *top_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
            (uv0_3 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_3 >> 16 as libc::c_int) as libc::c_int,
            top_dst.offset(((len - 1 as libc::c_int) * 2 as libc::c_int) as isize),
        );
        if !bottom_y.is_null() {
            let uv0_4: uint32_t = (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(l_uv)
                .wrapping_add(tl_uv)
                .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
            VP8YuvToRgba4444(
                *bottom_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
                (uv0_4 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_4 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst.offset(((len - 1 as libc::c_int) * 2 as libc::c_int) as isize),
            );
        }
    }
}
unsafe extern "C" fn UpsampleRgb565LinePair_C(
    mut top_y: *const uint8_t,
    mut bottom_y: *const uint8_t,
    mut top_u: *const uint8_t,
    mut top_v: *const uint8_t,
    mut cur_u: *const uint8_t,
    mut cur_v: *const uint8_t,
    mut top_dst: *mut uint8_t,
    mut bottom_dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let last_pixel_pair: libc::c_int = len - 1 as libc::c_int >> 1 as libc::c_int;
    let mut tl_uv: uint32_t = (*top_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*top_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let mut l_uv: uint32_t = (*cur_u.offset(0 as libc::c_int as isize) as libc::c_int
        | (*cur_v.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    let uv0: uint32_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(tl_uv)
        .wrapping_add(l_uv)
        .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
    VP8YuvToRgb565(
        *top_y.offset(0 as libc::c_int as isize) as libc::c_int,
        (uv0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        (uv0 >> 16 as libc::c_int) as libc::c_int,
        top_dst,
    );
    if !bottom_y.is_null() {
        let uv0_0: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(l_uv)
            .wrapping_add(tl_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgb565(
            *bottom_y.offset(0 as libc::c_int as isize) as libc::c_int,
            (uv0_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_0 >> 16 as libc::c_int) as libc::c_int,
            bottom_dst,
        );
    }
    x = 1 as libc::c_int;
    while x <= last_pixel_pair {
        let t_uv: uint32_t = (*top_u.offset(x as isize) as libc::c_int
            | (*top_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let uv: uint32_t = (*cur_u.offset(x as isize) as libc::c_int
            | (*cur_v.offset(x as isize) as libc::c_int) << 16 as libc::c_int)
            as uint32_t;
        let avg: uint32_t = tl_uv
            .wrapping_add(t_uv)
            .wrapping_add(l_uv)
            .wrapping_add(uv)
            .wrapping_add(0x80008 as libc::c_uint);
        let diag_12: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(t_uv.wrapping_add(l_uv)),
            ) >> 3 as libc::c_int;
        let diag_03: uint32_t = avg
            .wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul(tl_uv.wrapping_add(uv)),
            ) >> 3 as libc::c_int;
        let uv0_1: uint32_t = diag_12.wrapping_add(tl_uv) >> 1 as libc::c_int;
        let uv1: uint32_t = diag_03.wrapping_add(t_uv) >> 1 as libc::c_int;
        VP8YuvToRgb565(
            *top_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                as libc::c_int,
            (uv0_1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 1 as libc::c_int) * 2 as libc::c_int)
                        as isize,
                ),
        );
        VP8YuvToRgb565(
            *top_y.offset((2 as libc::c_int * x - 0 as libc::c_int) as isize)
                as libc::c_int,
            (uv1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv1 >> 16 as libc::c_int) as libc::c_int,
            top_dst
                .offset(
                    ((2 as libc::c_int * x - 0 as libc::c_int) * 2 as libc::c_int)
                        as isize,
                ),
        );
        if !bottom_y.is_null() {
            let uv0_2: uint32_t = diag_03.wrapping_add(l_uv) >> 1 as libc::c_int;
            let uv1_0: uint32_t = diag_12.wrapping_add(uv) >> 1 as libc::c_int;
            VP8YuvToRgb565(
                *bottom_y.offset((2 as libc::c_int * x - 1 as libc::c_int) as isize)
                    as libc::c_int,
                (uv0_2 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_2 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x - 1 as libc::c_int) * 2 as libc::c_int)
                            as isize,
                    ),
            );
            VP8YuvToRgb565(
                *bottom_y.offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                    as libc::c_int,
                (uv1_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv1_0 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst
                    .offset(
                        ((2 as libc::c_int * x + 0 as libc::c_int) * 2 as libc::c_int)
                            as isize,
                    ),
            );
        }
        tl_uv = t_uv;
        l_uv = uv;
        x += 1;
        x;
    }
    if len & 1 as libc::c_int == 0 {
        let uv0_3: uint32_t = (3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tl_uv)
            .wrapping_add(l_uv)
            .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
        VP8YuvToRgb565(
            *top_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
            (uv0_3 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
            (uv0_3 >> 16 as libc::c_int) as libc::c_int,
            top_dst.offset(((len - 1 as libc::c_int) * 2 as libc::c_int) as isize),
        );
        if !bottom_y.is_null() {
            let uv0_4: uint32_t = (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(l_uv)
                .wrapping_add(tl_uv)
                .wrapping_add(0x20002 as libc::c_uint) >> 2 as libc::c_int;
            VP8YuvToRgb565(
                *bottom_y.offset((len - 1 as libc::c_int) as isize) as libc::c_int,
                (uv0_4 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                (uv0_4 >> 16 as libc::c_int) as libc::c_int,
                bottom_dst.offset(((len - 1 as libc::c_int) * 2 as libc::c_int) as isize),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPGetLinePairConverter(
    mut alpha_is_last: libc::c_int,
) -> WebPUpsampleLinePairFunc {
    WebPInitUpsamplers();
    return WebPUpsamplers[(if alpha_is_last != 0 {
        MODE_BGRA as libc::c_int
    } else {
        MODE_ARGB as libc::c_int
    }) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn WebPYuv444ToRgba_C(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        VP8YuvToRgba(
            *y.offset(i as isize),
            *u.offset(i as isize),
            *v.offset(i as isize),
            &mut *dst.offset((i * 4 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPYuv444ToBgra_C(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        VP8YuvToBgra(
            *y.offset(i as isize),
            *u.offset(i as isize),
            *v.offset(i as isize),
            &mut *dst.offset((i * 4 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPYuv444ToRgb_C(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        VP8YuvToRgb(
            *y.offset(i as isize) as libc::c_int,
            *u.offset(i as isize) as libc::c_int,
            *v.offset(i as isize) as libc::c_int,
            &mut *dst.offset((i * 3 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPYuv444ToBgr_C(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        VP8YuvToBgr(
            *y.offset(i as isize) as libc::c_int,
            *u.offset(i as isize) as libc::c_int,
            *v.offset(i as isize) as libc::c_int,
            &mut *dst.offset((i * 3 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPYuv444ToArgb_C(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        VP8YuvToArgb(
            *y.offset(i as isize),
            *u.offset(i as isize),
            *v.offset(i as isize),
            &mut *dst.offset((i * 4 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPYuv444ToRgba4444_C(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        VP8YuvToRgba4444(
            *y.offset(i as isize) as libc::c_int,
            *u.offset(i as isize) as libc::c_int,
            *v.offset(i as isize) as libc::c_int,
            &mut *dst.offset((i * 2 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPYuv444ToRgb565_C(
    mut y: *const uint8_t,
    mut u: *const uint8_t,
    mut v: *const uint8_t,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        VP8YuvToRgb565(
            *y.offset(i as isize) as libc::c_int,
            *u.offset(i as isize) as libc::c_int,
            *v.offset(i as isize) as libc::c_int,
            &mut *dst.offset((i * 2 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut WebPYUV444Converters: [WebPYUV444Converter; 13] = [None; 13];
#[no_mangle]
pub unsafe extern "C" fn WebPInitYUV444Converters() {
    static mut WebPInitYUV444Converters_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut WebPInitYUV444Converters_body_lock) != 0) {
        WebPInitYUV444Converters_body();
        pthread_mutex_unlock(&mut WebPInitYUV444Converters_body_lock);
    }
}
unsafe extern "C" fn WebPInitYUV444Converters_body() {
    WebPYUV444Converters[MODE_RGBA as libc::c_int
        as usize] = Some(
        WebPYuv444ToRgba_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_BGRA as libc::c_int
        as usize] = Some(
        WebPYuv444ToBgra_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_RGB as libc::c_int
        as usize] = Some(
        WebPYuv444ToRgb_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_BGR as libc::c_int
        as usize] = Some(
        WebPYuv444ToBgr_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_ARGB as libc::c_int
        as usize] = Some(
        WebPYuv444ToArgb_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_RGBA_4444 as libc::c_int
        as usize] = Some(
        WebPYuv444ToRgba4444_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_RGB_565 as libc::c_int
        as usize] = Some(
        WebPYuv444ToRgb565_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_rgbA as libc::c_int
        as usize] = Some(
        WebPYuv444ToRgba_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_bgrA as libc::c_int
        as usize] = Some(
        WebPYuv444ToBgra_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_Argb as libc::c_int
        as usize] = Some(
        WebPYuv444ToArgb_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPYUV444Converters[MODE_rgbA_4444 as libc::c_int
        as usize] = Some(
        WebPYuv444ToRgba4444_C
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
pub unsafe extern "C" fn WebPInitUpsamplers() {
    static mut WebPInitUpsamplers_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut WebPInitUpsamplers_body_lock) != 0) {
        WebPInitUpsamplers_body();
        pthread_mutex_unlock(&mut WebPInitUpsamplers_body_lock);
    }
}
unsafe extern "C" fn WebPInitUpsamplers_body() {
    WebPUpsamplers[MODE_RGBA as libc::c_int
        as usize] = Some(
        UpsampleRgbaLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_BGRA as libc::c_int
        as usize] = Some(
        UpsampleBgraLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_rgbA as libc::c_int
        as usize] = Some(
        UpsampleRgbaLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_bgrA as libc::c_int
        as usize] = Some(
        UpsampleBgraLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_RGB as libc::c_int
        as usize] = Some(
        UpsampleRgbLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_BGR as libc::c_int
        as usize] = Some(
        UpsampleBgrLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_ARGB as libc::c_int
        as usize] = Some(
        UpsampleArgbLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_RGBA_4444 as libc::c_int
        as usize] = Some(
        UpsampleRgba4444LinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_RGB_565 as libc::c_int
        as usize] = Some(
        UpsampleRgb565LinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_Argb as libc::c_int
        as usize] = Some(
        UpsampleArgbLinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUpsamplers[MODE_rgbA_4444 as libc::c_int
        as usize] = Some(
        UpsampleRgba4444LinePair_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
}
