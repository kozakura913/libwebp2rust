use ::libc;
use ::c2rust_bitfields;

use super::token_enc::VP8Tokens;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    static VP8EntropyCost: [uint16_t; 256];
    static VP8EncBands: [uint8_t; 17];
    static mut VP8SetResidualCoeffs: VP8SetResidualCoeffsFunc;
    static mut VP8GetResidualCost: VP8GetResidualCostFunc;
    fn VP8IteratorNzToBytes(it: *mut VP8EncIterator);
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Residual {
    pub first: libc::c_int,
    pub last: libc::c_int,
    pub coeffs: *const int16_t,
    pub coeff_type: libc::c_int,
    pub prob: *mut ProbaArray,
    pub stats: *mut StatsArray,
    pub costs: CostArrayPtr,
}
pub type CostArrayPtr = *mut [*const uint16_t; 3];
pub type StatsArray = [[proba_t; 11]; 3];
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
pub type VP8SetResidualCoeffsFunc = Option::<
    unsafe extern "C" fn(*const int16_t, *mut VP8Residual) -> (),
>;
pub type VP8GetResidualCostFunc = Option::<
    unsafe extern "C" fn(libc::c_int, *const VP8Residual) -> libc::c_int,
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
#[inline]
unsafe extern "C" fn VP8RecordStats(
    mut bit: libc::c_int,
    stats: *mut proba_t,
) -> libc::c_int {
    let mut p: proba_t = *stats;
    if p >= 0xfffe0000 as libc::c_uint {
        p = p.wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int
            & 0x7fff7fff as libc::c_uint;
    }
    p = (p as libc::c_uint)
        .wrapping_add((0x10000 as libc::c_uint).wrapping_add(bit as libc::c_uint))
        as proba_t as proba_t;
    *stats = p;
    return bit;
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
#[no_mangle]
pub static mut VP8LevelCodes: [[uint16_t; 2]; 67] = [
    [0x1 as libc::c_int as uint16_t, 0 as libc::c_int as uint16_t],
    [0x7 as libc::c_int as uint16_t, 0x1 as libc::c_int as uint16_t],
    [0xf as libc::c_int as uint16_t, 0x5 as libc::c_int as uint16_t],
    [0xf as libc::c_int as uint16_t, 0xd as libc::c_int as uint16_t],
    [0x33 as libc::c_int as uint16_t, 0x3 as libc::c_int as uint16_t],
    [0x33 as libc::c_int as uint16_t, 0x3 as libc::c_int as uint16_t],
    [0x33 as libc::c_int as uint16_t, 0x23 as libc::c_int as uint16_t],
    [0x33 as libc::c_int as uint16_t, 0x23 as libc::c_int as uint16_t],
    [0x33 as libc::c_int as uint16_t, 0x23 as libc::c_int as uint16_t],
    [0x33 as libc::c_int as uint16_t, 0x23 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x13 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0xd3 as libc::c_int as uint16_t, 0x93 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x53 as libc::c_int as uint16_t],
    [0x153 as libc::c_int as uint16_t, 0x153 as libc::c_int as uint16_t],
];
unsafe extern "C" fn VariableLevelCost(
    mut level: libc::c_int,
    mut probas: *const uint8_t,
) -> libc::c_int {
    let mut pattern: libc::c_int = VP8LevelCodes[(level - 1 as libc::c_int)
        as usize][0 as libc::c_int as usize] as libc::c_int;
    let mut bits: libc::c_int = VP8LevelCodes[(level - 1 as libc::c_int)
        as usize][1 as libc::c_int as usize] as libc::c_int;
    let mut cost: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 2 as libc::c_int;
    while pattern != 0 {
        if pattern & 1 as libc::c_int != 0 {
            cost += VP8BitCost(bits & 1 as libc::c_int, *probas.offset(i as isize));
        }
        bits >>= 1 as libc::c_int;
        pattern >>= 1 as libc::c_int;
        i += 1;
        i;
    }
    return cost;
}
#[no_mangle]
pub unsafe extern "C" fn VP8CalculateLevelCosts(proba: *mut VP8EncProba) {
    let mut ctype: libc::c_int = 0;
    let mut band: libc::c_int = 0;
    let mut ctx: libc::c_int = 0;
    if (*proba).dirty_ == 0 {
        return;
    }
    ctype = 0 as libc::c_int;
    while ctype < NUM_TYPES as libc::c_int {
        let mut n: libc::c_int = 0;
        band = 0 as libc::c_int;
        while band < NUM_BANDS as libc::c_int {
            ctx = 0 as libc::c_int;
            while ctx < NUM_CTX as libc::c_int {
                let p: *const uint8_t = ((*proba)
                    .coeffs_[ctype as usize][band as usize][ctx as usize])
                    .as_mut_ptr();
                let table: *mut uint16_t = ((*proba)
                    .level_cost_[ctype as usize][band as usize][ctx as usize])
                    .as_mut_ptr();
                let cost0: libc::c_int = if ctx > 0 as libc::c_int {
                    VP8BitCost(1 as libc::c_int, *p.offset(0 as libc::c_int as isize))
                } else {
                    0 as libc::c_int
                };
                let cost_base: libc::c_int = VP8BitCost(
                    1 as libc::c_int,
                    *p.offset(1 as libc::c_int as isize),
                ) + cost0;
                let mut v: libc::c_int = 0;
                *table
                    .offset(
                        0 as libc::c_int as isize,
                    ) = (VP8BitCost(
                    0 as libc::c_int,
                    *p.offset(1 as libc::c_int as isize),
                ) + cost0) as uint16_t;
                v = 1 as libc::c_int;
                while v <= MAX_VARIABLE_LEVEL as libc::c_int {
                    *table
                        .offset(
                            v as isize,
                        ) = (cost_base + VariableLevelCost(v, p)) as uint16_t;
                    v += 1;
                    v;
                }
                ctx += 1;
                ctx;
            }
            band += 1;
            band;
        }
        n = 0 as libc::c_int;
        while n < 16 as libc::c_int {
            ctx = 0 as libc::c_int;
            while ctx < NUM_CTX as libc::c_int {
                (*proba)
                    .remapped_costs_[ctype
                    as usize][n
                    as usize][ctx
                    as usize] = ((*proba)
                    .level_cost_[ctype
                    as usize][VP8EncBands[n as usize] as usize][ctx as usize])
                    .as_mut_ptr();
                ctx += 1;
                ctx;
            }
            n += 1;
            n;
        }
        ctype += 1;
        ctype;
    }
    (*proba).dirty_ = 0 as libc::c_int;
}
#[no_mangle]
pub static mut VP8FixedCostsUV: [uint16_t; 4] = [
    302 as libc::c_int as uint16_t,
    984 as libc::c_int as uint16_t,
    439 as libc::c_int as uint16_t,
    642 as libc::c_int as uint16_t,
];
#[no_mangle]
pub static mut VP8FixedCostsI16: [uint16_t; 4] = [
    663 as libc::c_int as uint16_t,
    919 as libc::c_int as uint16_t,
    872 as libc::c_int as uint16_t,
    919 as libc::c_int as uint16_t,
];
#[no_mangle]
pub static mut VP8FixedCostsI4: [[[uint16_t; 10]; 10]; 10] = [
    [
        [
            40 as libc::c_int as uint16_t,
            1151 as libc::c_int as uint16_t,
            1723 as libc::c_int as uint16_t,
            1874 as libc::c_int as uint16_t,
            2103 as libc::c_int as uint16_t,
            2019 as libc::c_int as uint16_t,
            1628 as libc::c_int as uint16_t,
            1777 as libc::c_int as uint16_t,
            2226 as libc::c_int as uint16_t,
            2137 as libc::c_int as uint16_t,
        ],
        [
            192 as libc::c_int as uint16_t,
            469 as libc::c_int as uint16_t,
            1296 as libc::c_int as uint16_t,
            1308 as libc::c_int as uint16_t,
            1849 as libc::c_int as uint16_t,
            1794 as libc::c_int as uint16_t,
            1781 as libc::c_int as uint16_t,
            1703 as libc::c_int as uint16_t,
            1713 as libc::c_int as uint16_t,
            1522 as libc::c_int as uint16_t,
        ],
        [
            142 as libc::c_int as uint16_t,
            910 as libc::c_int as uint16_t,
            762 as libc::c_int as uint16_t,
            1684 as libc::c_int as uint16_t,
            1849 as libc::c_int as uint16_t,
            1576 as libc::c_int as uint16_t,
            1460 as libc::c_int as uint16_t,
            1305 as libc::c_int as uint16_t,
            1801 as libc::c_int as uint16_t,
            1657 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            641 as libc::c_int as uint16_t,
            1370 as libc::c_int as uint16_t,
            421 as libc::c_int as uint16_t,
            1182 as libc::c_int as uint16_t,
            1569 as libc::c_int as uint16_t,
            1612 as libc::c_int as uint16_t,
            1725 as libc::c_int as uint16_t,
            863 as libc::c_int as uint16_t,
            1007 as libc::c_int as uint16_t,
        ],
        [
            299 as libc::c_int as uint16_t,
            1059 as libc::c_int as uint16_t,
            1256 as libc::c_int as uint16_t,
            1108 as libc::c_int as uint16_t,
            636 as libc::c_int as uint16_t,
            1068 as libc::c_int as uint16_t,
            1581 as libc::c_int as uint16_t,
            1883 as libc::c_int as uint16_t,
            869 as libc::c_int as uint16_t,
            1142 as libc::c_int as uint16_t,
        ],
        [
            277 as libc::c_int as uint16_t,
            1111 as libc::c_int as uint16_t,
            707 as libc::c_int as uint16_t,
            1362 as libc::c_int as uint16_t,
            1089 as libc::c_int as uint16_t,
            672 as libc::c_int as uint16_t,
            1603 as libc::c_int as uint16_t,
            1541 as libc::c_int as uint16_t,
            1545 as libc::c_int as uint16_t,
            1291 as libc::c_int as uint16_t,
        ],
        [
            214 as libc::c_int as uint16_t,
            781 as libc::c_int as uint16_t,
            1609 as libc::c_int as uint16_t,
            1303 as libc::c_int as uint16_t,
            1632 as libc::c_int as uint16_t,
            2229 as libc::c_int as uint16_t,
            726 as libc::c_int as uint16_t,
            1560 as libc::c_int as uint16_t,
            1713 as libc::c_int as uint16_t,
            918 as libc::c_int as uint16_t,
        ],
        [
            152 as libc::c_int as uint16_t,
            1037 as libc::c_int as uint16_t,
            1046 as libc::c_int as uint16_t,
            1759 as libc::c_int as uint16_t,
            1983 as libc::c_int as uint16_t,
            2174 as libc::c_int as uint16_t,
            1358 as libc::c_int as uint16_t,
            742 as libc::c_int as uint16_t,
            1740 as libc::c_int as uint16_t,
            1390 as libc::c_int as uint16_t,
        ],
        [
            512 as libc::c_int as uint16_t,
            1046 as libc::c_int as uint16_t,
            1420 as libc::c_int as uint16_t,
            753 as libc::c_int as uint16_t,
            752 as libc::c_int as uint16_t,
            1297 as libc::c_int as uint16_t,
            1486 as libc::c_int as uint16_t,
            1613 as libc::c_int as uint16_t,
            460 as libc::c_int as uint16_t,
            1207 as libc::c_int as uint16_t,
        ],
        [
            424 as libc::c_int as uint16_t,
            827 as libc::c_int as uint16_t,
            1362 as libc::c_int as uint16_t,
            719 as libc::c_int as uint16_t,
            1462 as libc::c_int as uint16_t,
            1202 as libc::c_int as uint16_t,
            1199 as libc::c_int as uint16_t,
            1476 as libc::c_int as uint16_t,
            1199 as libc::c_int as uint16_t,
            538 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            240 as libc::c_int as uint16_t,
            402 as libc::c_int as uint16_t,
            1134 as libc::c_int as uint16_t,
            1491 as libc::c_int as uint16_t,
            1659 as libc::c_int as uint16_t,
            1505 as libc::c_int as uint16_t,
            1517 as libc::c_int as uint16_t,
            1555 as libc::c_int as uint16_t,
            1979 as libc::c_int as uint16_t,
            2099 as libc::c_int as uint16_t,
        ],
        [
            467 as libc::c_int as uint16_t,
            242 as libc::c_int as uint16_t,
            960 as libc::c_int as uint16_t,
            1232 as libc::c_int as uint16_t,
            1714 as libc::c_int as uint16_t,
            1620 as libc::c_int as uint16_t,
            1834 as libc::c_int as uint16_t,
            1570 as libc::c_int as uint16_t,
            1676 as libc::c_int as uint16_t,
            1391 as libc::c_int as uint16_t,
        ],
        [
            500 as libc::c_int as uint16_t,
            455 as libc::c_int as uint16_t,
            463 as libc::c_int as uint16_t,
            1507 as libc::c_int as uint16_t,
            1699 as libc::c_int as uint16_t,
            1282 as libc::c_int as uint16_t,
            1564 as libc::c_int as uint16_t,
            982 as libc::c_int as uint16_t,
            2114 as libc::c_int as uint16_t,
            2114 as libc::c_int as uint16_t,
        ],
        [
            672 as libc::c_int as uint16_t,
            643 as libc::c_int as uint16_t,
            1372 as libc::c_int as uint16_t,
            331 as libc::c_int as uint16_t,
            1589 as libc::c_int as uint16_t,
            1667 as libc::c_int as uint16_t,
            1453 as libc::c_int as uint16_t,
            1938 as libc::c_int as uint16_t,
            996 as libc::c_int as uint16_t,
            876 as libc::c_int as uint16_t,
        ],
        [
            458 as libc::c_int as uint16_t,
            783 as libc::c_int as uint16_t,
            1037 as libc::c_int as uint16_t,
            911 as libc::c_int as uint16_t,
            738 as libc::c_int as uint16_t,
            968 as libc::c_int as uint16_t,
            1165 as libc::c_int as uint16_t,
            1518 as libc::c_int as uint16_t,
            859 as libc::c_int as uint16_t,
            1033 as libc::c_int as uint16_t,
        ],
        [
            504 as libc::c_int as uint16_t,
            815 as libc::c_int as uint16_t,
            504 as libc::c_int as uint16_t,
            1139 as libc::c_int as uint16_t,
            1219 as libc::c_int as uint16_t,
            719 as libc::c_int as uint16_t,
            1506 as libc::c_int as uint16_t,
            1085 as libc::c_int as uint16_t,
            1268 as libc::c_int as uint16_t,
            1268 as libc::c_int as uint16_t,
        ],
        [
            333 as libc::c_int as uint16_t,
            630 as libc::c_int as uint16_t,
            1445 as libc::c_int as uint16_t,
            1239 as libc::c_int as uint16_t,
            1883 as libc::c_int as uint16_t,
            3672 as libc::c_int as uint16_t,
            799 as libc::c_int as uint16_t,
            1548 as libc::c_int as uint16_t,
            1865 as libc::c_int as uint16_t,
            598 as libc::c_int as uint16_t,
        ],
        [
            399 as libc::c_int as uint16_t,
            644 as libc::c_int as uint16_t,
            746 as libc::c_int as uint16_t,
            1342 as libc::c_int as uint16_t,
            1856 as libc::c_int as uint16_t,
            1350 as libc::c_int as uint16_t,
            1493 as libc::c_int as uint16_t,
            613 as libc::c_int as uint16_t,
            1855 as libc::c_int as uint16_t,
            1015 as libc::c_int as uint16_t,
        ],
        [
            622 as libc::c_int as uint16_t,
            749 as libc::c_int as uint16_t,
            1205 as libc::c_int as uint16_t,
            608 as libc::c_int as uint16_t,
            1066 as libc::c_int as uint16_t,
            1408 as libc::c_int as uint16_t,
            1290 as libc::c_int as uint16_t,
            1406 as libc::c_int as uint16_t,
            546 as libc::c_int as uint16_t,
            971 as libc::c_int as uint16_t,
        ],
        [
            500 as libc::c_int as uint16_t,
            753 as libc::c_int as uint16_t,
            1041 as libc::c_int as uint16_t,
            668 as libc::c_int as uint16_t,
            1230 as libc::c_int as uint16_t,
            1617 as libc::c_int as uint16_t,
            1297 as libc::c_int as uint16_t,
            1425 as libc::c_int as uint16_t,
            1383 as libc::c_int as uint16_t,
            523 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            394 as libc::c_int as uint16_t,
            553 as libc::c_int as uint16_t,
            523 as libc::c_int as uint16_t,
            1502 as libc::c_int as uint16_t,
            1536 as libc::c_int as uint16_t,
            981 as libc::c_int as uint16_t,
            1608 as libc::c_int as uint16_t,
            1142 as libc::c_int as uint16_t,
            1666 as libc::c_int as uint16_t,
            2181 as libc::c_int as uint16_t,
        ],
        [
            655 as libc::c_int as uint16_t,
            430 as libc::c_int as uint16_t,
            375 as libc::c_int as uint16_t,
            1411 as libc::c_int as uint16_t,
            1861 as libc::c_int as uint16_t,
            1220 as libc::c_int as uint16_t,
            1677 as libc::c_int as uint16_t,
            1135 as libc::c_int as uint16_t,
            1978 as libc::c_int as uint16_t,
            1553 as libc::c_int as uint16_t,
        ],
        [
            690 as libc::c_int as uint16_t,
            640 as libc::c_int as uint16_t,
            245 as libc::c_int as uint16_t,
            1954 as libc::c_int as uint16_t,
            2070 as libc::c_int as uint16_t,
            1194 as libc::c_int as uint16_t,
            1528 as libc::c_int as uint16_t,
            982 as libc::c_int as uint16_t,
            1972 as libc::c_int as uint16_t,
            2232 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            834 as libc::c_int as uint16_t,
            741 as libc::c_int as uint16_t,
            867 as libc::c_int as uint16_t,
            1131 as libc::c_int as uint16_t,
            980 as libc::c_int as uint16_t,
            1225 as libc::c_int as uint16_t,
            852 as libc::c_int as uint16_t,
            1092 as libc::c_int as uint16_t,
            784 as libc::c_int as uint16_t,
        ],
        [
            690 as libc::c_int as uint16_t,
            875 as libc::c_int as uint16_t,
            516 as libc::c_int as uint16_t,
            959 as libc::c_int as uint16_t,
            673 as libc::c_int as uint16_t,
            894 as libc::c_int as uint16_t,
            1056 as libc::c_int as uint16_t,
            1190 as libc::c_int as uint16_t,
            1528 as libc::c_int as uint16_t,
            1126 as libc::c_int as uint16_t,
        ],
        [
            740 as libc::c_int as uint16_t,
            951 as libc::c_int as uint16_t,
            384 as libc::c_int as uint16_t,
            1277 as libc::c_int as uint16_t,
            1177 as libc::c_int as uint16_t,
            492 as libc::c_int as uint16_t,
            1579 as libc::c_int as uint16_t,
            1155 as libc::c_int as uint16_t,
            1846 as libc::c_int as uint16_t,
            1513 as libc::c_int as uint16_t,
        ],
        [
            323 as libc::c_int as uint16_t,
            775 as libc::c_int as uint16_t,
            1062 as libc::c_int as uint16_t,
            1776 as libc::c_int as uint16_t,
            3062 as libc::c_int as uint16_t,
            1274 as libc::c_int as uint16_t,
            813 as libc::c_int as uint16_t,
            1188 as libc::c_int as uint16_t,
            1372 as libc::c_int as uint16_t,
            655 as libc::c_int as uint16_t,
        ],
        [
            488 as libc::c_int as uint16_t,
            971 as libc::c_int as uint16_t,
            484 as libc::c_int as uint16_t,
            1767 as libc::c_int as uint16_t,
            1515 as libc::c_int as uint16_t,
            1775 as libc::c_int as uint16_t,
            1115 as libc::c_int as uint16_t,
            503 as libc::c_int as uint16_t,
            1539 as libc::c_int as uint16_t,
            1461 as libc::c_int as uint16_t,
        ],
        [
            740 as libc::c_int as uint16_t,
            1006 as libc::c_int as uint16_t,
            998 as libc::c_int as uint16_t,
            709 as libc::c_int as uint16_t,
            851 as libc::c_int as uint16_t,
            1230 as libc::c_int as uint16_t,
            1337 as libc::c_int as uint16_t,
            788 as libc::c_int as uint16_t,
            741 as libc::c_int as uint16_t,
            721 as libc::c_int as uint16_t,
        ],
        [
            522 as libc::c_int as uint16_t,
            1073 as libc::c_int as uint16_t,
            573 as libc::c_int as uint16_t,
            1045 as libc::c_int as uint16_t,
            1346 as libc::c_int as uint16_t,
            887 as libc::c_int as uint16_t,
            1046 as libc::c_int as uint16_t,
            1146 as libc::c_int as uint16_t,
            1203 as libc::c_int as uint16_t,
            697 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            105 as libc::c_int as uint16_t,
            864 as libc::c_int as uint16_t,
            1442 as libc::c_int as uint16_t,
            1009 as libc::c_int as uint16_t,
            1934 as libc::c_int as uint16_t,
            1840 as libc::c_int as uint16_t,
            1519 as libc::c_int as uint16_t,
            1920 as libc::c_int as uint16_t,
            1673 as libc::c_int as uint16_t,
            1579 as libc::c_int as uint16_t,
        ],
        [
            534 as libc::c_int as uint16_t,
            305 as libc::c_int as uint16_t,
            1193 as libc::c_int as uint16_t,
            683 as libc::c_int as uint16_t,
            1388 as libc::c_int as uint16_t,
            2164 as libc::c_int as uint16_t,
            1802 as libc::c_int as uint16_t,
            1894 as libc::c_int as uint16_t,
            1264 as libc::c_int as uint16_t,
            1170 as libc::c_int as uint16_t,
        ],
        [
            305 as libc::c_int as uint16_t,
            518 as libc::c_int as uint16_t,
            877 as libc::c_int as uint16_t,
            1108 as libc::c_int as uint16_t,
            1426 as libc::c_int as uint16_t,
            3215 as libc::c_int as uint16_t,
            1425 as libc::c_int as uint16_t,
            1064 as libc::c_int as uint16_t,
            1320 as libc::c_int as uint16_t,
            1242 as libc::c_int as uint16_t,
        ],
        [
            683 as libc::c_int as uint16_t,
            732 as libc::c_int as uint16_t,
            1927 as libc::c_int as uint16_t,
            257 as libc::c_int as uint16_t,
            1493 as libc::c_int as uint16_t,
            2048 as libc::c_int as uint16_t,
            1858 as libc::c_int as uint16_t,
            1552 as libc::c_int as uint16_t,
            1055 as libc::c_int as uint16_t,
            947 as libc::c_int as uint16_t,
        ],
        [
            394 as libc::c_int as uint16_t,
            814 as libc::c_int as uint16_t,
            1024 as libc::c_int as uint16_t,
            660 as libc::c_int as uint16_t,
            959 as libc::c_int as uint16_t,
            1556 as libc::c_int as uint16_t,
            1282 as libc::c_int as uint16_t,
            1289 as libc::c_int as uint16_t,
            893 as libc::c_int as uint16_t,
            1047 as libc::c_int as uint16_t,
        ],
        [
            528 as libc::c_int as uint16_t,
            615 as libc::c_int as uint16_t,
            996 as libc::c_int as uint16_t,
            940 as libc::c_int as uint16_t,
            1201 as libc::c_int as uint16_t,
            635 as libc::c_int as uint16_t,
            1094 as libc::c_int as uint16_t,
            2515 as libc::c_int as uint16_t,
            803 as libc::c_int as uint16_t,
            1358 as libc::c_int as uint16_t,
        ],
        [
            347 as libc::c_int as uint16_t,
            614 as libc::c_int as uint16_t,
            1609 as libc::c_int as uint16_t,
            1187 as libc::c_int as uint16_t,
            3133 as libc::c_int as uint16_t,
            1345 as libc::c_int as uint16_t,
            1007 as libc::c_int as uint16_t,
            1339 as libc::c_int as uint16_t,
            1017 as libc::c_int as uint16_t,
            667 as libc::c_int as uint16_t,
        ],
        [
            218 as libc::c_int as uint16_t,
            740 as libc::c_int as uint16_t,
            878 as libc::c_int as uint16_t,
            1605 as libc::c_int as uint16_t,
            3650 as libc::c_int as uint16_t,
            3650 as libc::c_int as uint16_t,
            1345 as libc::c_int as uint16_t,
            758 as libc::c_int as uint16_t,
            1357 as libc::c_int as uint16_t,
            1617 as libc::c_int as uint16_t,
        ],
        [
            672 as libc::c_int as uint16_t,
            750 as libc::c_int as uint16_t,
            1541 as libc::c_int as uint16_t,
            558 as libc::c_int as uint16_t,
            1257 as libc::c_int as uint16_t,
            1599 as libc::c_int as uint16_t,
            1870 as libc::c_int as uint16_t,
            2135 as libc::c_int as uint16_t,
            402 as libc::c_int as uint16_t,
            1087 as libc::c_int as uint16_t,
        ],
        [
            592 as libc::c_int as uint16_t,
            684 as libc::c_int as uint16_t,
            1161 as libc::c_int as uint16_t,
            430 as libc::c_int as uint16_t,
            1092 as libc::c_int as uint16_t,
            1497 as libc::c_int as uint16_t,
            1475 as libc::c_int as uint16_t,
            1489 as libc::c_int as uint16_t,
            1095 as libc::c_int as uint16_t,
            822 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            228 as libc::c_int as uint16_t,
            1056 as libc::c_int as uint16_t,
            1059 as libc::c_int as uint16_t,
            1368 as libc::c_int as uint16_t,
            752 as libc::c_int as uint16_t,
            982 as libc::c_int as uint16_t,
            1512 as libc::c_int as uint16_t,
            1518 as libc::c_int as uint16_t,
            987 as libc::c_int as uint16_t,
            1782 as libc::c_int as uint16_t,
        ],
        [
            494 as libc::c_int as uint16_t,
            514 as libc::c_int as uint16_t,
            818 as libc::c_int as uint16_t,
            942 as libc::c_int as uint16_t,
            965 as libc::c_int as uint16_t,
            892 as libc::c_int as uint16_t,
            1610 as libc::c_int as uint16_t,
            1356 as libc::c_int as uint16_t,
            1048 as libc::c_int as uint16_t,
            1363 as libc::c_int as uint16_t,
        ],
        [
            512 as libc::c_int as uint16_t,
            648 as libc::c_int as uint16_t,
            591 as libc::c_int as uint16_t,
            1042 as libc::c_int as uint16_t,
            761 as libc::c_int as uint16_t,
            991 as libc::c_int as uint16_t,
            1196 as libc::c_int as uint16_t,
            1454 as libc::c_int as uint16_t,
            1309 as libc::c_int as uint16_t,
            1463 as libc::c_int as uint16_t,
        ],
        [
            683 as libc::c_int as uint16_t,
            749 as libc::c_int as uint16_t,
            1043 as libc::c_int as uint16_t,
            676 as libc::c_int as uint16_t,
            841 as libc::c_int as uint16_t,
            1396 as libc::c_int as uint16_t,
            1133 as libc::c_int as uint16_t,
            1138 as libc::c_int as uint16_t,
            654 as libc::c_int as uint16_t,
            939 as libc::c_int as uint16_t,
        ],
        [
            622 as libc::c_int as uint16_t,
            1101 as libc::c_int as uint16_t,
            1126 as libc::c_int as uint16_t,
            994 as libc::c_int as uint16_t,
            361 as libc::c_int as uint16_t,
            1077 as libc::c_int as uint16_t,
            1203 as libc::c_int as uint16_t,
            1318 as libc::c_int as uint16_t,
            877 as libc::c_int as uint16_t,
            1219 as libc::c_int as uint16_t,
        ],
        [
            631 as libc::c_int as uint16_t,
            1068 as libc::c_int as uint16_t,
            857 as libc::c_int as uint16_t,
            1650 as libc::c_int as uint16_t,
            651 as libc::c_int as uint16_t,
            477 as libc::c_int as uint16_t,
            1650 as libc::c_int as uint16_t,
            1419 as libc::c_int as uint16_t,
            828 as libc::c_int as uint16_t,
            1170 as libc::c_int as uint16_t,
        ],
        [
            555 as libc::c_int as uint16_t,
            727 as libc::c_int as uint16_t,
            1068 as libc::c_int as uint16_t,
            1335 as libc::c_int as uint16_t,
            3127 as libc::c_int as uint16_t,
            1339 as libc::c_int as uint16_t,
            820 as libc::c_int as uint16_t,
            1331 as libc::c_int as uint16_t,
            1077 as libc::c_int as uint16_t,
            429 as libc::c_int as uint16_t,
        ],
        [
            504 as libc::c_int as uint16_t,
            879 as libc::c_int as uint16_t,
            624 as libc::c_int as uint16_t,
            1398 as libc::c_int as uint16_t,
            889 as libc::c_int as uint16_t,
            889 as libc::c_int as uint16_t,
            1392 as libc::c_int as uint16_t,
            808 as libc::c_int as uint16_t,
            891 as libc::c_int as uint16_t,
            1406 as libc::c_int as uint16_t,
        ],
        [
            683 as libc::c_int as uint16_t,
            1602 as libc::c_int as uint16_t,
            1289 as libc::c_int as uint16_t,
            977 as libc::c_int as uint16_t,
            578 as libc::c_int as uint16_t,
            983 as libc::c_int as uint16_t,
            1280 as libc::c_int as uint16_t,
            1708 as libc::c_int as uint16_t,
            406 as libc::c_int as uint16_t,
            1122 as libc::c_int as uint16_t,
        ],
        [
            399 as libc::c_int as uint16_t,
            865 as libc::c_int as uint16_t,
            1433 as libc::c_int as uint16_t,
            1070 as libc::c_int as uint16_t,
            1072 as libc::c_int as uint16_t,
            764 as libc::c_int as uint16_t,
            968 as libc::c_int as uint16_t,
            1477 as libc::c_int as uint16_t,
            1223 as libc::c_int as uint16_t,
            678 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            333 as libc::c_int as uint16_t,
            760 as libc::c_int as uint16_t,
            935 as libc::c_int as uint16_t,
            1638 as libc::c_int as uint16_t,
            1010 as libc::c_int as uint16_t,
            529 as libc::c_int as uint16_t,
            1646 as libc::c_int as uint16_t,
            1410 as libc::c_int as uint16_t,
            1472 as libc::c_int as uint16_t,
            2219 as libc::c_int as uint16_t,
        ],
        [
            512 as libc::c_int as uint16_t,
            494 as libc::c_int as uint16_t,
            750 as libc::c_int as uint16_t,
            1160 as libc::c_int as uint16_t,
            1215 as libc::c_int as uint16_t,
            610 as libc::c_int as uint16_t,
            1870 as libc::c_int as uint16_t,
            1868 as libc::c_int as uint16_t,
            1628 as libc::c_int as uint16_t,
            1169 as libc::c_int as uint16_t,
        ],
        [
            572 as libc::c_int as uint16_t,
            646 as libc::c_int as uint16_t,
            492 as libc::c_int as uint16_t,
            1934 as libc::c_int as uint16_t,
            1208 as libc::c_int as uint16_t,
            603 as libc::c_int as uint16_t,
            1580 as libc::c_int as uint16_t,
            1099 as libc::c_int as uint16_t,
            1398 as libc::c_int as uint16_t,
            1995 as libc::c_int as uint16_t,
        ],
        [
            786 as libc::c_int as uint16_t,
            789 as libc::c_int as uint16_t,
            942 as libc::c_int as uint16_t,
            581 as libc::c_int as uint16_t,
            1018 as libc::c_int as uint16_t,
            951 as libc::c_int as uint16_t,
            1599 as libc::c_int as uint16_t,
            1207 as libc::c_int as uint16_t,
            731 as libc::c_int as uint16_t,
            768 as libc::c_int as uint16_t,
        ],
        [
            690 as libc::c_int as uint16_t,
            1015 as libc::c_int as uint16_t,
            672 as libc::c_int as uint16_t,
            1078 as libc::c_int as uint16_t,
            582 as libc::c_int as uint16_t,
            504 as libc::c_int as uint16_t,
            1693 as libc::c_int as uint16_t,
            1438 as libc::c_int as uint16_t,
            1108 as libc::c_int as uint16_t,
            2897 as libc::c_int as uint16_t,
        ],
        [
            768 as libc::c_int as uint16_t,
            1267 as libc::c_int as uint16_t,
            571 as libc::c_int as uint16_t,
            2005 as libc::c_int as uint16_t,
            1243 as libc::c_int as uint16_t,
            244 as libc::c_int as uint16_t,
            2881 as libc::c_int as uint16_t,
            1380 as libc::c_int as uint16_t,
            1786 as libc::c_int as uint16_t,
            1453 as libc::c_int as uint16_t,
        ],
        [
            452 as libc::c_int as uint16_t,
            899 as libc::c_int as uint16_t,
            1293 as libc::c_int as uint16_t,
            903 as libc::c_int as uint16_t,
            1311 as libc::c_int as uint16_t,
            3100 as libc::c_int as uint16_t,
            465 as libc::c_int as uint16_t,
            1311 as libc::c_int as uint16_t,
            1319 as libc::c_int as uint16_t,
            813 as libc::c_int as uint16_t,
        ],
        [
            394 as libc::c_int as uint16_t,
            927 as libc::c_int as uint16_t,
            942 as libc::c_int as uint16_t,
            1103 as libc::c_int as uint16_t,
            1358 as libc::c_int as uint16_t,
            1104 as libc::c_int as uint16_t,
            946 as libc::c_int as uint16_t,
            593 as libc::c_int as uint16_t,
            1363 as libc::c_int as uint16_t,
            1109 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            1005 as libc::c_int as uint16_t,
            1007 as libc::c_int as uint16_t,
            1016 as libc::c_int as uint16_t,
            658 as libc::c_int as uint16_t,
            1173 as libc::c_int as uint16_t,
            1021 as libc::c_int as uint16_t,
            1164 as libc::c_int as uint16_t,
            623 as libc::c_int as uint16_t,
            1028 as libc::c_int as uint16_t,
        ],
        [
            564 as libc::c_int as uint16_t,
            796 as libc::c_int as uint16_t,
            632 as libc::c_int as uint16_t,
            1005 as libc::c_int as uint16_t,
            1014 as libc::c_int as uint16_t,
            863 as libc::c_int as uint16_t,
            2316 as libc::c_int as uint16_t,
            1268 as libc::c_int as uint16_t,
            938 as libc::c_int as uint16_t,
            764 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            266 as libc::c_int as uint16_t,
            606 as libc::c_int as uint16_t,
            1098 as libc::c_int as uint16_t,
            1228 as libc::c_int as uint16_t,
            1497 as libc::c_int as uint16_t,
            1243 as libc::c_int as uint16_t,
            948 as libc::c_int as uint16_t,
            1030 as libc::c_int as uint16_t,
            1734 as libc::c_int as uint16_t,
            1461 as libc::c_int as uint16_t,
        ],
        [
            366 as libc::c_int as uint16_t,
            585 as libc::c_int as uint16_t,
            901 as libc::c_int as uint16_t,
            1060 as libc::c_int as uint16_t,
            1407 as libc::c_int as uint16_t,
            1247 as libc::c_int as uint16_t,
            876 as libc::c_int as uint16_t,
            1134 as libc::c_int as uint16_t,
            1620 as libc::c_int as uint16_t,
            1054 as libc::c_int as uint16_t,
        ],
        [
            452 as libc::c_int as uint16_t,
            565 as libc::c_int as uint16_t,
            542 as libc::c_int as uint16_t,
            1729 as libc::c_int as uint16_t,
            1479 as libc::c_int as uint16_t,
            1479 as libc::c_int as uint16_t,
            1016 as libc::c_int as uint16_t,
            886 as libc::c_int as uint16_t,
            2938 as libc::c_int as uint16_t,
            1150 as libc::c_int as uint16_t,
        ],
        [
            555 as libc::c_int as uint16_t,
            1088 as libc::c_int as uint16_t,
            1533 as libc::c_int as uint16_t,
            950 as libc::c_int as uint16_t,
            1354 as libc::c_int as uint16_t,
            895 as libc::c_int as uint16_t,
            834 as libc::c_int as uint16_t,
            1019 as libc::c_int as uint16_t,
            1021 as libc::c_int as uint16_t,
            496 as libc::c_int as uint16_t,
        ],
        [
            704 as libc::c_int as uint16_t,
            815 as libc::c_int as uint16_t,
            1193 as libc::c_int as uint16_t,
            971 as libc::c_int as uint16_t,
            973 as libc::c_int as uint16_t,
            640 as libc::c_int as uint16_t,
            1217 as libc::c_int as uint16_t,
            2214 as libc::c_int as uint16_t,
            832 as libc::c_int as uint16_t,
            578 as libc::c_int as uint16_t,
        ],
        [
            672 as libc::c_int as uint16_t,
            1245 as libc::c_int as uint16_t,
            579 as libc::c_int as uint16_t,
            871 as libc::c_int as uint16_t,
            875 as libc::c_int as uint16_t,
            774 as libc::c_int as uint16_t,
            872 as libc::c_int as uint16_t,
            1273 as libc::c_int as uint16_t,
            1027 as libc::c_int as uint16_t,
            949 as libc::c_int as uint16_t,
        ],
        [
            296 as libc::c_int as uint16_t,
            1134 as libc::c_int as uint16_t,
            2050 as libc::c_int as uint16_t,
            1784 as libc::c_int as uint16_t,
            1636 as libc::c_int as uint16_t,
            3425 as libc::c_int as uint16_t,
            442 as libc::c_int as uint16_t,
            1550 as libc::c_int as uint16_t,
            2076 as libc::c_int as uint16_t,
            722 as libc::c_int as uint16_t,
        ],
        [
            342 as libc::c_int as uint16_t,
            982 as libc::c_int as uint16_t,
            1259 as libc::c_int as uint16_t,
            1846 as libc::c_int as uint16_t,
            1848 as libc::c_int as uint16_t,
            1848 as libc::c_int as uint16_t,
            622 as libc::c_int as uint16_t,
            568 as libc::c_int as uint16_t,
            1847 as libc::c_int as uint16_t,
            1052 as libc::c_int as uint16_t,
        ],
        [
            555 as libc::c_int as uint16_t,
            1064 as libc::c_int as uint16_t,
            1304 as libc::c_int as uint16_t,
            828 as libc::c_int as uint16_t,
            746 as libc::c_int as uint16_t,
            1343 as libc::c_int as uint16_t,
            1075 as libc::c_int as uint16_t,
            1329 as libc::c_int as uint16_t,
            1078 as libc::c_int as uint16_t,
            494 as libc::c_int as uint16_t,
        ],
        [
            288 as libc::c_int as uint16_t,
            1167 as libc::c_int as uint16_t,
            1285 as libc::c_int as uint16_t,
            1174 as libc::c_int as uint16_t,
            1639 as libc::c_int as uint16_t,
            1639 as libc::c_int as uint16_t,
            833 as libc::c_int as uint16_t,
            2254 as libc::c_int as uint16_t,
            1304 as libc::c_int as uint16_t,
            509 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            342 as libc::c_int as uint16_t,
            719 as libc::c_int as uint16_t,
            767 as libc::c_int as uint16_t,
            1866 as libc::c_int as uint16_t,
            1757 as libc::c_int as uint16_t,
            1270 as libc::c_int as uint16_t,
            1246 as libc::c_int as uint16_t,
            550 as libc::c_int as uint16_t,
            1746 as libc::c_int as uint16_t,
            2151 as libc::c_int as uint16_t,
        ],
        [
            483 as libc::c_int as uint16_t,
            653 as libc::c_int as uint16_t,
            694 as libc::c_int as uint16_t,
            1509 as libc::c_int as uint16_t,
            1459 as libc::c_int as uint16_t,
            1410 as libc::c_int as uint16_t,
            1218 as libc::c_int as uint16_t,
            507 as libc::c_int as uint16_t,
            1914 as libc::c_int as uint16_t,
            1266 as libc::c_int as uint16_t,
        ],
        [
            488 as libc::c_int as uint16_t,
            757 as libc::c_int as uint16_t,
            447 as libc::c_int as uint16_t,
            2979 as libc::c_int as uint16_t,
            1813 as libc::c_int as uint16_t,
            1268 as libc::c_int as uint16_t,
            1654 as libc::c_int as uint16_t,
            539 as libc::c_int as uint16_t,
            1849 as libc::c_int as uint16_t,
            2109 as libc::c_int as uint16_t,
        ],
        [
            522 as libc::c_int as uint16_t,
            1097 as libc::c_int as uint16_t,
            1085 as libc::c_int as uint16_t,
            851 as libc::c_int as uint16_t,
            1365 as libc::c_int as uint16_t,
            1111 as libc::c_int as uint16_t,
            851 as libc::c_int as uint16_t,
            901 as libc::c_int as uint16_t,
            961 as libc::c_int as uint16_t,
            605 as libc::c_int as uint16_t,
        ],
        [
            709 as libc::c_int as uint16_t,
            716 as libc::c_int as uint16_t,
            841 as libc::c_int as uint16_t,
            728 as libc::c_int as uint16_t,
            736 as libc::c_int as uint16_t,
            945 as libc::c_int as uint16_t,
            941 as libc::c_int as uint16_t,
            862 as libc::c_int as uint16_t,
            2845 as libc::c_int as uint16_t,
            1057 as libc::c_int as uint16_t,
        ],
        [
            512 as libc::c_int as uint16_t,
            1323 as libc::c_int as uint16_t,
            500 as libc::c_int as uint16_t,
            1336 as libc::c_int as uint16_t,
            1083 as libc::c_int as uint16_t,
            681 as libc::c_int as uint16_t,
            1342 as libc::c_int as uint16_t,
            717 as libc::c_int as uint16_t,
            1604 as libc::c_int as uint16_t,
            1350 as libc::c_int as uint16_t,
        ],
        [
            452 as libc::c_int as uint16_t,
            1155 as libc::c_int as uint16_t,
            1372 as libc::c_int as uint16_t,
            1900 as libc::c_int as uint16_t,
            1501 as libc::c_int as uint16_t,
            3290 as libc::c_int as uint16_t,
            311 as libc::c_int as uint16_t,
            944 as libc::c_int as uint16_t,
            1919 as libc::c_int as uint16_t,
            922 as libc::c_int as uint16_t,
        ],
        [
            403 as libc::c_int as uint16_t,
            1520 as libc::c_int as uint16_t,
            977 as libc::c_int as uint16_t,
            2132 as libc::c_int as uint16_t,
            1733 as libc::c_int as uint16_t,
            3522 as libc::c_int as uint16_t,
            1076 as libc::c_int as uint16_t,
            276 as libc::c_int as uint16_t,
            3335 as libc::c_int as uint16_t,
            1547 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            1374 as libc::c_int as uint16_t,
            1101 as libc::c_int as uint16_t,
            615 as libc::c_int as uint16_t,
            673 as libc::c_int as uint16_t,
            2462 as libc::c_int as uint16_t,
            974 as libc::c_int as uint16_t,
            795 as libc::c_int as uint16_t,
            984 as libc::c_int as uint16_t,
            984 as libc::c_int as uint16_t,
        ],
        [
            547 as libc::c_int as uint16_t,
            1122 as libc::c_int as uint16_t,
            1062 as libc::c_int as uint16_t,
            812 as libc::c_int as uint16_t,
            1410 as libc::c_int as uint16_t,
            951 as libc::c_int as uint16_t,
            1140 as libc::c_int as uint16_t,
            622 as libc::c_int as uint16_t,
            1268 as libc::c_int as uint16_t,
            651 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            165 as libc::c_int as uint16_t,
            982 as libc::c_int as uint16_t,
            1235 as libc::c_int as uint16_t,
            938 as libc::c_int as uint16_t,
            1334 as libc::c_int as uint16_t,
            1366 as libc::c_int as uint16_t,
            1659 as libc::c_int as uint16_t,
            1578 as libc::c_int as uint16_t,
            964 as libc::c_int as uint16_t,
            1612 as libc::c_int as uint16_t,
        ],
        [
            592 as libc::c_int as uint16_t,
            422 as libc::c_int as uint16_t,
            925 as libc::c_int as uint16_t,
            847 as libc::c_int as uint16_t,
            1139 as libc::c_int as uint16_t,
            1112 as libc::c_int as uint16_t,
            1387 as libc::c_int as uint16_t,
            2036 as libc::c_int as uint16_t,
            861 as libc::c_int as uint16_t,
            1041 as libc::c_int as uint16_t,
        ],
        [
            403 as libc::c_int as uint16_t,
            837 as libc::c_int as uint16_t,
            732 as libc::c_int as uint16_t,
            770 as libc::c_int as uint16_t,
            941 as libc::c_int as uint16_t,
            1658 as libc::c_int as uint16_t,
            1250 as libc::c_int as uint16_t,
            809 as libc::c_int as uint16_t,
            1407 as libc::c_int as uint16_t,
            1407 as libc::c_int as uint16_t,
        ],
        [
            896 as libc::c_int as uint16_t,
            874 as libc::c_int as uint16_t,
            1071 as libc::c_int as uint16_t,
            381 as libc::c_int as uint16_t,
            1568 as libc::c_int as uint16_t,
            1722 as libc::c_int as uint16_t,
            1437 as libc::c_int as uint16_t,
            2192 as libc::c_int as uint16_t,
            480 as libc::c_int as uint16_t,
            1035 as libc::c_int as uint16_t,
        ],
        [
            640 as libc::c_int as uint16_t,
            1098 as libc::c_int as uint16_t,
            1012 as libc::c_int as uint16_t,
            1032 as libc::c_int as uint16_t,
            684 as libc::c_int as uint16_t,
            1382 as libc::c_int as uint16_t,
            1581 as libc::c_int as uint16_t,
            2106 as libc::c_int as uint16_t,
            416 as libc::c_int as uint16_t,
            865 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            1005 as libc::c_int as uint16_t,
            819 as libc::c_int as uint16_t,
            914 as libc::c_int as uint16_t,
            710 as libc::c_int as uint16_t,
            770 as libc::c_int as uint16_t,
            1418 as libc::c_int as uint16_t,
            920 as libc::c_int as uint16_t,
            838 as libc::c_int as uint16_t,
            1435 as libc::c_int as uint16_t,
        ],
        [
            415 as libc::c_int as uint16_t,
            1258 as libc::c_int as uint16_t,
            1245 as libc::c_int as uint16_t,
            870 as libc::c_int as uint16_t,
            1278 as libc::c_int as uint16_t,
            3067 as libc::c_int as uint16_t,
            770 as libc::c_int as uint16_t,
            1021 as libc::c_int as uint16_t,
            1287 as libc::c_int as uint16_t,
            522 as libc::c_int as uint16_t,
        ],
        [
            406 as libc::c_int as uint16_t,
            990 as libc::c_int as uint16_t,
            601 as libc::c_int as uint16_t,
            1009 as libc::c_int as uint16_t,
            1265 as libc::c_int as uint16_t,
            1265 as libc::c_int as uint16_t,
            1267 as libc::c_int as uint16_t,
            759 as libc::c_int as uint16_t,
            1017 as libc::c_int as uint16_t,
            1277 as libc::c_int as uint16_t,
        ],
        [
            968 as libc::c_int as uint16_t,
            1182 as libc::c_int as uint16_t,
            1329 as libc::c_int as uint16_t,
            788 as libc::c_int as uint16_t,
            1032 as libc::c_int as uint16_t,
            1292 as libc::c_int as uint16_t,
            1705 as libc::c_int as uint16_t,
            1714 as libc::c_int as uint16_t,
            203 as libc::c_int as uint16_t,
            1403 as libc::c_int as uint16_t,
        ],
        [
            732 as libc::c_int as uint16_t,
            877 as libc::c_int as uint16_t,
            1279 as libc::c_int as uint16_t,
            471 as libc::c_int as uint16_t,
            901 as libc::c_int as uint16_t,
            1161 as libc::c_int as uint16_t,
            1545 as libc::c_int as uint16_t,
            1294 as libc::c_int as uint16_t,
            755 as libc::c_int as uint16_t,
            755 as libc::c_int as uint16_t,
        ],
    ],
    [
        [
            111 as libc::c_int as uint16_t,
            931 as libc::c_int as uint16_t,
            1378 as libc::c_int as uint16_t,
            1185 as libc::c_int as uint16_t,
            1933 as libc::c_int as uint16_t,
            1648 as libc::c_int as uint16_t,
            1148 as libc::c_int as uint16_t,
            1714 as libc::c_int as uint16_t,
            1873 as libc::c_int as uint16_t,
            1307 as libc::c_int as uint16_t,
        ],
        [
            406 as libc::c_int as uint16_t,
            414 as libc::c_int as uint16_t,
            1030 as libc::c_int as uint16_t,
            1023 as libc::c_int as uint16_t,
            1910 as libc::c_int as uint16_t,
            1404 as libc::c_int as uint16_t,
            1313 as libc::c_int as uint16_t,
            1647 as libc::c_int as uint16_t,
            1509 as libc::c_int as uint16_t,
            793 as libc::c_int as uint16_t,
        ],
        [
            342 as libc::c_int as uint16_t,
            640 as libc::c_int as uint16_t,
            575 as libc::c_int as uint16_t,
            1088 as libc::c_int as uint16_t,
            1241 as libc::c_int as uint16_t,
            1349 as libc::c_int as uint16_t,
            1161 as libc::c_int as uint16_t,
            1350 as libc::c_int as uint16_t,
            1756 as libc::c_int as uint16_t,
            1502 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            766 as libc::c_int as uint16_t,
            1185 as libc::c_int as uint16_t,
            357 as libc::c_int as uint16_t,
            1682 as libc::c_int as uint16_t,
            1428 as libc::c_int as uint16_t,
            1329 as libc::c_int as uint16_t,
            1897 as libc::c_int as uint16_t,
            1219 as libc::c_int as uint16_t,
            802 as libc::c_int as uint16_t,
        ],
        [
            473 as libc::c_int as uint16_t,
            909 as libc::c_int as uint16_t,
            1164 as libc::c_int as uint16_t,
            771 as libc::c_int as uint16_t,
            719 as libc::c_int as uint16_t,
            2508 as libc::c_int as uint16_t,
            1427 as libc::c_int as uint16_t,
            1432 as libc::c_int as uint16_t,
            722 as libc::c_int as uint16_t,
            782 as libc::c_int as uint16_t,
        ],
        [
            342 as libc::c_int as uint16_t,
            892 as libc::c_int as uint16_t,
            785 as libc::c_int as uint16_t,
            1145 as libc::c_int as uint16_t,
            1150 as libc::c_int as uint16_t,
            794 as libc::c_int as uint16_t,
            1296 as libc::c_int as uint16_t,
            1550 as libc::c_int as uint16_t,
            973 as libc::c_int as uint16_t,
            1057 as libc::c_int as uint16_t,
        ],
        [
            208 as libc::c_int as uint16_t,
            1036 as libc::c_int as uint16_t,
            1326 as libc::c_int as uint16_t,
            1343 as libc::c_int as uint16_t,
            1606 as libc::c_int as uint16_t,
            3395 as libc::c_int as uint16_t,
            815 as libc::c_int as uint16_t,
            1455 as libc::c_int as uint16_t,
            1618 as libc::c_int as uint16_t,
            712 as libc::c_int as uint16_t,
        ],
        [
            228 as libc::c_int as uint16_t,
            928 as libc::c_int as uint16_t,
            890 as libc::c_int as uint16_t,
            1046 as libc::c_int as uint16_t,
            3499 as libc::c_int as uint16_t,
            1711 as libc::c_int as uint16_t,
            994 as libc::c_int as uint16_t,
            829 as libc::c_int as uint16_t,
            1720 as libc::c_int as uint16_t,
            1318 as libc::c_int as uint16_t,
        ],
        [
            768 as libc::c_int as uint16_t,
            724 as libc::c_int as uint16_t,
            1058 as libc::c_int as uint16_t,
            636 as libc::c_int as uint16_t,
            991 as libc::c_int as uint16_t,
            1075 as libc::c_int as uint16_t,
            1319 as libc::c_int as uint16_t,
            1324 as libc::c_int as uint16_t,
            616 as libc::c_int as uint16_t,
            825 as libc::c_int as uint16_t,
        ],
        [
            305 as libc::c_int as uint16_t,
            1167 as libc::c_int as uint16_t,
            1358 as libc::c_int as uint16_t,
            899 as libc::c_int as uint16_t,
            1587 as libc::c_int as uint16_t,
            1587 as libc::c_int as uint16_t,
            987 as libc::c_int as uint16_t,
            1988 as libc::c_int as uint16_t,
            1332 as libc::c_int as uint16_t,
            501 as libc::c_int as uint16_t,
        ],
    ],
];
#[no_mangle]
pub unsafe extern "C" fn VP8InitResidual(
    mut first: libc::c_int,
    mut coeff_type: libc::c_int,
    enc: *mut VP8Encoder,
    res: *mut VP8Residual,
) {
    (*res).coeff_type = coeff_type;
    (*res).prob = ((*enc).proba_.coeffs_[coeff_type as usize]).as_mut_ptr();
    (*res).stats = ((*enc).proba_.stats_[coeff_type as usize]).as_mut_ptr();
    (*res).costs = ((*enc).proba_.remapped_costs_[coeff_type as usize]).as_mut_ptr();
    (*res).first = first;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetCostLuma4(
    it: *mut VP8EncIterator,
    mut levels: *const int16_t,
) -> libc::c_int {
    let x: libc::c_int = (*it).i4_ & 3 as libc::c_int;
    let y: libc::c_int = (*it).i4_ >> 2 as libc::c_int;
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: 0 as *const int16_t,
        coeff_type: 0,
        prob: 0 as *mut ProbaArray,
        stats: 0 as *mut StatsArray,
        costs: 0 as *mut [*const uint16_t; 3],
    };
    let enc: *mut VP8Encoder = (*it).enc_;
    let mut R: libc::c_int = 0 as libc::c_int;
    let mut ctx: libc::c_int = 0;
    VP8InitResidual(0 as libc::c_int, 3 as libc::c_int, enc, &mut res);
    ctx = (*it).top_nz_[x as usize] + (*it).left_nz_[y as usize];
    VP8SetResidualCoeffs.expect("non-null function pointer")(levels, &mut res);
    R += VP8GetResidualCost.expect("non-null function pointer")(ctx, &mut res);
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetCostLuma16(
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
) -> libc::c_int {
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: 0 as *const int16_t,
        coeff_type: 0,
        prob: 0 as *mut ProbaArray,
        stats: 0 as *mut StatsArray,
        costs: 0 as *mut [*const uint16_t; 3],
    };
    let enc: *mut VP8Encoder = (*it).enc_;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut R: libc::c_int = 0 as libc::c_int;
    VP8IteratorNzToBytes(it);
    VP8InitResidual(0 as libc::c_int, 1 as libc::c_int, enc, &mut res);
    VP8SetResidualCoeffs
        .expect("non-null function pointer")(((*rd).y_dc_levels).as_ptr(), &mut res);
    R
        += VP8GetResidualCost
            .expect(
                "non-null function pointer",
            )(
            (*it).top_nz_[8 as libc::c_int as usize]
                + (*it).left_nz_[8 as libc::c_int as usize],
            &mut res,
        );
    VP8InitResidual(1 as libc::c_int, 0 as libc::c_int, enc, &mut res);
    y = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            let ctx: libc::c_int = (*it).top_nz_[x as usize]
                + (*it).left_nz_[y as usize];
            VP8SetResidualCoeffs
                .expect(
                    "non-null function pointer",
                )(
                ((*rd).y_ac_levels[(x + y * 4 as libc::c_int) as usize]).as_ptr(),
                &mut res,
            );
            R += VP8GetResidualCost.expect("non-null function pointer")(ctx, &mut res);
            (*it).left_nz_[y as usize] = (res.last >= 0 as libc::c_int) as libc::c_int;
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetCostUV(
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
) -> libc::c_int {
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: 0 as *const int16_t,
        coeff_type: 0,
        prob: 0 as *mut ProbaArray,
        stats: 0 as *mut StatsArray,
        costs: 0 as *mut [*const uint16_t; 3],
    };
    let enc: *mut VP8Encoder = (*it).enc_;
    let mut ch: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut R: libc::c_int = 0 as libc::c_int;
    VP8IteratorNzToBytes(it);
    VP8InitResidual(0 as libc::c_int, 2 as libc::c_int, enc, &mut res);
    ch = 0 as libc::c_int;
    while ch <= 2 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 2 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 2 as libc::c_int {
                let ctx: libc::c_int = (*it)
                    .top_nz_[(4 as libc::c_int + ch + x) as usize]
                    + (*it).left_nz_[(4 as libc::c_int + ch + y) as usize];
                VP8SetResidualCoeffs
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*rd)
                        .uv_levels[(ch * 2 as libc::c_int + x + y * 2 as libc::c_int)
                        as usize])
                        .as_ptr(),
                    &mut res,
                );
                R
                    += VP8GetResidualCost
                        .expect("non-null function pointer")(ctx, &mut res);
                (*it)
                    .left_nz_[(4 as libc::c_int + ch + y)
                    as usize] = (res.last >= 0 as libc::c_int) as libc::c_int;
                (*it)
                    .top_nz_[(4 as libc::c_int + ch + x)
                    as usize] = (*it).left_nz_[(4 as libc::c_int + ch + y) as usize];
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        ch += 2 as libc::c_int;
    }
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn VP8RecordCoeffs(
    mut ctx: libc::c_int,
    res: *const VP8Residual,
) -> libc::c_int {
    let mut n: libc::c_int = (*res).first;
    let mut s: *mut proba_t = ((*((*res).stats).offset(n as isize))[ctx as usize])
        .as_mut_ptr();
    if (*res).last < 0 as libc::c_int {
        VP8RecordStats(0 as libc::c_int, s.offset(0 as libc::c_int as isize));
        return 0 as libc::c_int;
    }
    while n <= (*res).last {
        let mut v: libc::c_int = 0;
        VP8RecordStats(1 as libc::c_int, s.offset(0 as libc::c_int as isize));
        loop {
            let fresh0 = n;
            n = n + 1;
            v = *((*res).coeffs).offset(fresh0 as isize) as libc::c_int;
            if !(v == 0 as libc::c_int) {
                break;
            }
            VP8RecordStats(0 as libc::c_int, s.offset(1 as libc::c_int as isize));
            s = ((*((*res).stats)
                .offset(VP8EncBands[n as usize] as isize))[0 as libc::c_int as usize])
                .as_mut_ptr();
        }
        VP8RecordStats(1 as libc::c_int, s.offset(1 as libc::c_int as isize));
        if VP8RecordStats(
            ((2 as libc::c_uint) < (v + 1 as libc::c_int) as libc::c_uint)
                as libc::c_int,
            s.offset(2 as libc::c_int as isize),
        ) == 0
        {
            s = ((*((*res).stats)
                .offset(VP8EncBands[n as usize] as isize))[1 as libc::c_int as usize])
                .as_mut_ptr();
        } else {
            v = abs(v);
            if v > MAX_VARIABLE_LEVEL as libc::c_int {
                v = MAX_VARIABLE_LEVEL as libc::c_int;
            }
            let bits: libc::c_int = VP8LevelCodes[(v - 1 as libc::c_int)
                as usize][1 as libc::c_int as usize] as libc::c_int;
            let mut pattern: libc::c_int = VP8LevelCodes[(v - 1 as libc::c_int)
                as usize][0 as libc::c_int as usize] as libc::c_int;
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            loop {
                pattern >>= 1 as libc::c_int;
                if !(pattern != 0 as libc::c_int) {
                    break;
                }
                let mask: libc::c_int = (2 as libc::c_int) << i;
                if pattern & 1 as libc::c_int != 0 {
                    VP8RecordStats(
                        (bits & mask != 0) as libc::c_int,
                        s.offset(3 as libc::c_int as isize).offset(i as isize),
                    );
                }
                i += 1;
                i;
            }
            s = ((*((*res).stats)
                .offset(VP8EncBands[n as usize] as isize))[2 as libc::c_int as usize])
                .as_mut_ptr();
        }
    }
    if n < 16 as libc::c_int {
        VP8RecordStats(0 as libc::c_int, s.offset(0 as libc::c_int as isize));
    }
    return 1 as libc::c_int;
}
