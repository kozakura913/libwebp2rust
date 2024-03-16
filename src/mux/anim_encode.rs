use ::libc;

use crate::src::utils::types::{Candidate, ComparePixelsFunc, FrameRectangle, SubFrameParams, VP8StatusCode, WebPAuxStats, WebPBitstreamFeatures, WebPConfig, WebPData, WebPDecBuffer, WebPDecoderConfig, WebPDecoderOptions, WebPEncodingError, WebPMemoryWriter, WebPMux, WebPMuxAnimBlend, WebPMuxAnimDispose, WebPMuxFrameInfo, WebPPicture, WebPPreset, WebPProgressHook, WebPRGBABuffer, WebPRGBABuffer_WebPYUVABuffer, WebPYUVABuffer, MODE_BGRA, MODE_RGB, VP8_ENC_ERROR_INVALID_CONFIGURATION, VP8_ENC_OK, VP8_STATUS_OK, WEBP_CHUNK_ANMF, WEBP_CHUNK_VP8X, WEBP_HINT_DEFAULT, WEBP_MUX_BLEND, WEBP_MUX_DISPOSE_BACKGROUND, WEBP_MUX_DISPOSE_NONE, WEBP_MUX_NO_BLEND, WEBP_PRESET_DEFAULT, WEBP_YUV420};
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn WebPMalloc(size: size_t) -> *mut libc::c_void;
    fn WebPFree(ptr: *mut libc::c_void);
    fn WebPNewInternal(_: libc::c_int) -> *mut WebPMux;
    fn WebPMuxDelete(mux: *mut WebPMux);
    fn WebPMuxCreateInternal(
        _: *const WebPData,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut WebPMux;
    fn WebPMuxSetChunk(
        mux: *mut WebPMux,
        fourcc: *const libc::c_char,
        chunk_data: *const WebPData,
        copy_data: libc::c_int,
    ) -> WebPMuxError;
    fn WebPMuxGetChunk(
        mux: *const WebPMux,
        fourcc: *const libc::c_char,
        chunk_data: *mut WebPData,
    ) -> WebPMuxError;
    fn WebPMuxDeleteChunk(
        mux: *mut WebPMux,
        fourcc: *const libc::c_char,
    ) -> WebPMuxError;
    fn WebPMuxSetImage(
        mux: *mut WebPMux,
        bitstream: *const WebPData,
        copy_data: libc::c_int,
    ) -> WebPMuxError;
    fn WebPMuxPushFrame(
        mux: *mut WebPMux,
        frame: *const WebPMuxFrameInfo,
        copy_data: libc::c_int,
    ) -> WebPMuxError;
    fn WebPMuxGetFrame(
        mux: *const WebPMux,
        nth: uint32_t,
        frame: *mut WebPMuxFrameInfo,
    ) -> WebPMuxError;
    fn WebPMuxSetAnimationParams(
        mux: *mut WebPMux,
        params: *const WebPMuxAnimParams,
    ) -> WebPMuxError;
    fn WebPMuxSetCanvasSize(
        mux: *mut WebPMux,
        width: libc::c_int,
        height: libc::c_int,
    ) -> WebPMuxError;
    fn WebPMuxGetCanvasSize(
        mux: *const WebPMux,
        width: *mut libc::c_int,
        height: *mut libc::c_int,
    ) -> WebPMuxError;
    fn WebPMuxAssemble(mux: *mut WebPMux, assembled_data: *mut WebPData) -> WebPMuxError;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPCopyPixels(src: *const WebPPicture, dst: *mut WebPPicture);
    fn WebPGetColorPalette(
        pic: *const WebPPicture,
        palette: *mut uint32_t,
    ) -> libc::c_int;
    fn WebPGetFeaturesInternal(
        _: *const uint8_t,
        _: size_t,
        _: *mut WebPBitstreamFeatures,
        _: libc::c_int,
    ) -> VP8StatusCode;
    fn WebPInitDecoderConfigInternal(
        _: *mut WebPDecoderConfig,
        _: libc::c_int,
    ) -> libc::c_int;
    fn WebPDecode(
        data: *const uint8_t,
        data_size: size_t,
        config: *mut WebPDecoderConfig,
    ) -> VP8StatusCode;
    fn WebPConfigInitInternal(
        _: *mut WebPConfig,
        _: WebPPreset,
        _: libc::c_float,
        _: libc::c_int,
    ) -> libc::c_int;
    fn WebPValidateConfig(config: *const WebPConfig) -> libc::c_int;
    fn WebPMemoryWriterInit(writer: *mut WebPMemoryWriter);
    fn WebPMemoryWriterClear(writer: *mut WebPMemoryWriter);
    fn WebPMemoryWrite(
        data: *const uint8_t,
        data_size: size_t,
        picture: *const WebPPicture,
    ) -> libc::c_int;
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: libc::c_int) -> libc::c_int;
    fn WebPPictureAlloc(picture: *mut WebPPicture) -> libc::c_int;
    fn WebPPictureFree(picture: *mut WebPPicture);
    fn WebPPictureCopy(src: *const WebPPicture, dst: *mut WebPPicture) -> libc::c_int;
    fn WebPPictureView(
        src: *const WebPPicture,
        left: libc::c_int,
        top: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        dst: *mut WebPPicture,
    ) -> libc::c_int;
    fn WebPPictureYUVAToARGB(picture: *mut WebPPicture) -> libc::c_int;
    fn WebPEncode(config: *const WebPConfig, picture: *mut WebPPicture) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;

pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxAnimParams {
    pub bgcolor: uint32_t,
    pub loop_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAnimEncoderOptions {
    pub anim_params: WebPMuxAnimParams,
    pub minimize_size: libc::c_int,
    pub kmin: libc::c_int,
    pub kmax: libc::c_int,
    pub allow_mixed: libc::c_int,
    pub verbose: libc::c_int,
    pub padding: [uint32_t; 4],
}
pub type WebPMuxError = libc::c_int;
pub const WEBP_MUX_NOT_ENOUGH_DATA: WebPMuxError = -4;
pub const WEBP_MUX_MEMORY_ERROR: WebPMuxError = -3;
pub const WEBP_MUX_BAD_DATA: WebPMuxError = -2;
pub const WEBP_MUX_INVALID_ARGUMENT: WebPMuxError = -1;
pub const WEBP_MUX_NOT_FOUND: WebPMuxError = 0;
pub const WEBP_MUX_OK: WebPMuxError = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAnimEncoder {
    pub canvas_width_: libc::c_int,
    pub canvas_height_: libc::c_int,
    pub options_: WebPAnimEncoderOptions,
    pub prev_rect_: FrameRectangle,
    pub last_config_: WebPConfig,
    pub last_config_reversed_: WebPConfig,
    pub curr_canvas_: *mut WebPPicture,
    pub curr_canvas_copy_: WebPPicture,
    pub curr_canvas_copy_modified_: libc::c_int,
    pub prev_canvas_: WebPPicture,
    pub prev_canvas_disposed_: WebPPicture,
    pub encoded_frames_: *mut EncodedFrame,
    pub size_: size_t,
    pub start_: size_t,
    pub count_: size_t,
    pub flush_count_: size_t,
    pub best_delta_: int64_t,
    pub keyframe_: libc::c_int,
    pub count_since_key_frame_: libc::c_int,
    pub first_timestamp_: libc::c_int,
    pub prev_timestamp_: libc::c_int,
    pub prev_candidate_undecided_: libc::c_int,
    pub is_first_frame_: libc::c_int,
    pub got_null_frame_: libc::c_int,
    pub in_frame_count_: size_t,
    pub out_frame_count_: size_t,
    pub mux_: *mut WebPMux,
    pub error_str_: [libc::c_char; 100],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}
pub const CANDIDATE_COUNT: C2RustUnnamed_0 = 4;
pub const LOSSY_DISP_NONE: C2RustUnnamed_0 = 2;
pub const LL_DISP_NONE: C2RustUnnamed_0 = 0;
pub const LOSSY_DISP_BG: C2RustUnnamed_0 = 3;
pub const LL_DISP_BG: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EncodedFrame {
    pub sub_frame_: WebPMuxFrameInfo,
    pub key_frame_: WebPMuxFrameInfo,
    pub is_key_frame_: libc::c_int,
}

pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn WebPDataInit(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        memset(
            webp_data as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<WebPData>() as libc::c_ulong,
        );
    }
}
#[inline]
unsafe extern "C" fn WebPDataClear(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        WebPFree((*webp_data).bytes as *mut libc::c_void);
        WebPDataInit(webp_data);
    }
}
#[inline]
unsafe extern "C" fn WebPDataCopy(
    mut src: *const WebPData,
    mut dst: *mut WebPData,
) -> libc::c_int {
    if src.is_null() || dst.is_null() {
        return 0 as libc::c_int;
    }
    WebPDataInit(dst);
    if !((*src).bytes).is_null() && (*src).size != 0 as libc::c_int as libc::c_ulong {
        (*dst).bytes = WebPMalloc((*src).size) as *mut uint8_t;
        if ((*dst).bytes).is_null() {
            return 0 as libc::c_int;
        }
        memcpy(
            (*dst).bytes as *mut libc::c_void,
            (*src).bytes as *const libc::c_void,
            (*src).size,
        );
        (*dst).size = (*src).size;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPMuxNew() -> *mut WebPMux {
    return WebPNewInternal(0x109 as libc::c_int);
}
#[inline]
unsafe extern "C" fn WebPMuxCreate(
    mut bitstream: *const WebPData,
    mut copy_data: libc::c_int,
) -> *mut WebPMux {
    return WebPMuxCreateInternal(bitstream, copy_data, 0x109 as libc::c_int);
}
#[inline]
unsafe extern "C" fn WebPGetFeatures(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut features: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    return WebPGetFeaturesInternal(data, data_size, features, 0x209 as libc::c_int);
}
#[inline]
unsafe extern "C" fn WebPInitDecoderConfig(
    mut config: *mut WebPDecoderConfig,
) -> libc::c_int {
    return WebPInitDecoderConfigInternal(config, 0x209 as libc::c_int);
}
#[inline]
unsafe extern "C" fn WebPConfigInit(mut config: *mut WebPConfig) -> libc::c_int {
    return WebPConfigInitInternal(
        config,
        WEBP_PRESET_DEFAULT,
        75.0f32,
        0x20f as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> libc::c_int {
    return WebPPictureInitInternal(picture, 0x20f as libc::c_int);
}
unsafe extern "C" fn ResetCounters(enc: *mut WebPAnimEncoder) {
    (*enc).start_ = 0 as libc::c_int as size_t;
    (*enc).count_ = 0 as libc::c_int as size_t;
    (*enc).flush_count_ = 0 as libc::c_int as size_t;
    (*enc).best_delta_ = ((1 as libc::c_ulonglong) << 32 as libc::c_int) as int64_t;
    (*enc).keyframe_ = -(1 as libc::c_int);
}
unsafe extern "C" fn DisableKeyframes(enc_options: *mut WebPAnimEncoderOptions) {
    (*enc_options).kmax = 2147483647 as libc::c_int;
    (*enc_options).kmin = (*enc_options).kmax - 1 as libc::c_int;
}
unsafe extern "C" fn SanitizeEncoderOptions(enc_options: *mut WebPAnimEncoderOptions) {
    let mut print_warning: libc::c_int = (*enc_options).verbose;
    if (*enc_options).minimize_size != 0 {
        DisableKeyframes(enc_options);
    }
    if (*enc_options).kmax == 1 as libc::c_int {
        (*enc_options).kmin = 0 as libc::c_int;
        (*enc_options).kmax = 0 as libc::c_int;
        return;
    } else if (*enc_options).kmax <= 0 as libc::c_int {
        DisableKeyframes(enc_options);
        print_warning = 0 as libc::c_int;
    }
    if (*enc_options).kmin >= (*enc_options).kmax {
        (*enc_options).kmin = (*enc_options).kmax - 1 as libc::c_int;
        if print_warning != 0 {
            eprintln!("WARNING: Setting kmin = {}, so that kmin < kmax.",(*enc_options).kmin);
        }
    } else {
        let kmin_limit: libc::c_int = (*enc_options).kmax / 2 as libc::c_int
            + 1 as libc::c_int;
        if (*enc_options).kmin < kmin_limit && kmin_limit < (*enc_options).kmax {
            (*enc_options).kmin = kmin_limit;
            if print_warning != 0 {
                eprintln!("WARNING: Setting kmin = {}, so that kmin >= kmax / 2 + 1.",(*enc_options).kmin);
            }
        }
    }
    if (*enc_options).kmax - (*enc_options).kmin > 30 as libc::c_int {
        (*enc_options).kmin = (*enc_options).kmax - 30 as libc::c_int;
        if print_warning != 0 {
            eprintln!("WARNING: Setting kmin = {}, so that kmax - kmin <= {}.",(*enc_options).kmin, 30 as libc::c_int);
        }
    }
}
unsafe extern "C" fn DefaultEncoderOptions(enc_options: *mut WebPAnimEncoderOptions) {
    (*enc_options).anim_params.loop_count = 0 as libc::c_int;
    (*enc_options).anim_params.bgcolor = 0xffffffff as libc::c_uint;
    (*enc_options).minimize_size = 0 as libc::c_int;
    DisableKeyframes(enc_options);
    (*enc_options).allow_mixed = 0 as libc::c_int;
    (*enc_options).verbose = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderOptionsInitInternal(
    mut enc_options: *mut WebPAnimEncoderOptions,
    mut abi_version: libc::c_int,
) -> libc::c_int {
    if enc_options.is_null()
        || abi_version >> 8 as libc::c_int != 0x109 as libc::c_int >> 8 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    DefaultEncoderOptions(enc_options);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ClearRectangle(
    picture: *mut WebPPicture,
    mut left: libc::c_int,
    mut top: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = top;
    while j < top + height {
        let dst: *mut uint32_t = ((*picture).argb)
            .offset((j * (*picture).argb_stride) as isize);
        let mut i: libc::c_int = 0;
        i = left;
        while i < left + width {
            *dst.offset(i as isize) = 0 as libc::c_int as uint32_t;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn WebPUtilClearPic(
    picture: *mut WebPPicture,
    rect: *const FrameRectangle,
) {
    if !rect.is_null() {
        ClearRectangle(
            picture,
            (*rect).x_offset_,
            (*rect).y_offset_,
            (*rect).width_,
            (*rect).height_,
        );
    } else {
        ClearRectangle(
            picture,
            0 as libc::c_int,
            0 as libc::c_int,
            (*picture).width,
            (*picture).height,
        );
    };
}
unsafe extern "C" fn MarkNoError(enc: *mut WebPAnimEncoder) {
    (*enc).error_str_[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn MarkError(enc: *mut WebPAnimEncoder, mut str: *const libc::c_char) {
    snprintf(
        ((*enc).error_str_).as_mut_ptr(),
        100 as libc::c_int as libc::c_ulong,
        b"%s.\0" as *const u8 as *const libc::c_char,
        str,
    ) < 0 as libc::c_int;
}
unsafe extern "C" fn MarkError2(
    enc: *mut WebPAnimEncoder,
    mut str: *const libc::c_char,
    mut error_code: libc::c_int,
) {
    snprintf(
        ((*enc).error_str_).as_mut_ptr(),
        100 as libc::c_int as libc::c_ulong,
        b"%s: %d.\0" as *const u8 as *const libc::c_char,
        str,
        error_code,
    ) < 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderNewInternal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut enc_options: *const WebPAnimEncoderOptions,
    mut abi_version: libc::c_int,
) -> *mut WebPAnimEncoder {
    let mut enc: *mut WebPAnimEncoder = 0 as *mut WebPAnimEncoder;
    if abi_version >> 8 as libc::c_int != 0x109 as libc::c_int >> 8 as libc::c_int {
        return 0 as *mut WebPAnimEncoder;
    }
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int
        || (width as libc::c_ulong).wrapping_mul(height as uint64_t) as libc::c_ulonglong
            >= (1 as libc::c_ulonglong) << 32 as libc::c_int
    {
        return 0 as *mut WebPAnimEncoder;
    }
    enc = WebPSafeCalloc(
        1 as libc::c_int as uint64_t,
        ::core::mem::size_of::<WebPAnimEncoder>() as libc::c_ulong,
    ) as *mut WebPAnimEncoder;
    if enc.is_null() {
        return 0 as *mut WebPAnimEncoder;
    }
    MarkNoError(enc);
    (*enc).canvas_width_=width;
    (*enc).canvas_height_=width;
    //*(&(*enc).canvas_width_ as *const libc::c_int as *mut libc::c_int) = width;
    //*(&(*enc).canvas_height_ as *const libc::c_int as *mut libc::c_int) = height;
    if !enc_options.is_null() {
        (*enc).options_=*enc_options;
        //*(&(*enc).options_ as *const WebPAnimEncoderOptions as *mut WebPAnimEncoderOptions) = *enc_options;
        SanitizeEncoderOptions(
            &(*enc).options_ as *const WebPAnimEncoderOptions
                as *mut WebPAnimEncoderOptions,
        );
    } else {
        DefaultEncoderOptions(
            &(*enc).options_ as *const WebPAnimEncoderOptions
                as *mut WebPAnimEncoderOptions,
        );
    }
    if !(WebPPictureInit(&mut (*enc).curr_canvas_copy_) == 0
        || WebPPictureInit(&mut (*enc).prev_canvas_) == 0
        || WebPPictureInit(&mut (*enc).prev_canvas_disposed_) == 0)
    {
        (*enc).curr_canvas_copy_.width = width;
        (*enc).curr_canvas_copy_.height = height;
        (*enc).curr_canvas_copy_.use_argb = 1 as libc::c_int;
        if !(WebPPictureAlloc(&mut (*enc).curr_canvas_copy_) == 0
            || WebPPictureCopy(&mut (*enc).curr_canvas_copy_, &mut (*enc).prev_canvas_)
                == 0
            || WebPPictureCopy(
                &mut (*enc).curr_canvas_copy_,
                &mut (*enc).prev_canvas_disposed_,
            ) == 0)
        {
            WebPUtilClearPic(&mut (*enc).prev_canvas_, 0 as *const FrameRectangle);
            (*enc).curr_canvas_copy_modified_ = 1 as libc::c_int;
            ResetCounters(enc);
            (*enc)
                .size_ = ((*enc).options_.kmax - (*enc).options_.kmin + 1 as libc::c_int)
                as size_t;
            if (*enc).size_ < 2 as libc::c_int as libc::c_ulong {
                (*enc).size_ = 2 as libc::c_int as size_t;
            }
            (*enc)
                .encoded_frames_ = WebPSafeCalloc(
                (*enc).size_,
                ::core::mem::size_of::<EncodedFrame>() as libc::c_ulong,
            ) as *mut EncodedFrame;
            if !((*enc).encoded_frames_).is_null() {
                (*enc).mux_ = WebPMuxNew();
                if !((*enc).mux_).is_null() {
                    (*enc).count_since_key_frame_ = 0 as libc::c_int;
                    (*enc).first_timestamp_ = 0 as libc::c_int;
                    (*enc).prev_timestamp_ = 0 as libc::c_int;
                    (*enc).prev_candidate_undecided_ = 0 as libc::c_int;
                    (*enc).is_first_frame_ = 1 as libc::c_int;
                    (*enc).got_null_frame_ = 0 as libc::c_int;
                    return enc;
                }
            }
        }
    }
    WebPAnimEncoderDelete(enc);
    return 0 as *mut WebPAnimEncoder;
}
unsafe extern "C" fn FrameRelease(encoded_frame: *mut EncodedFrame) {
    if !encoded_frame.is_null() {
        WebPDataClear(&mut (*encoded_frame).sub_frame_.bitstream);
        WebPDataClear(&mut (*encoded_frame).key_frame_.bitstream);
        memset(
            encoded_frame as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<EncodedFrame>() as libc::c_ulong,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderDelete(mut enc: *mut WebPAnimEncoder) {
    if !enc.is_null() {
        WebPPictureFree(&mut (*enc).curr_canvas_copy_);
        WebPPictureFree(&mut (*enc).prev_canvas_);
        WebPPictureFree(&mut (*enc).prev_canvas_disposed_);
        if !((*enc).encoded_frames_).is_null() {
            let mut i: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < (*enc).size_ {
                FrameRelease(&mut *((*enc).encoded_frames_).offset(i as isize));
                i = i.wrapping_add(1);
                i;
            }
            WebPSafeFree((*enc).encoded_frames_ as *mut libc::c_void);
        }
        WebPMuxDelete((*enc).mux_);
        WebPSafeFree(enc as *mut libc::c_void);
    }
}
unsafe extern "C" fn GetFrame(
    enc: *const WebPAnimEncoder,
    mut position: size_t,
) -> *mut EncodedFrame {
    return &mut *((*enc).encoded_frames_)
        .offset(((*enc).start_).wrapping_add(position) as isize) as *mut EncodedFrame;
}
#[inline]
unsafe extern "C" fn ComparePixelsLossless(
    mut src: *const uint32_t,
    mut src_step: libc::c_int,
    mut dst: *const uint32_t,
    mut dst_step: libc::c_int,
    mut length: libc::c_int,
    mut max_allowed_diff: libc::c_int,
) -> libc::c_int {
    loop {
        let fresh0 = length;
        length = length - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        if *src != *dst {
            return 0 as libc::c_int;
        }
        src = src.offset(src_step as isize);
        dst = dst.offset(dst_step as isize);
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn PixelsAreSimilar(
    mut src: uint32_t,
    mut dst: uint32_t,
    mut max_allowed_diff: libc::c_int,
) -> libc::c_int {
    let src_a: libc::c_int = (src >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let src_r: libc::c_int = (src >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let src_g: libc::c_int = (src >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let src_b: libc::c_int = (src >> 0 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let dst_a: libc::c_int = (dst >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let dst_r: libc::c_int = (dst >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let dst_g: libc::c_int = (dst >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    let dst_b: libc::c_int = (dst >> 0 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
    return (src_a == dst_a
        && abs(src_r - dst_r) * dst_a <= max_allowed_diff * 255 as libc::c_int
        && abs(src_g - dst_g) * dst_a <= max_allowed_diff * 255 as libc::c_int
        && abs(src_b - dst_b) * dst_a <= max_allowed_diff * 255 as libc::c_int)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn ComparePixelsLossy(
    mut src: *const uint32_t,
    mut src_step: libc::c_int,
    mut dst: *const uint32_t,
    mut dst_step: libc::c_int,
    mut length: libc::c_int,
    mut max_allowed_diff: libc::c_int,
) -> libc::c_int {
    loop {
        let fresh1 = length;
        length = length - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        if PixelsAreSimilar(*src, *dst, max_allowed_diff) == 0 {
            return 0 as libc::c_int;
        }
        src = src.offset(src_step as isize);
        dst = dst.offset(dst_step as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn IsEmptyRect(rect: *const FrameRectangle) -> libc::c_int {
    return ((*rect).width_ == 0 as libc::c_int || (*rect).height_ == 0 as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn QualityToMaxDiff(mut quality: libc::c_float) -> libc::c_int {
    let val: libc::c_double = pow(quality as libc::c_double / 100.0f64, 0.5f64);
    let max_diff: libc::c_double = 31 as libc::c_int as libc::c_double
        * (1 as libc::c_int as libc::c_double - val)
        + 1 as libc::c_int as libc::c_double * val;
    return (max_diff + 0.5f64) as libc::c_int;
}
unsafe extern "C" fn MinimizeChangeRectangle(
    src: *const WebPPicture,
    dst: *const WebPPicture,
    rect: *mut FrameRectangle,
    mut is_lossless: libc::c_int,
    mut quality: libc::c_float,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let compare_pixels: ComparePixelsFunc = if is_lossless != 0 {
        Some(
            ComparePixelsLossless
                as unsafe extern "C" fn(
                    *const uint32_t,
                    libc::c_int,
                    *const uint32_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        )
    } else {
        Some(
            ComparePixelsLossy
                as unsafe extern "C" fn(
                    *const uint32_t,
                    libc::c_int,
                    *const uint32_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        )
    };
    let max_allowed_diff_lossy: libc::c_int = QualityToMaxDiff(quality);
    let max_allowed_diff: libc::c_int = if is_lossless != 0 {
        0 as libc::c_int
    } else {
        max_allowed_diff_lossy
    };
    i = (*rect).x_offset_;
    while i < (*rect).x_offset_ + (*rect).width_ {
        let src_argb: *const uint32_t = &mut *((*src).argb)
            .offset(((*rect).y_offset_ * (*src).argb_stride + i) as isize)
            as *mut uint32_t;
        let dst_argb: *const uint32_t = &mut *((*dst).argb)
            .offset(((*rect).y_offset_ * (*dst).argb_stride + i) as isize)
            as *mut uint32_t;
        if !(compare_pixels
            .expect(
                "non-null function pointer",
            )(
            src_argb,
            (*src).argb_stride,
            dst_argb,
            (*dst).argb_stride,
            (*rect).height_,
            max_allowed_diff,
        ) != 0)
        {
            break;
        }
        (*rect).width_ -= 1;
        (*rect).width_;
        (*rect).x_offset_ += 1;
        (*rect).x_offset_;
        i += 1;
        i;
    }
    if (*rect).width_ == 0 as libc::c_int {
        current_block = 14664345972399101461;
    } else {
        i = (*rect).x_offset_ + (*rect).width_ - 1 as libc::c_int;
        while i >= (*rect).x_offset_ {
            let src_argb_0: *const uint32_t = &mut *((*src).argb)
                .offset(((*rect).y_offset_ * (*src).argb_stride + i) as isize)
                as *mut uint32_t;
            let dst_argb_0: *const uint32_t = &mut *((*dst).argb)
                .offset(((*rect).y_offset_ * (*dst).argb_stride + i) as isize)
                as *mut uint32_t;
            if !(compare_pixels
                .expect(
                    "non-null function pointer",
                )(
                src_argb_0,
                (*src).argb_stride,
                dst_argb_0,
                (*dst).argb_stride,
                (*rect).height_,
                max_allowed_diff,
            ) != 0)
            {
                break;
            }
            (*rect).width_ -= 1;
            (*rect).width_;
            i -= 1;
            i;
        }
        if (*rect).width_ == 0 as libc::c_int {
            current_block = 14664345972399101461;
        } else {
            j = (*rect).y_offset_;
            while j < (*rect).y_offset_ + (*rect).height_ {
                let src_argb_1: *const uint32_t = &mut *((*src).argb)
                    .offset((j * (*src).argb_stride + (*rect).x_offset_) as isize)
                    as *mut uint32_t;
                let dst_argb_1: *const uint32_t = &mut *((*dst).argb)
                    .offset((j * (*dst).argb_stride + (*rect).x_offset_) as isize)
                    as *mut uint32_t;
                if !(compare_pixels
                    .expect(
                        "non-null function pointer",
                    )(
                    src_argb_1,
                    1 as libc::c_int,
                    dst_argb_1,
                    1 as libc::c_int,
                    (*rect).width_,
                    max_allowed_diff,
                ) != 0)
                {
                    break;
                }
                (*rect).height_ -= 1;
                (*rect).height_;
                (*rect).y_offset_ += 1;
                (*rect).y_offset_;
                j += 1;
                j;
            }
            if (*rect).height_ == 0 as libc::c_int {
                current_block = 14664345972399101461;
            } else {
                j = (*rect).y_offset_ + (*rect).height_ - 1 as libc::c_int;
                while j >= (*rect).y_offset_ {
                    let src_argb_2: *const uint32_t = &mut *((*src).argb)
                        .offset((j * (*src).argb_stride + (*rect).x_offset_) as isize)
                        as *mut uint32_t;
                    let dst_argb_2: *const uint32_t = &mut *((*dst).argb)
                        .offset((j * (*dst).argb_stride + (*rect).x_offset_) as isize)
                        as *mut uint32_t;
                    if !(compare_pixels
                        .expect(
                            "non-null function pointer",
                        )(
                        src_argb_2,
                        1 as libc::c_int,
                        dst_argb_2,
                        1 as libc::c_int,
                        (*rect).width_,
                        max_allowed_diff,
                    ) != 0)
                    {
                        break;
                    }
                    (*rect).height_ -= 1;
                    (*rect).height_;
                    j -= 1;
                    j;
                }
                if (*rect).height_ == 0 as libc::c_int {
                    current_block = 14664345972399101461;
                } else if IsEmptyRect(rect) != 0 {
                    current_block = 14664345972399101461;
                } else {
                    current_block = 14072441030219150333;
                }
            }
        }
    }
    match current_block {
        14664345972399101461 => {
            (*rect).x_offset_ = 0 as libc::c_int;
            (*rect).y_offset_ = 0 as libc::c_int;
            (*rect).width_ = 0 as libc::c_int;
            (*rect).height_ = 0 as libc::c_int;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn SnapToEvenOffsets(rect: *mut FrameRectangle) {
    (*rect).width_ += (*rect).x_offset_ & 1 as libc::c_int;
    (*rect).height_ += (*rect).y_offset_ & 1 as libc::c_int;
    (*rect).x_offset_ &= !(1 as libc::c_int);
    (*rect).y_offset_ &= !(1 as libc::c_int);
}
unsafe extern "C" fn SubFrameParamsInit(
    params: *mut SubFrameParams,
    mut should_try: libc::c_int,
    mut empty_rect_allowed: libc::c_int,
) -> libc::c_int {
    (*params).should_try_ = should_try;
    (*params).empty_rect_allowed_ = empty_rect_allowed;
    if WebPPictureInit(&mut (*params).sub_frame_ll_) == 0
        || WebPPictureInit(&mut (*params).sub_frame_lossy_) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn SubFrameParamsFree(params: *mut SubFrameParams) {
    WebPPictureFree(&mut (*params).sub_frame_ll_);
    WebPPictureFree(&mut (*params).sub_frame_lossy_);
}
unsafe extern "C" fn GetSubRect(
    prev_canvas: *const WebPPicture,
    curr_canvas: *const WebPPicture,
    mut is_key_frame: libc::c_int,
    mut is_first_frame: libc::c_int,
    mut empty_rect_allowed: libc::c_int,
    mut is_lossless: libc::c_int,
    mut quality: libc::c_float,
    rect: *mut FrameRectangle,
    sub_frame: *mut WebPPicture,
) -> libc::c_int {
    if is_key_frame == 0 || is_first_frame != 0 {
        MinimizeChangeRectangle(prev_canvas, curr_canvas, rect, is_lossless, quality);
    }
    if IsEmptyRect(rect) != 0 {
        if empty_rect_allowed != 0 {
            return 1 as libc::c_int
        } else {
            (*rect).width_ = 1 as libc::c_int;
            (*rect).height_ = 1 as libc::c_int;
        }
    }
    SnapToEvenOffsets(rect);
    return WebPPictureView(
        curr_canvas,
        (*rect).x_offset_,
        (*rect).y_offset_,
        (*rect).width_,
        (*rect).height_,
        sub_frame,
    );
}
unsafe extern "C" fn GetSubRects(
    prev_canvas: *const WebPPicture,
    curr_canvas: *const WebPPicture,
    mut is_key_frame: libc::c_int,
    mut is_first_frame: libc::c_int,
    mut quality: libc::c_float,
    params: *mut SubFrameParams,
) -> libc::c_int {
    (*params).rect_ll_.x_offset_ = 0 as libc::c_int;
    (*params).rect_ll_.y_offset_ = 0 as libc::c_int;
    (*params).rect_ll_.width_ = (*curr_canvas).width;
    (*params).rect_ll_.height_ = (*curr_canvas).height;
    if GetSubRect(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        (*params).empty_rect_allowed_,
        1 as libc::c_int,
        quality,
        &mut (*params).rect_ll_,
        &mut (*params).sub_frame_ll_,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    (*params).rect_lossy_ = (*params).rect_ll_;
    return GetSubRect(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        (*params).empty_rect_allowed_,
        0 as libc::c_int,
        quality,
        &mut (*params).rect_lossy_,
        &mut (*params).sub_frame_lossy_,
    );
}
#[inline]
unsafe extern "C" fn clip(
    mut v: libc::c_int,
    mut min_v: libc::c_int,
    mut max_v: libc::c_int,
) -> libc::c_int {
    return if v < min_v { min_v } else if v > max_v { max_v } else { v };
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderRefineRect(
    prev_canvas: *const WebPPicture,
    curr_canvas: *const WebPPicture,
    mut is_lossless: libc::c_int,
    mut quality: libc::c_float,
    x_offset: *mut libc::c_int,
    y_offset: *mut libc::c_int,
    width: *mut libc::c_int,
    height: *mut libc::c_int,
) -> libc::c_int {
    let mut rect: FrameRectangle = FrameRectangle {
        x_offset_: 0,
        y_offset_: 0,
        width_: 0,
        height_: 0,
    };
    let mut right: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    if prev_canvas.is_null() || curr_canvas.is_null()
        || (*prev_canvas).width != (*curr_canvas).width
        || (*prev_canvas).height != (*curr_canvas).height || (*prev_canvas).use_argb == 0
        || (*curr_canvas).use_argb == 0
    {
        return 0 as libc::c_int;
    }
    right = clip(*x_offset + *width, 0 as libc::c_int, (*curr_canvas).width);
    left = clip(*x_offset, 0 as libc::c_int, (*curr_canvas).width - 1 as libc::c_int);
    bottom = clip(*y_offset + *height, 0 as libc::c_int, (*curr_canvas).height);
    top = clip(*y_offset, 0 as libc::c_int, (*curr_canvas).height - 1 as libc::c_int);
    rect.x_offset_ = left;
    rect.y_offset_ = top;
    rect
        .width_ = clip(
        right - left,
        0 as libc::c_int,
        (*curr_canvas).width - rect.x_offset_,
    );
    rect
        .height_ = clip(
        bottom - top,
        0 as libc::c_int,
        (*curr_canvas).height - rect.y_offset_,
    );
    MinimizeChangeRectangle(prev_canvas, curr_canvas, &mut rect, is_lossless, quality);
    SnapToEvenOffsets(&mut rect);
    *x_offset = rect.x_offset_;
    *y_offset = rect.y_offset_;
    *width = rect.width_;
    *height = rect.height_;
    return 1 as libc::c_int;
}
unsafe extern "C" fn DisposeFrameRectangle(
    mut dispose_method: libc::c_int,
    rect: *const FrameRectangle,
    curr_canvas: *mut WebPPicture,
) {
    if dispose_method == WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int {
        WebPUtilClearPic(curr_canvas, rect);
    }
}
unsafe extern "C" fn RectArea(rect: *const FrameRectangle) -> uint32_t {
    return ((*rect).width_ as uint32_t).wrapping_mul((*rect).height_ as libc::c_uint);
}
unsafe extern "C" fn IsLosslessBlendingPossible(
    src: *const WebPPicture,
    dst: *const WebPPicture,
    rect: *const FrameRectangle,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = (*rect).y_offset_;
    while j < (*rect).y_offset_ + (*rect).height_ {
        i = (*rect).x_offset_;
        while i < (*rect).x_offset_ + (*rect).width_ {
            let src_pixel: uint32_t = *((*src).argb)
                .offset((j * (*src).argb_stride + i) as isize);
            let dst_pixel: uint32_t = *((*dst).argb)
                .offset((j * (*dst).argb_stride + i) as isize);
            let dst_alpha: uint32_t = dst_pixel >> 24 as libc::c_int;
            if dst_alpha != 0xff as libc::c_int as libc::c_uint && src_pixel != dst_pixel
            {
                return 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn IsLossyBlendingPossible(
    src: *const WebPPicture,
    dst: *const WebPPicture,
    rect: *const FrameRectangle,
    mut quality: libc::c_float,
) -> libc::c_int {
    let max_allowed_diff_lossy: libc::c_int = QualityToMaxDiff(quality);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = (*rect).y_offset_;
    while j < (*rect).y_offset_ + (*rect).height_ {
        i = (*rect).x_offset_;
        while i < (*rect).x_offset_ + (*rect).width_ {
            let src_pixel: uint32_t = *((*src).argb)
                .offset((j * (*src).argb_stride + i) as isize);
            let dst_pixel: uint32_t = *((*dst).argb)
                .offset((j * (*dst).argb_stride + i) as isize);
            let dst_alpha: uint32_t = dst_pixel >> 24 as libc::c_int;
            if dst_alpha != 0xff as libc::c_int as libc::c_uint
                && PixelsAreSimilar(src_pixel, dst_pixel, max_allowed_diff_lossy) == 0
            {
                return 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn IncreaseTransparency(
    src: *const WebPPicture,
    rect: *const FrameRectangle,
    dst: *mut WebPPicture,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut modified: libc::c_int = 0 as libc::c_int;
    j = (*rect).y_offset_;
    while j < (*rect).y_offset_ + (*rect).height_ {
        let psrc: *const uint32_t = ((*src).argb)
            .offset((j * (*src).argb_stride) as isize);
        let pdst: *mut uint32_t = ((*dst).argb)
            .offset((j * (*dst).argb_stride) as isize);
        i = (*rect).x_offset_;
        while i < (*rect).x_offset_ + (*rect).width_ {
            if *psrc.offset(i as isize) == *pdst.offset(i as isize)
                && *pdst.offset(i as isize) != 0 as libc::c_int as libc::c_uint
            {
                *pdst.offset(i as isize) = 0 as libc::c_int as uint32_t;
                modified = 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return modified;
}
unsafe extern "C" fn FlattenSimilarBlocks(
    src: *const WebPPicture,
    rect: *const FrameRectangle,
    dst: *mut WebPPicture,
    mut quality: libc::c_float,
) -> libc::c_int {
    let max_allowed_diff_lossy: libc::c_int = QualityToMaxDiff(quality);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut modified: libc::c_int = 0 as libc::c_int;
    let block_size: libc::c_int = 8 as libc::c_int;
    let y_start: libc::c_int = (*rect).y_offset_ + block_size
        & !(block_size - 1 as libc::c_int);
    let y_end: libc::c_int = (*rect).y_offset_ + (*rect).height_
        & !(block_size - 1 as libc::c_int);
    let x_start: libc::c_int = (*rect).x_offset_ + block_size
        & !(block_size - 1 as libc::c_int);
    let x_end: libc::c_int = (*rect).x_offset_ + (*rect).width_
        & !(block_size - 1 as libc::c_int);
    j = y_start;
    while j < y_end {
        i = x_start;
        while i < x_end {
            let mut cnt: libc::c_int = 0 as libc::c_int;
            let mut avg_r: libc::c_int = 0 as libc::c_int;
            let mut avg_g: libc::c_int = 0 as libc::c_int;
            let mut avg_b: libc::c_int = 0 as libc::c_int;
            let mut x: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let psrc: *const uint32_t = ((*src).argb)
                .offset((j * (*src).argb_stride) as isize)
                .offset(i as isize);
            let pdst: *mut uint32_t = ((*dst).argb)
                .offset((j * (*dst).argb_stride) as isize)
                .offset(i as isize);
            y = 0 as libc::c_int;
            while y < block_size {
                x = 0 as libc::c_int;
                while x < block_size {
                    let src_pixel: uint32_t = *psrc
                        .offset((x + y * (*src).argb_stride) as isize);
                    let alpha: libc::c_int = (src_pixel >> 24 as libc::c_int)
                        as libc::c_int;
                    if alpha == 0xff as libc::c_int
                        && PixelsAreSimilar(
                            src_pixel,
                            *pdst.offset((x + y * (*dst).argb_stride) as isize),
                            max_allowed_diff_lossy,
                        ) != 0
                    {
                        cnt += 1;
                        cnt;
                        avg_r = (avg_r as libc::c_uint)
                            .wrapping_add(
                                src_pixel >> 16 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_uint,
                            ) as libc::c_int as libc::c_int;
                        avg_g = (avg_g as libc::c_uint)
                            .wrapping_add(
                                src_pixel >> 8 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_uint,
                            ) as libc::c_int as libc::c_int;
                        avg_b = (avg_b as libc::c_uint)
                            .wrapping_add(
                                src_pixel >> 0 as libc::c_int
                                    & 0xff as libc::c_int as libc::c_uint,
                            ) as libc::c_int as libc::c_int;
                    }
                    x += 1;
                    x;
                }
                y += 1;
                y;
            }
            if cnt == block_size * block_size {
                let color: uint32_t = ((0 as libc::c_int) << 24 as libc::c_int
                    | avg_r / cnt << 16 as libc::c_int | avg_g / cnt << 8 as libc::c_int
                    | avg_b / cnt << 0 as libc::c_int) as uint32_t;
                y = 0 as libc::c_int;
                while y < block_size {
                    x = 0 as libc::c_int;
                    while x < block_size {
                        *pdst.offset((x + y * (*dst).argb_stride) as isize) = color;
                        x += 1;
                        x;
                    }
                    y += 1;
                    y;
                }
                modified = 1 as libc::c_int;
            }
            i += block_size;
        }
        j += block_size;
    }
    return modified;
}
unsafe extern "C" fn EncodeFrame(
    config: *const WebPConfig,
    pic: *mut WebPPicture,
    memory: *mut WebPMemoryWriter,
) -> libc::c_int {
    (*pic).use_argb = 1 as libc::c_int;
    (*pic)
        .writer = Some(
        WebPMemoryWrite
            as unsafe extern "C" fn(
                *const uint8_t,
                size_t,
                *const WebPPicture,
            ) -> libc::c_int,
    );
    (*pic).custom_ptr = memory as *mut libc::c_void;
    if WebPEncode(config, pic) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn EncodeCandidate(
    sub_frame: *mut WebPPicture,
    rect: *const FrameRectangle,
    encoder_config: *const WebPConfig,
    mut use_blending: libc::c_int,
    candidate: *mut Candidate,
) -> WebPEncodingError {
    let mut config: WebPConfig = *encoder_config;
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    memset(
        candidate as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Candidate>() as libc::c_ulong,
    );
    (*candidate).rect_ = *rect;
    (*candidate).info_.id = WEBP_CHUNK_ANMF;
    (*candidate).info_.x_offset = (*rect).x_offset_;
    (*candidate).info_.y_offset = (*rect).y_offset_;
    (*candidate).info_.dispose_method = WEBP_MUX_DISPOSE_NONE;
    (*candidate)
        .info_
        .blend_method = (if use_blending != 0 {
        WEBP_MUX_BLEND as libc::c_int
    } else {
        WEBP_MUX_NO_BLEND as libc::c_int
    }) as WebPMuxAnimBlend;
    (*candidate).info_.duration = 0 as libc::c_int;
    WebPMemoryWriterInit(&mut (*candidate).mem_);
    if config.lossless == 0 && use_blending != 0 {
        config.autofilter = 0 as libc::c_int;
        config.filter_strength = 0 as libc::c_int;
    }
    if EncodeFrame(&mut config, sub_frame, &mut (*candidate).mem_) == 0 {
        error_code = (*sub_frame).error_code;
        WebPMemoryWriterClear(&mut (*candidate).mem_);
        return error_code;
    } else {
        (*candidate).evaluate_ = 1 as libc::c_int;
        return error_code;
    };
}
unsafe extern "C" fn CopyCurrentCanvas(enc: *mut WebPAnimEncoder) {
    if (*enc).curr_canvas_copy_modified_ != 0 {
        WebPCopyPixels((*enc).curr_canvas_, &mut (*enc).curr_canvas_copy_);
        (*enc).curr_canvas_copy_.progress_hook = (*(*enc).curr_canvas_).progress_hook;
        (*enc).curr_canvas_copy_.user_data = (*(*enc).curr_canvas_).user_data;
        (*enc).curr_canvas_copy_modified_ = 0 as libc::c_int;
    }
}
unsafe extern "C" fn GenerateCandidates(
    enc: *mut WebPAnimEncoder,
    mut candidates: *mut Candidate,
    mut dispose_method: WebPMuxAnimDispose,
    mut is_lossless: libc::c_int,
    mut is_key_frame: libc::c_int,
    params: *mut SubFrameParams,
    config_ll: *const WebPConfig,
    config_lossy: *const WebPConfig,
) -> WebPEncodingError {
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    let is_dispose_none: libc::c_int = (dispose_method as libc::c_uint
        == WEBP_MUX_DISPOSE_NONE as libc::c_int as libc::c_uint) as libc::c_int;
    let candidate_ll: *mut Candidate = if is_dispose_none != 0 {
        &mut *candidates.offset(LL_DISP_NONE as libc::c_int as isize) as *mut Candidate
    } else {
        &mut *candidates.offset(LL_DISP_BG as libc::c_int as isize) as *mut Candidate
    };
    let candidate_lossy: *mut Candidate = if is_dispose_none != 0 {
        &mut *candidates.offset(LOSSY_DISP_NONE as libc::c_int as isize)
            as *mut Candidate
    } else {
        &mut *candidates.offset(LOSSY_DISP_BG as libc::c_int as isize) as *mut Candidate
    };
    let curr_canvas: *mut WebPPicture = &mut (*enc).curr_canvas_copy_;
    let prev_canvas: *const WebPPicture = if is_dispose_none != 0 {
        &mut (*enc).prev_canvas_
    } else {
        &mut (*enc).prev_canvas_disposed_
    };
    let mut use_blending_ll: libc::c_int = 0;
    let mut use_blending_lossy: libc::c_int = 0;
    let mut evaluate_ll: libc::c_int = 0;
    let mut evaluate_lossy: libc::c_int = 0;
    CopyCurrentCanvas(enc);
    use_blending_ll = (is_key_frame == 0
        && IsLosslessBlendingPossible(prev_canvas, curr_canvas, &mut (*params).rect_ll_)
            != 0) as libc::c_int;
    use_blending_lossy = (is_key_frame == 0
        && IsLossyBlendingPossible(
            prev_canvas,
            curr_canvas,
            &mut (*params).rect_lossy_,
            (*config_lossy).quality,
        ) != 0) as libc::c_int;
    if (*enc).options_.allow_mixed == 0 {
        evaluate_ll = is_lossless;
        evaluate_lossy = (is_lossless == 0) as libc::c_int;
    } else if (*enc).options_.minimize_size != 0 {
        evaluate_ll = 1 as libc::c_int;
        evaluate_lossy = 1 as libc::c_int;
    } else {
        let num_colors: libc::c_int = WebPGetColorPalette(
            &mut (*params).sub_frame_ll_,
            0 as *mut uint32_t,
        );
        evaluate_ll = (num_colors < 194 as libc::c_int) as libc::c_int;
        evaluate_lossy = (num_colors >= 31 as libc::c_int) as libc::c_int;
    }
    if evaluate_ll != 0 {
        CopyCurrentCanvas(enc);
        if use_blending_ll != 0 {
            (*enc)
                .curr_canvas_copy_modified_ = IncreaseTransparency(
                prev_canvas,
                &mut (*params).rect_ll_,
                curr_canvas,
            );
        }
        error_code = EncodeCandidate(
            &mut (*params).sub_frame_ll_,
            &mut (*params).rect_ll_,
            config_ll,
            use_blending_ll,
            candidate_ll,
        );
        if error_code as libc::c_uint != VP8_ENC_OK as libc::c_int as libc::c_uint {
            return error_code;
        }
    }
    if evaluate_lossy != 0 {
        CopyCurrentCanvas(enc);
        if use_blending_lossy != 0 {
            (*enc)
                .curr_canvas_copy_modified_ = FlattenSimilarBlocks(
                prev_canvas,
                &mut (*params).rect_lossy_,
                curr_canvas,
                (*config_lossy).quality,
            );
        }
        error_code = EncodeCandidate(
            &mut (*params).sub_frame_lossy_,
            &mut (*params).rect_lossy_,
            config_lossy,
            use_blending_lossy,
            candidate_lossy,
        );
        if error_code as libc::c_uint != VP8_ENC_OK as libc::c_int as libc::c_uint {
            return error_code;
        }
        (*enc).curr_canvas_copy_modified_ = 1 as libc::c_int;
    }
    return error_code;
}
unsafe extern "C" fn GetEncodedData(
    memory: *const WebPMemoryWriter,
    encoded_data: *mut WebPData,
) {
    (*encoded_data).bytes = (*memory).mem;
    (*encoded_data).size = (*memory).size;
}
unsafe extern "C" fn SetPreviousDisposeMethod(
    enc: *mut WebPAnimEncoder,
    mut dispose_method: WebPMuxAnimDispose,
) {
    let position: size_t = ((*enc).count_)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong);
    let prev_enc_frame: *mut EncodedFrame = GetFrame(enc, position);
    if (*enc).prev_candidate_undecided_ != 0 {
        (*prev_enc_frame).sub_frame_.dispose_method = dispose_method;
        (*prev_enc_frame).key_frame_.dispose_method = dispose_method;
    } else {
        let prev_info: *mut WebPMuxFrameInfo = if (*prev_enc_frame).is_key_frame_ != 0 {
            &mut (*prev_enc_frame).key_frame_
        } else {
            &mut (*prev_enc_frame).sub_frame_
        };
        (*prev_info).dispose_method = dispose_method;
    };
}
unsafe extern "C" fn IncreasePreviousDuration(
    enc: *mut WebPAnimEncoder,
    mut duration: libc::c_int,
) -> libc::c_int {
    let position: size_t = ((*enc).count_)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let prev_enc_frame: *mut EncodedFrame = GetFrame(enc, position);
    let mut new_duration: libc::c_int = 0;
    new_duration = (*prev_enc_frame).sub_frame_.duration + duration;
    if new_duration >= (1 as libc::c_int) << 24 as libc::c_int {
        let rect: FrameRectangle = {
            let mut init = FrameRectangle {
                x_offset_: 0 as libc::c_int,
                y_offset_: 0 as libc::c_int,
                width_: 1 as libc::c_int,
                height_: 1 as libc::c_int,
            };
            init
        };
        let lossless_1x1_bytes: [uint8_t; 28] = [
            0x52 as libc::c_int as uint8_t,
            0x49 as libc::c_int as uint8_t,
            0x46 as libc::c_int as uint8_t,
            0x46 as libc::c_int as uint8_t,
            0x14 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x57 as libc::c_int as uint8_t,
            0x45 as libc::c_int as uint8_t,
            0x42 as libc::c_int as uint8_t,
            0x50 as libc::c_int as uint8_t,
            0x56 as libc::c_int as uint8_t,
            0x50 as libc::c_int as uint8_t,
            0x38 as libc::c_int as uint8_t,
            0x4c as libc::c_int as uint8_t,
            0x8 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x2f as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x10 as libc::c_int as uint8_t,
            0x88 as libc::c_int as uint8_t,
            0x88 as libc::c_int as uint8_t,
            0x8 as libc::c_int as uint8_t,
        ];
        let lossless_1x1: WebPData = {
            let mut init = WebPData {
                bytes: lossless_1x1_bytes.as_ptr(),
                size: ::core::mem::size_of::<[uint8_t; 28]>() as libc::c_ulong,
            };
            init
        };
        let lossy_1x1_bytes: [uint8_t; 72] = [
            0x52 as libc::c_int as uint8_t,
            0x49 as libc::c_int as uint8_t,
            0x46 as libc::c_int as uint8_t,
            0x46 as libc::c_int as uint8_t,
            0x40 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x57 as libc::c_int as uint8_t,
            0x45 as libc::c_int as uint8_t,
            0x42 as libc::c_int as uint8_t,
            0x50 as libc::c_int as uint8_t,
            0x56 as libc::c_int as uint8_t,
            0x50 as libc::c_int as uint8_t,
            0x38 as libc::c_int as uint8_t,
            0x58 as libc::c_int as uint8_t,
            0xa as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x10 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x41 as libc::c_int as uint8_t,
            0x4c as libc::c_int as uint8_t,
            0x50 as libc::c_int as uint8_t,
            0x48 as libc::c_int as uint8_t,
            0x2 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x56 as libc::c_int as uint8_t,
            0x50 as libc::c_int as uint8_t,
            0x38 as libc::c_int as uint8_t,
            0x20 as libc::c_int as uint8_t,
            0x18 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x30 as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x9d as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
            0x2a as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x1 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x2 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x34 as libc::c_int as uint8_t,
            0x25 as libc::c_int as uint8_t,
            0xa4 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x3 as libc::c_int as uint8_t,
            0x70 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0xfe as libc::c_int as uint8_t,
            0xfb as libc::c_int as uint8_t,
            0xfd as libc::c_int as uint8_t,
            0x50 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ];
        let lossy_1x1: WebPData = {
            let mut init = WebPData {
                bytes: lossy_1x1_bytes.as_ptr(),
                size: ::core::mem::size_of::<[uint8_t; 72]>() as libc::c_ulong,
            };
            init
        };
        let can_use_lossless: libc::c_int = ((*enc).last_config_.lossless != 0
            || (*enc).options_.allow_mixed != 0) as libc::c_int;
        let curr_enc_frame: *mut EncodedFrame = GetFrame(enc, (*enc).count_);
        (*curr_enc_frame).is_key_frame_ = 0 as libc::c_int;
        (*curr_enc_frame).sub_frame_.id = WEBP_CHUNK_ANMF;
        (*curr_enc_frame).sub_frame_.x_offset = 0 as libc::c_int;
        (*curr_enc_frame).sub_frame_.y_offset = 0 as libc::c_int;
        (*curr_enc_frame).sub_frame_.dispose_method = WEBP_MUX_DISPOSE_NONE;
        (*curr_enc_frame).sub_frame_.blend_method = WEBP_MUX_BLEND;
        (*curr_enc_frame).sub_frame_.duration = duration;
        if WebPDataCopy(
            if can_use_lossless != 0 { &lossless_1x1 } else { &lossy_1x1 },
            &mut (*curr_enc_frame).sub_frame_.bitstream,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        (*enc).count_ = ((*enc).count_).wrapping_add(1);
        (*enc).count_;
        (*enc).count_since_key_frame_ += 1;
        (*enc).count_since_key_frame_;
        (*enc)
            .flush_count_ = ((*enc).count_)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        (*enc).prev_candidate_undecided_ = 0 as libc::c_int;
        (*enc).prev_rect_ = rect;
    } else {
        (*prev_enc_frame).sub_frame_.duration = new_duration;
        (*prev_enc_frame).key_frame_.duration = new_duration;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn PickBestCandidate(
    enc: *mut WebPAnimEncoder,
    candidates: *mut Candidate,
    mut is_key_frame: libc::c_int,
    encoded_frame: *mut EncodedFrame,
) {
    let mut i: libc::c_int = 0;
    let mut best_idx: libc::c_int = -(1 as libc::c_int);
    let mut best_size: size_t = !(0 as libc::c_int) as size_t;
    i = 0 as libc::c_int;
    while i < CANDIDATE_COUNT as libc::c_int {
        if (*candidates.offset(i as isize)).evaluate_ != 0 {
            let candidate_size: size_t = (*candidates.offset(i as isize)).mem_.size;
            if candidate_size < best_size {
                best_idx = i;
                best_size = candidate_size;
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < CANDIDATE_COUNT as libc::c_int {
        if (*candidates.offset(i as isize)).evaluate_ != 0 {
            if i == best_idx {
                let dst: *mut WebPMuxFrameInfo = if is_key_frame != 0 {
                    &mut (*encoded_frame).key_frame_
                } else {
                    &mut (*encoded_frame).sub_frame_
                };
                *dst = (*candidates.offset(i as isize)).info_;
                GetEncodedData(
                    &mut (*candidates.offset(i as isize)).mem_,
                    &mut (*dst).bitstream,
                );
                if is_key_frame == 0 {
                    let prev_dispose_method: WebPMuxAnimDispose = (if best_idx
                        == LL_DISP_NONE as libc::c_int
                        || best_idx == LOSSY_DISP_NONE as libc::c_int
                    {
                        WEBP_MUX_DISPOSE_NONE as libc::c_int
                    } else {
                        WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int
                    }) as WebPMuxAnimDispose;
                    SetPreviousDisposeMethod(enc, prev_dispose_method);
                }
                (*enc).prev_rect_ = (*candidates.offset(i as isize)).rect_;
            } else {
                WebPMemoryWriterClear(&mut (*candidates.offset(i as isize)).mem_);
                (*candidates.offset(i as isize)).evaluate_ = 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn SetFrame(
    enc: *mut WebPAnimEncoder,
    config: *const WebPConfig,
    mut is_key_frame: libc::c_int,
    encoded_frame: *mut EncodedFrame,
    frame_skipped: *mut libc::c_int,
) -> WebPEncodingError {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    let curr_canvas: *const WebPPicture = &mut (*enc).curr_canvas_copy_;
    let prev_canvas: *const WebPPicture = &mut (*enc).prev_canvas_;
    let mut candidates: [Candidate; 4] = [Candidate {
        mem_: WebPMemoryWriter {
            mem: 0 as *mut uint8_t,
            size: 0,
            max_size: 0,
            pad: [0; 1],
        },
        info_: WebPMuxFrameInfo {
            bitstream: WebPData {
                bytes: 0 as *const uint8_t,
                size: 0,
            },
            x_offset: 0,
            y_offset: 0,
            duration: 0,
            id: WEBP_CHUNK_VP8X,
            dispose_method: WEBP_MUX_DISPOSE_NONE,
            blend_method: WEBP_MUX_BLEND,
            pad: [0; 1],
        },
        rect_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        evaluate_: 0,
    }; 4];
    let is_lossless: libc::c_int = (*config).lossless;
    let consider_lossless: libc::c_int = (is_lossless != 0
        || (*enc).options_.allow_mixed != 0) as libc::c_int;
    let consider_lossy: libc::c_int = (is_lossless == 0
        || (*enc).options_.allow_mixed != 0) as libc::c_int;
    let is_first_frame: libc::c_int = (*enc).is_first_frame_;
    let empty_rect_allowed_none: libc::c_int = (is_first_frame == 0) as libc::c_int;
    let empty_rect_allowed_bg: libc::c_int = 0 as libc::c_int;
    let dispose_bg_possible: libc::c_int = (is_key_frame == 0
        && (*enc).prev_candidate_undecided_ == 0) as libc::c_int;
    let mut dispose_none_params: SubFrameParams = SubFrameParams {
        should_try_: 0,
        empty_rect_allowed_: 0,
        rect_ll_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_ll_: WebPPicture {
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
        },
        rect_lossy_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_lossy_: WebPPicture {
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
        },
    };
    let mut dispose_bg_params: SubFrameParams = SubFrameParams {
        should_try_: 0,
        empty_rect_allowed_: 0,
        rect_ll_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_ll_: WebPPicture {
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
        },
        rect_lossy_: FrameRectangle {
            x_offset_: 0,
            y_offset_: 0,
            width_: 0,
            height_: 0,
        },
        sub_frame_lossy_: WebPPicture {
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
        },
    };
    let mut config_ll: WebPConfig = *config;
    let mut config_lossy: WebPConfig = *config;
    config_ll.lossless = 1 as libc::c_int;
    config_lossy.lossless = 0 as libc::c_int;
    (*enc).last_config_ = *config;
    (*enc)
        .last_config_reversed_ = if (*config).lossless != 0 {
        config_lossy
    } else {
        config_ll
    };
    *frame_skipped = 0 as libc::c_int;
    if SubFrameParamsInit(
        &mut dispose_none_params,
        1 as libc::c_int,
        empty_rect_allowed_none,
    ) == 0
        || SubFrameParamsInit(
            &mut dispose_bg_params,
            0 as libc::c_int,
            empty_rect_allowed_bg,
        ) == 0
    {
        return VP8_ENC_ERROR_INVALID_CONFIGURATION;
    }
    memset(
        candidates.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[Candidate; 4]>() as libc::c_ulong,
    );
    if GetSubRects(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        config_lossy.quality,
        &mut dispose_none_params,
    ) == 0
    {
        error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
        current_block = 9435687255277069653;
    } else if consider_lossless != 0
        && IsEmptyRect(&mut dispose_none_params.rect_ll_) != 0
        || consider_lossy != 0 && IsEmptyRect(&mut dispose_none_params.rect_lossy_) != 0
    {
        *frame_skipped = 1 as libc::c_int;
        current_block = 17243318549450349268;
    } else {
        if dispose_bg_possible != 0 {
            let prev_canvas_disposed: *mut WebPPicture = &mut (*enc)
                .prev_canvas_disposed_;
            WebPCopyPixels(prev_canvas, prev_canvas_disposed);
            DisposeFrameRectangle(
                WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int,
                &mut (*enc).prev_rect_,
                prev_canvas_disposed,
            );
            if GetSubRects(
                prev_canvas_disposed,
                curr_canvas,
                is_key_frame,
                is_first_frame,
                config_lossy.quality,
                &mut dispose_bg_params,
            ) == 0
            {
                error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
                current_block = 9435687255277069653;
            } else {
                if (*enc).options_.minimize_size != 0 {
                    dispose_bg_params.should_try_ = 1 as libc::c_int;
                    dispose_none_params.should_try_ = 1 as libc::c_int;
                } else if is_lossless != 0
                    && RectArea(&mut dispose_bg_params.rect_ll_)
                        < RectArea(&mut dispose_none_params.rect_ll_)
                    || is_lossless == 0
                        && RectArea(&mut dispose_bg_params.rect_lossy_)
                            < RectArea(&mut dispose_none_params.rect_lossy_)
                {
                    dispose_bg_params.should_try_ = 1 as libc::c_int;
                    dispose_none_params.should_try_ = 0 as libc::c_int;
                }
                current_block = 14359455889292382949;
            }
        } else {
            current_block = 14359455889292382949;
        }
        match current_block {
            9435687255277069653 => {}
            _ => {
                if dispose_none_params.should_try_ != 0 {
                    error_code = GenerateCandidates(
                        enc,
                        candidates.as_mut_ptr(),
                        WEBP_MUX_DISPOSE_NONE,
                        is_lossless,
                        is_key_frame,
                        &mut dispose_none_params,
                        &mut config_ll,
                        &mut config_lossy,
                    );
                    if error_code as libc::c_uint
                        != VP8_ENC_OK as libc::c_int as libc::c_uint
                    {
                        current_block = 9435687255277069653;
                    } else {
                        current_block = 5494826135382683477;
                    }
                } else {
                    current_block = 5494826135382683477;
                }
                match current_block {
                    9435687255277069653 => {}
                    _ => {
                        if dispose_bg_params.should_try_ != 0 {
                            error_code = GenerateCandidates(
                                enc,
                                candidates.as_mut_ptr(),
                                WEBP_MUX_DISPOSE_BACKGROUND,
                                is_lossless,
                                is_key_frame,
                                &mut dispose_bg_params,
                                &mut config_ll,
                                &mut config_lossy,
                            );
                            if error_code as libc::c_uint
                                != VP8_ENC_OK as libc::c_int as libc::c_uint
                            {
                                current_block = 9435687255277069653;
                            } else {
                                current_block = 2604890879466389055;
                            }
                        } else {
                            current_block = 2604890879466389055;
                        }
                        match current_block {
                            9435687255277069653 => {}
                            _ => {
                                PickBestCandidate(
                                    enc,
                                    candidates.as_mut_ptr(),
                                    is_key_frame,
                                    encoded_frame,
                                );
                                current_block = 17243318549450349268;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        9435687255277069653 => {
            i = 0 as libc::c_int;
            while i < CANDIDATE_COUNT as libc::c_int {
                if candidates[i as usize].evaluate_ != 0 {
                    WebPMemoryWriterClear(
                        &mut (*candidates.as_mut_ptr().offset(i as isize)).mem_,
                    );
                }
                i += 1;
                i;
            }
        }
        _ => {}
    }
    SubFrameParamsFree(&mut dispose_none_params);
    SubFrameParamsFree(&mut dispose_bg_params);
    return error_code;
}
unsafe extern "C" fn KeyFramePenalty(encoded_frame: *const EncodedFrame) -> int64_t {
    return ((*encoded_frame).key_frame_.bitstream.size as int64_t as libc::c_ulong)
        .wrapping_sub((*encoded_frame).sub_frame_.bitstream.size) as int64_t;
}
unsafe extern "C" fn CacheFrame(
    enc: *mut WebPAnimEncoder,
    config: *const WebPConfig,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut frame_skipped: libc::c_int = 0 as libc::c_int;
    let mut error_code: WebPEncodingError = VP8_ENC_OK;
    let position: size_t = (*enc).count_;
    let encoded_frame: *mut EncodedFrame = GetFrame(enc, position);
    (*enc).count_ = ((*enc).count_).wrapping_add(1);
    (*enc).count_;
    if (*enc).is_first_frame_ != 0 {
        error_code = SetFrame(
            enc,
            config,
            1 as libc::c_int,
            encoded_frame,
            &mut frame_skipped,
        );
        if error_code as libc::c_uint != VP8_ENC_OK as libc::c_int as libc::c_uint {
            current_block = 886215699392744263;
        } else {
            (*encoded_frame).is_key_frame_ = 1 as libc::c_int;
            (*enc).flush_count_ = 0 as libc::c_int as size_t;
            (*enc).count_since_key_frame_ = 0 as libc::c_int;
            (*enc).prev_candidate_undecided_ = 0 as libc::c_int;
            current_block = 17784502470059252271;
        }
    } else {
        (*enc).count_since_key_frame_ += 1;
        (*enc).count_since_key_frame_;
        if (*enc).count_since_key_frame_ <= (*enc).options_.kmin {
            error_code = SetFrame(
                enc,
                config,
                0 as libc::c_int,
                encoded_frame,
                &mut frame_skipped,
            );
            if error_code as libc::c_uint != VP8_ENC_OK as libc::c_int as libc::c_uint {
                current_block = 886215699392744263;
            } else if frame_skipped != 0 {
                current_block = 8341084358799622041;
            } else {
                (*encoded_frame).is_key_frame_ = 0 as libc::c_int;
                (*enc)
                    .flush_count_ = ((*enc).count_)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                (*enc).prev_candidate_undecided_ = 0 as libc::c_int;
                current_block = 17784502470059252271;
            }
        } else {
            let mut curr_delta: int64_t = 0;
            let mut prev_rect_key: FrameRectangle = FrameRectangle {
                x_offset_: 0,
                y_offset_: 0,
                width_: 0,
                height_: 0,
            };
            let mut prev_rect_sub: FrameRectangle = FrameRectangle {
                x_offset_: 0,
                y_offset_: 0,
                width_: 0,
                height_: 0,
            };
            error_code = SetFrame(
                enc,
                config,
                0 as libc::c_int,
                encoded_frame,
                &mut frame_skipped,
            );
            if error_code as libc::c_uint != VP8_ENC_OK as libc::c_int as libc::c_uint {
                current_block = 886215699392744263;
            } else if frame_skipped != 0 {
                current_block = 8341084358799622041;
            } else {
                prev_rect_sub = (*enc).prev_rect_;
                error_code = SetFrame(
                    enc,
                    config,
                    1 as libc::c_int,
                    encoded_frame,
                    &mut frame_skipped,
                );
                if error_code as libc::c_uint
                    != VP8_ENC_OK as libc::c_int as libc::c_uint
                {
                    current_block = 886215699392744263;
                } else {
                    prev_rect_key = (*enc).prev_rect_;
                    curr_delta = KeyFramePenalty(encoded_frame);
                    if curr_delta <= (*enc).best_delta_ {
                        if (*enc).keyframe_ != -(1 as libc::c_int) {
                            let old_keyframe: *mut EncodedFrame = GetFrame(
                                enc,
                                (*enc).keyframe_ as size_t,
                            );
                            (*old_keyframe).is_key_frame_ = 0 as libc::c_int;
                        }
                        (*encoded_frame).is_key_frame_ = 1 as libc::c_int;
                        (*enc).prev_candidate_undecided_ = 1 as libc::c_int;
                        (*enc).keyframe_ = position as libc::c_int;
                        (*enc).best_delta_ = curr_delta;
                        (*enc)
                            .flush_count_ = ((*enc).count_)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    } else {
                        (*encoded_frame).is_key_frame_ = 0 as libc::c_int;
                        (*enc).prev_candidate_undecided_ = 0 as libc::c_int;
                    }
                    if (*enc).count_since_key_frame_ >= (*enc).options_.kmax {
                        (*enc)
                            .flush_count_ = ((*enc).count_)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        (*enc).count_since_key_frame_ = 0 as libc::c_int;
                        (*enc).keyframe_ = -(1 as libc::c_int);
                        (*enc)
                            .best_delta_ = ((1 as libc::c_ulonglong)
                            << 32 as libc::c_int) as int64_t;
                    }
                    if (*enc).prev_candidate_undecided_ == 0 {
                        (*enc)
                            .prev_rect_ = if (*encoded_frame).is_key_frame_ != 0 {
                            prev_rect_key
                        } else {
                            prev_rect_sub
                        };
                    }
                    current_block = 17784502470059252271;
                }
            }
        }
    }
    match current_block {
        17784502470059252271 => {
            WebPCopyPixels((*enc).curr_canvas_, &mut (*enc).prev_canvas_);
            (*enc).is_first_frame_ = 0 as libc::c_int;
            current_block = 8341084358799622041;
        }
        _ => {}
    }
    match current_block {
        8341084358799622041 => {
            ok = 1 as libc::c_int;
            (*enc).in_frame_count_ = ((*enc).in_frame_count_).wrapping_add(1);
            (*enc).in_frame_count_;
        }
        _ => {}
    }
    if ok == 0 || frame_skipped != 0 {
        FrameRelease(encoded_frame);
        (*enc).count_ = ((*enc).count_).wrapping_sub(1);
        (*enc).count_;
        if (*enc).is_first_frame_ == 0 {
            (*enc).count_since_key_frame_ -= 1;
            (*enc).count_since_key_frame_;
        }
        if ok == 0 {
            MarkError2(
                enc,
                b"ERROR adding frame. WebPEncodingError\0" as *const u8
                    as *const libc::c_char,
                error_code as libc::c_int,
            );
        }
    }
    (*(*enc).curr_canvas_).error_code = error_code;
    return ok;
}
unsafe extern "C" fn FlushFrames(enc: *mut WebPAnimEncoder) -> libc::c_int {
    while (*enc).flush_count_ > 0 as libc::c_int as libc::c_ulong {
        let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
        let curr: *mut EncodedFrame = GetFrame(enc, 0 as libc::c_int as size_t);
        let info: *const WebPMuxFrameInfo = if (*curr).is_key_frame_ != 0 {
            &mut (*curr).key_frame_
        } else {
            &mut (*curr).sub_frame_
        };
        err = WebPMuxPushFrame((*enc).mux_, info, 1 as libc::c_int);
        if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
            MarkError2(
                enc,
                b"ERROR adding frame. WebPMuxError\0" as *const u8
                    as *const libc::c_char,
                err as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        if (*enc).options_.verbose != 0 {
            eprintln!(
                "INFO: Added frame. offset:{},{} dispose:{} blend:{}",
                (*info).x_offset,
                (*info).y_offset,
                (*info).dispose_method as libc::c_uint,
                (*info).blend_method as libc::c_uint,
            );
        }
        (*enc).out_frame_count_ = ((*enc).out_frame_count_).wrapping_add(1);
        (*enc).out_frame_count_;
        FrameRelease(curr);
        (*enc).start_ = ((*enc).start_).wrapping_add(1);
        (*enc).start_;
        (*enc).flush_count_ = ((*enc).flush_count_).wrapping_sub(1);
        (*enc).flush_count_;
        (*enc).count_ = ((*enc).count_).wrapping_sub(1);
        (*enc).count_;
        if (*enc).keyframe_ != -(1 as libc::c_int) {
            (*enc).keyframe_ -= 1;
            (*enc).keyframe_;
        }
    }
    if (*enc).count_ == 1 as libc::c_int as libc::c_ulong
        && (*enc).start_ != 0 as libc::c_int as libc::c_ulong
    {
        let enc_start_tmp: libc::c_int = (*enc).start_ as libc::c_int;
        let mut temp: EncodedFrame = *((*enc).encoded_frames_)
            .offset(0 as libc::c_int as isize);
        *((*enc).encoded_frames_)
            .offset(
                0 as libc::c_int as isize,
            ) = *((*enc).encoded_frames_).offset(enc_start_tmp as isize);
        *((*enc).encoded_frames_).offset(enc_start_tmp as isize) = temp;
        FrameRelease(&mut *((*enc).encoded_frames_).offset(enc_start_tmp as isize));
        (*enc).start_ = 0 as libc::c_int as size_t;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderAdd(
    mut enc: *mut WebPAnimEncoder,
    mut frame: *mut WebPPicture,
    mut timestamp: libc::c_int,
    mut encoder_config: *const WebPConfig,
) -> libc::c_int {
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
    let mut ok: libc::c_int = 0;
    if enc.is_null() {
        return 0 as libc::c_int;
    }
    MarkNoError(enc);
    if (*enc).is_first_frame_ == 0 {
        let prev_frame_duration: uint32_t = (timestamp as uint32_t)
            .wrapping_sub((*enc).prev_timestamp_ as libc::c_uint);
        if prev_frame_duration
            >= ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
        {
            if !frame.is_null() {
                (*frame).error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
            }
            MarkError(
                enc,
                b"ERROR adding frame: timestamps must be non-decreasing\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        if IncreasePreviousDuration(enc, prev_frame_duration as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        if (*enc).count_ == (*enc).size_ && FlushFrames(enc) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        (*enc).first_timestamp_ = timestamp;
    }
    if frame.is_null() {
        (*enc).got_null_frame_ = 1 as libc::c_int;
        (*enc).prev_timestamp_ = timestamp;
        return 1 as libc::c_int;
    }
    if (*frame).width != (*enc).canvas_width_ || (*frame).height != (*enc).canvas_height_
    {
        (*frame).error_code = VP8_ENC_ERROR_INVALID_CONFIGURATION;
        MarkError(
            enc,
            b"ERROR adding frame: Invalid frame dimensions\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*frame).use_argb == 0 {
        if (*enc).options_.verbose != 0 {
            eprintln!("WARNING: Converting frame from YUV(A) to ARGB format; this incurs a small loss.");
        }
        if WebPPictureYUVAToARGB(frame) == 0 {
            MarkError(
                enc,
                b"ERROR converting frame from YUV(A) to ARGB\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    if !encoder_config.is_null() {
        if WebPValidateConfig(encoder_config) == 0 {
            MarkError(
                enc,
                b"ERROR adding frame: Invalid WebPConfig\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
        config = *encoder_config;
    } else {
        if WebPConfigInit(&mut config) == 0 {
            MarkError(enc, b"Cannot Init config\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        config.lossless = 1 as libc::c_int;
    }
    (*enc).curr_canvas_ = frame;
    CopyCurrentCanvas(enc);
    ok = (CacheFrame(enc, &mut config) != 0 && FlushFrames(enc) != 0) as libc::c_int;
    (*enc).curr_canvas_ = 0 as *mut WebPPicture;
    (*enc).curr_canvas_copy_modified_ = 1 as libc::c_int;
    if ok != 0 {
        (*enc).prev_timestamp_ = timestamp;
    }
    return ok;
}
unsafe extern "C" fn DecodeFrameOntoCanvas(
    frame: *const WebPMuxFrameInfo,
    canvas: *mut WebPPicture,
) -> libc::c_int {
    let image: *const WebPData = &(*frame).bitstream;
    let mut sub_image: WebPPicture = WebPPicture {
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
    let mut config: WebPDecoderConfig = WebPDecoderConfig {
        input: WebPBitstreamFeatures {
            width: 0,
            height: 0,
            has_alpha: 0,
            has_animation: 0,
            format: 0,
            pad: [0; 5],
        },
        output: WebPDecBuffer {
            colorspace: MODE_RGB,
            width: 0,
            height: 0,
            is_external_memory: 0,
            u: WebPRGBABuffer_WebPYUVABuffer {
                RGBA: WebPRGBABuffer {
                    rgba: 0 as *mut uint8_t,
                    stride: 0,
                    size: 0,
                },
            },
            pad: [0; 4],
            private_memory: 0 as *mut uint8_t,
        },
        options: WebPDecoderOptions {
            bypass_filtering: 0,
            no_fancy_upsampling: 0,
            use_cropping: 0,
            crop_left: 0,
            crop_top: 0,
            crop_width: 0,
            crop_height: 0,
            use_scaling: 0,
            scaled_width: 0,
            scaled_height: 0,
            use_threads: 0,
            dithering_strength: 0,
            flip: 0,
            alpha_dithering_strength: 0,
            pad: [0; 5],
        },
    };
    if WebPInitDecoderConfig(&mut config) == 0 {
        return 0 as libc::c_int;
    }
    WebPUtilClearPic(canvas, 0 as *const FrameRectangle);
    if WebPGetFeatures((*image).bytes, (*image).size, &mut config.input) as libc::c_uint
        != VP8_STATUS_OK as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if WebPPictureView(
        canvas,
        (*frame).x_offset,
        (*frame).y_offset,
        config.input.width,
        config.input.height,
        &mut sub_image,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    config.output.is_external_memory = 1 as libc::c_int;
    config.output.colorspace = MODE_BGRA;
    config.output.u.RGBA.rgba = sub_image.argb as *mut uint8_t;
    config.output.u.RGBA.stride = sub_image.argb_stride * 4 as libc::c_int;
    config
        .output
        .u
        .RGBA
        .size = (config.output.u.RGBA.stride * sub_image.height) as size_t;
    if WebPDecode((*image).bytes, (*image).size, &mut config) as libc::c_uint
        != VP8_STATUS_OK as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn FrameToFullCanvas(
    enc: *mut WebPAnimEncoder,
    frame: *const WebPMuxFrameInfo,
    full_image: *mut WebPData,
) -> libc::c_int {
    let mut current_block: u64;
    let canvas_buf: *mut WebPPicture = &mut (*enc).curr_canvas_copy_;
    let mut mem1: WebPMemoryWriter = WebPMemoryWriter {
        mem: 0 as *mut uint8_t,
        size: 0,
        max_size: 0,
        pad: [0; 1],
    };
    let mut mem2: WebPMemoryWriter = WebPMemoryWriter {
        mem: 0 as *mut uint8_t,
        size: 0,
        max_size: 0,
        pad: [0; 1],
    };
    WebPMemoryWriterInit(&mut mem1);
    WebPMemoryWriterInit(&mut mem2);
    if !(DecodeFrameOntoCanvas(frame, canvas_buf) == 0) {
        if !(EncodeFrame(&mut (*enc).last_config_, canvas_buf, &mut mem1) == 0) {
            GetEncodedData(&mut mem1, full_image);
            if (*enc).options_.allow_mixed != 0 {
                if EncodeFrame(&mut (*enc).last_config_reversed_, canvas_buf, &mut mem2)
                    == 0
                {
                    current_block = 2296659974064666431;
                } else {
                    if mem2.size < mem1.size {
                        GetEncodedData(&mut mem2, full_image);
                        WebPMemoryWriterClear(&mut mem1);
                    } else {
                        WebPMemoryWriterClear(&mut mem2);
                    }
                    current_block = 7746791466490516765;
                }
            } else {
                current_block = 7746791466490516765;
            }
            match current_block {
                2296659974064666431 => {}
                _ => return 1 as libc::c_int,
            }
        }
    }
    WebPMemoryWriterClear(&mut mem1);
    WebPMemoryWriterClear(&mut mem2);
    return 0 as libc::c_int;
}
unsafe extern "C" fn OptimizeSingleFrame(
    enc: *mut WebPAnimEncoder,
    webp_data: *mut WebPData,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_OK;
    let mut canvas_width: libc::c_int = 0;
    let mut canvas_height: libc::c_int = 0;
    let mut frame: WebPMuxFrameInfo = WebPMuxFrameInfo {
        bitstream: WebPData {
            bytes: 0 as *const uint8_t,
            size: 0,
        },
        x_offset: 0,
        y_offset: 0,
        duration: 0,
        id: WEBP_CHUNK_VP8X,
        dispose_method: WEBP_MUX_DISPOSE_NONE,
        blend_method: WEBP_MUX_BLEND,
        pad: [0; 1],
    };
    let mut full_image: WebPData = WebPData {
        bytes: 0 as *const uint8_t,
        size: 0,
    };
    let mut webp_data2: WebPData = WebPData {
        bytes: 0 as *const uint8_t,
        size: 0,
    };
    let mux: *mut WebPMux = WebPMuxCreate(webp_data, 0 as libc::c_int);
    if mux.is_null() {
        return WEBP_MUX_BAD_DATA;
    }
    WebPDataInit(&mut frame.bitstream);
    WebPDataInit(&mut full_image);
    WebPDataInit(&mut webp_data2);
    err = WebPMuxGetFrame(mux, 1 as libc::c_int as uint32_t, &mut frame);
    if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
        if !(frame.id as libc::c_uint != WEBP_CHUNK_ANMF as libc::c_int as libc::c_uint)
        {
            err = WebPMuxGetCanvasSize(mux, &mut canvas_width, &mut canvas_height);
            if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
                if FrameToFullCanvas(enc, &mut frame, &mut full_image) == 0 {
                    err = WEBP_MUX_BAD_DATA;
                } else {
                    err = WebPMuxSetImage(mux, &mut full_image, 1 as libc::c_int);
                    if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
                        err = WebPMuxAssemble(mux, &mut webp_data2);
                        if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
                            if webp_data2.size < (*webp_data).size {
                                WebPDataClear(webp_data);
                                *webp_data = webp_data2;
                                WebPDataInit(&mut webp_data2);
                            }
                        }
                    }
                }
            }
        }
    }
    WebPDataClear(&mut frame.bitstream);
    WebPDataClear(&mut full_image);
    WebPMuxDelete(mux);
    WebPDataClear(&mut webp_data2);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderAssemble(
    mut enc: *mut WebPAnimEncoder,
    mut webp_data: *mut WebPData,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mux: *mut WebPMux = 0 as *mut WebPMux;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if enc.is_null() {
        return 0 as libc::c_int;
    }
    MarkNoError(enc);
    if webp_data.is_null() {
        MarkError(
            enc,
            b"ERROR assembling: NULL input\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*enc).in_frame_count_ == 0 as libc::c_int as libc::c_ulong {
        MarkError(
            enc,
            b"ERROR: No frames to assemble\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*enc).got_null_frame_ == 0
        && (*enc).in_frame_count_ > 1 as libc::c_int as libc::c_ulong
        && (*enc).count_ > 0 as libc::c_int as libc::c_ulong
    {
        let delta_time: libc::c_double = ((*enc).prev_timestamp_ as uint32_t)
            .wrapping_sub((*enc).first_timestamp_ as libc::c_uint) as libc::c_double;
        let average_duration: libc::c_int = (delta_time
            / ((*enc).in_frame_count_).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_double) as libc::c_int;
        if IncreasePreviousDuration(enc, average_duration) == 0 {
            return 0 as libc::c_int;
        }
    }
    (*enc).flush_count_ = (*enc).count_;
    if FlushFrames(enc) == 0 {
        return 0 as libc::c_int;
    }
    mux = (*enc).mux_;
    err = WebPMuxSetCanvasSize(mux, (*enc).canvas_width_, (*enc).canvas_height_);
    if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
        err = WebPMuxSetAnimationParams(mux, &(*enc).options_.anim_params);
        if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
            err = WebPMuxAssemble(mux, webp_data);
            if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
                if (*enc).out_frame_count_ == 1 as libc::c_int as libc::c_ulong {
                    err = OptimizeSingleFrame(enc, webp_data);
                    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                        current_block = 13954271901034346992;
                    } else {
                        current_block = 11194104282611034094;
                    }
                } else {
                    current_block = 11194104282611034094;
                }
                match current_block {
                    13954271901034346992 => {}
                    _ => return 1 as libc::c_int,
                }
            }
        }
    }
    MarkError2(
        enc,
        b"ERROR assembling WebP\0" as *const u8 as *const libc::c_char,
        err as libc::c_int,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderGetError(
    mut enc: *mut WebPAnimEncoder,
) -> *const libc::c_char {
    if enc.is_null() {
        return 0 as *const libc::c_char;
    }
    return ((*enc).error_str_).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderSetChunk(
    mut enc: *mut WebPAnimEncoder,
    mut fourcc: *const libc::c_char,
    mut chunk_data: *const WebPData,
    mut copy_data: libc::c_int,
) -> WebPMuxError {
    if enc.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return WebPMuxSetChunk((*enc).mux_, fourcc, chunk_data, copy_data);
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderGetChunk(
    mut enc: *const WebPAnimEncoder,
    mut fourcc: *const libc::c_char,
    mut chunk_data: *mut WebPData,
) -> WebPMuxError {
    if enc.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return WebPMuxGetChunk((*enc).mux_, fourcc, chunk_data);
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimEncoderDeleteChunk(
    mut enc: *mut WebPAnimEncoder,
    mut fourcc: *const libc::c_char,
) -> WebPMuxError {
    if enc.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return WebPMuxDeleteChunk((*enc).mux_, fourcc);
}
