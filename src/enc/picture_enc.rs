use ::libc;

use crate::src::utils::types::{WebPAuxStats, WebPEncodingError, WebPPicture, VP8_ENC_ERROR_BAD_DIMENSION, VP8_ENC_ERROR_INVALID_CONFIGURATION, VP8_ENC_ERROR_OUT_OF_MEMORY, VP8_ENC_OK};
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPPictureImportRGB(
        picture: *mut WebPPicture,
        rgb: *const uint8_t,
        rgb_stride: libc::c_int,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPEncode(config: *const WebPConfig, picture: *mut WebPPicture) -> libc::c_int;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn WebPConfigInitInternal(
        _: *mut WebPConfig,
        _: WebPPreset,
        _: libc::c_float,
        _: libc::c_int,
    ) -> libc::c_int;
    fn WebPPictureImportBGR(
        picture: *mut WebPPicture,
        bgr: *const uint8_t,
        bgr_stride: libc::c_int,
    ) -> libc::c_int;
    fn WebPPictureImportRGBA(
        picture: *mut WebPPicture,
        rgba: *const uint8_t,
        rgba_stride: libc::c_int,
    ) -> libc::c_int;
    fn WebPPictureImportBGRA(
        picture: *mut WebPPicture,
        bgra: *const uint8_t,
        bgra_stride: libc::c_int,
    ) -> libc::c_int;
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
pub type uintptr_t = libc::c_ulong;

pub type WebPProgressHook = Option::<
    unsafe extern "C" fn(libc::c_int, *const WebPPicture) -> libc::c_int,
>;

pub type WebPWriterFunction = Option::<
    unsafe extern "C" fn(*const uint8_t, size_t, *const WebPPicture) -> libc::c_int,
>;
pub type WebPEncCSP = libc::c_uint;
pub const WEBP_CSP_ALPHA_BIT: WebPEncCSP = 4;
pub const WEBP_CSP_UV_MASK: WebPEncCSP = 3;
pub const WEBP_YUV420A: WebPEncCSP = 4;
pub const WEBP_YUV420: WebPEncCSP = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPConfig {
    pub lossless: libc::c_int,
    pub quality: libc::c_float,
    pub method: libc::c_int,
    pub image_hint: WebPImageHint,
    pub target_size: libc::c_int,
    pub target_PSNR: libc::c_float,
    pub segments: libc::c_int,
    pub sns_strength: libc::c_int,
    pub filter_strength: libc::c_int,
    pub filter_sharpness: libc::c_int,
    pub filter_type: libc::c_int,
    pub autofilter: libc::c_int,
    pub alpha_compression: libc::c_int,
    pub alpha_filtering: libc::c_int,
    pub alpha_quality: libc::c_int,
    pub pass: libc::c_int,
    pub show_compressed: libc::c_int,
    pub preprocessing: libc::c_int,
    pub partitions: libc::c_int,
    pub partition_limit: libc::c_int,
    pub emulate_jpeg_size: libc::c_int,
    pub thread_level: libc::c_int,
    pub low_memory: libc::c_int,
    pub near_lossless: libc::c_int,
    pub exact: libc::c_int,
    pub use_delta_palette: libc::c_int,
    pub use_sharp_yuv: libc::c_int,
    pub qmin: libc::c_int,
    pub qmax: libc::c_int,
}
pub type WebPImageHint = libc::c_uint;
pub const WEBP_HINT_LAST: WebPImageHint = 4;
pub const WEBP_HINT_GRAPH: WebPImageHint = 3;
pub const WEBP_HINT_PHOTO: WebPImageHint = 2;
pub const WEBP_HINT_PICTURE: WebPImageHint = 1;
pub const WEBP_HINT_DEFAULT: WebPImageHint = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMemoryWriter {
    pub mem: *mut uint8_t,
    pub size: size_t,
    pub max_size: size_t,
    pub pad: [uint32_t; 1],
}
pub type Importer = Option::<
    unsafe extern "C" fn(*mut WebPPicture, *const uint8_t, libc::c_int) -> libc::c_int,
>;
pub type WebPPreset = libc::c_uint;
pub const WEBP_PRESET_TEXT: WebPPreset = 5;
pub const WEBP_PRESET_ICON: WebPPreset = 4;
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> libc::c_int {
    return WebPPictureInitInternal(picture, 0x20f as libc::c_int);
}
#[inline]
unsafe extern "C" fn WebPConfigPreset(
    mut config: *mut WebPConfig,
    mut preset: WebPPreset,
    mut quality: libc::c_float,
) -> libc::c_int {
    return WebPConfigInitInternal(config, preset, quality, 0x20f as libc::c_int);
}
unsafe extern "C" fn DummyWriter(
    mut data: *const uint8_t,
    mut data_size: size_t,
    picture: *const WebPPicture,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureInitInternal(
    mut picture: *mut WebPPicture,
    mut version: libc::c_int,
) -> libc::c_int {
    if version >> 8 as libc::c_int != 0x20f as libc::c_int >> 8 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !picture.is_null() {
        memset(
            picture as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<WebPPicture>() as libc::c_ulong,
        );
        (*picture)
            .writer = Some(
            DummyWriter
                as unsafe extern "C" fn(
                    *const uint8_t,
                    size_t,
                    *const WebPPicture,
                ) -> libc::c_int,
        );
        WebPEncodingSetError(picture, VP8_ENC_OK);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPValidatePicture(
    picture: *const WebPPicture,
) -> libc::c_int {
    if picture.is_null() {
        return 0 as libc::c_int;
    }
    if (*picture).width <= 0 as libc::c_int || (*picture).height <= 0 as libc::c_int {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    if (*picture).width <= 0 as libc::c_int
        || (*picture).width / 4 as libc::c_int
            > 2147483647 as libc::c_int / 4 as libc::c_int
        || (*picture).height <= 0 as libc::c_int
        || (*picture).height / 4 as libc::c_int
            > 2147483647 as libc::c_int / 4 as libc::c_int
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    if (*picture).colorspace as libc::c_uint
        != WEBP_YUV420 as libc::c_int as libc::c_uint
        && (*picture).colorspace as libc::c_uint
            != WEBP_YUV420A as libc::c_int as libc::c_uint
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn WebPPictureResetBufferARGB(picture: *mut WebPPicture) {
    (*picture).memory_argb_ = 0 as *mut libc::c_void;
    (*picture).argb = 0 as *mut uint32_t;
    (*picture).argb_stride = 0 as libc::c_int;
}
unsafe extern "C" fn WebPPictureResetBufferYUVA(picture: *mut WebPPicture) {
    (*picture).memory_ = 0 as *mut libc::c_void;
    (*picture).a = 0 as *mut uint8_t;
    (*picture).v = (*picture).a;
    (*picture).u = (*picture).v;
    (*picture).y = (*picture).u;
    (*picture).uv_stride = 0 as libc::c_int;
    (*picture).y_stride = (*picture).uv_stride;
    (*picture).a_stride = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureResetBuffers(picture: *mut WebPPicture) {
    WebPPictureResetBufferARGB(picture);
    WebPPictureResetBufferYUVA(picture);
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureAllocARGB(picture: *mut WebPPicture) -> libc::c_int {
    let mut memory: *mut libc::c_void = 0 as *mut libc::c_void;
    let width: libc::c_int = (*picture).width;
    let height: libc::c_int = (*picture).height;
    let argb_size: uint64_t = (width as uint64_t).wrapping_mul(height as libc::c_ulong);
    if WebPValidatePicture(picture) == 0 {
        return 0 as libc::c_int;
    }
    WebPSafeFree((*picture).memory_argb_);
    WebPPictureResetBufferARGB(picture);
    memory = WebPSafeMalloc(
        argb_size.wrapping_add(31 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    if memory.is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    (*picture).memory_argb_ = memory;
    (*picture)
        .argb = ((memory as uintptr_t).wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut uint32_t;
    (*picture).argb_stride = width;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureAllocYUVA(picture: *mut WebPPicture) -> libc::c_int {
    let has_alpha: libc::c_int = (*picture).colorspace as libc::c_int
        & WEBP_CSP_ALPHA_BIT as libc::c_int;
    let width: libc::c_int = (*picture).width;
    let height: libc::c_int = (*picture).height;
    let y_stride: libc::c_int = width;
    let uv_width: libc::c_int = (width as int64_t + 1 as libc::c_int as libc::c_long
        >> 1 as libc::c_int) as libc::c_int;
    let uv_height: libc::c_int = (height as int64_t + 1 as libc::c_int as libc::c_long
        >> 1 as libc::c_int) as libc::c_int;
    let uv_stride: libc::c_int = uv_width;
    let mut a_width: libc::c_int = 0;
    let mut a_stride: libc::c_int = 0;
    let mut y_size: uint64_t = 0;
    let mut uv_size: uint64_t = 0;
    let mut a_size: uint64_t = 0;
    let mut total_size: uint64_t = 0;
    let mut mem: *mut uint8_t = 0 as *mut uint8_t;
    if WebPValidatePicture(picture) == 0 {
        return 0 as libc::c_int;
    }
    WebPSafeFree((*picture).memory_);
    WebPPictureResetBufferYUVA(picture);
    a_width = if has_alpha != 0 { width } else { 0 as libc::c_int };
    a_stride = a_width;
    y_size = (y_stride as uint64_t).wrapping_mul(height as libc::c_ulong);
    uv_size = (uv_stride as uint64_t).wrapping_mul(uv_height as libc::c_ulong);
    a_size = (a_stride as uint64_t).wrapping_mul(height as libc::c_ulong);
    total_size = y_size
        .wrapping_add(a_size)
        .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(uv_size));
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int
        || uv_width <= 0 as libc::c_int || uv_height <= 0 as libc::c_int
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    mem = WebPSafeMalloc(total_size, ::core::mem::size_of::<uint8_t>() as libc::c_ulong)
        as *mut uint8_t;
    if mem.is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    (*picture).memory_ = mem as *mut libc::c_void;
    (*picture).y_stride = y_stride;
    (*picture).uv_stride = uv_stride;
    (*picture).a_stride = a_stride;
    (*picture).y = mem;
    mem = mem.offset(y_size as isize);
    (*picture).u = mem;
    mem = mem.offset(uv_size as isize);
    (*picture).v = mem;
    mem = mem.offset(uv_size as isize);
    if a_size > 0 as libc::c_int as libc::c_ulong {
        (*picture).a = mem;
        mem = mem.offset(a_size as isize);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureAlloc(mut picture: *mut WebPPicture) -> libc::c_int {
    if !picture.is_null() {
        WebPPictureFree(picture);
        if (*picture).use_argb == 0 {
            return WebPPictureAllocYUVA(picture)
        } else {
            return WebPPictureAllocARGB(picture)
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureFree(mut picture: *mut WebPPicture) {
    if !picture.is_null() {
        WebPSafeFree((*picture).memory_);
        WebPSafeFree((*picture).memory_argb_);
        WebPPictureResetBuffers(picture);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPMemoryWriterInit(mut writer: *mut WebPMemoryWriter) {
    (*writer).mem = 0 as *mut uint8_t;
    (*writer).size = 0 as libc::c_int as size_t;
    (*writer).max_size = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMemoryWrite(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut picture: *const WebPPicture,
) -> libc::c_int {
    let w: *mut WebPMemoryWriter = (*picture).custom_ptr as *mut WebPMemoryWriter;
    let mut next_size: uint64_t = 0;
    if w.is_null() {
        return 1 as libc::c_int;
    }
    next_size = ((*w).size).wrapping_add(data_size);
    if next_size > (*w).max_size {
        let mut new_mem: *mut uint8_t = 0 as *mut uint8_t;
        let mut next_max_size: uint64_t = (2 as libc::c_ulonglong)
            .wrapping_mul((*w).max_size as libc::c_ulonglong) as uint64_t;
        if next_max_size < next_size {
            next_max_size = next_size;
        }
        if (next_max_size as libc::c_ulonglong) < 8192 as libc::c_ulonglong {
            next_max_size = 8192 as libc::c_ulonglong as uint64_t;
        }
        new_mem = WebPSafeMalloc(next_max_size, 1 as libc::c_int as size_t)
            as *mut uint8_t;
        if new_mem.is_null() {
            return 0 as libc::c_int;
        }
        if (*w).size > 0 as libc::c_int as libc::c_ulong {
            memcpy(
                new_mem as *mut libc::c_void,
                (*w).mem as *const libc::c_void,
                (*w).size,
            );
        }
        WebPSafeFree((*w).mem as *mut libc::c_void);
        (*w).mem = new_mem;
        (*w).max_size = next_max_size;
    }
    if data_size > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            ((*w).mem).offset((*w).size as isize) as *mut libc::c_void,
            data as *const libc::c_void,
            data_size,
        );
        (*w)
            .size = ((*w).size as libc::c_ulong).wrapping_add(data_size) as size_t
            as size_t;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMemoryWriterClear(mut writer: *mut WebPMemoryWriter) {
    if !writer.is_null() {
        WebPSafeFree((*writer).mem as *mut libc::c_void);
        (*writer).mem = 0 as *mut uint8_t;
        (*writer).size = 0 as libc::c_int as size_t;
        (*writer).max_size = 0 as libc::c_int as size_t;
    }
}
unsafe extern "C" fn Encode(
    mut rgba: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut import: Importer,
    mut quality_factor: libc::c_float,
    mut lossless: libc::c_int,
    mut output: *mut *mut uint8_t,
) -> size_t {
    let mut pic: WebPPicture = WebPPicture {
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
    let mut config: WebPConfig = WebPConfig {
        lossless: 0,
        quality: 0.,
        method: 0,
        image_hint: WEBP_HINT_DEFAULT,
        target_size: 0,
        target_PSNR: 0.,
        segments: 0,
        sns_strength: 0,
        filter_strength: 0,
        filter_sharpness: 0,
        filter_type: 0,
        autofilter: 0,
        alpha_compression: 0,
        alpha_filtering: 0,
        alpha_quality: 0,
        pass: 0,
        show_compressed: 0,
        preprocessing: 0,
        partitions: 0,
        partition_limit: 0,
        emulate_jpeg_size: 0,
        thread_level: 0,
        low_memory: 0,
        near_lossless: 0,
        exact: 0,
        use_delta_palette: 0,
        use_sharp_yuv: 0,
        qmin: 0,
        qmax: 0,
    };
    let mut wrt: WebPMemoryWriter = WebPMemoryWriter {
        mem: 0 as *mut uint8_t,
        size: 0,
        max_size: 0,
        pad: [0; 1],
    };
    let mut ok: libc::c_int = 0;
    if output.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if WebPConfigPreset(&mut config, WEBP_PRESET_DEFAULT, quality_factor) == 0
        || WebPPictureInit(&mut pic) == 0
    {
        return 0 as libc::c_int as size_t;
    }
    config.lossless = (lossless != 0) as libc::c_int;
    pic.use_argb = (lossless != 0) as libc::c_int;
    pic.width = width;
    pic.height = height;
    pic
        .writer = Some(
        WebPMemoryWrite
            as unsafe extern "C" fn(
                *const uint8_t,
                size_t,
                *const WebPPicture,
            ) -> libc::c_int,
    );
    pic.custom_ptr = &mut wrt as *mut WebPMemoryWriter as *mut libc::c_void;
    WebPMemoryWriterInit(&mut wrt);
    ok = (import.expect("non-null function pointer")(&mut pic, rgba, stride) != 0
        && WebPEncode(&mut config, &mut pic) != 0) as libc::c_int;
    WebPPictureFree(&mut pic);
    if ok == 0 {
        WebPMemoryWriterClear(&mut wrt);
        *output = 0 as *mut uint8_t;
        return 0 as libc::c_int as size_t;
    }
    *output = wrt.mem;
    return wrt.size;
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeRGB(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut q: libc::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGB
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        q,
        0 as libc::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeRGBA(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut q: libc::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGBA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        q,
        0 as libc::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeBGR(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut q: libc::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGR
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        q,
        0 as libc::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeBGRA(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut q: libc::c_float,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGRA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        q,
        0 as libc::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessRGB(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGB
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        70.0f64 as libc::c_float,
        1 as libc::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessRGBA(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportRGBA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        70.0f64 as libc::c_float,
        1 as libc::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessBGR(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGR
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        70.0f64 as libc::c_float,
        1 as libc::c_int,
        out,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPEncodeLosslessBGRA(
    mut in_0: *const uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bps: libc::c_int,
    mut out: *mut *mut uint8_t,
) -> size_t {
    return Encode(
        in_0,
        w,
        h,
        bps,
        Some(
            WebPPictureImportBGRA
                as unsafe extern "C" fn(
                    *mut WebPPicture,
                    *const uint8_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        70.0f64 as libc::c_float,
        1 as libc::c_int,
        out,
    );
}
