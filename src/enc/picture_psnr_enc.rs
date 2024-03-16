use ::libc;
extern "C" {
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: libc::c_int) -> libc::c_int;
    fn WebPPictureFree(picture: *mut WebPPicture);
    fn WebPPictureView(
        src: *const WebPPicture,
        left: libc::c_int,
        top: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        dst: *mut WebPPicture,
    ) -> libc::c_int;
    fn WebPPictureYUVAToARGB(picture: *mut WebPPicture) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    static mut VP8SSIMGetClipped: VP8SSIMGetClippedFunc;
    static mut VP8SSIMGet: VP8SSIMGetFunc;
    static mut VP8AccumulateSSE: VP8AccumulateSSEFunc;
    fn VP8SSIMDspInit();
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type AccumulateFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_double,
>;
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
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> libc::c_int {
    return WebPPictureInitInternal(picture, 0x20f as libc::c_int);
}
unsafe extern "C" fn AccumulateLSIM(
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_double {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut total_sse: libc::c_double = 0.0f64;
    y = 0 as libc::c_int;
    while y < h {
        let y_0: libc::c_int = if (y - 2 as libc::c_int) < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            y - 2 as libc::c_int
        };
        let y_1: libc::c_int = if y + 2 as libc::c_int + 1 as libc::c_int >= h {
            h
        } else {
            y + 2 as libc::c_int + 1 as libc::c_int
        };
        x = 0 as libc::c_int;
        while x < w {
            let x_0: libc::c_int = if (x - 2 as libc::c_int) < 0 as libc::c_int {
                0 as libc::c_int
            } else {
                x - 2 as libc::c_int
            };
            let x_1: libc::c_int = if x + 2 as libc::c_int + 1 as libc::c_int >= w {
                w
            } else {
                x + 2 as libc::c_int + 1 as libc::c_int
            };
            let mut best_sse: libc::c_double = 255.0f64 * 255.0f64;
            let value: libc::c_double = *ref_0.offset((y * ref_stride + x) as isize)
                as libc::c_double;
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            j = y_0;
            while j < y_1 {
                let s: *const uint8_t = src.offset((j * src_stride) as isize);
                i = x_0;
                while i < x_1 {
                    let diff: libc::c_double = *s.offset(i as isize) as libc::c_int
                        as libc::c_double - value;
                    let sse: libc::c_double = diff * diff;
                    if sse < best_sse {
                        best_sse = sse;
                    }
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
            total_sse += best_sse;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return total_sse;
}
unsafe extern "C" fn AccumulateSSE(
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_double {
    let mut y: libc::c_int = 0;
    let mut total_sse: libc::c_double = 0.0f64;
    y = 0 as libc::c_int;
    while y < h {
        total_sse
            += VP8AccumulateSSE.expect("non-null function pointer")(src, ref_0, w)
                as libc::c_double;
        src = src.offset(src_stride as isize);
        ref_0 = ref_0.offset(ref_stride as isize);
        y += 1;
        y;
    }
    return total_sse;
}
unsafe extern "C" fn AccumulateSSIM(
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_double {
    let w0: libc::c_int = if w < 3 as libc::c_int { w } else { 3 as libc::c_int };
    let w1: libc::c_int = w - 3 as libc::c_int - 1 as libc::c_int;
    let h0: libc::c_int = if h < 3 as libc::c_int { h } else { 3 as libc::c_int };
    let h1: libc::c_int = h - 3 as libc::c_int - 1 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    y = 0 as libc::c_int;
    while y < h0 {
        x = 0 as libc::c_int;
        while x < w {
            sum
                += VP8SSIMGetClipped
                    .expect(
                        "non-null function pointer",
                    )(src, src_stride, ref_0, ref_stride, x, y, w, h);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    while y < h1 {
        x = 0 as libc::c_int;
        while x < w0 {
            sum
                += VP8SSIMGetClipped
                    .expect(
                        "non-null function pointer",
                    )(src, src_stride, ref_0, ref_stride, x, y, w, h);
            x += 1;
            x;
        }
        while x < w1 {
            let off1: libc::c_int = x - 3 as libc::c_int
                + (y - 3 as libc::c_int) * src_stride;
            let off2: libc::c_int = x - 3 as libc::c_int
                + (y - 3 as libc::c_int) * ref_stride;
            sum
                += VP8SSIMGet
                    .expect(
                        "non-null function pointer",
                    )(
                    src.offset(off1 as isize),
                    src_stride,
                    ref_0.offset(off2 as isize),
                    ref_stride,
                );
            x += 1;
            x;
        }
        while x < w {
            sum
                += VP8SSIMGetClipped
                    .expect(
                        "non-null function pointer",
                    )(src, src_stride, ref_0, ref_stride, x, y, w, h);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    while y < h {
        x = 0 as libc::c_int;
        while x < w {
            sum
                += VP8SSIMGetClipped
                    .expect(
                        "non-null function pointer",
                    )(src, src_stride, ref_0, ref_stride, x, y, w, h);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return sum;
}
static mut kMinDistortion_dB: libc::c_double = 99.0f64;
unsafe extern "C" fn GetPSNR(
    mut v: libc::c_double,
    mut size: libc::c_double,
) -> libc::c_double {
    return if v > 0.0f64 && size > 0.0f64 {
        -4.3429448f64 * log(v / (size * 255 as libc::c_int as libc::c_double * 255.0f64))
    } else {
        kMinDistortion_dB
    };
}
unsafe extern "C" fn GetLogSSIM(
    mut v: libc::c_double,
    mut size: libc::c_double,
) -> libc::c_double {
    v = if size > 0.0f64 { v / size } else { 1.0f64 };
    return if v < 1.0f64 { -10.0f64 * log10(1.0f64 - v) } else { kMinDistortion_dB };
}
#[no_mangle]
pub unsafe extern "C" fn WebPPlaneDistortion(
    mut src: *const uint8_t,
    mut src_stride: size_t,
    mut ref_0: *const uint8_t,
    mut ref_stride: size_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut x_step: size_t,
    mut type_0: libc::c_int,
    mut distortion: *mut libc::c_float,
    mut result: *mut libc::c_float,
) -> libc::c_int {
    let mut allocated: *mut uint8_t = 0 as *mut uint8_t;
    let metric: AccumulateFunc = if type_0 == 0 as libc::c_int {
        Some(
            AccumulateSSE
                as unsafe extern "C" fn(
                    *const uint8_t,
                    libc::c_int,
                    *const uint8_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_double,
        )
    } else if type_0 == 1 as libc::c_int {
        Some(
            AccumulateSSIM
                as unsafe extern "C" fn(
                    *const uint8_t,
                    libc::c_int,
                    *const uint8_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_double,
        )
    } else {
        Some(
            AccumulateLSIM
                as unsafe extern "C" fn(
                    *const uint8_t,
                    libc::c_int,
                    *const uint8_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_double,
        )
    };
    if src.is_null() || ref_0.is_null()
        || src_stride < x_step.wrapping_mul(width as libc::c_ulong)
        || ref_stride < x_step.wrapping_mul(width as libc::c_ulong) || result.is_null()
        || distortion.is_null()
    {
        return 0 as libc::c_int;
    }
    VP8SSIMDspInit();
    if x_step != 1 as libc::c_int as libc::c_ulong {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut tmp1: *mut uint8_t = 0 as *mut uint8_t;
        let mut tmp2: *mut uint8_t = 0 as *mut uint8_t;
        allocated = WebPSafeMalloc(
            (2 as libc::c_ulonglong)
                .wrapping_mul(width as libc::c_ulonglong)
                .wrapping_mul(height as libc::c_ulonglong) as uint64_t,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        ) as *mut uint8_t;
        if allocated.is_null() {
            return 0 as libc::c_int;
        }
        tmp1 = allocated;
        tmp2 = tmp1
            .offset((width as size_t).wrapping_mul(height as libc::c_ulong) as isize);
        y = 0 as libc::c_int;
        while y < height {
            x = 0 as libc::c_int;
            while x < width {
                *tmp1
                    .offset(
                        (x + y * width) as isize,
                    ) = *src
                    .offset(
                        (x as libc::c_ulong)
                            .wrapping_mul(x_step)
                            .wrapping_add((y as libc::c_ulong).wrapping_mul(src_stride))
                            as isize,
                    );
                *tmp2
                    .offset(
                        (x + y * width) as isize,
                    ) = *ref_0
                    .offset(
                        (x as libc::c_ulong)
                            .wrapping_mul(x_step)
                            .wrapping_add((y as libc::c_ulong).wrapping_mul(ref_stride))
                            as isize,
                    );
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        src = tmp1;
        ref_0 = tmp2;
    }
    *distortion = metric
        .expect("non-null function pointer")(src, width, ref_0, width, width, height)
        as libc::c_float;
    WebPSafeFree(allocated as *mut libc::c_void);
    *result = if type_0 == 1 as libc::c_int {
        GetLogSSIM(
            *distortion as libc::c_double,
            width as libc::c_double * height as libc::c_double,
        ) as libc::c_float
    } else {
        GetPSNR(
            *distortion as libc::c_double,
            width as libc::c_double * height as libc::c_double,
        ) as libc::c_float
    };
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureDistortion(
    mut src: *const WebPPicture,
    mut ref_0: *const WebPPicture,
    mut type_0: libc::c_int,
    mut results: *mut libc::c_float,
) -> libc::c_int {
    let mut current_block: u64;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut p0: WebPPicture = WebPPicture {
        use_argb: 0,
        colorspace: WEBP_YUV420,
        width: 0,
        height: 0,
        y: 0 as *mut uint8_t,
        u: 0 as *mut uint8_t,
        v: 0 as *mut uint8_t,
        y_stride: 0,
        uv_stride: 0,
        a: 0 as *mut uint8_t,
        a_stride: 0,
        pad1: [0; 2],
        argb: 0 as *mut uint32_t,
        argb_stride: 0,
        pad2: [0; 3],
        writer: None,
        custom_ptr: 0 as *mut libc::c_void,
        extra_info_type: 0,
        extra_info: 0 as *mut uint8_t,
        stats: 0 as *mut WebPAuxStats,
        error_code: VP8_ENC_OK,
        progress_hook: None,
        user_data: 0 as *mut libc::c_void,
        pad3: [0; 3],
        pad4: 0 as *mut uint8_t,
        pad5: 0 as *mut uint8_t,
        pad6: [0; 8],
        memory_: 0 as *mut libc::c_void,
        memory_argb_: 0 as *mut libc::c_void,
        pad7: [0 as *mut libc::c_void; 2],
    };
    let mut p1: WebPPicture = WebPPicture {
        use_argb: 0,
        colorspace: WEBP_YUV420,
        width: 0,
        height: 0,
        y: 0 as *mut uint8_t,
        u: 0 as *mut uint8_t,
        v: 0 as *mut uint8_t,
        y_stride: 0,
        uv_stride: 0,
        a: 0 as *mut uint8_t,
        a_stride: 0,
        pad1: [0; 2],
        argb: 0 as *mut uint32_t,
        argb_stride: 0,
        pad2: [0; 3],
        writer: None,
        custom_ptr: 0 as *mut libc::c_void,
        extra_info_type: 0,
        extra_info: 0 as *mut uint8_t,
        stats: 0 as *mut WebPAuxStats,
        error_code: VP8_ENC_OK,
        progress_hook: None,
        user_data: 0 as *mut libc::c_void,
        pad3: [0; 3],
        pad4: 0 as *mut uint8_t,
        pad5: 0 as *mut uint8_t,
        pad6: [0; 8],
        memory_: 0 as *mut libc::c_void,
        memory_argb_: 0 as *mut libc::c_void,
        pad7: [0 as *mut libc::c_void; 2],
    };
    let mut total_size: libc::c_double = 0.0f64;
    let mut total_distortion: libc::c_double = 0.0f64;
    if src.is_null() || ref_0.is_null() || (*src).width != (*ref_0).width
        || (*src).height != (*ref_0).height || results.is_null()
    {
        return 0 as libc::c_int;
    }
    VP8SSIMDspInit();
    if WebPPictureInit(&mut p0) == 0 || WebPPictureInit(&mut p1) == 0 {
        return 0 as libc::c_int;
    }
    w = (*src).width;
    h = (*src).height;
    if !(WebPPictureView(src, 0 as libc::c_int, 0 as libc::c_int, w, h, &mut p0) == 0) {
        if !(WebPPictureView(ref_0, 0 as libc::c_int, 0 as libc::c_int, w, h, &mut p1)
            == 0)
        {
            if !(p0.use_argb == 0 as libc::c_int && WebPPictureYUVAToARGB(&mut p0) == 0)
            {
                if !(p1.use_argb == 0 as libc::c_int
                    && WebPPictureYUVAToARGB(&mut p1) == 0)
                {
                    c = 0 as libc::c_int;
                    loop {
                        if !(c < 4 as libc::c_int) {
                            current_block = 15652330335145281839;
                            break;
                        }
                        let mut distortion: libc::c_float = 0.;
                        let stride0: size_t = (4 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(p0.argb_stride as size_t);
                        let stride1: size_t = (4 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(p1.argb_stride as size_t);
                        let offset: libc::c_int = c ^ 0 as libc::c_int;
                        if WebPPlaneDistortion(
                            (p0.argb as *const uint8_t).offset(offset as isize),
                            stride0,
                            (p1.argb as *const uint8_t).offset(offset as isize),
                            stride1,
                            w,
                            h,
                            4 as libc::c_int as size_t,
                            type_0,
                            &mut distortion,
                            results.offset(c as isize),
                        ) == 0
                        {
                            current_block = 6459748727782714526;
                            break;
                        }
                        total_distortion += distortion as libc::c_double;
                        total_size += (w * h) as libc::c_double;
                        c += 1;
                        c;
                    }
                    match current_block {
                        6459748727782714526 => {}
                        _ => {
                            *results
                                .offset(
                                    4 as libc::c_int as isize,
                                ) = if type_0 == 1 as libc::c_int {
                                GetLogSSIM(total_distortion, total_size) as libc::c_float
                            } else {
                                GetPSNR(total_distortion, total_size) as libc::c_float
                            };
                            ok = 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    WebPPictureFree(&mut p0);
    WebPPictureFree(&mut p1);
    return ok;
}
