use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut WebPAlphaReplace: Option::<
        unsafe extern "C" fn(*mut uint32_t, libc::c_int, uint32_t) -> (),
    >;
    fn WebPInitAlphaProcessing();
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
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
pub const YUV_HALF: C2RustUnnamed = 32768;
pub const YUV_FIX: C2RustUnnamed = 16;
pub type C2RustUnnamed = libc::c_uint;
pub const YUV_MASK2: C2RustUnnamed = 16383;
pub const YUV_FIX2: C2RustUnnamed = 6;
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
unsafe extern "C" fn IsTransparentARGBArea(
    mut ptr: *const uint32_t,
    mut stride: libc::c_int,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < size {
        x = 0 as libc::c_int;
        while x < size {
            if *ptr.offset(x as isize) & 0xff000000 as libc::c_uint != 0 {
                return 0 as libc::c_int;
            }
            x += 1;
            x;
        }
        ptr = ptr.offset(stride as isize);
        y += 1;
        y;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Flatten(
    mut ptr: *mut uint8_t,
    mut v: libc::c_int,
    mut stride: libc::c_int,
    mut size: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < size {
        memset(ptr as *mut libc::c_void, v, size as libc::c_ulong);
        ptr = ptr.offset(stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn FlattenARGB(
    mut ptr: *mut uint32_t,
    mut v: uint32_t,
    mut stride: libc::c_int,
    mut size: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < size {
        x = 0 as libc::c_int;
        while x < size {
            *ptr.offset(x as isize) = v;
            x += 1;
            x;
        }
        ptr = ptr.offset(stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn SmoothenBlock(
    mut a_ptr: *const uint8_t,
    mut a_stride: libc::c_int,
    mut y_ptr: *mut uint8_t,
    mut y_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut alpha_ptr: *const uint8_t = a_ptr;
    let mut luma_ptr: *mut uint8_t = y_ptr;
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            if *alpha_ptr.offset(x as isize) as libc::c_int != 0 as libc::c_int {
                count += 1;
                count;
                sum += *luma_ptr.offset(x as isize) as libc::c_int;
            }
            x += 1;
            x;
        }
        alpha_ptr = alpha_ptr.offset(a_stride as isize);
        luma_ptr = luma_ptr.offset(y_stride as isize);
        y += 1;
        y;
    }
    if count > 0 as libc::c_int && count < width * height {
        let avg_u8: uint8_t = (sum / count) as uint8_t;
        alpha_ptr = a_ptr;
        luma_ptr = y_ptr;
        y = 0 as libc::c_int;
        while y < height {
            x = 0 as libc::c_int;
            while x < width {
                if *alpha_ptr.offset(x as isize) as libc::c_int == 0 as libc::c_int {
                    *luma_ptr.offset(x as isize) = avg_u8;
                }
                x += 1;
                x;
            }
            alpha_ptr = alpha_ptr.offset(a_stride as isize);
            luma_ptr = luma_ptr.offset(y_stride as isize);
            y += 1;
            y;
        }
    }
    return (count == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPReplaceTransparentPixels(
    pic: *mut WebPPicture,
    mut color: uint32_t,
) {
    if !pic.is_null() && (*pic).use_argb != 0 {
        let mut y: libc::c_int = (*pic).height;
        let mut argb: *mut uint32_t = (*pic).argb;
        color &= 0xffffff as libc::c_uint;
        WebPInitAlphaProcessing();
        loop {
            let fresh0 = y;
            y = y - 1;
            if !(fresh0 > 0 as libc::c_int) {
                break;
            }
            WebPAlphaReplace
                .expect("non-null function pointer")(argb, (*pic).width, color);
            argb = argb.offset((*pic).argb_stride as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPCleanupTransparentArea(mut pic: *mut WebPPicture) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    if pic.is_null() {
        return;
    }
    w = (*pic).width / 8 as libc::c_int;
    h = (*pic).height / 8 as libc::c_int;
    if (*pic).use_argb != 0 {
        let mut argb_value: uint32_t = 0 as libc::c_int as uint32_t;
        y = 0 as libc::c_int;
        while y < h {
            let mut need_reset: libc::c_int = 1 as libc::c_int;
            x = 0 as libc::c_int;
            while x < w {
                let off: libc::c_int = (y * (*pic).argb_stride + x) * 8 as libc::c_int;
                if IsTransparentARGBArea(
                    ((*pic).argb).offset(off as isize),
                    (*pic).argb_stride,
                    8 as libc::c_int,
                ) != 0
                {
                    if need_reset != 0 {
                        argb_value = *((*pic).argb).offset(off as isize);
                        need_reset = 0 as libc::c_int;
                    }
                    FlattenARGB(
                        ((*pic).argb).offset(off as isize),
                        argb_value,
                        (*pic).argb_stride,
                        8 as libc::c_int,
                    );
                } else {
                    need_reset = 1 as libc::c_int;
                }
                x += 1;
                x;
            }
            y += 1;
            y;
        }
    } else {
        let width: libc::c_int = (*pic).width;
        let height: libc::c_int = (*pic).height;
        let y_stride: libc::c_int = (*pic).y_stride;
        let uv_stride: libc::c_int = (*pic).uv_stride;
        let a_stride: libc::c_int = (*pic).a_stride;
        let mut y_ptr: *mut uint8_t = (*pic).y;
        let mut u_ptr: *mut uint8_t = (*pic).u;
        let mut v_ptr: *mut uint8_t = (*pic).v;
        let mut a_ptr: *const uint8_t = (*pic).a;
        let mut values: [libc::c_int; 3] = [0 as libc::c_int, 0, 0];
        if a_ptr.is_null() || y_ptr.is_null() || u_ptr.is_null() || v_ptr.is_null() {
            return;
        }
        y = 0 as libc::c_int;
        while y + 8 as libc::c_int <= height {
            let mut need_reset_0: libc::c_int = 1 as libc::c_int;
            x = 0 as libc::c_int;
            while x + 8 as libc::c_int <= width {
                if SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    8 as libc::c_int,
                    8 as libc::c_int,
                ) != 0
                {
                    if need_reset_0 != 0 {
                        values[0 as libc::c_int
                            as usize] = *y_ptr.offset(x as isize) as libc::c_int;
                        values[1 as libc::c_int
                            as usize] = *u_ptr.offset((x >> 1 as libc::c_int) as isize)
                            as libc::c_int;
                        values[2 as libc::c_int
                            as usize] = *v_ptr.offset((x >> 1 as libc::c_int) as isize)
                            as libc::c_int;
                        need_reset_0 = 0 as libc::c_int;
                    }
                    Flatten(
                        y_ptr.offset(x as isize),
                        values[0 as libc::c_int as usize],
                        y_stride,
                        8 as libc::c_int,
                    );
                    Flatten(
                        u_ptr.offset((x >> 1 as libc::c_int) as isize),
                        values[1 as libc::c_int as usize],
                        uv_stride,
                        8 as libc::c_int / 2 as libc::c_int,
                    );
                    Flatten(
                        v_ptr.offset((x >> 1 as libc::c_int) as isize),
                        values[2 as libc::c_int as usize],
                        uv_stride,
                        8 as libc::c_int / 2 as libc::c_int,
                    );
                } else {
                    need_reset_0 = 1 as libc::c_int;
                }
                x += 8 as libc::c_int;
            }
            if x < width {
                SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    width - x,
                    8 as libc::c_int,
                );
            }
            a_ptr = a_ptr.offset((8 as libc::c_int * a_stride) as isize);
            y_ptr = y_ptr.offset((8 as libc::c_int * y_stride) as isize);
            u_ptr = u_ptr
                .offset((8 as libc::c_int / 2 as libc::c_int * uv_stride) as isize);
            v_ptr = v_ptr
                .offset((8 as libc::c_int / 2 as libc::c_int * uv_stride) as isize);
            y += 8 as libc::c_int;
        }
        if y < height {
            let sub_height: libc::c_int = height - y;
            x = 0 as libc::c_int;
            while x + 8 as libc::c_int <= width {
                SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    8 as libc::c_int,
                    sub_height,
                );
                x += 8 as libc::c_int;
            }
            if x < width {
                SmoothenBlock(
                    a_ptr.offset(x as isize),
                    a_stride,
                    y_ptr.offset(x as isize),
                    y_stride,
                    width - x,
                    sub_height,
                );
            }
        }
    };
}
#[inline]
unsafe extern "C" fn MakeARGB32(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> uint32_t {
    return 0xff000000 as libc::c_uint | (r << 16 as libc::c_int) as libc::c_uint
        | (g << 8 as libc::c_int) as libc::c_uint | b as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn WebPBlendAlpha(
    mut picture: *mut WebPPicture,
    mut background_rgb: uint32_t,
) {
    let red: libc::c_int = (background_rgb >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let green: libc::c_int = (background_rgb >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let blue: libc::c_int = (background_rgb >> 0 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if picture.is_null() {
        return;
    }
    if (*picture).use_argb == 0 {
        let uv_width: libc::c_int = (*picture).width >> 1 as libc::c_int;
        let Y0: libc::c_int = VP8RGBToY(red, green, blue, YUV_HALF as libc::c_int);
        let U0: libc::c_int = VP8RGBToU(
            4 as libc::c_int * red,
            4 as libc::c_int * green,
            4 as libc::c_int * blue,
            4 as libc::c_int * YUV_HALF as libc::c_int,
        );
        let V0: libc::c_int = VP8RGBToV(
            4 as libc::c_int * red,
            4 as libc::c_int * green,
            4 as libc::c_int * blue,
            4 as libc::c_int * YUV_HALF as libc::c_int,
        );
        let has_alpha: libc::c_int = ((*picture).colorspace as libc::c_uint
            & WEBP_CSP_ALPHA_BIT as libc::c_int as libc::c_uint) as libc::c_int;
        let mut y_ptr: *mut uint8_t = (*picture).y;
        let mut u_ptr: *mut uint8_t = (*picture).u;
        let mut v_ptr: *mut uint8_t = (*picture).v;
        let mut a_ptr: *mut uint8_t = (*picture).a;
        if has_alpha == 0 || a_ptr.is_null() {
            return;
        }
        y = 0 as libc::c_int;
        while y < (*picture).height {
            x = 0 as libc::c_int;
            while x < (*picture).width {
                let alpha: uint8_t = *a_ptr.offset(x as isize);
                if (alpha as libc::c_int) < 0xff as libc::c_int {
                    *y_ptr
                        .offset(
                            x as isize,
                        ) = ((Y0 * (255 as libc::c_int - alpha as libc::c_int)
                        + *y_ptr.offset(x as isize) as libc::c_int
                            * alpha as libc::c_int) * 0x101 as libc::c_int
                        + 256 as libc::c_int >> 16 as libc::c_int) as uint8_t;
                }
                x += 1;
                x;
            }
            if y & 1 as libc::c_int == 0 as libc::c_int {
                let a_ptr2: *mut uint8_t = if y + 1 as libc::c_int == (*picture).height {
                    a_ptr
                } else {
                    a_ptr.offset((*picture).a_stride as isize)
                };
                x = 0 as libc::c_int;
                while x < uv_width {
                    let alpha_0: uint32_t = (*a_ptr
                        .offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                        as libc::c_int
                        + *a_ptr
                            .offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                            as libc::c_int
                        + *a_ptr2
                            .offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                            as libc::c_int
                        + *a_ptr2
                            .offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                            as libc::c_int) as uint32_t;
                    *u_ptr
                        .offset(
                            x as isize,
                        ) = ((U0 as libc::c_uint)
                        .wrapping_mul(
                            (1020 as libc::c_int as libc::c_uint).wrapping_sub(alpha_0),
                        )
                        .wrapping_add(
                            (*u_ptr.offset(x as isize) as libc::c_uint)
                                .wrapping_mul(alpha_0),
                        )
                        .wrapping_mul(0x101 as libc::c_int as libc::c_uint)
                        .wrapping_add(1024 as libc::c_int as libc::c_uint)
                        >> 18 as libc::c_int) as uint8_t;
                    *v_ptr
                        .offset(
                            x as isize,
                        ) = ((V0 as libc::c_uint)
                        .wrapping_mul(
                            (1020 as libc::c_int as libc::c_uint).wrapping_sub(alpha_0),
                        )
                        .wrapping_add(
                            (*v_ptr.offset(x as isize) as libc::c_uint)
                                .wrapping_mul(alpha_0),
                        )
                        .wrapping_mul(0x101 as libc::c_int as libc::c_uint)
                        .wrapping_add(1024 as libc::c_int as libc::c_uint)
                        >> 18 as libc::c_int) as uint8_t;
                    x += 1;
                    x;
                }
                if (*picture).width & 1 as libc::c_int != 0 {
                    let alpha_1: uint32_t = (2 as libc::c_int
                        * (*a_ptr
                            .offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                            as libc::c_int
                            + *a_ptr2
                                .offset((2 as libc::c_int * x + 0 as libc::c_int) as isize)
                                as libc::c_int)) as uint32_t;
                    *u_ptr
                        .offset(
                            x as isize,
                        ) = ((U0 as libc::c_uint)
                        .wrapping_mul(
                            (1020 as libc::c_int as libc::c_uint).wrapping_sub(alpha_1),
                        )
                        .wrapping_add(
                            (*u_ptr.offset(x as isize) as libc::c_uint)
                                .wrapping_mul(alpha_1),
                        )
                        .wrapping_mul(0x101 as libc::c_int as libc::c_uint)
                        .wrapping_add(1024 as libc::c_int as libc::c_uint)
                        >> 18 as libc::c_int) as uint8_t;
                    *v_ptr
                        .offset(
                            x as isize,
                        ) = ((V0 as libc::c_uint)
                        .wrapping_mul(
                            (1020 as libc::c_int as libc::c_uint).wrapping_sub(alpha_1),
                        )
                        .wrapping_add(
                            (*v_ptr.offset(x as isize) as libc::c_uint)
                                .wrapping_mul(alpha_1),
                        )
                        .wrapping_mul(0x101 as libc::c_int as libc::c_uint)
                        .wrapping_add(1024 as libc::c_int as libc::c_uint)
                        >> 18 as libc::c_int) as uint8_t;
                }
            } else {
                u_ptr = u_ptr.offset((*picture).uv_stride as isize);
                v_ptr = v_ptr.offset((*picture).uv_stride as isize);
            }
            memset(
                a_ptr as *mut libc::c_void,
                0xff as libc::c_int,
                (*picture).width as libc::c_ulong,
            );
            a_ptr = a_ptr.offset((*picture).a_stride as isize);
            y_ptr = y_ptr.offset((*picture).y_stride as isize);
            y += 1;
            y;
        }
    } else {
        let mut argb: *mut uint32_t = (*picture).argb;
        let background: uint32_t = MakeARGB32(red, green, blue);
        y = 0 as libc::c_int;
        while y < (*picture).height {
            x = 0 as libc::c_int;
            while x < (*picture).width {
                let alpha_2: libc::c_int = (*argb.offset(x as isize) >> 24 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
                if alpha_2 != 0xff as libc::c_int {
                    if alpha_2 > 0 as libc::c_int {
                        let mut r: libc::c_int = (*argb.offset(x as isize)
                            >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                            as libc::c_int;
                        let mut g: libc::c_int = (*argb.offset(x as isize)
                            >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                            as libc::c_int;
                        let mut b: libc::c_int = (*argb.offset(x as isize)
                            >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                            as libc::c_int;
                        r = (red * (255 as libc::c_int - alpha_2) + r * alpha_2)
                            * 0x101 as libc::c_int + 256 as libc::c_int
                            >> 16 as libc::c_int;
                        g = (green * (255 as libc::c_int - alpha_2) + g * alpha_2)
                            * 0x101 as libc::c_int + 256 as libc::c_int
                            >> 16 as libc::c_int;
                        b = (blue * (255 as libc::c_int - alpha_2) + b * alpha_2)
                            * 0x101 as libc::c_int + 256 as libc::c_int
                            >> 16 as libc::c_int;
                        *argb.offset(x as isize) = MakeARGB32(r, g, b);
                    } else {
                        *argb.offset(x as isize) = background;
                    }
                }
                x += 1;
                x;
            }
            argb = argb.offset((*picture).argb_stride as isize);
            y += 1;
            y;
        }
    };
}
