use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut VP8GetCPUInfo: VP8CPUInfo;
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
pub type VP8CPUInfo = Option::<unsafe extern "C" fn(CPUFeature) -> libc::c_int>;
unsafe extern "C" fn Mult(mut x: uint8_t, mut mult: uint32_t) -> uint32_t {
    let v: uint32_t = (x as libc::c_uint)
        .wrapping_mul(mult)
        .wrapping_add((1 as libc::c_uint) << 24 as libc::c_int >> 1 as libc::c_int)
        >> 24 as libc::c_int;
    return v;
}
#[inline]
unsafe extern "C" fn GetScale(mut a: uint32_t, mut inverse: libc::c_int) -> uint32_t {
    return if inverse != 0 {
        ((255 as libc::c_uint) << 24 as libc::c_int).wrapping_div(a)
    } else {
        a.wrapping_mul(
            ((1 as libc::c_uint) << 24 as libc::c_int).wrapping_div(255 as libc::c_uint),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPMultARGBRow_C(
    ptr: *mut uint32_t,
    mut width: libc::c_int,
    mut inverse: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < width {
        let argb: uint32_t = *ptr.offset(x as isize);
        if argb < 0xff000000 as libc::c_uint {
            if argb <= 0xffffff as libc::c_uint {
                *ptr.offset(x as isize) = 0 as libc::c_int as uint32_t;
            } else {
                let alpha: uint32_t = argb >> 24 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint;
                let scale: uint32_t = GetScale(alpha, inverse);
                let mut out: uint32_t = argb & 0xff000000 as libc::c_uint;
                out
                    |= Mult((argb >> 0 as libc::c_int) as uint8_t, scale)
                        << 0 as libc::c_int;
                out
                    |= Mult((argb >> 8 as libc::c_int) as uint8_t, scale)
                        << 8 as libc::c_int;
                out
                    |= Mult((argb >> 16 as libc::c_int) as uint8_t, scale)
                        << 16 as libc::c_int;
                *ptr.offset(x as isize) = out;
            }
        }
        x += 1;
        x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPMultRow_C(
    ptr: *mut uint8_t,
    alpha: *const uint8_t,
    mut width: libc::c_int,
    mut inverse: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < width {
        let a: uint32_t = *alpha.offset(x as isize) as uint32_t;
        if a != 255 as libc::c_int as libc::c_uint {
            if a == 0 as libc::c_int as libc::c_uint {
                *ptr.offset(x as isize) = 0 as libc::c_int as uint8_t;
            } else {
                let scale: uint32_t = GetScale(a, inverse);
                *ptr
                    .offset(
                        x as isize,
                    ) = Mult(*ptr.offset(x as isize), scale) as uint8_t;
            }
        }
        x += 1;
        x;
    }
}
#[no_mangle]
pub static mut WebPMultARGBRow: Option::<
    unsafe extern "C" fn(*mut uint32_t, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPMultRow: Option::<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn WebPMultARGBRows(
    mut ptr: *mut uint8_t,
    mut stride: libc::c_int,
    mut width: libc::c_int,
    mut num_rows: libc::c_int,
    mut inverse: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < num_rows {
        WebPMultARGBRow
            .expect("non-null function pointer")(ptr as *mut uint32_t, width, inverse);
        ptr = ptr.offset(stride as isize);
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPMultRows(
    mut ptr: *mut uint8_t,
    mut stride: libc::c_int,
    mut alpha: *const uint8_t,
    mut alpha_stride: libc::c_int,
    mut width: libc::c_int,
    mut num_rows: libc::c_int,
    mut inverse: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < num_rows {
        WebPMultRow.expect("non-null function pointer")(ptr, alpha, width, inverse);
        ptr = ptr.offset(stride as isize);
        alpha = alpha.offset(alpha_stride as isize);
        n += 1;
        n;
    }
}
unsafe extern "C" fn ApplyAlphaMultiply_C(
    mut rgba: *mut uint8_t,
    mut alpha_first: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut stride: libc::c_int,
) {
    loop {
        let fresh0 = h;
        h = h - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        let rgb: *mut uint8_t = rgba
            .offset(
                (if alpha_first != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
                    as isize,
            );
        let alpha: *const uint8_t = rgba
            .offset(
                (if alpha_first != 0 { 0 as libc::c_int } else { 3 as libc::c_int })
                    as isize,
            );
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < w {
            let a: uint32_t = *alpha.offset((4 as libc::c_int * i) as isize) as uint32_t;
            if a != 0xff as libc::c_int as libc::c_uint {
                let mult: uint32_t = a.wrapping_mul(32897 as libc::c_uint);
                *rgb
                    .offset(
                        (4 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = ((*rgb.offset((4 as libc::c_int * i + 0 as libc::c_int) as isize)
                    as libc::c_uint)
                    .wrapping_mul(mult) >> 23 as libc::c_int) as uint8_t;
                *rgb
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = ((*rgb.offset((4 as libc::c_int * i + 1 as libc::c_int) as isize)
                    as libc::c_uint)
                    .wrapping_mul(mult) >> 23 as libc::c_int) as uint8_t;
                *rgb
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = ((*rgb.offset((4 as libc::c_int * i + 2 as libc::c_int) as isize)
                    as libc::c_uint)
                    .wrapping_mul(mult) >> 23 as libc::c_int) as uint8_t;
            }
            i += 1;
            i;
        }
        rgba = rgba.offset(stride as isize);
    };
}
#[inline]
unsafe extern "C" fn dither_hi(mut x: uint8_t) -> uint8_t {
    return (x as libc::c_int & 0xf0 as libc::c_int
        | x as libc::c_int >> 4 as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn dither_lo(mut x: uint8_t) -> uint8_t {
    return (x as libc::c_int & 0xf as libc::c_int
        | (x as libc::c_int) << 4 as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn multiply(mut x: uint8_t, mut m: uint32_t) -> uint8_t {
    return ((x as libc::c_uint).wrapping_mul(m) >> 16 as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn ApplyAlphaMultiply4444_C(
    mut rgba4444: *mut uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut stride: libc::c_int,
    mut rg_byte_pos: libc::c_int,
) {
    loop {
        let fresh1 = h;
        h = h - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < w {
            let rg: uint32_t = *rgba4444
                .offset((2 as libc::c_int * i + rg_byte_pos) as isize) as uint32_t;
            let ba: uint32_t = *rgba4444
                .offset(
                    (2 as libc::c_int * i + (rg_byte_pos ^ 1 as libc::c_int)) as isize,
                ) as uint32_t;
            let a: uint8_t = (ba & 0xf as libc::c_int as libc::c_uint) as uint8_t;
            let mult: uint32_t = (a as libc::c_int * 0x1111 as libc::c_int) as uint32_t;
            let r: uint8_t = multiply(dither_hi(rg as uint8_t), mult);
            let g: uint8_t = multiply(dither_lo(rg as uint8_t), mult);
            let b: uint8_t = multiply(dither_hi(ba as uint8_t), mult);
            *rgba4444
                .offset(
                    (2 as libc::c_int * i + rg_byte_pos) as isize,
                ) = (r as libc::c_int & 0xf0 as libc::c_int
                | g as libc::c_int >> 4 as libc::c_int & 0xf as libc::c_int) as uint8_t;
            *rgba4444
                .offset(
                    (2 as libc::c_int * i + (rg_byte_pos ^ 1 as libc::c_int)) as isize,
                ) = (b as libc::c_int & 0xf0 as libc::c_int | a as libc::c_int)
                as uint8_t;
            i += 1;
            i;
        }
        rgba4444 = rgba4444.offset(stride as isize);
    };
}
unsafe extern "C" fn ApplyAlphaMultiply_16b_C(
    mut rgba4444: *mut uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut stride: libc::c_int,
) {
    ApplyAlphaMultiply4444_C(rgba4444, w, h, stride, 0 as libc::c_int);
}
unsafe extern "C" fn DispatchAlpha_C(
    mut alpha: *const uint8_t,
    mut alpha_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: libc::c_int,
) -> libc::c_int {
    let mut alpha_mask: uint32_t = 0xff as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < height {
        i = 0 as libc::c_int;
        while i < width {
            let alpha_value: uint32_t = *alpha.offset(i as isize) as uint32_t;
            *dst.offset((4 as libc::c_int * i) as isize) = alpha_value as uint8_t;
            alpha_mask &= alpha_value;
            i += 1;
            i;
        }
        alpha = alpha.offset(alpha_stride as isize);
        dst = dst.offset(dst_stride as isize);
        j += 1;
        j;
    }
    return (alpha_mask != 0xff as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn DispatchAlphaToGreen_C(
    mut alpha: *const uint8_t,
    mut alpha_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut dst: *mut uint32_t,
    mut dst_stride: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < height {
        i = 0 as libc::c_int;
        while i < width {
            *dst
                .offset(
                    i as isize,
                ) = ((*alpha.offset(i as isize) as libc::c_int) << 8 as libc::c_int)
                as uint32_t;
            i += 1;
            i;
        }
        alpha = alpha.offset(alpha_stride as isize);
        dst = dst.offset(dst_stride as isize);
        j += 1;
        j;
    }
}
unsafe extern "C" fn ExtractAlpha_C(
    mut argb: *const uint8_t,
    mut argb_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut alpha: *mut uint8_t,
    mut alpha_stride: libc::c_int,
) -> libc::c_int {
    let mut alpha_mask: uint8_t = 0xff as libc::c_int as uint8_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < height {
        i = 0 as libc::c_int;
        while i < width {
            let alpha_value: uint8_t = *argb.offset((4 as libc::c_int * i) as isize);
            *alpha.offset(i as isize) = alpha_value;
            alpha_mask = (alpha_mask as libc::c_int & alpha_value as libc::c_int)
                as uint8_t;
            i += 1;
            i;
        }
        argb = argb.offset(argb_stride as isize);
        alpha = alpha.offset(alpha_stride as isize);
        j += 1;
        j;
    }
    return (alpha_mask as libc::c_int == 0xff as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ExtractGreen_C(
    mut argb: *const uint32_t,
    mut alpha: *mut uint8_t,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *alpha
            .offset(
                i as isize,
            ) = (*argb.offset(i as isize) >> 8 as libc::c_int) as uint8_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn HasAlpha8b_C(
    mut src: *const uint8_t,
    mut length: libc::c_int,
) -> libc::c_int {
    loop {
        let fresh2 = length;
        length = length - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        let fresh3 = src;
        src = src.offset(1);
        if *fresh3 as libc::c_int != 0xff as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn HasAlpha32b_C(
    mut src: *const uint8_t,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    loop {
        let fresh4 = length;
        length = length - 1;
        if !(fresh4 > 0 as libc::c_int) {
            break;
        }
        if *src.offset(x as isize) as libc::c_int != 0xff as libc::c_int {
            return 1 as libc::c_int;
        }
        x += 4 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn AlphaReplace_C(
    mut src: *mut uint32_t,
    mut length: libc::c_int,
    mut color: uint32_t,
) {
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < length {
        if *src.offset(x as isize) >> 24 as libc::c_int
            == 0 as libc::c_int as libc::c_uint
        {
            *src.offset(x as isize) = color;
        }
        x += 1;
        x;
    }
}
#[inline]
unsafe extern "C" fn MakeARGB32(
    mut a: libc::c_int,
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> uint32_t {
    return (a as uint32_t) << 24 as libc::c_int
        | (r << 16 as libc::c_int) as libc::c_uint
        | (g << 8 as libc::c_int) as libc::c_uint | b as libc::c_uint;
}
unsafe extern "C" fn PackRGB_C(
    mut r: *const uint8_t,
    mut g: *const uint8_t,
    mut b: *const uint8_t,
    mut len: libc::c_int,
    mut step: libc::c_int,
    mut out: *mut uint32_t,
) {
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        *out
            .offset(
                i as isize,
            ) = MakeARGB32(
            0xff as libc::c_int,
            *r.offset(offset as isize) as libc::c_int,
            *g.offset(offset as isize) as libc::c_int,
            *b.offset(offset as isize) as libc::c_int,
        );
        offset += step;
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut WebPApplyAlphaMultiply: Option::<
    unsafe extern "C" fn(
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut WebPApplyAlphaMultiply4444: Option::<
    unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPDispatchAlpha: Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
    ) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut WebPDispatchAlphaToGreen: Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut uint32_t,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut WebPExtractAlpha: Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
    ) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut WebPExtractGreen: Option::<
    unsafe extern "C" fn(*const uint32_t, *mut uint8_t, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut WebPPackRGB: Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        *mut uint32_t,
    ) -> (),
> = None;
#[no_mangle]
pub static mut WebPHasAlpha8b: Option::<
    unsafe extern "C" fn(*const uint8_t, libc::c_int) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut WebPHasAlpha32b: Option::<
    unsafe extern "C" fn(*const uint8_t, libc::c_int) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut WebPAlphaReplace: Option::<
    unsafe extern "C" fn(*mut uint32_t, libc::c_int, uint32_t) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn WebPInitAlphaProcessing() {
    static mut WebPInitAlphaProcessing_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut WebPInitAlphaProcessing_body_lock) != 0) {
        WebPInitAlphaProcessing_body();
        pthread_mutex_unlock(&mut WebPInitAlphaProcessing_body_lock);
    }
}
unsafe extern "C" fn WebPInitAlphaProcessing_body() {
    WebPMultARGBRow = Some(
        WebPMultARGBRow_C
            as unsafe extern "C" fn(*mut uint32_t, libc::c_int, libc::c_int) -> (),
    );
    WebPMultRow = Some(
        WebPMultRow_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    WebPApplyAlphaMultiply4444 = Some(
        ApplyAlphaMultiply_16b_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    WebPPackRGB = Some(
        PackRGB_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                *mut uint32_t,
            ) -> (),
    );
    WebPApplyAlphaMultiply = Some(
        ApplyAlphaMultiply_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    WebPDispatchAlpha = Some(
        DispatchAlpha_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
                libc::c_int,
            ) -> libc::c_int,
    );
    WebPDispatchAlphaToGreen = Some(
        DispatchAlphaToGreen_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut uint32_t,
                libc::c_int,
            ) -> (),
    );
    WebPExtractAlpha = Some(
        ExtractAlpha_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
                libc::c_int,
            ) -> libc::c_int,
    );
    WebPExtractGreen = Some(
        ExtractGreen_C
            as unsafe extern "C" fn(*const uint32_t, *mut uint8_t, libc::c_int) -> (),
    );
    WebPHasAlpha8b = Some(
        HasAlpha8b_C as unsafe extern "C" fn(*const uint8_t, libc::c_int) -> libc::c_int,
    );
    WebPHasAlpha32b = Some(
        HasAlpha32b_C as unsafe extern "C" fn(*const uint8_t, libc::c_int) -> libc::c_int,
    );
    WebPAlphaReplace = Some(
        AlphaReplace_C
            as unsafe extern "C" fn(*mut uint32_t, libc::c_int, uint32_t) -> (),
    );
    VP8GetCPUInfo.is_some();
}
