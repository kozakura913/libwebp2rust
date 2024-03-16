use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut SharpYuvUpdateY: Option::<
        unsafe extern "C" fn(
            *const uint16_t,
            *const uint16_t,
            *mut uint16_t,
            libc::c_int,
            libc::c_int,
        ) -> uint64_t,
    >;
    static mut SharpYuvUpdateRGB: Option::<
        unsafe extern "C" fn(
            *const int16_t,
            *const int16_t,
            *mut int16_t,
            libc::c_int,
        ) -> (),
    >;
    static mut SharpYuvFilterRow: Option::<
        unsafe extern "C" fn(
            *const int16_t,
            *const int16_t,
            libc::c_int,
            *const uint16_t,
            *mut uint16_t,
            libc::c_int,
        ) -> (),
    >;
    fn SharpYuvInitDsp();
    fn SharpYuvInitGammaTables();
    fn SharpYuvGammaToLinear(
        v: uint16_t,
        bit_depth: libc::c_int,
        transfer_type: SharpYuvTransferFunctionType,
    ) -> uint32_t;
    fn SharpYuvLinearToGamma(
        value: uint32_t,
        bit_depth: libc::c_int,
        transfer_type: SharpYuvTransferFunctionType,
    ) -> uint16_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [libc::c_int; 4],
    pub rgb_to_u: [libc::c_int; 4],
    pub rgb_to_v: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvOptions {
    pub yuv_matrix: *const SharpYuvConversionMatrix,
    pub transfer_type: SharpYuvTransferFunctionType,
}
pub type SharpYuvTransferFunctionType = libc::c_uint;
pub const kSharpYuvTransferFunctionNum: SharpYuvTransferFunctionType = 19;
pub const kSharpYuvTransferFunctionHlg: SharpYuvTransferFunctionType = 18;
pub const kSharpYuvTransferFunctionSmpte428: SharpYuvTransferFunctionType = 17;
pub const kSharpYuvTransferFunctionSmpte2084: SharpYuvTransferFunctionType = 16;
pub const kSharpYuvTransferFunctionBt2020_12Bit: SharpYuvTransferFunctionType = 15;
pub const kSharpYuvTransferFunctionBt2020_10Bit: SharpYuvTransferFunctionType = 14;
pub const kSharpYuvTransferFunctionSrgb: SharpYuvTransferFunctionType = 13;
pub const kSharpYuvTransferFunctionBt1361: SharpYuvTransferFunctionType = 12;
pub const kSharpYuvTransferFunctionIec61966: SharpYuvTransferFunctionType = 11;
pub const kSharpYuvTransferFunctionLog100_Sqrt10: SharpYuvTransferFunctionType = 10;
pub const kSharpYuvTransferFunctionLog100: SharpYuvTransferFunctionType = 9;
pub const kSharpYuvTransferFunctionLinear: SharpYuvTransferFunctionType = 8;
pub const kSharpYuvTransferFunctionSmpte240: SharpYuvTransferFunctionType = 7;
pub const kSharpYuvTransferFunctionBt601: SharpYuvTransferFunctionType = 6;
pub const kSharpYuvTransferFunctionBt470Bg: SharpYuvTransferFunctionType = 5;
pub const kSharpYuvTransferFunctionBt470M: SharpYuvTransferFunctionType = 4;
pub const kSharpYuvTransferFunctionBt709: SharpYuvTransferFunctionType = 1;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type fixed_y_t = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type fixed_t = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
#[no_mangle]
pub unsafe extern "C" fn SharpYuvGetVersion() -> libc::c_int {
    return (0 as libc::c_int) << 24 as libc::c_int
        | (4 as libc::c_int) << 16 as libc::c_int | 0 as libc::c_int;
}
static mut kNumIterations: libc::c_int = 4 as libc::c_int;
static mut kYuvHalf: libc::c_int = (1 as libc::c_int)
    << 16 as libc::c_int - 1 as libc::c_int;
static mut kMaxBitDepth: libc::c_int = 14 as libc::c_int;
unsafe extern "C" fn GetPrecisionShift(mut rgb_bit_depth: libc::c_int) -> libc::c_int {
    return if rgb_bit_depth + 2 as libc::c_int <= kMaxBitDepth {
        2 as libc::c_int
    } else {
        kMaxBitDepth - rgb_bit_depth
    };
}
unsafe extern "C" fn clip_8b(mut v: fixed_t) -> uint8_t {
    return (if v as libc::c_int & !(0xff as libc::c_int) == 0 {
        v as uint8_t as libc::c_uint
    } else if (v as libc::c_int) < 0 as libc::c_int {
        0 as libc::c_uint
    } else {
        255 as libc::c_uint
    }) as uint8_t;
}
unsafe extern "C" fn clip(mut v: fixed_t, mut max: libc::c_int) -> uint16_t {
    return (if (v as libc::c_int) < 0 as libc::c_int {
        0 as libc::c_int
    } else if v as libc::c_int > max {
        max
    } else {
        v as uint16_t as libc::c_int
    }) as uint16_t;
}
unsafe extern "C" fn clip_bit_depth(
    mut y: libc::c_int,
    mut bit_depth: libc::c_int,
) -> fixed_y_t {
    let max: libc::c_int = ((1 as libc::c_int) << bit_depth) - 1 as libc::c_int;
    return (if y & !max == 0 {
        y as fixed_y_t as libc::c_int
    } else if y < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        max
    }) as fixed_y_t;
}
unsafe extern "C" fn RGBToGray(
    mut r: int64_t,
    mut g: int64_t,
    mut b: int64_t,
) -> libc::c_int {
    let luma: int64_t = 13933 as libc::c_int as libc::c_long * r
        + 46871 as libc::c_int as libc::c_long * g
        + 4732 as libc::c_int as libc::c_long * b + kYuvHalf as libc::c_long;
    return (luma >> 16 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ScaleDown(
    mut a: uint16_t,
    mut b: uint16_t,
    mut c: uint16_t,
    mut d: uint16_t,
    mut rgb_bit_depth: libc::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) -> uint32_t {
    let bit_depth: libc::c_int = rgb_bit_depth + GetPrecisionShift(rgb_bit_depth);
    let A: uint32_t = SharpYuvGammaToLinear(a, bit_depth, transfer_type);
    let B: uint32_t = SharpYuvGammaToLinear(b, bit_depth, transfer_type);
    let C: uint32_t = SharpYuvGammaToLinear(c, bit_depth, transfer_type);
    let D: uint32_t = SharpYuvGammaToLinear(d, bit_depth, transfer_type);
    return SharpYuvLinearToGamma(
        A
            .wrapping_add(B)
            .wrapping_add(C)
            .wrapping_add(D)
            .wrapping_add(2 as libc::c_int as libc::c_uint) >> 2 as libc::c_int,
        bit_depth,
        transfer_type,
    ) as uint32_t;
}
#[inline]
unsafe extern "C" fn UpdateW(
    mut src: *const fixed_y_t,
    mut dst: *mut fixed_y_t,
    mut w: libc::c_int,
    mut rgb_bit_depth: libc::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) {
    let bit_depth: libc::c_int = rgb_bit_depth + GetPrecisionShift(rgb_bit_depth);
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        let R: uint32_t = SharpYuvGammaToLinear(
            *src.offset((0 as libc::c_int * w + i) as isize),
            bit_depth,
            transfer_type,
        );
        let G: uint32_t = SharpYuvGammaToLinear(
            *src.offset((1 as libc::c_int * w + i) as isize),
            bit_depth,
            transfer_type,
        );
        let B: uint32_t = SharpYuvGammaToLinear(
            *src.offset((2 as libc::c_int * w + i) as isize),
            bit_depth,
            transfer_type,
        );
        let Y: uint32_t = RGBToGray(R as int64_t, G as int64_t, B as int64_t)
            as uint32_t;
        *dst.offset(i as isize) = SharpYuvLinearToGamma(Y, bit_depth, transfer_type);
        i += 1;
        if !(i < w) {
            break;
        }
    };
}
unsafe extern "C" fn UpdateChroma(
    mut src1: *const fixed_y_t,
    mut src2: *const fixed_y_t,
    mut dst: *mut fixed_t,
    mut uv_w: libc::c_int,
    mut rgb_bit_depth: libc::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        let r: libc::c_int = ScaleDown(
            *src1.offset((0 as libc::c_int * uv_w + 0 as libc::c_int) as isize),
            *src1.offset((0 as libc::c_int * uv_w + 1 as libc::c_int) as isize),
            *src2.offset((0 as libc::c_int * uv_w + 0 as libc::c_int) as isize),
            *src2.offset((0 as libc::c_int * uv_w + 1 as libc::c_int) as isize),
            rgb_bit_depth,
            transfer_type,
        ) as libc::c_int;
        let g: libc::c_int = ScaleDown(
            *src1.offset((2 as libc::c_int * uv_w + 0 as libc::c_int) as isize),
            *src1.offset((2 as libc::c_int * uv_w + 1 as libc::c_int) as isize),
            *src2.offset((2 as libc::c_int * uv_w + 0 as libc::c_int) as isize),
            *src2.offset((2 as libc::c_int * uv_w + 1 as libc::c_int) as isize),
            rgb_bit_depth,
            transfer_type,
        ) as libc::c_int;
        let b: libc::c_int = ScaleDown(
            *src1.offset((4 as libc::c_int * uv_w + 0 as libc::c_int) as isize),
            *src1.offset((4 as libc::c_int * uv_w + 1 as libc::c_int) as isize),
            *src2.offset((4 as libc::c_int * uv_w + 0 as libc::c_int) as isize),
            *src2.offset((4 as libc::c_int * uv_w + 1 as libc::c_int) as isize),
            rgb_bit_depth,
            transfer_type,
        ) as libc::c_int;
        let W: libc::c_int = RGBToGray(r as int64_t, g as int64_t, b as int64_t);
        *dst.offset((0 as libc::c_int * uv_w) as isize) = (r - W) as fixed_t;
        *dst.offset((1 as libc::c_int * uv_w) as isize) = (g - W) as fixed_t;
        *dst.offset((2 as libc::c_int * uv_w) as isize) = (b - W) as fixed_t;
        dst = dst.offset(1 as libc::c_int as isize);
        src1 = src1.offset(2 as libc::c_int as isize);
        src2 = src2.offset(2 as libc::c_int as isize);
        i += 1;
        if !(i < uv_w) {
            break;
        }
    };
}
unsafe extern "C" fn StoreGray(
    mut rgb: *const fixed_y_t,
    mut y: *mut fixed_y_t,
    mut w: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        *y
            .offset(
                i as isize,
            ) = RGBToGray(
            *rgb.offset((0 as libc::c_int * w + i) as isize) as int64_t,
            *rgb.offset((1 as libc::c_int * w + i) as isize) as int64_t,
            *rgb.offset((2 as libc::c_int * w + i) as isize) as int64_t,
        ) as fixed_y_t;
        i += 1;
        if !(i < w) {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn Filter2(
    mut A: libc::c_int,
    mut B: libc::c_int,
    mut W0: libc::c_int,
    mut bit_depth: libc::c_int,
) -> fixed_y_t {
    let v0: libc::c_int = A * 3 as libc::c_int + B + 2 as libc::c_int
        >> 2 as libc::c_int;
    return clip_bit_depth(v0 + W0, bit_depth);
}
#[inline]
unsafe extern "C" fn Shift(mut v: libc::c_int, mut shift: libc::c_int) -> libc::c_int {
    return if shift >= 0 as libc::c_int { v << shift } else { v >> -shift };
}
unsafe extern "C" fn ImportOneRow(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    mut rgb_step: libc::c_int,
    mut rgb_bit_depth: libc::c_int,
    mut pic_width: libc::c_int,
    dst: *mut fixed_y_t,
) {
    let step: libc::c_int = if rgb_bit_depth > 8 as libc::c_int {
        rgb_step / 2 as libc::c_int
    } else {
        rgb_step
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    let w: libc::c_int = pic_width + 1 as libc::c_int & !(1 as libc::c_int);
    loop {
        let off: libc::c_int = i * step;
        let shift: libc::c_int = GetPrecisionShift(rgb_bit_depth);
        if rgb_bit_depth == 8 as libc::c_int {
            *dst
                .offset(
                    (i + 0 as libc::c_int * w) as isize,
                ) = Shift(*r_ptr.offset(off as isize) as libc::c_int, shift)
                as fixed_y_t;
            *dst
                .offset(
                    (i + 1 as libc::c_int * w) as isize,
                ) = Shift(*g_ptr.offset(off as isize) as libc::c_int, shift)
                as fixed_y_t;
            *dst
                .offset(
                    (i + 2 as libc::c_int * w) as isize,
                ) = Shift(*b_ptr.offset(off as isize) as libc::c_int, shift)
                as fixed_y_t;
        } else {
            *dst
                .offset(
                    (i + 0 as libc::c_int * w) as isize,
                ) = Shift(
                *(r_ptr as *mut uint16_t).offset(off as isize) as libc::c_int,
                shift,
            ) as fixed_y_t;
            *dst
                .offset(
                    (i + 1 as libc::c_int * w) as isize,
                ) = Shift(
                *(g_ptr as *mut uint16_t).offset(off as isize) as libc::c_int,
                shift,
            ) as fixed_y_t;
            *dst
                .offset(
                    (i + 2 as libc::c_int * w) as isize,
                ) = Shift(
                *(b_ptr as *mut uint16_t).offset(off as isize) as libc::c_int,
                shift,
            ) as fixed_y_t;
        }
        i += 1;
        if !(i < pic_width) {
            break;
        }
    }
    if pic_width & 1 as libc::c_int != 0 {
        *dst
            .offset(
                (pic_width + 0 as libc::c_int * w) as isize,
            ) = *dst
            .offset((pic_width + 0 as libc::c_int * w - 1 as libc::c_int) as isize);
        *dst
            .offset(
                (pic_width + 1 as libc::c_int * w) as isize,
            ) = *dst
            .offset((pic_width + 1 as libc::c_int * w - 1 as libc::c_int) as isize);
        *dst
            .offset(
                (pic_width + 2 as libc::c_int * w) as isize,
            ) = *dst
            .offset((pic_width + 2 as libc::c_int * w - 1 as libc::c_int) as isize);
    }
}
unsafe extern "C" fn InterpolateTwoRows(
    best_y: *const fixed_y_t,
    mut prev_uv: *const fixed_t,
    mut cur_uv: *const fixed_t,
    mut next_uv: *const fixed_t,
    mut w: libc::c_int,
    mut out1: *mut fixed_y_t,
    mut out2: *mut fixed_y_t,
    mut rgb_bit_depth: libc::c_int,
) {
    let uv_w: libc::c_int = w >> 1 as libc::c_int;
    let len: libc::c_int = w - 1 as libc::c_int >> 1 as libc::c_int;
    let mut k: libc::c_int = 3 as libc::c_int;
    let bit_depth: libc::c_int = rgb_bit_depth + GetPrecisionShift(rgb_bit_depth);
    loop {
        let fresh0 = k;
        k = k - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        *out1
            .offset(
                0 as libc::c_int as isize,
            ) = Filter2(
            *cur_uv.offset(0 as libc::c_int as isize) as libc::c_int,
            *prev_uv.offset(0 as libc::c_int as isize) as libc::c_int,
            *best_y.offset(0 as libc::c_int as isize) as libc::c_int,
            bit_depth,
        );
        *out2
            .offset(
                0 as libc::c_int as isize,
            ) = Filter2(
            *cur_uv.offset(0 as libc::c_int as isize) as libc::c_int,
            *next_uv.offset(0 as libc::c_int as isize) as libc::c_int,
            *best_y.offset(w as isize) as libc::c_int,
            bit_depth,
        );
        SharpYuvFilterRow
            .expect(
                "non-null function pointer",
            )(
            cur_uv,
            prev_uv,
            len,
            best_y.offset(0 as libc::c_int as isize).offset(1 as libc::c_int as isize),
            out1.offset(1 as libc::c_int as isize),
            bit_depth,
        );
        SharpYuvFilterRow
            .expect(
                "non-null function pointer",
            )(
            cur_uv,
            next_uv,
            len,
            best_y.offset(w as isize).offset(1 as libc::c_int as isize),
            out2.offset(1 as libc::c_int as isize),
            bit_depth,
        );
        if w & 1 as libc::c_int == 0 {
            *out1
                .offset(
                    (w - 1 as libc::c_int) as isize,
                ) = Filter2(
                *cur_uv.offset((uv_w - 1 as libc::c_int) as isize) as libc::c_int,
                *prev_uv.offset((uv_w - 1 as libc::c_int) as isize) as libc::c_int,
                *best_y.offset((w - 1 as libc::c_int + 0 as libc::c_int) as isize)
                    as libc::c_int,
                bit_depth,
            );
            *out2
                .offset(
                    (w - 1 as libc::c_int) as isize,
                ) = Filter2(
                *cur_uv.offset((uv_w - 1 as libc::c_int) as isize) as libc::c_int,
                *next_uv.offset((uv_w - 1 as libc::c_int) as isize) as libc::c_int,
                *best_y.offset((w - 1 as libc::c_int + w) as isize) as libc::c_int,
                bit_depth,
            );
        }
        out1 = out1.offset(w as isize);
        out2 = out2.offset(w as isize);
        prev_uv = prev_uv.offset(uv_w as isize);
        cur_uv = cur_uv.offset(uv_w as isize);
        next_uv = next_uv.offset(uv_w as isize);
    };
}
#[inline]
unsafe extern "C" fn RGBToYUVComponent(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    mut coeffs: *const libc::c_int,
    mut sfix: libc::c_int,
) -> libc::c_int {
    let srounder: libc::c_int = (1 as libc::c_int)
        << 16 as libc::c_int + sfix - 1 as libc::c_int;
    let luma: libc::c_int = *coeffs.offset(0 as libc::c_int as isize) * r
        + *coeffs.offset(1 as libc::c_int as isize) * g
        + *coeffs.offset(2 as libc::c_int as isize) * b
        + *coeffs.offset(3 as libc::c_int as isize) + srounder;
    return luma >> 16 as libc::c_int + sfix;
}
unsafe extern "C" fn ConvertWRGBToYUV(
    mut best_y: *const fixed_y_t,
    mut best_uv: *const fixed_t,
    mut y_ptr: *mut uint8_t,
    mut y_stride: libc::c_int,
    mut u_ptr: *mut uint8_t,
    mut u_stride: libc::c_int,
    mut v_ptr: *mut uint8_t,
    mut v_stride: libc::c_int,
    mut rgb_bit_depth: libc::c_int,
    mut yuv_bit_depth: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut yuv_matrix: *const SharpYuvConversionMatrix,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let best_uv_base: *const fixed_t = best_uv;
    let w: libc::c_int = width + 1 as libc::c_int & !(1 as libc::c_int);
    let h: libc::c_int = height + 1 as libc::c_int & !(1 as libc::c_int);
    let uv_w: libc::c_int = w >> 1 as libc::c_int;
    let uv_h: libc::c_int = h >> 1 as libc::c_int;
    let sfix: libc::c_int = GetPrecisionShift(rgb_bit_depth);
    let yuv_max: libc::c_int = ((1 as libc::c_int) << yuv_bit_depth) - 1 as libc::c_int;
    best_uv = best_uv_base;
    j = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        loop {
            let off: libc::c_int = i >> 1 as libc::c_int;
            let W: libc::c_int = *best_y.offset(i as isize) as libc::c_int;
            let r: libc::c_int = *best_uv
                .offset((off + 0 as libc::c_int * uv_w) as isize) as libc::c_int + W;
            let g: libc::c_int = *best_uv
                .offset((off + 1 as libc::c_int * uv_w) as isize) as libc::c_int + W;
            let b: libc::c_int = *best_uv
                .offset((off + 2 as libc::c_int * uv_w) as isize) as libc::c_int + W;
            let y: libc::c_int = RGBToYUVComponent(
                r,
                g,
                b,
                ((*yuv_matrix).rgb_to_y).as_ptr(),
                sfix,
            );
            if yuv_bit_depth <= 8 as libc::c_int {
                *y_ptr.offset(i as isize) = clip_8b(y as fixed_t);
            } else {
                *(y_ptr as *mut uint16_t)
                    .offset(i as isize) = clip(y as fixed_t, yuv_max);
            }
            i += 1;
            if !(i < width) {
                break;
            }
        }
        best_y = best_y.offset(w as isize);
        best_uv = best_uv
            .offset(((j & 1 as libc::c_int) * 3 as libc::c_int * uv_w) as isize);
        y_ptr = y_ptr.offset(y_stride as isize);
        j += 1;
        if !(j < height) {
            break;
        }
    }
    best_uv = best_uv_base;
    j = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        loop {
            let r_0: libc::c_int = *best_uv
                .offset((i + 0 as libc::c_int * uv_w) as isize) as libc::c_int;
            let g_0: libc::c_int = *best_uv
                .offset((i + 1 as libc::c_int * uv_w) as isize) as libc::c_int;
            let b_0: libc::c_int = *best_uv
                .offset((i + 2 as libc::c_int * uv_w) as isize) as libc::c_int;
            let u: libc::c_int = RGBToYUVComponent(
                r_0,
                g_0,
                b_0,
                ((*yuv_matrix).rgb_to_u).as_ptr(),
                sfix,
            );
            let v: libc::c_int = RGBToYUVComponent(
                r_0,
                g_0,
                b_0,
                ((*yuv_matrix).rgb_to_v).as_ptr(),
                sfix,
            );
            if yuv_bit_depth <= 8 as libc::c_int {
                *u_ptr.offset(i as isize) = clip_8b(u as fixed_t);
                *v_ptr.offset(i as isize) = clip_8b(v as fixed_t);
            } else {
                *(u_ptr as *mut uint16_t)
                    .offset(i as isize) = clip(u as fixed_t, yuv_max);
                *(v_ptr as *mut uint16_t)
                    .offset(i as isize) = clip(v as fixed_t, yuv_max);
            }
            i += 1;
            if !(i < uv_w) {
                break;
            }
        }
        best_uv = best_uv.offset((3 as libc::c_int * uv_w) as isize);
        u_ptr = u_ptr.offset(u_stride as isize);
        v_ptr = v_ptr.offset(v_stride as isize);
        j += 1;
        if !(j < uv_h) {
            break;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn SafeMalloc(
    mut nmemb: uint64_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let total_size: uint64_t = nmemb.wrapping_mul(size);
    if total_size != total_size {
        return 0 as *mut libc::c_void;
    }
    return malloc(total_size);
}
unsafe extern "C" fn DoSharpArgbToYuv(
    mut r_ptr: *const uint8_t,
    mut g_ptr: *const uint8_t,
    mut b_ptr: *const uint8_t,
    mut rgb_step: libc::c_int,
    mut rgb_stride: libc::c_int,
    mut rgb_bit_depth: libc::c_int,
    mut y_ptr: *mut uint8_t,
    mut y_stride: libc::c_int,
    mut u_ptr: *mut uint8_t,
    mut u_stride: libc::c_int,
    mut v_ptr: *mut uint8_t,
    mut v_stride: libc::c_int,
    mut yuv_bit_depth: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut yuv_matrix: *const SharpYuvConversionMatrix,
    mut transfer_type: SharpYuvTransferFunctionType,
) -> libc::c_int {
    let w: libc::c_int = width + 1 as libc::c_int & !(1 as libc::c_int);
    let h: libc::c_int = height + 1 as libc::c_int & !(1 as libc::c_int);
    let uv_w: libc::c_int = w >> 1 as libc::c_int;
    let uv_h: libc::c_int = h >> 1 as libc::c_int;
    let y_bit_depth: libc::c_int = rgb_bit_depth + GetPrecisionShift(rgb_bit_depth);
    let mut prev_diff_y_sum: uint64_t = !(0 as libc::c_int) as uint64_t;
    let mut j: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let tmp_buffer: *mut fixed_y_t = SafeMalloc(
        ((w * 3 as libc::c_int) as uint64_t)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<fixed_y_t>() as libc::c_ulong,
    ) as *mut fixed_y_t;
    let best_y_base: *mut fixed_y_t = SafeMalloc(
        (w as uint64_t).wrapping_mul(h as libc::c_ulong),
        ::core::mem::size_of::<fixed_y_t>() as libc::c_ulong,
    ) as *mut fixed_y_t;
    let target_y_base: *mut fixed_y_t = SafeMalloc(
        (w as uint64_t).wrapping_mul(h as libc::c_ulong),
        ::core::mem::size_of::<fixed_y_t>() as libc::c_ulong,
    ) as *mut fixed_y_t;
    let best_rgb_y: *mut fixed_y_t = SafeMalloc(
        (w as uint64_t).wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<fixed_y_t>() as libc::c_ulong,
    ) as *mut fixed_y_t;
    let best_uv_base: *mut fixed_t = SafeMalloc(
        ((uv_w * 3 as libc::c_int) as uint64_t).wrapping_mul(uv_h as libc::c_ulong),
        ::core::mem::size_of::<fixed_t>() as libc::c_ulong,
    ) as *mut fixed_t;
    let target_uv_base: *mut fixed_t = SafeMalloc(
        ((uv_w * 3 as libc::c_int) as uint64_t).wrapping_mul(uv_h as libc::c_ulong),
        ::core::mem::size_of::<fixed_t>() as libc::c_ulong,
    ) as *mut fixed_t;
    let best_rgb_uv: *mut fixed_t = SafeMalloc(
        ((uv_w * 3 as libc::c_int) as uint64_t)
            .wrapping_mul(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<fixed_t>() as libc::c_ulong,
    ) as *mut fixed_t;
    let mut best_y: *mut fixed_y_t = best_y_base;
    let mut target_y: *mut fixed_y_t = target_y_base;
    let mut best_uv: *mut fixed_t = best_uv_base;
    let mut target_uv: *mut fixed_t = target_uv_base;
    let diff_y_threshold: uint64_t = (3.0f64 * w as libc::c_double * h as libc::c_double)
        as uint64_t;
    let mut ok: libc::c_int = 0;
    if best_y_base.is_null() || best_uv_base.is_null() || target_y_base.is_null()
        || target_uv_base.is_null() || best_rgb_y.is_null() || best_rgb_uv.is_null()
        || tmp_buffer.is_null()
    {
        ok = 0 as libc::c_int;
    } else {
        j = 0 as libc::c_int;
        while j < height {
            let is_last_row: libc::c_int = (j == height - 1 as libc::c_int)
                as libc::c_int;
            let src1: *mut fixed_y_t = tmp_buffer
                .offset((0 as libc::c_int * w) as isize);
            let src2: *mut fixed_y_t = tmp_buffer
                .offset((3 as libc::c_int * w) as isize);
            ImportOneRow(r_ptr, g_ptr, b_ptr, rgb_step, rgb_bit_depth, width, src1);
            if is_last_row == 0 {
                ImportOneRow(
                    r_ptr.offset(rgb_stride as isize),
                    g_ptr.offset(rgb_stride as isize),
                    b_ptr.offset(rgb_stride as isize),
                    rgb_step,
                    rgb_bit_depth,
                    width,
                    src2,
                );
            } else {
                memcpy(
                    src2 as *mut libc::c_void,
                    src1 as *const libc::c_void,
                    ((3 as libc::c_int * w) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<fixed_y_t>() as libc::c_ulong,
                        ),
                );
            }
            StoreGray(src1, best_y.offset(0 as libc::c_int as isize), w);
            StoreGray(src2, best_y.offset(w as isize), w);
            UpdateW(src1, target_y, w, rgb_bit_depth, transfer_type);
            UpdateW(src2, target_y.offset(w as isize), w, rgb_bit_depth, transfer_type);
            UpdateChroma(src1, src2, target_uv, uv_w, rgb_bit_depth, transfer_type);
            memcpy(
                best_uv as *mut libc::c_void,
                target_uv as *const libc::c_void,
                ((3 as libc::c_int * uv_w) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<fixed_t>() as libc::c_ulong),
            );
            best_y = best_y.offset((2 as libc::c_int * w) as isize);
            best_uv = best_uv.offset((3 as libc::c_int * uv_w) as isize);
            target_y = target_y.offset((2 as libc::c_int * w) as isize);
            target_uv = target_uv.offset((3 as libc::c_int * uv_w) as isize);
            r_ptr = r_ptr.offset((2 as libc::c_int * rgb_stride) as isize);
            g_ptr = g_ptr.offset((2 as libc::c_int * rgb_stride) as isize);
            b_ptr = b_ptr.offset((2 as libc::c_int * rgb_stride) as isize);
            j += 2 as libc::c_int;
        }
        iter = 0 as libc::c_int;
        while iter < kNumIterations {
            let mut cur_uv: *const fixed_t = best_uv_base;
            let mut prev_uv: *const fixed_t = best_uv_base;
            let mut diff_y_sum: uint64_t = 0 as libc::c_int as uint64_t;
            best_y = best_y_base;
            best_uv = best_uv_base;
            target_y = target_y_base;
            target_uv = target_uv_base;
            j = 0 as libc::c_int;
            loop {
                let src1_0: *mut fixed_y_t = tmp_buffer
                    .offset((0 as libc::c_int * w) as isize);
                let src2_0: *mut fixed_y_t = tmp_buffer
                    .offset((3 as libc::c_int * w) as isize);
                let next_uv: *const fixed_t = cur_uv
                    .offset(
                        (if j < h - 2 as libc::c_int {
                            3 as libc::c_int * uv_w
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
                InterpolateTwoRows(
                    best_y,
                    prev_uv,
                    cur_uv,
                    next_uv,
                    w,
                    src1_0,
                    src2_0,
                    rgb_bit_depth,
                );
                prev_uv = cur_uv;
                cur_uv = next_uv;
                UpdateW(
                    src1_0,
                    best_rgb_y.offset((0 as libc::c_int * w) as isize),
                    w,
                    rgb_bit_depth,
                    transfer_type,
                );
                UpdateW(
                    src2_0,
                    best_rgb_y.offset((1 as libc::c_int * w) as isize),
                    w,
                    rgb_bit_depth,
                    transfer_type,
                );
                UpdateChroma(
                    src1_0,
                    src2_0,
                    best_rgb_uv,
                    uv_w,
                    rgb_bit_depth,
                    transfer_type,
                );
                diff_y_sum = (diff_y_sum as libc::c_ulong)
                    .wrapping_add(
                        SharpYuvUpdateY
                            .expect(
                                "non-null function pointer",
                            )(
                            target_y,
                            best_rgb_y,
                            best_y,
                            2 as libc::c_int * w,
                            y_bit_depth,
                        ),
                    ) as uint64_t as uint64_t;
                SharpYuvUpdateRGB
                    .expect(
                        "non-null function pointer",
                    )(target_uv, best_rgb_uv, best_uv, 3 as libc::c_int * uv_w);
                best_y = best_y.offset((2 as libc::c_int * w) as isize);
                best_uv = best_uv.offset((3 as libc::c_int * uv_w) as isize);
                target_y = target_y.offset((2 as libc::c_int * w) as isize);
                target_uv = target_uv.offset((3 as libc::c_int * uv_w) as isize);
                j += 2 as libc::c_int;
                if !(j < h) {
                    break;
                }
            }
            if iter > 0 as libc::c_int {
                if diff_y_sum < diff_y_threshold {
                    break;
                }
                if diff_y_sum > prev_diff_y_sum {
                    break;
                }
            }
            prev_diff_y_sum = diff_y_sum;
            iter += 1;
            iter;
        }
        ok = ConvertWRGBToYUV(
            best_y_base,
            best_uv_base,
            y_ptr,
            y_stride,
            u_ptr,
            u_stride,
            v_ptr,
            v_stride,
            rgb_bit_depth,
            yuv_bit_depth,
            width,
            height,
            yuv_matrix,
        );
    }
    free(best_y_base as *mut libc::c_void);
    free(best_uv_base as *mut libc::c_void);
    free(target_y_base as *mut libc::c_void);
    free(target_uv_base as *mut libc::c_void);
    free(best_rgb_y as *mut libc::c_void);
    free(best_rgb_uv as *mut libc::c_void);
    free(tmp_buffer as *mut libc::c_void);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvInit() {
    static mut sharpyuv_lock: pthread_mutex_t = pthread_mutex_t {
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
    if pthread_mutex_lock(&mut sharpyuv_lock) != 0 {
        return;
    }
    SharpYuvInitDsp();
    SharpYuvInitGammaTables();
    pthread_mutex_unlock(&mut sharpyuv_lock);
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvConvert(
    mut r_ptr: *const libc::c_void,
    mut g_ptr: *const libc::c_void,
    mut b_ptr: *const libc::c_void,
    mut rgb_step: libc::c_int,
    mut rgb_stride: libc::c_int,
    mut rgb_bit_depth: libc::c_int,
    mut y_ptr: *mut libc::c_void,
    mut y_stride: libc::c_int,
    mut u_ptr: *mut libc::c_void,
    mut u_stride: libc::c_int,
    mut v_ptr: *mut libc::c_void,
    mut v_stride: libc::c_int,
    mut yuv_bit_depth: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut yuv_matrix: *const SharpYuvConversionMatrix,
) -> libc::c_int {
    let mut options: SharpYuvOptions = SharpYuvOptions {
        yuv_matrix: 0 as *const SharpYuvConversionMatrix,
        transfer_type: 0 as SharpYuvTransferFunctionType,
    };
    options.yuv_matrix = yuv_matrix;
    options.transfer_type = kSharpYuvTransferFunctionSrgb;
    return SharpYuvConvertWithOptions(
        r_ptr,
        g_ptr,
        b_ptr,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_stride,
        u_ptr,
        u_stride,
        v_ptr,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &mut options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvOptionsInitInternal(
    mut yuv_matrix: *const SharpYuvConversionMatrix,
    mut options: *mut SharpYuvOptions,
    mut version: libc::c_int,
) -> libc::c_int {
    let major: libc::c_int = version >> 24 as libc::c_int;
    let minor: libc::c_int = version >> 16 as libc::c_int & 0xff as libc::c_int;
    if options.is_null() || yuv_matrix.is_null()
        || major == 0 as libc::c_int && major == 0 as libc::c_int
            && minor != 4 as libc::c_int || major != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*options).yuv_matrix = yuv_matrix;
    (*options).transfer_type = kSharpYuvTransferFunctionSrgb;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvConvertWithOptions(
    mut r_ptr: *const libc::c_void,
    mut g_ptr: *const libc::c_void,
    mut b_ptr: *const libc::c_void,
    mut rgb_step: libc::c_int,
    mut rgb_stride: libc::c_int,
    mut rgb_bit_depth: libc::c_int,
    mut y_ptr: *mut libc::c_void,
    mut y_stride: libc::c_int,
    mut u_ptr: *mut libc::c_void,
    mut u_stride: libc::c_int,
    mut v_ptr: *mut libc::c_void,
    mut v_stride: libc::c_int,
    mut yuv_bit_depth: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut options: *const SharpYuvOptions,
) -> libc::c_int {
    let mut yuv_matrix: *const SharpYuvConversionMatrix = (*options).yuv_matrix;
    let mut transfer_type: SharpYuvTransferFunctionType = (*options).transfer_type;
    let mut scaled_matrix: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
        rgb_to_y: [0; 4],
        rgb_to_u: [0; 4],
        rgb_to_v: [0; 4],
    };
    let rgb_max: libc::c_int = ((1 as libc::c_int) << rgb_bit_depth) - 1 as libc::c_int;
    let rgb_round: libc::c_int = (1 as libc::c_int) << rgb_bit_depth - 1 as libc::c_int;
    let yuv_max: libc::c_int = ((1 as libc::c_int) << yuv_bit_depth) - 1 as libc::c_int;
    let sfix: libc::c_int = GetPrecisionShift(rgb_bit_depth);
    if width < 1 as libc::c_int || height < 1 as libc::c_int
        || width == 2147483647 as libc::c_int || height == 2147483647 as libc::c_int
        || r_ptr.is_null() || g_ptr.is_null() || b_ptr.is_null() || y_ptr.is_null()
        || u_ptr.is_null() || v_ptr.is_null()
    {
        return 0 as libc::c_int;
    }
    if rgb_bit_depth != 8 as libc::c_int && rgb_bit_depth != 10 as libc::c_int
        && rgb_bit_depth != 12 as libc::c_int && rgb_bit_depth != 16 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if yuv_bit_depth != 8 as libc::c_int && yuv_bit_depth != 10 as libc::c_int
        && yuv_bit_depth != 12 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if rgb_bit_depth > 8 as libc::c_int
        && (rgb_step % 2 as libc::c_int != 0 as libc::c_int
            || rgb_stride % 2 as libc::c_int != 0 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if yuv_bit_depth > 8 as libc::c_int
        && (y_stride % 2 as libc::c_int != 0 as libc::c_int
            || u_stride % 2 as libc::c_int != 0 as libc::c_int
            || v_stride % 2 as libc::c_int != 0 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    SharpYuvInit();
    if rgb_bit_depth == yuv_bit_depth {
        memcpy(
            &mut scaled_matrix as *mut SharpYuvConversionMatrix as *mut libc::c_void,
            yuv_matrix as *const libc::c_void,
            ::core::mem::size_of::<SharpYuvConversionMatrix>() as libc::c_ulong,
        );
    } else {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            scaled_matrix
                .rgb_to_y[i
                as usize] = ((*yuv_matrix).rgb_to_y[i as usize] * yuv_max + rgb_round)
                / rgb_max;
            scaled_matrix
                .rgb_to_u[i
                as usize] = ((*yuv_matrix).rgb_to_u[i as usize] * yuv_max + rgb_round)
                / rgb_max;
            scaled_matrix
                .rgb_to_v[i
                as usize] = ((*yuv_matrix).rgb_to_v[i as usize] * yuv_max + rgb_round)
                / rgb_max;
            i += 1;
            i;
        }
    }
    scaled_matrix
        .rgb_to_y[3 as libc::c_int
        as usize] = Shift((*yuv_matrix).rgb_to_y[3 as libc::c_int as usize], sfix);
    scaled_matrix
        .rgb_to_u[3 as libc::c_int
        as usize] = Shift((*yuv_matrix).rgb_to_u[3 as libc::c_int as usize], sfix);
    scaled_matrix
        .rgb_to_v[3 as libc::c_int
        as usize] = Shift((*yuv_matrix).rgb_to_v[3 as libc::c_int as usize], sfix);
    return DoSharpArgbToYuv(
        r_ptr as *const uint8_t,
        g_ptr as *const uint8_t,
        b_ptr as *const uint8_t,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr as *mut uint8_t,
        y_stride,
        u_ptr as *mut uint8_t,
        u_stride,
        v_ptr as *mut uint8_t,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &mut scaled_matrix,
        transfer_type,
    );
}
