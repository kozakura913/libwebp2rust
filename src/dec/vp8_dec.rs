use ::libc;
extern "C" {
    static mut VP8TransformWHT: VP8WHT;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn WebPDeallocateAlphaMemory(dec: *mut VP8Decoder);
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
    fn VP8EnterCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> VP8StatusCode;
    fn VP8InitFrame(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8ParseIntraModeRow(br: *mut VP8BitReader, dec: *mut VP8Decoder) -> libc::c_int;
    fn VP8ProcessRow(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8ExitCritical(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int;
    fn VP8ResetProba(proba: *mut VP8Proba);
    fn VP8GetSignedValue(br: *mut VP8BitReader, num_bits: libc::c_int) -> int32_t;
    fn VP8InitBitReader(br: *mut VP8BitReader, start: *const uint8_t, size: size_t);
    fn VP8ParseQuant(dec: *mut VP8Decoder);
    fn VP8GetValue(br: *mut VP8BitReader, num_bits: libc::c_int) -> uint32_t;
    fn VP8ParseProba(br: *mut VP8BitReader, dec: *mut VP8Decoder);
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn VP8LoadFinalBytes(br: *mut VP8BitReader);
    static kVP8Log2Range: [uint8_t; 128];
    static kVP8NewRange: [uint8_t; 128];
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type GetCoeffsFunc = Option::<
    unsafe extern "C" fn(
        *mut VP8BitReader,
        *const *const VP8BandProbas,
        libc::c_int,
        *const libc::c_int,
        libc::c_int,
        *mut int16_t,
    ) -> libc::c_int,
>;
pub type lbit_t = uint64_t;
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
pub const NUM_MODE_LF_DELTAS: C2RustUnnamed_2 = 4;
pub const NUM_REF_LF_DELTAS: C2RustUnnamed_2 = 4;
pub const MB_FEATURE_TREE_PROBS: C2RustUnnamed_2 = 3;
pub const NUM_MB_SEGMENTS: C2RustUnnamed_2 = 4;
pub const B_DC_PRED: C2RustUnnamed_1 = 0;
pub type VP8WHT = Option::<unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ()>;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NUM_B_DC_MODES: C2RustUnnamed_1 = 7;
pub const B_DC_PRED_NOTOPLEFT: C2RustUnnamed_1 = 6;
pub const B_DC_PRED_NOLEFT: C2RustUnnamed_1 = 5;
pub const B_DC_PRED_NOTOP: C2RustUnnamed_1 = 4;
pub const NUM_PRED_MODES: C2RustUnnamed_1 = 4;
pub const B_PRED: C2RustUnnamed_1 = 10;
pub const TM_PRED: C2RustUnnamed_1 = 1;
pub const H_PRED: C2RustUnnamed_1 = 3;
pub const V_PRED: C2RustUnnamed_1 = 2;
pub const DC_PRED: C2RustUnnamed_1 = 0;
pub const NUM_BMODES: C2RustUnnamed_1 = 10;
pub const B_HU_PRED: C2RustUnnamed_1 = 9;
pub const B_HD_PRED: C2RustUnnamed_1 = 8;
pub const B_VL_PRED: C2RustUnnamed_1 = 7;
pub const B_LD_PRED: C2RustUnnamed_1 = 6;
pub const B_VR_PRED: C2RustUnnamed_1 = 5;
pub const B_RD_PRED: C2RustUnnamed_1 = 4;
pub const B_HE_PRED: C2RustUnnamed_1 = 3;
pub const B_VE_PRED: C2RustUnnamed_1 = 2;
pub const B_TM_PRED: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const NUM_PROBAS: C2RustUnnamed_2 = 11;
pub const NUM_CTX: C2RustUnnamed_2 = 3;
pub const NUM_BANDS: C2RustUnnamed_2 = 8;
pub const NUM_TYPES: C2RustUnnamed_2 = 4;
pub const MAX_NUM_PARTITIONS: C2RustUnnamed_2 = 8;
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> libc::c_int {
    return 31 as libc::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn BSwap64(mut x: uint64_t) -> uint64_t {
    return x.swap_bytes();
}
#[inline]
unsafe extern "C" fn VP8GetSigned(
    br: *mut VP8BitReader,
    mut v: libc::c_int,
) -> libc::c_int {
    if (*br).bits_ < 0 as libc::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: libc::c_int = (*br).bits_;
    let split: range_t = (*br).range_ >> 1 as libc::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let mask: int32_t = split.wrapping_sub(value) as int32_t >> 31 as libc::c_int;
    (*br).bits_ -= 1 as libc::c_int;
    (*br)
        .range_ = ((*br).range_ as libc::c_uint).wrapping_add(mask as range_t) as range_t
        as range_t;
    (*br).range_ |= 1 as libc::c_int as libc::c_uint;
    (*br)
        .value_ = ((*br).value_ as libc::c_ulong)
        .wrapping_sub(
            ((split.wrapping_add(1 as libc::c_int as libc::c_uint) & mask as uint32_t)
                as bit_t) << pos,
        ) as bit_t as bit_t;
    return (v ^ mask) - mask;
}
#[inline]
unsafe extern "C" fn VP8LoadNewBytes(br: *mut VP8BitReader) {
    if (*br).buf_ < (*br).buf_max_ {
        let mut bits: bit_t = 0;
        let mut in_bits: lbit_t = 0;
        memcpy(
            &mut in_bits as *mut lbit_t as *mut libc::c_void,
            (*br).buf_ as *const libc::c_void,
            ::core::mem::size_of::<lbit_t>() as libc::c_ulong,
        );
        (*br)
            .buf_ = ((*br).buf_)
            .offset((56 as libc::c_int >> 3 as libc::c_int) as isize);
        bits = BSwap64(in_bits);
        bits >>= 64 as libc::c_int - 56 as libc::c_int;
        (*br).value_ = bits | (*br).value_ << 56 as libc::c_int;
        (*br).bits_ += 56 as libc::c_int;
    } else {
        VP8LoadFinalBytes(br);
    };
}
#[inline]
unsafe extern "C" fn VP8GetBitAlt(
    br: *mut VP8BitReader,
    mut prob: libc::c_int,
) -> libc::c_int {
    let mut range: range_t = (*br).range_;
    if (*br).bits_ < 0 as libc::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: libc::c_int = (*br).bits_;
    let split: range_t = range.wrapping_mul(prob as libc::c_uint) >> 8 as libc::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let mut bit: libc::c_int = 0;
    if value > split {
        range = (range as libc::c_uint)
            .wrapping_sub(split.wrapping_add(1 as libc::c_int as libc::c_uint))
            as range_t as range_t;
        (*br)
            .value_ = ((*br).value_ as libc::c_ulong)
            .wrapping_sub(
                (split.wrapping_add(1 as libc::c_int as libc::c_uint) as bit_t) << pos,
            ) as bit_t as bit_t;
        bit = 1 as libc::c_int;
    } else {
        range = split;
        bit = 0 as libc::c_int;
    }
    if range <= 0x7e as libc::c_int as range_t {
        let shift: libc::c_int = kVP8Log2Range[range as usize] as libc::c_int;
        range = kVP8NewRange[range as usize] as range_t;
        (*br).bits_ -= shift;
    }
    (*br).range_ = range;
    return bit;
}
#[inline]
unsafe extern "C" fn VP8GetBit(
    br: *mut VP8BitReader,
    mut prob: libc::c_int,
) -> libc::c_int {
    let mut range: range_t = (*br).range_;
    if (*br).bits_ < 0 as libc::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: libc::c_int = (*br).bits_;
    let split: range_t = range.wrapping_mul(prob as libc::c_uint) >> 8 as libc::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let bit: libc::c_int = (value > split) as libc::c_int;
    if bit != 0 {
        range = (range as libc::c_uint).wrapping_sub(split) as range_t as range_t;
        (*br)
            .value_ = ((*br).value_ as libc::c_ulong)
            .wrapping_sub(
                (split.wrapping_add(1 as libc::c_int as libc::c_uint) as bit_t) << pos,
            ) as bit_t as bit_t;
    } else {
        range = split.wrapping_add(1 as libc::c_int as libc::c_uint);
    }
    let shift: libc::c_int = 7 as libc::c_int ^ BitsLog2Floor(range);
    range <<= shift;
    (*br).bits_ -= shift;
    (*br).range_ = range.wrapping_sub(1 as libc::c_int as libc::c_uint);
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn WebPGetDecoderVersion() -> libc::c_int {
    return (1 as libc::c_int) << 16 as libc::c_int
        | (3 as libc::c_int) << 8 as libc::c_int | 2 as libc::c_int;
}
static mut GetCoeffs: GetCoeffsFunc = None;
unsafe extern "C" fn SetOk(dec: *mut VP8Decoder) {
    (*dec).status_ = VP8_STATUS_OK;
    (*dec).error_msg_ = b"OK\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitIoInternal(
    io: *mut VP8Io,
    mut version: libc::c_int,
) -> libc::c_int {
    if version >> 8 as libc::c_int != 0x209 as libc::c_int >> 8 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !io.is_null() {
        memset(
            io as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<VP8Io>() as libc::c_ulong,
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8New() -> *mut VP8Decoder {
    let dec: *mut VP8Decoder = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<VP8Decoder>() as libc::c_ulong,
    ) as *mut VP8Decoder;
    if !dec.is_null() {
        SetOk(dec);
        ((*WebPGetWorkerInterface()).Init)
            .expect("non-null function pointer")(&mut (*dec).worker_);
        (*dec).ready_ = 0 as libc::c_int;
        (*dec).num_parts_minus_one_ = 0 as libc::c_int as uint32_t;
        InitGetCoeffs();
    }
    return dec;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Status(dec: *mut VP8Decoder) -> VP8StatusCode {
    if dec.is_null() {
        return VP8_STATUS_INVALID_PARAM;
    }
    return (*dec).status_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8StatusMessage(dec: *mut VP8Decoder) -> *const libc::c_char {
    if dec.is_null() {
        return b"no object\0" as *const u8 as *const libc::c_char;
    }
    if ((*dec).error_msg_).is_null() {
        return b"OK\0" as *const u8 as *const libc::c_char;
    }
    return (*dec).error_msg_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Delete(dec: *mut VP8Decoder) {
    if !dec.is_null() {
        VP8Clear(dec);
        WebPSafeFree(dec as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetError(
    dec: *mut VP8Decoder,
    mut error: VP8StatusCode,
    msg: *const libc::c_char,
) -> libc::c_int {
    if (*dec).status_ as libc::c_uint == VP8_STATUS_OK as libc::c_int as libc::c_uint {
        (*dec).status_ = error;
        (*dec).error_msg_ = msg;
        (*dec).ready_ = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8CheckSignature(
    data: *const uint8_t,
    mut data_size: size_t,
) -> libc::c_int {
    return (data_size >= 3 as libc::c_int as libc::c_ulong
        && *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x9d as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0x1 as libc::c_int
        && *data.offset(2 as libc::c_int as isize) as libc::c_int == 0x2a as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetInfo(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut chunk_size: size_t,
    width: *mut libc::c_int,
    height: *mut libc::c_int,
) -> libc::c_int {
    if data.is_null() || data_size < 10 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if VP8CheckSignature(
        data.offset(3 as libc::c_int as isize),
        data_size.wrapping_sub(3 as libc::c_int as libc::c_ulong),
    ) == 0
    {
        return 0 as libc::c_int
    } else {
        let bits: uint32_t = (*data.offset(0 as libc::c_int as isize) as libc::c_int
            | (*data.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int
            | (*data.offset(2 as libc::c_int as isize) as libc::c_int)
                << 16 as libc::c_int) as uint32_t;
        let key_frame: libc::c_int = (bits & 1 as libc::c_int as libc::c_uint == 0)
            as libc::c_int;
        let w: libc::c_int = ((*data.offset(7 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int | *data.offset(6 as libc::c_int as isize) as libc::c_int)
            & 0x3fff as libc::c_int;
        let h: libc::c_int = ((*data.offset(9 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int | *data.offset(8 as libc::c_int as isize) as libc::c_int)
            & 0x3fff as libc::c_int;
        if key_frame == 0 {
            return 0 as libc::c_int;
        }
        if bits >> 1 as libc::c_int & 7 as libc::c_int as libc::c_uint
            > 3 as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        if bits >> 4 as libc::c_int & 1 as libc::c_int as libc::c_uint == 0 {
            return 0 as libc::c_int;
        }
        if (bits >> 5 as libc::c_int) as libc::c_ulong >= chunk_size {
            return 0 as libc::c_int;
        }
        if w == 0 as libc::c_int || h == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if !width.is_null() {
            *width = w;
        }
        if !height.is_null() {
            *height = h;
        }
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn ResetSegmentHeader(hdr: *mut VP8SegmentHeader) {
    (*hdr).use_segment_ = 0 as libc::c_int;
    (*hdr).update_map_ = 0 as libc::c_int;
    (*hdr).absolute_delta_ = 1 as libc::c_int;
    memset(
        ((*hdr).quantizer_).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[int8_t; 4]>() as libc::c_ulong,
    );
    memset(
        ((*hdr).filter_strength_).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[int8_t; 4]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn ParseSegmentHeader(
    mut br: *mut VP8BitReader,
    mut hdr: *mut VP8SegmentHeader,
    mut proba: *mut VP8Proba,
) -> libc::c_int {
    (*hdr).use_segment_ = VP8GetValue(br, 1 as libc::c_int) as libc::c_int;
    if (*hdr).use_segment_ != 0 {
        (*hdr).update_map_ = VP8GetValue(br, 1 as libc::c_int) as libc::c_int;
        if VP8GetValue(br, 1 as libc::c_int) != 0 {
            let mut s: libc::c_int = 0;
            (*hdr).absolute_delta_ = VP8GetValue(br, 1 as libc::c_int) as libc::c_int;
            s = 0 as libc::c_int;
            while s < NUM_MB_SEGMENTS as libc::c_int {
                (*hdr)
                    .quantizer_[s
                    as usize] = (if VP8GetValue(br, 1 as libc::c_int) != 0 {
                    VP8GetSignedValue(br, 7 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as int8_t;
                s += 1;
                s;
            }
            s = 0 as libc::c_int;
            while s < NUM_MB_SEGMENTS as libc::c_int {
                (*hdr)
                    .filter_strength_[s
                    as usize] = (if VP8GetValue(br, 1 as libc::c_int) != 0 {
                    VP8GetSignedValue(br, 6 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as int8_t;
                s += 1;
                s;
            }
        }
        if (*hdr).update_map_ != 0 {
            let mut s_0: libc::c_int = 0;
            s_0 = 0 as libc::c_int;
            while s_0 < MB_FEATURE_TREE_PROBS as libc::c_int {
                (*proba)
                    .segments_[s_0
                    as usize] = (if VP8GetValue(br, 1 as libc::c_int) != 0 {
                    VP8GetValue(br, 8 as libc::c_int)
                } else {
                    255 as libc::c_uint
                }) as uint8_t;
                s_0 += 1;
                s_0;
            }
        }
    } else {
        (*hdr).update_map_ = 0 as libc::c_int;
    }
    return ((*br).eof_ == 0) as libc::c_int;
}
unsafe extern "C" fn ParsePartitions(
    dec: *mut VP8Decoder,
    mut buf: *const uint8_t,
    mut size: size_t,
) -> VP8StatusCode {
    let br: *mut VP8BitReader = &mut (*dec).br_;
    let mut sz: *const uint8_t = buf;
    let mut buf_end: *const uint8_t = buf.offset(size as isize);
    let mut part_start: *const uint8_t = 0 as *const uint8_t;
    let mut size_left: size_t = size;
    let mut last_part: size_t = 0;
    let mut p: size_t = 0;
    (*dec)
        .num_parts_minus_one_ = (((1 as libc::c_int)
        << VP8GetValue(br, 2 as libc::c_int)) - 1 as libc::c_int) as uint32_t;
    last_part = (*dec).num_parts_minus_one_ as size_t;
    if size < (3 as libc::c_int as libc::c_ulong).wrapping_mul(last_part) {
        return VP8_STATUS_NOT_ENOUGH_DATA;
    }
    part_start = buf
        .offset(last_part.wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize);
    size_left = (size_left as libc::c_ulong)
        .wrapping_sub(last_part.wrapping_mul(3 as libc::c_int as libc::c_ulong))
        as size_t as size_t;
    p = 0 as libc::c_int as size_t;
    while p < last_part {
        let mut psize: size_t = (*sz.offset(0 as libc::c_int as isize) as libc::c_int
            | (*sz.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
            | (*sz.offset(2 as libc::c_int as isize) as libc::c_int)
                << 16 as libc::c_int) as size_t;
        if psize > size_left {
            psize = size_left;
        }
        VP8InitBitReader(
            ((*dec).parts_).as_mut_ptr().offset(p as isize),
            part_start,
            psize,
        );
        part_start = part_start.offset(psize as isize);
        size_left = (size_left as libc::c_ulong).wrapping_sub(psize) as size_t as size_t;
        sz = sz.offset(3 as libc::c_int as isize);
        p = p.wrapping_add(1);
        p;
    }
    VP8InitBitReader(
        ((*dec).parts_).as_mut_ptr().offset(last_part as isize),
        part_start,
        size_left,
    );
    if part_start < buf_end {
        return VP8_STATUS_OK;
    }
    return (if (*dec).incremental_ != 0 {
        VP8_STATUS_SUSPENDED as libc::c_int
    } else {
        VP8_STATUS_NOT_ENOUGH_DATA as libc::c_int
    }) as VP8StatusCode;
}
unsafe extern "C" fn ParseFilterHeader(
    mut br: *mut VP8BitReader,
    dec: *mut VP8Decoder,
) -> libc::c_int {
    let hdr: *mut VP8FilterHeader = &mut (*dec).filter_hdr_;
    (*hdr).simple_ = VP8GetValue(br, 1 as libc::c_int) as libc::c_int;
    (*hdr).level_ = VP8GetValue(br, 6 as libc::c_int) as libc::c_int;
    (*hdr).sharpness_ = VP8GetValue(br, 3 as libc::c_int) as libc::c_int;
    (*hdr).use_lf_delta_ = VP8GetValue(br, 1 as libc::c_int) as libc::c_int;
    if (*hdr).use_lf_delta_ != 0 {
        if VP8GetValue(br, 1 as libc::c_int) != 0 {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < NUM_REF_LF_DELTAS as libc::c_int {
                if VP8GetValue(br, 1 as libc::c_int) != 0 {
                    (*hdr)
                        .ref_lf_delta_[i
                        as usize] = VP8GetSignedValue(br, 6 as libc::c_int);
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < NUM_MODE_LF_DELTAS as libc::c_int {
                if VP8GetValue(br, 1 as libc::c_int) != 0 {
                    (*hdr)
                        .mode_lf_delta_[i
                        as usize] = VP8GetSignedValue(br, 6 as libc::c_int);
                }
                i += 1;
                i;
            }
        }
    }
    (*dec)
        .filter_type_ = if (*hdr).level_ == 0 as libc::c_int {
        0 as libc::c_int
    } else if (*hdr).simple_ != 0 {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
    return ((*br).eof_ == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetHeaders(
    dec: *mut VP8Decoder,
    io: *mut VP8Io,
) -> libc::c_int {
    let mut buf: *const uint8_t = 0 as *const uint8_t;
    let mut buf_size: size_t = 0;
    let mut frm_hdr: *mut VP8FrameHeader = 0 as *mut VP8FrameHeader;
    let mut pic_hdr: *mut VP8PictureHeader = 0 as *mut VP8PictureHeader;
    let mut br: *mut VP8BitReader = 0 as *mut VP8BitReader;
    let mut status: VP8StatusCode = VP8_STATUS_OK;
    if dec.is_null() {
        return 0 as libc::c_int;
    }
    SetOk(dec);
    if io.is_null() {
        return VP8SetError(
            dec,
            VP8_STATUS_INVALID_PARAM,
            b"null VP8Io passed to VP8GetHeaders()\0" as *const u8 as *const libc::c_char,
        );
    }
    buf = (*io).data;
    buf_size = (*io).data_size;
    if buf_size < 4 as libc::c_int as libc::c_ulong {
        return VP8SetError(
            dec,
            VP8_STATUS_NOT_ENOUGH_DATA,
            b"Truncated header.\0" as *const u8 as *const libc::c_char,
        );
    }
    let bits: uint32_t = (*buf.offset(0 as libc::c_int as isize) as libc::c_int
        | (*buf.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | (*buf.offset(2 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as uint32_t;
    frm_hdr = &mut (*dec).frm_hdr_;
    (*frm_hdr)
        .key_frame_ = (bits & 1 as libc::c_int as libc::c_uint == 0) as libc::c_int
        as uint8_t;
    (*frm_hdr)
        .profile_ = (bits >> 1 as libc::c_int & 7 as libc::c_int as libc::c_uint)
        as uint8_t;
    (*frm_hdr)
        .show_ = (bits >> 4 as libc::c_int & 1 as libc::c_int as libc::c_uint)
        as uint8_t;
    (*frm_hdr).partition_length_ = bits >> 5 as libc::c_int;
    if (*frm_hdr).profile_ as libc::c_int > 3 as libc::c_int {
        return VP8SetError(
            dec,
            VP8_STATUS_BITSTREAM_ERROR,
            b"Incorrect keyframe parameters.\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*frm_hdr).show_ == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_UNSUPPORTED_FEATURE,
            b"Frame not displayable.\0" as *const u8 as *const libc::c_char,
        );
    }
    buf = buf.offset(3 as libc::c_int as isize);
    buf_size = (buf_size as libc::c_ulong)
        .wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
    pic_hdr = &mut (*dec).pic_hdr_;
    if (*frm_hdr).key_frame_ != 0 {
        if buf_size < 7 as libc::c_int as libc::c_ulong {
            return VP8SetError(
                dec,
                VP8_STATUS_NOT_ENOUGH_DATA,
                b"cannot parse picture header\0" as *const u8 as *const libc::c_char,
            );
        }
        if VP8CheckSignature(buf, buf_size) == 0 {
            return VP8SetError(
                dec,
                VP8_STATUS_BITSTREAM_ERROR,
                b"Bad code word\0" as *const u8 as *const libc::c_char,
            );
        }
        (*pic_hdr)
            .width_ = (((*buf.offset(4 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int | *buf.offset(3 as libc::c_int as isize) as libc::c_int)
            & 0x3fff as libc::c_int) as uint16_t;
        (*pic_hdr)
            .xscale_ = (*buf.offset(4 as libc::c_int as isize) as libc::c_int
            >> 6 as libc::c_int) as uint8_t;
        (*pic_hdr)
            .height_ = (((*buf.offset(6 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int | *buf.offset(5 as libc::c_int as isize) as libc::c_int)
            & 0x3fff as libc::c_int) as uint16_t;
        (*pic_hdr)
            .yscale_ = (*buf.offset(6 as libc::c_int as isize) as libc::c_int
            >> 6 as libc::c_int) as uint8_t;
        buf = buf.offset(7 as libc::c_int as isize);
        buf_size = (buf_size as libc::c_ulong)
            .wrapping_sub(7 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*dec)
            .mb_w_ = (*pic_hdr).width_ as libc::c_int + 15 as libc::c_int
            >> 4 as libc::c_int;
        (*dec)
            .mb_h_ = (*pic_hdr).height_ as libc::c_int + 15 as libc::c_int
            >> 4 as libc::c_int;
        (*io).width = (*pic_hdr).width_ as libc::c_int;
        (*io).height = (*pic_hdr).height_ as libc::c_int;
        (*io).use_cropping = 0 as libc::c_int;
        (*io).crop_top = 0 as libc::c_int;
        (*io).crop_left = 0 as libc::c_int;
        (*io).crop_right = (*io).width;
        (*io).crop_bottom = (*io).height;
        (*io).use_scaling = 0 as libc::c_int;
        (*io).scaled_width = (*io).width;
        (*io).scaled_height = (*io).height;
        (*io).mb_w = (*io).width;
        (*io).mb_h = (*io).height;
        VP8ResetProba(&mut (*dec).proba_);
        ResetSegmentHeader(&mut (*dec).segment_hdr_);
    }
    if (*frm_hdr).partition_length_ as libc::c_ulong > buf_size {
        return VP8SetError(
            dec,
            VP8_STATUS_NOT_ENOUGH_DATA,
            b"bad partition length\0" as *const u8 as *const libc::c_char,
        );
    }
    br = &mut (*dec).br_;
    VP8InitBitReader(br, buf, (*frm_hdr).partition_length_ as size_t);
    buf = buf.offset((*frm_hdr).partition_length_ as isize);
    buf_size = (buf_size as libc::c_ulong)
        .wrapping_sub((*frm_hdr).partition_length_ as libc::c_ulong) as size_t as size_t;
    if (*frm_hdr).key_frame_ != 0 {
        (*pic_hdr).colorspace_ = VP8GetValue(br, 1 as libc::c_int) as uint8_t;
        (*pic_hdr).clamp_type_ = VP8GetValue(br, 1 as libc::c_int) as uint8_t;
    }
    if ParseSegmentHeader(br, &mut (*dec).segment_hdr_, &mut (*dec).proba_) == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_BITSTREAM_ERROR,
            b"cannot parse segment header\0" as *const u8 as *const libc::c_char,
        );
    }
    if ParseFilterHeader(br, dec) == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_BITSTREAM_ERROR,
            b"cannot parse filter header\0" as *const u8 as *const libc::c_char,
        );
    }
    status = ParsePartitions(dec, buf, buf_size);
    if status as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
        return VP8SetError(
            dec,
            status,
            b"cannot parse partitions\0" as *const u8 as *const libc::c_char,
        );
    }
    VP8ParseQuant(dec);
    if (*frm_hdr).key_frame_ == 0 {
        return VP8SetError(
            dec,
            VP8_STATUS_UNSUPPORTED_FEATURE,
            b"Not a key frame.\0" as *const u8 as *const libc::c_char,
        );
    }
    VP8GetValue(br, 1 as libc::c_int);
    VP8ParseProba(br, dec);
    (*dec).ready_ = 1 as libc::c_int;
    return 1 as libc::c_int;
}
static mut kCat3: [uint8_t; 4] = [
    173 as libc::c_int as uint8_t,
    148 as libc::c_int as uint8_t,
    140 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
static mut kCat4: [uint8_t; 5] = [
    176 as libc::c_int as uint8_t,
    155 as libc::c_int as uint8_t,
    140 as libc::c_int as uint8_t,
    135 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
static mut kCat5: [uint8_t; 6] = [
    180 as libc::c_int as uint8_t,
    157 as libc::c_int as uint8_t,
    141 as libc::c_int as uint8_t,
    134 as libc::c_int as uint8_t,
    130 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
static mut kCat6: [uint8_t; 12] = [
    254 as libc::c_int as uint8_t,
    254 as libc::c_int as uint8_t,
    243 as libc::c_int as uint8_t,
    230 as libc::c_int as uint8_t,
    196 as libc::c_int as uint8_t,
    177 as libc::c_int as uint8_t,
    153 as libc::c_int as uint8_t,
    140 as libc::c_int as uint8_t,
    133 as libc::c_int as uint8_t,
    130 as libc::c_int as uint8_t,
    129 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
static mut kCat3456: [*const uint8_t; 4] = unsafe {
    [kCat3.as_ptr(), kCat4.as_ptr(), kCat5.as_ptr(), kCat6.as_ptr()]
};
static mut kZigzag: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
unsafe extern "C" fn GetLargeValue(
    br: *mut VP8BitReader,
    p: *const uint8_t,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    if VP8GetBit(br, *p.offset(3 as libc::c_int as isize) as libc::c_int) == 0 {
        if VP8GetBit(br, *p.offset(4 as libc::c_int as isize) as libc::c_int) == 0 {
            v = 2 as libc::c_int;
        } else {
            v = 3 as libc::c_int
                + VP8GetBit(br, *p.offset(5 as libc::c_int as isize) as libc::c_int);
        }
    } else if VP8GetBit(br, *p.offset(6 as libc::c_int as isize) as libc::c_int) == 0 {
        if VP8GetBit(br, *p.offset(7 as libc::c_int as isize) as libc::c_int) == 0 {
            v = 5 as libc::c_int + VP8GetBit(br, 159 as libc::c_int);
        } else {
            v = 7 as libc::c_int + 2 as libc::c_int * VP8GetBit(br, 165 as libc::c_int);
            v += VP8GetBit(br, 145 as libc::c_int);
        }
    } else {
        let mut tab: *const uint8_t = 0 as *const uint8_t;
        let bit1: libc::c_int = VP8GetBit(
            br,
            *p.offset(8 as libc::c_int as isize) as libc::c_int,
        );
        let bit0: libc::c_int = VP8GetBit(
            br,
            *p.offset((9 as libc::c_int + bit1) as isize) as libc::c_int,
        );
        let cat: libc::c_int = 2 as libc::c_int * bit1 + bit0;
        v = 0 as libc::c_int;
        tab = kCat3456[cat as usize];
        while *tab != 0 {
            v += v + VP8GetBit(br, *tab as libc::c_int);
            tab = tab.offset(1);
            tab;
        }
        v += 3 as libc::c_int + ((8 as libc::c_int) << cat);
    }
    return v;
}
unsafe extern "C" fn GetCoeffsFast(
    br: *mut VP8BitReader,
    mut prob: *const *const VP8BandProbas,
    mut ctx: libc::c_int,
    mut dq: *const libc::c_int,
    mut n: libc::c_int,
    mut out: *mut int16_t,
) -> libc::c_int {
    let mut p: *const uint8_t = ((**prob.offset(n as isize)).probas_[ctx as usize])
        .as_ptr();
    while n < 16 as libc::c_int {
        if VP8GetBit(br, *p.offset(0 as libc::c_int as isize) as libc::c_int) == 0 {
            return n;
        }
        while VP8GetBit(br, *p.offset(1 as libc::c_int as isize) as libc::c_int) == 0 {
            n += 1;
            p = ((**prob.offset(n as isize)).probas_[0 as libc::c_int as usize])
                .as_ptr();
            if n == 16 as libc::c_int {
                return 16 as libc::c_int;
            }
        }
        let p_ctx: *const VP8ProbaArray = &*((**prob
            .offset((n + 1 as libc::c_int) as isize))
            .probas_)
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const VP8ProbaArray;
        let mut v: libc::c_int = 0;
        if VP8GetBit(br, *p.offset(2 as libc::c_int as isize) as libc::c_int) == 0 {
            v = 1 as libc::c_int;
            p = (*p_ctx.offset(1 as libc::c_int as isize)).as_ptr();
        } else {
            v = GetLargeValue(br, p);
            p = (*p_ctx.offset(2 as libc::c_int as isize)).as_ptr();
        }
        *out
            .offset(
                kZigzag[n as usize] as isize,
            ) = (VP8GetSigned(br, v)
            * *dq.offset((n > 0 as libc::c_int) as libc::c_int as isize)) as int16_t;
        n += 1;
        n;
    }
    return 16 as libc::c_int;
}
unsafe extern "C" fn GetCoeffsAlt(
    br: *mut VP8BitReader,
    mut prob: *const *const VP8BandProbas,
    mut ctx: libc::c_int,
    mut dq: *const libc::c_int,
    mut n: libc::c_int,
    mut out: *mut int16_t,
) -> libc::c_int {
    let mut p: *const uint8_t = ((**prob.offset(n as isize)).probas_[ctx as usize])
        .as_ptr();
    while n < 16 as libc::c_int {
        if VP8GetBitAlt(br, *p.offset(0 as libc::c_int as isize) as libc::c_int) == 0 {
            return n;
        }
        while VP8GetBitAlt(br, *p.offset(1 as libc::c_int as isize) as libc::c_int) == 0
        {
            n += 1;
            p = ((**prob.offset(n as isize)).probas_[0 as libc::c_int as usize])
                .as_ptr();
            if n == 16 as libc::c_int {
                return 16 as libc::c_int;
            }
        }
        let p_ctx: *const VP8ProbaArray = &*((**prob
            .offset((n + 1 as libc::c_int) as isize))
            .probas_)
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const VP8ProbaArray;
        let mut v: libc::c_int = 0;
        if VP8GetBitAlt(br, *p.offset(2 as libc::c_int as isize) as libc::c_int) == 0 {
            v = 1 as libc::c_int;
            p = (*p_ctx.offset(1 as libc::c_int as isize)).as_ptr();
        } else {
            v = GetLargeValue(br, p);
            p = (*p_ctx.offset(2 as libc::c_int as isize)).as_ptr();
        }
        *out
            .offset(
                kZigzag[n as usize] as isize,
            ) = (VP8GetSigned(br, v)
            * *dq.offset((n > 0 as libc::c_int) as libc::c_int as isize)) as int16_t;
        n += 1;
        n;
    }
    return 16 as libc::c_int;
}
unsafe extern "C" fn InitGetCoeffs_body() {
    ::core::ptr::write_volatile(
        &mut GetCoeffs as *mut GetCoeffsFunc,
        Some(
            GetCoeffsFast
                as unsafe extern "C" fn(
                    *mut VP8BitReader,
                    *const *const VP8BandProbas,
                    libc::c_int,
                    *const libc::c_int,
                    libc::c_int,
                    *mut int16_t,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn InitGetCoeffs() {
    static mut InitGetCoeffs_body_lock: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int as libc::c_short,
                __elision: 0 as libc::c_int as libc::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    if !(pthread_mutex_lock(&mut InitGetCoeffs_body_lock) != 0) {
        InitGetCoeffs_body();
        pthread_mutex_unlock(&mut InitGetCoeffs_body_lock);
    }
}
#[inline]
unsafe extern "C" fn NzCodeBits(
    mut nz_coeffs: uint32_t,
    mut nz: libc::c_int,
    mut dc_nz: libc::c_int,
) -> uint32_t {
    nz_coeffs <<= 2 as libc::c_int;
    nz_coeffs
        |= (if nz > 3 as libc::c_int {
            3 as libc::c_int
        } else if nz > 1 as libc::c_int {
            2 as libc::c_int
        } else {
            dc_nz
        }) as libc::c_uint;
    return nz_coeffs;
}
unsafe extern "C" fn ParseResiduals(
    dec: *mut VP8Decoder,
    mb: *mut VP8MB,
    token_br: *mut VP8BitReader,
) -> libc::c_int {
    let bands: *mut [*const VP8BandProbas; 17] = ((*dec).proba_.bands_ptr_).as_mut_ptr();
    let mut ac_proba: *const *const VP8BandProbas = 0 as *const *const VP8BandProbas;
    let block: *mut VP8MBData = ((*dec).mb_data_).offset((*dec).mb_x_ as isize);
    let q: *const VP8QuantMatrix = &mut *((*dec).dqm_)
        .as_mut_ptr()
        .offset((*block).segment_ as isize) as *mut VP8QuantMatrix;
    let mut dst: *mut int16_t = ((*block).coeffs_).as_mut_ptr();
    let left_mb: *mut VP8MB = ((*dec).mb_info_).offset(-(1 as libc::c_int as isize));
    let mut tnz: uint8_t = 0;
    let mut lnz: uint8_t = 0;
    let mut non_zero_y: uint32_t = 0 as libc::c_int as uint32_t;
    let mut non_zero_uv: uint32_t = 0 as libc::c_int as uint32_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut out_t_nz: uint32_t = 0;
    let mut out_l_nz: uint32_t = 0;
    let mut first: libc::c_int = 0;
    memset(
        dst as *mut libc::c_void,
        0 as libc::c_int,
        (384 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
    );
    if (*block).is_i4x4_ == 0 {
        let mut dc: [int16_t; 16] = [
            0 as libc::c_int as int16_t,
            0,
            0,
            0,
            0,
            0,
            0,
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
        let ctx: libc::c_int = (*mb).nz_dc_ as libc::c_int
            + (*left_mb).nz_dc_ as libc::c_int;
        let nz: libc::c_int = GetCoeffs
            .expect(
                "non-null function pointer",
            )(
            token_br,
            (*bands.offset(1 as libc::c_int as isize)).as_mut_ptr()
                as *const *const VP8BandProbas,
            ctx,
            ((*q).y2_mat_).as_ptr(),
            0 as libc::c_int,
            dc.as_mut_ptr(),
        );
        (*left_mb).nz_dc_ = (nz > 0 as libc::c_int) as libc::c_int as uint8_t;
        (*mb).nz_dc_ = (*left_mb).nz_dc_;
        if nz > 1 as libc::c_int {
            VP8TransformWHT.expect("non-null function pointer")(dc.as_mut_ptr(), dst);
        } else {
            let mut i: libc::c_int = 0;
            let dc0: libc::c_int = dc[0 as libc::c_int as usize] as libc::c_int
                + 3 as libc::c_int >> 3 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int * 16 as libc::c_int {
                *dst.offset(i as isize) = dc0 as int16_t;
                i += 16 as libc::c_int;
            }
        }
        first = 1 as libc::c_int;
        ac_proba = (*bands.offset(0 as libc::c_int as isize)).as_mut_ptr();
    } else {
        first = 0 as libc::c_int;
        ac_proba = (*bands.offset(3 as libc::c_int as isize)).as_mut_ptr();
    }
    tnz = ((*mb).nz_ as libc::c_int & 0xf as libc::c_int) as uint8_t;
    lnz = ((*left_mb).nz_ as libc::c_int & 0xf as libc::c_int) as uint8_t;
    y = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        let mut l: libc::c_int = lnz as libc::c_int & 1 as libc::c_int;
        let mut nz_coeffs: uint32_t = 0 as libc::c_int as uint32_t;
        x = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            let ctx_0: libc::c_int = l + (tnz as libc::c_int & 1 as libc::c_int);
            let nz_0: libc::c_int = GetCoeffs
                .expect(
                    "non-null function pointer",
                )(token_br, ac_proba, ctx_0, ((*q).y1_mat_).as_ptr(), first, dst);
            l = (nz_0 > first) as libc::c_int;
            tnz = (tnz as libc::c_int >> 1 as libc::c_int | l << 7 as libc::c_int)
                as uint8_t;
            nz_coeffs = NzCodeBits(
                nz_coeffs,
                nz_0,
                (*dst.offset(0 as libc::c_int as isize) as libc::c_int
                    != 0 as libc::c_int) as libc::c_int,
            );
            dst = dst.offset(16 as libc::c_int as isize);
            x += 1;
            x;
        }
        tnz = (tnz as libc::c_int >> 4 as libc::c_int) as uint8_t;
        lnz = (lnz as libc::c_int >> 1 as libc::c_int | l << 7 as libc::c_int)
            as uint8_t;
        non_zero_y = non_zero_y << 8 as libc::c_int | nz_coeffs;
        y += 1;
        y;
    }
    out_t_nz = tnz as uint32_t;
    out_l_nz = (lnz as libc::c_int >> 4 as libc::c_int) as uint32_t;
    ch = 0 as libc::c_int;
    while ch < 4 as libc::c_int {
        let mut nz_coeffs_0: uint32_t = 0 as libc::c_int as uint32_t;
        tnz = ((*mb).nz_ as libc::c_int >> 4 as libc::c_int + ch) as uint8_t;
        lnz = ((*left_mb).nz_ as libc::c_int >> 4 as libc::c_int + ch) as uint8_t;
        y = 0 as libc::c_int;
        while y < 2 as libc::c_int {
            let mut l_0: libc::c_int = lnz as libc::c_int & 1 as libc::c_int;
            x = 0 as libc::c_int;
            while x < 2 as libc::c_int {
                let ctx_1: libc::c_int = l_0 + (tnz as libc::c_int & 1 as libc::c_int);
                let nz_1: libc::c_int = GetCoeffs
                    .expect(
                        "non-null function pointer",
                    )(
                    token_br,
                    (*bands.offset(2 as libc::c_int as isize)).as_mut_ptr()
                        as *const *const VP8BandProbas,
                    ctx_1,
                    ((*q).uv_mat_).as_ptr(),
                    0 as libc::c_int,
                    dst,
                );
                l_0 = (nz_1 > 0 as libc::c_int) as libc::c_int;
                tnz = (tnz as libc::c_int >> 1 as libc::c_int | l_0 << 3 as libc::c_int)
                    as uint8_t;
                nz_coeffs_0 = NzCodeBits(
                    nz_coeffs_0,
                    nz_1,
                    (*dst.offset(0 as libc::c_int as isize) as libc::c_int
                        != 0 as libc::c_int) as libc::c_int,
                );
                dst = dst.offset(16 as libc::c_int as isize);
                x += 1;
                x;
            }
            tnz = (tnz as libc::c_int >> 2 as libc::c_int) as uint8_t;
            lnz = (lnz as libc::c_int >> 1 as libc::c_int | l_0 << 5 as libc::c_int)
                as uint8_t;
            y += 1;
            y;
        }
        non_zero_uv |= nz_coeffs_0 << 4 as libc::c_int * ch;
        out_t_nz |= (((tnz as libc::c_int) << 4 as libc::c_int) << ch) as libc::c_uint;
        out_l_nz |= ((lnz as libc::c_int & 0xf0 as libc::c_int) << ch) as libc::c_uint;
        ch += 2 as libc::c_int;
    }
    (*mb).nz_ = out_t_nz as uint8_t;
    (*left_mb).nz_ = out_l_nz as uint8_t;
    (*block).non_zero_y_ = non_zero_y;
    (*block).non_zero_uv_ = non_zero_uv;
    (*block)
        .dither_ = (if non_zero_uv & 0xaaaa as libc::c_int as libc::c_uint != 0 {
        0 as libc::c_int
    } else {
        (*q).dither_
    }) as uint8_t;
    return (non_zero_y | non_zero_uv == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8DecodeMB(
    dec: *mut VP8Decoder,
    token_br: *mut VP8BitReader,
) -> libc::c_int {
    let left: *mut VP8MB = ((*dec).mb_info_).offset(-(1 as libc::c_int as isize));
    let mb: *mut VP8MB = ((*dec).mb_info_).offset((*dec).mb_x_ as isize);
    let block: *mut VP8MBData = ((*dec).mb_data_).offset((*dec).mb_x_ as isize);
    let mut skip: libc::c_int = if (*dec).use_skip_proba_ != 0 {
        (*block).skip_ as libc::c_int
    } else {
        0 as libc::c_int
    };
    if skip == 0 {
        skip = ParseResiduals(dec, mb, token_br);
    } else {
        (*mb).nz_ = 0 as libc::c_int as uint8_t;
        (*left).nz_ = (*mb).nz_;
        if (*block).is_i4x4_ == 0 {
            (*mb).nz_dc_ = 0 as libc::c_int as uint8_t;
            (*left).nz_dc_ = (*mb).nz_dc_;
        }
        (*block).non_zero_y_ = 0 as libc::c_int as uint32_t;
        (*block).non_zero_uv_ = 0 as libc::c_int as uint32_t;
        (*block).dither_ = 0 as libc::c_int as uint8_t;
    }
    if (*dec).filter_type_ > 0 as libc::c_int {
        let finfo: *mut VP8FInfo = ((*dec).f_info_).offset((*dec).mb_x_ as isize);
        *finfo = (*dec)
            .fstrengths_[(*block).segment_ as usize][(*block).is_i4x4_ as usize];
        (*finfo)
            .f_inner_ = ((*finfo).f_inner_ as libc::c_int | (skip == 0) as libc::c_int)
            as uint8_t;
    }
    return ((*token_br).eof_ == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitScanline(dec: *mut VP8Decoder) {
    let left: *mut VP8MB = ((*dec).mb_info_).offset(-(1 as libc::c_int as isize));
    (*left).nz_ = 0 as libc::c_int as uint8_t;
    (*left).nz_dc_ = 0 as libc::c_int as uint8_t;
    memset(
        ((*dec).intra_l_).as_mut_ptr() as *mut libc::c_void,
        B_DC_PRED as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong,
    );
    (*dec).mb_x_ = 0 as libc::c_int;
}
unsafe extern "C" fn ParseFrame(
    dec: *mut VP8Decoder,
    mut io: *mut VP8Io,
) -> libc::c_int {
    (*dec).mb_y_ = 0 as libc::c_int;
    while (*dec).mb_y_ < (*dec).br_mb_y_ {
        let token_br: *mut VP8BitReader = &mut *((*dec).parts_)
            .as_mut_ptr()
            .offset(
                ((*dec).mb_y_ as libc::c_uint & (*dec).num_parts_minus_one_) as isize,
            ) as *mut VP8BitReader;
        if VP8ParseIntraModeRow(&mut (*dec).br_, dec) == 0 {
            return VP8SetError(
                dec,
                VP8_STATUS_NOT_ENOUGH_DATA,
                b"Premature end-of-partition0 encountered.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        while (*dec).mb_x_ < (*dec).mb_w_ {
            if VP8DecodeMB(dec, token_br) == 0 {
                return VP8SetError(
                    dec,
                    VP8_STATUS_NOT_ENOUGH_DATA,
                    b"Premature end-of-file encountered.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*dec).mb_x_ += 1;
            (*dec).mb_x_;
        }
        VP8InitScanline(dec);
        if VP8ProcessRow(dec, io) == 0 {
            return VP8SetError(
                dec,
                VP8_STATUS_USER_ABORT,
                b"Output aborted.\0" as *const u8 as *const libc::c_char,
            );
        }
        (*dec).mb_y_ += 1;
        (*dec).mb_y_;
    }
    if (*dec).mt_method_ > 0 as libc::c_int {
        if ((*WebPGetWorkerInterface()).Sync_0)
            .expect("non-null function pointer")(&mut (*dec).worker_) == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Decode(dec: *mut VP8Decoder, io: *mut VP8Io) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    if dec.is_null() {
        return 0 as libc::c_int;
    }
    if io.is_null() {
        return VP8SetError(
            dec,
            VP8_STATUS_INVALID_PARAM,
            b"NULL VP8Io parameter in VP8Decode().\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*dec).ready_ == 0 {
        if VP8GetHeaders(dec, io) == 0 {
            return 0 as libc::c_int;
        }
    }
    ok = (VP8EnterCritical(dec, io) as libc::c_uint
        == VP8_STATUS_OK as libc::c_int as libc::c_uint) as libc::c_int;
    if ok != 0 {
        if ok != 0 {
            ok = VP8InitFrame(dec, io);
        }
        if ok != 0 {
            ok = ParseFrame(dec, io);
        }
        ok &= VP8ExitCritical(dec, io);
    }
    if ok == 0 {
        VP8Clear(dec);
        return 0 as libc::c_int;
    }
    (*dec).ready_ = 0 as libc::c_int;
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Clear(dec: *mut VP8Decoder) {
    if dec.is_null() {
        return;
    }
    ((*WebPGetWorkerInterface()).End)
        .expect("non-null function pointer")(&mut (*dec).worker_);
    WebPDeallocateAlphaMemory(dec);
    WebPSafeFree((*dec).mem_);
    (*dec).mem_ = 0 as *mut libc::c_void;
    (*dec).mem_size_ = 0 as libc::c_int as size_t;
    memset(
        &mut (*dec).br_ as *mut VP8BitReader as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8BitReader>() as libc::c_ulong,
    );
    (*dec).ready_ = 0 as libc::c_int;
}
