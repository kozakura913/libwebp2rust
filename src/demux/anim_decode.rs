use ::libc;

use crate::src::utils::types::{self, VP8StatusCode, WebPData, WebPDecoderConfig, WebPRGBABuffer, VP8_STATUS_OK};

use super::demux::WebPDemuxer;
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
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
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
    fn WebPDemuxInternal(
        _: *const WebPData,
        _: libc::c_int,
        _: *mut WebPDemuxState,
        _: libc::c_int,
    ) -> *mut WebPDemuxer;
    fn WebPDemuxDelete(dmux: *mut WebPDemuxer);
    fn WebPDemuxGetI(dmux: *const WebPDemuxer, feature: WebPFormatFeature) -> uint32_t;
    fn WebPDemuxGetFrame(
        dmux: *const WebPDemuxer,
        frame_number: libc::c_int,
        iter: *mut WebPIterator,
    ) -> libc::c_int;
    fn WebPDemuxReleaseIterator(iter: *mut WebPIterator);
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
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub is_external_memory: libc::c_int,
    pub u: types::WebPRGBABuffer_WebPYUVABuffer,
    pub pad: [uint32_t; 4],
    pub private_memory: *mut uint8_t,
}
pub type WEBP_CSP_MODE = libc::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const MODE_Argb: WEBP_CSP_MODE = 9;
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub const MODE_RGB: WEBP_CSP_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPBitstreamFeatures {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub has_alpha: libc::c_int,
    pub has_animation: libc::c_int,
    pub format: libc::c_int,
    pub pad: [uint32_t; 5],
}
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
pub type WebPMuxAnimDispose = libc::c_uint;
pub const WEBP_MUX_DISPOSE_BACKGROUND: WebPMuxAnimDispose = 1;
pub const WEBP_MUX_DISPOSE_NONE: WebPMuxAnimDispose = 0;
pub type WebPMuxAnimBlend = libc::c_uint;
pub const WEBP_MUX_NO_BLEND: WebPMuxAnimBlend = 1;
pub const WEBP_MUX_BLEND: WebPMuxAnimBlend = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPIterator {
    pub frame_num: libc::c_int,
    pub num_frames: libc::c_int,
    pub x_offset: libc::c_int,
    pub y_offset: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub duration: libc::c_int,
    pub dispose_method: WebPMuxAnimDispose,
    pub complete: libc::c_int,
    pub fragment: WebPData,
    pub has_alpha: libc::c_int,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [uint32_t; 2],
    pub private_: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAnimInfo {
    pub canvas_width: uint32_t,
    pub canvas_height: uint32_t,
    pub loop_count: uint32_t,
    pub bgcolor: uint32_t,
    pub frame_count: uint32_t,
    pub pad: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAnimDecoderOptions {
    pub color_mode: WEBP_CSP_MODE,
    pub use_threads: libc::c_int,
    pub padding: [uint32_t; 7],
}
pub type WebPDemuxState = libc::c_int;
pub const WEBP_DEMUX_DONE: WebPDemuxState = 2;
pub const WEBP_DEMUX_PARSED_HEADER: WebPDemuxState = 1;
pub const WEBP_DEMUX_PARSING_HEADER: WebPDemuxState = 0;
pub const WEBP_DEMUX_PARSE_ERROR: WebPDemuxState = -1;
pub type WebPFormatFeature = libc::c_uint;
pub const WEBP_FF_FRAME_COUNT: WebPFormatFeature = 5;
pub const WEBP_FF_BACKGROUND_COLOR: WebPFormatFeature = 4;
pub const WEBP_FF_LOOP_COUNT: WebPFormatFeature = 3;
pub const WEBP_FF_CANVAS_HEIGHT: WebPFormatFeature = 2;
pub const WEBP_FF_CANVAS_WIDTH: WebPFormatFeature = 1;
pub const WEBP_FF_FORMAT_FLAGS: WebPFormatFeature = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPAnimDecoder {
    pub demux_: *mut WebPDemuxer,
    pub config_: WebPDecoderConfig,
    pub blend_func_: BlendRowFunc,
    pub info_: WebPAnimInfo,
    pub curr_frame_: *mut uint8_t,
    pub prev_frame_disposed_: *mut uint8_t,
    pub prev_frame_timestamp_: libc::c_int,
    pub prev_iter_: WebPIterator,
    pub prev_frame_was_keyframe_: libc::c_int,
    pub next_frame_: libc::c_int,
}
pub type BlendRowFunc = Option::<
    unsafe extern "C" fn(*mut uint32_t, *const uint32_t, libc::c_int) -> (),
>;
#[inline]
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> libc::c_int {
    return (size == size) as libc::c_int;
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
unsafe extern "C" fn WebPDemux(mut data: *const WebPData) -> *mut WebPDemuxer {
    return WebPDemuxInternal(
        data,
        0 as libc::c_int,
        0 as *mut WebPDemuxState,
        0x107 as libc::c_int,
    );
}
unsafe extern "C" fn DefaultDecoderOptions(dec_options: *mut WebPAnimDecoderOptions) {
    (*dec_options).color_mode = MODE_RGBA;
    (*dec_options).use_threads = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderOptionsInitInternal(
    mut dec_options: *mut WebPAnimDecoderOptions,
    mut abi_version: libc::c_int,
) -> libc::c_int {
    if dec_options.is_null()
        || abi_version >> 8 as libc::c_int != 0x107 as libc::c_int >> 8 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    DefaultDecoderOptions(dec_options);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ApplyDecoderOptions(
    dec_options: *const WebPAnimDecoderOptions,
    dec: *mut WebPAnimDecoder,
) -> libc::c_int {
    let mut mode: WEBP_CSP_MODE = MODE_RGB;
    let mut config: *mut WebPDecoderConfig = &mut (*dec).config_;
    mode = (*dec_options).color_mode;
    if mode as libc::c_uint != MODE_RGBA as libc::c_int as libc::c_uint
        && mode as libc::c_uint != MODE_BGRA as libc::c_int as libc::c_uint
        && mode as libc::c_uint != MODE_rgbA as libc::c_int as libc::c_uint
        && mode as libc::c_uint != MODE_bgrA as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    (*dec)
        .blend_func_ = if mode as libc::c_uint
        == MODE_RGBA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_BGRA as libc::c_int as libc::c_uint
    {
        Some(
            BlendPixelRowNonPremult
                as unsafe extern "C" fn(
                    *mut uint32_t,
                    *const uint32_t,
                    libc::c_int,
                ) -> (),
        )
    } else {
        Some(
            BlendPixelRowPremult
                as unsafe extern "C" fn(
                    *mut uint32_t,
                    *const uint32_t,
                    libc::c_int,
                ) -> (),
        )
    };
    if WebPInitDecoderConfig(config) == 0 {
        return 0 as libc::c_int;
    }
    (*config).output.colorspace = mode;
    (*config).output.is_external_memory = 1 as libc::c_int;
    (*config).options.use_threads = (*dec_options).use_threads;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderNewInternal(
    mut webp_data: *const WebPData,
    mut dec_options: *const WebPAnimDecoderOptions,
    mut abi_version: libc::c_int,
) -> *mut WebPAnimDecoder {
    let mut options: WebPAnimDecoderOptions = WebPAnimDecoderOptions {
        color_mode: MODE_RGB,
        use_threads: 0,
        padding: [0; 7],
    };
    let mut dec: *mut WebPAnimDecoder = 0 as *mut WebPAnimDecoder;
    let mut features: WebPBitstreamFeatures = WebPBitstreamFeatures {
        width: 0,
        height: 0,
        has_alpha: 0,
        has_animation: 0,
        format: 0,
        pad: [0; 5],
    };
    if webp_data.is_null()
        || abi_version >> 8 as libc::c_int != 0x107 as libc::c_int >> 8 as libc::c_int
    {
        return 0 as *mut WebPAnimDecoder;
    }
    if WebPGetFeatures((*webp_data).bytes, (*webp_data).size, &mut features)
        as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint
    {
        return 0 as *mut WebPAnimDecoder;
    }
    dec = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<WebPAnimDecoder>() as libc::c_ulong,
    ) as *mut WebPAnimDecoder;
    if !dec.is_null() {
        if !dec_options.is_null() {
            options = *dec_options;
        } else {
            DefaultDecoderOptions(&mut options);
        }
        if !(ApplyDecoderOptions(&mut options, dec) == 0) {
            (*dec).demux_ = WebPDemux(webp_data);
            if !((*dec).demux_).is_null() {
                (*dec)
                    .info_
                    .canvas_width = WebPDemuxGetI((*dec).demux_, WEBP_FF_CANVAS_WIDTH);
                (*dec)
                    .info_
                    .canvas_height = WebPDemuxGetI((*dec).demux_, WEBP_FF_CANVAS_HEIGHT);
                (*dec)
                    .info_
                    .loop_count = WebPDemuxGetI((*dec).demux_, WEBP_FF_LOOP_COUNT);
                (*dec)
                    .info_
                    .bgcolor = WebPDemuxGetI((*dec).demux_, WEBP_FF_BACKGROUND_COLOR);
                (*dec)
                    .info_
                    .frame_count = WebPDemuxGetI((*dec).demux_, WEBP_FF_FRAME_COUNT);
                (*dec)
                    .curr_frame_ = WebPSafeCalloc(
                    ((*dec).info_.canvas_width)
                        .wrapping_mul(4 as libc::c_int as libc::c_uint) as uint64_t,
                    (*dec).info_.canvas_height as size_t,
                ) as *mut uint8_t;
                if !((*dec).curr_frame_).is_null() {
                    (*dec)
                        .prev_frame_disposed_ = WebPSafeCalloc(
                        ((*dec).info_.canvas_width)
                            .wrapping_mul(4 as libc::c_int as libc::c_uint) as uint64_t,
                        (*dec).info_.canvas_height as size_t,
                    ) as *mut uint8_t;
                    if !((*dec).prev_frame_disposed_).is_null() {
                        WebPAnimDecoderReset(dec);
                        return dec;
                    }
                }
            }
        }
    }
    WebPAnimDecoderDelete(dec);
    return 0 as *mut WebPAnimDecoder;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderGetInfo(
    mut dec: *const WebPAnimDecoder,
    mut info: *mut WebPAnimInfo,
) -> libc::c_int {
    if dec.is_null() || info.is_null() {
        return 0 as libc::c_int;
    }
    *info = (*dec).info_;
    return 1 as libc::c_int;
}
unsafe extern "C" fn IsFullFrame(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut canvas_width: libc::c_int,
    mut canvas_height: libc::c_int,
) -> libc::c_int {
    return (width == canvas_width && height == canvas_height) as libc::c_int;
}
unsafe extern "C" fn ZeroFillCanvas(
    mut buf: *mut uint8_t,
    mut canvas_width: uint32_t,
    mut canvas_height: uint32_t,
) -> libc::c_int {
    let size: uint64_t = (canvas_width as uint64_t)
        .wrapping_mul(canvas_height as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong);
    if CheckSizeOverflow(size) == 0 {
        return 0 as libc::c_int;
    }
    memset(buf as *mut libc::c_void, 0 as libc::c_int, size);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ZeroFillFrameRect(
    mut buf: *mut uint8_t,
    mut buf_stride: libc::c_int,
    mut x_offset: libc::c_int,
    mut y_offset: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    buf = buf.offset((y_offset * buf_stride + x_offset * 4 as libc::c_int) as isize);
    j = 0 as libc::c_int;
    while j < height {
        memset(
            buf as *mut libc::c_void,
            0 as libc::c_int,
            (width * 4 as libc::c_int) as libc::c_ulong,
        );
        buf = buf.offset(buf_stride as isize);
        j += 1;
        j;
    }
}
unsafe extern "C" fn CopyCanvas(
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
    mut width: uint32_t,
    mut height: uint32_t,
) -> libc::c_int {
    let size: uint64_t = (width as uint64_t)
        .wrapping_mul(height as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong);
    if CheckSizeOverflow(size) == 0 {
        return 0 as libc::c_int;
    }
    memcpy(dst as *mut libc::c_void, src as *const libc::c_void, size);
    return 1 as libc::c_int;
}
unsafe extern "C" fn IsKeyFrame(
    curr: *const WebPIterator,
    prev: *const WebPIterator,
    mut prev_frame_was_key_frame: libc::c_int,
    mut canvas_width: libc::c_int,
    mut canvas_height: libc::c_int,
) -> libc::c_int {
    if (*curr).frame_num == 1 as libc::c_int {
        return 1 as libc::c_int
    } else if ((*curr).has_alpha == 0
        || (*curr).blend_method as libc::c_uint
            == WEBP_MUX_NO_BLEND as libc::c_int as libc::c_uint)
        && IsFullFrame((*curr).width, (*curr).height, canvas_width, canvas_height) != 0
    {
        return 1 as libc::c_int
    } else {
        return ((*prev).dispose_method as libc::c_uint
            == WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int as libc::c_uint
            && (IsFullFrame((*prev).width, (*prev).height, canvas_width, canvas_height)
                != 0 || prev_frame_was_key_frame != 0)) as libc::c_int
    };
}
unsafe extern "C" fn BlendChannelNonPremult(
    mut src: uint32_t,
    mut src_a: uint8_t,
    mut dst: uint32_t,
    mut dst_a: uint8_t,
    mut scale: uint32_t,
    mut shift: libc::c_int,
) -> uint8_t {
    let src_channel: uint8_t = (src >> shift & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    let dst_channel: uint8_t = (dst >> shift & 0xff as libc::c_int as libc::c_uint)
        as uint8_t;
    let blend_unscaled: uint32_t = (src_channel as libc::c_int * src_a as libc::c_int
        + dst_channel as libc::c_int * dst_a as libc::c_int) as uint32_t;
    return (blend_unscaled.wrapping_mul(scale) >> 3 as libc::c_int * 8 as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn BlendPixelNonPremult(
    mut src: uint32_t,
    mut dst: uint32_t,
) -> uint32_t {
    let src_a: uint8_t = (src >> 3 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    if src_a as libc::c_int == 0 as libc::c_int {
        return dst
    } else {
        let dst_a: uint8_t = (dst >> 3 as libc::c_int * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        let dst_factor_a: uint8_t = (dst_a as libc::c_int
            * (256 as libc::c_int - src_a as libc::c_int) >> 8 as libc::c_int)
            as uint8_t;
        let blend_a: uint8_t = (src_a as libc::c_int + dst_factor_a as libc::c_int)
            as uint8_t;
        let scale: uint32_t = ((1 as libc::c_ulong) << 24 as libc::c_int)
            .wrapping_div(blend_a as libc::c_ulong) as uint32_t;
        let blend_r: uint8_t = BlendChannelNonPremult(
            src,
            src_a,
            dst,
            dst_factor_a,
            scale,
            0 as libc::c_int * 8 as libc::c_int,
        );
        let blend_g: uint8_t = BlendChannelNonPremult(
            src,
            src_a,
            dst,
            dst_factor_a,
            scale,
            1 as libc::c_int * 8 as libc::c_int,
        );
        let blend_b: uint8_t = BlendChannelNonPremult(
            src,
            src_a,
            dst,
            dst_factor_a,
            scale,
            2 as libc::c_int * 8 as libc::c_int,
        );
        return (blend_r as uint32_t) << 0 as libc::c_int * 8 as libc::c_int
            | (blend_g as uint32_t) << 1 as libc::c_int * 8 as libc::c_int
            | (blend_b as uint32_t) << 2 as libc::c_int * 8 as libc::c_int
            | (blend_a as uint32_t) << 3 as libc::c_int * 8 as libc::c_int;
    };
}
unsafe extern "C" fn BlendPixelRowNonPremult(
    src: *mut uint32_t,
    dst: *const uint32_t,
    mut num_pixels: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        let src_alpha: uint8_t = (*src.offset(i as isize)
            >> 3 as libc::c_int * 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        if src_alpha as libc::c_int != 0xff as libc::c_int {
            *src
                .offset(
                    i as isize,
                ) = BlendPixelNonPremult(
                *src.offset(i as isize),
                *dst.offset(i as isize),
            );
        }
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn ChannelwiseMultiply(
    mut pix: uint32_t,
    mut scale: uint32_t,
) -> uint32_t {
    let mut mask: uint32_t = 0xff00ff as libc::c_int as uint32_t;
    let mut rb: uint32_t = (pix & mask).wrapping_mul(scale) >> 8 as libc::c_int;
    let mut ag: uint32_t = (pix >> 8 as libc::c_int & mask).wrapping_mul(scale);
    return rb & mask | ag & !mask;
}
unsafe extern "C" fn BlendPixelPremult(
    mut src: uint32_t,
    mut dst: uint32_t,
) -> uint32_t {
    let src_a: uint8_t = (src >> 3 as libc::c_int * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    return src
        .wrapping_add(
            ChannelwiseMultiply(
                dst,
                (256 as libc::c_int - src_a as libc::c_int) as uint32_t,
            ),
        );
}
unsafe extern "C" fn BlendPixelRowPremult(
    src: *mut uint32_t,
    dst: *const uint32_t,
    mut num_pixels: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_pixels {
        let src_alpha: uint8_t = (*src.offset(i as isize)
            >> 3 as libc::c_int * 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        if src_alpha as libc::c_int != 0xff as libc::c_int {
            *src
                .offset(
                    i as isize,
                ) = BlendPixelPremult(*src.offset(i as isize), *dst.offset(i as isize));
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn FindBlendRangeAtRow(
    src: *const WebPIterator,
    dst: *const WebPIterator,
    mut canvas_y: libc::c_int,
    left1: *mut libc::c_int,
    width1: *mut libc::c_int,
    left2: *mut libc::c_int,
    width2: *mut libc::c_int,
) {
    let src_max_x: libc::c_int = (*src).x_offset + (*src).width;
    let dst_max_x: libc::c_int = (*dst).x_offset + (*dst).width;
    let dst_max_y: libc::c_int = (*dst).y_offset + (*dst).height;
    *left1 = -(1 as libc::c_int);
    *width1 = 0 as libc::c_int;
    *left2 = -(1 as libc::c_int);
    *width2 = 0 as libc::c_int;
    if canvas_y < (*dst).y_offset || canvas_y >= dst_max_y
        || (*src).x_offset >= dst_max_x || src_max_x <= (*dst).x_offset
    {
        *left1 = (*src).x_offset;
        *width1 = (*src).width;
        return;
    }
    if (*src).x_offset < (*dst).x_offset {
        *left1 = (*src).x_offset;
        *width1 = (*dst).x_offset - (*src).x_offset;
    }
    if src_max_x > dst_max_x {
        *left2 = dst_max_x;
        *width2 = src_max_x - dst_max_x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderGetNext(
    mut dec: *mut WebPAnimDecoder,
    mut buf_ptr: *mut *mut uint8_t,
    mut timestamp_ptr: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iter: WebPIterator = WebPIterator {
        frame_num: 0,
        num_frames: 0,
        x_offset: 0,
        y_offset: 0,
        width: 0,
        height: 0,
        duration: 0,
        dispose_method: WEBP_MUX_DISPOSE_NONE,
        complete: 0,
        fragment: WebPData {
            bytes: 0 as *const uint8_t,
            size: 0,
        },
        has_alpha: 0,
        blend_method: WEBP_MUX_BLEND,
        pad: [0; 2],
        private_: 0 as *mut libc::c_void,
    };
    let mut width: uint32_t = 0;
    let mut height: uint32_t = 0;
    let mut is_key_frame: libc::c_int = 0;
    let mut timestamp: libc::c_int = 0;
    let mut blend_row: BlendRowFunc = None;
    if dec.is_null() || buf_ptr.is_null() || timestamp_ptr.is_null() {
        return 0 as libc::c_int;
    }
    if WebPAnimDecoderHasMoreFrames(dec) == 0 {
        return 0 as libc::c_int;
    }
    width = (*dec).info_.canvas_width;
    height = (*dec).info_.canvas_height;
    blend_row = (*dec).blend_func_;
    if WebPDemuxGetFrame((*dec).demux_, (*dec).next_frame_, &mut iter) == 0 {
        return 0 as libc::c_int;
    }
    timestamp = (*dec).prev_frame_timestamp_ + iter.duration;
    is_key_frame = IsKeyFrame(
        &mut iter,
        &mut (*dec).prev_iter_,
        (*dec).prev_frame_was_keyframe_,
        width as libc::c_int,
        height as libc::c_int,
    );
    if is_key_frame != 0 {
        if ZeroFillCanvas((*dec).curr_frame_, width, height) == 0 {
            current_block = 12770081554652077468;
        } else {
            current_block = 7976072742316086414;
        }
    } else if CopyCanvas((*dec).prev_frame_disposed_, (*dec).curr_frame_, width, height)
        == 0
    {
        current_block = 12770081554652077468;
    } else {
        current_block = 7976072742316086414;
    }
    match current_block {
        7976072742316086414 => {
            let mut in_0: *const uint8_t = iter.fragment.bytes;
            let in_size: size_t = iter.fragment.size;
            let stride: uint32_t = width.wrapping_mul(4 as libc::c_int as libc::c_uint);
            let out_offset: uint64_t = (iter.y_offset as uint64_t)
                .wrapping_mul(stride as libc::c_ulong)
                .wrapping_add(
                    (iter.x_offset as uint64_t)
                        .wrapping_mul(4 as libc::c_int as libc::c_ulong),
                );
            let size: uint64_t = (iter.height as uint64_t)
                .wrapping_mul(stride as libc::c_ulong);
            let config: *mut WebPDecoderConfig = &mut (*dec).config_;
            let buf: *mut WebPRGBABuffer = &mut (*config).output.u.RGBA;
            if !(size != size) {
                (*buf).stride = stride as libc::c_int;
                (*buf).size = size;
                (*buf).rgba = ((*dec).curr_frame_).offset(out_offset as isize);
                if !(WebPDecode(in_0, in_size, config) as libc::c_uint
                    != VP8_STATUS_OK as libc::c_int as libc::c_uint)
                {
                    if iter.frame_num > 1 as libc::c_int
                        && iter.blend_method as libc::c_uint
                            == WEBP_MUX_BLEND as libc::c_int as libc::c_uint
                        && is_key_frame == 0
                    {
                        if (*dec).prev_iter_.dispose_method as libc::c_uint
                            == WEBP_MUX_DISPOSE_NONE as libc::c_int as libc::c_uint
                        {
                            let mut y: libc::c_int = 0;
                            y = 0 as libc::c_int;
                            while y < iter.height {
                                let offset: size_t = ((iter.y_offset + y) as libc::c_uint)
                                    .wrapping_mul(width)
                                    .wrapping_add(iter.x_offset as libc::c_uint) as size_t;
                                blend_row
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ((*dec).curr_frame_ as *mut uint32_t)
                                        .offset(offset as isize),
                                    ((*dec).prev_frame_disposed_ as *mut uint32_t)
                                        .offset(offset as isize),
                                    iter.width,
                                );
                                y += 1;
                                y;
                            }
                        } else {
                            let mut y_0: libc::c_int = 0;
                            y_0 = 0 as libc::c_int;
                            while y_0 < iter.height {
                                let canvas_y: libc::c_int = iter.y_offset + y_0;
                                let mut left1: libc::c_int = 0;
                                let mut width1: libc::c_int = 0;
                                let mut left2: libc::c_int = 0;
                                let mut width2: libc::c_int = 0;
                                FindBlendRangeAtRow(
                                    &mut iter,
                                    &mut (*dec).prev_iter_,
                                    canvas_y,
                                    &mut left1,
                                    &mut width1,
                                    &mut left2,
                                    &mut width2,
                                );
                                if width1 > 0 as libc::c_int {
                                    let offset1: size_t = (canvas_y as libc::c_uint)
                                        .wrapping_mul(width)
                                        .wrapping_add(left1 as libc::c_uint) as size_t;
                                    blend_row
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        ((*dec).curr_frame_ as *mut uint32_t)
                                            .offset(offset1 as isize),
                                        ((*dec).prev_frame_disposed_ as *mut uint32_t)
                                            .offset(offset1 as isize),
                                        width1,
                                    );
                                }
                                if width2 > 0 as libc::c_int {
                                    let offset2: size_t = (canvas_y as libc::c_uint)
                                        .wrapping_mul(width)
                                        .wrapping_add(left2 as libc::c_uint) as size_t;
                                    blend_row
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        ((*dec).curr_frame_ as *mut uint32_t)
                                            .offset(offset2 as isize),
                                        ((*dec).prev_frame_disposed_ as *mut uint32_t)
                                            .offset(offset2 as isize),
                                        width2,
                                    );
                                }
                                y_0 += 1;
                                y_0;
                            }
                        }
                    }
                    (*dec).prev_frame_timestamp_ = timestamp;
                    WebPDemuxReleaseIterator(&mut (*dec).prev_iter_);
                    (*dec).prev_iter_ = iter;
                    (*dec).prev_frame_was_keyframe_ = is_key_frame;
                    if !(CopyCanvas(
                        (*dec).curr_frame_,
                        (*dec).prev_frame_disposed_,
                        width,
                        height,
                    ) == 0)
                    {
                        if (*dec).prev_iter_.dispose_method as libc::c_uint
                            == WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int as libc::c_uint
                        {
                            ZeroFillFrameRect(
                                (*dec).prev_frame_disposed_,
                                width.wrapping_mul(4 as libc::c_int as libc::c_uint)
                                    as libc::c_int,
                                (*dec).prev_iter_.x_offset,
                                (*dec).prev_iter_.y_offset,
                                (*dec).prev_iter_.width,
                                (*dec).prev_iter_.height,
                            );
                        }
                        (*dec).next_frame_ += 1;
                        (*dec).next_frame_;
                        *buf_ptr = (*dec).curr_frame_;
                        *timestamp_ptr = timestamp;
                        return 1 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    WebPDemuxReleaseIterator(&mut iter);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderHasMoreFrames(
    mut dec: *const WebPAnimDecoder,
) -> libc::c_int {
    if dec.is_null() {
        return 0 as libc::c_int;
    }
    return ((*dec).next_frame_ <= (*dec).info_.frame_count as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderReset(mut dec: *mut WebPAnimDecoder) {
    if !dec.is_null() {
        (*dec).prev_frame_timestamp_ = 0 as libc::c_int;
        WebPDemuxReleaseIterator(&mut (*dec).prev_iter_);
        memset(
            &mut (*dec).prev_iter_ as *mut WebPIterator as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<WebPIterator>() as libc::c_ulong,
        );
        (*dec).prev_frame_was_keyframe_ = 0 as libc::c_int;
        (*dec).next_frame_ = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderGetDemuxer(
    mut dec: *const WebPAnimDecoder,
) -> *const WebPDemuxer {
    if dec.is_null() {
        return 0 as *const WebPDemuxer;
    }
    return (*dec).demux_;
}
#[no_mangle]
pub unsafe extern "C" fn WebPAnimDecoderDelete(mut dec: *mut WebPAnimDecoder) {
    if !dec.is_null() {
        WebPDemuxReleaseIterator(&mut (*dec).prev_iter_);
        WebPDemuxDelete((*dec).demux_);
        WebPSafeFree((*dec).curr_frame_ as *mut libc::c_void);
        WebPSafeFree((*dec).prev_frame_disposed_ as *mut libc::c_void);
        WebPSafeFree(dec as *mut libc::c_void);
    }
}
