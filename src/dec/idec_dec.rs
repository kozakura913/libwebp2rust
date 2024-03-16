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
    fn WebPInitDecBufferInternal(_: *mut WebPDecBuffer, _: libc::c_int) -> libc::c_int;
    fn WebPFreeDecBuffer(buffer: *mut WebPDecBuffer);
    fn WebPInitCustomIo(params: *mut WebPDecParams, io: *mut VP8Io);
    fn WebPAvoidSlowMemory(
        output: *const WebPDecBuffer,
        features: *const WebPBitstreamFeatures,
    ) -> libc::c_int;
    fn WebPResetDecParams(params: *mut WebPDecParams);
    fn VP8InitIoInternal(_: *mut VP8Io, _: libc::c_int) -> libc::c_int;
    fn VP8Delete(dec: *mut VP8Decoder);
    fn WebPCopyDecBufferPixels(
        src: *const WebPDecBuffer,
        dst: *mut WebPDecBuffer,
    ) -> VP8StatusCode;
    fn WebPFlipBuffer(buffer: *mut WebPDecBuffer) -> VP8StatusCode;
    fn WebPAllocateDecBuffer(
        width: libc::c_int,
        height: libc::c_int,
        options: *const WebPDecoderOptions,
        buffer: *mut WebPDecBuffer,
    ) -> VP8StatusCode;
    fn VP8GetHeaders(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8GetInfo(
        data: *const uint8_t,
        data_size: size_t,
        chunk_size: size_t,
        width: *mut libc::c_int,
        height: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8New() -> *mut VP8Decoder;
    fn WebPParseHeaders(headers: *mut WebPHeaderStructure) -> VP8StatusCode;
    fn WebPGetFeaturesInternal(
        _: *const uint8_t,
        _: size_t,
        _: *mut WebPBitstreamFeatures,
        _: libc::c_int,
    ) -> VP8StatusCode;
    fn VP8RemapBitReader(br: *mut VP8BitReader, offset: ptrdiff_t);
    fn VP8LBitReaderSetBuffer(
        br: *mut VP8LBitReader,
        buffer: *const uint8_t,
        length: size_t,
    );
    fn VP8LNew() -> *mut VP8LDecoder;
    fn VP8GetThreadMethod(
        options: *const WebPDecoderOptions,
        headers: *const WebPHeaderStructure,
        width: libc::c_int,
        height: libc::c_int,
    ) -> libc::c_int;
    fn VP8InitDithering(options: *const WebPDecoderOptions, dec: *mut VP8Decoder);
    fn VP8BitReaderSetBuffer(br: *mut VP8BitReader, start: *const uint8_t, size: size_t);
    fn VP8EnterCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> VP8StatusCode;
    fn VP8InitFrame(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8ParseIntraModeRow(br: *mut VP8BitReader, dec: *mut VP8Decoder) -> libc::c_int;
    fn VP8DecodeMB(dec: *mut VP8Decoder, token_br: *mut VP8BitReader) -> libc::c_int;
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn VP8InitScanline(dec: *mut VP8Decoder);
    fn VP8ProcessRow(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8LDecodeHeader(dec: *mut VP8LDecoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8LDecodeImage(dec: *mut VP8LDecoder) -> libc::c_int;
    fn VP8ExitCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8LDelete(dec: *mut VP8LDecoder);
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
pub type ptrdiff_t = libc::c_long;
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
pub struct WebPIDecoder {
    pub state_: DecState,
    pub params_: WebPDecParams,
    pub is_lossless_: libc::c_int,
    pub dec_: *mut libc::c_void,
    pub io_: VP8Io,
    pub mem_: MemBuffer,
    pub output_: WebPDecBuffer,
    pub final_output_: *mut WebPDecBuffer,
    pub chunk_size_: size_t,
    pub last_mb_y_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemBuffer {
    pub mode_: MemBufferMode,
    pub start_: size_t,
    pub end_: size_t,
    pub buf_size_: size_t,
    pub buf_: *mut uint8_t,
    pub part0_size_: size_t,
    pub part0_buf_: *const uint8_t,
}
pub type MemBufferMode = libc::c_uint;
pub const MEM_MODE_MAP: MemBufferMode = 2;
pub const MEM_MODE_APPEND: MemBufferMode = 1;
pub const MEM_MODE_NONE: MemBufferMode = 0;
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
pub type DecState = libc::c_uint;
pub const STATE_ERROR: DecState = 7;
pub const STATE_DONE: DecState = 6;
pub const STATE_VP8L_DATA: DecState = 5;
pub const STATE_VP8L_HEADER: DecState = 4;
pub const STATE_VP8_DATA: DecState = 3;
pub const STATE_VP8_PARTS0: DecState = 2;
pub const STATE_VP8_HEADER: DecState = 1;
pub const STATE_WEBP_HEADER: DecState = 0;
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
pub struct WebPDecoderConfig {
    pub input: WebPBitstreamFeatures,
    pub output: WebPDecBuffer,
    pub options: WebPDecoderOptions,
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
pub struct MBContext {
    pub left_: VP8MB,
    pub info_: VP8MB,
    pub token_br_: VP8BitReader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorkerInterface {
    pub Init: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Reset: Option::<unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int>,
    pub Sync_0: Option::<unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int>,
    pub Launch: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Execute: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub End: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPHeaderStructure {
    pub data: *const uint8_t,
    pub data_size: size_t,
    pub have_all_data: libc::c_int,
    pub offset: size_t,
    pub alpha_data: *const uint8_t,
    pub alpha_data_size: size_t,
    pub compressed_size: size_t,
    pub riff_size: size_t,
    pub is_lossless: libc::c_int,
}
#[inline]
unsafe extern "C" fn WebPInitDecBuffer(mut buffer: *mut WebPDecBuffer) -> libc::c_int {
    return WebPInitDecBufferInternal(buffer, 0x209 as libc::c_int);
}
#[inline]
unsafe extern "C" fn VP8InitIo(io: *mut VP8Io) -> libc::c_int {
    return VP8InitIoInternal(io, 0x209 as libc::c_int);
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
unsafe extern "C" fn MemDataSize(mut mem: *const MemBuffer) -> size_t {
    return ((*mem).end_).wrapping_sub((*mem).start_);
}
unsafe extern "C" fn NeedCompressedAlpha(idec: *const WebPIDecoder) -> libc::c_int {
    if (*idec).state_ as libc::c_uint == STATE_WEBP_HEADER as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*idec).is_lossless_ != 0 {
        return 0 as libc::c_int
    } else {
        let dec: *const VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
        return (!((*dec).alpha_data_).is_null() && (*dec).is_alpha_decoded_ == 0)
            as libc::c_int;
    };
}
unsafe extern "C" fn DoRemap(idec: *mut WebPIDecoder, mut offset: ptrdiff_t) {
    let mem: *mut MemBuffer = &mut (*idec).mem_;
    let new_base: *const uint8_t = ((*mem).buf_).offset((*mem).start_ as isize);
    (*idec).io_.data = new_base;
    (*idec).io_.data_size = MemDataSize(mem);
    if !((*idec).dec_).is_null() {
        if (*idec).is_lossless_ == 0 {
            let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
            let last_part: uint32_t = (*dec).num_parts_minus_one_;
            if offset != 0 as libc::c_int as libc::c_long {
                let mut p: uint32_t = 0;
                p = 0 as libc::c_int as uint32_t;
                while p <= last_part {
                    VP8RemapBitReader(
                        ((*dec).parts_).as_mut_ptr().offset(p as isize),
                        offset,
                    );
                    p = p.wrapping_add(1);
                    p;
                }
                if (*mem).mode_ as libc::c_uint
                    == MEM_MODE_MAP as libc::c_int as libc::c_uint
                {
                    VP8RemapBitReader(&mut (*dec).br_, offset);
                }
            }
            let last_start: *const uint8_t = (*dec).parts_[last_part as usize].buf_;
            VP8BitReaderSetBuffer(
                &mut *((*dec).parts_).as_mut_ptr().offset(last_part as isize),
                last_start,
                ((*mem).buf_).offset((*mem).end_ as isize).offset_from(last_start)
                    as libc::c_long as size_t,
            );
            if NeedCompressedAlpha(idec) != 0 {
                let alph_dec: *mut ALPHDecoder = (*dec).alph_dec_;
                (*dec).alpha_data_ = ((*dec).alpha_data_).offset(offset as isize);
                if !alph_dec.is_null() && !((*alph_dec).vp8l_dec_).is_null() {
                    if (*alph_dec).method_ == 1 as libc::c_int {
                        let alph_vp8l_dec: *mut VP8LDecoder = (*alph_dec).vp8l_dec_;
                        VP8LBitReaderSetBuffer(
                            &mut (*alph_vp8l_dec).br_,
                            ((*dec).alpha_data_).offset(1 as libc::c_int as isize),
                            ((*dec).alpha_data_size_)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                    }
                }
            }
        } else {
            let dec_0: *mut VP8LDecoder = (*idec).dec_ as *mut VP8LDecoder;
            VP8LBitReaderSetBuffer(&mut (*dec_0).br_, new_base, MemDataSize(mem));
        }
    }
}
unsafe extern "C" fn AppendToMemBuffer(
    idec: *mut WebPIDecoder,
    data: *const uint8_t,
    mut data_size: size_t,
) -> libc::c_int {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let mem: *mut MemBuffer = &mut (*idec).mem_;
    let need_compressed_alpha: libc::c_int = NeedCompressedAlpha(idec);
    let old_start: *const uint8_t = if ((*mem).buf_).is_null() {
        0 as *mut uint8_t
    } else {
        ((*mem).buf_).offset((*mem).start_ as isize)
    };
    let old_base: *const uint8_t = if need_compressed_alpha != 0 {
        (*dec).alpha_data_
    } else {
        old_start
    };
    if data_size
        > (!(0 as libc::c_uint))
            .wrapping_sub(8 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if ((*mem).end_).wrapping_add(data_size) > (*mem).buf_size_ {
        let new_mem_start: size_t = old_start.offset_from(old_base) as libc::c_long
            as size_t;
        let current_size: size_t = (MemDataSize(mem)).wrapping_add(new_mem_start);
        let new_size: uint64_t = current_size.wrapping_add(data_size);
        let extra_size: uint64_t = new_size
            .wrapping_add(4096 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
        let new_buf: *mut uint8_t = WebPSafeMalloc(
            extra_size,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        ) as *mut uint8_t;
        if new_buf.is_null() {
            return 0 as libc::c_int;
        }
        if !old_base.is_null() {
            memcpy(
                new_buf as *mut libc::c_void,
                old_base as *const libc::c_void,
                current_size,
            );
        }
        WebPSafeFree((*mem).buf_ as *mut libc::c_void);
        (*mem).buf_ = new_buf;
        (*mem).buf_size_ = extra_size;
        (*mem).start_ = new_mem_start;
        (*mem).end_ = current_size;
    }
    memcpy(
        ((*mem).buf_).offset((*mem).end_ as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        data_size,
    );
    (*mem)
        .end_ = ((*mem).end_ as libc::c_ulong).wrapping_add(data_size) as size_t
        as size_t;
    DoRemap(
        idec,
        ((*mem).buf_).offset((*mem).start_ as isize).offset_from(old_start)
            as libc::c_long,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn RemapMemBuffer(
    idec: *mut WebPIDecoder,
    data: *const uint8_t,
    mut data_size: size_t,
) -> libc::c_int {
    let mem: *mut MemBuffer = &mut (*idec).mem_;
    let old_buf: *const uint8_t = (*mem).buf_;
    let old_start: *const uint8_t = if old_buf.is_null() {
        0 as *const uint8_t
    } else {
        old_buf.offset((*mem).start_ as isize)
    };
    if data_size < (*mem).buf_size_ {
        return 0 as libc::c_int;
    }
    (*mem).buf_ = data as *mut uint8_t;
    (*mem).buf_size_ = data_size;
    (*mem).end_ = (*mem).buf_size_;
    DoRemap(
        idec,
        ((*mem).buf_).offset((*mem).start_ as isize).offset_from(old_start)
            as libc::c_long,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn InitMemBuffer(mem: *mut MemBuffer) {
    (*mem).mode_ = MEM_MODE_NONE;
    (*mem).buf_ = 0 as *mut uint8_t;
    (*mem).buf_size_ = 0 as libc::c_int as size_t;
    (*mem).part0_buf_ = 0 as *const uint8_t;
    (*mem).part0_size_ = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn ClearMemBuffer(mem: *mut MemBuffer) {
    if (*mem).mode_ as libc::c_uint == MEM_MODE_APPEND as libc::c_int as libc::c_uint {
        WebPSafeFree((*mem).buf_ as *mut libc::c_void);
        WebPSafeFree((*mem).part0_buf_ as *mut libc::c_void);
    }
}
unsafe extern "C" fn CheckMemBufferMode(
    mem: *mut MemBuffer,
    mut expected: MemBufferMode,
) -> libc::c_int {
    if (*mem).mode_ as libc::c_uint == MEM_MODE_NONE as libc::c_int as libc::c_uint {
        (*mem).mode_ = expected;
    } else if (*mem).mode_ as libc::c_uint != expected as libc::c_uint {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn FinishDecoding(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let options: *const WebPDecoderOptions = (*idec).params_.options;
    let output: *mut WebPDecBuffer = (*idec).params_.output;
    (*idec).state_ = STATE_DONE;
    if !options.is_null() && (*options).flip != 0 {
        let status: VP8StatusCode = WebPFlipBuffer(output);
        if status as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
            return status;
        }
    }
    if !((*idec).final_output_).is_null() {
        let status_0: VP8StatusCode = WebPCopyDecBufferPixels(
            output,
            (*idec).final_output_,
        );
        WebPFreeDecBuffer(&mut (*idec).output_);
        if status_0 as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
            return status_0;
        }
        *output = *(*idec).final_output_;
        (*idec).final_output_ = 0 as *mut WebPDecBuffer;
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn SaveContext(
    mut dec: *const VP8Decoder,
    mut token_br: *const VP8BitReader,
    context: *mut MBContext,
) {
    (*context).left_ = *((*dec).mb_info_).offset(-(1 as libc::c_int) as isize);
    (*context).info_ = *((*dec).mb_info_).offset((*dec).mb_x_ as isize);
    (*context).token_br_ = *token_br;
}
unsafe extern "C" fn RestoreContext(
    mut context: *const MBContext,
    dec: *mut VP8Decoder,
    token_br: *mut VP8BitReader,
) {
    *((*dec).mb_info_).offset(-(1 as libc::c_int) as isize) = (*context).left_;
    *((*dec).mb_info_).offset((*dec).mb_x_ as isize) = (*context).info_;
    *token_br = (*context).token_br_;
}
unsafe extern "C" fn IDecError(
    idec: *mut WebPIDecoder,
    mut error: VP8StatusCode,
) -> VP8StatusCode {
    if (*idec).state_ as libc::c_uint == STATE_VP8_DATA as libc::c_int as libc::c_uint {
        VP8ExitCritical((*idec).dec_ as *mut VP8Decoder, &mut (*idec).io_);
    }
    (*idec).state_ = STATE_ERROR;
    return error;
}
unsafe extern "C" fn ChangeState(
    idec: *mut WebPIDecoder,
    mut new_state: DecState,
    mut consumed_bytes: size_t,
) {
    let mem: *mut MemBuffer = &mut (*idec).mem_;
    (*idec).state_ = new_state;
    (*mem)
        .start_ = ((*mem).start_ as libc::c_ulong).wrapping_add(consumed_bytes) as size_t
        as size_t;
    (*idec).io_.data = ((*mem).buf_).offset((*mem).start_ as isize);
    (*idec).io_.data_size = MemDataSize(mem);
}
unsafe extern "C" fn DecodeWebPHeaders(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let mem: *mut MemBuffer = &mut (*idec).mem_;
    let mut data: *const uint8_t = ((*mem).buf_).offset((*mem).start_ as isize);
    let mut curr_size: size_t = MemDataSize(mem);
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    let mut headers: WebPHeaderStructure = WebPHeaderStructure {
        data: 0 as *const uint8_t,
        data_size: 0,
        have_all_data: 0,
        offset: 0,
        alpha_data: 0 as *const uint8_t,
        alpha_data_size: 0,
        compressed_size: 0,
        riff_size: 0,
        is_lossless: 0,
    };
    headers.data = data;
    headers.data_size = curr_size;
    headers.have_all_data = 0 as libc::c_int;
    status = WebPParseHeaders(&mut headers);
    if status as libc::c_uint
        == VP8_STATUS_NOT_ENOUGH_DATA as libc::c_int as libc::c_uint
    {
        return VP8_STATUS_SUSPENDED
    } else if status as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
        return IDecError(idec, status)
    }
    (*idec).chunk_size_ = headers.compressed_size;
    (*idec).is_lossless_ = headers.is_lossless;
    if (*idec).is_lossless_ == 0 {
        let dec: *mut VP8Decoder = VP8New();
        if dec.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        (*dec).incremental_ = 1 as libc::c_int;
        (*idec).dec_ = dec as *mut libc::c_void;
        (*dec).alpha_data_ = headers.alpha_data;
        (*dec).alpha_data_size_ = headers.alpha_data_size;
        ChangeState(idec, STATE_VP8_HEADER, headers.offset);
    } else {
        let dec_0: *mut VP8LDecoder = VP8LNew();
        if dec_0.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        (*idec).dec_ = dec_0 as *mut libc::c_void;
        ChangeState(idec, STATE_VP8L_HEADER, headers.offset);
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodeVP8FrameHeader(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let mut data: *const uint8_t = ((*idec).mem_.buf_)
        .offset((*idec).mem_.start_ as isize);
    let curr_size: size_t = MemDataSize(&mut (*idec).mem_);
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut bits: uint32_t = 0;
    if curr_size < 10 as libc::c_int as libc::c_ulong {
        return VP8_STATUS_SUSPENDED;
    }
    if VP8GetInfo(data, curr_size, (*idec).chunk_size_, &mut width, &mut height) == 0 {
        return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
    }
    bits = (*data.offset(0 as libc::c_int as isize) as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | (*data.offset(2 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    (*idec)
        .mem_
        .part0_size_ = (bits >> 5 as libc::c_int)
        .wrapping_add(10 as libc::c_int as libc::c_uint) as size_t;
    (*idec).io_.data = data;
    (*idec).io_.data_size = curr_size;
    (*idec).state_ = STATE_VP8_PARTS0;
    return VP8_STATUS_OK;
}
unsafe extern "C" fn CopyParts0Data(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let br: *mut VP8BitReader = &mut (*dec).br_;
    let part_size: size_t = ((*br).buf_end_).offset_from((*br).buf_) as libc::c_long
        as size_t;
    let mem: *mut MemBuffer = &mut (*idec).mem_;
    if part_size == 0 as libc::c_int as libc::c_ulong {
        return VP8_STATUS_BITSTREAM_ERROR;
    }
    if (*mem).mode_ as libc::c_uint == MEM_MODE_APPEND as libc::c_int as libc::c_uint {
        let part0_buf: *mut uint8_t = WebPSafeMalloc(
            1 as libc::c_ulonglong as uint64_t,
            part_size,
        ) as *mut uint8_t;
        if part0_buf.is_null() {
            return VP8_STATUS_OUT_OF_MEMORY;
        }
        memcpy(
            part0_buf as *mut libc::c_void,
            (*br).buf_ as *const libc::c_void,
            part_size,
        );
        (*mem).part0_buf_ = part0_buf;
        VP8BitReaderSetBuffer(br, part0_buf, part_size);
    }
    (*mem)
        .start_ = ((*mem).start_ as libc::c_ulong).wrapping_add(part_size) as size_t
        as size_t;
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodePartition0(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let io: *mut VP8Io = &mut (*idec).io_;
    let params: *const WebPDecParams = &mut (*idec).params_;
    let output: *mut WebPDecBuffer = (*params).output;
    if MemDataSize(&mut (*idec).mem_) < (*idec).mem_.part0_size_ {
        return VP8_STATUS_SUSPENDED;
    }
    if VP8GetHeaders(dec, io) == 0 {
        let status: VP8StatusCode = (*dec).status_;
        if status as libc::c_uint == VP8_STATUS_SUSPENDED as libc::c_int as libc::c_uint
            || status as libc::c_uint
                == VP8_STATUS_NOT_ENOUGH_DATA as libc::c_int as libc::c_uint
        {
            return VP8_STATUS_SUSPENDED;
        }
        return IDecError(idec, status);
    }
    (*dec)
        .status_ = WebPAllocateDecBuffer(
        (*io).width,
        (*io).height,
        (*params).options,
        output,
    );
    if (*dec).status_ as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
        return IDecError(idec, (*dec).status_);
    }
    (*dec)
        .mt_method_ = VP8GetThreadMethod(
        (*params).options,
        0 as *const WebPHeaderStructure,
        (*io).width,
        (*io).height,
    );
    VP8InitDithering((*params).options, dec);
    (*dec).status_ = CopyParts0Data(idec);
    if (*dec).status_ as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
        return IDecError(idec, (*dec).status_);
    }
    if VP8EnterCritical(dec, io) as libc::c_uint
        != VP8_STATUS_OK as libc::c_int as libc::c_uint
    {
        return IDecError(idec, (*dec).status_);
    }
    (*idec).state_ = STATE_VP8_DATA;
    if VP8InitFrame(dec, io) == 0 {
        return IDecError(idec, (*dec).status_);
    }
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodeRemaining(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
    let io: *mut VP8Io = &mut (*idec).io_;
    if (*dec).ready_ == 0 {
        return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
    }
    while (*dec).mb_y_ < (*dec).mb_h_ {
        if (*idec).last_mb_y_ != (*dec).mb_y_ {
            if VP8ParseIntraModeRow(&mut (*dec).br_, dec) == 0 {
                return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
            }
            (*idec).last_mb_y_ = (*dec).mb_y_;
        }
        while (*dec).mb_x_ < (*dec).mb_w_ {
            let token_br: *mut VP8BitReader = &mut *((*dec).parts_)
                .as_mut_ptr()
                .offset(
                    ((*dec).mb_y_ as libc::c_uint & (*dec).num_parts_minus_one_) as isize,
                ) as *mut VP8BitReader;
            let mut context: MBContext = MBContext {
                left_: VP8MB { nz_: 0, nz_dc_: 0 },
                info_: VP8MB { nz_: 0, nz_dc_: 0 },
                token_br_: VP8BitReader {
                    value_: 0,
                    range_: 0,
                    bits_: 0,
                    buf_: 0 as *const uint8_t,
                    buf_end_: 0 as *const uint8_t,
                    buf_max_: 0 as *const uint8_t,
                    eof_: 0,
                },
            };
            SaveContext(dec, token_br, &mut context);
            if VP8DecodeMB(dec, token_br) == 0 {
                if (*dec).num_parts_minus_one_ == 0 as libc::c_int as libc::c_uint
                    && MemDataSize(&mut (*idec).mem_)
                        > 4096 as libc::c_int as libc::c_ulong
                {
                    return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
                }
                if (*dec).mt_method_ > 0 as libc::c_int {
                    if ((*WebPGetWorkerInterface()).Sync_0)
                        .expect("non-null function pointer")(&mut (*dec).worker_) == 0
                    {
                        return IDecError(idec, VP8_STATUS_BITSTREAM_ERROR);
                    }
                }
                RestoreContext(&mut context, dec, token_br);
                return VP8_STATUS_SUSPENDED;
            }
            if (*dec).num_parts_minus_one_ == 0 as libc::c_int as libc::c_uint {
                (*idec)
                    .mem_
                    .start_ = ((*token_br).buf_).offset_from((*idec).mem_.buf_)
                    as libc::c_long as size_t;
            }
            (*dec).mb_x_ += 1;
            (*dec).mb_x_;
        }
        VP8InitScanline(dec);
        if VP8ProcessRow(dec, io) == 0 {
            return IDecError(idec, VP8_STATUS_USER_ABORT);
        }
        (*dec).mb_y_ += 1;
        (*dec).mb_y_;
    }
    if VP8ExitCritical(dec, io) == 0 {
        (*idec).state_ = STATE_ERROR;
        return IDecError(idec, VP8_STATUS_USER_ABORT);
    }
    (*dec).ready_ = 0 as libc::c_int;
    return FinishDecoding(idec);
}
unsafe extern "C" fn ErrorStatusLossless(
    idec: *mut WebPIDecoder,
    mut status: VP8StatusCode,
) -> VP8StatusCode {
    if status as libc::c_uint == VP8_STATUS_SUSPENDED as libc::c_int as libc::c_uint
        || status as libc::c_uint
            == VP8_STATUS_NOT_ENOUGH_DATA as libc::c_int as libc::c_uint
    {
        return VP8_STATUS_SUSPENDED;
    }
    return IDecError(idec, status);
}
unsafe extern "C" fn DecodeVP8LHeader(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let io: *mut VP8Io = &mut (*idec).io_;
    let dec: *mut VP8LDecoder = (*idec).dec_ as *mut VP8LDecoder;
    let params: *const WebPDecParams = &mut (*idec).params_;
    let output: *mut WebPDecBuffer = (*params).output;
    let mut curr_size: size_t = MemDataSize(&mut (*idec).mem_);
    if curr_size < (*idec).chunk_size_ >> 3 as libc::c_int {
        (*dec).status_ = VP8_STATUS_SUSPENDED;
        return ErrorStatusLossless(idec, (*dec).status_);
    }
    if VP8LDecodeHeader(dec, io) == 0 {
        if (*dec).status_ as libc::c_uint
            == VP8_STATUS_BITSTREAM_ERROR as libc::c_int as libc::c_uint
            && curr_size < (*idec).chunk_size_
        {
            (*dec).status_ = VP8_STATUS_SUSPENDED;
        }
        return ErrorStatusLossless(idec, (*dec).status_);
    }
    (*dec)
        .status_ = WebPAllocateDecBuffer(
        (*io).width,
        (*io).height,
        (*params).options,
        output,
    );
    if (*dec).status_ as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
        return IDecError(idec, (*dec).status_);
    }
    (*idec).state_ = STATE_VP8L_DATA;
    return VP8_STATUS_OK;
}
unsafe extern "C" fn DecodeVP8LData(idec: *mut WebPIDecoder) -> VP8StatusCode {
    let dec: *mut VP8LDecoder = (*idec).dec_ as *mut VP8LDecoder;
    let curr_size: size_t = MemDataSize(&mut (*idec).mem_);
    (*dec).incremental_ = (curr_size < (*idec).chunk_size_) as libc::c_int;
    if VP8LDecodeImage(dec) == 0 {
        return ErrorStatusLossless(idec, (*dec).status_);
    }
    return (if (*dec).status_ as libc::c_uint
        == VP8_STATUS_SUSPENDED as libc::c_int as libc::c_uint
    {
        (*dec).status_ as libc::c_uint
    } else {
        FinishDecoding(idec) as libc::c_uint
    }) as VP8StatusCode;
}
unsafe extern "C" fn IDecode(mut idec: *mut WebPIDecoder) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_SUSPENDED;
    if (*idec).state_ as libc::c_uint == STATE_WEBP_HEADER as libc::c_int as libc::c_uint
    {
        status = DecodeWebPHeaders(idec);
    } else if ((*idec).dec_).is_null() {
        return VP8_STATUS_SUSPENDED
    }
    if (*idec).state_ as libc::c_uint == STATE_VP8_HEADER as libc::c_int as libc::c_uint
    {
        status = DecodeVP8FrameHeader(idec);
    }
    if (*idec).state_ as libc::c_uint == STATE_VP8_PARTS0 as libc::c_int as libc::c_uint
    {
        status = DecodePartition0(idec);
    }
    if (*idec).state_ as libc::c_uint == STATE_VP8_DATA as libc::c_int as libc::c_uint {
        let dec: *const VP8Decoder = (*idec).dec_ as *mut VP8Decoder;
        if dec.is_null() {
            return VP8_STATUS_SUSPENDED;
        }
        status = DecodeRemaining(idec);
    }
    if (*idec).state_ as libc::c_uint == STATE_VP8L_HEADER as libc::c_int as libc::c_uint
    {
        status = DecodeVP8LHeader(idec);
    }
    if (*idec).state_ as libc::c_uint == STATE_VP8L_DATA as libc::c_int as libc::c_uint {
        status = DecodeVP8LData(idec);
    }
    return status;
}
unsafe extern "C" fn NewDecoder(
    output_buffer: *mut WebPDecBuffer,
    features: *const WebPBitstreamFeatures,
) -> *mut WebPIDecoder {
    let mut idec: *mut WebPIDecoder = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<WebPIDecoder>() as libc::c_ulong,
    ) as *mut WebPIDecoder;
    if idec.is_null() {
        return 0 as *mut WebPIDecoder;
    }
    (*idec).state_ = STATE_WEBP_HEADER;
    (*idec).chunk_size_ = 0 as libc::c_int as size_t;
    (*idec).last_mb_y_ = -(1 as libc::c_int);
    InitMemBuffer(&mut (*idec).mem_);
    if WebPInitDecBuffer(&mut (*idec).output_) == 0 || VP8InitIo(&mut (*idec).io_) == 0 {
        WebPSafeFree(idec as *mut libc::c_void);
        return 0 as *mut WebPIDecoder;
    }
    WebPResetDecParams(&mut (*idec).params_);
    if output_buffer.is_null() || WebPAvoidSlowMemory(output_buffer, features) != 0 {
        (*idec).params_.output = &mut (*idec).output_;
        (*idec).final_output_ = output_buffer;
        if !output_buffer.is_null() {
            (*(*idec).params_.output).colorspace = (*output_buffer).colorspace;
        }
    } else {
        (*idec).params_.output = output_buffer;
        (*idec).final_output_ = 0 as *mut WebPDecBuffer;
    }
    WebPInitCustomIo(&mut (*idec).params_, &mut (*idec).io_);
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewDecoder(
    mut output_buffer: *mut WebPDecBuffer,
) -> *mut WebPIDecoder {
    return NewDecoder(output_buffer, 0 as *const WebPBitstreamFeatures);
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecode(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut config: *mut WebPDecoderConfig,
) -> *mut WebPIDecoder {
    let mut idec: *mut WebPIDecoder = 0 as *mut WebPIDecoder;
    let mut tmp_features: WebPBitstreamFeatures = WebPBitstreamFeatures {
        width: 0,
        height: 0,
        has_alpha: 0,
        has_animation: 0,
        format: 0,
        pad: [0; 5],
    };
    let features: *mut WebPBitstreamFeatures = if config.is_null() {
        &mut tmp_features
    } else {
        &mut (*config).input
    };
    memset(
        &mut tmp_features as *mut WebPBitstreamFeatures as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPBitstreamFeatures>() as libc::c_ulong,
    );
    if !data.is_null() && data_size > 0 as libc::c_int as libc::c_ulong {
        if WebPGetFeatures(data, data_size, features) as libc::c_uint
            != VP8_STATUS_OK as libc::c_int as libc::c_uint
        {
            return 0 as *mut WebPIDecoder;
        }
    }
    idec = if !config.is_null() {
        NewDecoder(&mut (*config).output, features)
    } else {
        NewDecoder(0 as *mut WebPDecBuffer, features)
    };
    if idec.is_null() {
        return 0 as *mut WebPIDecoder;
    }
    if !config.is_null() {
        (*idec).params_.options = &mut (*config).options;
    }
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDelete(mut idec: *mut WebPIDecoder) {
    if idec.is_null() {
        return;
    }
    if !((*idec).dec_).is_null() {
        if (*idec).is_lossless_ == 0 {
            if (*idec).state_ as libc::c_uint
                == STATE_VP8_DATA as libc::c_int as libc::c_uint
            {
                VP8ExitCritical((*idec).dec_ as *mut VP8Decoder, &mut (*idec).io_);
            }
            VP8Delete((*idec).dec_ as *mut VP8Decoder);
        } else {
            VP8LDelete((*idec).dec_ as *mut VP8LDecoder);
        }
    }
    ClearMemBuffer(&mut (*idec).mem_);
    WebPFreeDecBuffer(&mut (*idec).output_);
    WebPSafeFree(idec as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewRGB(
    mut csp: WEBP_CSP_MODE,
    mut output_buffer: *mut uint8_t,
    mut output_buffer_size: size_t,
    mut output_stride: libc::c_int,
) -> *mut WebPIDecoder {
    let is_external_memory: libc::c_int = if !output_buffer.is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut idec: *mut WebPIDecoder = 0 as *mut WebPIDecoder;
    if csp as libc::c_uint >= MODE_YUV as libc::c_int as libc::c_uint {
        return 0 as *mut WebPIDecoder;
    }
    if is_external_memory == 0 as libc::c_int {
        output_buffer_size = 0 as libc::c_int as size_t;
        output_stride = 0 as libc::c_int;
    } else if output_stride == 0 as libc::c_int
        || output_buffer_size == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut WebPIDecoder
    }
    idec = WebPINewDecoder(0 as *mut WebPDecBuffer);
    if idec.is_null() {
        return 0 as *mut WebPIDecoder;
    }
    (*idec).output_.colorspace = csp;
    (*idec).output_.is_external_memory = is_external_memory;
    (*idec).output_.u.RGBA.rgba = output_buffer;
    (*idec).output_.u.RGBA.stride = output_stride;
    (*idec).output_.u.RGBA.size = output_buffer_size;
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewYUVA(
    mut luma: *mut uint8_t,
    mut luma_size: size_t,
    mut luma_stride: libc::c_int,
    mut u: *mut uint8_t,
    mut u_size: size_t,
    mut u_stride: libc::c_int,
    mut v: *mut uint8_t,
    mut v_size: size_t,
    mut v_stride: libc::c_int,
    mut a: *mut uint8_t,
    mut a_size: size_t,
    mut a_stride: libc::c_int,
) -> *mut WebPIDecoder {
    let is_external_memory: libc::c_int = if !luma.is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut idec: *mut WebPIDecoder = 0 as *mut WebPIDecoder;
    let mut colorspace: WEBP_CSP_MODE = MODE_RGB;
    if is_external_memory == 0 as libc::c_int {
        a_size = 0 as libc::c_int as size_t;
        v_size = a_size;
        u_size = v_size;
        luma_size = u_size;
        a_stride = 0 as libc::c_int;
        v_stride = a_stride;
        u_stride = v_stride;
        luma_stride = u_stride;
        a = 0 as *mut uint8_t;
        v = a;
        u = v;
        colorspace = MODE_YUVA;
    } else {
        if u.is_null() || v.is_null() {
            return 0 as *mut WebPIDecoder;
        }
        if luma_size == 0 as libc::c_int as libc::c_ulong
            || u_size == 0 as libc::c_int as libc::c_ulong
            || v_size == 0 as libc::c_int as libc::c_ulong
        {
            return 0 as *mut WebPIDecoder;
        }
        if luma_stride == 0 as libc::c_int || u_stride == 0 as libc::c_int
            || v_stride == 0 as libc::c_int
        {
            return 0 as *mut WebPIDecoder;
        }
        if !a.is_null() {
            if a_size == 0 as libc::c_int as libc::c_ulong
                || a_stride == 0 as libc::c_int
            {
                return 0 as *mut WebPIDecoder;
            }
        }
        colorspace = (if a.is_null() {
            MODE_YUV as libc::c_int
        } else {
            MODE_YUVA as libc::c_int
        }) as WEBP_CSP_MODE;
    }
    idec = WebPINewDecoder(0 as *mut WebPDecBuffer);
    if idec.is_null() {
        return 0 as *mut WebPIDecoder;
    }
    (*idec).output_.colorspace = colorspace;
    (*idec).output_.is_external_memory = is_external_memory;
    (*idec).output_.u.YUVA.y = luma;
    (*idec).output_.u.YUVA.y_stride = luma_stride;
    (*idec).output_.u.YUVA.y_size = luma_size;
    (*idec).output_.u.YUVA.u = u;
    (*idec).output_.u.YUVA.u_stride = u_stride;
    (*idec).output_.u.YUVA.u_size = u_size;
    (*idec).output_.u.YUVA.v = v;
    (*idec).output_.u.YUVA.v_stride = v_stride;
    (*idec).output_.u.YUVA.v_size = v_size;
    (*idec).output_.u.YUVA.a = a;
    (*idec).output_.u.YUVA.a_stride = a_stride;
    (*idec).output_.u.YUVA.a_size = a_size;
    return idec;
}
#[no_mangle]
pub unsafe extern "C" fn WebPINewYUV(
    mut luma: *mut uint8_t,
    mut luma_size: size_t,
    mut luma_stride: libc::c_int,
    mut u: *mut uint8_t,
    mut u_size: size_t,
    mut u_stride: libc::c_int,
    mut v: *mut uint8_t,
    mut v_size: size_t,
    mut v_stride: libc::c_int,
) -> *mut WebPIDecoder {
    return WebPINewYUVA(
        luma,
        luma_size,
        luma_stride,
        u,
        u_size,
        u_stride,
        v,
        v_size,
        v_stride,
        0 as *mut uint8_t,
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn IDecCheckStatus(idec: *const WebPIDecoder) -> VP8StatusCode {
    if (*idec).state_ as libc::c_uint == STATE_ERROR as libc::c_int as libc::c_uint {
        return VP8_STATUS_BITSTREAM_ERROR;
    }
    if (*idec).state_ as libc::c_uint == STATE_DONE as libc::c_int as libc::c_uint {
        return VP8_STATUS_OK;
    }
    return VP8_STATUS_SUSPENDED;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIAppend(
    mut idec: *mut WebPIDecoder,
    mut data: *const uint8_t,
    mut data_size: size_t,
) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if idec.is_null() || data.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    status = IDecCheckStatus(idec);
    if status as libc::c_uint != VP8_STATUS_SUSPENDED as libc::c_int as libc::c_uint {
        return status;
    }
    if CheckMemBufferMode(&mut (*idec).mem_, MEM_MODE_APPEND) == 0 {
        return VP8_STATUS_INVALID_PARAM;
    }
    if AppendToMemBuffer(idec, data, data_size) == 0 {
        return VP8_STATUS_OUT_OF_MEMORY;
    }
    return IDecode(idec);
}
#[no_mangle]
pub unsafe extern "C" fn WebPIUpdate(
    mut idec: *mut WebPIDecoder,
    mut data: *const uint8_t,
    mut data_size: size_t,
) -> VP8StatusCode {
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if idec.is_null() || data.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    status = IDecCheckStatus(idec);
    if status as libc::c_uint != VP8_STATUS_SUSPENDED as libc::c_int as libc::c_uint {
        return status;
    }
    if CheckMemBufferMode(&mut (*idec).mem_, MEM_MODE_MAP) == 0 {
        return VP8_STATUS_INVALID_PARAM;
    }
    if RemapMemBuffer(idec, data, data_size) == 0 {
        return VP8_STATUS_INVALID_PARAM;
    }
    return IDecode(idec);
}
unsafe extern "C" fn GetOutputBuffer(idec: *const WebPIDecoder) -> *const WebPDecBuffer {
    if idec.is_null() || ((*idec).dec_).is_null() {
        return 0 as *const WebPDecBuffer;
    }
    if (*idec).state_ as libc::c_uint <= STATE_VP8_PARTS0 as libc::c_int as libc::c_uint
    {
        return 0 as *const WebPDecBuffer;
    }
    if !((*idec).final_output_).is_null() {
        return 0 as *const WebPDecBuffer;
    }
    return (*idec).params_.output;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecodedArea(
    mut idec: *const WebPIDecoder,
    mut left: *mut libc::c_int,
    mut top: *mut libc::c_int,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) -> *const WebPDecBuffer {
    let src: *const WebPDecBuffer = GetOutputBuffer(idec);
    if !left.is_null() {
        *left = 0 as libc::c_int;
    }
    if !top.is_null() {
        *top = 0 as libc::c_int;
    }
    if !src.is_null() {
        if !width.is_null() {
            *width = (*src).width;
        }
        if !height.is_null() {
            *height = (*idec).params_.last_y;
        }
    } else {
        if !width.is_null() {
            *width = 0 as libc::c_int;
        }
        if !height.is_null() {
            *height = 0 as libc::c_int;
        }
    }
    return src;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecGetRGB(
    mut idec: *const WebPIDecoder,
    mut last_y: *mut libc::c_int,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut stride: *mut libc::c_int,
) -> *mut uint8_t {
    let src: *const WebPDecBuffer = GetOutputBuffer(idec);
    if src.is_null() {
        return 0 as *mut uint8_t;
    }
    if (*src).colorspace as libc::c_uint >= MODE_YUV as libc::c_int as libc::c_uint {
        return 0 as *mut uint8_t;
    }
    if !last_y.is_null() {
        *last_y = (*idec).params_.last_y;
    }
    if !width.is_null() {
        *width = (*src).width;
    }
    if !height.is_null() {
        *height = (*src).height;
    }
    if !stride.is_null() {
        *stride = (*src).u.RGBA.stride;
    }
    return (*src).u.RGBA.rgba;
}
#[no_mangle]
pub unsafe extern "C" fn WebPIDecGetYUVA(
    mut idec: *const WebPIDecoder,
    mut last_y: *mut libc::c_int,
    mut u: *mut *mut uint8_t,
    mut v: *mut *mut uint8_t,
    mut a: *mut *mut uint8_t,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut stride: *mut libc::c_int,
    mut uv_stride: *mut libc::c_int,
    mut a_stride: *mut libc::c_int,
) -> *mut uint8_t {
    let src: *const WebPDecBuffer = GetOutputBuffer(idec);
    if src.is_null() {
        return 0 as *mut uint8_t;
    }
    if ((*src).colorspace as libc::c_uint) < MODE_YUV as libc::c_int as libc::c_uint {
        return 0 as *mut uint8_t;
    }
    if !last_y.is_null() {
        *last_y = (*idec).params_.last_y;
    }
    if !u.is_null() {
        *u = (*src).u.YUVA.u;
    }
    if !v.is_null() {
        *v = (*src).u.YUVA.v;
    }
    if !a.is_null() {
        *a = (*src).u.YUVA.a;
    }
    if !width.is_null() {
        *width = (*src).width;
    }
    if !height.is_null() {
        *height = (*src).height;
    }
    if !stride.is_null() {
        *stride = (*src).u.YUVA.y_stride;
    }
    if !uv_stride.is_null() {
        *uv_stride = (*src).u.YUVA.u_stride;
    }
    if !a_stride.is_null() {
        *a_stride = (*src).u.YUVA.a_stride;
    }
    return (*src).u.YUVA.y;
}
#[no_mangle]
pub unsafe extern "C" fn WebPISetIOHooks(
    idec: *mut WebPIDecoder,
    mut put: VP8IoPutHook,
    mut setup: VP8IoSetupHook,
    mut teardown: VP8IoTeardownHook,
    mut user_data: *mut libc::c_void,
) -> libc::c_int {
    if idec.is_null()
        || (*idec).state_ as libc::c_uint
            > STATE_WEBP_HEADER as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    (*idec).io_.put = put;
    (*idec).io_.setup = setup;
    (*idec).io_.teardown = teardown;
    (*idec).io_.opaque = user_data;
    return 1 as libc::c_int;
}
