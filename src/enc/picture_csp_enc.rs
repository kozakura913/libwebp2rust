use ::libc;

use crate::src::utils::types::{WebPEncodingError, WebPPicture, VP8_ENC_ERROR_INVALID_CONFIGURATION, VP8_ENC_ERROR_NULL_PARAMETER, VP8_ENC_ERROR_OUT_OF_MEMORY};
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn SharpYuvConvert(
        r_ptr: *const libc::c_void,
        g_ptr: *const libc::c_void,
        b_ptr: *const libc::c_void,
        rgb_step: libc::c_int,
        rgb_stride: libc::c_int,
        rgb_bit_depth: libc::c_int,
        y_ptr: *mut libc::c_void,
        y_stride: libc::c_int,
        u_ptr: *mut libc::c_void,
        u_stride: libc::c_int,
        v_ptr: *mut libc::c_void,
        v_stride: libc::c_int,
        yuv_bit_depth: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        yuv_matrix: *const SharpYuvConversionMatrix,
    ) -> libc::c_int;
    fn SharpYuvGetConversionMatrix(
        matrix_type: SharpYuvMatrixType,
    ) -> *const SharpYuvConversionMatrix;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn WebPGetLinePairConverter(alpha_is_last: libc::c_int) -> WebPUpsampleLinePairFunc;
    static mut WebPConvertRGBA32ToUV: Option::<
        unsafe extern "C" fn(
            *const uint16_t,
            *mut uint8_t,
            *mut uint8_t,
            libc::c_int,
        ) -> (),
    >;
    static mut WebPConvertRGB24ToY: Option::<
        unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
    >;
    static mut WebPConvertBGR24ToY: Option::<
        unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
    >;
    fn WebPInitConvertARGBToYUV();
    static mut WebPExtractAlpha: Option::<
        unsafe extern "C" fn(
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >;
    static mut WebPPackRGB: Option::<
        unsafe extern "C" fn(
            *const uint8_t,
            *const uint8_t,
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            *mut uint32_t,
        ) -> (),
    >;
    static mut WebPHasAlpha8b: Option::<
        unsafe extern "C" fn(*const uint8_t, libc::c_int) -> libc::c_int,
    >;
    static mut WebPHasAlpha32b: Option::<
        unsafe extern "C" fn(*const uint8_t, libc::c_int) -> libc::c_int,
    >;
    fn WebPInitAlphaProcessing();
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPPictureAlloc(picture: *mut WebPPicture) -> libc::c_int;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn WebPPictureAllocYUVA(picture: *mut WebPPicture) -> libc::c_int;
    fn WebPPictureAllocARGB(picture: *mut WebPPicture) -> libc::c_int;
    fn VP8InitRandom(rg: *mut VP8Random, dithering: libc::c_float);
    static mut VP8LConvertBGRAToRGBA: VP8LConvertFunc;
    fn VP8LDspInit();
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [libc::c_int; 4],
    pub rgb_to_u: [libc::c_int; 4],
    pub rgb_to_v: [libc::c_int; 4],
}
pub type SharpYuvMatrixType = libc::c_uint;
pub const kSharpYuvMatrixNum: SharpYuvMatrixType = 5;
pub const kSharpYuvMatrixRec709Full: SharpYuvMatrixType = 4;
pub const kSharpYuvMatrixRec709Limited: SharpYuvMatrixType = 3;
pub const kSharpYuvMatrixRec601Full: SharpYuvMatrixType = 2;
pub const kSharpYuvMatrixRec601Limited: SharpYuvMatrixType = 1;
pub const kSharpYuvMatrixWebp: SharpYuvMatrixType = 0;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAuxStats {
    pub coded_size: libc::c_int,
    pub PSNR: [libc::c_float; 5],
    pub block_count: [libc::c_int; 3],
    pub header_bytes: [libc::c_int; 2],
    pub residual_bytes: [[libc::c_int; 4]; 3],
    pub segment_size: [libc::c_int; 4],
    pub segment_quant: [libc::c_int; 4],
    pub segment_level: [libc::c_int; 4],
    pub alpha_data_size: libc::c_int,
    pub layer_data_size: libc::c_int,
    pub lossless_features: uint32_t,
    pub histogram_bits: libc::c_int,
    pub transform_bits: libc::c_int,
    pub cache_bits: libc::c_int,
    pub palette_size: libc::c_int,
    pub lossless_size: libc::c_int,
    pub lossless_hdr_size: libc::c_int,
    pub lossless_data_size: libc::c_int,
    pub pad: [uint32_t; 2],
}
pub type WebPWriterFunction = Option::<
    unsafe extern "C" fn(*const uint8_t, size_t, *const WebPPicture) -> libc::c_int,
>;
pub type WebPEncCSP = libc::c_uint;
pub const WEBP_CSP_ALPHA_BIT: WebPEncCSP = 4;
pub const WEBP_CSP_UV_MASK: WebPEncCSP = 3;
pub const WEBP_YUV420A: WebPEncCSP = 4;
pub const WEBP_YUV420: WebPEncCSP = 0;
pub type VP8LConvertFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, libc::c_int, *mut uint8_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Random {
    pub index1_: libc::c_int,
    pub index2_: libc::c_int,
    pub tab_: [uint32_t; 55],
    pub amp_: libc::c_int,
}
pub const YUV_FIX: C2RustUnnamed_0 = 16;
pub const YUV_HALF: C2RustUnnamed_0 = 32768;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const YUV_MASK2: C2RustUnnamed_0 = 16383;
pub const YUV_FIX2: C2RustUnnamed_0 = 6;
#[inline]
unsafe extern "C" fn VP8RandomBits2(
    rg: *mut VP8Random,
    mut num_bits: libc::c_int,
    mut amp: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    diff = ((*rg).tab_[(*rg).index1_ as usize])
        .wrapping_sub((*rg).tab_[(*rg).index2_ as usize]) as libc::c_int;
    if diff < 0 as libc::c_int {
        diff = (diff as libc::c_uint)
            .wrapping_add((1 as libc::c_uint) << 31 as libc::c_int) as libc::c_int
            as libc::c_int;
    }
    (*rg).tab_[(*rg).index1_ as usize] = diff as uint32_t;
    (*rg).index1_ += 1;
    if (*rg).index1_ == 55 as libc::c_int {
        (*rg).index1_ = 0 as libc::c_int;
    }
    (*rg).index2_ += 1;
    if (*rg).index2_ == 55 as libc::c_int {
        (*rg).index2_ = 0 as libc::c_int;
    }
    diff = ((diff as uint32_t) << 1 as libc::c_int) as libc::c_int
        >> 32 as libc::c_int - num_bits;
    diff = diff * amp >> 8 as libc::c_int;
    diff += (1 as libc::c_int) << num_bits - 1 as libc::c_int;
    return diff;
}
#[inline]
unsafe extern "C" fn VP8RandomBits(
    rg: *mut VP8Random,
    mut num_bits: libc::c_int,
) -> libc::c_int {
    return VP8RandomBits2(rg, num_bits, (*rg).amp_);
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
unsafe extern "C" fn CheckNonOpaque(
    mut alpha: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut x_step: libc::c_int,
    mut y_step: libc::c_int,
) -> libc::c_int {
    if alpha.is_null() {
        return 0 as libc::c_int;
    }
    WebPInitAlphaProcessing();
    if x_step == 1 as libc::c_int {
        loop {
            let fresh0 = height;
            height = height - 1;
            if !(fresh0 > 0 as libc::c_int) {
                break;
            }
            if WebPHasAlpha8b.expect("non-null function pointer")(alpha, width) != 0 {
                return 1 as libc::c_int;
            }
            alpha = alpha.offset(y_step as isize);
        }
    } else {
        loop {
            let fresh1 = height;
            height = height - 1;
            if !(fresh1 > 0 as libc::c_int) {
                break;
            }
            if WebPHasAlpha32b.expect("non-null function pointer")(alpha, width) != 0 {
                return 1 as libc::c_int;
            }
            alpha = alpha.offset(y_step as isize);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureHasTransparency(
    mut picture: *const WebPPicture,
) -> libc::c_int {
    if picture.is_null() {
        return 0 as libc::c_int;
    }
    if (*picture).use_argb != 0 {
        if !((*picture).argb).is_null() {
            return CheckNonOpaque(
                ((*picture).argb as *const uint8_t)
                    .offset((3 as libc::c_int - 0 as libc::c_int) as isize),
                (*picture).width,
                (*picture).height,
                4 as libc::c_int,
                ((*picture).argb_stride as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    as libc::c_int,
            );
        }
        return 0 as libc::c_int;
    }
    return CheckNonOpaque(
        (*picture).a,
        (*picture).width,
        (*picture).height,
        1 as libc::c_int,
        (*picture).a_stride,
    );
}
static mut kGamma: libc::c_double = 0.80f64;
static mut kGammaScale: libc::c_int = ((1 as libc::c_int) << 12 as libc::c_int)
    - 1 as libc::c_int;
static mut kGammaTabScale: libc::c_int = (1 as libc::c_int) << 7 as libc::c_int;
static mut kGammaTabRounder: libc::c_int = (1 as libc::c_int) << 7 as libc::c_int
    >> 1 as libc::c_int;
static mut kLinearToGammaTab: [libc::c_int; 33] = [0; 33];
static mut kGammaToLinearTab: [uint16_t; 256] = [0; 256];
static mut kGammaTablesOk: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn InitGammaTables() {
    static mut InitGammaTables_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut InitGammaTables_body_lock) != 0) {
        InitGammaTables_body();
        pthread_mutex_unlock(&mut InitGammaTables_body_lock);
    }
}
unsafe extern "C" fn InitGammaTables_body() {
    if kGammaTablesOk == 0 {
        let mut v: libc::c_int = 0;
        let scale: libc::c_double = ((1 as libc::c_int) << 7 as libc::c_int)
            as libc::c_double / kGammaScale as libc::c_double;
        let norm: libc::c_double = 1.0f64 / 255.0f64;
        v = 0 as libc::c_int;
        while v <= 255 as libc::c_int {
            kGammaToLinearTab[v
                as usize] = (pow(norm * v as libc::c_double, kGamma)
                * kGammaScale as libc::c_double + 0.5f64) as uint16_t;
            v += 1;
            v;
        }
        v = 0 as libc::c_int;
        while v <= (1 as libc::c_int) << 12 as libc::c_int - 7 as libc::c_int {
            kLinearToGammaTab[v
                as usize] = (255.0f64 * pow(scale * v as libc::c_double, 1.0f64 / kGamma)
                + 0.5f64) as libc::c_int;
            v += 1;
            v;
        }
        ::core::ptr::write_volatile(
            &mut kGammaTablesOk as *mut libc::c_int,
            1 as libc::c_int,
        );
    }
}
#[inline]
unsafe extern "C" fn GammaToLinear(mut v: uint8_t) -> uint32_t {
    return kGammaToLinearTab[v as usize] as uint32_t;
}
#[inline]
unsafe extern "C" fn Interpolate(mut v: libc::c_int) -> libc::c_int {
    let tab_pos: libc::c_int = v >> 7 as libc::c_int + 2 as libc::c_int;
    let x: libc::c_int = v & (kGammaTabScale << 2 as libc::c_int) - 1 as libc::c_int;
    let v0: libc::c_int = kLinearToGammaTab[tab_pos as usize];
    let v1: libc::c_int = kLinearToGammaTab[(tab_pos + 1 as libc::c_int) as usize];
    let y: libc::c_int = v1 * x + v0 * ((kGammaTabScale << 2 as libc::c_int) - x);
    return y;
}
#[inline]
unsafe extern "C" fn LinearToGamma(
    mut base_value: uint32_t,
    mut shift: libc::c_int,
) -> libc::c_int {
    let y: libc::c_int = Interpolate((base_value << shift) as libc::c_int);
    return y + kGammaTabRounder >> 7 as libc::c_int;
}
unsafe extern "C" fn RGBToY(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    rg: *mut VP8Random,
) -> libc::c_int {
    return if rg.is_null() {
        VP8RGBToY(r, g, b, YUV_HALF as libc::c_int)
    } else {
        VP8RGBToY(r, g, b, VP8RandomBits(rg, YUV_FIX as libc::c_int))
    };
}
unsafe extern "C" fn RGBToU(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    rg: *mut VP8Random,
) -> libc::c_int {
    return if rg.is_null() {
        VP8RGBToU(r, g, b, (YUV_HALF as libc::c_int) << 2 as libc::c_int)
    } else {
        VP8RGBToU(r, g, b, VP8RandomBits(rg, YUV_FIX as libc::c_int + 2 as libc::c_int))
    };
}
unsafe extern "C" fn RGBToV(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
    rg: *mut VP8Random,
) -> libc::c_int {
    return if rg.is_null() {
        VP8RGBToV(r, g, b, (YUV_HALF as libc::c_int) << 2 as libc::c_int)
    } else {
        VP8RGBToV(r, g, b, VP8RandomBits(rg, YUV_FIX as libc::c_int + 2 as libc::c_int))
    };
}
static mut kMinDimensionIterativeConversion: libc::c_int = 4 as libc::c_int;
unsafe extern "C" fn PreprocessARGB(
    mut r_ptr: *const uint8_t,
    mut g_ptr: *const uint8_t,
    mut b_ptr: *const uint8_t,
    mut step: libc::c_int,
    mut rgb_stride: libc::c_int,
    picture: *mut WebPPicture,
) -> libc::c_int {
    let ok: libc::c_int = SharpYuvConvert(
        r_ptr as *const libc::c_void,
        g_ptr as *const libc::c_void,
        b_ptr as *const libc::c_void,
        step,
        rgb_stride,
        8 as libc::c_int,
        (*picture).y as *mut libc::c_void,
        (*picture).y_stride,
        (*picture).u as *mut libc::c_void,
        (*picture).uv_stride,
        (*picture).v as *mut libc::c_void,
        (*picture).uv_stride,
        8 as libc::c_int,
        (*picture).width,
        (*picture).height,
        SharpYuvGetConversionMatrix(kSharpYuvMatrixWebp),
    );
    if ok == 0 {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
static mut kAlphaFix: libc::c_int = 19 as libc::c_int;
static mut kInvAlpha: [uint32_t; 1021] = [
    0 as libc::c_int as uint32_t,
    524288 as libc::c_int as uint32_t,
    262144 as libc::c_int as uint32_t,
    174762 as libc::c_int as uint32_t,
    131072 as libc::c_int as uint32_t,
    104857 as libc::c_int as uint32_t,
    87381 as libc::c_int as uint32_t,
    74898 as libc::c_int as uint32_t,
    65536 as libc::c_int as uint32_t,
    58254 as libc::c_int as uint32_t,
    52428 as libc::c_int as uint32_t,
    47662 as libc::c_int as uint32_t,
    43690 as libc::c_int as uint32_t,
    40329 as libc::c_int as uint32_t,
    37449 as libc::c_int as uint32_t,
    34952 as libc::c_int as uint32_t,
    32768 as libc::c_int as uint32_t,
    30840 as libc::c_int as uint32_t,
    29127 as libc::c_int as uint32_t,
    27594 as libc::c_int as uint32_t,
    26214 as libc::c_int as uint32_t,
    24966 as libc::c_int as uint32_t,
    23831 as libc::c_int as uint32_t,
    22795 as libc::c_int as uint32_t,
    21845 as libc::c_int as uint32_t,
    20971 as libc::c_int as uint32_t,
    20164 as libc::c_int as uint32_t,
    19418 as libc::c_int as uint32_t,
    18724 as libc::c_int as uint32_t,
    18078 as libc::c_int as uint32_t,
    17476 as libc::c_int as uint32_t,
    16912 as libc::c_int as uint32_t,
    16384 as libc::c_int as uint32_t,
    15887 as libc::c_int as uint32_t,
    15420 as libc::c_int as uint32_t,
    14979 as libc::c_int as uint32_t,
    14563 as libc::c_int as uint32_t,
    14169 as libc::c_int as uint32_t,
    13797 as libc::c_int as uint32_t,
    13443 as libc::c_int as uint32_t,
    13107 as libc::c_int as uint32_t,
    12787 as libc::c_int as uint32_t,
    12483 as libc::c_int as uint32_t,
    12192 as libc::c_int as uint32_t,
    11915 as libc::c_int as uint32_t,
    11650 as libc::c_int as uint32_t,
    11397 as libc::c_int as uint32_t,
    11155 as libc::c_int as uint32_t,
    10922 as libc::c_int as uint32_t,
    10699 as libc::c_int as uint32_t,
    10485 as libc::c_int as uint32_t,
    10280 as libc::c_int as uint32_t,
    10082 as libc::c_int as uint32_t,
    9892 as libc::c_int as uint32_t,
    9709 as libc::c_int as uint32_t,
    9532 as libc::c_int as uint32_t,
    9362 as libc::c_int as uint32_t,
    9198 as libc::c_int as uint32_t,
    9039 as libc::c_int as uint32_t,
    8886 as libc::c_int as uint32_t,
    8738 as libc::c_int as uint32_t,
    8594 as libc::c_int as uint32_t,
    8456 as libc::c_int as uint32_t,
    8322 as libc::c_int as uint32_t,
    8192 as libc::c_int as uint32_t,
    8065 as libc::c_int as uint32_t,
    7943 as libc::c_int as uint32_t,
    7825 as libc::c_int as uint32_t,
    7710 as libc::c_int as uint32_t,
    7598 as libc::c_int as uint32_t,
    7489 as libc::c_int as uint32_t,
    7384 as libc::c_int as uint32_t,
    7281 as libc::c_int as uint32_t,
    7182 as libc::c_int as uint32_t,
    7084 as libc::c_int as uint32_t,
    6990 as libc::c_int as uint32_t,
    6898 as libc::c_int as uint32_t,
    6808 as libc::c_int as uint32_t,
    6721 as libc::c_int as uint32_t,
    6636 as libc::c_int as uint32_t,
    6553 as libc::c_int as uint32_t,
    6472 as libc::c_int as uint32_t,
    6393 as libc::c_int as uint32_t,
    6316 as libc::c_int as uint32_t,
    6241 as libc::c_int as uint32_t,
    6168 as libc::c_int as uint32_t,
    6096 as libc::c_int as uint32_t,
    6026 as libc::c_int as uint32_t,
    5957 as libc::c_int as uint32_t,
    5890 as libc::c_int as uint32_t,
    5825 as libc::c_int as uint32_t,
    5761 as libc::c_int as uint32_t,
    5698 as libc::c_int as uint32_t,
    5637 as libc::c_int as uint32_t,
    5577 as libc::c_int as uint32_t,
    5518 as libc::c_int as uint32_t,
    5461 as libc::c_int as uint32_t,
    5405 as libc::c_int as uint32_t,
    5349 as libc::c_int as uint32_t,
    5295 as libc::c_int as uint32_t,
    5242 as libc::c_int as uint32_t,
    5190 as libc::c_int as uint32_t,
    5140 as libc::c_int as uint32_t,
    5090 as libc::c_int as uint32_t,
    5041 as libc::c_int as uint32_t,
    4993 as libc::c_int as uint32_t,
    4946 as libc::c_int as uint32_t,
    4899 as libc::c_int as uint32_t,
    4854 as libc::c_int as uint32_t,
    4809 as libc::c_int as uint32_t,
    4766 as libc::c_int as uint32_t,
    4723 as libc::c_int as uint32_t,
    4681 as libc::c_int as uint32_t,
    4639 as libc::c_int as uint32_t,
    4599 as libc::c_int as uint32_t,
    4559 as libc::c_int as uint32_t,
    4519 as libc::c_int as uint32_t,
    4481 as libc::c_int as uint32_t,
    4443 as libc::c_int as uint32_t,
    4405 as libc::c_int as uint32_t,
    4369 as libc::c_int as uint32_t,
    4332 as libc::c_int as uint32_t,
    4297 as libc::c_int as uint32_t,
    4262 as libc::c_int as uint32_t,
    4228 as libc::c_int as uint32_t,
    4194 as libc::c_int as uint32_t,
    4161 as libc::c_int as uint32_t,
    4128 as libc::c_int as uint32_t,
    4096 as libc::c_int as uint32_t,
    4064 as libc::c_int as uint32_t,
    4032 as libc::c_int as uint32_t,
    4002 as libc::c_int as uint32_t,
    3971 as libc::c_int as uint32_t,
    3942 as libc::c_int as uint32_t,
    3912 as libc::c_int as uint32_t,
    3883 as libc::c_int as uint32_t,
    3855 as libc::c_int as uint32_t,
    3826 as libc::c_int as uint32_t,
    3799 as libc::c_int as uint32_t,
    3771 as libc::c_int as uint32_t,
    3744 as libc::c_int as uint32_t,
    3718 as libc::c_int as uint32_t,
    3692 as libc::c_int as uint32_t,
    3666 as libc::c_int as uint32_t,
    3640 as libc::c_int as uint32_t,
    3615 as libc::c_int as uint32_t,
    3591 as libc::c_int as uint32_t,
    3566 as libc::c_int as uint32_t,
    3542 as libc::c_int as uint32_t,
    3518 as libc::c_int as uint32_t,
    3495 as libc::c_int as uint32_t,
    3472 as libc::c_int as uint32_t,
    3449 as libc::c_int as uint32_t,
    3426 as libc::c_int as uint32_t,
    3404 as libc::c_int as uint32_t,
    3382 as libc::c_int as uint32_t,
    3360 as libc::c_int as uint32_t,
    3339 as libc::c_int as uint32_t,
    3318 as libc::c_int as uint32_t,
    3297 as libc::c_int as uint32_t,
    3276 as libc::c_int as uint32_t,
    3256 as libc::c_int as uint32_t,
    3236 as libc::c_int as uint32_t,
    3216 as libc::c_int as uint32_t,
    3196 as libc::c_int as uint32_t,
    3177 as libc::c_int as uint32_t,
    3158 as libc::c_int as uint32_t,
    3139 as libc::c_int as uint32_t,
    3120 as libc::c_int as uint32_t,
    3102 as libc::c_int as uint32_t,
    3084 as libc::c_int as uint32_t,
    3066 as libc::c_int as uint32_t,
    3048 as libc::c_int as uint32_t,
    3030 as libc::c_int as uint32_t,
    3013 as libc::c_int as uint32_t,
    2995 as libc::c_int as uint32_t,
    2978 as libc::c_int as uint32_t,
    2962 as libc::c_int as uint32_t,
    2945 as libc::c_int as uint32_t,
    2928 as libc::c_int as uint32_t,
    2912 as libc::c_int as uint32_t,
    2896 as libc::c_int as uint32_t,
    2880 as libc::c_int as uint32_t,
    2864 as libc::c_int as uint32_t,
    2849 as libc::c_int as uint32_t,
    2833 as libc::c_int as uint32_t,
    2818 as libc::c_int as uint32_t,
    2803 as libc::c_int as uint32_t,
    2788 as libc::c_int as uint32_t,
    2774 as libc::c_int as uint32_t,
    2759 as libc::c_int as uint32_t,
    2744 as libc::c_int as uint32_t,
    2730 as libc::c_int as uint32_t,
    2716 as libc::c_int as uint32_t,
    2702 as libc::c_int as uint32_t,
    2688 as libc::c_int as uint32_t,
    2674 as libc::c_int as uint32_t,
    2661 as libc::c_int as uint32_t,
    2647 as libc::c_int as uint32_t,
    2634 as libc::c_int as uint32_t,
    2621 as libc::c_int as uint32_t,
    2608 as libc::c_int as uint32_t,
    2595 as libc::c_int as uint32_t,
    2582 as libc::c_int as uint32_t,
    2570 as libc::c_int as uint32_t,
    2557 as libc::c_int as uint32_t,
    2545 as libc::c_int as uint32_t,
    2532 as libc::c_int as uint32_t,
    2520 as libc::c_int as uint32_t,
    2508 as libc::c_int as uint32_t,
    2496 as libc::c_int as uint32_t,
    2484 as libc::c_int as uint32_t,
    2473 as libc::c_int as uint32_t,
    2461 as libc::c_int as uint32_t,
    2449 as libc::c_int as uint32_t,
    2438 as libc::c_int as uint32_t,
    2427 as libc::c_int as uint32_t,
    2416 as libc::c_int as uint32_t,
    2404 as libc::c_int as uint32_t,
    2394 as libc::c_int as uint32_t,
    2383 as libc::c_int as uint32_t,
    2372 as libc::c_int as uint32_t,
    2361 as libc::c_int as uint32_t,
    2351 as libc::c_int as uint32_t,
    2340 as libc::c_int as uint32_t,
    2330 as libc::c_int as uint32_t,
    2319 as libc::c_int as uint32_t,
    2309 as libc::c_int as uint32_t,
    2299 as libc::c_int as uint32_t,
    2289 as libc::c_int as uint32_t,
    2279 as libc::c_int as uint32_t,
    2269 as libc::c_int as uint32_t,
    2259 as libc::c_int as uint32_t,
    2250 as libc::c_int as uint32_t,
    2240 as libc::c_int as uint32_t,
    2231 as libc::c_int as uint32_t,
    2221 as libc::c_int as uint32_t,
    2212 as libc::c_int as uint32_t,
    2202 as libc::c_int as uint32_t,
    2193 as libc::c_int as uint32_t,
    2184 as libc::c_int as uint32_t,
    2175 as libc::c_int as uint32_t,
    2166 as libc::c_int as uint32_t,
    2157 as libc::c_int as uint32_t,
    2148 as libc::c_int as uint32_t,
    2139 as libc::c_int as uint32_t,
    2131 as libc::c_int as uint32_t,
    2122 as libc::c_int as uint32_t,
    2114 as libc::c_int as uint32_t,
    2105 as libc::c_int as uint32_t,
    2097 as libc::c_int as uint32_t,
    2088 as libc::c_int as uint32_t,
    2080 as libc::c_int as uint32_t,
    2072 as libc::c_int as uint32_t,
    2064 as libc::c_int as uint32_t,
    2056 as libc::c_int as uint32_t,
    2048 as libc::c_int as uint32_t,
    2040 as libc::c_int as uint32_t,
    2032 as libc::c_int as uint32_t,
    2024 as libc::c_int as uint32_t,
    2016 as libc::c_int as uint32_t,
    2008 as libc::c_int as uint32_t,
    2001 as libc::c_int as uint32_t,
    1993 as libc::c_int as uint32_t,
    1985 as libc::c_int as uint32_t,
    1978 as libc::c_int as uint32_t,
    1971 as libc::c_int as uint32_t,
    1963 as libc::c_int as uint32_t,
    1956 as libc::c_int as uint32_t,
    1949 as libc::c_int as uint32_t,
    1941 as libc::c_int as uint32_t,
    1934 as libc::c_int as uint32_t,
    1927 as libc::c_int as uint32_t,
    1920 as libc::c_int as uint32_t,
    1913 as libc::c_int as uint32_t,
    1906 as libc::c_int as uint32_t,
    1899 as libc::c_int as uint32_t,
    1892 as libc::c_int as uint32_t,
    1885 as libc::c_int as uint32_t,
    1879 as libc::c_int as uint32_t,
    1872 as libc::c_int as uint32_t,
    1865 as libc::c_int as uint32_t,
    1859 as libc::c_int as uint32_t,
    1852 as libc::c_int as uint32_t,
    1846 as libc::c_int as uint32_t,
    1839 as libc::c_int as uint32_t,
    1833 as libc::c_int as uint32_t,
    1826 as libc::c_int as uint32_t,
    1820 as libc::c_int as uint32_t,
    1814 as libc::c_int as uint32_t,
    1807 as libc::c_int as uint32_t,
    1801 as libc::c_int as uint32_t,
    1795 as libc::c_int as uint32_t,
    1789 as libc::c_int as uint32_t,
    1783 as libc::c_int as uint32_t,
    1777 as libc::c_int as uint32_t,
    1771 as libc::c_int as uint32_t,
    1765 as libc::c_int as uint32_t,
    1759 as libc::c_int as uint32_t,
    1753 as libc::c_int as uint32_t,
    1747 as libc::c_int as uint32_t,
    1741 as libc::c_int as uint32_t,
    1736 as libc::c_int as uint32_t,
    1730 as libc::c_int as uint32_t,
    1724 as libc::c_int as uint32_t,
    1718 as libc::c_int as uint32_t,
    1713 as libc::c_int as uint32_t,
    1707 as libc::c_int as uint32_t,
    1702 as libc::c_int as uint32_t,
    1696 as libc::c_int as uint32_t,
    1691 as libc::c_int as uint32_t,
    1685 as libc::c_int as uint32_t,
    1680 as libc::c_int as uint32_t,
    1675 as libc::c_int as uint32_t,
    1669 as libc::c_int as uint32_t,
    1664 as libc::c_int as uint32_t,
    1659 as libc::c_int as uint32_t,
    1653 as libc::c_int as uint32_t,
    1648 as libc::c_int as uint32_t,
    1643 as libc::c_int as uint32_t,
    1638 as libc::c_int as uint32_t,
    1633 as libc::c_int as uint32_t,
    1628 as libc::c_int as uint32_t,
    1623 as libc::c_int as uint32_t,
    1618 as libc::c_int as uint32_t,
    1613 as libc::c_int as uint32_t,
    1608 as libc::c_int as uint32_t,
    1603 as libc::c_int as uint32_t,
    1598 as libc::c_int as uint32_t,
    1593 as libc::c_int as uint32_t,
    1588 as libc::c_int as uint32_t,
    1583 as libc::c_int as uint32_t,
    1579 as libc::c_int as uint32_t,
    1574 as libc::c_int as uint32_t,
    1569 as libc::c_int as uint32_t,
    1565 as libc::c_int as uint32_t,
    1560 as libc::c_int as uint32_t,
    1555 as libc::c_int as uint32_t,
    1551 as libc::c_int as uint32_t,
    1546 as libc::c_int as uint32_t,
    1542 as libc::c_int as uint32_t,
    1537 as libc::c_int as uint32_t,
    1533 as libc::c_int as uint32_t,
    1528 as libc::c_int as uint32_t,
    1524 as libc::c_int as uint32_t,
    1519 as libc::c_int as uint32_t,
    1515 as libc::c_int as uint32_t,
    1510 as libc::c_int as uint32_t,
    1506 as libc::c_int as uint32_t,
    1502 as libc::c_int as uint32_t,
    1497 as libc::c_int as uint32_t,
    1493 as libc::c_int as uint32_t,
    1489 as libc::c_int as uint32_t,
    1485 as libc::c_int as uint32_t,
    1481 as libc::c_int as uint32_t,
    1476 as libc::c_int as uint32_t,
    1472 as libc::c_int as uint32_t,
    1468 as libc::c_int as uint32_t,
    1464 as libc::c_int as uint32_t,
    1460 as libc::c_int as uint32_t,
    1456 as libc::c_int as uint32_t,
    1452 as libc::c_int as uint32_t,
    1448 as libc::c_int as uint32_t,
    1444 as libc::c_int as uint32_t,
    1440 as libc::c_int as uint32_t,
    1436 as libc::c_int as uint32_t,
    1432 as libc::c_int as uint32_t,
    1428 as libc::c_int as uint32_t,
    1424 as libc::c_int as uint32_t,
    1420 as libc::c_int as uint32_t,
    1416 as libc::c_int as uint32_t,
    1413 as libc::c_int as uint32_t,
    1409 as libc::c_int as uint32_t,
    1405 as libc::c_int as uint32_t,
    1401 as libc::c_int as uint32_t,
    1398 as libc::c_int as uint32_t,
    1394 as libc::c_int as uint32_t,
    1390 as libc::c_int as uint32_t,
    1387 as libc::c_int as uint32_t,
    1383 as libc::c_int as uint32_t,
    1379 as libc::c_int as uint32_t,
    1376 as libc::c_int as uint32_t,
    1372 as libc::c_int as uint32_t,
    1368 as libc::c_int as uint32_t,
    1365 as libc::c_int as uint32_t,
    1361 as libc::c_int as uint32_t,
    1358 as libc::c_int as uint32_t,
    1354 as libc::c_int as uint32_t,
    1351 as libc::c_int as uint32_t,
    1347 as libc::c_int as uint32_t,
    1344 as libc::c_int as uint32_t,
    1340 as libc::c_int as uint32_t,
    1337 as libc::c_int as uint32_t,
    1334 as libc::c_int as uint32_t,
    1330 as libc::c_int as uint32_t,
    1327 as libc::c_int as uint32_t,
    1323 as libc::c_int as uint32_t,
    1320 as libc::c_int as uint32_t,
    1317 as libc::c_int as uint32_t,
    1314 as libc::c_int as uint32_t,
    1310 as libc::c_int as uint32_t,
    1307 as libc::c_int as uint32_t,
    1304 as libc::c_int as uint32_t,
    1300 as libc::c_int as uint32_t,
    1297 as libc::c_int as uint32_t,
    1294 as libc::c_int as uint32_t,
    1291 as libc::c_int as uint32_t,
    1288 as libc::c_int as uint32_t,
    1285 as libc::c_int as uint32_t,
    1281 as libc::c_int as uint32_t,
    1278 as libc::c_int as uint32_t,
    1275 as libc::c_int as uint32_t,
    1272 as libc::c_int as uint32_t,
    1269 as libc::c_int as uint32_t,
    1266 as libc::c_int as uint32_t,
    1263 as libc::c_int as uint32_t,
    1260 as libc::c_int as uint32_t,
    1257 as libc::c_int as uint32_t,
    1254 as libc::c_int as uint32_t,
    1251 as libc::c_int as uint32_t,
    1248 as libc::c_int as uint32_t,
    1245 as libc::c_int as uint32_t,
    1242 as libc::c_int as uint32_t,
    1239 as libc::c_int as uint32_t,
    1236 as libc::c_int as uint32_t,
    1233 as libc::c_int as uint32_t,
    1230 as libc::c_int as uint32_t,
    1227 as libc::c_int as uint32_t,
    1224 as libc::c_int as uint32_t,
    1222 as libc::c_int as uint32_t,
    1219 as libc::c_int as uint32_t,
    1216 as libc::c_int as uint32_t,
    1213 as libc::c_int as uint32_t,
    1210 as libc::c_int as uint32_t,
    1208 as libc::c_int as uint32_t,
    1205 as libc::c_int as uint32_t,
    1202 as libc::c_int as uint32_t,
    1199 as libc::c_int as uint32_t,
    1197 as libc::c_int as uint32_t,
    1194 as libc::c_int as uint32_t,
    1191 as libc::c_int as uint32_t,
    1188 as libc::c_int as uint32_t,
    1186 as libc::c_int as uint32_t,
    1183 as libc::c_int as uint32_t,
    1180 as libc::c_int as uint32_t,
    1178 as libc::c_int as uint32_t,
    1175 as libc::c_int as uint32_t,
    1172 as libc::c_int as uint32_t,
    1170 as libc::c_int as uint32_t,
    1167 as libc::c_int as uint32_t,
    1165 as libc::c_int as uint32_t,
    1162 as libc::c_int as uint32_t,
    1159 as libc::c_int as uint32_t,
    1157 as libc::c_int as uint32_t,
    1154 as libc::c_int as uint32_t,
    1152 as libc::c_int as uint32_t,
    1149 as libc::c_int as uint32_t,
    1147 as libc::c_int as uint32_t,
    1144 as libc::c_int as uint32_t,
    1142 as libc::c_int as uint32_t,
    1139 as libc::c_int as uint32_t,
    1137 as libc::c_int as uint32_t,
    1134 as libc::c_int as uint32_t,
    1132 as libc::c_int as uint32_t,
    1129 as libc::c_int as uint32_t,
    1127 as libc::c_int as uint32_t,
    1125 as libc::c_int as uint32_t,
    1122 as libc::c_int as uint32_t,
    1120 as libc::c_int as uint32_t,
    1117 as libc::c_int as uint32_t,
    1115 as libc::c_int as uint32_t,
    1113 as libc::c_int as uint32_t,
    1110 as libc::c_int as uint32_t,
    1108 as libc::c_int as uint32_t,
    1106 as libc::c_int as uint32_t,
    1103 as libc::c_int as uint32_t,
    1101 as libc::c_int as uint32_t,
    1099 as libc::c_int as uint32_t,
    1096 as libc::c_int as uint32_t,
    1094 as libc::c_int as uint32_t,
    1092 as libc::c_int as uint32_t,
    1089 as libc::c_int as uint32_t,
    1087 as libc::c_int as uint32_t,
    1085 as libc::c_int as uint32_t,
    1083 as libc::c_int as uint32_t,
    1081 as libc::c_int as uint32_t,
    1078 as libc::c_int as uint32_t,
    1076 as libc::c_int as uint32_t,
    1074 as libc::c_int as uint32_t,
    1072 as libc::c_int as uint32_t,
    1069 as libc::c_int as uint32_t,
    1067 as libc::c_int as uint32_t,
    1065 as libc::c_int as uint32_t,
    1063 as libc::c_int as uint32_t,
    1061 as libc::c_int as uint32_t,
    1059 as libc::c_int as uint32_t,
    1057 as libc::c_int as uint32_t,
    1054 as libc::c_int as uint32_t,
    1052 as libc::c_int as uint32_t,
    1050 as libc::c_int as uint32_t,
    1048 as libc::c_int as uint32_t,
    1046 as libc::c_int as uint32_t,
    1044 as libc::c_int as uint32_t,
    1042 as libc::c_int as uint32_t,
    1040 as libc::c_int as uint32_t,
    1038 as libc::c_int as uint32_t,
    1036 as libc::c_int as uint32_t,
    1034 as libc::c_int as uint32_t,
    1032 as libc::c_int as uint32_t,
    1030 as libc::c_int as uint32_t,
    1028 as libc::c_int as uint32_t,
    1026 as libc::c_int as uint32_t,
    1024 as libc::c_int as uint32_t,
    1022 as libc::c_int as uint32_t,
    1020 as libc::c_int as uint32_t,
    1018 as libc::c_int as uint32_t,
    1016 as libc::c_int as uint32_t,
    1014 as libc::c_int as uint32_t,
    1012 as libc::c_int as uint32_t,
    1010 as libc::c_int as uint32_t,
    1008 as libc::c_int as uint32_t,
    1006 as libc::c_int as uint32_t,
    1004 as libc::c_int as uint32_t,
    1002 as libc::c_int as uint32_t,
    1000 as libc::c_int as uint32_t,
    998 as libc::c_int as uint32_t,
    996 as libc::c_int as uint32_t,
    994 as libc::c_int as uint32_t,
    992 as libc::c_int as uint32_t,
    991 as libc::c_int as uint32_t,
    989 as libc::c_int as uint32_t,
    987 as libc::c_int as uint32_t,
    985 as libc::c_int as uint32_t,
    983 as libc::c_int as uint32_t,
    981 as libc::c_int as uint32_t,
    979 as libc::c_int as uint32_t,
    978 as libc::c_int as uint32_t,
    976 as libc::c_int as uint32_t,
    974 as libc::c_int as uint32_t,
    972 as libc::c_int as uint32_t,
    970 as libc::c_int as uint32_t,
    969 as libc::c_int as uint32_t,
    967 as libc::c_int as uint32_t,
    965 as libc::c_int as uint32_t,
    963 as libc::c_int as uint32_t,
    961 as libc::c_int as uint32_t,
    960 as libc::c_int as uint32_t,
    958 as libc::c_int as uint32_t,
    956 as libc::c_int as uint32_t,
    954 as libc::c_int as uint32_t,
    953 as libc::c_int as uint32_t,
    951 as libc::c_int as uint32_t,
    949 as libc::c_int as uint32_t,
    948 as libc::c_int as uint32_t,
    946 as libc::c_int as uint32_t,
    944 as libc::c_int as uint32_t,
    942 as libc::c_int as uint32_t,
    941 as libc::c_int as uint32_t,
    939 as libc::c_int as uint32_t,
    937 as libc::c_int as uint32_t,
    936 as libc::c_int as uint32_t,
    934 as libc::c_int as uint32_t,
    932 as libc::c_int as uint32_t,
    931 as libc::c_int as uint32_t,
    929 as libc::c_int as uint32_t,
    927 as libc::c_int as uint32_t,
    926 as libc::c_int as uint32_t,
    924 as libc::c_int as uint32_t,
    923 as libc::c_int as uint32_t,
    921 as libc::c_int as uint32_t,
    919 as libc::c_int as uint32_t,
    918 as libc::c_int as uint32_t,
    916 as libc::c_int as uint32_t,
    914 as libc::c_int as uint32_t,
    913 as libc::c_int as uint32_t,
    911 as libc::c_int as uint32_t,
    910 as libc::c_int as uint32_t,
    908 as libc::c_int as uint32_t,
    907 as libc::c_int as uint32_t,
    905 as libc::c_int as uint32_t,
    903 as libc::c_int as uint32_t,
    902 as libc::c_int as uint32_t,
    900 as libc::c_int as uint32_t,
    899 as libc::c_int as uint32_t,
    897 as libc::c_int as uint32_t,
    896 as libc::c_int as uint32_t,
    894 as libc::c_int as uint32_t,
    893 as libc::c_int as uint32_t,
    891 as libc::c_int as uint32_t,
    890 as libc::c_int as uint32_t,
    888 as libc::c_int as uint32_t,
    887 as libc::c_int as uint32_t,
    885 as libc::c_int as uint32_t,
    884 as libc::c_int as uint32_t,
    882 as libc::c_int as uint32_t,
    881 as libc::c_int as uint32_t,
    879 as libc::c_int as uint32_t,
    878 as libc::c_int as uint32_t,
    876 as libc::c_int as uint32_t,
    875 as libc::c_int as uint32_t,
    873 as libc::c_int as uint32_t,
    872 as libc::c_int as uint32_t,
    870 as libc::c_int as uint32_t,
    869 as libc::c_int as uint32_t,
    868 as libc::c_int as uint32_t,
    866 as libc::c_int as uint32_t,
    865 as libc::c_int as uint32_t,
    863 as libc::c_int as uint32_t,
    862 as libc::c_int as uint32_t,
    860 as libc::c_int as uint32_t,
    859 as libc::c_int as uint32_t,
    858 as libc::c_int as uint32_t,
    856 as libc::c_int as uint32_t,
    855 as libc::c_int as uint32_t,
    853 as libc::c_int as uint32_t,
    852 as libc::c_int as uint32_t,
    851 as libc::c_int as uint32_t,
    849 as libc::c_int as uint32_t,
    848 as libc::c_int as uint32_t,
    846 as libc::c_int as uint32_t,
    845 as libc::c_int as uint32_t,
    844 as libc::c_int as uint32_t,
    842 as libc::c_int as uint32_t,
    841 as libc::c_int as uint32_t,
    840 as libc::c_int as uint32_t,
    838 as libc::c_int as uint32_t,
    837 as libc::c_int as uint32_t,
    836 as libc::c_int as uint32_t,
    834 as libc::c_int as uint32_t,
    833 as libc::c_int as uint32_t,
    832 as libc::c_int as uint32_t,
    830 as libc::c_int as uint32_t,
    829 as libc::c_int as uint32_t,
    828 as libc::c_int as uint32_t,
    826 as libc::c_int as uint32_t,
    825 as libc::c_int as uint32_t,
    824 as libc::c_int as uint32_t,
    823 as libc::c_int as uint32_t,
    821 as libc::c_int as uint32_t,
    820 as libc::c_int as uint32_t,
    819 as libc::c_int as uint32_t,
    817 as libc::c_int as uint32_t,
    816 as libc::c_int as uint32_t,
    815 as libc::c_int as uint32_t,
    814 as libc::c_int as uint32_t,
    812 as libc::c_int as uint32_t,
    811 as libc::c_int as uint32_t,
    810 as libc::c_int as uint32_t,
    809 as libc::c_int as uint32_t,
    807 as libc::c_int as uint32_t,
    806 as libc::c_int as uint32_t,
    805 as libc::c_int as uint32_t,
    804 as libc::c_int as uint32_t,
    802 as libc::c_int as uint32_t,
    801 as libc::c_int as uint32_t,
    800 as libc::c_int as uint32_t,
    799 as libc::c_int as uint32_t,
    798 as libc::c_int as uint32_t,
    796 as libc::c_int as uint32_t,
    795 as libc::c_int as uint32_t,
    794 as libc::c_int as uint32_t,
    793 as libc::c_int as uint32_t,
    791 as libc::c_int as uint32_t,
    790 as libc::c_int as uint32_t,
    789 as libc::c_int as uint32_t,
    788 as libc::c_int as uint32_t,
    787 as libc::c_int as uint32_t,
    786 as libc::c_int as uint32_t,
    784 as libc::c_int as uint32_t,
    783 as libc::c_int as uint32_t,
    782 as libc::c_int as uint32_t,
    781 as libc::c_int as uint32_t,
    780 as libc::c_int as uint32_t,
    779 as libc::c_int as uint32_t,
    777 as libc::c_int as uint32_t,
    776 as libc::c_int as uint32_t,
    775 as libc::c_int as uint32_t,
    774 as libc::c_int as uint32_t,
    773 as libc::c_int as uint32_t,
    772 as libc::c_int as uint32_t,
    771 as libc::c_int as uint32_t,
    769 as libc::c_int as uint32_t,
    768 as libc::c_int as uint32_t,
    767 as libc::c_int as uint32_t,
    766 as libc::c_int as uint32_t,
    765 as libc::c_int as uint32_t,
    764 as libc::c_int as uint32_t,
    763 as libc::c_int as uint32_t,
    762 as libc::c_int as uint32_t,
    760 as libc::c_int as uint32_t,
    759 as libc::c_int as uint32_t,
    758 as libc::c_int as uint32_t,
    757 as libc::c_int as uint32_t,
    756 as libc::c_int as uint32_t,
    755 as libc::c_int as uint32_t,
    754 as libc::c_int as uint32_t,
    753 as libc::c_int as uint32_t,
    752 as libc::c_int as uint32_t,
    751 as libc::c_int as uint32_t,
    750 as libc::c_int as uint32_t,
    748 as libc::c_int as uint32_t,
    747 as libc::c_int as uint32_t,
    746 as libc::c_int as uint32_t,
    745 as libc::c_int as uint32_t,
    744 as libc::c_int as uint32_t,
    743 as libc::c_int as uint32_t,
    742 as libc::c_int as uint32_t,
    741 as libc::c_int as uint32_t,
    740 as libc::c_int as uint32_t,
    739 as libc::c_int as uint32_t,
    738 as libc::c_int as uint32_t,
    737 as libc::c_int as uint32_t,
    736 as libc::c_int as uint32_t,
    735 as libc::c_int as uint32_t,
    734 as libc::c_int as uint32_t,
    733 as libc::c_int as uint32_t,
    732 as libc::c_int as uint32_t,
    731 as libc::c_int as uint32_t,
    730 as libc::c_int as uint32_t,
    729 as libc::c_int as uint32_t,
    728 as libc::c_int as uint32_t,
    727 as libc::c_int as uint32_t,
    726 as libc::c_int as uint32_t,
    725 as libc::c_int as uint32_t,
    724 as libc::c_int as uint32_t,
    723 as libc::c_int as uint32_t,
    722 as libc::c_int as uint32_t,
    721 as libc::c_int as uint32_t,
    720 as libc::c_int as uint32_t,
    719 as libc::c_int as uint32_t,
    718 as libc::c_int as uint32_t,
    717 as libc::c_int as uint32_t,
    716 as libc::c_int as uint32_t,
    715 as libc::c_int as uint32_t,
    714 as libc::c_int as uint32_t,
    713 as libc::c_int as uint32_t,
    712 as libc::c_int as uint32_t,
    711 as libc::c_int as uint32_t,
    710 as libc::c_int as uint32_t,
    709 as libc::c_int as uint32_t,
    708 as libc::c_int as uint32_t,
    707 as libc::c_int as uint32_t,
    706 as libc::c_int as uint32_t,
    705 as libc::c_int as uint32_t,
    704 as libc::c_int as uint32_t,
    703 as libc::c_int as uint32_t,
    702 as libc::c_int as uint32_t,
    701 as libc::c_int as uint32_t,
    700 as libc::c_int as uint32_t,
    699 as libc::c_int as uint32_t,
    699 as libc::c_int as uint32_t,
    698 as libc::c_int as uint32_t,
    697 as libc::c_int as uint32_t,
    696 as libc::c_int as uint32_t,
    695 as libc::c_int as uint32_t,
    694 as libc::c_int as uint32_t,
    693 as libc::c_int as uint32_t,
    692 as libc::c_int as uint32_t,
    691 as libc::c_int as uint32_t,
    690 as libc::c_int as uint32_t,
    689 as libc::c_int as uint32_t,
    688 as libc::c_int as uint32_t,
    688 as libc::c_int as uint32_t,
    687 as libc::c_int as uint32_t,
    686 as libc::c_int as uint32_t,
    685 as libc::c_int as uint32_t,
    684 as libc::c_int as uint32_t,
    683 as libc::c_int as uint32_t,
    682 as libc::c_int as uint32_t,
    681 as libc::c_int as uint32_t,
    680 as libc::c_int as uint32_t,
    680 as libc::c_int as uint32_t,
    679 as libc::c_int as uint32_t,
    678 as libc::c_int as uint32_t,
    677 as libc::c_int as uint32_t,
    676 as libc::c_int as uint32_t,
    675 as libc::c_int as uint32_t,
    674 as libc::c_int as uint32_t,
    673 as libc::c_int as uint32_t,
    673 as libc::c_int as uint32_t,
    672 as libc::c_int as uint32_t,
    671 as libc::c_int as uint32_t,
    670 as libc::c_int as uint32_t,
    669 as libc::c_int as uint32_t,
    668 as libc::c_int as uint32_t,
    667 as libc::c_int as uint32_t,
    667 as libc::c_int as uint32_t,
    666 as libc::c_int as uint32_t,
    665 as libc::c_int as uint32_t,
    664 as libc::c_int as uint32_t,
    663 as libc::c_int as uint32_t,
    662 as libc::c_int as uint32_t,
    661 as libc::c_int as uint32_t,
    661 as libc::c_int as uint32_t,
    660 as libc::c_int as uint32_t,
    659 as libc::c_int as uint32_t,
    658 as libc::c_int as uint32_t,
    657 as libc::c_int as uint32_t,
    657 as libc::c_int as uint32_t,
    656 as libc::c_int as uint32_t,
    655 as libc::c_int as uint32_t,
    654 as libc::c_int as uint32_t,
    653 as libc::c_int as uint32_t,
    652 as libc::c_int as uint32_t,
    652 as libc::c_int as uint32_t,
    651 as libc::c_int as uint32_t,
    650 as libc::c_int as uint32_t,
    649 as libc::c_int as uint32_t,
    648 as libc::c_int as uint32_t,
    648 as libc::c_int as uint32_t,
    647 as libc::c_int as uint32_t,
    646 as libc::c_int as uint32_t,
    645 as libc::c_int as uint32_t,
    644 as libc::c_int as uint32_t,
    644 as libc::c_int as uint32_t,
    643 as libc::c_int as uint32_t,
    642 as libc::c_int as uint32_t,
    641 as libc::c_int as uint32_t,
    640 as libc::c_int as uint32_t,
    640 as libc::c_int as uint32_t,
    639 as libc::c_int as uint32_t,
    638 as libc::c_int as uint32_t,
    637 as libc::c_int as uint32_t,
    637 as libc::c_int as uint32_t,
    636 as libc::c_int as uint32_t,
    635 as libc::c_int as uint32_t,
    634 as libc::c_int as uint32_t,
    633 as libc::c_int as uint32_t,
    633 as libc::c_int as uint32_t,
    632 as libc::c_int as uint32_t,
    631 as libc::c_int as uint32_t,
    630 as libc::c_int as uint32_t,
    630 as libc::c_int as uint32_t,
    629 as libc::c_int as uint32_t,
    628 as libc::c_int as uint32_t,
    627 as libc::c_int as uint32_t,
    627 as libc::c_int as uint32_t,
    626 as libc::c_int as uint32_t,
    625 as libc::c_int as uint32_t,
    624 as libc::c_int as uint32_t,
    624 as libc::c_int as uint32_t,
    623 as libc::c_int as uint32_t,
    622 as libc::c_int as uint32_t,
    621 as libc::c_int as uint32_t,
    621 as libc::c_int as uint32_t,
    620 as libc::c_int as uint32_t,
    619 as libc::c_int as uint32_t,
    618 as libc::c_int as uint32_t,
    618 as libc::c_int as uint32_t,
    617 as libc::c_int as uint32_t,
    616 as libc::c_int as uint32_t,
    616 as libc::c_int as uint32_t,
    615 as libc::c_int as uint32_t,
    614 as libc::c_int as uint32_t,
    613 as libc::c_int as uint32_t,
    613 as libc::c_int as uint32_t,
    612 as libc::c_int as uint32_t,
    611 as libc::c_int as uint32_t,
    611 as libc::c_int as uint32_t,
    610 as libc::c_int as uint32_t,
    609 as libc::c_int as uint32_t,
    608 as libc::c_int as uint32_t,
    608 as libc::c_int as uint32_t,
    607 as libc::c_int as uint32_t,
    606 as libc::c_int as uint32_t,
    606 as libc::c_int as uint32_t,
    605 as libc::c_int as uint32_t,
    604 as libc::c_int as uint32_t,
    604 as libc::c_int as uint32_t,
    603 as libc::c_int as uint32_t,
    602 as libc::c_int as uint32_t,
    601 as libc::c_int as uint32_t,
    601 as libc::c_int as uint32_t,
    600 as libc::c_int as uint32_t,
    599 as libc::c_int as uint32_t,
    599 as libc::c_int as uint32_t,
    598 as libc::c_int as uint32_t,
    597 as libc::c_int as uint32_t,
    597 as libc::c_int as uint32_t,
    596 as libc::c_int as uint32_t,
    595 as libc::c_int as uint32_t,
    595 as libc::c_int as uint32_t,
    594 as libc::c_int as uint32_t,
    593 as libc::c_int as uint32_t,
    593 as libc::c_int as uint32_t,
    592 as libc::c_int as uint32_t,
    591 as libc::c_int as uint32_t,
    591 as libc::c_int as uint32_t,
    590 as libc::c_int as uint32_t,
    589 as libc::c_int as uint32_t,
    589 as libc::c_int as uint32_t,
    588 as libc::c_int as uint32_t,
    587 as libc::c_int as uint32_t,
    587 as libc::c_int as uint32_t,
    586 as libc::c_int as uint32_t,
    585 as libc::c_int as uint32_t,
    585 as libc::c_int as uint32_t,
    584 as libc::c_int as uint32_t,
    583 as libc::c_int as uint32_t,
    583 as libc::c_int as uint32_t,
    582 as libc::c_int as uint32_t,
    581 as libc::c_int as uint32_t,
    581 as libc::c_int as uint32_t,
    580 as libc::c_int as uint32_t,
    579 as libc::c_int as uint32_t,
    579 as libc::c_int as uint32_t,
    578 as libc::c_int as uint32_t,
    578 as libc::c_int as uint32_t,
    577 as libc::c_int as uint32_t,
    576 as libc::c_int as uint32_t,
    576 as libc::c_int as uint32_t,
    575 as libc::c_int as uint32_t,
    574 as libc::c_int as uint32_t,
    574 as libc::c_int as uint32_t,
    573 as libc::c_int as uint32_t,
    572 as libc::c_int as uint32_t,
    572 as libc::c_int as uint32_t,
    571 as libc::c_int as uint32_t,
    571 as libc::c_int as uint32_t,
    570 as libc::c_int as uint32_t,
    569 as libc::c_int as uint32_t,
    569 as libc::c_int as uint32_t,
    568 as libc::c_int as uint32_t,
    568 as libc::c_int as uint32_t,
    567 as libc::c_int as uint32_t,
    566 as libc::c_int as uint32_t,
    566 as libc::c_int as uint32_t,
    565 as libc::c_int as uint32_t,
    564 as libc::c_int as uint32_t,
    564 as libc::c_int as uint32_t,
    563 as libc::c_int as uint32_t,
    563 as libc::c_int as uint32_t,
    562 as libc::c_int as uint32_t,
    561 as libc::c_int as uint32_t,
    561 as libc::c_int as uint32_t,
    560 as libc::c_int as uint32_t,
    560 as libc::c_int as uint32_t,
    559 as libc::c_int as uint32_t,
    558 as libc::c_int as uint32_t,
    558 as libc::c_int as uint32_t,
    557 as libc::c_int as uint32_t,
    557 as libc::c_int as uint32_t,
    556 as libc::c_int as uint32_t,
    555 as libc::c_int as uint32_t,
    555 as libc::c_int as uint32_t,
    554 as libc::c_int as uint32_t,
    554 as libc::c_int as uint32_t,
    553 as libc::c_int as uint32_t,
    553 as libc::c_int as uint32_t,
    552 as libc::c_int as uint32_t,
    551 as libc::c_int as uint32_t,
    551 as libc::c_int as uint32_t,
    550 as libc::c_int as uint32_t,
    550 as libc::c_int as uint32_t,
    549 as libc::c_int as uint32_t,
    548 as libc::c_int as uint32_t,
    548 as libc::c_int as uint32_t,
    547 as libc::c_int as uint32_t,
    547 as libc::c_int as uint32_t,
    546 as libc::c_int as uint32_t,
    546 as libc::c_int as uint32_t,
    545 as libc::c_int as uint32_t,
    544 as libc::c_int as uint32_t,
    544 as libc::c_int as uint32_t,
    543 as libc::c_int as uint32_t,
    543 as libc::c_int as uint32_t,
    542 as libc::c_int as uint32_t,
    542 as libc::c_int as uint32_t,
    541 as libc::c_int as uint32_t,
    541 as libc::c_int as uint32_t,
    540 as libc::c_int as uint32_t,
    539 as libc::c_int as uint32_t,
    539 as libc::c_int as uint32_t,
    538 as libc::c_int as uint32_t,
    538 as libc::c_int as uint32_t,
    537 as libc::c_int as uint32_t,
    537 as libc::c_int as uint32_t,
    536 as libc::c_int as uint32_t,
    536 as libc::c_int as uint32_t,
    535 as libc::c_int as uint32_t,
    534 as libc::c_int as uint32_t,
    534 as libc::c_int as uint32_t,
    533 as libc::c_int as uint32_t,
    533 as libc::c_int as uint32_t,
    532 as libc::c_int as uint32_t,
    532 as libc::c_int as uint32_t,
    531 as libc::c_int as uint32_t,
    531 as libc::c_int as uint32_t,
    530 as libc::c_int as uint32_t,
    530 as libc::c_int as uint32_t,
    529 as libc::c_int as uint32_t,
    529 as libc::c_int as uint32_t,
    528 as libc::c_int as uint32_t,
    527 as libc::c_int as uint32_t,
    527 as libc::c_int as uint32_t,
    526 as libc::c_int as uint32_t,
    526 as libc::c_int as uint32_t,
    525 as libc::c_int as uint32_t,
    525 as libc::c_int as uint32_t,
    524 as libc::c_int as uint32_t,
    524 as libc::c_int as uint32_t,
    523 as libc::c_int as uint32_t,
    523 as libc::c_int as uint32_t,
    522 as libc::c_int as uint32_t,
    522 as libc::c_int as uint32_t,
    521 as libc::c_int as uint32_t,
    521 as libc::c_int as uint32_t,
    520 as libc::c_int as uint32_t,
    520 as libc::c_int as uint32_t,
    519 as libc::c_int as uint32_t,
    519 as libc::c_int as uint32_t,
    518 as libc::c_int as uint32_t,
    518 as libc::c_int as uint32_t,
    517 as libc::c_int as uint32_t,
    517 as libc::c_int as uint32_t,
    516 as libc::c_int as uint32_t,
    516 as libc::c_int as uint32_t,
    515 as libc::c_int as uint32_t,
    515 as libc::c_int as uint32_t,
    514 as libc::c_int as uint32_t,
    514 as libc::c_int as uint32_t,
];
#[inline]
unsafe extern "C" fn LinearToGammaWeighted(
    mut src: *const uint8_t,
    mut a_ptr: *const uint8_t,
    mut total_a: uint32_t,
    mut step: libc::c_int,
    mut rgb_stride: libc::c_int,
) -> libc::c_int {
    let sum: uint32_t = (*a_ptr.offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_mul(GammaToLinear(*src.offset(0 as libc::c_int as isize)))
        .wrapping_add(
            (*a_ptr.offset(step as isize) as libc::c_uint)
                .wrapping_mul(GammaToLinear(*src.offset(step as isize))),
        )
        .wrapping_add(
            (*a_ptr.offset(rgb_stride as isize) as libc::c_uint)
                .wrapping_mul(GammaToLinear(*src.offset(rgb_stride as isize))),
        )
        .wrapping_add(
            (*a_ptr.offset((rgb_stride + step) as isize) as libc::c_uint)
                .wrapping_mul(GammaToLinear(*src.offset((rgb_stride + step) as isize))),
        );
    return LinearToGamma(
        sum.wrapping_mul(kInvAlpha[total_a as usize]) >> kAlphaFix - 2 as libc::c_int,
        0 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn ConvertRowToY(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    mut step: libc::c_int,
    dst_y: *mut uint8_t,
    mut width: libc::c_int,
    rg: *mut VP8Random,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < width {
        *dst_y
            .offset(
                i as isize,
            ) = RGBToY(
            *r_ptr.offset(j as isize) as libc::c_int,
            *g_ptr.offset(j as isize) as libc::c_int,
            *b_ptr.offset(j as isize) as libc::c_int,
            rg,
        ) as uint8_t;
        i += 1 as libc::c_int;
        j += step;
    }
}
#[inline]
unsafe extern "C" fn AccumulateRGBA(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    a_ptr: *const uint8_t,
    mut rgb_stride: libc::c_int,
    mut dst: *mut uint16_t,
    mut width: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < width >> 1 as libc::c_int {
        let a: uint32_t = (*a_ptr.offset(j as isize).offset(0 as libc::c_int as isize)
            as libc::c_int
            + *a_ptr.offset(j as isize).offset(rgb_stride as isize) as libc::c_int
            + (*a_ptr
                .offset(j as isize)
                .offset(4 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int
                + *a_ptr
                    .offset(j as isize)
                    .offset(4 as libc::c_int as isize)
                    .offset(rgb_stride as isize) as libc::c_int)) as uint32_t;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        if a == (4 as libc::c_int * 0xff as libc::c_int) as libc::c_uint
            || a == 0 as libc::c_int as libc::c_uint
        {
            r = LinearToGamma(
                (GammaToLinear(
                    *r_ptr.offset(j as isize).offset(0 as libc::c_int as isize),
                ))
                    .wrapping_add(
                        GammaToLinear(
                            *r_ptr.offset(j as isize).offset(4 as libc::c_int as isize),
                        ),
                    )
                    .wrapping_add(
                        GammaToLinear(
                            *r_ptr.offset(j as isize).offset(rgb_stride as isize),
                        ),
                    )
                    .wrapping_add(
                        GammaToLinear(
                            *r_ptr
                                .offset(j as isize)
                                .offset((rgb_stride + 4 as libc::c_int) as isize),
                        ),
                    ),
                0 as libc::c_int,
            );
            g = LinearToGamma(
                (GammaToLinear(
                    *g_ptr.offset(j as isize).offset(0 as libc::c_int as isize),
                ))
                    .wrapping_add(
                        GammaToLinear(
                            *g_ptr.offset(j as isize).offset(4 as libc::c_int as isize),
                        ),
                    )
                    .wrapping_add(
                        GammaToLinear(
                            *g_ptr.offset(j as isize).offset(rgb_stride as isize),
                        ),
                    )
                    .wrapping_add(
                        GammaToLinear(
                            *g_ptr
                                .offset(j as isize)
                                .offset((rgb_stride + 4 as libc::c_int) as isize),
                        ),
                    ),
                0 as libc::c_int,
            );
            b = LinearToGamma(
                (GammaToLinear(
                    *b_ptr.offset(j as isize).offset(0 as libc::c_int as isize),
                ))
                    .wrapping_add(
                        GammaToLinear(
                            *b_ptr.offset(j as isize).offset(4 as libc::c_int as isize),
                        ),
                    )
                    .wrapping_add(
                        GammaToLinear(
                            *b_ptr.offset(j as isize).offset(rgb_stride as isize),
                        ),
                    )
                    .wrapping_add(
                        GammaToLinear(
                            *b_ptr
                                .offset(j as isize)
                                .offset((rgb_stride + 4 as libc::c_int) as isize),
                        ),
                    ),
                0 as libc::c_int,
            );
        } else {
            r = LinearToGammaWeighted(
                r_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a,
                4 as libc::c_int,
                rgb_stride,
            );
            g = LinearToGammaWeighted(
                g_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a,
                4 as libc::c_int,
                rgb_stride,
            );
            b = LinearToGammaWeighted(
                b_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a,
                4 as libc::c_int,
                rgb_stride,
            );
        }
        *dst.offset(0 as libc::c_int as isize) = r as uint16_t;
        *dst.offset(1 as libc::c_int as isize) = g as uint16_t;
        *dst.offset(2 as libc::c_int as isize) = b as uint16_t;
        *dst.offset(3 as libc::c_int as isize) = a as uint16_t;
        i += 1 as libc::c_int;
        j += 2 as libc::c_int * 4 as libc::c_int;
        dst = dst.offset(4 as libc::c_int as isize);
    }
    if width & 1 as libc::c_int != 0 {
        let a_0: uint32_t = (2 as libc::c_uint)
            .wrapping_mul(
                (*a_ptr.offset(j as isize).offset(0 as libc::c_int as isize)
                    as libc::c_int
                    + *a_ptr.offset(j as isize).offset(rgb_stride as isize)
                        as libc::c_int) as libc::c_uint,
            );
        let mut r_0: libc::c_int = 0;
        let mut g_0: libc::c_int = 0;
        let mut b_0: libc::c_int = 0;
        if a_0 == (4 as libc::c_int * 0xff as libc::c_int) as libc::c_uint
            || a_0 == 0 as libc::c_int as libc::c_uint
        {
            r_0 = LinearToGamma(
                (GammaToLinear(
                    *r_ptr.offset(j as isize).offset(0 as libc::c_int as isize),
                ))
                    .wrapping_add(
                        GammaToLinear(
                            *r_ptr.offset(j as isize).offset(rgb_stride as isize),
                        ),
                    ),
                1 as libc::c_int,
            );
            g_0 = LinearToGamma(
                (GammaToLinear(
                    *g_ptr.offset(j as isize).offset(0 as libc::c_int as isize),
                ))
                    .wrapping_add(
                        GammaToLinear(
                            *g_ptr.offset(j as isize).offset(rgb_stride as isize),
                        ),
                    ),
                1 as libc::c_int,
            );
            b_0 = LinearToGamma(
                (GammaToLinear(
                    *b_ptr.offset(j as isize).offset(0 as libc::c_int as isize),
                ))
                    .wrapping_add(
                        GammaToLinear(
                            *b_ptr.offset(j as isize).offset(rgb_stride as isize),
                        ),
                    ),
                1 as libc::c_int,
            );
        } else {
            r_0 = LinearToGammaWeighted(
                r_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a_0,
                0 as libc::c_int,
                rgb_stride,
            );
            g_0 = LinearToGammaWeighted(
                g_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a_0,
                0 as libc::c_int,
                rgb_stride,
            );
            b_0 = LinearToGammaWeighted(
                b_ptr.offset(j as isize),
                a_ptr.offset(j as isize),
                a_0,
                0 as libc::c_int,
                rgb_stride,
            );
        }
        *dst.offset(0 as libc::c_int as isize) = r_0 as uint16_t;
        *dst.offset(1 as libc::c_int as isize) = g_0 as uint16_t;
        *dst.offset(2 as libc::c_int as isize) = b_0 as uint16_t;
        *dst.offset(3 as libc::c_int as isize) = a_0 as uint16_t;
    }
}
#[inline]
unsafe extern "C" fn AccumulateRGB(
    r_ptr: *const uint8_t,
    g_ptr: *const uint8_t,
    b_ptr: *const uint8_t,
    mut step: libc::c_int,
    mut rgb_stride: libc::c_int,
    mut dst: *mut uint16_t,
    mut width: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < width >> 1 as libc::c_int {
        *dst
            .offset(
                0 as libc::c_int as isize,
            ) = LinearToGamma(
            (GammaToLinear(*r_ptr.offset(j as isize).offset(0 as libc::c_int as isize)))
                .wrapping_add(
                    GammaToLinear(*r_ptr.offset(j as isize).offset(step as isize)),
                )
                .wrapping_add(
                    GammaToLinear(*r_ptr.offset(j as isize).offset(rgb_stride as isize)),
                )
                .wrapping_add(
                    GammaToLinear(
                        *r_ptr.offset(j as isize).offset((rgb_stride + step) as isize),
                    ),
                ),
            0 as libc::c_int,
        ) as uint16_t;
        *dst
            .offset(
                1 as libc::c_int as isize,
            ) = LinearToGamma(
            (GammaToLinear(*g_ptr.offset(j as isize).offset(0 as libc::c_int as isize)))
                .wrapping_add(
                    GammaToLinear(*g_ptr.offset(j as isize).offset(step as isize)),
                )
                .wrapping_add(
                    GammaToLinear(*g_ptr.offset(j as isize).offset(rgb_stride as isize)),
                )
                .wrapping_add(
                    GammaToLinear(
                        *g_ptr.offset(j as isize).offset((rgb_stride + step) as isize),
                    ),
                ),
            0 as libc::c_int,
        ) as uint16_t;
        *dst
            .offset(
                2 as libc::c_int as isize,
            ) = LinearToGamma(
            (GammaToLinear(*b_ptr.offset(j as isize).offset(0 as libc::c_int as isize)))
                .wrapping_add(
                    GammaToLinear(*b_ptr.offset(j as isize).offset(step as isize)),
                )
                .wrapping_add(
                    GammaToLinear(*b_ptr.offset(j as isize).offset(rgb_stride as isize)),
                )
                .wrapping_add(
                    GammaToLinear(
                        *b_ptr.offset(j as isize).offset((rgb_stride + step) as isize),
                    ),
                ),
            0 as libc::c_int,
        ) as uint16_t;
        i += 1 as libc::c_int;
        j += 2 as libc::c_int * step;
        dst = dst.offset(4 as libc::c_int as isize);
    }
    if width & 1 as libc::c_int != 0 {
        *dst
            .offset(
                0 as libc::c_int as isize,
            ) = LinearToGamma(
            (GammaToLinear(*r_ptr.offset(j as isize).offset(0 as libc::c_int as isize)))
                .wrapping_add(
                    GammaToLinear(*r_ptr.offset(j as isize).offset(rgb_stride as isize)),
                ),
            1 as libc::c_int,
        ) as uint16_t;
        *dst
            .offset(
                1 as libc::c_int as isize,
            ) = LinearToGamma(
            (GammaToLinear(*g_ptr.offset(j as isize).offset(0 as libc::c_int as isize)))
                .wrapping_add(
                    GammaToLinear(*g_ptr.offset(j as isize).offset(rgb_stride as isize)),
                ),
            1 as libc::c_int,
        ) as uint16_t;
        *dst
            .offset(
                2 as libc::c_int as isize,
            ) = LinearToGamma(
            (GammaToLinear(*b_ptr.offset(j as isize).offset(0 as libc::c_int as isize)))
                .wrapping_add(
                    GammaToLinear(*b_ptr.offset(j as isize).offset(rgb_stride as isize)),
                ),
            1 as libc::c_int,
        ) as uint16_t;
    }
}
#[inline]
unsafe extern "C" fn ConvertRowsToUV(
    mut rgb: *const uint16_t,
    dst_u: *mut uint8_t,
    dst_v: *mut uint8_t,
    mut width: libc::c_int,
    rg: *mut VP8Random,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < width {
        let r: libc::c_int = *rgb.offset(0 as libc::c_int as isize) as libc::c_int;
        let g: libc::c_int = *rgb.offset(1 as libc::c_int as isize) as libc::c_int;
        let b: libc::c_int = *rgb.offset(2 as libc::c_int as isize) as libc::c_int;
        *dst_u.offset(i as isize) = RGBToU(r, g, b, rg) as uint8_t;
        *dst_v.offset(i as isize) = RGBToV(r, g, b, rg) as uint8_t;
        i += 1 as libc::c_int;
        rgb = rgb.offset(4 as libc::c_int as isize);
    }
}
unsafe extern "C" fn ImportYUVAFromRGBA(
    mut r_ptr: *const uint8_t,
    mut g_ptr: *const uint8_t,
    mut b_ptr: *const uint8_t,
    mut a_ptr: *const uint8_t,
    mut step: libc::c_int,
    mut rgb_stride: libc::c_int,
    mut dithering: libc::c_float,
    mut use_iterative_conversion: libc::c_int,
    picture: *mut WebPPicture,
) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let width: libc::c_int = (*picture).width;
    let height: libc::c_int = (*picture).height;
    let has_alpha: libc::c_int = CheckNonOpaque(a_ptr, width, height, step, rgb_stride);
    let is_rgb: libc::c_int = (r_ptr < b_ptr) as libc::c_int;
    (*picture)
        .colorspace = (if has_alpha != 0 {
        WEBP_YUV420A as libc::c_int
    } else {
        WEBP_YUV420 as libc::c_int
    }) as WebPEncCSP;
    (*picture).use_argb = 0 as libc::c_int;
    if width < kMinDimensionIterativeConversion
        || height < kMinDimensionIterativeConversion
    {
        use_iterative_conversion = 0 as libc::c_int;
    }
    if WebPPictureAllocYUVA(picture) == 0 {
        return 0 as libc::c_int;
    }
    has_alpha != 0;
    if use_iterative_conversion != 0 {
        crate::sharpyuv::sharpyuv::SharpYuvInit();
        if PreprocessARGB(r_ptr, g_ptr, b_ptr, step, rgb_stride, picture) == 0 {
            return 0 as libc::c_int;
        }
        if has_alpha != 0 {
            WebPExtractAlpha
                .expect(
                    "non-null function pointer",
                )(a_ptr, rgb_stride, width, height, (*picture).a, (*picture).a_stride);
        }
    } else {
        let uv_width: libc::c_int = width + 1 as libc::c_int >> 1 as libc::c_int;
        let mut use_dsp: libc::c_int = (step == 3 as libc::c_int) as libc::c_int;
        let tmp_rgb: *mut uint16_t = WebPSafeMalloc(
            (4 as libc::c_int * uv_width) as uint64_t,
            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
        ) as *mut uint16_t;
        let mut dst_y: *mut uint8_t = (*picture).y;
        let mut dst_u: *mut uint8_t = (*picture).u;
        let mut dst_v: *mut uint8_t = (*picture).v;
        let mut dst_a: *mut uint8_t = (*picture).a;
        let mut base_rg: VP8Random = VP8Random {
            index1_: 0,
            index2_: 0,
            tab_: [0; 55],
            amp_: 0,
        };
        let mut rg: *mut VP8Random = 0 as *mut VP8Random;
        if dithering as libc::c_double > 0.0f64 {
            VP8InitRandom(&mut base_rg, dithering);
            rg = &mut base_rg;
            use_dsp = 0 as libc::c_int;
        }
        WebPInitConvertARGBToYUV();
        InitGammaTables();
        if tmp_rgb.is_null() {
            return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        y = 0 as libc::c_int;
        while y < height >> 1 as libc::c_int {
            let mut rows_have_alpha: libc::c_int = has_alpha;
            if use_dsp != 0 {
                if is_rgb != 0 {
                    WebPConvertRGB24ToY
                        .expect("non-null function pointer")(r_ptr, dst_y, width);
                    WebPConvertRGB24ToY
                        .expect(
                            "non-null function pointer",
                        )(
                        r_ptr.offset(rgb_stride as isize),
                        dst_y.offset((*picture).y_stride as isize),
                        width,
                    );
                } else {
                    WebPConvertBGR24ToY
                        .expect("non-null function pointer")(b_ptr, dst_y, width);
                    WebPConvertBGR24ToY
                        .expect(
                            "non-null function pointer",
                        )(
                        b_ptr.offset(rgb_stride as isize),
                        dst_y.offset((*picture).y_stride as isize),
                        width,
                    );
                }
            } else {
                ConvertRowToY(r_ptr, g_ptr, b_ptr, step, dst_y, width, rg);
                ConvertRowToY(
                    r_ptr.offset(rgb_stride as isize),
                    g_ptr.offset(rgb_stride as isize),
                    b_ptr.offset(rgb_stride as isize),
                    step,
                    dst_y.offset((*picture).y_stride as isize),
                    width,
                    rg,
                );
            }
            dst_y = dst_y.offset((2 as libc::c_int * (*picture).y_stride) as isize);
            if has_alpha != 0 {
                rows_have_alpha
                    &= (WebPExtractAlpha
                        .expect(
                            "non-null function pointer",
                        )(
                        a_ptr,
                        rgb_stride,
                        width,
                        2 as libc::c_int,
                        dst_a,
                        (*picture).a_stride,
                    ) == 0) as libc::c_int;
                dst_a = dst_a.offset((2 as libc::c_int * (*picture).a_stride) as isize);
            }
            if rows_have_alpha == 0 {
                AccumulateRGB(r_ptr, g_ptr, b_ptr, step, rgb_stride, tmp_rgb, width);
            } else {
                AccumulateRGBA(r_ptr, g_ptr, b_ptr, a_ptr, rgb_stride, tmp_rgb, width);
            }
            if rg.is_null() {
                WebPConvertRGBA32ToUV
                    .expect(
                        "non-null function pointer",
                    )(tmp_rgb, dst_u, dst_v, uv_width);
            } else {
                ConvertRowsToUV(tmp_rgb, dst_u, dst_v, uv_width, rg);
            }
            dst_u = dst_u.offset((*picture).uv_stride as isize);
            dst_v = dst_v.offset((*picture).uv_stride as isize);
            r_ptr = r_ptr.offset((2 as libc::c_int * rgb_stride) as isize);
            b_ptr = b_ptr.offset((2 as libc::c_int * rgb_stride) as isize);
            g_ptr = g_ptr.offset((2 as libc::c_int * rgb_stride) as isize);
            if has_alpha != 0 {
                a_ptr = a_ptr.offset((2 as libc::c_int * rgb_stride) as isize);
            }
            y += 1;
            y;
        }
        if height & 1 as libc::c_int != 0 {
            let mut row_has_alpha: libc::c_int = has_alpha;
            if use_dsp != 0 {
                if r_ptr < b_ptr {
                    WebPConvertRGB24ToY
                        .expect("non-null function pointer")(r_ptr, dst_y, width);
                } else {
                    WebPConvertBGR24ToY
                        .expect("non-null function pointer")(b_ptr, dst_y, width);
                }
            } else {
                ConvertRowToY(r_ptr, g_ptr, b_ptr, step, dst_y, width, rg);
            }
            if row_has_alpha != 0 {
                row_has_alpha
                    &= (WebPExtractAlpha
                        .expect(
                            "non-null function pointer",
                        )(
                        a_ptr,
                        0 as libc::c_int,
                        width,
                        1 as libc::c_int,
                        dst_a,
                        0 as libc::c_int,
                    ) == 0) as libc::c_int;
            }
            if row_has_alpha == 0 {
                AccumulateRGB(
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    step,
                    0 as libc::c_int,
                    tmp_rgb,
                    width,
                );
            } else {
                AccumulateRGBA(
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    a_ptr,
                    0 as libc::c_int,
                    tmp_rgb,
                    width,
                );
            }
            if rg.is_null() {
                WebPConvertRGBA32ToUV
                    .expect(
                        "non-null function pointer",
                    )(tmp_rgb, dst_u, dst_v, uv_width);
            } else {
                ConvertRowsToUV(tmp_rgb, dst_u, dst_v, uv_width, rg);
            }
        }
        WebPSafeFree(tmp_rgb as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn PictureARGBToYUVA(
    mut picture: *mut WebPPicture,
    mut colorspace: WebPEncCSP,
    mut dithering: libc::c_float,
    mut use_iterative_conversion: libc::c_int,
) -> libc::c_int {
    if picture.is_null() {
        return 0 as libc::c_int;
    }
    if ((*picture).argb).is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER)
    } else if colorspace as libc::c_uint
        & WEBP_CSP_UV_MASK as libc::c_int as libc::c_uint
        != WEBP_YUV420 as libc::c_int as libc::c_uint
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_INVALID_CONFIGURATION)
    } else {
        let argb: *const uint8_t = (*picture).argb as *const uint8_t;
        let a: *const uint8_t = argb
            .offset((3 as libc::c_int - 0 as libc::c_int) as isize);
        let r: *const uint8_t = argb
            .offset((3 as libc::c_int - 1 as libc::c_int) as isize);
        let g: *const uint8_t = argb
            .offset((3 as libc::c_int - 2 as libc::c_int) as isize);
        let b: *const uint8_t = argb
            .offset((3 as libc::c_int - 3 as libc::c_int) as isize);
        (*picture).colorspace = WEBP_YUV420;
        return ImportYUVAFromRGBA(
            r,
            g,
            b,
            a,
            4 as libc::c_int,
            4 as libc::c_int * (*picture).argb_stride,
            dithering,
            use_iterative_conversion,
            picture,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureARGBToYUVADithered(
    mut picture: *mut WebPPicture,
    mut colorspace: WebPEncCSP,
    mut dithering: libc::c_float,
) -> libc::c_int {
    return PictureARGBToYUVA(picture, colorspace, dithering, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureARGBToYUVA(
    mut picture: *mut WebPPicture,
    mut colorspace: WebPEncCSP,
) -> libc::c_int {
    return PictureARGBToYUVA(picture, colorspace, 0.0f32, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureSharpARGBToYUVA(
    mut picture: *mut WebPPicture,
) -> libc::c_int {
    return PictureARGBToYUVA(picture, WEBP_YUV420, 0.0f32, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureSmartARGBToYUVA(
    mut picture: *mut WebPPicture,
) -> libc::c_int {
    return WebPPictureSharpARGBToYUVA(picture);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureYUVAToARGB(
    mut picture: *mut WebPPicture,
) -> libc::c_int {
    if picture.is_null() {
        return 0 as libc::c_int;
    }
    if ((*picture).y).is_null() || ((*picture).u).is_null() || ((*picture).v).is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER);
    }
    if (*picture).colorspace as libc::c_uint
        & WEBP_CSP_ALPHA_BIT as libc::c_int as libc::c_uint != 0
        && ((*picture).a).is_null()
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER);
    }
    if (*picture).colorspace as libc::c_uint
        & WEBP_CSP_UV_MASK as libc::c_int as libc::c_uint
        != WEBP_YUV420 as libc::c_int as libc::c_uint
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    if WebPPictureAllocARGB(picture) == 0 {
        return 0 as libc::c_int;
    }
    (*picture).use_argb = 1 as libc::c_int;
    let mut y: libc::c_int = 0;
    let width: libc::c_int = (*picture).width;
    let height: libc::c_int = (*picture).height;
    let argb_stride: libc::c_int = 4 as libc::c_int * (*picture).argb_stride;
    let mut dst: *mut uint8_t = (*picture).argb as *mut uint8_t;
    let mut cur_u: *const uint8_t = (*picture).u;
    let mut cur_v: *const uint8_t = (*picture).v;
    let mut cur_y: *const uint8_t = (*picture).y;
    let mut upsample: WebPUpsampleLinePairFunc = WebPGetLinePairConverter(
        (3 as libc::c_int - 0 as libc::c_int > 0 as libc::c_int) as libc::c_int,
    );
    upsample
        .expect(
            "non-null function pointer",
        )(
        cur_y,
        0 as *const uint8_t,
        cur_u,
        cur_v,
        cur_u,
        cur_v,
        dst,
        0 as *mut uint8_t,
        width,
    );
    cur_y = cur_y.offset((*picture).y_stride as isize);
    dst = dst.offset(argb_stride as isize);
    y = 1 as libc::c_int;
    while (y + 1 as libc::c_int) < height {
        let top_u: *const uint8_t = cur_u;
        let top_v: *const uint8_t = cur_v;
        cur_u = cur_u.offset((*picture).uv_stride as isize);
        cur_v = cur_v.offset((*picture).uv_stride as isize);
        upsample
            .expect(
                "non-null function pointer",
            )(
            cur_y,
            cur_y.offset((*picture).y_stride as isize),
            top_u,
            top_v,
            cur_u,
            cur_v,
            dst,
            dst.offset(argb_stride as isize),
            width,
        );
        cur_y = cur_y.offset((2 as libc::c_int * (*picture).y_stride) as isize);
        dst = dst.offset((2 as libc::c_int * argb_stride) as isize);
        y += 2 as libc::c_int;
    }
    if height > 1 as libc::c_int && height & 1 as libc::c_int == 0 {
        upsample
            .expect(
                "non-null function pointer",
            )(
            cur_y,
            0 as *const uint8_t,
            cur_u,
            cur_v,
            cur_u,
            cur_v,
            dst,
            0 as *mut uint8_t,
            width,
        );
    }
    if (*picture).colorspace as libc::c_uint
        & WEBP_CSP_ALPHA_BIT as libc::c_int as libc::c_uint != 0
    {
        y = 0 as libc::c_int;
        while y < height {
            let argb_dst: *mut uint32_t = ((*picture).argb)
                .offset((y * (*picture).argb_stride) as isize);
            let src: *const uint8_t = ((*picture).a)
                .offset((y * (*picture).a_stride) as isize);
            let mut x: libc::c_int = 0;
            x = 0 as libc::c_int;
            while x < width {
                *argb_dst
                    .offset(
                        x as isize,
                    ) = *argb_dst.offset(x as isize) & 0xffffff as libc::c_uint
                    | (*src.offset(x as isize) as uint32_t) << 24 as libc::c_int;
                x += 1;
                x;
            }
            y += 1;
            y;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Import(
    picture: *mut WebPPicture,
    mut rgb: *const uint8_t,
    mut rgb_stride: libc::c_int,
    mut step: libc::c_int,
    mut swap_rb: libc::c_int,
    mut import_alpha: libc::c_int,
) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut r_ptr: *const uint8_t = rgb
        .offset(
            (if swap_rb != 0 { 2 as libc::c_int } else { 0 as libc::c_int }) as isize,
        );
    let mut g_ptr: *const uint8_t = rgb.offset(1 as libc::c_int as isize);
    let mut b_ptr: *const uint8_t = rgb
        .offset(
            (if swap_rb != 0 { 0 as libc::c_int } else { 2 as libc::c_int }) as isize,
        );
    let width: libc::c_int = (*picture).width;
    let height: libc::c_int = (*picture).height;
    if abs(rgb_stride)
        < (if import_alpha != 0 { 4 as libc::c_int } else { 3 as libc::c_int }) * width
    {
        return 0 as libc::c_int;
    }
    if (*picture).use_argb == 0 {
        let mut a_ptr: *const uint8_t = if import_alpha != 0 {
            rgb.offset(3 as libc::c_int as isize)
        } else {
            0 as *const uint8_t
        };
        return ImportYUVAFromRGBA(
            r_ptr,
            g_ptr,
            b_ptr,
            a_ptr,
            step,
            rgb_stride,
            0.0f32,
            0 as libc::c_int,
            picture,
        );
    }
    if WebPPictureAlloc(picture) == 0 {
        return 0 as libc::c_int;
    }
    VP8LDspInit();
    WebPInitAlphaProcessing();
    if import_alpha != 0 {
        let mut dst: *mut uint32_t = (*picture).argb;
        let do_copy: libc::c_int = (3 as libc::c_int - 0 as libc::c_int
            == 3 as libc::c_int && swap_rb != 0) as libc::c_int;
        if do_copy != 0 {
            y = 0 as libc::c_int;
            while y < height {
                memcpy(
                    dst as *mut libc::c_void,
                    rgb as *const libc::c_void,
                    (width * 4 as libc::c_int) as libc::c_ulong,
                );
                rgb = rgb.offset(rgb_stride as isize);
                dst = dst.offset((*picture).argb_stride as isize);
                y += 1;
                y;
            }
        } else {
            y = 0 as libc::c_int;
            while y < height {
                VP8LConvertBGRAToRGBA
                    .expect(
                        "non-null function pointer",
                    )(rgb as *const uint32_t, width, dst as *mut uint8_t);
                rgb = rgb.offset(rgb_stride as isize);
                dst = dst.offset((*picture).argb_stride as isize);
                y += 1;
                y;
            }
        }
    } else {
        let mut dst_0: *mut uint32_t = (*picture).argb;
        y = 0 as libc::c_int;
        while y < height {
            WebPPackRGB
                .expect(
                    "non-null function pointer",
                )(r_ptr, g_ptr, b_ptr, width, step, dst_0);
            r_ptr = r_ptr.offset(rgb_stride as isize);
            g_ptr = g_ptr.offset(rgb_stride as isize);
            b_ptr = b_ptr.offset(rgb_stride as isize);
            dst_0 = dst_0.offset((*picture).argb_stride as isize);
            y += 1;
            y;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportBGR(
    mut picture: *mut WebPPicture,
    mut bgr: *const uint8_t,
    mut bgr_stride: libc::c_int,
) -> libc::c_int {
    return if !picture.is_null() && !bgr.is_null() {
        Import(
            picture,
            bgr,
            bgr_stride,
            3 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        )
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportBGRA(
    mut picture: *mut WebPPicture,
    mut bgra: *const uint8_t,
    mut bgra_stride: libc::c_int,
) -> libc::c_int {
    return if !picture.is_null() && !bgra.is_null() {
        Import(
            picture,
            bgra,
            bgra_stride,
            4 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
        )
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportBGRX(
    mut picture: *mut WebPPicture,
    mut bgrx: *const uint8_t,
    mut bgrx_stride: libc::c_int,
) -> libc::c_int {
    return if !picture.is_null() && !bgrx.is_null() {
        Import(
            picture,
            bgrx,
            bgrx_stride,
            4 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        )
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportRGB(
    mut picture: *mut WebPPicture,
    mut rgb: *const uint8_t,
    mut rgb_stride: libc::c_int,
) -> libc::c_int {
    return if !picture.is_null() && !rgb.is_null() {
        Import(
            picture,
            rgb,
            rgb_stride,
            3 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        )
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportRGBA(
    mut picture: *mut WebPPicture,
    mut rgba: *const uint8_t,
    mut rgba_stride: libc::c_int,
) -> libc::c_int {
    return if !picture.is_null() && !rgba.is_null() {
        Import(
            picture,
            rgba,
            rgba_stride,
            4 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        )
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureImportRGBX(
    mut picture: *mut WebPPicture,
    mut rgbx: *const uint8_t,
    mut rgbx_stride: libc::c_int,
) -> libc::c_int {
    return if !picture.is_null() && !rgbx.is_null() {
        Import(
            picture,
            rgbx,
            rgbx_stride,
            4 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        )
    } else {
        0 as libc::c_int
    };
}
