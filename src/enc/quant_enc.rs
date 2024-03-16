use ::libc;
use ::c2rust_bitfields;

use super::token_enc::VP8Tokens;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut VP8ITransform: VP8Idct;
    static mut VP8FTransform: VP8Fdct;
    static mut VP8FTransform2: VP8Fdct;
    static mut VP8FTransformWHT: VP8WHT;
    static mut VP8EncPredLuma4: VP8Intra4Preds;
    static mut VP8EncPredLuma16: VP8IntraPreds;
    static mut VP8EncPredChroma8: VP8IntraPreds;
    static mut VP8SSE16x16: VP8Metric;
    static mut VP8SSE16x8: VP8Metric;
    static mut VP8SSE4x4: VP8Metric;
    static mut VP8TDisto4x4: VP8WMetric;
    static mut VP8TDisto16x16: VP8WMetric;
    static mut VP8Copy4x4: VP8BlockCopy;
    static mut VP8Copy16x8: VP8BlockCopy;
    static mut VP8EncQuantizeBlock: VP8QuantizeBlock;
    static mut VP8EncQuantize2Blocks: VP8Quantize2Blocks;
    static mut VP8EncQuantizeBlockWHT: VP8QuantizeBlockWHT;
    static VP8EntropyCost: [uint16_t; 256];
    static VP8LevelFixedCosts: [uint16_t; 2048];
    static VP8EncBands: [uint8_t; 17];
    static mut VP8TransformWHT: VP8WHT;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VP8GetCostLuma4(it: *mut VP8EncIterator, levels: *const int16_t) -> libc::c_int;
    fn VP8IteratorStartI4(it: *mut VP8EncIterator);
    fn VP8IteratorRotateI4(
        it: *mut VP8EncIterator,
        yuv_out: *const uint8_t,
    ) -> libc::c_int;
    fn VP8IteratorNzToBytes(it: *mut VP8EncIterator);
    fn VP8SetIntra16Mode(it: *const VP8EncIterator, mode: libc::c_int);
    fn VP8SetIntra4Mode(it: *const VP8EncIterator, modes: *const uint8_t);
    fn VP8SetIntraUVMode(it: *const VP8EncIterator, mode: libc::c_int);
    fn VP8SetSkip(it: *const VP8EncIterator, skip: libc::c_int);
    fn VP8GetCostLuma16(it: *mut VP8EncIterator, rd: *const VP8ModeScore) -> libc::c_int;
    fn VP8GetCostUV(it: *mut VP8EncIterator, rd: *const VP8ModeScore) -> libc::c_int;
    fn VP8FilterStrengthFromDelta(
        sharpness: libc::c_int,
        delta: libc::c_int,
    ) -> libc::c_int;
    static VP8FixedCostsUV: [uint16_t; 4];
    static VP8FixedCostsI16: [uint16_t; 4];
    static VP8FixedCostsI4: [[[uint16_t; 10]; 10]; 10];
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type VP8Idct = Option::<
    unsafe extern "C" fn(*const uint8_t, *const int16_t, *mut uint8_t, libc::c_int) -> (),
>;
pub type VP8Fdct = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut int16_t) -> (),
>;
pub type VP8WHT = Option::<unsafe extern "C" fn(*const int16_t, *mut int16_t) -> ()>;
pub type VP8IntraPreds = Option::<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, *const uint8_t) -> (),
>;
pub type VP8Intra4Preds = Option::<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t) -> (),
>;
pub type VP8Metric = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> libc::c_int,
>;
pub type VP8WMetric = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *const uint16_t) -> libc::c_int,
>;
pub type VP8BlockCopy = Option::<
    unsafe extern "C" fn(*const uint8_t, *mut uint8_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Matrix {
    pub q_: [uint16_t; 16],
    pub iq_: [uint16_t; 16],
    pub bias_: [uint32_t; 16],
    pub zthresh_: [uint32_t; 16],
    pub sharpen_: [uint16_t; 16],
}
pub type VP8QuantizeBlock = Option::<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> libc::c_int,
>;
pub type VP8Quantize2Blocks = Option::<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> libc::c_int,
>;
pub type VP8QuantizeBlockWHT = Option::<
    unsafe extern "C" fn(*mut int16_t, *mut int16_t, *const VP8Matrix) -> libc::c_int,
>;
pub type CostArrayPtr = *mut [*const uint16_t; 3];
pub type StatsArray = [[proba_t; 11]; 3];
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
pub type C2RustUnnamed = libc::c_uint;
pub const NUM_B_DC_MODES: C2RustUnnamed = 7;
pub const B_DC_PRED_NOTOPLEFT: C2RustUnnamed = 6;
pub const B_DC_PRED_NOLEFT: C2RustUnnamed = 5;
pub const B_DC_PRED_NOTOP: C2RustUnnamed = 4;
pub const NUM_PRED_MODES: C2RustUnnamed = 4;
pub const B_PRED: C2RustUnnamed = 10;
pub const TM_PRED: C2RustUnnamed = 1;
pub const H_PRED: C2RustUnnamed = 3;
pub const V_PRED: C2RustUnnamed = 2;
pub const DC_PRED: C2RustUnnamed = 0;
pub const NUM_BMODES: C2RustUnnamed = 10;
pub const B_HU_PRED: C2RustUnnamed = 9;
pub const B_HD_PRED: C2RustUnnamed = 8;
pub const B_VL_PRED: C2RustUnnamed = 7;
pub const B_LD_PRED: C2RustUnnamed = 6;
pub const B_VR_PRED: C2RustUnnamed = 5;
pub const B_RD_PRED: C2RustUnnamed = 4;
pub const B_HE_PRED: C2RustUnnamed = 3;
pub const B_VE_PRED: C2RustUnnamed = 2;
pub const B_TM_PRED: C2RustUnnamed = 1;
pub const B_DC_PRED: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUM_PROBAS: C2RustUnnamed_0 = 11;
pub const NUM_CTX: C2RustUnnamed_0 = 3;
pub const NUM_BANDS: C2RustUnnamed_0 = 8;
pub const NUM_TYPES: C2RustUnnamed_0 = 4;
pub const MAX_NUM_PARTITIONS: C2RustUnnamed_0 = 8;
pub const NUM_MODE_LF_DELTAS: C2RustUnnamed_0 = 4;
pub const NUM_REF_LF_DELTAS: C2RustUnnamed_0 = 4;
pub const NUM_MB_SEGMENTS: C2RustUnnamed_0 = 4;
pub const MB_FEATURE_TREE_PROBS: C2RustUnnamed_0 = 3;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MAX_LEVEL: C2RustUnnamed_1 = 2047;
pub const MAX_VARIABLE_LEVEL: C2RustUnnamed_1 = 67;
pub const MAX_LF_LEVELS: C2RustUnnamed_1 = 64;
pub type VP8RDLevel = libc::c_uint;
pub const RD_OPT_TRELLIS_ALL: VP8RDLevel = 3;
pub const RD_OPT_TRELLIS: VP8RDLevel = 2;
pub const RD_OPT_BASIC: VP8RDLevel = 1;
pub const RD_OPT_NONE: VP8RDLevel = 0;
pub type score_t = int64_t;
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
pub struct VP8ModeScore {
    pub D: score_t,
    pub SD: score_t,
    pub H: score_t,
    pub R: score_t,
    pub score: score_t,
    pub y_dc_levels: [int16_t; 16],
    pub y_ac_levels: [[int16_t; 16]; 16],
    pub uv_levels: [[int16_t; 16]; 8],
    pub mode_i16: libc::c_int,
    pub modes_i4: [uint8_t; 16],
    pub mode_uv: libc::c_int,
    pub nz: uint32_t,
    pub derr: [[int8_t; 3]; 2],
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
pub const TYPE_CHROMA_A: C2RustUnnamed_2 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub prev: int8_t,
    pub sign: int8_t,
    pub level: int16_t,
}
pub const TYPE_I16_AC: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScoreState {
    pub score: score_t,
    pub costs: *const uint16_t,
}
pub const TYPE_I4_AC: C2RustUnnamed_2 = 3;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const TYPE_I16_DC: C2RustUnnamed_2 = 1;
#[inline]
unsafe extern "C" fn IsFlatSource16(mut src: *const uint8_t) -> libc::c_int {
    let v: uint32_t = (*src.offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if memcmp(
            src.offset(0 as libc::c_int as isize) as *const libc::c_void,
            &v as *const uint32_t as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
            || memcmp(
                src.offset(4 as libc::c_int as isize) as *const libc::c_void,
                &v as *const uint32_t as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) != 0
            || memcmp(
                src.offset(8 as libc::c_int as isize) as *const libc::c_void,
                &v as *const uint32_t as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) != 0
            || memcmp(
                src.offset(12 as libc::c_int as isize) as *const libc::c_void,
                &v as *const uint32_t as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) != 0
        {
            return 0 as libc::c_int;
        }
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn IsFlat_C(
    mut levels: *const int16_t,
    mut num_blocks: libc::c_int,
    mut thresh: libc::c_int,
) -> libc::c_int {
    let mut score: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = num_blocks;
        num_blocks = num_blocks - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i < 16 as libc::c_int {
            score
                += (*levels.offset(i as isize) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int;
            if score > thresh {
                return 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        levels = levels.offset(16 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn QUANTDIV(
    mut n: uint32_t,
    mut iQ: uint32_t,
    mut B: uint32_t,
) -> libc::c_int {
    return (n.wrapping_mul(iQ).wrapping_add(B) >> 17 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8BitCost(
    mut bit: libc::c_int,
    mut proba: uint8_t,
) -> libc::c_int {
    return if bit == 0 {
        VP8EntropyCost[proba as usize] as libc::c_int
    } else {
        VP8EntropyCost[(255 as libc::c_int - proba as libc::c_int) as usize]
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8LevelCost(
    table: *const uint16_t,
    mut level: libc::c_int,
) -> libc::c_int {
    return VP8LevelFixedCosts[level as usize] as libc::c_int
        + *table
            .offset(
                (if level > MAX_VARIABLE_LEVEL as libc::c_int {
                    MAX_VARIABLE_LEVEL as libc::c_int
                } else {
                    level
                }) as isize,
            ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn clip(
    mut v: libc::c_int,
    mut m: libc::c_int,
    mut M: libc::c_int,
) -> libc::c_int {
    return if v < m { m } else if v > M { M } else { v };
}
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
static mut kDcTable: [uint8_t; 128] = [
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
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
    64 as libc::c_int as uint8_t,
    65 as libc::c_int as uint8_t,
    66 as libc::c_int as uint8_t,
    67 as libc::c_int as uint8_t,
    68 as libc::c_int as uint8_t,
    69 as libc::c_int as uint8_t,
    70 as libc::c_int as uint8_t,
    71 as libc::c_int as uint8_t,
    72 as libc::c_int as uint8_t,
    73 as libc::c_int as uint8_t,
    74 as libc::c_int as uint8_t,
    75 as libc::c_int as uint8_t,
    76 as libc::c_int as uint8_t,
    76 as libc::c_int as uint8_t,
    77 as libc::c_int as uint8_t,
    78 as libc::c_int as uint8_t,
    79 as libc::c_int as uint8_t,
    80 as libc::c_int as uint8_t,
    81 as libc::c_int as uint8_t,
    82 as libc::c_int as uint8_t,
    83 as libc::c_int as uint8_t,
    84 as libc::c_int as uint8_t,
    85 as libc::c_int as uint8_t,
    86 as libc::c_int as uint8_t,
    87 as libc::c_int as uint8_t,
    88 as libc::c_int as uint8_t,
    89 as libc::c_int as uint8_t,
    91 as libc::c_int as uint8_t,
    93 as libc::c_int as uint8_t,
    95 as libc::c_int as uint8_t,
    96 as libc::c_int as uint8_t,
    98 as libc::c_int as uint8_t,
    100 as libc::c_int as uint8_t,
    101 as libc::c_int as uint8_t,
    102 as libc::c_int as uint8_t,
    104 as libc::c_int as uint8_t,
    106 as libc::c_int as uint8_t,
    108 as libc::c_int as uint8_t,
    110 as libc::c_int as uint8_t,
    112 as libc::c_int as uint8_t,
    114 as libc::c_int as uint8_t,
    116 as libc::c_int as uint8_t,
    118 as libc::c_int as uint8_t,
    122 as libc::c_int as uint8_t,
    124 as libc::c_int as uint8_t,
    126 as libc::c_int as uint8_t,
    128 as libc::c_int as uint8_t,
    130 as libc::c_int as uint8_t,
    132 as libc::c_int as uint8_t,
    134 as libc::c_int as uint8_t,
    136 as libc::c_int as uint8_t,
    138 as libc::c_int as uint8_t,
    140 as libc::c_int as uint8_t,
    143 as libc::c_int as uint8_t,
    145 as libc::c_int as uint8_t,
    148 as libc::c_int as uint8_t,
    151 as libc::c_int as uint8_t,
    154 as libc::c_int as uint8_t,
    157 as libc::c_int as uint8_t,
];
static mut kAcTable: [uint16_t; 128] = [
    4 as libc::c_int as uint16_t,
    5 as libc::c_int as uint16_t,
    6 as libc::c_int as uint16_t,
    7 as libc::c_int as uint16_t,
    8 as libc::c_int as uint16_t,
    9 as libc::c_int as uint16_t,
    10 as libc::c_int as uint16_t,
    11 as libc::c_int as uint16_t,
    12 as libc::c_int as uint16_t,
    13 as libc::c_int as uint16_t,
    14 as libc::c_int as uint16_t,
    15 as libc::c_int as uint16_t,
    16 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    18 as libc::c_int as uint16_t,
    19 as libc::c_int as uint16_t,
    20 as libc::c_int as uint16_t,
    21 as libc::c_int as uint16_t,
    22 as libc::c_int as uint16_t,
    23 as libc::c_int as uint16_t,
    24 as libc::c_int as uint16_t,
    25 as libc::c_int as uint16_t,
    26 as libc::c_int as uint16_t,
    27 as libc::c_int as uint16_t,
    28 as libc::c_int as uint16_t,
    29 as libc::c_int as uint16_t,
    30 as libc::c_int as uint16_t,
    31 as libc::c_int as uint16_t,
    32 as libc::c_int as uint16_t,
    33 as libc::c_int as uint16_t,
    34 as libc::c_int as uint16_t,
    35 as libc::c_int as uint16_t,
    36 as libc::c_int as uint16_t,
    37 as libc::c_int as uint16_t,
    38 as libc::c_int as uint16_t,
    39 as libc::c_int as uint16_t,
    40 as libc::c_int as uint16_t,
    41 as libc::c_int as uint16_t,
    42 as libc::c_int as uint16_t,
    43 as libc::c_int as uint16_t,
    44 as libc::c_int as uint16_t,
    45 as libc::c_int as uint16_t,
    46 as libc::c_int as uint16_t,
    47 as libc::c_int as uint16_t,
    48 as libc::c_int as uint16_t,
    49 as libc::c_int as uint16_t,
    50 as libc::c_int as uint16_t,
    51 as libc::c_int as uint16_t,
    52 as libc::c_int as uint16_t,
    53 as libc::c_int as uint16_t,
    54 as libc::c_int as uint16_t,
    55 as libc::c_int as uint16_t,
    56 as libc::c_int as uint16_t,
    57 as libc::c_int as uint16_t,
    58 as libc::c_int as uint16_t,
    60 as libc::c_int as uint16_t,
    62 as libc::c_int as uint16_t,
    64 as libc::c_int as uint16_t,
    66 as libc::c_int as uint16_t,
    68 as libc::c_int as uint16_t,
    70 as libc::c_int as uint16_t,
    72 as libc::c_int as uint16_t,
    74 as libc::c_int as uint16_t,
    76 as libc::c_int as uint16_t,
    78 as libc::c_int as uint16_t,
    80 as libc::c_int as uint16_t,
    82 as libc::c_int as uint16_t,
    84 as libc::c_int as uint16_t,
    86 as libc::c_int as uint16_t,
    88 as libc::c_int as uint16_t,
    90 as libc::c_int as uint16_t,
    92 as libc::c_int as uint16_t,
    94 as libc::c_int as uint16_t,
    96 as libc::c_int as uint16_t,
    98 as libc::c_int as uint16_t,
    100 as libc::c_int as uint16_t,
    102 as libc::c_int as uint16_t,
    104 as libc::c_int as uint16_t,
    106 as libc::c_int as uint16_t,
    108 as libc::c_int as uint16_t,
    110 as libc::c_int as uint16_t,
    112 as libc::c_int as uint16_t,
    114 as libc::c_int as uint16_t,
    116 as libc::c_int as uint16_t,
    119 as libc::c_int as uint16_t,
    122 as libc::c_int as uint16_t,
    125 as libc::c_int as uint16_t,
    128 as libc::c_int as uint16_t,
    131 as libc::c_int as uint16_t,
    134 as libc::c_int as uint16_t,
    137 as libc::c_int as uint16_t,
    140 as libc::c_int as uint16_t,
    143 as libc::c_int as uint16_t,
    146 as libc::c_int as uint16_t,
    149 as libc::c_int as uint16_t,
    152 as libc::c_int as uint16_t,
    155 as libc::c_int as uint16_t,
    158 as libc::c_int as uint16_t,
    161 as libc::c_int as uint16_t,
    164 as libc::c_int as uint16_t,
    167 as libc::c_int as uint16_t,
    170 as libc::c_int as uint16_t,
    173 as libc::c_int as uint16_t,
    177 as libc::c_int as uint16_t,
    181 as libc::c_int as uint16_t,
    185 as libc::c_int as uint16_t,
    189 as libc::c_int as uint16_t,
    193 as libc::c_int as uint16_t,
    197 as libc::c_int as uint16_t,
    201 as libc::c_int as uint16_t,
    205 as libc::c_int as uint16_t,
    209 as libc::c_int as uint16_t,
    213 as libc::c_int as uint16_t,
    217 as libc::c_int as uint16_t,
    221 as libc::c_int as uint16_t,
    225 as libc::c_int as uint16_t,
    229 as libc::c_int as uint16_t,
    234 as libc::c_int as uint16_t,
    239 as libc::c_int as uint16_t,
    245 as libc::c_int as uint16_t,
    249 as libc::c_int as uint16_t,
    254 as libc::c_int as uint16_t,
    259 as libc::c_int as uint16_t,
    264 as libc::c_int as uint16_t,
    269 as libc::c_int as uint16_t,
    274 as libc::c_int as uint16_t,
    279 as libc::c_int as uint16_t,
    284 as libc::c_int as uint16_t,
];
static mut kAcTable2: [uint16_t; 128] = [
    8 as libc::c_int as uint16_t,
    8 as libc::c_int as uint16_t,
    9 as libc::c_int as uint16_t,
    10 as libc::c_int as uint16_t,
    12 as libc::c_int as uint16_t,
    13 as libc::c_int as uint16_t,
    15 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    18 as libc::c_int as uint16_t,
    20 as libc::c_int as uint16_t,
    21 as libc::c_int as uint16_t,
    23 as libc::c_int as uint16_t,
    24 as libc::c_int as uint16_t,
    26 as libc::c_int as uint16_t,
    27 as libc::c_int as uint16_t,
    29 as libc::c_int as uint16_t,
    31 as libc::c_int as uint16_t,
    32 as libc::c_int as uint16_t,
    34 as libc::c_int as uint16_t,
    35 as libc::c_int as uint16_t,
    37 as libc::c_int as uint16_t,
    38 as libc::c_int as uint16_t,
    40 as libc::c_int as uint16_t,
    41 as libc::c_int as uint16_t,
    43 as libc::c_int as uint16_t,
    44 as libc::c_int as uint16_t,
    46 as libc::c_int as uint16_t,
    48 as libc::c_int as uint16_t,
    49 as libc::c_int as uint16_t,
    51 as libc::c_int as uint16_t,
    52 as libc::c_int as uint16_t,
    54 as libc::c_int as uint16_t,
    55 as libc::c_int as uint16_t,
    57 as libc::c_int as uint16_t,
    58 as libc::c_int as uint16_t,
    60 as libc::c_int as uint16_t,
    62 as libc::c_int as uint16_t,
    63 as libc::c_int as uint16_t,
    65 as libc::c_int as uint16_t,
    66 as libc::c_int as uint16_t,
    68 as libc::c_int as uint16_t,
    69 as libc::c_int as uint16_t,
    71 as libc::c_int as uint16_t,
    72 as libc::c_int as uint16_t,
    74 as libc::c_int as uint16_t,
    75 as libc::c_int as uint16_t,
    77 as libc::c_int as uint16_t,
    79 as libc::c_int as uint16_t,
    80 as libc::c_int as uint16_t,
    82 as libc::c_int as uint16_t,
    83 as libc::c_int as uint16_t,
    85 as libc::c_int as uint16_t,
    86 as libc::c_int as uint16_t,
    88 as libc::c_int as uint16_t,
    89 as libc::c_int as uint16_t,
    93 as libc::c_int as uint16_t,
    96 as libc::c_int as uint16_t,
    99 as libc::c_int as uint16_t,
    102 as libc::c_int as uint16_t,
    105 as libc::c_int as uint16_t,
    108 as libc::c_int as uint16_t,
    111 as libc::c_int as uint16_t,
    114 as libc::c_int as uint16_t,
    117 as libc::c_int as uint16_t,
    120 as libc::c_int as uint16_t,
    124 as libc::c_int as uint16_t,
    127 as libc::c_int as uint16_t,
    130 as libc::c_int as uint16_t,
    133 as libc::c_int as uint16_t,
    136 as libc::c_int as uint16_t,
    139 as libc::c_int as uint16_t,
    142 as libc::c_int as uint16_t,
    145 as libc::c_int as uint16_t,
    148 as libc::c_int as uint16_t,
    151 as libc::c_int as uint16_t,
    155 as libc::c_int as uint16_t,
    158 as libc::c_int as uint16_t,
    161 as libc::c_int as uint16_t,
    164 as libc::c_int as uint16_t,
    167 as libc::c_int as uint16_t,
    170 as libc::c_int as uint16_t,
    173 as libc::c_int as uint16_t,
    176 as libc::c_int as uint16_t,
    179 as libc::c_int as uint16_t,
    184 as libc::c_int as uint16_t,
    189 as libc::c_int as uint16_t,
    193 as libc::c_int as uint16_t,
    198 as libc::c_int as uint16_t,
    203 as libc::c_int as uint16_t,
    207 as libc::c_int as uint16_t,
    212 as libc::c_int as uint16_t,
    217 as libc::c_int as uint16_t,
    221 as libc::c_int as uint16_t,
    226 as libc::c_int as uint16_t,
    230 as libc::c_int as uint16_t,
    235 as libc::c_int as uint16_t,
    240 as libc::c_int as uint16_t,
    244 as libc::c_int as uint16_t,
    249 as libc::c_int as uint16_t,
    254 as libc::c_int as uint16_t,
    258 as libc::c_int as uint16_t,
    263 as libc::c_int as uint16_t,
    268 as libc::c_int as uint16_t,
    274 as libc::c_int as uint16_t,
    280 as libc::c_int as uint16_t,
    286 as libc::c_int as uint16_t,
    292 as libc::c_int as uint16_t,
    299 as libc::c_int as uint16_t,
    305 as libc::c_int as uint16_t,
    311 as libc::c_int as uint16_t,
    317 as libc::c_int as uint16_t,
    323 as libc::c_int as uint16_t,
    330 as libc::c_int as uint16_t,
    336 as libc::c_int as uint16_t,
    342 as libc::c_int as uint16_t,
    348 as libc::c_int as uint16_t,
    354 as libc::c_int as uint16_t,
    362 as libc::c_int as uint16_t,
    370 as libc::c_int as uint16_t,
    379 as libc::c_int as uint16_t,
    385 as libc::c_int as uint16_t,
    393 as libc::c_int as uint16_t,
    401 as libc::c_int as uint16_t,
    409 as libc::c_int as uint16_t,
    416 as libc::c_int as uint16_t,
    424 as libc::c_int as uint16_t,
    432 as libc::c_int as uint16_t,
    440 as libc::c_int as uint16_t,
];
static mut kBiasMatrices: [[uint8_t; 2]; 3] = [
    [96 as libc::c_int as uint8_t, 110 as libc::c_int as uint8_t],
    [96 as libc::c_int as uint8_t, 108 as libc::c_int as uint8_t],
    [110 as libc::c_int as uint8_t, 115 as libc::c_int as uint8_t],
];
static mut kFreqSharpening: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
];
unsafe extern "C" fn ExpandMatrix(
    m: *mut VP8Matrix,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let is_ac_coeff: libc::c_int = (i > 0 as libc::c_int) as libc::c_int;
        let bias: libc::c_int = kBiasMatrices[type_0 as usize][is_ac_coeff as usize]
            as libc::c_int;
        (*m)
            .iq_[i
            as usize] = (((1 as libc::c_int) << 17 as libc::c_int)
            / (*m).q_[i as usize] as libc::c_int) as uint16_t;
        (*m)
            .bias_[i
            as usize] = (bias << 17 as libc::c_int - 8 as libc::c_int) as uint32_t;
        (*m)
            .zthresh_[i
            as usize] = ((((1 as libc::c_int) << 17 as libc::c_int) - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_sub((*m).bias_[i as usize])
            .wrapping_div((*m).iq_[i as usize] as libc::c_uint);
        i += 1;
        i;
    }
    i = 2 as libc::c_int;
    while i < 16 as libc::c_int {
        (*m).q_[i as usize] = (*m).q_[1 as libc::c_int as usize];
        (*m).iq_[i as usize] = (*m).iq_[1 as libc::c_int as usize];
        (*m).bias_[i as usize] = (*m).bias_[1 as libc::c_int as usize];
        (*m).zthresh_[i as usize] = (*m).zthresh_[1 as libc::c_int as usize];
        i += 1;
        i;
    }
    sum = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if type_0 == 0 as libc::c_int {
            (*m)
                .sharpen_[i
                as usize] = (kFreqSharpening[i as usize] as libc::c_int
                * (*m).q_[i as usize] as libc::c_int >> 11 as libc::c_int) as uint16_t;
        } else {
            (*m).sharpen_[i as usize] = 0 as libc::c_int as uint16_t;
        }
        sum += (*m).q_[i as usize] as libc::c_int;
        i += 1;
        i;
    }
    return sum + 8 as libc::c_int >> 4 as libc::c_int;
}
unsafe extern "C" fn CheckLambdaValue(v: *mut libc::c_int) {
    if *v < 1 as libc::c_int {
        *v = 1 as libc::c_int;
    }
}
unsafe extern "C" fn SetupMatrices(mut enc: *mut VP8Encoder) {
    let mut i: libc::c_int = 0;
    let tlambda_scale: libc::c_int = if (*enc).method_ >= 4 as libc::c_int {
        (*(*enc).config_).sns_strength
    } else {
        0 as libc::c_int
    };
    let num_segments: libc::c_int = (*enc).segment_hdr_.num_segments_;
    i = 0 as libc::c_int;
    while i < num_segments {
        let m: *mut VP8SegmentInfo = &mut *((*enc).dqm_).as_mut_ptr().offset(i as isize)
            as *mut VP8SegmentInfo;
        let q: libc::c_int = (*m).quant_;
        let mut q_i4: libc::c_int = 0;
        let mut q_i16: libc::c_int = 0;
        let mut q_uv: libc::c_int = 0;
        (*m)
            .y1_
            .q_[0 as libc::c_int
            as usize] = kDcTable[clip(
            q + (*enc).dq_y1_dc_,
            0 as libc::c_int,
            127 as libc::c_int,
        ) as usize] as uint16_t;
        (*m)
            .y1_
            .q_[1 as libc::c_int
            as usize] = kAcTable[clip(q, 0 as libc::c_int, 127 as libc::c_int) as usize];
        (*m)
            .y2_
            .q_[0 as libc::c_int
            as usize] = (kDcTable[clip(
            q + (*enc).dq_y2_dc_,
            0 as libc::c_int,
            127 as libc::c_int,
        ) as usize] as libc::c_int * 2 as libc::c_int) as uint16_t;
        (*m)
            .y2_
            .q_[1 as libc::c_int
            as usize] = kAcTable2[clip(
            q + (*enc).dq_y2_ac_,
            0 as libc::c_int,
            127 as libc::c_int,
        ) as usize];
        (*m)
            .uv_
            .q_[0 as libc::c_int
            as usize] = kDcTable[clip(
            q + (*enc).dq_uv_dc_,
            0 as libc::c_int,
            117 as libc::c_int,
        ) as usize] as uint16_t;
        (*m)
            .uv_
            .q_[1 as libc::c_int
            as usize] = kAcTable[clip(
            q + (*enc).dq_uv_ac_,
            0 as libc::c_int,
            127 as libc::c_int,
        ) as usize];
        q_i4 = ExpandMatrix(&mut (*m).y1_, 0 as libc::c_int);
        q_i16 = ExpandMatrix(&mut (*m).y2_, 1 as libc::c_int);
        q_uv = ExpandMatrix(&mut (*m).uv_, 2 as libc::c_int);
        (*m).lambda_i4_ = 3 as libc::c_int * q_i4 * q_i4 >> 7 as libc::c_int;
        (*m).lambda_i16_ = 3 as libc::c_int * q_i16 * q_i16;
        (*m).lambda_uv_ = 3 as libc::c_int * q_uv * q_uv >> 6 as libc::c_int;
        (*m).lambda_mode_ = 1 as libc::c_int * q_i4 * q_i4 >> 7 as libc::c_int;
        (*m).lambda_trellis_i4_ = 7 as libc::c_int * q_i4 * q_i4 >> 3 as libc::c_int;
        (*m).lambda_trellis_i16_ = q_i16 * q_i16 >> 2 as libc::c_int;
        (*m).lambda_trellis_uv_ = q_uv * q_uv << 1 as libc::c_int;
        (*m).tlambda_ = tlambda_scale * q_i4 >> 5 as libc::c_int;
        CheckLambdaValue(&mut (*m).lambda_i4_);
        CheckLambdaValue(&mut (*m).lambda_i16_);
        CheckLambdaValue(&mut (*m).lambda_uv_);
        CheckLambdaValue(&mut (*m).lambda_mode_);
        CheckLambdaValue(&mut (*m).lambda_trellis_i4_);
        CheckLambdaValue(&mut (*m).lambda_trellis_i16_);
        CheckLambdaValue(&mut (*m).lambda_trellis_uv_);
        CheckLambdaValue(&mut (*m).tlambda_);
        (*m)
            .min_disto_ = 20 as libc::c_int
            * (*m).y1_.q_[0 as libc::c_int as usize] as libc::c_int;
        (*m).max_edge_ = 0 as libc::c_int;
        (*m).i4_penalty_ = (1000 as libc::c_int * q_i4 * q_i4) as score_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn SetupFilterStrength(enc: *mut VP8Encoder) {
    let mut i: libc::c_int = 0;
    let level0: libc::c_int = 5 as libc::c_int * (*(*enc).config_).filter_strength;
    i = 0 as libc::c_int;
    while i < NUM_MB_SEGMENTS as libc::c_int {
        let m: *mut VP8SegmentInfo = &mut *((*enc).dqm_).as_mut_ptr().offset(i as isize)
            as *mut VP8SegmentInfo;
        let qstep: libc::c_int = kAcTable[clip(
            (*m).quant_,
            0 as libc::c_int,
            127 as libc::c_int,
        ) as usize] as libc::c_int >> 2 as libc::c_int;
        let base_strength: libc::c_int = VP8FilterStrengthFromDelta(
            (*enc).filter_hdr_.sharpness_,
            qstep,
        );
        let f: libc::c_int = base_strength * level0 / (256 as libc::c_int + (*m).beta_);
        (*m)
            .fstrength_ = if f < 2 as libc::c_int {
            0 as libc::c_int
        } else if f > 63 as libc::c_int {
            63 as libc::c_int
        } else {
            f
        };
        i += 1;
        i;
    }
    (*enc).filter_hdr_.level_ = (*enc).dqm_[0 as libc::c_int as usize].fstrength_;
    (*enc)
        .filter_hdr_
        .simple_ = ((*(*enc).config_).filter_type == 0 as libc::c_int) as libc::c_int;
    (*enc).filter_hdr_.sharpness_ = (*(*enc).config_).filter_sharpness;
}
unsafe extern "C" fn QualityToCompression(mut c: libc::c_double) -> libc::c_double {
    let linear_c: libc::c_double = if c < 0.75f64 {
        c * (2.0f64 / 3.0f64)
    } else {
        2.0f64 * c - 1.0f64
    };
    let v: libc::c_double = pow(linear_c, 1 as libc::c_int as libc::c_double / 3.0f64);
    return v;
}
unsafe extern "C" fn QualityToJPEGCompression(
    mut c: libc::c_double,
    mut alpha: libc::c_double,
) -> libc::c_double {
    let amin: libc::c_double = 0.30f64;
    let amax: libc::c_double = 0.85f64;
    let exp_min: libc::c_double = 0.4f64;
    let exp_max: libc::c_double = 0.9f64;
    let slope: libc::c_double = (exp_min - exp_max) / (amax - amin);
    let expn: libc::c_double = if alpha > amax {
        exp_min
    } else if alpha < amin {
        exp_max
    } else {
        exp_max + slope * (alpha - amin)
    };
    let v: libc::c_double = pow(c, expn);
    return v;
}
unsafe extern "C" fn SegmentsAreEquivalent(
    S1: *const VP8SegmentInfo,
    S2: *const VP8SegmentInfo,
) -> libc::c_int {
    return ((*S1).quant_ == (*S2).quant_ && (*S1).fstrength_ == (*S2).fstrength_)
        as libc::c_int;
}
unsafe extern "C" fn SimplifySegments(enc: *mut VP8Encoder) {
    let mut map: [libc::c_int; 4] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    let num_segments: libc::c_int = if (*enc).segment_hdr_.num_segments_
        < NUM_MB_SEGMENTS as libc::c_int
    {
        (*enc).segment_hdr_.num_segments_
    } else {
        NUM_MB_SEGMENTS as libc::c_int
    };
    let mut num_final_segments: libc::c_int = 1 as libc::c_int;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    s1 = 1 as libc::c_int;
    while s1 < num_segments {
        let S1: *const VP8SegmentInfo = &mut *((*enc).dqm_)
            .as_mut_ptr()
            .offset(s1 as isize) as *mut VP8SegmentInfo;
        let mut found: libc::c_int = 0 as libc::c_int;
        s2 = 0 as libc::c_int;
        while s2 < num_final_segments {
            let S2: *const VP8SegmentInfo = &mut *((*enc).dqm_)
                .as_mut_ptr()
                .offset(s2 as isize) as *mut VP8SegmentInfo;
            if SegmentsAreEquivalent(S1, S2) != 0 {
                found = 1 as libc::c_int;
                break;
            } else {
                s2 += 1;
                s2;
            }
        }
        map[s1 as usize] = s2;
        if found == 0 {
            if num_final_segments != s1 {
                (*enc).dqm_[num_final_segments as usize] = (*enc).dqm_[s1 as usize];
            }
            num_final_segments += 1;
            num_final_segments;
        }
        s1 += 1;
        s1;
    }
    if num_final_segments < num_segments {
        let mut i: libc::c_int = (*enc).mb_w_ * (*enc).mb_h_;
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 > 0 as libc::c_int) {
                break;
            }
            let ref mut fresh2 = *((*enc).mb_info_).offset(i as isize);
            (*fresh2)
                .set_segment_(
                    map[(*((*enc).mb_info_).offset(i as isize)).segment_() as usize]
                        as libc::c_uint,
                );
        }
        (*enc).segment_hdr_.num_segments_ = num_final_segments;
        i = num_final_segments;
        while i < num_segments {
            (*enc)
                .dqm_[i
                as usize] = (*enc)
                .dqm_[(num_final_segments - 1 as libc::c_int) as usize];
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetSegmentParams(
    enc: *mut VP8Encoder,
    mut quality: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut dq_uv_ac: libc::c_int = 0;
    let mut dq_uv_dc: libc::c_int = 0;
    let num_segments: libc::c_int = (*enc).segment_hdr_.num_segments_;
    let amp: libc::c_double = 0.9f64 * (*(*enc).config_).sns_strength as libc::c_double
        / 100.0f64 / 128.0f64;
    let Q: libc::c_double = quality as libc::c_double / 100.0f64;
    let c_base: libc::c_double = if (*(*enc).config_).emulate_jpeg_size != 0 {
        QualityToJPEGCompression(Q, (*enc).alpha_ as libc::c_double / 255.0f64)
    } else {
        QualityToCompression(Q)
    };
    i = 0 as libc::c_int;
    while i < num_segments {
        let expn: libc::c_double = 1.0f64
            - amp * (*enc).dqm_[i as usize].alpha_ as libc::c_double;
        let c: libc::c_double = pow(c_base, expn);
        let q: libc::c_int = (127.0f64 * (1.0f64 - c)) as libc::c_int;
        (*enc).dqm_[i as usize].quant_ = clip(q, 0 as libc::c_int, 127 as libc::c_int);
        i += 1;
        i;
    }
    (*enc).base_quant_ = (*enc).dqm_[0 as libc::c_int as usize].quant_;
    i = num_segments;
    while i < NUM_MB_SEGMENTS as libc::c_int {
        (*enc).dqm_[i as usize].quant_ = (*enc).base_quant_;
        i += 1;
        i;
    }
    dq_uv_ac = ((*enc).uv_alpha_ - 64 as libc::c_int)
        * (6 as libc::c_int - -(4 as libc::c_int))
        / (100 as libc::c_int - 30 as libc::c_int);
    dq_uv_ac = dq_uv_ac * (*(*enc).config_).sns_strength / 100 as libc::c_int;
    dq_uv_ac = clip(dq_uv_ac, -(4 as libc::c_int), 6 as libc::c_int);
    dq_uv_dc = -(4 as libc::c_int) * (*(*enc).config_).sns_strength / 100 as libc::c_int;
    dq_uv_dc = clip(dq_uv_dc, -(15 as libc::c_int), 15 as libc::c_int);
    (*enc).dq_y1_dc_ = 0 as libc::c_int;
    (*enc).dq_y2_dc_ = 0 as libc::c_int;
    (*enc).dq_y2_ac_ = 0 as libc::c_int;
    (*enc).dq_uv_dc_ = dq_uv_dc;
    (*enc).dq_uv_ac_ = dq_uv_ac;
    SetupFilterStrength(enc);
    if num_segments > 1 as libc::c_int {
        SimplifySegments(enc);
    }
    SetupMatrices(enc);
}
#[no_mangle]
pub static mut VP8I16ModeOffsets: [uint16_t; 4] = [
    (0 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 16 as libc::c_int)
        as uint16_t,
    (1 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (1 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 16 as libc::c_int)
        as uint16_t,
];
#[no_mangle]
pub static mut VP8UVModeOffsets: [uint16_t; 4] = [
    (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
        + 1 as libc::c_int * 16 as libc::c_int) as uint16_t,
    (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
        + 8 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (2 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
        + 8 as libc::c_int * 32 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int)
        as uint16_t,
];
#[no_mangle]
pub static mut VP8I4ModeOffsets: [uint16_t; 10] = [
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int)
        as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int
        + 4 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int
        + 8 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int
        + 12 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int
        + 16 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int
        + 20 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int
        + 24 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int
        + 28 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
        + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
        + 4 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as uint16_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8MakeLuma16Preds(it: *const VP8EncIterator) {
    let left: *const uint8_t = if (*it).x_ != 0 {
        (*it).y_left_
    } else {
        0 as *mut uint8_t
    };
    let top: *const uint8_t = if (*it).y_ != 0 {
        (*it).y_top_
    } else {
        0 as *mut uint8_t
    };
    VP8EncPredLuma16.expect("non-null function pointer")((*it).yuv_p_, left, top);
}
#[no_mangle]
pub unsafe extern "C" fn VP8MakeChroma8Preds(it: *const VP8EncIterator) {
    let left: *const uint8_t = if (*it).x_ != 0 {
        (*it).u_left_
    } else {
        0 as *mut uint8_t
    };
    let top: *const uint8_t = if (*it).y_ != 0 {
        (*it).uv_top_
    } else {
        0 as *mut uint8_t
    };
    VP8EncPredChroma8.expect("non-null function pointer")((*it).yuv_p_, left, top);
}
#[no_mangle]
pub unsafe extern "C" fn VP8MakeIntra4Preds(it: *const VP8EncIterator) {
    VP8EncPredLuma4.expect("non-null function pointer")((*it).yuv_p_, (*it).i4_top_);
}
#[no_mangle]
pub static mut VP8Scan: [uint16_t; 16] = [
    (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (8 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (12 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (8 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (12 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (4 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (8 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (12 as libc::c_int + 8 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (4 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (8 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (12 as libc::c_int + 12 as libc::c_int * 32 as libc::c_int) as uint16_t,
];
static mut VP8ScanUV: [uint16_t; 8] = [
    (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (8 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (12 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (8 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (12 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
];
static mut kWeightY: [uint16_t; 16] = [
    38 as libc::c_int as uint16_t,
    32 as libc::c_int as uint16_t,
    20 as libc::c_int as uint16_t,
    9 as libc::c_int as uint16_t,
    32 as libc::c_int as uint16_t,
    28 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    7 as libc::c_int as uint16_t,
    20 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    10 as libc::c_int as uint16_t,
    4 as libc::c_int as uint16_t,
    9 as libc::c_int as uint16_t,
    7 as libc::c_int as uint16_t,
    4 as libc::c_int as uint16_t,
    2 as libc::c_int as uint16_t,
];
static mut kWeightTrellis: [uint16_t; 16] = [
    30 as libc::c_int as uint16_t,
    27 as libc::c_int as uint16_t,
    19 as libc::c_int as uint16_t,
    11 as libc::c_int as uint16_t,
    27 as libc::c_int as uint16_t,
    24 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    10 as libc::c_int as uint16_t,
    19 as libc::c_int as uint16_t,
    17 as libc::c_int as uint16_t,
    12 as libc::c_int as uint16_t,
    8 as libc::c_int as uint16_t,
    11 as libc::c_int as uint16_t,
    10 as libc::c_int as uint16_t,
    8 as libc::c_int as uint16_t,
    6 as libc::c_int as uint16_t,
];
unsafe extern "C" fn InitScore(rd: *mut VP8ModeScore) {
    (*rd).D = 0 as libc::c_int as score_t;
    (*rd).SD = 0 as libc::c_int as score_t;
    (*rd).R = 0 as libc::c_int as score_t;
    (*rd).H = 0 as libc::c_int as score_t;
    (*rd).nz = 0 as libc::c_int as uint32_t;
    (*rd).score = 0x7fffffffffffff as libc::c_longlong as score_t;
}
unsafe extern "C" fn CopyScore(dst: *mut VP8ModeScore, src: *const VP8ModeScore) {
    (*dst).D = (*src).D;
    (*dst).SD = (*src).SD;
    (*dst).R = (*src).R;
    (*dst).H = (*src).H;
    (*dst).nz = (*src).nz;
    (*dst).score = (*src).score;
}
unsafe extern "C" fn AddScore(dst: *mut VP8ModeScore, src: *const VP8ModeScore) {
    (*dst).D += (*src).D;
    (*dst).SD += (*src).SD;
    (*dst).R += (*src).R;
    (*dst).H += (*src).H;
    (*dst).nz |= (*src).nz;
    (*dst).score += (*src).score;
}
#[inline]
unsafe extern "C" fn SetRDScore(mut lambda: libc::c_int, rd: *mut VP8ModeScore) {
    (*rd)
        .score = ((*rd).R + (*rd).H) * lambda as libc::c_long
        + 256 as libc::c_int as libc::c_long * ((*rd).D + (*rd).SD);
}
#[inline]
unsafe extern "C" fn RDScoreTrellis(
    mut lambda: libc::c_int,
    mut rate: score_t,
    mut distortion: score_t,
) -> score_t {
    return rate * lambda as libc::c_long
        + 256 as libc::c_int as libc::c_long * distortion;
}
unsafe extern "C" fn TrellisQuantizeBlock(
    enc: *const VP8Encoder,
    mut in_0: *mut int16_t,
    mut out: *mut int16_t,
    mut ctx0: libc::c_int,
    mut coeff_type: libc::c_int,
    mtx: *const VP8Matrix,
    mut lambda: libc::c_int,
) -> libc::c_int {
    let probas: *const ProbaArray = ((*enc).proba_.coeffs_[coeff_type as usize])
        .as_ptr();
    let costs: CostArrayPtr = ((*enc).proba_.remapped_costs_[coeff_type as usize])
        .as_ptr() as CostArrayPtr;
    let first: libc::c_int = if coeff_type == TYPE_I16_AC as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut nodes: [[Node; 2]; 16] = [[Node { prev: 0, sign: 0, level: 0 }; 2]; 16];
    let mut score_states: [[ScoreState; 2]; 2] = [[ScoreState {
        score: 0,
        costs: 0 as *const uint16_t,
    }; 2]; 2];
    let mut ss_cur: *mut ScoreState = &mut *(*score_states
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset((0 as libc::c_int + 0 as libc::c_int) as isize) as *mut ScoreState;
    let mut ss_prev: *mut ScoreState = &mut *(*score_states
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize))
        .as_mut_ptr()
        .offset((0 as libc::c_int + 0 as libc::c_int) as isize) as *mut ScoreState;
    let mut best_path: [libc::c_int; 3] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ];
    let mut best_score: score_t = 0;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut cost: score_t = 0;
    let thresh: libc::c_int = (*mtx).q_[1 as libc::c_int as usize] as libc::c_int
        * (*mtx).q_[1 as libc::c_int as usize] as libc::c_int / 4 as libc::c_int;
    let last_proba: libc::c_int = (*probas
        .offset(
            VP8EncBands[first as usize] as isize,
        ))[ctx0 as usize][0 as libc::c_int as usize] as libc::c_int;
    last = first - 1 as libc::c_int;
    n = 15 as libc::c_int;
    while n >= first {
        let j: libc::c_int = kZigzag[n as usize] as libc::c_int;
        let err: libc::c_int = *in_0.offset(j as isize) as libc::c_int
            * *in_0.offset(j as isize) as libc::c_int;
        if err > thresh {
            last = n;
            break;
        } else {
            n -= 1;
            n;
        }
    }
    if last < 15 as libc::c_int {
        last += 1;
        last;
    }
    cost = VP8BitCost(0 as libc::c_int, last_proba as uint8_t) as score_t;
    best_score = RDScoreTrellis(lambda, cost, 0 as libc::c_int as score_t);
    m = -(0 as libc::c_int);
    while m <= 1 as libc::c_int {
        let rate: score_t = (if ctx0 == 0 as libc::c_int {
            VP8BitCost(1 as libc::c_int, last_proba as uint8_t)
        } else {
            0 as libc::c_int
        }) as score_t;
        (*ss_cur.offset(m as isize))
            .score = RDScoreTrellis(lambda, rate, 0 as libc::c_int as score_t);
        let ref mut fresh3 = (*ss_cur.offset(m as isize)).costs;
        *fresh3 = (*costs.offset(first as isize))[ctx0 as usize];
        m += 1;
        m;
    }
    n = first;
    while n <= last {
        let j_0: libc::c_int = kZigzag[n as usize] as libc::c_int;
        let Q: uint32_t = (*mtx).q_[j_0 as usize] as uint32_t;
        let iQ: uint32_t = (*mtx).iq_[j_0 as usize] as uint32_t;
        let B: uint32_t = ((0 as libc::c_int) << 17 as libc::c_int - 8 as libc::c_int)
            as uint32_t;
        let sign: libc::c_int = ((*in_0.offset(j_0 as isize) as libc::c_int)
            < 0 as libc::c_int) as libc::c_int;
        let coeff0: uint32_t = ((if sign != 0 {
            -(*in_0.offset(j_0 as isize) as libc::c_int)
        } else {
            *in_0.offset(j_0 as isize) as libc::c_int
        }) + (*mtx).sharpen_[j_0 as usize] as libc::c_int) as uint32_t;
        let mut level0: libc::c_int = QUANTDIV(coeff0, iQ, B);
        let mut thresh_level: libc::c_int = QUANTDIV(
            coeff0,
            iQ,
            ((0x80 as libc::c_int) << 17 as libc::c_int - 8 as libc::c_int) as uint32_t,
        );
        if thresh_level > MAX_LEVEL as libc::c_int {
            thresh_level = MAX_LEVEL as libc::c_int;
        }
        if level0 > MAX_LEVEL as libc::c_int {
            level0 = MAX_LEVEL as libc::c_int;
        }
        let tmp: *mut ScoreState = ss_cur;
        ss_cur = ss_prev;
        ss_prev = tmp;
        m = -(0 as libc::c_int);
        while m <= 1 as libc::c_int {
            let cur: *mut Node = &mut *(*nodes.as_mut_ptr().offset(n as isize))
                .as_mut_ptr()
                .offset((m + 0 as libc::c_int) as isize) as *mut Node;
            let level: libc::c_int = level0 + m;
            let ctx: libc::c_int = if level > 2 as libc::c_int {
                2 as libc::c_int
            } else {
                level
            };
            let band: libc::c_int = VP8EncBands[(n + 1 as libc::c_int) as usize]
                as libc::c_int;
            let mut base_score: score_t = 0;
            let mut best_cur_score: score_t = 0;
            let mut best_prev: libc::c_int = 0;
            let mut cost_0: score_t = 0;
            let mut score: score_t = 0;
            let ref mut fresh4 = (*ss_cur.offset(m as isize)).costs;
            *fresh4 = (*costs.offset((n + 1 as libc::c_int) as isize))[ctx as usize];
            if level < 0 as libc::c_int || level > thresh_level {
                (*ss_cur.offset(m as isize))
                    .score = 0x7fffffffffffff as libc::c_longlong as score_t;
            } else {
                let new_error: libc::c_int = coeff0
                    .wrapping_sub((level as libc::c_uint).wrapping_mul(Q))
                    as libc::c_int;
                let delta_error: libc::c_int = (kWeightTrellis[j_0 as usize]
                    as libc::c_uint)
                    .wrapping_mul(
                        ((new_error * new_error) as libc::c_uint)
                            .wrapping_sub(coeff0.wrapping_mul(coeff0)),
                    ) as libc::c_int;
                base_score = RDScoreTrellis(
                    lambda,
                    0 as libc::c_int as score_t,
                    delta_error as score_t,
                );
                cost_0 = VP8LevelCost(
                    (*ss_prev.offset(-(0 as libc::c_int) as isize)).costs,
                    level,
                ) as score_t;
                best_cur_score = (*ss_prev.offset(-(0 as libc::c_int) as isize)).score
                    + RDScoreTrellis(lambda, cost_0, 0 as libc::c_int as score_t);
                best_prev = -(0 as libc::c_int);
                p = -(0 as libc::c_int) + 1 as libc::c_int;
                while p <= 1 as libc::c_int {
                    cost_0 = VP8LevelCost((*ss_prev.offset(p as isize)).costs, level)
                        as score_t;
                    score = (*ss_prev.offset(p as isize)).score
                        + RDScoreTrellis(lambda, cost_0, 0 as libc::c_int as score_t);
                    if score < best_cur_score {
                        best_cur_score = score;
                        best_prev = p;
                    }
                    p += 1;
                    p;
                }
                best_cur_score += base_score;
                (*cur).sign = sign as int8_t;
                (*cur).level = level as int16_t;
                (*cur).prev = best_prev as int8_t;
                (*ss_cur.offset(m as isize)).score = best_cur_score;
                if level != 0 as libc::c_int && best_cur_score < best_score {
                    let last_pos_cost: score_t = (if n < 15 as libc::c_int {
                        VP8BitCost(
                            0 as libc::c_int,
                            (*probas
                                .offset(
                                    band as isize,
                                ))[ctx as usize][0 as libc::c_int as usize],
                        )
                    } else {
                        0 as libc::c_int
                    }) as score_t;
                    let last_pos_score: score_t = RDScoreTrellis(
                        lambda,
                        last_pos_cost,
                        0 as libc::c_int as score_t,
                    );
                    score = best_cur_score + last_pos_score;
                    if score < best_score {
                        best_score = score;
                        best_path[0 as libc::c_int as usize] = n;
                        best_path[1 as libc::c_int as usize] = m;
                        best_path[2 as libc::c_int as usize] = best_prev;
                    }
                }
            }
            m += 1;
            m;
        }
        n += 1;
        n;
    }
    if coeff_type == TYPE_I16_AC as libc::c_int {
        memset(
            in_0.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (15 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
        );
        memset(
            out.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (15 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
        );
    } else {
        memset(
            in_0 as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
        );
        memset(
            out as *mut libc::c_void,
            0 as libc::c_int,
            (16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
        );
    }
    if best_path[0 as libc::c_int as usize] == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut best_node: libc::c_int = best_path[1 as libc::c_int as usize];
    n = best_path[0 as libc::c_int as usize];
    nodes[n as usize][(best_node + 0 as libc::c_int) as usize]
        .prev = best_path[2 as libc::c_int as usize] as int8_t;
    while n >= first {
        let node: *const Node = &mut *(*nodes.as_mut_ptr().offset(n as isize))
            .as_mut_ptr()
            .offset((best_node + 0 as libc::c_int) as isize) as *mut Node;
        let j_1: libc::c_int = kZigzag[n as usize] as libc::c_int;
        *out
            .offset(
                n as isize,
            ) = (if (*node).sign as libc::c_int != 0 {
            -((*node).level as libc::c_int)
        } else {
            (*node).level as libc::c_int
        }) as int16_t;
        nz |= (*node).level as libc::c_int;
        *in_0
            .offset(
                j_1 as isize,
            ) = (*out.offset(n as isize) as libc::c_int
            * (*mtx).q_[j_1 as usize] as libc::c_int) as int16_t;
        best_node = (*node).prev as libc::c_int;
        n -= 1;
        n;
    }
    return (nz != 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ReconstructIntra16(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
    yuv_out: *mut uint8_t,
    mut mode: libc::c_int,
) -> libc::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let ref_0: *const uint8_t = ((*it).yuv_p_)
        .offset(VP8I16ModeOffsets[mode as usize] as libc::c_int as isize);
    let src: *const uint8_t = ((*it).yuv_in_).offset(0 as libc::c_int as isize);
    let dqm: *const VP8SegmentInfo = &*((*enc).dqm_)
        .as_ptr()
        .offset((*(*it).mb_).segment_() as isize) as *const VP8SegmentInfo;
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut tmp: [[int16_t; 16]; 16] = [[0; 16]; 16];
    let mut dc_tmp: [int16_t; 16] = [0; 16];
    n = 0 as libc::c_int;
    while n < 16 as libc::c_int {
        VP8FTransform2
            .expect(
                "non-null function pointer",
            )(
            src.offset(VP8Scan[n as usize] as libc::c_int as isize),
            ref_0.offset(VP8Scan[n as usize] as libc::c_int as isize),
            (tmp[n as usize]).as_mut_ptr(),
        );
        n += 2 as libc::c_int;
    }
    VP8FTransformWHT
        .expect(
            "non-null function pointer",
        )((tmp[0 as libc::c_int as usize]).as_mut_ptr(), dc_tmp.as_mut_ptr());
    nz
        |= VP8EncQuantizeBlockWHT
            .expect(
                "non-null function pointer",
            )(dc_tmp.as_mut_ptr(), ((*rd).y_dc_levels).as_mut_ptr(), &(*dqm).y2_)
            << 24 as libc::c_int;
    if 1 as libc::c_int != 0 && (*it).do_trellis_ != 0 {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        VP8IteratorNzToBytes(it);
        y = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while y < 4 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 4 as libc::c_int {
                let ctx: libc::c_int = (*it).top_nz_[x as usize]
                    + (*it).left_nz_[y as usize];
                let non_zero: libc::c_int = TrellisQuantizeBlock(
                    enc,
                    (tmp[n as usize]).as_mut_ptr(),
                    ((*rd).y_ac_levels[n as usize]).as_mut_ptr(),
                    ctx,
                    TYPE_I16_AC as libc::c_int,
                    &(*dqm).y1_,
                    (*dqm).lambda_trellis_i16_,
                );
                (*it).left_nz_[y as usize] = non_zero;
                (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
                (*rd)
                    .y_ac_levels[n
                    as usize][0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
                nz |= non_zero << n;
                x += 1;
                x;
                n += 1;
                n;
            }
            y += 1;
            y;
        }
    } else {
        n = 0 as libc::c_int;
        while n < 16 as libc::c_int {
            tmp[(n + 1 as libc::c_int)
                as usize][0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
            tmp[n
                as usize][0 as libc::c_int
                as usize] = tmp[(n + 1 as libc::c_int)
                as usize][0 as libc::c_int as usize];
            nz
                |= VP8EncQuantize2Blocks
                    .expect(
                        "non-null function pointer",
                    )(
                    (tmp[n as usize]).as_mut_ptr(),
                    ((*rd).y_ac_levels[n as usize]).as_mut_ptr(),
                    &(*dqm).y1_,
                ) << n;
            n += 2 as libc::c_int;
        }
    }
    VP8TransformWHT
        .expect(
            "non-null function pointer",
        )(dc_tmp.as_mut_ptr(), (tmp[0 as libc::c_int as usize]).as_mut_ptr());
    n = 0 as libc::c_int;
    while n < 16 as libc::c_int {
        VP8ITransform
            .expect(
                "non-null function pointer",
            )(
            ref_0.offset(VP8Scan[n as usize] as libc::c_int as isize),
            (tmp[n as usize]).as_mut_ptr(),
            yuv_out.offset(VP8Scan[n as usize] as libc::c_int as isize),
            1 as libc::c_int,
        );
        n += 2 as libc::c_int;
    }
    return nz;
}
unsafe extern "C" fn ReconstructIntra4(
    it: *mut VP8EncIterator,
    mut levels: *mut int16_t,
    src: *const uint8_t,
    yuv_out: *mut uint8_t,
    mut mode: libc::c_int,
) -> libc::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let ref_0: *const uint8_t = ((*it).yuv_p_)
        .offset(VP8I4ModeOffsets[mode as usize] as libc::c_int as isize);
    let dqm: *const VP8SegmentInfo = &*((*enc).dqm_)
        .as_ptr()
        .offset((*(*it).mb_).segment_() as isize) as *const VP8SegmentInfo;
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut tmp: [int16_t; 16] = [0; 16];
    VP8FTransform.expect("non-null function pointer")(src, ref_0, tmp.as_mut_ptr());
    if 1 as libc::c_int != 0 && (*it).do_trellis_ != 0 {
        let x: libc::c_int = (*it).i4_ & 3 as libc::c_int;
        let y: libc::c_int = (*it).i4_ >> 2 as libc::c_int;
        let ctx: libc::c_int = (*it).top_nz_[x as usize] + (*it).left_nz_[y as usize];
        nz = TrellisQuantizeBlock(
            enc,
            tmp.as_mut_ptr(),
            levels,
            ctx,
            TYPE_I4_AC as libc::c_int,
            &(*dqm).y1_,
            (*dqm).lambda_trellis_i4_,
        );
    } else {
        nz = VP8EncQuantizeBlock
            .expect("non-null function pointer")(tmp.as_mut_ptr(), levels, &(*dqm).y1_);
    }
    VP8ITransform
        .expect(
            "non-null function pointer",
        )(ref_0, tmp.as_mut_ptr(), yuv_out, 0 as libc::c_int);
    return nz;
}
unsafe extern "C" fn QuantizeSingle(
    v: *mut int16_t,
    mtx: *const VP8Matrix,
) -> libc::c_int {
    let mut V: libc::c_int = *v as libc::c_int;
    let sign: libc::c_int = (V < 0 as libc::c_int) as libc::c_int;
    if sign != 0 {
        V = -V;
    }
    if V > (*mtx).zthresh_[0 as libc::c_int as usize] as libc::c_int {
        let qV: libc::c_int = QUANTDIV(
            V as uint32_t,
            (*mtx).iq_[0 as libc::c_int as usize] as uint32_t,
            (*mtx).bias_[0 as libc::c_int as usize],
        ) * (*mtx).q_[0 as libc::c_int as usize] as libc::c_int;
        let err: libc::c_int = V - qV;
        *v = (if sign != 0 { -qV } else { qV }) as int16_t;
        return (if sign != 0 { -err } else { err }) >> 1 as libc::c_int;
    }
    *v = 0 as libc::c_int as int16_t;
    return (if sign != 0 { -V } else { V }) >> 1 as libc::c_int;
}
unsafe extern "C" fn CorrectDCValues(
    it: *const VP8EncIterator,
    mtx: *const VP8Matrix,
    mut tmp: *mut [int16_t; 16],
    rd: *mut VP8ModeScore,
) {
    let mut ch: libc::c_int = 0;
    ch = 0 as libc::c_int;
    while ch <= 1 as libc::c_int {
        let top: *const int8_t = ((*((*it).top_derr_)
            .offset((*it).x_ as isize))[ch as usize])
            .as_mut_ptr();
        let left: *const int8_t = ((*it).left_derr_[ch as usize]).as_ptr();
        let c: *mut [int16_t; 16] = &mut *tmp.offset((ch * 4 as libc::c_int) as isize)
            as *mut [int16_t; 16];
        let mut err0: libc::c_int = 0;
        let mut err1: libc::c_int = 0;
        let mut err2: libc::c_int = 0;
        let mut err3: libc::c_int = 0;
        let ref mut fresh5 = (*c
            .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
        *fresh5 = (*fresh5 as libc::c_int
            + (7 as libc::c_int * *top.offset(0 as libc::c_int as isize) as libc::c_int
                + 8 as libc::c_int
                    * *left.offset(0 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int - 1 as libc::c_int)) as int16_t;
        err0 = QuantizeSingle(
            &mut *(*c.offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            mtx,
        );
        let ref mut fresh6 = (*c
            .offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
        *fresh6 = (*fresh6 as libc::c_int
            + (7 as libc::c_int * *top.offset(1 as libc::c_int as isize) as libc::c_int
                + 8 as libc::c_int * err0 >> 4 as libc::c_int - 1 as libc::c_int))
            as int16_t;
        err1 = QuantizeSingle(
            &mut *(*c.offset(1 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            mtx,
        );
        let ref mut fresh7 = (*c
            .offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
        *fresh7 = (*fresh7 as libc::c_int
            + (7 as libc::c_int * err0
                + 8 as libc::c_int
                    * *left.offset(1 as libc::c_int as isize) as libc::c_int
                >> 4 as libc::c_int - 1 as libc::c_int)) as int16_t;
        err2 = QuantizeSingle(
            &mut *(*c.offset(2 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            mtx,
        );
        let ref mut fresh8 = (*c
            .offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
        *fresh8 = (*fresh8 as libc::c_int
            + (7 as libc::c_int * err1 + 8 as libc::c_int * err2
                >> 4 as libc::c_int - 1 as libc::c_int)) as int16_t;
        err3 = QuantizeSingle(
            &mut *(*c.offset(3 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            mtx,
        );
        (*rd).derr[ch as usize][0 as libc::c_int as usize] = err1 as int8_t;
        (*rd).derr[ch as usize][1 as libc::c_int as usize] = err2 as int8_t;
        (*rd).derr[ch as usize][2 as libc::c_int as usize] = err3 as int8_t;
        ch += 1;
        ch;
    }
}
unsafe extern "C" fn StoreDiffusionErrors(
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
) {
    let mut ch: libc::c_int = 0;
    ch = 0 as libc::c_int;
    while ch <= 1 as libc::c_int {
        let top: *mut int8_t = ((*((*it).top_derr_)
            .offset((*it).x_ as isize))[ch as usize])
            .as_mut_ptr();
        let left: *mut int8_t = ((*it).left_derr_[ch as usize]).as_mut_ptr();
        *left
            .offset(
                0 as libc::c_int as isize,
            ) = (*rd).derr[ch as usize][0 as libc::c_int as usize];
        *left
            .offset(
                1 as libc::c_int as isize,
            ) = (3 as libc::c_int
            * (*rd).derr[ch as usize][2 as libc::c_int as usize] as libc::c_int
            >> 2 as libc::c_int) as int8_t;
        *top
            .offset(
                0 as libc::c_int as isize,
            ) = (*rd).derr[ch as usize][1 as libc::c_int as usize];
        *top
            .offset(
                1 as libc::c_int as isize,
            ) = ((*rd).derr[ch as usize][2 as libc::c_int as usize] as libc::c_int
            - *left.offset(1 as libc::c_int as isize) as libc::c_int) as int8_t;
        ch += 1;
        ch;
    }
}
unsafe extern "C" fn ReconstructUV(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
    yuv_out: *mut uint8_t,
    mut mode: libc::c_int,
) -> libc::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let ref_0: *const uint8_t = ((*it).yuv_p_)
        .offset(VP8UVModeOffsets[mode as usize] as libc::c_int as isize);
    let src: *const uint8_t = ((*it).yuv_in_).offset(16 as libc::c_int as isize);
    let dqm: *const VP8SegmentInfo = &*((*enc).dqm_)
        .as_ptr()
        .offset((*(*it).mb_).segment_() as isize) as *const VP8SegmentInfo;
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut tmp: [[int16_t; 16]; 8] = [[0; 16]; 8];
    n = 0 as libc::c_int;
    while n < 8 as libc::c_int {
        VP8FTransform2
            .expect(
                "non-null function pointer",
            )(
            src.offset(VP8ScanUV[n as usize] as libc::c_int as isize),
            ref_0.offset(VP8ScanUV[n as usize] as libc::c_int as isize),
            (tmp[n as usize]).as_mut_ptr(),
        );
        n += 2 as libc::c_int;
    }
    if !((*it).top_derr_).is_null() {
        CorrectDCValues(it, &(*dqm).uv_, tmp.as_mut_ptr(), rd);
    }
    if 0 as libc::c_int != 0 && (*it).do_trellis_ != 0 {
        let mut ch: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        ch = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while ch <= 2 as libc::c_int {
            y = 0 as libc::c_int;
            while y < 2 as libc::c_int {
                x = 0 as libc::c_int;
                while x < 2 as libc::c_int {
                    let ctx: libc::c_int = (*it)
                        .top_nz_[(4 as libc::c_int + ch + x) as usize]
                        + (*it).left_nz_[(4 as libc::c_int + ch + y) as usize];
                    let non_zero: libc::c_int = TrellisQuantizeBlock(
                        enc,
                        (tmp[n as usize]).as_mut_ptr(),
                        ((*rd).uv_levels[n as usize]).as_mut_ptr(),
                        ctx,
                        TYPE_CHROMA_A as libc::c_int,
                        &(*dqm).uv_,
                        (*dqm).lambda_trellis_uv_,
                    );
                    (*it).left_nz_[(4 as libc::c_int + ch + y) as usize] = non_zero;
                    (*it)
                        .top_nz_[(4 as libc::c_int + ch + x)
                        as usize] = (*it).left_nz_[(4 as libc::c_int + ch + y) as usize];
                    nz |= non_zero << n;
                    x += 1;
                    x;
                    n += 1;
                    n;
                }
                y += 1;
                y;
            }
            ch += 2 as libc::c_int;
        }
    } else {
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            nz
                |= VP8EncQuantize2Blocks
                    .expect(
                        "non-null function pointer",
                    )(
                    (tmp[n as usize]).as_mut_ptr(),
                    ((*rd).uv_levels[n as usize]).as_mut_ptr(),
                    &(*dqm).uv_,
                ) << n;
            n += 2 as libc::c_int;
        }
    }
    n = 0 as libc::c_int;
    while n < 8 as libc::c_int {
        VP8ITransform
            .expect(
                "non-null function pointer",
            )(
            ref_0.offset(VP8ScanUV[n as usize] as libc::c_int as isize),
            (tmp[n as usize]).as_mut_ptr(),
            yuv_out.offset(VP8ScanUV[n as usize] as libc::c_int as isize),
            1 as libc::c_int,
        );
        n += 2 as libc::c_int;
    }
    return nz << 16 as libc::c_int;
}
unsafe extern "C" fn StoreMaxDelta(dqm: *mut VP8SegmentInfo, mut DCs: *const int16_t) {
    let v0: libc::c_int = abs(*DCs.offset(1 as libc::c_int as isize) as libc::c_int);
    let v1: libc::c_int = abs(*DCs.offset(2 as libc::c_int as isize) as libc::c_int);
    let v2: libc::c_int = abs(*DCs.offset(4 as libc::c_int as isize) as libc::c_int);
    let mut max_v: libc::c_int = if v1 > v0 { v1 } else { v0 };
    max_v = if v2 > max_v { v2 } else { max_v };
    if max_v > (*dqm).max_edge_ {
        (*dqm).max_edge_ = max_v;
    }
}
unsafe extern "C" fn SwapModeScore(
    mut a: *mut *mut VP8ModeScore,
    mut b: *mut *mut VP8ModeScore,
) {
    let tmp: *mut VP8ModeScore = *a;
    *a = *b;
    *b = tmp;
}
unsafe extern "C" fn SwapPtr(mut a: *mut *mut uint8_t, mut b: *mut *mut uint8_t) {
    let tmp: *mut uint8_t = *a;
    *a = *b;
    *b = tmp;
}
unsafe extern "C" fn SwapOut(it: *mut VP8EncIterator) {
    SwapPtr(&mut (*it).yuv_out_, &mut (*it).yuv_out2_);
}
unsafe extern "C" fn PickBestIntra16(
    it: *mut VP8EncIterator,
    mut rd: *mut VP8ModeScore,
) {
    let kNumBlocks: libc::c_int = 16 as libc::c_int;
    let dqm: *mut VP8SegmentInfo = &mut *((*(*it).enc_).dqm_)
        .as_mut_ptr()
        .offset((*(*it).mb_).segment_() as isize) as *mut VP8SegmentInfo;
    let lambda: libc::c_int = (*dqm).lambda_i16_;
    let tlambda: libc::c_int = (*dqm).tlambda_;
    let src: *const uint8_t = ((*it).yuv_in_).offset(0 as libc::c_int as isize);
    let mut rd_tmp: VP8ModeScore = VP8ModeScore {
        D: 0,
        SD: 0,
        H: 0,
        R: 0,
        score: 0,
        y_dc_levels: [0; 16],
        y_ac_levels: [[0; 16]; 16],
        uv_levels: [[0; 16]; 8],
        mode_i16: 0,
        modes_i4: [0; 16],
        mode_uv: 0,
        nz: 0,
        derr: [[0; 3]; 2],
    };
    let mut rd_cur: *mut VP8ModeScore = &mut rd_tmp;
    let mut rd_best: *mut VP8ModeScore = rd;
    let mut mode: libc::c_int = 0;
    let mut is_flat: libc::c_int = IsFlatSource16(
        ((*it).yuv_in_).offset(0 as libc::c_int as isize),
    );
    (*rd).mode_i16 = -(1 as libc::c_int);
    mode = 0 as libc::c_int;
    while mode < NUM_PRED_MODES as libc::c_int {
        let tmp_dst: *mut uint8_t = ((*it).yuv_out2_).offset(0 as libc::c_int as isize);
        (*rd_cur).mode_i16 = mode;
        (*rd_cur).nz = ReconstructIntra16(it, rd_cur, tmp_dst, mode) as uint32_t;
        (*rd_cur)
            .D = VP8SSE16x16.expect("non-null function pointer")(src, tmp_dst)
            as score_t;
        (*rd_cur)
            .SD = (if tlambda != 0 {
            tlambda
                * VP8TDisto16x16
                    .expect("non-null function pointer")(src, tmp_dst, kWeightY.as_ptr())
                + 128 as libc::c_int >> 8 as libc::c_int
        } else {
            0 as libc::c_int
        }) as score_t;
        (*rd_cur).H = VP8FixedCostsI16[mode as usize] as score_t;
        (*rd_cur).R = VP8GetCostLuma16(it, rd_cur) as score_t;
        if is_flat != 0 {
            is_flat = IsFlat_C(
                ((*rd_cur).y_ac_levels[0 as libc::c_int as usize]).as_mut_ptr(),
                kNumBlocks,
                0 as libc::c_int,
            );
            if is_flat != 0 {
                (*rd_cur).D *= 2 as libc::c_int as libc::c_long;
                (*rd_cur).SD *= 2 as libc::c_int as libc::c_long;
            }
        }
        SetRDScore(lambda, rd_cur);
        if mode == 0 as libc::c_int || (*rd_cur).score < (*rd_best).score {
            SwapModeScore(&mut rd_cur, &mut rd_best);
            SwapOut(it);
        }
        mode += 1;
        mode;
    }
    if rd_best != rd {
        memcpy(
            rd as *mut libc::c_void,
            rd_best as *const libc::c_void,
            ::core::mem::size_of::<VP8ModeScore>() as libc::c_ulong,
        );
    }
    SetRDScore((*dqm).lambda_mode_, rd);
    VP8SetIntra16Mode(it, (*rd).mode_i16);
    if (*rd).nz & 0x100ffff as libc::c_int as libc::c_uint
        == 0x1000000 as libc::c_int as libc::c_uint
        && (*rd).D > (*dqm).min_disto_ as libc::c_long
    {
        StoreMaxDelta(dqm, ((*rd).y_dc_levels).as_mut_ptr() as *const int16_t);
    }
}
unsafe extern "C" fn GetCostModeI4(
    it: *mut VP8EncIterator,
    mut modes: *const uint8_t,
) -> *const uint16_t {
    let preds_w: libc::c_int = (*(*it).enc_).preds_w_;
    let x: libc::c_int = (*it).i4_ & 3 as libc::c_int;
    let y: libc::c_int = (*it).i4_ >> 2 as libc::c_int;
    let left: libc::c_int = if x == 0 as libc::c_int {
        *((*it).preds_).offset((y * preds_w - 1 as libc::c_int) as isize) as libc::c_int
    } else {
        *modes.offset(((*it).i4_ - 1 as libc::c_int) as isize) as libc::c_int
    };
    let top: libc::c_int = if y == 0 as libc::c_int {
        *((*it).preds_).offset((-preds_w + x) as isize) as libc::c_int
    } else {
        *modes.offset(((*it).i4_ - 4 as libc::c_int) as isize) as libc::c_int
    };
    return (VP8FixedCostsI4[top as usize][left as usize]).as_ptr();
}
unsafe extern "C" fn PickBestIntra4(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
) -> libc::c_int {
    let enc: *const VP8Encoder = (*it).enc_;
    let dqm: *const VP8SegmentInfo = &*((*enc).dqm_)
        .as_ptr()
        .offset((*(*it).mb_).segment_() as isize) as *const VP8SegmentInfo;
    let lambda: libc::c_int = (*dqm).lambda_i4_;
    let tlambda: libc::c_int = (*dqm).tlambda_;
    let src0: *const uint8_t = ((*it).yuv_in_).offset(0 as libc::c_int as isize);
    let best_blocks: *mut uint8_t = ((*it).yuv_out2_).offset(0 as libc::c_int as isize);
    let mut total_header_bits: libc::c_int = 0 as libc::c_int;
    let mut rd_best: VP8ModeScore = VP8ModeScore {
        D: 0,
        SD: 0,
        H: 0,
        R: 0,
        score: 0,
        y_dc_levels: [0; 16],
        y_ac_levels: [[0; 16]; 16],
        uv_levels: [[0; 16]; 8],
        mode_i16: 0,
        modes_i4: [0; 16],
        mode_uv: 0,
        nz: 0,
        derr: [[0; 3]; 2],
    };
    if (*enc).max_i4_header_bits_ == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    InitScore(&mut rd_best);
    rd_best.H = 211 as libc::c_int as score_t;
    SetRDScore((*dqm).lambda_mode_, &mut rd_best);
    VP8IteratorStartI4(it);
    loop {
        let kNumBlocks: libc::c_int = 1 as libc::c_int;
        let mut rd_i4: VP8ModeScore = VP8ModeScore {
            D: 0,
            SD: 0,
            H: 0,
            R: 0,
            score: 0,
            y_dc_levels: [0; 16],
            y_ac_levels: [[0; 16]; 16],
            uv_levels: [[0; 16]; 8],
            mode_i16: 0,
            modes_i4: [0; 16],
            mode_uv: 0,
            nz: 0,
            derr: [[0; 3]; 2],
        };
        let mut mode: libc::c_int = 0;
        let mut best_mode: libc::c_int = -(1 as libc::c_int);
        let src: *const uint8_t = src0
            .offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize);
        let mode_costs: *const uint16_t = GetCostModeI4(
            it,
            ((*rd).modes_i4).as_mut_ptr() as *const uint8_t,
        );
        let mut best_block: *mut uint8_t = best_blocks
            .offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize);
        let mut tmp_dst: *mut uint8_t = ((*it).yuv_p_)
            .offset(
                (3 as libc::c_int * 16 as libc::c_int * 32 as libc::c_int
                    + 4 as libc::c_int * 32 as libc::c_int + 8 as libc::c_int) as isize,
            );
        InitScore(&mut rd_i4);
        VP8MakeIntra4Preds(it);
        mode = 0 as libc::c_int;
        while mode < NUM_BMODES as libc::c_int {
            let mut rd_tmp: VP8ModeScore = VP8ModeScore {
                D: 0,
                SD: 0,
                H: 0,
                R: 0,
                score: 0,
                y_dc_levels: [0; 16],
                y_ac_levels: [[0; 16]; 16],
                uv_levels: [[0; 16]; 8],
                mode_i16: 0,
                modes_i4: [0; 16],
                mode_uv: 0,
                nz: 0,
                derr: [[0; 3]; 2],
            };
            let mut tmp_levels: [int16_t; 16] = [0; 16];
            rd_tmp
                .nz = (ReconstructIntra4(it, tmp_levels.as_mut_ptr(), src, tmp_dst, mode)
                << (*it).i4_) as uint32_t;
            rd_tmp
                .D = VP8SSE4x4.expect("non-null function pointer")(src, tmp_dst)
                as score_t;
            rd_tmp
                .SD = (if tlambda != 0 {
                tlambda
                    * VP8TDisto4x4
                        .expect(
                            "non-null function pointer",
                        )(src, tmp_dst, kWeightY.as_ptr()) + 128 as libc::c_int
                    >> 8 as libc::c_int
            } else {
                0 as libc::c_int
            }) as score_t;
            rd_tmp.H = *mode_costs.offset(mode as isize) as score_t;
            if mode > 0 as libc::c_int
                && IsFlat_C(tmp_levels.as_mut_ptr(), kNumBlocks, 3 as libc::c_int) != 0
            {
                rd_tmp.R = (140 as libc::c_int * kNumBlocks) as score_t;
            } else {
                rd_tmp.R = 0 as libc::c_int as score_t;
            }
            SetRDScore(lambda, &mut rd_tmp);
            if !(best_mode >= 0 as libc::c_int && rd_tmp.score >= rd_i4.score) {
                rd_tmp.R
                    += VP8GetCostLuma4(it, tmp_levels.as_mut_ptr() as *const int16_t)
                        as libc::c_long;
                SetRDScore(lambda, &mut rd_tmp);
                if best_mode < 0 as libc::c_int || rd_tmp.score < rd_i4.score {
                    CopyScore(&mut rd_i4, &mut rd_tmp);
                    best_mode = mode;
                    SwapPtr(&mut tmp_dst, &mut best_block);
                    memcpy(
                        (rd_best.y_ac_levels[(*it).i4_ as usize]).as_mut_ptr()
                            as *mut libc::c_void,
                        tmp_levels.as_mut_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<[int16_t; 16]>() as libc::c_ulong,
                    );
                }
            }
            mode += 1;
            mode;
        }
        SetRDScore((*dqm).lambda_mode_, &mut rd_i4);
        AddScore(&mut rd_best, &mut rd_i4);
        if rd_best.score >= (*rd).score {
            return 0 as libc::c_int;
        }
        total_header_bits += rd_i4.H as libc::c_int;
        if total_header_bits > (*enc).max_i4_header_bits_ {
            return 0 as libc::c_int;
        }
        if best_block
            != best_blocks.offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize)
        {
            VP8Copy4x4
                .expect(
                    "non-null function pointer",
                )(
                best_block,
                best_blocks.offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize),
            );
        }
        (*rd).modes_i4[(*it).i4_ as usize] = best_mode as uint8_t;
        (*it)
            .left_nz_[((*it).i4_ >> 2 as libc::c_int)
            as usize] = if rd_i4.nz != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
        (*it)
            .top_nz_[((*it).i4_ & 3 as libc::c_int)
            as usize] = (*it).left_nz_[((*it).i4_ >> 2 as libc::c_int) as usize];
        if !(VP8IteratorRotateI4(it, best_blocks) != 0) {
            break;
        }
    }
    CopyScore(rd, &mut rd_best);
    VP8SetIntra4Mode(it, ((*rd).modes_i4).as_mut_ptr());
    SwapOut(it);
    memcpy(
        ((*rd).y_ac_levels).as_mut_ptr() as *mut libc::c_void,
        (rd_best.y_ac_levels).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[[int16_t; 16]; 16]>() as libc::c_ulong,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn PickBestUV(it: *mut VP8EncIterator, rd: *mut VP8ModeScore) {
    let kNumBlocks: libc::c_int = 8 as libc::c_int;
    let dqm: *const VP8SegmentInfo = &mut *((*(*it).enc_).dqm_)
        .as_mut_ptr()
        .offset((*(*it).mb_).segment_() as isize) as *mut VP8SegmentInfo;
    let lambda: libc::c_int = (*dqm).lambda_uv_;
    let src: *const uint8_t = ((*it).yuv_in_).offset(16 as libc::c_int as isize);
    let mut tmp_dst: *mut uint8_t = ((*it).yuv_out2_).offset(16 as libc::c_int as isize);
    let mut dst0: *mut uint8_t = ((*it).yuv_out_).offset(16 as libc::c_int as isize);
    let mut dst: *mut uint8_t = dst0;
    let mut rd_best: VP8ModeScore = VP8ModeScore {
        D: 0,
        SD: 0,
        H: 0,
        R: 0,
        score: 0,
        y_dc_levels: [0; 16],
        y_ac_levels: [[0; 16]; 16],
        uv_levels: [[0; 16]; 8],
        mode_i16: 0,
        modes_i4: [0; 16],
        mode_uv: 0,
        nz: 0,
        derr: [[0; 3]; 2],
    };
    let mut mode: libc::c_int = 0;
    (*rd).mode_uv = -(1 as libc::c_int);
    InitScore(&mut rd_best);
    mode = 0 as libc::c_int;
    while mode < NUM_PRED_MODES as libc::c_int {
        let mut rd_uv: VP8ModeScore = VP8ModeScore {
            D: 0,
            SD: 0,
            H: 0,
            R: 0,
            score: 0,
            y_dc_levels: [0; 16],
            y_ac_levels: [[0; 16]; 16],
            uv_levels: [[0; 16]; 8],
            mode_i16: 0,
            modes_i4: [0; 16],
            mode_uv: 0,
            nz: 0,
            derr: [[0; 3]; 2],
        };
        rd_uv.nz = ReconstructUV(it, &mut rd_uv, tmp_dst, mode) as uint32_t;
        rd_uv
            .D = VP8SSE16x8.expect("non-null function pointer")(src, tmp_dst) as score_t;
        rd_uv.SD = 0 as libc::c_int as score_t;
        rd_uv.H = VP8FixedCostsUV[mode as usize] as score_t;
        rd_uv.R = VP8GetCostUV(it, &mut rd_uv) as score_t;
        if mode > 0 as libc::c_int
            && IsFlat_C(
                (rd_uv.uv_levels[0 as libc::c_int as usize]).as_mut_ptr(),
                kNumBlocks,
                2 as libc::c_int,
            ) != 0
        {
            rd_uv.R += (140 as libc::c_int * kNumBlocks) as libc::c_long;
        }
        SetRDScore(lambda, &mut rd_uv);
        if mode == 0 as libc::c_int || rd_uv.score < rd_best.score {
            CopyScore(&mut rd_best, &mut rd_uv);
            (*rd).mode_uv = mode;
            memcpy(
                ((*rd).uv_levels).as_mut_ptr() as *mut libc::c_void,
                (rd_uv.uv_levels).as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[[int16_t; 16]; 8]>() as libc::c_ulong,
            );
            if !((*it).top_derr_).is_null() {
                memcpy(
                    ((*rd).derr).as_mut_ptr() as *mut libc::c_void,
                    (rd_uv.derr).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[[int8_t; 3]; 2]>() as libc::c_ulong,
                );
            }
            SwapPtr(&mut dst, &mut tmp_dst);
        }
        mode += 1;
        mode;
    }
    VP8SetIntraUVMode(it, (*rd).mode_uv);
    AddScore(rd, &mut rd_best);
    if dst != dst0 {
        VP8Copy16x8.expect("non-null function pointer")(dst, dst0);
    }
    if !((*it).top_derr_).is_null() {
        StoreDiffusionErrors(it, rd);
    }
}
unsafe extern "C" fn SimpleQuantize(it: *mut VP8EncIterator, rd: *mut VP8ModeScore) {
    let enc: *const VP8Encoder = (*it).enc_;
    let is_i16: libc::c_int = ((*(*it).mb_).type_() as libc::c_int == 1 as libc::c_int)
        as libc::c_int;
    let mut nz: libc::c_int = 0 as libc::c_int;
    if is_i16 != 0 {
        nz = ReconstructIntra16(
            it,
            rd,
            ((*it).yuv_out_).offset(0 as libc::c_int as isize),
            *((*it).preds_).offset(0 as libc::c_int as isize) as libc::c_int,
        );
    } else {
        VP8IteratorStartI4(it);
        loop {
            let mode: libc::c_int = *((*it).preds_)
                .offset(
                    (((*it).i4_ & 3 as libc::c_int)
                        + ((*it).i4_ >> 2 as libc::c_int) * (*enc).preds_w_) as isize,
                ) as libc::c_int;
            let src: *const uint8_t = ((*it).yuv_in_)
                .offset(0 as libc::c_int as isize)
                .offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize);
            let dst: *mut uint8_t = ((*it).yuv_out_)
                .offset(0 as libc::c_int as isize)
                .offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize);
            VP8MakeIntra4Preds(it);
            nz
                |= ReconstructIntra4(
                    it,
                    ((*rd).y_ac_levels[(*it).i4_ as usize]).as_mut_ptr(),
                    src,
                    dst,
                    mode,
                ) << (*it).i4_;
            if !(VP8IteratorRotateI4(
                it,
                ((*it).yuv_out_).offset(0 as libc::c_int as isize),
            ) != 0)
            {
                break;
            }
        }
    }
    nz
        |= ReconstructUV(
            it,
            rd,
            ((*it).yuv_out_).offset(16 as libc::c_int as isize),
            (*(*it).mb_).uv_mode_() as libc::c_int,
        );
    (*rd).nz = nz as uint32_t;
}
unsafe extern "C" fn RefineUsingDistortion(
    it: *mut VP8EncIterator,
    mut try_both_modes: libc::c_int,
    mut refine_uv_mode: libc::c_int,
    rd: *mut VP8ModeScore,
) {
    let mut best_score: score_t = 0x7fffffffffffff as libc::c_longlong as score_t;
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut mode: libc::c_int = 0;
    let mut is_i16: libc::c_int = (try_both_modes != 0
        || (*(*it).mb_).type_() as libc::c_int == 1 as libc::c_int) as libc::c_int;
    let dqm: *const VP8SegmentInfo = &mut *((*(*it).enc_).dqm_)
        .as_mut_ptr()
        .offset((*(*it).mb_).segment_() as isize) as *mut VP8SegmentInfo;
    let lambda_d_i16: libc::c_int = 106 as libc::c_int;
    let lambda_d_i4: libc::c_int = 11 as libc::c_int;
    let lambda_d_uv: libc::c_int = 120 as libc::c_int;
    let mut score_i4: score_t = (*dqm).i4_penalty_;
    let mut i4_bit_sum: score_t = 0 as libc::c_int as score_t;
    let bit_limit: score_t = if try_both_modes != 0 {
        (*(*it).enc_).mb_header_limit_ as libc::c_long
    } else {
        0x7fffffffffffff as libc::c_longlong as score_t
    };
    if is_i16 != 0 {
        let mut best_mode: libc::c_int = -(1 as libc::c_int);
        let src: *const uint8_t = ((*it).yuv_in_).offset(0 as libc::c_int as isize);
        mode = 0 as libc::c_int;
        while mode < NUM_PRED_MODES as libc::c_int {
            let ref_0: *const uint8_t = ((*it).yuv_p_)
                .offset(VP8I16ModeOffsets[mode as usize] as libc::c_int as isize);
            let score: score_t = VP8SSE16x16
                .expect("non-null function pointer")(src, ref_0) as score_t
                * 256 as libc::c_int as libc::c_long
                + (VP8FixedCostsI16[mode as usize] as libc::c_int * lambda_d_i16)
                    as libc::c_long;
            if !(mode > 0 as libc::c_int
                && VP8FixedCostsI16[mode as usize] as libc::c_long > bit_limit)
            {
                if score < best_score {
                    best_mode = mode;
                    best_score = score;
                }
            }
            mode += 1;
            mode;
        }
        if (*it).x_ == 0 as libc::c_int || (*it).y_ == 0 as libc::c_int {
            if IsFlatSource16(src) != 0 {
                best_mode = if (*it).x_ == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    2 as libc::c_int
                };
                try_both_modes = 0 as libc::c_int;
            }
        }
        VP8SetIntra16Mode(it, best_mode);
    }
    if try_both_modes != 0 || is_i16 == 0 {
        is_i16 = 0 as libc::c_int;
        VP8IteratorStartI4(it);
        loop {
            let mut best_i4_mode: libc::c_int = -(1 as libc::c_int);
            let mut best_i4_score: score_t = 0x7fffffffffffff as libc::c_longlong
                as score_t;
            let src_0: *const uint8_t = ((*it).yuv_in_)
                .offset(0 as libc::c_int as isize)
                .offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize);
            let mode_costs: *const uint16_t = GetCostModeI4(
                it,
                ((*rd).modes_i4).as_mut_ptr() as *const uint8_t,
            );
            VP8MakeIntra4Preds(it);
            mode = 0 as libc::c_int;
            while mode < NUM_BMODES as libc::c_int {
                let ref_1: *const uint8_t = ((*it).yuv_p_)
                    .offset(VP8I4ModeOffsets[mode as usize] as libc::c_int as isize);
                let score_0: score_t = (VP8SSE4x4
                    .expect("non-null function pointer")(src_0, ref_1)
                    * 256 as libc::c_int
                    + *mode_costs.offset(mode as isize) as libc::c_int * lambda_d_i4)
                    as score_t;
                if score_0 < best_i4_score {
                    best_i4_mode = mode;
                    best_i4_score = score_0;
                }
                mode += 1;
                mode;
            }
            i4_bit_sum += *mode_costs.offset(best_i4_mode as isize) as libc::c_long;
            (*rd).modes_i4[(*it).i4_ as usize] = best_i4_mode as uint8_t;
            score_i4 += best_i4_score;
            if score_i4 >= best_score || i4_bit_sum > bit_limit {
                is_i16 = 1 as libc::c_int;
                break;
            } else {
                let tmp_dst: *mut uint8_t = ((*it).yuv_out2_)
                    .offset(0 as libc::c_int as isize)
                    .offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize);
                nz
                    |= ReconstructIntra4(
                        it,
                        ((*rd).y_ac_levels[(*it).i4_ as usize]).as_mut_ptr(),
                        src_0,
                        tmp_dst,
                        best_i4_mode,
                    ) << (*it).i4_;
                if !(VP8IteratorRotateI4(
                    it,
                    ((*it).yuv_out2_).offset(0 as libc::c_int as isize),
                ) != 0)
                {
                    break;
                }
            }
        }
    }
    if is_i16 == 0 {
        VP8SetIntra4Mode(it, ((*rd).modes_i4).as_mut_ptr());
        SwapOut(it);
        best_score = score_i4;
    } else {
        nz = ReconstructIntra16(
            it,
            rd,
            ((*it).yuv_out_).offset(0 as libc::c_int as isize),
            *((*it).preds_).offset(0 as libc::c_int as isize) as libc::c_int,
        );
    }
    if refine_uv_mode != 0 {
        let mut best_mode_0: libc::c_int = -(1 as libc::c_int);
        let mut best_uv_score: score_t = 0x7fffffffffffff as libc::c_longlong as score_t;
        let src_1: *const uint8_t = ((*it).yuv_in_).offset(16 as libc::c_int as isize);
        mode = 0 as libc::c_int;
        while mode < NUM_PRED_MODES as libc::c_int {
            let ref_2: *const uint8_t = ((*it).yuv_p_)
                .offset(VP8UVModeOffsets[mode as usize] as libc::c_int as isize);
            let score_1: score_t = (VP8SSE16x8
                .expect("non-null function pointer")(src_1, ref_2) * 256 as libc::c_int
                + VP8FixedCostsUV[mode as usize] as libc::c_int * lambda_d_uv)
                as score_t;
            if score_1 < best_uv_score {
                best_mode_0 = mode;
                best_uv_score = score_1;
            }
            mode += 1;
            mode;
        }
        VP8SetIntraUVMode(it, best_mode_0);
    }
    nz
        |= ReconstructUV(
            it,
            rd,
            ((*it).yuv_out_).offset(16 as libc::c_int as isize),
            (*(*it).mb_).uv_mode_() as libc::c_int,
        );
    (*rd).nz = nz as uint32_t;
    (*rd).score = best_score;
}
#[no_mangle]
pub unsafe extern "C" fn VP8Decimate(
    it: *mut VP8EncIterator,
    rd: *mut VP8ModeScore,
    mut rd_opt: VP8RDLevel,
) -> libc::c_int {
    let mut is_skipped: libc::c_int = 0;
    let method: libc::c_int = (*(*it).enc_).method_;
    InitScore(rd);
    VP8MakeLuma16Preds(it);
    VP8MakeChroma8Preds(it);
    if rd_opt as libc::c_uint > RD_OPT_NONE as libc::c_int as libc::c_uint {
        (*it)
            .do_trellis_ = (rd_opt as libc::c_uint
            >= RD_OPT_TRELLIS_ALL as libc::c_int as libc::c_uint) as libc::c_int;
        PickBestIntra16(it, rd);
        if method >= 2 as libc::c_int {
            PickBestIntra4(it, rd);
        }
        PickBestUV(it, rd);
        if rd_opt as libc::c_uint == RD_OPT_TRELLIS as libc::c_int as libc::c_uint {
            (*it).do_trellis_ = 1 as libc::c_int;
            SimpleQuantize(it, rd);
        }
    } else {
        RefineUsingDistortion(
            it,
            (method >= 2 as libc::c_int) as libc::c_int,
            (method >= 1 as libc::c_int) as libc::c_int,
            rd,
        );
    }
    is_skipped = ((*rd).nz == 0 as libc::c_int as libc::c_uint) as libc::c_int;
    VP8SetSkip(it, is_skipped);
    return is_skipped;
}
