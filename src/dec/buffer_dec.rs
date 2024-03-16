use ::libc;

use crate::src::utils::types::{MODE_Argb, MODE_bgrA, MODE_rgbA, MODE_rgbA_4444, WebPBitstreamFeatures, WebPDecBuffer, WebPRGBABuffer, WebPYUVABuffer, MODE_ARGB, MODE_BGRA, MODE_LAST, MODE_RGB, MODE_RGBA, MODE_RGBA_4444, MODE_YUV, MODE_YUVA, WEBP_CSP_MODE};
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn WebPRescalerGetScaledDimensions(
        src_width: libc::c_int,
        src_height: libc::c_int,
        scaled_width: *mut libc::c_int,
        scaled_height: *mut libc::c_int,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPCheckCropDimensions(
        image_width: libc::c_int,
        image_height: libc::c_int,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> libc::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPCopyPlane(
        src: *const uint8_t,
        src_stride: libc::c_int,
        dst: *mut uint8_t,
        dst_stride: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: libc::c_int,
    pub no_fancy_upsampling: libc::c_int,
    pub use_cropping: libc::c_int,
    pub crop_left: libc::c_int,
    pub crop_top: libc::c_int,
    pub crop_width: libc::c_int,
    pub crop_height: libc::c_int,
    pub use_scaling: libc::c_int,
    pub scaled_width: libc::c_int,
    pub scaled_height: libc::c_int,
    pub use_threads: libc::c_int,
    pub dithering_strength: libc::c_int,
    pub flip: libc::c_int,
    pub alpha_dithering_strength: libc::c_int,
    pub pad: [uint32_t; 5],
}
pub type VP8StatusCode = libc::c_uint;
pub const VP8_STATUS_NOT_ENOUGH_DATA: VP8StatusCode = 7;
pub const VP8_STATUS_USER_ABORT: VP8StatusCode = 6;
pub const VP8_STATUS_SUSPENDED: VP8StatusCode = 5;
pub const VP8_STATUS_UNSUPPORTED_FEATURE: VP8StatusCode = 4;
pub const VP8_STATUS_BITSTREAM_ERROR: VP8StatusCode = 3;
pub const VP8_STATUS_INVALID_PARAM: VP8StatusCode = 2;
pub const VP8_STATUS_OUT_OF_MEMORY: VP8StatusCode = 1;
pub const VP8_STATUS_OK: VP8StatusCode = 0;
#[inline]
unsafe extern "C" fn WebPIsPremultipliedMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return (mode as libc::c_uint == MODE_rgbA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_bgrA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_Argb as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_rgbA_4444 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsAlphaMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return (mode as libc::c_uint == MODE_RGBA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_BGRA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_ARGB as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_RGBA_4444 as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_YUVA as libc::c_int as libc::c_uint
        || WebPIsPremultipliedMode(mode) != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsRGBMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return ((mode as libc::c_uint) < MODE_YUV as libc::c_int as libc::c_uint)
        as libc::c_int;
}
static mut kModeBpp: [uint8_t; 13] = [
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
];
unsafe extern "C" fn IsValidColorspace(mut webp_csp_mode: libc::c_int) -> libc::c_int {
    return (webp_csp_mode >= MODE_RGB as libc::c_int
        && webp_csp_mode < MODE_LAST as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn CheckDecBuffer(buffer: *const WebPDecBuffer) -> VP8StatusCode {
    let mut ok: libc::c_int = 1 as libc::c_int;
    let mode: WEBP_CSP_MODE = (*buffer).colorspace;
    let width: libc::c_int = (*buffer).width;
    let height: libc::c_int = (*buffer).height;
    if IsValidColorspace(mode as libc::c_int) == 0 {
        ok = 0 as libc::c_int;
    } else if WebPIsRGBMode(mode) == 0 {
        let buf: *const WebPYUVABuffer = &(*buffer).u.YUVA;
        let uv_width: libc::c_int = (width + 1 as libc::c_int) / 2 as libc::c_int;
        let uv_height: libc::c_int = (height + 1 as libc::c_int) / 2 as libc::c_int;
        let y_stride: libc::c_int = abs((*buf).y_stride);
        let u_stride: libc::c_int = abs((*buf).u_stride);
        let v_stride: libc::c_int = abs((*buf).v_stride);
        let a_stride: libc::c_int = abs((*buf).a_stride);
        let y_size: uint64_t = (y_stride as uint64_t)
            .wrapping_mul((height - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(width as libc::c_ulong);
        let u_size: uint64_t = (u_stride as uint64_t)
            .wrapping_mul((uv_height - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(uv_width as libc::c_ulong);
        let v_size: uint64_t = (v_stride as uint64_t)
            .wrapping_mul((uv_height - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(uv_width as libc::c_ulong);
        let a_size: uint64_t = (a_stride as uint64_t)
            .wrapping_mul((height - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(width as libc::c_ulong);
        ok &= (y_size <= (*buf).y_size) as libc::c_int;
        ok &= (u_size <= (*buf).u_size) as libc::c_int;
        ok &= (v_size <= (*buf).v_size) as libc::c_int;
        ok &= (y_stride >= width) as libc::c_int;
        ok &= (u_stride >= uv_width) as libc::c_int;
        ok &= (v_stride >= uv_width) as libc::c_int;
        ok &= ((*buf).y != 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int;
        ok &= ((*buf).u != 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int;
        ok &= ((*buf).v != 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int;
        if mode as libc::c_uint == MODE_YUVA as libc::c_int as libc::c_uint {
            ok &= (a_stride >= width) as libc::c_int;
            ok &= (a_size <= (*buf).a_size) as libc::c_int;
            ok &= ((*buf).a != 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int;
        }
    } else {
        let buf_0: *const WebPRGBABuffer = &(*buffer).u.RGBA;
        let stride: libc::c_int = abs((*buf_0).stride);
        let size: uint64_t = (stride as uint64_t)
            .wrapping_mul((height - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(
                (width as uint64_t)
                    .wrapping_mul(kModeBpp[mode as usize] as libc::c_ulong),
            );
        ok &= (size <= (*buf_0).size) as libc::c_int;
        ok &= (stride >= width * kModeBpp[mode as usize] as libc::c_int) as libc::c_int;
        ok &= ((*buf_0).rgba != 0 as *mut libc::c_void as *mut uint8_t) as libc::c_int;
    }
    return (if ok != 0 {
        VP8_STATUS_OK as libc::c_int
    } else {
        VP8_STATUS_INVALID_PARAM as libc::c_int
    }) as VP8StatusCode;
}
unsafe extern "C" fn AllocateBuffer(buffer: *mut WebPDecBuffer) -> VP8StatusCode {
    let w: libc::c_int = (*buffer).width;
    let h: libc::c_int = (*buffer).height;
    let mode: WEBP_CSP_MODE = (*buffer).colorspace;
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int
        || IsValidColorspace(mode as libc::c_int) == 0
    {
        return VP8_STATUS_INVALID_PARAM;
    }
    if (*buffer).is_external_memory <= 0 as libc::c_int
        && ((*buffer).private_memory).is_null()
    {
        let mut output: *mut uint8_t = 0 as *mut uint8_t;
        let mut uv_stride: libc::c_int = 0 as libc::c_int;
        let mut a_stride: libc::c_int = 0 as libc::c_int;
        let mut uv_size: uint64_t = 0 as libc::c_int as uint64_t;
        let mut a_size: uint64_t = 0 as libc::c_int as uint64_t;
        let mut total_size: uint64_t = 0;
        let mut stride: libc::c_int = 0;
        let mut size: uint64_t = 0;
        if (w as uint64_t).wrapping_mul(kModeBpp[mode as usize] as libc::c_ulong)
            as libc::c_ulonglong >= (1 as libc::c_ulonglong) << 31 as libc::c_int
        {
            return VP8_STATUS_INVALID_PARAM;
        }
        stride = w * kModeBpp[mode as usize] as libc::c_int;
        size = (stride as uint64_t).wrapping_mul(h as libc::c_ulong);
        if WebPIsRGBMode(mode) == 0 {
            uv_stride = (w + 1 as libc::c_int) / 2 as libc::c_int;
            uv_size = (uv_stride as uint64_t)
                .wrapping_mul(
                    ((h + 1 as libc::c_int) / 2 as libc::c_int) as libc::c_ulong,
                );
            if mode as libc::c_uint == MODE_YUVA as libc::c_int as libc::c_uint {
                a_stride = w;
                a_size = (a_stride as uint64_t).wrapping_mul(h as libc::c_ulong);
            }
        }
        total_size = size
            .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(uv_size))
            .wrapping_add(a_size);
        output = WebPSafeMalloc(
            total_size,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        ) as *mut uint8_t;
        if output.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        (*buffer).private_memory = output;
        if WebPIsRGBMode(mode) == 0 {
            let buf: *mut WebPYUVABuffer = &mut (*buffer).u.YUVA;
            (*buf).y = output;
            (*buf).y_stride = stride;
            (*buf).y_size = size;
            (*buf).u = output.offset(size as isize);
            (*buf).u_stride = uv_stride;
            (*buf).u_size = uv_size;
            (*buf).v = output.offset(size as isize).offset(uv_size as isize);
            (*buf).v_stride = uv_stride;
            (*buf).v_size = uv_size;
            if mode as libc::c_uint == MODE_YUVA as libc::c_int as libc::c_uint {
                (*buf)
                    .a = output
                    .offset(size as isize)
                    .offset(
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(uv_size)
                            as isize,
                    );
            }
            (*buf).a_size = a_size;
            (*buf).a_stride = a_stride;
        } else {
            let buf_0: *mut WebPRGBABuffer = &mut (*buffer).u.RGBA;
            (*buf_0).rgba = output;
            (*buf_0).stride = stride;
            (*buf_0).size = size;
        }
    }
    return CheckDecBuffer(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn WebPFlipBuffer(buffer: *mut WebPDecBuffer) -> VP8StatusCode {
    if buffer.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    if WebPIsRGBMode((*buffer).colorspace) != 0 {
        let buf: *mut WebPRGBABuffer = &mut (*buffer).u.RGBA;
        (*buf)
            .rgba = ((*buf).rgba)
            .offset(
                (((*buffer).height - 1 as libc::c_int) as int64_t
                    * (*buf).stride as libc::c_long) as isize,
            );
        (*buf).stride = -(*buf).stride;
    } else {
        let buf_0: *mut WebPYUVABuffer = &mut (*buffer).u.YUVA;
        let H: int64_t = (*buffer).height as int64_t;
        (*buf_0)
            .y = ((*buf_0).y)
            .offset(
                ((H - 1 as libc::c_int as libc::c_long)
                    * (*buf_0).y_stride as libc::c_long) as isize,
            );
        (*buf_0).y_stride = -(*buf_0).y_stride;
        (*buf_0)
            .u = ((*buf_0).u)
            .offset(
                ((H - 1 as libc::c_int as libc::c_long >> 1 as libc::c_int)
                    * (*buf_0).u_stride as libc::c_long) as isize,
            );
        (*buf_0).u_stride = -(*buf_0).u_stride;
        (*buf_0)
            .v = ((*buf_0).v)
            .offset(
                ((H - 1 as libc::c_int as libc::c_long >> 1 as libc::c_int)
                    * (*buf_0).v_stride as libc::c_long) as isize,
            );
        (*buf_0).v_stride = -(*buf_0).v_stride;
        if !((*buf_0).a).is_null() {
            (*buf_0)
                .a = ((*buf_0).a)
                .offset(
                    ((H - 1 as libc::c_int as libc::c_long)
                        * (*buf_0).a_stride as libc::c_long) as isize,
                );
            (*buf_0).a_stride = -(*buf_0).a_stride;
        }
    }
    return VP8_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAllocateDecBuffer(
    mut width: libc::c_int,
    mut height: libc::c_int,
    options: *const WebPDecoderOptions,
    buffer: *mut WebPDecBuffer,
) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if buffer.is_null() || width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return VP8_STATUS_INVALID_PARAM;
    }
    if !options.is_null() {
        if (*options).use_cropping != 0 {
            let cw: libc::c_int = (*options).crop_width;
            let ch: libc::c_int = (*options).crop_height;
            let x: libc::c_int = (*options).crop_left & !(1 as libc::c_int);
            let y: libc::c_int = (*options).crop_top & !(1 as libc::c_int);
            if WebPCheckCropDimensions(width, height, x, y, cw, ch) == 0 {
                return VP8_STATUS_INVALID_PARAM;
            }
            width = cw;
            height = ch;
        }
        if (*options).use_scaling != 0 {
            let mut scaled_width: libc::c_int = (*options).scaled_width;
            let mut scaled_height: libc::c_int = (*options).scaled_height;
            if WebPRescalerGetScaledDimensions(
                width,
                height,
                &mut scaled_width,
                &mut scaled_height,
            ) == 0
            {
                return VP8_STATUS_INVALID_PARAM;
            }
            width = scaled_width;
            height = scaled_height;
        }
    }
    (*buffer).width = width;
    (*buffer).height = height;
    status = AllocateBuffer(buffer);
    if status as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
        return status;
    }
    if !options.is_null() && (*options).flip != 0 {
        status = WebPFlipBuffer(buffer);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitDecBufferInternal(
    mut buffer: *mut WebPDecBuffer,
    mut version: libc::c_int,
) -> libc::c_int {
    if version >> 8 as libc::c_int != 0x209 as libc::c_int >> 8 as libc::c_int {
        return 0 as libc::c_int;
    }
    if buffer.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        buffer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPDecBuffer>() as libc::c_ulong,
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPFreeDecBuffer(mut buffer: *mut WebPDecBuffer) {
    if !buffer.is_null() {
        if (*buffer).is_external_memory <= 0 as libc::c_int {
            WebPSafeFree((*buffer).private_memory as *mut libc::c_void);
        }
        (*buffer).private_memory = 0 as *mut uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPCopyDecBuffer(
    src: *const WebPDecBuffer,
    dst: *mut WebPDecBuffer,
) {
    if !src.is_null() && !dst.is_null() {
        *dst = *src;
        if !((*src).private_memory).is_null() {
            (*dst).is_external_memory = 1 as libc::c_int;
            (*dst).private_memory = 0 as *mut uint8_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPGrabDecBuffer(
    src: *mut WebPDecBuffer,
    dst: *mut WebPDecBuffer,
) {
    if !src.is_null() && !dst.is_null() {
        *dst = *src;
        if !((*src).private_memory).is_null() {
            (*src).is_external_memory = 1 as libc::c_int;
            (*src).private_memory = 0 as *mut uint8_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPCopyDecBufferPixels(
    src_buf: *const WebPDecBuffer,
    dst_buf: *mut WebPDecBuffer,
) -> VP8StatusCode {
    (*dst_buf).width = (*src_buf).width;
    (*dst_buf).height = (*src_buf).height;
    if CheckDecBuffer(dst_buf) as libc::c_uint
        != VP8_STATUS_OK as libc::c_int as libc::c_uint
    {
        return VP8_STATUS_INVALID_PARAM;
    }
    if WebPIsRGBMode((*src_buf).colorspace) != 0 {
        let src: *const WebPRGBABuffer = &(*src_buf).u.RGBA;
        let dst: *const WebPRGBABuffer = &mut (*dst_buf).u.RGBA;
        WebPCopyPlane(
            (*src).rgba,
            (*src).stride,
            (*dst).rgba,
            (*dst).stride,
            (*src_buf).width * kModeBpp[(*src_buf).colorspace as usize] as libc::c_int,
            (*src_buf).height,
        );
    } else {
        let src_0: *const WebPYUVABuffer = &(*src_buf).u.YUVA;
        let dst_0: *const WebPYUVABuffer = &mut (*dst_buf).u.YUVA;
        WebPCopyPlane(
            (*src_0).y,
            (*src_0).y_stride,
            (*dst_0).y,
            (*dst_0).y_stride,
            (*src_buf).width,
            (*src_buf).height,
        );
        WebPCopyPlane(
            (*src_0).u,
            (*src_0).u_stride,
            (*dst_0).u,
            (*dst_0).u_stride,
            ((*src_buf).width + 1 as libc::c_int) / 2 as libc::c_int,
            ((*src_buf).height + 1 as libc::c_int) / 2 as libc::c_int,
        );
        WebPCopyPlane(
            (*src_0).v,
            (*src_0).v_stride,
            (*dst_0).v,
            (*dst_0).v_stride,
            ((*src_buf).width + 1 as libc::c_int) / 2 as libc::c_int,
            ((*src_buf).height + 1 as libc::c_int) / 2 as libc::c_int,
        );
        if WebPIsAlphaMode((*src_buf).colorspace) != 0 {
            WebPCopyPlane(
                (*src_0).a,
                (*src_0).a_stride,
                (*dst_0).a,
                (*dst_0).a_stride,
                (*src_buf).width,
                (*src_buf).height,
            );
        }
    }
    return VP8_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAvoidSlowMemory(
    output: *const WebPDecBuffer,
    features: *const WebPBitstreamFeatures,
) -> libc::c_int {
    return ((*output).is_external_memory >= 2 as libc::c_int
        && WebPIsPremultipliedMode((*output).colorspace) != 0
        && (!features.is_null() && (*features).has_alpha != 0)) as libc::c_int;
}
