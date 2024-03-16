use ::libc;
extern "C" {
    fn WebPPictureAlloc(picture: *mut WebPPicture) -> libc::c_int;
    fn WebPPictureFree(picture: *mut WebPPicture);
    fn WebPCopyPlane(
        src: *const uint8_t,
        src_stride: libc::c_int,
        dst: *mut uint8_t,
        dst_stride: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );
    fn WebPPictureResetBuffers(picture: *mut WebPPicture);
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn WebPMultRows(
        ptr: *mut uint8_t,
        stride: libc::c_int,
        alpha: *const uint8_t,
        alpha_stride: libc::c_int,
        width: libc::c_int,
        num_rows: libc::c_int,
        inverse: libc::c_int,
    );
    fn WebPMultARGBRows(
        ptr: *mut uint8_t,
        stride: libc::c_int,
        width: libc::c_int,
        num_rows: libc::c_int,
        inverse: libc::c_int,
    );
    fn WebPInitAlphaProcessing();
    fn WebPRescalerInit(
        rescaler: *mut WebPRescaler,
        src_width: libc::c_int,
        src_height: libc::c_int,
        dst: *mut uint8_t,
        dst_width: libc::c_int,
        dst_height: libc::c_int,
        dst_stride: libc::c_int,
        num_channels: libc::c_int,
        work: *mut rescaler_t,
    ) -> libc::c_int;
    fn WebPRescalerGetScaledDimensions(
        src_width: libc::c_int,
        src_height: libc::c_int,
        scaled_width: *mut libc::c_int,
        scaled_height: *mut libc::c_int,
    ) -> libc::c_int;
    fn WebPRescalerImport(
        rescaler: *mut WebPRescaler,
        num_rows: libc::c_int,
        src: *const uint8_t,
        src_stride: libc::c_int,
    ) -> libc::c_int;
    fn WebPRescalerExport(rescaler: *mut WebPRescaler) -> libc::c_int;
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
pub type rescaler_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRescaler {
    pub x_expand: libc::c_int,
    pub y_expand: libc::c_int,
    pub num_channels: libc::c_int,
    pub fx_scale: uint32_t,
    pub fy_scale: uint32_t,
    pub fxy_scale: uint32_t,
    pub y_accum: libc::c_int,
    pub y_add: libc::c_int,
    pub y_sub: libc::c_int,
    pub x_add: libc::c_int,
    pub x_sub: libc::c_int,
    pub src_width: libc::c_int,
    pub src_height: libc::c_int,
    pub dst_width: libc::c_int,
    pub dst_height: libc::c_int,
    pub src_y: libc::c_int,
    pub dst_y: libc::c_int,
    pub dst: *mut uint8_t,
    pub dst_stride: libc::c_int,
    pub irow: *mut rescaler_t,
    pub frow: *mut rescaler_t,
}
unsafe extern "C" fn PictureGrabSpecs(src: *const WebPPicture, dst: *mut WebPPicture) {
    *dst = *src;
    WebPPictureResetBuffers(dst);
}
unsafe extern "C" fn SnapTopLeftPosition(
    pic: *const WebPPicture,
    left: *mut libc::c_int,
    top: *mut libc::c_int,
) {
    if (*pic).use_argb == 0 {
        *left &= !(1 as libc::c_int);
        *top &= !(1 as libc::c_int);
    }
}
unsafe extern "C" fn AdjustAndCheckRectangle(
    pic: *const WebPPicture,
    left: *mut libc::c_int,
    top: *mut libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    SnapTopLeftPosition(pic, left, top);
    if *left < 0 as libc::c_int || *top < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if *left + width > (*pic).width {
        return 0 as libc::c_int;
    }
    if *top + height > (*pic).height {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureCopy(
    mut src: *const WebPPicture,
    mut dst: *mut WebPPicture,
) -> libc::c_int {
    if src.is_null() || dst.is_null() {
        return 0 as libc::c_int;
    }
    if src == dst as *const WebPPicture {
        return 1 as libc::c_int;
    }
    PictureGrabSpecs(src, dst);
    if WebPPictureAlloc(dst) == 0 {
        return 0 as libc::c_int;
    }
    if (*src).use_argb == 0 {
        WebPCopyPlane(
            (*src).y,
            (*src).y_stride,
            (*dst).y,
            (*dst).y_stride,
            (*dst).width,
            (*dst).height,
        );
        WebPCopyPlane(
            (*src).u,
            (*src).uv_stride,
            (*dst).u,
            (*dst).uv_stride,
            (*dst).width + 1 as libc::c_int >> 1 as libc::c_int,
            (*dst).height + 1 as libc::c_int >> 1 as libc::c_int,
        );
        WebPCopyPlane(
            (*src).v,
            (*src).uv_stride,
            (*dst).v,
            (*dst).uv_stride,
            (*dst).width + 1 as libc::c_int >> 1 as libc::c_int,
            (*dst).height + 1 as libc::c_int >> 1 as libc::c_int,
        );
        if !((*dst).a).is_null() {
            WebPCopyPlane(
                (*src).a,
                (*src).a_stride,
                (*dst).a,
                (*dst).a_stride,
                (*dst).width,
                (*dst).height,
            );
        }
    } else {
        WebPCopyPlane(
            (*src).argb as *const uint8_t,
            4 as libc::c_int * (*src).argb_stride,
            (*dst).argb as *mut uint8_t,
            4 as libc::c_int * (*dst).argb_stride,
            4 as libc::c_int * (*dst).width,
            (*dst).height,
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureIsView(
    mut picture: *const WebPPicture,
) -> libc::c_int {
    if picture.is_null() {
        return 0 as libc::c_int;
    }
    if (*picture).use_argb != 0 {
        return ((*picture).memory_argb_ == 0 as *mut libc::c_void) as libc::c_int;
    }
    return ((*picture).memory_ == 0 as *mut libc::c_void) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureView(
    mut src: *const WebPPicture,
    mut left: libc::c_int,
    mut top: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut dst: *mut WebPPicture,
) -> libc::c_int {
    if src.is_null() || dst.is_null() {
        return 0 as libc::c_int;
    }
    if AdjustAndCheckRectangle(src, &mut left, &mut top, width, height) == 0 {
        return 0 as libc::c_int;
    }
    if src != dst as *const WebPPicture {
        PictureGrabSpecs(src, dst);
    }
    (*dst).width = width;
    (*dst).height = height;
    if (*src).use_argb == 0 {
        (*dst)
            .y = ((*src).y)
            .offset((top * (*src).y_stride) as isize)
            .offset(left as isize);
        (*dst)
            .u = ((*src).u)
            .offset(((top >> 1 as libc::c_int) * (*src).uv_stride) as isize)
            .offset((left >> 1 as libc::c_int) as isize);
        (*dst)
            .v = ((*src).v)
            .offset(((top >> 1 as libc::c_int) * (*src).uv_stride) as isize)
            .offset((left >> 1 as libc::c_int) as isize);
        (*dst).y_stride = (*src).y_stride;
        (*dst).uv_stride = (*src).uv_stride;
        if !((*src).a).is_null() {
            (*dst)
                .a = ((*src).a)
                .offset((top * (*src).a_stride) as isize)
                .offset(left as isize);
            (*dst).a_stride = (*src).a_stride;
        }
    } else {
        (*dst)
            .argb = ((*src).argb)
            .offset((top * (*src).argb_stride) as isize)
            .offset(left as isize);
        (*dst).argb_stride = (*src).argb_stride;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureCrop(
    mut pic: *mut WebPPicture,
    mut left: libc::c_int,
    mut top: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut tmp: WebPPicture = WebPPicture {
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
    if pic.is_null() {
        return 0 as libc::c_int;
    }
    if AdjustAndCheckRectangle(pic, &mut left, &mut top, width, height) == 0 {
        return 0 as libc::c_int;
    }
    PictureGrabSpecs(pic, &mut tmp);
    tmp.width = width;
    tmp.height = height;
    if WebPPictureAlloc(&mut tmp) == 0 {
        return WebPEncodingSetError(pic, tmp.error_code);
    }
    if (*pic).use_argb == 0 {
        let y_offset: libc::c_int = top * (*pic).y_stride + left;
        let uv_offset: libc::c_int = top / 2 as libc::c_int * (*pic).uv_stride
            + left / 2 as libc::c_int;
        WebPCopyPlane(
            ((*pic).y).offset(y_offset as isize),
            (*pic).y_stride,
            tmp.y,
            tmp.y_stride,
            width,
            height,
        );
        WebPCopyPlane(
            ((*pic).u).offset(uv_offset as isize),
            (*pic).uv_stride,
            tmp.u,
            tmp.uv_stride,
            width + 1 as libc::c_int >> 1 as libc::c_int,
            height + 1 as libc::c_int >> 1 as libc::c_int,
        );
        WebPCopyPlane(
            ((*pic).v).offset(uv_offset as isize),
            (*pic).uv_stride,
            tmp.v,
            tmp.uv_stride,
            width + 1 as libc::c_int >> 1 as libc::c_int,
            height + 1 as libc::c_int >> 1 as libc::c_int,
        );
        if !(tmp.a).is_null() {
            let a_offset: libc::c_int = top * (*pic).a_stride + left;
            WebPCopyPlane(
                ((*pic).a).offset(a_offset as isize),
                (*pic).a_stride,
                tmp.a,
                tmp.a_stride,
                width,
                height,
            );
        }
    } else {
        let src: *const uint8_t = ((*pic).argb)
            .offset((top * (*pic).argb_stride) as isize)
            .offset(left as isize) as *const uint8_t;
        WebPCopyPlane(
            src,
            (*pic).argb_stride * 4 as libc::c_int,
            tmp.argb as *mut uint8_t,
            tmp.argb_stride * 4 as libc::c_int,
            width * 4 as libc::c_int,
            height,
        );
    }
    WebPPictureFree(pic);
    *pic = tmp;
    return 1 as libc::c_int;
}
unsafe extern "C" fn RescalePlane(
    mut src: *const uint8_t,
    mut src_width: libc::c_int,
    mut src_height: libc::c_int,
    mut src_stride: libc::c_int,
    mut dst: *mut uint8_t,
    mut dst_width: libc::c_int,
    mut dst_height: libc::c_int,
    mut dst_stride: libc::c_int,
    work: *mut rescaler_t,
    mut num_channels: libc::c_int,
) -> libc::c_int {
    let mut rescaler: WebPRescaler = WebPRescaler {
        x_expand: 0,
        y_expand: 0,
        num_channels: 0,
        fx_scale: 0,
        fy_scale: 0,
        fxy_scale: 0,
        y_accum: 0,
        y_add: 0,
        y_sub: 0,
        x_add: 0,
        x_sub: 0,
        src_width: 0,
        src_height: 0,
        dst_width: 0,
        dst_height: 0,
        src_y: 0,
        dst_y: 0,
        dst: 0 as *mut uint8_t,
        dst_stride: 0,
        irow: 0 as *mut rescaler_t,
        frow: 0 as *mut rescaler_t,
    };
    let mut y: libc::c_int = 0 as libc::c_int;
    if WebPRescalerInit(
        &mut rescaler,
        src_width,
        src_height,
        dst,
        dst_width,
        dst_height,
        dst_stride,
        num_channels,
        work,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    while y < src_height {
        y
            += WebPRescalerImport(
                &mut rescaler,
                src_height - y,
                src.offset((y * src_stride) as isize),
                src_stride,
            );
        WebPRescalerExport(&mut rescaler);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn AlphaMultiplyARGB(pic: *mut WebPPicture, mut inverse: libc::c_int) {
    WebPMultARGBRows(
        (*pic).argb as *mut uint8_t,
        ((*pic).argb_stride as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_int,
        (*pic).width,
        (*pic).height,
        inverse,
    );
}
unsafe extern "C" fn AlphaMultiplyY(pic: *mut WebPPicture, mut inverse: libc::c_int) {
    if !((*pic).a).is_null() {
        WebPMultRows(
            (*pic).y,
            (*pic).y_stride,
            (*pic).a,
            (*pic).a_stride,
            (*pic).width,
            (*pic).height,
            inverse,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPPictureRescale(
    mut picture: *mut WebPPicture,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut tmp: WebPPicture = WebPPicture {
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
    let mut prev_width: libc::c_int = 0;
    let mut prev_height: libc::c_int = 0;
    let mut work: *mut rescaler_t = 0 as *mut rescaler_t;
    if picture.is_null() {
        return 0 as libc::c_int;
    }
    prev_width = (*picture).width;
    prev_height = (*picture).height;
    if WebPRescalerGetScaledDimensions(prev_width, prev_height, &mut width, &mut height)
        == 0
    {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
    }
    PictureGrabSpecs(picture, &mut tmp);
    tmp.width = width;
    tmp.height = height;
    if WebPPictureAlloc(&mut tmp) == 0 {
        return WebPEncodingSetError(picture, tmp.error_code);
    }
    if (*picture).use_argb == 0 {
        work = WebPSafeMalloc(
            (2 as libc::c_ulonglong).wrapping_mul(width as libc::c_ulonglong)
                as uint64_t,
            ::core::mem::size_of::<rescaler_t>() as libc::c_ulong,
        ) as *mut rescaler_t;
        if work.is_null() {
            WebPPictureFree(&mut tmp);
            return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        if !((*picture).a).is_null() {
            WebPInitAlphaProcessing();
            if RescalePlane(
                (*picture).a,
                prev_width,
                prev_height,
                (*picture).a_stride,
                tmp.a,
                width,
                height,
                tmp.a_stride,
                work,
                1 as libc::c_int,
            ) == 0
            {
                return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
            }
        }
        AlphaMultiplyY(picture, 0 as libc::c_int);
        if RescalePlane(
            (*picture).y,
            prev_width,
            prev_height,
            (*picture).y_stride,
            tmp.y,
            width,
            height,
            tmp.y_stride,
            work,
            1 as libc::c_int,
        ) == 0
            || RescalePlane(
                (*picture).u,
                prev_width + 1 as libc::c_int >> 1 as libc::c_int,
                prev_height + 1 as libc::c_int >> 1 as libc::c_int,
                (*picture).uv_stride,
                tmp.u,
                width + 1 as libc::c_int >> 1 as libc::c_int,
                height + 1 as libc::c_int >> 1 as libc::c_int,
                tmp.uv_stride,
                work,
                1 as libc::c_int,
            ) == 0
            || RescalePlane(
                (*picture).v,
                prev_width + 1 as libc::c_int >> 1 as libc::c_int,
                prev_height + 1 as libc::c_int >> 1 as libc::c_int,
                (*picture).uv_stride,
                tmp.v,
                width + 1 as libc::c_int >> 1 as libc::c_int,
                height + 1 as libc::c_int >> 1 as libc::c_int,
                tmp.uv_stride,
                work,
                1 as libc::c_int,
            ) == 0
        {
            return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
        }
        AlphaMultiplyY(&mut tmp, 1 as libc::c_int);
    } else {
        work = WebPSafeMalloc(
            (2 as libc::c_ulonglong)
                .wrapping_mul(width as libc::c_ulonglong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulonglong) as uint64_t,
            ::core::mem::size_of::<rescaler_t>() as libc::c_ulong,
        ) as *mut rescaler_t;
        if work.is_null() {
            WebPPictureFree(&mut tmp);
            return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        WebPInitAlphaProcessing();
        AlphaMultiplyARGB(picture, 0 as libc::c_int);
        if RescalePlane(
            (*picture).argb as *const uint8_t,
            prev_width,
            prev_height,
            (*picture).argb_stride * 4 as libc::c_int,
            tmp.argb as *mut uint8_t,
            width,
            height,
            tmp.argb_stride * 4 as libc::c_int,
            work,
            4 as libc::c_int,
        ) == 0
        {
            return WebPEncodingSetError(picture, VP8_ENC_ERROR_BAD_DIMENSION);
        }
        AlphaMultiplyARGB(&mut tmp, 1 as libc::c_int);
    }
    WebPPictureFree(picture);
    WebPSafeFree(work as *mut libc::c_void);
    *picture = tmp;
    return 1 as libc::c_int;
}
