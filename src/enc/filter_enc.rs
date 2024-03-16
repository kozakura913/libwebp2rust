use ::libc;
use ::c2rust_bitfields;

use super::token_enc::VP8Tokens;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut VP8VFilter8i: VP8ChromaFilterFunc;
    static mut VP8HFilter16i: VP8LumaFilterFunc;
    static mut VP8VFilter16i: VP8LumaFilterFunc;
    static mut VP8SSIMGetClipped: VP8SSIMGetClippedFunc;
    fn VP8SSIMDspInit();
    static mut VP8SimpleVFilter16i: VP8SimpleFilterFunc;
    static mut VP8SimpleHFilter16i: VP8SimpleFilterFunc;
    static mut VP8HFilter8i: VP8ChromaFilterFunc;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const NUM_PROBAS: C2RustUnnamed = 11;
pub const NUM_CTX: C2RustUnnamed = 3;
pub const NUM_BANDS: C2RustUnnamed = 8;
pub const NUM_TYPES: C2RustUnnamed = 4;
pub const MAX_NUM_PARTITIONS: C2RustUnnamed = 8;
pub const NUM_MODE_LF_DELTAS: C2RustUnnamed = 4;
pub const NUM_REF_LF_DELTAS: C2RustUnnamed = 4;
pub const NUM_MB_SEGMENTS: C2RustUnnamed = 4;
pub const MB_FEATURE_TREE_PROBS: C2RustUnnamed = 3;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Matrix {
    pub q_: [uint16_t; 16],
    pub iq_: [uint16_t; 16],
    pub bias_: [uint32_t; 16],
    pub zthresh_: [uint32_t; 16],
    pub sharpen_: [uint16_t; 16],
}
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
pub type VP8SimpleFilterFunc = Option::<
    unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int) -> (),
>;
pub type VP8LumaFilterFunc = Option::<
    unsafe extern "C" fn(
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type VP8ChromaFilterFunc = Option::<
    unsafe extern "C" fn(
        *mut uint8_t,
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8BitWriter {
    pub range_: int32_t,
    pub value_: int32_t,
    pub run_: libc::c_int,
    pub nb_bits_: libc::c_int,
    pub buf_: *mut uint8_t,
    pub pos_: size_t,
    pub max_pos_: size_t,
    pub error_: libc::c_int,
}
pub type WebPWorkerStatus = libc::c_uint;
pub const WORK: WebPWorkerStatus = 2;
pub const OK: WebPWorkerStatus = 1;
pub const NOT_OK: WebPWorkerStatus = 0;
pub type WebPWorkerHook = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MAX_LEVEL: C2RustUnnamed_0 = 2047;
pub const MAX_VARIABLE_LEVEL: C2RustUnnamed_0 = 67;
pub const MAX_LF_LEVELS: C2RustUnnamed_0 = 64;
pub type VP8RDLevel = libc::c_uint;
pub const RD_OPT_TRELLIS_ALL: VP8RDLevel = 3;
pub const RD_OPT_TRELLIS: VP8RDLevel = 2;
pub const RD_OPT_BASIC: VP8RDLevel = 1;
pub const RD_OPT_NONE: VP8RDLevel = 0;
pub type score_t = int64_t;
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
pub type StatsArray = [[proba_t; 11]; 3];
pub type CostArray = [[uint16_t; 68]; 3];
pub type CostArrayMap = [[*const uint16_t; 3]; 16];
pub type LFStats = [[libc::c_double; 64]; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Encoder {
    pub config_: *const WebPConfig,
    pub pic_: *mut WebPPicture,
    pub filter_hdr_: VP8EncFilterHeader,
    pub segment_hdr_: VP8EncSegmentHeader,
    pub profile_: libc::c_int,
    pub mb_w_: libc::c_int,
    pub mb_h_: libc::c_int,
    pub preds_w_: libc::c_int,
    pub num_parts_: libc::c_int,
    pub bw_: VP8BitWriter,
    pub parts_: [VP8BitWriter; 8],
    pub tokens_: VP8TBuffer,
    pub percent_: libc::c_int,
    pub has_alpha_: libc::c_int,
    pub alpha_data_: *mut uint8_t,
    pub alpha_data_size_: uint32_t,
    pub alpha_worker_: WebPWorker,
    pub dqm_: [VP8SegmentInfo; 4],
    pub base_quant_: libc::c_int,
    pub alpha_: libc::c_int,
    pub uv_alpha_: libc::c_int,
    pub dq_y1_dc_: libc::c_int,
    pub dq_y2_dc_: libc::c_int,
    pub dq_y2_ac_: libc::c_int,
    pub dq_uv_dc_: libc::c_int,
    pub dq_uv_ac_: libc::c_int,
    pub proba_: VP8EncProba,
    pub sse_: [uint64_t; 4],
    pub sse_count_: uint64_t,
    pub coded_size_: libc::c_int,
    pub residual_bytes_: [[libc::c_int; 4]; 3],
    pub block_count_: [libc::c_int; 3],
    pub method_: libc::c_int,
    pub rd_opt_level_: VP8RDLevel,
    pub max_i4_header_bits_: libc::c_int,
    pub mb_header_limit_: libc::c_int,
    pub thread_level_: libc::c_int,
    pub do_search_: libc::c_int,
    pub use_tokens_: libc::c_int,
    pub mb_info_: *mut VP8MBInfo,
    pub preds_: *mut uint8_t,
    pub nz_: *mut uint32_t,
    pub y_top_: *mut uint8_t,
    pub uv_top_: *mut uint8_t,
    pub lf_stats_: *mut LFStats,
    pub top_derr_: *mut DError,
}
pub type DError = [[int8_t; 2]; 2];
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VP8MBInfo {
    #[bitfield(name = "type_", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "uv_mode_", ty = "libc::c_uint", bits = "2..=3")]
    #[bitfield(name = "skip_", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "segment_", ty = "libc::c_uint", bits = "5..=6")]
    pub type__uv_mode__skip__segment_: [u8; 1],
    pub alpha_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncProba {
    pub segments_: [uint8_t; 3],
    pub skip_proba_: uint8_t,
    pub coeffs_: [[ProbaArray; 8]; 4],
    pub stats_: [[StatsArray; 8]; 4],
    pub level_cost_: [[CostArray; 8]; 4],
    pub remapped_costs_: [CostArrayMap; 4],
    pub dirty_: libc::c_int,
    pub use_skip_proba_: libc::c_int,
    pub nb_skip_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8SegmentInfo {
    pub y1_: VP8Matrix,
    pub y2_: VP8Matrix,
    pub uv_: VP8Matrix,
    pub alpha_: libc::c_int,
    pub beta_: libc::c_int,
    pub quant_: libc::c_int,
    pub fstrength_: libc::c_int,
    pub max_edge_: libc::c_int,
    pub min_disto_: libc::c_int,
    pub lambda_i16_: libc::c_int,
    pub lambda_i4_: libc::c_int,
    pub lambda_uv_: libc::c_int,
    pub lambda_mode_: libc::c_int,
    pub lambda_trellis_: libc::c_int,
    pub tlambda_: libc::c_int,
    pub lambda_trellis_i16_: libc::c_int,
    pub lambda_trellis_i4_: libc::c_int,
    pub lambda_trellis_uv_: libc::c_int,
    pub i4_penalty_: score_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8TBuffer {
    pub pages_: *mut VP8Tokens,
    pub last_page_: *mut *mut VP8Tokens,
    pub tokens_: *mut uint16_t,
    pub left_: libc::c_int,
    pub page_size_: libc::c_int,
    pub error_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncSegmentHeader {
    pub num_segments_: libc::c_int,
    pub update_map_: libc::c_int,
    pub size_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncFilterHeader {
    pub simple_: libc::c_int,
    pub level_: libc::c_int,
    pub sharpness_: libc::c_int,
    pub i4x4_lf_delta_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8EncIterator {
    pub x_: libc::c_int,
    pub y_: libc::c_int,
    pub yuv_in_: *mut uint8_t,
    pub yuv_out_: *mut uint8_t,
    pub yuv_out2_: *mut uint8_t,
    pub yuv_p_: *mut uint8_t,
    pub enc_: *mut VP8Encoder,
    pub mb_: *mut VP8MBInfo,
    pub bw_: *mut VP8BitWriter,
    pub preds_: *mut uint8_t,
    pub nz_: *mut uint32_t,
    pub i4_boundary_: [uint8_t; 37],
    pub i4_top_: *mut uint8_t,
    pub i4_: libc::c_int,
    pub top_nz_: [libc::c_int; 9],
    pub left_nz_: [libc::c_int; 9],
    pub bit_count_: [[uint64_t; 3]; 4],
    pub luma_bits_: uint64_t,
    pub uv_bits_: uint64_t,
    pub lf_stats_: *mut LFStats,
    pub do_trellis_: libc::c_int,
    pub count_down_: libc::c_int,
    pub count_down0_: libc::c_int,
    pub percent0_: libc::c_int,
    pub left_derr_: DError,
    pub top_derr_: *mut DError,
    pub y_left_: *mut uint8_t,
    pub u_left_: *mut uint8_t,
    pub v_left_: *mut uint8_t,
    pub y_top_: *mut uint8_t,
    pub uv_top_: *mut uint8_t,
    pub yuv_left_mem_: [uint8_t; 88],
    pub yuv_mem_: [uint8_t; 3359],
}
static mut kLevelsFromDelta: [[uint8_t; 64]; 8] = [
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        37 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        40 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        43 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        46 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        49 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        55 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        58 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        61 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        37 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        40 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        43 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        46 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        49 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        55 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        58 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        61 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        37 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        40 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        43 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        46 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        49 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        55 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        58 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        61 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        37 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        40 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        43 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        46 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        49 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        55 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        58 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        61 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        37 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        40 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        43 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        46 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        49 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        55 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        58 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        61 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn VP8FilterStrengthFromDelta(
    mut sharpness: libc::c_int,
    mut delta: libc::c_int,
) -> libc::c_int {
    let pos: libc::c_int = if delta < 64 as libc::c_int {
        delta
    } else {
        64 as libc::c_int - 1 as libc::c_int
    };
    return kLevelsFromDelta[sharpness as usize][pos as usize] as libc::c_int;
}
unsafe extern "C" fn GetILevel(
    mut sharpness: libc::c_int,
    mut level: libc::c_int,
) -> libc::c_int {
    if sharpness > 0 as libc::c_int {
        if sharpness > 4 as libc::c_int {
            level >>= 2 as libc::c_int;
        } else {
            level >>= 1 as libc::c_int;
        }
        if level > 9 as libc::c_int - sharpness {
            level = 9 as libc::c_int - sharpness;
        }
    }
    if level < 1 as libc::c_int {
        level = 1 as libc::c_int;
    }
    return level;
}
unsafe extern "C" fn DoFilter(it: *const VP8EncIterator, mut level: libc::c_int) {
    let enc: *const VP8Encoder = (*it).enc_;
    let ilevel: libc::c_int = GetILevel((*(*enc).config_).filter_sharpness, level);
    let limit: libc::c_int = 2 as libc::c_int * level + ilevel;
    let y_dst: *mut uint8_t = ((*it).yuv_out2_).offset(0 as libc::c_int as isize);
    let u_dst: *mut uint8_t = ((*it).yuv_out2_).offset(16 as libc::c_int as isize);
    let v_dst: *mut uint8_t = ((*it).yuv_out2_)
        .offset((16 as libc::c_int + 8 as libc::c_int) as isize);
    memcpy(
        y_dst as *mut libc::c_void,
        (*it).yuv_out_ as *const libc::c_void,
        ((32 as libc::c_int * 16 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
    );
    if (*enc).filter_hdr_.simple_ == 1 as libc::c_int {
        VP8SimpleHFilter16i
            .expect("non-null function pointer")(y_dst, 32 as libc::c_int, limit);
        VP8SimpleVFilter16i
            .expect("non-null function pointer")(y_dst, 32 as libc::c_int, limit);
    } else {
        let hev_thresh: libc::c_int = if level >= 40 as libc::c_int {
            2 as libc::c_int
        } else if level >= 15 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        VP8HFilter16i
            .expect(
                "non-null function pointer",
            )(y_dst, 32 as libc::c_int, limit, ilevel, hev_thresh);
        VP8HFilter8i
            .expect(
                "non-null function pointer",
            )(u_dst, v_dst, 32 as libc::c_int, limit, ilevel, hev_thresh);
        VP8VFilter16i
            .expect(
                "non-null function pointer",
            )(y_dst, 32 as libc::c_int, limit, ilevel, hev_thresh);
        VP8VFilter8i
            .expect(
                "non-null function pointer",
            )(u_dst, v_dst, 32 as libc::c_int, limit, ilevel, hev_thresh);
    };
}
unsafe extern "C" fn GetMBSSIM(
    mut yuv1: *const uint8_t,
    mut yuv2: *const uint8_t,
) -> libc::c_double {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    y = 3 as libc::c_int;
    while y < 16 as libc::c_int - 3 as libc::c_int {
        x = 3 as libc::c_int;
        while x < 16 as libc::c_int - 3 as libc::c_int {
            sum
                += VP8SSIMGetClipped
                    .expect(
                        "non-null function pointer",
                    )(
                    yuv1.offset(0 as libc::c_int as isize),
                    32 as libc::c_int,
                    yuv2.offset(0 as libc::c_int as isize),
                    32 as libc::c_int,
                    x,
                    y,
                    16 as libc::c_int,
                    16 as libc::c_int,
                );
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    x = 1 as libc::c_int;
    while x < 7 as libc::c_int {
        y = 1 as libc::c_int;
        while y < 7 as libc::c_int {
            sum
                += VP8SSIMGetClipped
                    .expect(
                        "non-null function pointer",
                    )(
                    yuv1.offset(16 as libc::c_int as isize),
                    32 as libc::c_int,
                    yuv2.offset(16 as libc::c_int as isize),
                    32 as libc::c_int,
                    x,
                    y,
                    8 as libc::c_int,
                    8 as libc::c_int,
                );
            sum
                += VP8SSIMGetClipped
                    .expect(
                        "non-null function pointer",
                    )(
                    yuv1.offset((16 as libc::c_int + 8 as libc::c_int) as isize),
                    32 as libc::c_int,
                    yuv2.offset((16 as libc::c_int + 8 as libc::c_int) as isize),
                    32 as libc::c_int,
                    x,
                    y,
                    8 as libc::c_int,
                    8 as libc::c_int,
                );
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitFilter(it: *mut VP8EncIterator) {
    if !((*it).lf_stats_).is_null() {
        let mut s: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        s = 0 as libc::c_int;
        while s < NUM_MB_SEGMENTS as libc::c_int {
            i = 0 as libc::c_int;
            while i < MAX_LF_LEVELS as libc::c_int {
                (*(*it)
                    .lf_stats_)[s
                    as usize][i as usize] = 0 as libc::c_int as libc::c_double;
                i += 1;
                i;
            }
            s += 1;
            s;
        }
        VP8SSIMDspInit();
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8StoreFilterStats(it: *mut VP8EncIterator) {
    let mut d: libc::c_int = 0;
    let enc: *mut VP8Encoder = (*it).enc_;
    let s: libc::c_int = (*(*it).mb_).segment_() as libc::c_int;
    let level0: libc::c_int = (*enc).dqm_[s as usize].fstrength_;
    let delta_min: libc::c_int = -(*enc).dqm_[s as usize].quant_;
    let delta_max: libc::c_int = (*enc).dqm_[s as usize].quant_;
    let step_size: libc::c_int = if delta_max - delta_min >= 4 as libc::c_int {
        4 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if ((*it).lf_stats_).is_null() {
        return;
    }
    if (*(*it).mb_).type_() as libc::c_int == 1 as libc::c_int
        && (*(*it).mb_).skip_() as libc::c_int != 0
    {
        return;
    }
    (*(*it).lf_stats_)[s as usize][0 as libc::c_int as usize]
        += GetMBSSIM((*it).yuv_in_, (*it).yuv_out_);
    d = delta_min;
    while d <= delta_max {
        let level: libc::c_int = level0 + d;
        if !(level <= 0 as libc::c_int || level >= MAX_LF_LEVELS as libc::c_int) {
            DoFilter(it, level);
            (*(*it).lf_stats_)[s as usize][level as usize]
                += GetMBSSIM((*it).yuv_in_, (*it).yuv_out2_);
        }
        d += step_size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8AdjustFilterStrength(it: *mut VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    if !((*it).lf_stats_).is_null() {
        let mut s: libc::c_int = 0;
        s = 0 as libc::c_int;
        while s < NUM_MB_SEGMENTS as libc::c_int {
            let mut i: libc::c_int = 0;
            let mut best_level: libc::c_int = 0 as libc::c_int;
            let mut best_v: libc::c_double = 1.00001f64
                * (*(*it).lf_stats_)[s as usize][0 as libc::c_int as usize];
            i = 1 as libc::c_int;
            while i < MAX_LF_LEVELS as libc::c_int {
                let v: libc::c_double = (*(*it).lf_stats_)[s as usize][i as usize];
                if v > best_v {
                    best_v = v;
                    best_level = i;
                }
                i += 1;
                i;
            }
            (*enc).dqm_[s as usize].fstrength_ = best_level;
            s += 1;
            s;
        }
        return;
    }
    if (*(*enc).config_).filter_strength > 0 as libc::c_int {
        let mut max_level: libc::c_int = 0 as libc::c_int;
        let mut s_0: libc::c_int = 0;
        s_0 = 0 as libc::c_int;
        while s_0 < NUM_MB_SEGMENTS as libc::c_int {
            let dqm: *mut VP8SegmentInfo = &mut *((*enc).dqm_)
                .as_mut_ptr()
                .offset(s_0 as isize) as *mut VP8SegmentInfo;
            let delta: libc::c_int = (*dqm).max_edge_
                * (*dqm).y2_.q_[1 as libc::c_int as usize] as libc::c_int
                >> 3 as libc::c_int;
            let level: libc::c_int = VP8FilterStrengthFromDelta(
                (*enc).filter_hdr_.sharpness_,
                delta,
            );
            if level > (*dqm).fstrength_ {
                (*dqm).fstrength_ = level;
            }
            if max_level < (*dqm).fstrength_ {
                max_level = (*dqm).fstrength_;
            }
            s_0 += 1;
            s_0;
        }
        (*enc).filter_hdr_.level_ = max_level;
    }
}
