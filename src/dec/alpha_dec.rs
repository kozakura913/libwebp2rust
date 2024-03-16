use ::libc;
extern "C" {
    fn WebPInitCustomIo(params: *mut WebPDecParams, io: *mut VP8Io);
    fn VP8InitIoInternal(_: *mut VP8Io, _: libc::c_int) -> libc::c_int;
    fn VP8FiltersInit();
    static mut WebPUnfilters: [WebPUnfilterFunc; 4];
    fn VP8LDelete(dec: *mut VP8LDecoder);
    fn VP8LDecodeAlphaHeader(
        alph_dec: *mut ALPHDecoder,
        data: *const uint8_t,
        data_size: size_t,
    ) -> libc::c_int;
    fn VP8LDecodeAlphaImageStream(
        alph_dec: *mut ALPHDecoder,
        last_row: libc::c_int,
    ) -> libc::c_int;
    fn VP8SetError(
        dec: *mut VP8Decoder,
        error: VP8StatusCode,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn WebPDequantizeLevels(
        data: *mut uint8_t,
        width: libc::c_int,
        height: libc::c_int,
        stride: libc::c_int,
        strength: libc::c_int,
    ) -> libc::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRGBABuffer {
    pub rgba: *mut uint8_t,
    pub stride: libc::c_int,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPYUVABuffer {
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub a: *mut uint8_t,
    pub y_stride: libc::c_int,
    pub u_stride: libc::c_int,
    pub v_stride: libc::c_int,
    pub a_stride: libc::c_int,
    pub y_size: size_t,
    pub u_size: size_t,
    pub v_size: size_t,
    pub a_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub is_external_memory: libc::c_int,
    pub u: C2RustUnnamed,
    pub pad: [uint32_t; 4],
    pub private_memory: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Io {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mb_y: libc::c_int,
    pub mb_w: libc::c_int,
    pub mb_h: libc::c_int,
    pub y: *const uint8_t,
    pub u: *const uint8_t,
    pub v: *const uint8_t,
    pub y_stride: libc::c_int,
    pub uv_stride: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub put: VP8IoPutHook,
    pub setup: VP8IoSetupHook,
    pub teardown: VP8IoTeardownHook,
    pub fancy_upsampling: libc::c_int,
    pub data_size: size_t,
    pub data: *const uint8_t,
    pub bypass_filtering: libc::c_int,
    pub use_cropping: libc::c_int,
    pub crop_left: libc::c_int,
    pub crop_right: libc::c_int,
    pub crop_top: libc::c_int,
    pub crop_bottom: libc::c_int,
    pub use_scaling: libc::c_int,
    pub scaled_width: libc::c_int,
    pub scaled_height: libc::c_int,
    pub a: *const uint8_t,
}
pub type VP8IoTeardownHook = Option::<unsafe extern "C" fn(*const VP8Io) -> ()>;
pub type VP8IoSetupHook = Option::<unsafe extern "C" fn(*mut VP8Io) -> libc::c_int>;
pub type VP8IoPutHook = Option::<unsafe extern "C" fn(*const VP8Io) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Decoder {
    pub status_: VP8StatusCode,
    pub ready_: libc::c_int,
    pub error_msg_: *const libc::c_char,
    pub br_: VP8BitReader,
    pub incremental_: libc::c_int,
    pub frm_hdr_: VP8FrameHeader,
    pub pic_hdr_: VP8PictureHeader,
    pub filter_hdr_: VP8FilterHeader,
    pub segment_hdr_: VP8SegmentHeader,
    pub worker_: WebPWorker,
    pub mt_method_: libc::c_int,
    pub cache_id_: libc::c_int,
    pub num_caches_: libc::c_int,
    pub thread_ctx_: VP8ThreadContext,
    pub mb_w_: libc::c_int,
    pub mb_h_: libc::c_int,
    pub tl_mb_x_: libc::c_int,
    pub tl_mb_y_: libc::c_int,
    pub br_mb_x_: libc::c_int,
    pub br_mb_y_: libc::c_int,
    pub num_parts_minus_one_: uint32_t,
    pub parts_: [VP8BitReader; 8],
    pub dither_: libc::c_int,
    pub dithering_rg_: VP8Random,
    pub dqm_: [VP8QuantMatrix; 4],
    pub proba_: VP8Proba,
    pub use_skip_proba_: libc::c_int,
    pub skip_p_: uint8_t,
    pub intra_t_: *mut uint8_t,
    pub intra_l_: [uint8_t; 4],
    pub yuv_t_: *mut VP8TopSamples,
    pub mb_info_: *mut VP8MB,
    pub f_info_: *mut VP8FInfo,
    pub yuv_b_: *mut uint8_t,
    pub cache_y_: *mut uint8_t,
    pub cache_u_: *mut uint8_t,
    pub cache_v_: *mut uint8_t,
    pub cache_y_stride_: libc::c_int,
    pub cache_uv_stride_: libc::c_int,
    pub mem_: *mut libc::c_void,
    pub mem_size_: size_t,
    pub mb_x_: libc::c_int,
    pub mb_y_: libc::c_int,
    pub mb_data_: *mut VP8MBData,
    pub filter_type_: libc::c_int,
    pub fstrengths_: [[VP8FInfo; 2]; 4],
    pub alph_dec_: *mut ALPHDecoder,
    pub alpha_data_: *const uint8_t,
    pub alpha_data_size_: size_t,
    pub is_alpha_decoded_: libc::c_int,
    pub alpha_plane_mem_: *mut uint8_t,
    pub alpha_plane_: *mut uint8_t,
    pub alpha_prev_line_: *const uint8_t,
    pub alpha_dithering_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ALPHDecoder {
    pub width_: libc::c_int,
    pub height_: libc::c_int,
    pub method_: libc::c_int,
    pub filter_: WEBP_FILTER_TYPE,
    pub pre_processing_: libc::c_int,
    pub vp8l_dec_: *mut VP8LDecoder,
    pub io_: VP8Io,
    pub use_8b_decode_: libc::c_int,
    pub output_: *mut uint8_t,
    pub prev_line_: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LDecoder {
    pub status_: VP8StatusCode,
    pub state_: VP8LDecodeState,
    pub io_: *mut VP8Io,
    pub output_: *const WebPDecBuffer,
    pub pixels_: *mut uint32_t,
    pub argb_cache_: *mut uint32_t,
    pub br_: VP8LBitReader,
    pub incremental_: libc::c_int,
    pub saved_br_: VP8LBitReader,
    pub saved_last_pixel_: libc::c_int,
    pub width_: libc::c_int,
    pub height_: libc::c_int,
    pub last_row_: libc::c_int,
    pub last_pixel_: libc::c_int,
    pub last_out_row_: libc::c_int,
    pub hdr_: VP8LMetadata,
    pub next_transform_: libc::c_int,
    pub transforms_: [VP8LTransform; 4],
    pub transforms_seen_: uint32_t,
    pub rescaler_memory: *mut uint8_t,
    pub rescaler: *mut WebPRescaler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LTransform {
    pub type_: VP8LImageTransformType,
    pub bits_: libc::c_int,
    pub xsize_: libc::c_int,
    pub ysize_: libc::c_int,
    pub data_: *mut uint32_t,
}
pub type VP8LImageTransformType = libc::c_uint;
pub const COLOR_INDEXING_TRANSFORM: VP8LImageTransformType = 3;
pub const SUBTRACT_GREEN_TRANSFORM: VP8LImageTransformType = 2;
pub const CROSS_COLOR_TRANSFORM: VP8LImageTransformType = 1;
pub const PREDICTOR_TRANSFORM: VP8LImageTransformType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMetadata {
    pub color_cache_size_: libc::c_int,
    pub color_cache_: VP8LColorCache,
    pub saved_color_cache_: VP8LColorCache,
    pub huffman_mask_: libc::c_int,
    pub huffman_subsample_bits_: libc::c_int,
    pub huffman_xsize_: libc::c_int,
    pub huffman_image_: *mut uint32_t,
    pub num_htree_groups_: libc::c_int,
    pub htree_groups_: *mut HTreeGroup,
    pub huffman_tables_: HuffmanTables,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTables {
    pub root: HuffmanTablesSegment,
    pub curr_segment: *mut HuffmanTablesSegment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTablesSegment {
    pub start: *mut HuffmanCode,
    pub curr_table: *mut HuffmanCode,
    pub next: *mut HuffmanTablesSegment,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTreeGroup {
    pub htrees: [*mut HuffmanCode; 5],
    pub is_trivial_literal: libc::c_int,
    pub literal_arb: uint32_t,
    pub is_trivial_code: libc::c_int,
    pub use_packed_table: libc::c_int,
    pub packed_table: [HuffmanCode32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode32 {
    pub bits: libc::c_int,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LColorCache {
    pub colors_: *mut uint32_t,
    pub hash_shift_: libc::c_int,
    pub hash_bits_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitReader {
    pub val_: vp8l_val_t,
    pub buf_: *const uint8_t,
    pub len_: size_t,
    pub pos_: size_t,
    pub bit_pos_: libc::c_int,
    pub eos_: libc::c_int,
}
pub type vp8l_val_t = uint64_t;
pub type VP8LDecodeState = libc::c_uint;
pub const READ_DIM: VP8LDecodeState = 2;
pub const READ_HDR: VP8LDecodeState = 1;
pub const READ_DATA: VP8LDecodeState = 0;
pub type WEBP_FILTER_TYPE = libc::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8FInfo {
    pub f_limit_: uint8_t,
    pub f_ilevel_: uint8_t,
    pub f_inner_: uint8_t,
    pub hev_thresh_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8MBData {
    pub coeffs_: [int16_t; 384],
    pub is_i4x4_: uint8_t,
    pub imodes_: [uint8_t; 16],
    pub uvmode_: uint8_t,
    pub non_zero_y_: uint32_t,
    pub non_zero_uv_: uint32_t,
    pub dither_: uint8_t,
    pub skip_: uint8_t,
    pub segment_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8MB {
    pub nz_: uint8_t,
    pub nz_dc_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8TopSamples {
    pub y: [uint8_t; 16],
    pub u: [uint8_t; 8],
    pub v: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Proba {
    pub segments_: [uint8_t; 3],
    pub bands_: [[VP8BandProbas; 8]; 4],
    pub bands_ptr_: [[*const VP8BandProbas; 17]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BandProbas {
    pub probas_: [VP8ProbaArray; 3],
}
pub type VP8ProbaArray = [uint8_t; 11];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8QuantMatrix {
    pub y1_mat_: quant_t,
    pub y2_mat_: quant_t,
    pub uv_mat_: quant_t,
    pub uv_quant_: libc::c_int,
    pub dither_: libc::c_int,
}
pub type quant_t = [libc::c_int; 2];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Random {
    pub index1_: libc::c_int,
    pub index2_: libc::c_int,
    pub tab_: [uint32_t; 55],
    pub amp_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BitReader {
    pub value_: bit_t,
    pub range_: range_t,
    pub bits_: libc::c_int,
    pub buf_: *const uint8_t,
    pub buf_end_: *const uint8_t,
    pub buf_max_: *const uint8_t,
    pub eof_: libc::c_int,
}
pub type range_t = uint32_t;
pub type bit_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8ThreadContext {
    pub id_: libc::c_int,
    pub mb_y_: libc::c_int,
    pub filter_row_: libc::c_int,
    pub f_info_: *mut VP8FInfo,
    pub mb_data_: *mut VP8MBData,
    pub io_: VP8Io,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorker {
    pub impl_: *mut libc::c_void,
    pub status_: WebPWorkerStatus,
    pub hook: WebPWorkerHook,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
    pub had_error: libc::c_int,
}
pub type WebPWorkerHook = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
pub type WebPWorkerStatus = libc::c_uint;
pub const WORK: WebPWorkerStatus = 2;
pub const OK: WebPWorkerStatus = 1;
pub const NOT_OK: WebPWorkerStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8SegmentHeader {
    pub use_segment_: libc::c_int,
    pub update_map_: libc::c_int,
    pub absolute_delta_: libc::c_int,
    pub quantizer_: [int8_t; 4],
    pub filter_strength_: [int8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8FilterHeader {
    pub simple_: libc::c_int,
    pub level_: libc::c_int,
    pub sharpness_: libc::c_int,
    pub use_lf_delta_: libc::c_int,
    pub ref_lf_delta_: [libc::c_int; 4],
    pub mode_lf_delta_: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8PictureHeader {
    pub width_: uint16_t,
    pub height_: uint16_t,
    pub xscale_: uint8_t,
    pub yscale_: uint8_t,
    pub colorspace_: uint8_t,
    pub clamp_type_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8FrameHeader {
    pub key_frame_: uint8_t,
    pub profile_: uint8_t,
    pub show_: uint8_t,
    pub partition_length_: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecParams {
    pub output: *mut WebPDecBuffer,
    pub tmp_y: *mut uint8_t,
    pub tmp_u: *mut uint8_t,
    pub tmp_v: *mut uint8_t,
    pub last_y: libc::c_int,
    pub options: *const WebPDecoderOptions,
    pub scaler_y: *mut WebPRescaler,
    pub scaler_u: *mut WebPRescaler,
    pub scaler_v: *mut WebPRescaler,
    pub scaler_a: *mut WebPRescaler,
    pub memory: *mut libc::c_void,
    pub emit: OutputFunc,
    pub emit_alpha: OutputAlphaFunc,
    pub emit_alpha_row: OutputRowFunc,
}
pub type OutputRowFunc = Option::<
    unsafe extern "C" fn(*mut WebPDecParams, libc::c_int, libc::c_int) -> libc::c_int,
>;
pub type OutputAlphaFunc = Option::<
    unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams, libc::c_int) -> libc::c_int,
>;
pub type OutputFunc = Option::<
    unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> libc::c_int,
>;
pub type WebPUnfilterFunc = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut uint8_t, libc::c_int) -> (),
>;
#[inline]
unsafe extern "C" fn VP8InitIo(io: *mut VP8Io) -> libc::c_int {
    return VP8InitIoInternal(io, 0x209 as libc::c_int);
}
unsafe extern "C" fn ALPHNew() -> *mut ALPHDecoder {
    let dec: *mut ALPHDecoder = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<ALPHDecoder>() as libc::c_ulong,
    ) as *mut ALPHDecoder;
    return dec;
}
unsafe extern "C" fn ALPHDelete(dec: *mut ALPHDecoder) {
    if !dec.is_null() {
        VP8LDelete((*dec).vp8l_dec_);
        (*dec).vp8l_dec_ = 0 as *mut VP8LDecoder;
        WebPSafeFree(dec as *mut libc::c_void);
    }
}
unsafe extern "C" fn ALPHInit(
    dec: *mut ALPHDecoder,
    mut data: *const uint8_t,
    mut data_size: size_t,
    src_io: *const VP8Io,
    mut output: *mut uint8_t,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let alpha_data: *const uint8_t = data.offset(1 as libc::c_int as isize);
    let alpha_data_size: size_t = data_size
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut rsrv: libc::c_int = 0;
    let io: *mut VP8Io = &mut (*dec).io_;
    VP8FiltersInit();
    (*dec).output_ = output;
    (*dec).width_ = (*src_io).width;
    (*dec).height_ = (*src_io).height;
    if data_size <= 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*dec)
        .method_ = *data.offset(0 as libc::c_int as isize) as libc::c_int
        >> 0 as libc::c_int & 0x3 as libc::c_int;
    (*dec)
        .filter_ = (*data.offset(0 as libc::c_int as isize) as libc::c_int
        >> 2 as libc::c_int & 0x3 as libc::c_int) as WEBP_FILTER_TYPE;
    (*dec)
        .pre_processing_ = *data.offset(0 as libc::c_int as isize) as libc::c_int
        >> 4 as libc::c_int & 0x3 as libc::c_int;
    rsrv = *data.offset(0 as libc::c_int as isize) as libc::c_int >> 6 as libc::c_int
        & 0x3 as libc::c_int;
    if (*dec).method_ < 0 as libc::c_int || (*dec).method_ > 1 as libc::c_int
        || (*dec).filter_ as libc::c_uint
            >= WEBP_FILTER_LAST as libc::c_int as libc::c_uint
        || (*dec).pre_processing_ > 1 as libc::c_int || rsrv != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if VP8InitIo(io) == 0 {
        return 0 as libc::c_int;
    }
    WebPInitCustomIo(0 as *mut WebPDecParams, io);
    (*io).opaque = dec as *mut libc::c_void;
    (*io).width = (*src_io).width;
    (*io).height = (*src_io).height;
    (*io).use_cropping = (*src_io).use_cropping;
    (*io).crop_left = (*src_io).crop_left;
    (*io).crop_right = (*src_io).crop_right;
    (*io).crop_top = (*src_io).crop_top;
    (*io).crop_bottom = (*src_io).crop_bottom;
    if (*dec).method_ == 0 as libc::c_int {
        let alpha_decoded_size: size_t = ((*dec).width_ * (*dec).height_) as size_t;
        ok = (alpha_data_size >= alpha_decoded_size) as libc::c_int;
    } else {
        ok = VP8LDecodeAlphaHeader(dec, alpha_data, alpha_data_size);
    }
    return ok;
}
unsafe extern "C" fn ALPHDecode(
    dec: *mut VP8Decoder,
    mut row: libc::c_int,
    mut num_rows: libc::c_int,
) -> libc::c_int {
    let alph_dec: *mut ALPHDecoder = (*dec).alph_dec_;
    let width: libc::c_int = (*alph_dec).width_;
    let height: libc::c_int = (*alph_dec).io_.crop_bottom;
    if (*alph_dec).method_ == 0 as libc::c_int {
        let mut y: libc::c_int = 0;
        let mut prev_line: *const uint8_t = (*dec).alpha_prev_line_;
        let mut deltas: *const uint8_t = ((*dec).alpha_data_)
            .offset(1 as libc::c_int as isize)
            .offset((row * width) as isize);
        let mut dst: *mut uint8_t = ((*dec).alpha_plane_).offset((row * width) as isize);
        y = 0 as libc::c_int;
        while y < num_rows {
            (WebPUnfilters[(*alph_dec).filter_ as usize])
                .expect("non-null function pointer")(prev_line, deltas, dst, width);
            prev_line = dst;
            dst = dst.offset(width as isize);
            deltas = deltas.offset(width as isize);
            y += 1;
            y;
        }
        (*dec).alpha_prev_line_ = prev_line;
    } else if VP8LDecodeAlphaImageStream(alph_dec, row + num_rows) == 0 {
        return 0 as libc::c_int
    }
    if row + num_rows >= height {
        (*dec).is_alpha_decoded_ = 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn AllocateAlphaPlane(
    dec: *mut VP8Decoder,
    io: *const VP8Io,
) -> libc::c_int {
    let stride: libc::c_int = (*io).width;
    let height: libc::c_int = (*io).crop_bottom;
    let alpha_size: uint64_t = (stride as uint64_t)
        .wrapping_mul(height as libc::c_ulong);
    (*dec)
        .alpha_plane_mem_ = WebPSafeMalloc(
        alpha_size,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    if ((*dec).alpha_plane_mem_).is_null() {
        return VP8SetError(
            dec,
            VP8_STATUS_OUT_OF_MEMORY,
            b"Alpha decoder initialization failed.\0" as *const u8 as *const libc::c_char,
        );
    }
    (*dec).alpha_plane_ = (*dec).alpha_plane_mem_;
    (*dec).alpha_prev_line_ = 0 as *const uint8_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPDeallocateAlphaMemory(dec: *mut VP8Decoder) {
    WebPSafeFree((*dec).alpha_plane_mem_ as *mut libc::c_void);
    (*dec).alpha_plane_mem_ = 0 as *mut uint8_t;
    (*dec).alpha_plane_ = 0 as *mut uint8_t;
    ALPHDelete((*dec).alph_dec_);
    (*dec).alph_dec_ = 0 as *mut ALPHDecoder;
}
#[no_mangle]
pub unsafe extern "C" fn VP8DecompressAlphaRows(
    dec: *mut VP8Decoder,
    io: *const VP8Io,
    mut row: libc::c_int,
    mut num_rows: libc::c_int,
) -> *const uint8_t {
    let mut current_block: u64;
    let width: libc::c_int = (*io).width;
    let height: libc::c_int = (*io).crop_bottom;
    if row < 0 as libc::c_int || num_rows <= 0 as libc::c_int || row + num_rows > height
    {
        return 0 as *const uint8_t;
    }
    if (*dec).is_alpha_decoded_ == 0 {
        if ((*dec).alph_dec_).is_null() {
            (*dec).alph_dec_ = ALPHNew();
            if ((*dec).alph_dec_).is_null() {
                VP8SetError(
                    dec,
                    VP8_STATUS_OUT_OF_MEMORY,
                    b"Alpha decoder initialization failed.\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as *const uint8_t;
            }
            if AllocateAlphaPlane(dec, io) == 0 {
                current_block = 11666549232534811887;
            } else if ALPHInit(
                (*dec).alph_dec_,
                (*dec).alpha_data_,
                (*dec).alpha_data_size_,
                io,
                (*dec).alpha_plane_,
            ) == 0
            {
                let vp8l_dec: *mut VP8LDecoder = (*(*dec).alph_dec_).vp8l_dec_;
                VP8SetError(
                    dec,
                    (if vp8l_dec.is_null() {
                        VP8_STATUS_OUT_OF_MEMORY as libc::c_int as libc::c_uint
                    } else {
                        (*vp8l_dec).status_ as libc::c_uint
                    }) as VP8StatusCode,
                    b"Alpha decoder initialization failed.\0" as *const u8
                        as *const libc::c_char,
                );
                current_block = 11666549232534811887;
            } else {
                if (*(*dec).alph_dec_).pre_processing_ != 1 as libc::c_int {
                    (*dec).alpha_dithering_ = 0 as libc::c_int;
                } else {
                    num_rows = height - row;
                }
                current_block = 15976848397966268834;
            }
        } else {
            current_block = 15976848397966268834;
        }
        match current_block {
            15976848397966268834 => {
                if ALPHDecode(dec, row, num_rows) == 0 {
                    current_block = 11666549232534811887;
                } else if (*dec).is_alpha_decoded_ != 0 {
                    ALPHDelete((*dec).alph_dec_);
                    (*dec).alph_dec_ = 0 as *mut ALPHDecoder;
                    if (*dec).alpha_dithering_ > 0 as libc::c_int {
                        let alpha: *mut uint8_t = ((*dec).alpha_plane_)
                            .offset(((*io).crop_top * width) as isize)
                            .offset((*io).crop_left as isize);
                        if WebPDequantizeLevels(
                            alpha,
                            (*io).crop_right - (*io).crop_left,
                            (*io).crop_bottom - (*io).crop_top,
                            width,
                            (*dec).alpha_dithering_,
                        ) == 0
                        {
                            current_block = 11666549232534811887;
                        } else {
                            current_block = 17478428563724192186;
                        }
                    } else {
                        current_block = 17478428563724192186;
                    }
                } else {
                    current_block = 17478428563724192186;
                }
            }
            _ => {}
        }
        match current_block {
            17478428563724192186 => {}
            _ => {
                WebPDeallocateAlphaMemory(dec);
                return 0 as *const uint8_t;
            }
        }
    }
    return ((*dec).alpha_plane_).offset((row * width) as isize);
}
