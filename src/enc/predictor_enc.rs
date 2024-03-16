use ::libc;
extern "C" {
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
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut VP8LPredictors: [VP8LPredictorFunc; 16];
    static mut VP8LTransformColor: VP8LTransformColorFunc;
    static mut VP8LCollectColorBlueTransforms: VP8LCollectColorBlueTransformsFunc;
    static mut VP8LCollectColorRedTransforms: VP8LCollectColorRedTransformsFunc;
    static mut VP8LPredictorsSub: [VP8LPredictorAddSubFunc; 16];
    static mut VP8LCombinedShannonEntropy: VP8LCombinedShannonEntropyFunc;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPPicture {
    pub use_argb: libc::c_int,
    pub colorspace: WebPEncCSP,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub y_stride: libc::c_int,
    pub uv_stride: libc::c_int,
    pub a: *mut uint8_t,
    pub a_stride: libc::c_int,
    pub pad1: [uint32_t; 2],
    pub argb: *mut uint32_t,
    pub argb_stride: libc::c_int,
    pub pad2: [uint32_t; 3],
    pub writer: WebPWriterFunction,
    pub custom_ptr: *mut libc::c_void,
    pub extra_info_type: libc::c_int,
    pub extra_info: *mut uint8_t,
    pub stats: *mut WebPAuxStats,
    pub error_code: WebPEncodingError,
    pub progress_hook: WebPProgressHook,
    pub user_data: *mut libc::c_void,
    pub pad3: [uint32_t; 3],
    pub pad4: *mut uint8_t,
    pub pad5: *mut uint8_t,
    pub pad6: [uint32_t; 8],
    pub memory_: *mut libc::c_void,
    pub memory_argb_: *mut libc::c_void,
    pub pad7: [*mut libc::c_void; 2],
}
pub type WebPProgressHook = Option::<
    unsafe extern "C" fn(libc::c_int, *const WebPPicture) -> libc::c_int,
>;
pub type WebPEncodingError = libc::c_uint;
pub const VP8_ENC_ERROR_LAST: WebPEncodingError = 11;
pub const VP8_ENC_ERROR_USER_ABORT: WebPEncodingError = 10;
pub const VP8_ENC_ERROR_FILE_TOO_BIG: WebPEncodingError = 9;
pub const VP8_ENC_ERROR_BAD_WRITE: WebPEncodingError = 8;
pub const VP8_ENC_ERROR_PARTITION_OVERFLOW: WebPEncodingError = 7;
pub const VP8_ENC_ERROR_PARTITION0_OVERFLOW: WebPEncodingError = 6;
pub const VP8_ENC_ERROR_BAD_DIMENSION: WebPEncodingError = 5;
pub const VP8_ENC_ERROR_INVALID_CONFIGURATION: WebPEncodingError = 4;
pub const VP8_ENC_ERROR_NULL_PARAMETER: WebPEncodingError = 3;
pub const VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY: WebPEncodingError = 2;
pub const VP8_ENC_ERROR_OUT_OF_MEMORY: WebPEncodingError = 1;
pub const VP8_ENC_OK: WebPEncodingError = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMultipliers {
    pub green_to_red_: uint8_t,
    pub green_to_blue_: uint8_t,
    pub red_to_blue_: uint8_t,
}
pub type VP8LTransformColorFunc = Option::<
    unsafe extern "C" fn(*const VP8LMultipliers, *mut uint32_t, libc::c_int) -> (),
>;
pub type VP8LCollectColorBlueTransformsFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
>;
pub type VP8LCollectColorRedTransformsFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
>;
pub type VP8LCombinedShannonEntropyFunc = Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> libc::c_float,
>;
#[inline]
unsafe extern "C" fn VP8LSubPixels(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let alpha_and_green: uint32_t = (0xff00ff as libc::c_uint)
        .wrapping_add(a & 0xff00ff00 as libc::c_uint)
        .wrapping_sub(b & 0xff00ff00 as libc::c_uint);
    let red_and_blue: uint32_t = (0xff00ff00 as libc::c_uint)
        .wrapping_add(a & 0xff00ff as libc::c_uint)
        .wrapping_sub(b & 0xff00ff as libc::c_uint);
    return alpha_and_green & 0xff00ff00 as libc::c_uint
        | red_and_blue & 0xff00ff as libc::c_uint;
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
unsafe extern "C" fn VP8LNearLosslessBits(
    mut near_lossless_quality: libc::c_int,
) -> libc::c_int {
    return 5 as libc::c_int - near_lossless_quality / 20 as libc::c_int;
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
static mut kSpatialPredictorBias: libc::c_float = 15.0f32;
static mut kPredLowEffort: libc::c_int = 11 as libc::c_int;
static mut kMaskAlpha: uint32_t = 0xff000000 as libc::c_uint;
#[inline]
unsafe extern "C" fn GetMin(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { b } else { a };
}
unsafe extern "C" fn PredictionCostSpatial(
    mut counts: *const libc::c_int,
    mut weight_0: libc::c_int,
    mut exp_val: libc::c_float,
) -> libc::c_float {
    let significant_symbols: libc::c_int = 256 as libc::c_int >> 4 as libc::c_int;
    let exp_decay_factor: libc::c_float = 0.6f32;
    let mut bits: libc::c_float = weight_0 as libc::c_float
        * *counts.offset(0 as libc::c_int as isize) as libc::c_float;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < significant_symbols {
        bits
            += exp_val
                * (*counts.offset(i as isize)
                    + *counts.offset((256 as libc::c_int - i) as isize))
                    as libc::c_float;
        exp_val *= exp_decay_factor;
        i += 1;
        i;
    }
    return (-0.1f64 * bits as libc::c_double) as libc::c_float;
}
unsafe extern "C" fn PredictionCostSpatialHistogram(
    mut accumulated: *const [libc::c_int; 256],
    mut tile: *const [libc::c_int; 256],
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut retval: libc::c_float = 0.0f32;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let kExpValue: libc::c_float = 0.94f32;
        retval
            += PredictionCostSpatial(
                (*tile.offset(i as isize)).as_ptr(),
                1 as libc::c_int,
                kExpValue,
            );
        retval
            += VP8LCombinedShannonEntropy
                .expect(
                    "non-null function pointer",
                )(
                (*tile.offset(i as isize)).as_ptr(),
                (*accumulated.offset(i as isize)).as_ptr(),
            );
        i += 1;
        i;
    }
    return retval;
}
#[inline]
unsafe extern "C" fn UpdateHisto(
    mut histo_argb: *mut [libc::c_int; 256],
    mut argb: uint32_t,
) {
    let ref mut fresh0 = (*histo_argb
        .offset(0 as libc::c_int as isize))[(argb >> 24 as libc::c_int) as usize];
    *fresh0 += 1;
    *fresh0;
    let ref mut fresh1 = (*histo_argb
        .offset(
            1 as libc::c_int as isize,
        ))[(argb >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize];
    *fresh1 += 1;
    *fresh1;
    let ref mut fresh2 = (*histo_argb
        .offset(
            2 as libc::c_int as isize,
        ))[(argb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as usize];
    *fresh2 += 1;
    *fresh2;
    let ref mut fresh3 = (*histo_argb
        .offset(
            3 as libc::c_int as isize,
        ))[(argb & 0xff as libc::c_int as libc::c_uint) as usize];
    *fresh3 += 1;
    *fresh3;
}
#[inline]
unsafe extern "C" fn PredictBatch(
    mut mode: libc::c_int,
    mut x_start: libc::c_int,
    mut y: libc::c_int,
    mut num_pixels: libc::c_int,
    mut current: *const uint32_t,
    mut upper: *const uint32_t,
    mut out: *mut uint32_t,
) {
    if x_start == 0 as libc::c_int {
        if y == 0 as libc::c_int {
            (VP8LPredictorsSub[0 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(current, 0 as *const uint32_t, 1 as libc::c_int, out);
        } else {
            (VP8LPredictorsSub[2 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(current, upper, 1 as libc::c_int, out);
        }
        x_start += 1;
        x_start;
        out = out.offset(1);
        out;
        num_pixels -= 1;
        num_pixels;
    }
    if y == 0 as libc::c_int {
        (VP8LPredictorsSub[1 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(current.offset(x_start as isize), 0 as *const uint32_t, num_pixels, out);
    } else {
        (VP8LPredictorsSub[mode as usize])
            .expect(
                "non-null function pointer",
            )(
            current.offset(x_start as isize),
            upper.offset(x_start as isize),
            num_pixels,
            out,
        );
    };
}
#[inline]
unsafe extern "C" fn GetMax(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { b } else { a };
}
unsafe extern "C" fn MaxDiffBetweenPixels(
    mut p1: uint32_t,
    mut p2: uint32_t,
) -> libc::c_int {
    let diff_a: libc::c_int = abs(
        (p1 >> 24 as libc::c_int) as libc::c_int
            - (p2 >> 24 as libc::c_int) as libc::c_int,
    );
    let diff_r: libc::c_int = abs(
        (p1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int
            - (p2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_int,
    );
    let diff_g: libc::c_int = abs(
        (p1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int
            - (p2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_int,
    );
    let diff_b: libc::c_int = abs(
        (p1 & 0xff as libc::c_int as libc::c_uint) as libc::c_int
            - (p2 & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
    );
    return GetMax(GetMax(diff_a, diff_r), GetMax(diff_g, diff_b));
}
unsafe extern "C" fn MaxDiffAroundPixel(
    mut current: uint32_t,
    mut up: uint32_t,
    mut down: uint32_t,
    mut left: uint32_t,
    mut right: uint32_t,
) -> libc::c_int {
    let diff_up: libc::c_int = MaxDiffBetweenPixels(current, up);
    let diff_down: libc::c_int = MaxDiffBetweenPixels(current, down);
    let diff_left: libc::c_int = MaxDiffBetweenPixels(current, left);
    let diff_right: libc::c_int = MaxDiffBetweenPixels(current, right);
    return GetMax(GetMax(diff_up, diff_down), GetMax(diff_left, diff_right));
}
unsafe extern "C" fn AddGreenToBlueAndRed(mut argb: uint32_t) -> uint32_t {
    let green: uint32_t = argb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
    let mut red_blue: uint32_t = argb & 0xff00ff as libc::c_uint;
    red_blue = (red_blue as libc::c_uint)
        .wrapping_add(green << 16 as libc::c_int | green) as uint32_t as uint32_t;
    red_blue &= 0xff00ff as libc::c_uint;
    return argb & 0xff00ff00 as libc::c_uint | red_blue;
}
unsafe extern "C" fn MaxDiffsForRow(
    mut width: libc::c_int,
    mut stride: libc::c_int,
    argb: *const uint32_t,
    max_diffs: *mut uint8_t,
    mut used_subtract_green: libc::c_int,
) {
    let mut current: uint32_t = 0;
    let mut up: uint32_t = 0;
    let mut down: uint32_t = 0;
    let mut left: uint32_t = 0;
    let mut right: uint32_t = 0;
    let mut x: libc::c_int = 0;
    if width <= 2 as libc::c_int {
        return;
    }
    current = *argb.offset(0 as libc::c_int as isize);
    right = *argb.offset(1 as libc::c_int as isize);
    if used_subtract_green != 0 {
        current = AddGreenToBlueAndRed(current);
        right = AddGreenToBlueAndRed(right);
    }
    x = 1 as libc::c_int;
    while x < width - 1 as libc::c_int {
        up = *argb.offset((-stride + x) as isize);
        down = *argb.offset((stride + x) as isize);
        left = current;
        current = right;
        right = *argb.offset((x + 1 as libc::c_int) as isize);
        if used_subtract_green != 0 {
            up = AddGreenToBlueAndRed(up);
            down = AddGreenToBlueAndRed(down);
            right = AddGreenToBlueAndRed(right);
        }
        *max_diffs
            .offset(
                x as isize,
            ) = MaxDiffAroundPixel(current, up, down, left, right) as uint8_t;
        x += 1;
        x;
    }
}
unsafe extern "C" fn NearLosslessComponent(
    mut value: uint8_t,
    mut predict: uint8_t,
    mut boundary: uint8_t,
    mut quantization: libc::c_int,
) -> uint8_t {
    let residual: libc::c_int = value as libc::c_int - predict as libc::c_int
        & 0xff as libc::c_int;
    let boundary_residual: libc::c_int = boundary as libc::c_int - predict as libc::c_int
        & 0xff as libc::c_int;
    let lower: libc::c_int = residual & !(quantization - 1 as libc::c_int);
    let upper: libc::c_int = lower + quantization;
    let bias: libc::c_int = ((boundary as libc::c_int - value as libc::c_int
        & 0xff as libc::c_int) < boundary_residual) as libc::c_int;
    if residual - lower < upper - residual + bias {
        if residual > boundary_residual && lower <= boundary_residual {
            return (lower + (quantization >> 1 as libc::c_int)) as uint8_t;
        }
        return lower as uint8_t;
    } else {
        if residual <= boundary_residual && upper > boundary_residual {
            return (lower + (quantization >> 1 as libc::c_int)) as uint8_t;
        }
        return (upper & 0xff as libc::c_int) as uint8_t;
    };
}
#[inline]
unsafe extern "C" fn NearLosslessDiff(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    return (a as libc::c_int - b as libc::c_int & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn NearLossless(
    mut value: uint32_t,
    mut predict: uint32_t,
    mut max_quantization: libc::c_int,
    mut max_diff: libc::c_int,
    mut used_subtract_green: libc::c_int,
) -> uint32_t {
    let mut quantization: libc::c_int = 0;
    let mut new_green: uint8_t = 0 as libc::c_int as uint8_t;
    let mut green_diff: uint8_t = 0 as libc::c_int as uint8_t;
    let mut a: uint8_t = 0;
    let mut r: uint8_t = 0;
    let mut g: uint8_t = 0;
    let mut b: uint8_t = 0;
    if max_diff <= 2 as libc::c_int {
        return VP8LSubPixels(value, predict);
    }
    quantization = max_quantization;
    while quantization >= max_diff {
        quantization >>= 1 as libc::c_int;
    }
    if value >> 24 as libc::c_int == 0 as libc::c_int as libc::c_uint
        || value >> 24 as libc::c_int == 0xff as libc::c_int as libc::c_uint
    {
        a = NearLosslessDiff(
            (value >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t,
            (predict >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t,
        );
    } else {
        a = NearLosslessComponent(
            (value >> 24 as libc::c_int) as uint8_t,
            (predict >> 24 as libc::c_int) as uint8_t,
            0xff as libc::c_int as uint8_t,
            quantization,
        );
    }
    g = NearLosslessComponent(
        (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        (predict >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        0xff as libc::c_int as uint8_t,
        quantization,
    );
    if used_subtract_green != 0 {
        new_green = ((predict >> 8 as libc::c_int).wrapping_add(g as libc::c_uint)
            & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        green_diff = NearLosslessDiff(
            new_green,
            (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        );
    }
    r = NearLosslessComponent(
        NearLosslessDiff(
            (value >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t,
            green_diff,
        ),
        (predict >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        (0xff as libc::c_int - new_green as libc::c_int) as uint8_t,
        quantization,
    );
    b = NearLosslessComponent(
        NearLosslessDiff(
            (value & 0xff as libc::c_int as libc::c_uint) as uint8_t,
            green_diff,
        ),
        (predict & 0xff as libc::c_int as libc::c_uint) as uint8_t,
        (0xff as libc::c_int - new_green as libc::c_int) as uint8_t,
        quantization,
    );
    return (a as uint32_t) << 24 as libc::c_int | (r as uint32_t) << 16 as libc::c_int
        | (g as uint32_t) << 8 as libc::c_int | b as libc::c_uint;
}
#[inline]
unsafe extern "C" fn GetResidual(
    mut width: libc::c_int,
    mut height: libc::c_int,
    upper_row: *mut uint32_t,
    current_row: *mut uint32_t,
    max_diffs: *const uint8_t,
    mut mode: libc::c_int,
    mut x_start: libc::c_int,
    mut x_end: libc::c_int,
    mut y: libc::c_int,
    mut max_quantization: libc::c_int,
    mut exact: libc::c_int,
    mut used_subtract_green: libc::c_int,
    out: *mut uint32_t,
) {
    if exact != 0 {
        PredictBatch(mode, x_start, y, x_end - x_start, current_row, upper_row, out);
    } else {
        let pred_func: VP8LPredictorFunc = VP8LPredictors[mode as usize];
        let mut x: libc::c_int = 0;
        x = x_start;
        while x < x_end {
            let mut predict: uint32_t = 0;
            let mut residual: uint32_t = 0;
            if y == 0 as libc::c_int {
                predict = if x == 0 as libc::c_int {
                    0xff000000 as libc::c_uint
                } else {
                    *current_row.offset((x - 1 as libc::c_int) as isize)
                };
            } else if x == 0 as libc::c_int {
                predict = *upper_row.offset(x as isize);
            } else {
                predict = pred_func
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *current_row.offset((x - 1 as libc::c_int) as isize),
                    upper_row.offset(x as isize),
                );
            }
            if max_quantization == 1 as libc::c_int || mode == 0 as libc::c_int
                || y == 0 as libc::c_int || y == height - 1 as libc::c_int
                || x == 0 as libc::c_int || x == width - 1 as libc::c_int
            {
                residual = VP8LSubPixels(*current_row.offset(x as isize), predict);
            } else {
                residual = NearLossless(
                    *current_row.offset(x as isize),
                    predict,
                    max_quantization,
                    *max_diffs.offset(x as isize) as libc::c_int,
                    used_subtract_green,
                );
                *current_row.offset(x as isize) = VP8LAddPixels(predict, residual);
            }
            if *current_row.offset(x as isize) & kMaskAlpha
                == 0 as libc::c_int as libc::c_uint
            {
                residual &= kMaskAlpha;
                *current_row.offset(x as isize) = predict & !kMaskAlpha;
                if x == 0 as libc::c_int && y != 0 as libc::c_int {
                    *upper_row
                        .offset(
                            width as isize,
                        ) = *current_row.offset(0 as libc::c_int as isize);
                }
            }
            *out.offset((x - x_start) as isize) = residual;
            x += 1;
            x;
        }
    };
}
unsafe extern "C" fn GetBestPredictorForTile(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tile_x: libc::c_int,
    mut tile_y: libc::c_int,
    mut bits: libc::c_int,
    mut accumulated: *mut [libc::c_int; 256],
    argb_scratch: *mut uint32_t,
    argb: *const uint32_t,
    mut max_quantization: libc::c_int,
    mut exact: libc::c_int,
    mut used_subtract_green: libc::c_int,
    modes: *const uint32_t,
) -> libc::c_int {
    let kNumPredModes: libc::c_int = 14 as libc::c_int;
    let start_x: libc::c_int = tile_x << bits;
    let start_y: libc::c_int = tile_y << bits;
    let tile_size: libc::c_int = (1 as libc::c_int) << bits;
    let max_y: libc::c_int = GetMin(tile_size, height - start_y);
    let max_x: libc::c_int = GetMin(tile_size, width - start_x);
    let have_left: libc::c_int = (start_x > 0 as libc::c_int) as libc::c_int;
    let context_start_x: libc::c_int = start_x - have_left;
    let context_width: libc::c_int = max_x + have_left
        + (max_x < width - start_x) as libc::c_int;
    let tiles_per_row: libc::c_int = VP8LSubSampleSize(
        width as uint32_t,
        bits as uint32_t,
    ) as libc::c_int;
    let left_mode: libc::c_int = (if tile_x > 0 as libc::c_int {
        *modes.offset((tile_y * tiles_per_row + tile_x - 1 as libc::c_int) as isize)
            >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint
    } else {
        0xff as libc::c_int as libc::c_uint
    }) as libc::c_int;
    let above_mode: libc::c_int = (if tile_y > 0 as libc::c_int {
        *modes.offset(((tile_y - 1 as libc::c_int) * tiles_per_row + tile_x) as isize)
            >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint
    } else {
        0xff as libc::c_int as libc::c_uint
    }) as libc::c_int;
    let mut upper_row: *mut uint32_t = argb_scratch;
    let mut current_row: *mut uint32_t = upper_row
        .offset(width as isize)
        .offset(1 as libc::c_int as isize);
    let max_diffs: *mut uint8_t = current_row
        .offset(width as isize)
        .offset(1 as libc::c_int as isize) as *mut uint8_t;
    let mut best_diff: libc::c_float = 1e30f32;
    let mut best_mode: libc::c_int = 0 as libc::c_int;
    let mut mode: libc::c_int = 0;
    let mut histo_stack_1: [[libc::c_int; 256]; 4] = [[0; 256]; 4];
    let mut histo_stack_2: [[libc::c_int; 256]; 4] = [[0; 256]; 4];
    let mut histo_argb: *mut [libc::c_int; 256] = histo_stack_1.as_mut_ptr();
    let mut best_histo: *mut [libc::c_int; 256] = histo_stack_2.as_mut_ptr();
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut residuals: [uint32_t; 64] = [0; 64];
    mode = 0 as libc::c_int;
    while mode < kNumPredModes {
        let mut cur_diff: libc::c_float = 0.;
        let mut relative_y: libc::c_int = 0;
        memset(
            histo_argb as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[[libc::c_int; 256]; 4]>() as libc::c_ulong,
        );
        if start_y > 0 as libc::c_int {
            memcpy(
                current_row.offset(context_start_x as isize) as *mut libc::c_void,
                argb
                    .offset(((start_y - 1 as libc::c_int) * width) as isize)
                    .offset(context_start_x as isize) as *const libc::c_void,
                (::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(
                        (max_x + have_left + 1 as libc::c_int) as libc::c_ulong,
                    ),
            );
        }
        relative_y = 0 as libc::c_int;
        while relative_y < max_y {
            let y: libc::c_int = start_y + relative_y;
            let mut relative_x: libc::c_int = 0;
            let mut tmp: *mut uint32_t = upper_row;
            upper_row = current_row;
            current_row = tmp;
            memcpy(
                current_row.offset(context_start_x as isize) as *mut libc::c_void,
                argb.offset((y * width) as isize).offset(context_start_x as isize)
                    as *const libc::c_void,
                (::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(
                        (max_x + have_left
                            + ((y + 1 as libc::c_int) < height) as libc::c_int)
                            as libc::c_ulong,
                    ),
            );
            if max_quantization > 1 as libc::c_int && y >= 1 as libc::c_int
                && (y + 1 as libc::c_int) < height
            {
                MaxDiffsForRow(
                    context_width,
                    width,
                    argb.offset((y * width) as isize).offset(context_start_x as isize),
                    max_diffs.offset(context_start_x as isize),
                    used_subtract_green,
                );
            }
            GetResidual(
                width,
                height,
                upper_row,
                current_row,
                max_diffs,
                mode,
                start_x,
                start_x + max_x,
                y,
                max_quantization,
                exact,
                used_subtract_green,
                residuals.as_mut_ptr(),
            );
            relative_x = 0 as libc::c_int;
            while relative_x < max_x {
                UpdateHisto(histo_argb, residuals[relative_x as usize]);
                relative_x += 1;
                relative_x;
            }
            relative_y += 1;
            relative_y;
        }
        cur_diff = PredictionCostSpatialHistogram(
            accumulated as *const [libc::c_int; 256],
            histo_argb as *const [libc::c_int; 256],
        );
        if mode == left_mode {
            cur_diff -= kSpatialPredictorBias;
        }
        if mode == above_mode {
            cur_diff -= kSpatialPredictorBias;
        }
        if cur_diff < best_diff {
            let mut tmp_0: *mut [libc::c_int; 256] = histo_argb;
            histo_argb = best_histo;
            best_histo = tmp_0;
            best_diff = cur_diff;
            best_mode = mode;
        }
        mode += 1;
        mode;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int {
            (*accumulated.offset(i as isize))[j as usize]
                += (*best_histo.offset(i as isize))[j as usize];
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return best_mode;
}
unsafe extern "C" fn CopyImageWithPrediction(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut bits: libc::c_int,
    modes: *mut uint32_t,
    argb_scratch: *mut uint32_t,
    argb: *mut uint32_t,
    mut low_effort: libc::c_int,
    mut max_quantization: libc::c_int,
    mut exact: libc::c_int,
    mut used_subtract_green: libc::c_int,
) {
    let tiles_per_row: libc::c_int = VP8LSubSampleSize(
        width as uint32_t,
        bits as uint32_t,
    ) as libc::c_int;
    let mut upper_row: *mut uint32_t = argb_scratch;
    let mut current_row: *mut uint32_t = upper_row
        .offset(width as isize)
        .offset(1 as libc::c_int as isize);
    let mut current_max_diffs: *mut uint8_t = current_row
        .offset(width as isize)
        .offset(1 as libc::c_int as isize) as *mut uint8_t;
    let mut lower_max_diffs: *mut uint8_t = current_max_diffs.offset(width as isize);
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0;
        let tmp32: *mut uint32_t = upper_row;
        upper_row = current_row;
        current_row = tmp32;
        memcpy(
            current_row as *mut libc::c_void,
            argb.offset((y * width) as isize) as *const libc::c_void,
            (::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(
                    (width + ((y + 1 as libc::c_int) < height) as libc::c_int)
                        as libc::c_ulong,
                ),
        );
        if low_effort != 0 {
            PredictBatch(
                kPredLowEffort,
                0 as libc::c_int,
                y,
                width,
                current_row,
                upper_row,
                argb.offset((y * width) as isize),
            );
        } else {
            if max_quantization > 1 as libc::c_int {
                let tmp8: *mut uint8_t = current_max_diffs;
                current_max_diffs = lower_max_diffs;
                lower_max_diffs = tmp8;
                if (y + 2 as libc::c_int) < height {
                    MaxDiffsForRow(
                        width,
                        width,
                        argb.offset(((y + 1 as libc::c_int) * width) as isize),
                        lower_max_diffs,
                        used_subtract_green,
                    );
                }
            }
            x = 0 as libc::c_int;
            while x < width {
                let mode: libc::c_int = (*modes
                    .offset(((y >> bits) * tiles_per_row + (x >> bits)) as isize)
                    >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let mut x_end: libc::c_int = x + ((1 as libc::c_int) << bits);
                if x_end > width {
                    x_end = width;
                }
                GetResidual(
                    width,
                    height,
                    upper_row,
                    current_row,
                    current_max_diffs,
                    mode,
                    x,
                    x_end,
                    y,
                    max_quantization,
                    exact,
                    used_subtract_green,
                    argb.offset((y * width) as isize).offset(x as isize),
                );
                x = x_end;
            }
        }
        y += 1;
        y;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LResidualImage(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut bits: libc::c_int,
    mut low_effort: libc::c_int,
    argb: *mut uint32_t,
    argb_scratch: *mut uint32_t,
    image: *mut uint32_t,
    mut near_lossless_quality: libc::c_int,
    mut exact: libc::c_int,
    mut used_subtract_green: libc::c_int,
    pic: *const WebPPicture,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let tiles_per_row: libc::c_int = VP8LSubSampleSize(
        width as uint32_t,
        bits as uint32_t,
    ) as libc::c_int;
    let tiles_per_col: libc::c_int = VP8LSubSampleSize(
        height as uint32_t,
        bits as uint32_t,
    ) as libc::c_int;
    let mut percent_start: libc::c_int = *percent;
    let mut tile_y: libc::c_int = 0;
    let mut histo: [[libc::c_int; 256]; 4] = [[0; 256]; 4];
    let max_quantization: libc::c_int = (1 as libc::c_int)
        << VP8LNearLosslessBits(near_lossless_quality);
    if low_effort != 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < tiles_per_row * tiles_per_col {
            *image
                .offset(
                    i as isize,
                ) = 0xff000000 as libc::c_uint
                | (kPredLowEffort << 8 as libc::c_int) as libc::c_uint;
            i += 1;
            i;
        }
    } else {
        memset(
            histo.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[[libc::c_int; 256]; 4]>() as libc::c_ulong,
        );
        tile_y = 0 as libc::c_int;
        while tile_y < tiles_per_col {
            let mut tile_x: libc::c_int = 0;
            tile_x = 0 as libc::c_int;
            while tile_x < tiles_per_row {
                let pred: libc::c_int = GetBestPredictorForTile(
                    width,
                    height,
                    tile_x,
                    tile_y,
                    bits,
                    histo.as_mut_ptr(),
                    argb_scratch,
                    argb,
                    max_quantization,
                    exact,
                    used_subtract_green,
                    image,
                );
                *image
                    .offset(
                        (tile_y * tiles_per_row + tile_x) as isize,
                    ) = 0xff000000 as libc::c_uint
                    | (pred << 8 as libc::c_int) as libc::c_uint;
                tile_x += 1;
                tile_x;
            }
            if WebPReportProgress(
                pic,
                percent_start + percent_range * tile_y / tiles_per_col,
                percent,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            tile_y += 1;
            tile_y;
        }
    }
    CopyImageWithPrediction(
        width,
        height,
        bits,
        image,
        argb_scratch,
        argb,
        low_effort,
        max_quantization,
        exact,
        used_subtract_green,
    );
    return WebPReportProgress(pic, percent_start + percent_range, percent);
}
#[inline]
unsafe extern "C" fn MultipliersClear(m: *mut VP8LMultipliers) {
    (*m).green_to_red_ = 0 as libc::c_int as uint8_t;
    (*m).green_to_blue_ = 0 as libc::c_int as uint8_t;
    (*m).red_to_blue_ = 0 as libc::c_int as uint8_t;
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
#[inline]
unsafe extern "C" fn MultipliersToColorCode(m: *const VP8LMultipliers) -> uint32_t {
    return 0xff000000 as libc::c_uint
        | ((*m).red_to_blue_ as uint32_t) << 16 as libc::c_int
        | ((*m).green_to_blue_ as uint32_t) << 8 as libc::c_int
        | (*m).green_to_red_ as libc::c_uint;
}
unsafe extern "C" fn PredictionCostCrossColor(
    mut accumulated: *const libc::c_int,
    mut counts: *const libc::c_int,
) -> libc::c_float {
    static mut kExpValue: libc::c_float = 2.4f32;
    return VP8LCombinedShannonEntropy
        .expect("non-null function pointer")(counts, accumulated)
        + PredictionCostSpatial(counts, 3 as libc::c_int, kExpValue);
}
unsafe extern "C" fn GetPredictionCostCrossColorRed(
    mut argb: *const uint32_t,
    mut stride: libc::c_int,
    mut tile_width: libc::c_int,
    mut tile_height: libc::c_int,
    mut prev_x: VP8LMultipliers,
    mut prev_y: VP8LMultipliers,
    mut green_to_red: libc::c_int,
    mut accumulated_red_histo: *const libc::c_int,
) -> libc::c_float {
    let mut histo: [libc::c_int; 256] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut cur_diff: libc::c_float = 0.;
    VP8LCollectColorRedTransforms
        .expect(
            "non-null function pointer",
        )(argb, stride, tile_width, tile_height, green_to_red, histo.as_mut_ptr());
    cur_diff = PredictionCostCrossColor(
        accumulated_red_histo,
        histo.as_mut_ptr() as *const libc::c_int,
    );
    if green_to_red as uint8_t as libc::c_int == prev_x.green_to_red_ as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    if green_to_red as uint8_t as libc::c_int == prev_y.green_to_red_ as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    if green_to_red == 0 as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    return cur_diff;
}
unsafe extern "C" fn GetBestGreenToRed(
    mut argb: *const uint32_t,
    mut stride: libc::c_int,
    mut tile_width: libc::c_int,
    mut tile_height: libc::c_int,
    mut prev_x: VP8LMultipliers,
    mut prev_y: VP8LMultipliers,
    mut quality: libc::c_int,
    mut accumulated_red_histo: *const libc::c_int,
    best_tx: *mut VP8LMultipliers,
) {
    let kMaxIters: libc::c_int = 4 as libc::c_int
        + (7 as libc::c_int * quality >> 8 as libc::c_int);
    let mut green_to_red_best: libc::c_int = 0 as libc::c_int;
    let mut iter: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut best_diff: libc::c_float = GetPredictionCostCrossColorRed(
        argb,
        stride,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        green_to_red_best,
        accumulated_red_histo,
    );
    iter = 0 as libc::c_int;
    while iter < kMaxIters {
        let delta: libc::c_int = 32 as libc::c_int >> iter;
        offset = -delta;
        while offset <= delta {
            let green_to_red_cur: libc::c_int = offset + green_to_red_best;
            let cur_diff: libc::c_float = GetPredictionCostCrossColorRed(
                argb,
                stride,
                tile_width,
                tile_height,
                prev_x,
                prev_y,
                green_to_red_cur,
                accumulated_red_histo,
            );
            if cur_diff < best_diff {
                best_diff = cur_diff;
                green_to_red_best = green_to_red_cur;
            }
            offset += 2 as libc::c_int * delta;
        }
        iter += 1;
        iter;
    }
    (*best_tx).green_to_red_ = (green_to_red_best & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn GetPredictionCostCrossColorBlue(
    mut argb: *const uint32_t,
    mut stride: libc::c_int,
    mut tile_width: libc::c_int,
    mut tile_height: libc::c_int,
    mut prev_x: VP8LMultipliers,
    mut prev_y: VP8LMultipliers,
    mut green_to_blue: libc::c_int,
    mut red_to_blue: libc::c_int,
    mut accumulated_blue_histo: *const libc::c_int,
) -> libc::c_float {
    let mut histo: [libc::c_int; 256] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut cur_diff: libc::c_float = 0.;
    VP8LCollectColorBlueTransforms
        .expect(
            "non-null function pointer",
        )(
        argb,
        stride,
        tile_width,
        tile_height,
        green_to_blue,
        red_to_blue,
        histo.as_mut_ptr(),
    );
    cur_diff = PredictionCostCrossColor(
        accumulated_blue_histo,
        histo.as_mut_ptr() as *const libc::c_int,
    );
    if green_to_blue as uint8_t as libc::c_int == prev_x.green_to_blue_ as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    if green_to_blue as uint8_t as libc::c_int == prev_y.green_to_blue_ as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    if red_to_blue as uint8_t as libc::c_int == prev_x.red_to_blue_ as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    if red_to_blue as uint8_t as libc::c_int == prev_y.red_to_blue_ as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    if green_to_blue == 0 as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    if red_to_blue == 0 as libc::c_int {
        cur_diff -= 3 as libc::c_int as libc::c_float;
    }
    return cur_diff;
}
unsafe extern "C" fn GetBestGreenRedToBlue(
    mut argb: *const uint32_t,
    mut stride: libc::c_int,
    mut tile_width: libc::c_int,
    mut tile_height: libc::c_int,
    mut prev_x: VP8LMultipliers,
    mut prev_y: VP8LMultipliers,
    mut quality: libc::c_int,
    mut accumulated_blue_histo: *const libc::c_int,
    best_tx: *mut VP8LMultipliers,
) {
    let offset: [[int8_t; 2]; 8] = [
        [0 as libc::c_int as int8_t, -(1 as libc::c_int) as int8_t],
        [0 as libc::c_int as int8_t, 1 as libc::c_int as int8_t],
        [-(1 as libc::c_int) as int8_t, 0 as libc::c_int as int8_t],
        [1 as libc::c_int as int8_t, 0 as libc::c_int as int8_t],
        [-(1 as libc::c_int) as int8_t, -(1 as libc::c_int) as int8_t],
        [-(1 as libc::c_int) as int8_t, 1 as libc::c_int as int8_t],
        [1 as libc::c_int as int8_t, -(1 as libc::c_int) as int8_t],
        [1 as libc::c_int as int8_t, 1 as libc::c_int as int8_t],
    ];
    let delta_lut: [int8_t; 7] = [
        16 as libc::c_int as int8_t,
        16 as libc::c_int as int8_t,
        8 as libc::c_int as int8_t,
        4 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
    ];
    let iters: libc::c_int = if quality < 25 as libc::c_int {
        1 as libc::c_int
    } else if quality > 50 as libc::c_int {
        7 as libc::c_int
    } else {
        4 as libc::c_int
    };
    let mut green_to_blue_best: libc::c_int = 0 as libc::c_int;
    let mut red_to_blue_best: libc::c_int = 0 as libc::c_int;
    let mut iter: libc::c_int = 0;
    let mut best_diff: libc::c_float = GetPredictionCostCrossColorBlue(
        argb,
        stride,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        green_to_blue_best,
        red_to_blue_best,
        accumulated_blue_histo,
    );
    iter = 0 as libc::c_int;
    while iter < iters {
        let delta: libc::c_int = delta_lut[iter as usize] as libc::c_int;
        let mut axis: libc::c_int = 0;
        axis = 0 as libc::c_int;
        while axis < 8 as libc::c_int {
            let green_to_blue_cur: libc::c_int = offset[axis
                as usize][0 as libc::c_int as usize] as libc::c_int * delta
                + green_to_blue_best;
            let red_to_blue_cur: libc::c_int = offset[axis
                as usize][1 as libc::c_int as usize] as libc::c_int * delta
                + red_to_blue_best;
            let cur_diff: libc::c_float = GetPredictionCostCrossColorBlue(
                argb,
                stride,
                tile_width,
                tile_height,
                prev_x,
                prev_y,
                green_to_blue_cur,
                red_to_blue_cur,
                accumulated_blue_histo,
            );
            if cur_diff < best_diff {
                best_diff = cur_diff;
                green_to_blue_best = green_to_blue_cur;
                red_to_blue_best = red_to_blue_cur;
            }
            if quality < 25 as libc::c_int && iter == 4 as libc::c_int {
                break;
            } else {
                axis += 1;
                axis;
            }
        }
        if delta == 2 as libc::c_int && green_to_blue_best == 0 as libc::c_int
            && red_to_blue_best == 0 as libc::c_int
        {
            break;
        } else {
            iter += 1;
            iter;
        }
    }
    (*best_tx).green_to_blue_ = (green_to_blue_best & 0xff as libc::c_int) as uint8_t;
    (*best_tx).red_to_blue_ = (red_to_blue_best & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn GetBestColorTransformForTile(
    mut tile_x: libc::c_int,
    mut tile_y: libc::c_int,
    mut bits: libc::c_int,
    mut prev_x: VP8LMultipliers,
    mut prev_y: VP8LMultipliers,
    mut quality: libc::c_int,
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    mut accumulated_red_histo: *const libc::c_int,
    mut accumulated_blue_histo: *const libc::c_int,
    argb: *const uint32_t,
) -> VP8LMultipliers {
    let max_tile_size: libc::c_int = (1 as libc::c_int) << bits;
    let tile_y_offset: libc::c_int = tile_y * max_tile_size;
    let tile_x_offset: libc::c_int = tile_x * max_tile_size;
    let all_x_max: libc::c_int = GetMin(tile_x_offset + max_tile_size, xsize);
    let all_y_max: libc::c_int = GetMin(tile_y_offset + max_tile_size, ysize);
    let tile_width: libc::c_int = all_x_max - tile_x_offset;
    let tile_height: libc::c_int = all_y_max - tile_y_offset;
    let tile_argb: *const uint32_t = argb
        .offset((tile_y_offset * xsize) as isize)
        .offset(tile_x_offset as isize);
    let mut best_tx: VP8LMultipliers = VP8LMultipliers {
        green_to_red_: 0,
        green_to_blue_: 0,
        red_to_blue_: 0,
    };
    MultipliersClear(&mut best_tx);
    GetBestGreenToRed(
        tile_argb,
        xsize,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        quality,
        accumulated_red_histo,
        &mut best_tx,
    );
    GetBestGreenRedToBlue(
        tile_argb,
        xsize,
        tile_width,
        tile_height,
        prev_x,
        prev_y,
        quality,
        accumulated_blue_histo,
        &mut best_tx,
    );
    return best_tx;
}
unsafe extern "C" fn CopyTileWithColorTransform(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    mut tile_x: libc::c_int,
    mut tile_y: libc::c_int,
    mut max_tile_size: libc::c_int,
    mut color_transform: VP8LMultipliers,
    mut argb: *mut uint32_t,
) {
    let xscan: libc::c_int = GetMin(max_tile_size, xsize - tile_x);
    let mut yscan: libc::c_int = GetMin(max_tile_size, ysize - tile_y);
    argb = argb.offset((tile_y * xsize + tile_x) as isize);
    loop {
        let fresh4 = yscan;
        yscan = yscan - 1;
        if !(fresh4 > 0 as libc::c_int) {
            break;
        }
        VP8LTransformColor
            .expect("non-null function pointer")(&mut color_transform, argb, xscan);
        argb = argb.offset(xsize as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorSpaceTransform(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut bits: libc::c_int,
    mut quality: libc::c_int,
    argb: *mut uint32_t,
    mut image: *mut uint32_t,
    pic: *const WebPPicture,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let max_tile_size: libc::c_int = (1 as libc::c_int) << bits;
    let tile_xsize: libc::c_int = VP8LSubSampleSize(width as uint32_t, bits as uint32_t)
        as libc::c_int;
    let tile_ysize: libc::c_int = VP8LSubSampleSize(height as uint32_t, bits as uint32_t)
        as libc::c_int;
    let mut percent_start: libc::c_int = *percent;
    let mut accumulated_red_histo: [libc::c_int; 256] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut accumulated_blue_histo: [libc::c_int; 256] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut tile_x: libc::c_int = 0;
    let mut tile_y: libc::c_int = 0;
    let mut prev_x: VP8LMultipliers = VP8LMultipliers {
        green_to_red_: 0,
        green_to_blue_: 0,
        red_to_blue_: 0,
    };
    let mut prev_y: VP8LMultipliers = VP8LMultipliers {
        green_to_red_: 0,
        green_to_blue_: 0,
        red_to_blue_: 0,
    };
    MultipliersClear(&mut prev_y);
    MultipliersClear(&mut prev_x);
    tile_y = 0 as libc::c_int;
    while tile_y < tile_ysize {
        tile_x = 0 as libc::c_int;
        while tile_x < tile_xsize {
            let mut y: libc::c_int = 0;
            let tile_x_offset: libc::c_int = tile_x * max_tile_size;
            let tile_y_offset: libc::c_int = tile_y * max_tile_size;
            let all_x_max: libc::c_int = GetMin(tile_x_offset + max_tile_size, width);
            let all_y_max: libc::c_int = GetMin(tile_y_offset + max_tile_size, height);
            let offset: libc::c_int = tile_y * tile_xsize + tile_x;
            if tile_y != 0 as libc::c_int {
                ColorCodeToMultipliers(
                    *image.offset((offset - tile_xsize) as isize),
                    &mut prev_y,
                );
            }
            prev_x = GetBestColorTransformForTile(
                tile_x,
                tile_y,
                bits,
                prev_x,
                prev_y,
                quality,
                width,
                height,
                accumulated_red_histo.as_mut_ptr() as *const libc::c_int,
                accumulated_blue_histo.as_mut_ptr() as *const libc::c_int,
                argb,
            );
            *image.offset(offset as isize) = MultipliersToColorCode(&mut prev_x);
            CopyTileWithColorTransform(
                width,
                height,
                tile_x_offset,
                tile_y_offset,
                max_tile_size,
                prev_x,
                argb,
            );
            y = tile_y_offset;
            while y < all_y_max {
                let mut ix: libc::c_int = y * width + tile_x_offset;
                let ix_end: libc::c_int = ix + all_x_max - tile_x_offset;
                while ix < ix_end {
                    let pix: uint32_t = *argb.offset(ix as isize);
                    if !(ix >= 2 as libc::c_int
                        && pix == *argb.offset((ix - 2 as libc::c_int) as isize)
                        && pix == *argb.offset((ix - 1 as libc::c_int) as isize))
                    {
                        if !(ix >= width + 2 as libc::c_int
                            && *argb.offset((ix - 2 as libc::c_int) as isize)
                                == *argb.offset((ix - width - 2 as libc::c_int) as isize)
                            && *argb.offset((ix - 1 as libc::c_int) as isize)
                                == *argb.offset((ix - width - 1 as libc::c_int) as isize)
                            && pix == *argb.offset((ix - width) as isize))
                        {
                            accumulated_red_histo[(pix >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_uint) as usize] += 1;
                            accumulated_red_histo[(pix >> 16 as libc::c_int
                                & 0xff as libc::c_int as libc::c_uint) as usize];
                            accumulated_blue_histo[(pix >> 0 as libc::c_int
                                & 0xff as libc::c_int as libc::c_uint) as usize] += 1;
                            accumulated_blue_histo[(pix >> 0 as libc::c_int
                                & 0xff as libc::c_int as libc::c_uint) as usize];
                        }
                    }
                    ix += 1;
                    ix;
                }
                y += 1;
                y;
            }
            tile_x += 1;
            tile_x;
        }
        if WebPReportProgress(
            pic,
            percent_start + percent_range * tile_y / tile_ysize,
            percent,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        tile_y += 1;
        tile_y;
    }
    return 1 as libc::c_int;
}
