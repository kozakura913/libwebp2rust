use ::libc;
use ::c2rust_bitfields;

use super::token_enc::VP8Tokens;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn VP8SetSegmentParams(enc: *mut VP8Encoder, quality: libc::c_float);
    fn VP8RecordCoeffs(ctx: libc::c_int, res: *const VP8Residual) -> libc::c_int;
    fn VP8CalculateLevelCosts(proba: *mut VP8EncProba);
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8InitFilter(it: *mut VP8EncIterator);
    fn VP8Decimate(
        it: *mut VP8EncIterator,
        rd: *mut VP8ModeScore,
        rd_opt: VP8RDLevel,
    ) -> libc::c_int;
    fn VP8InitResidual(
        first: libc::c_int,
        coeff_type: libc::c_int,
        enc: *mut VP8Encoder,
        res: *mut VP8Residual,
    );
    fn VP8StoreFilterStats(it: *mut VP8EncIterator);
    fn VP8AdjustFilterStrength(it: *mut VP8EncIterator);
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn VP8EncFreeBitWriters(enc: *mut VP8Encoder);
    static VP8CoeffsUpdateProba: [[[[uint8_t; 11]; 3]; 8]; 4];
    static VP8CoeffsProba0: [[[[uint8_t; 11]; 3]; 8]; 4];
    fn VP8EstimateTokenSize(b: *mut VP8TBuffer, probas: *const uint8_t) -> size_t;
    fn VP8RecordCoeffTokens(
        ctx: libc::c_int,
        res: *const VP8Residual,
        tokens: *mut VP8TBuffer,
    ) -> libc::c_int;
    fn VP8EmitTokens(
        b: *mut VP8TBuffer,
        bw: *mut VP8BitWriter,
        probas: *const uint8_t,
        final_pass: libc::c_int,
    ) -> libc::c_int;
    fn VP8TBufferClear(b: *mut VP8TBuffer);
    fn VP8IteratorBytesToNz(it: *mut VP8EncIterator);
    fn VP8IteratorNzToBytes(it: *mut VP8EncIterator);
    fn VP8IteratorProgress(it: *const VP8EncIterator, delta: libc::c_int) -> libc::c_int;
    fn VP8IteratorSaveBoundary(it: *mut VP8EncIterator);
    fn VP8IteratorNext(it: *mut VP8EncIterator) -> libc::c_int;
    fn VP8IteratorExport(it: *const VP8EncIterator);
    fn VP8IteratorImport(it: *mut VP8EncIterator, tmp_32: *mut uint8_t);
    fn VP8IteratorInit(enc: *mut VP8Encoder, it: *mut VP8EncIterator);
    fn VP8BitWriterInit(bw: *mut VP8BitWriter, expected_size: size_t) -> libc::c_int;
    fn VP8BitWriterFinish(bw: *mut VP8BitWriter) -> *mut uint8_t;
    fn VP8PutBit(
        bw: *mut VP8BitWriter,
        bit: libc::c_int,
        prob: libc::c_int,
    ) -> libc::c_int;
    fn VP8PutBitUniform(bw: *mut VP8BitWriter, bit: libc::c_int) -> libc::c_int;
    static mut VP8SSE16x16: VP8Metric;
    static mut VP8SSE8x8: VP8Metric;
    static VP8EntropyCost: [uint16_t; 256];
    static VP8EncBands: [uint8_t; 17];
    static mut VP8SetResidualCoeffs: VP8SetResidualCoeffsFunc;
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
pub type VP8Metric = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t) -> libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PassStats {
    pub is_first: libc::c_int,
    pub dq: libc::c_float,
    pub q: libc::c_float,
    pub last_q: libc::c_float,
    pub qmin: libc::c_float,
    pub qmax: libc::c_float,
    pub value: libc::c_double,
    pub last_value: libc::c_double,
    pub target: libc::c_double,
    pub do_size_search: libc::c_int,
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
unsafe extern "C" fn VP8BitWriterPos(bw: *const VP8BitWriter) -> uint64_t {
    let nb_bits: uint64_t = (8 as libc::c_int + (*bw).nb_bits_) as uint64_t;
    return ((*bw).pos_)
        .wrapping_add((*bw).run_ as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add(nb_bits);
}
unsafe extern "C" fn Clamp(
    mut v: libc::c_float,
    mut min: libc::c_float,
    mut max: libc::c_float,
) -> libc::c_float {
    return if v < min { min } else if v > max { max } else { v };
}
unsafe extern "C" fn InitPassStats(
    enc: *const VP8Encoder,
    s: *mut PassStats,
) -> libc::c_int {
    let target_size: uint64_t = (*(*enc).config_).target_size as uint64_t;
    let do_size_search: libc::c_int = (target_size != 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let target_PSNR: libc::c_float = (*(*enc).config_).target_PSNR;
    (*s).is_first = 1 as libc::c_int;
    (*s).dq = 10.0f32;
    (*s).qmin = 1.0f32 * (*(*enc).config_).qmin as libc::c_float;
    (*s).qmax = 1.0f32 * (*(*enc).config_).qmax as libc::c_float;
    (*s).last_q = Clamp((*(*enc).config_).quality, (*s).qmin, (*s).qmax);
    (*s).q = (*s).last_q;
    (*s)
        .target = if do_size_search != 0 {
        target_size as libc::c_double
    } else if target_PSNR as libc::c_double > 0.0f64 {
        target_PSNR as libc::c_double
    } else {
        40.0f64
    };
    (*s).last_value = 0.0f64;
    (*s).value = (*s).last_value;
    (*s).do_size_search = do_size_search;
    return do_size_search;
}
unsafe extern "C" fn ComputeNextQ(s: *mut PassStats) -> libc::c_float {
    let mut dq: libc::c_float = 0.;
    if (*s).is_first != 0 {
        dq = if (*s).value > (*s).target { -(*s).dq } else { (*s).dq };
        (*s).is_first = 0 as libc::c_int;
    } else if (*s).value != (*s).last_value {
        let slope: libc::c_double = ((*s).target - (*s).value)
            / ((*s).last_value - (*s).value);
        dq = (slope * ((*s).last_q - (*s).q) as libc::c_double) as libc::c_float;
    } else {
        dq = 0.0f64 as libc::c_float;
    }
    (*s).dq = Clamp(dq, -30.0f32, 30.0f32);
    (*s).last_q = (*s).q;
    (*s).last_value = (*s).value;
    (*s).q = Clamp((*s).q + (*s).dq, (*s).qmin, (*s).qmax);
    return (*s).q;
}
#[no_mangle]
pub static mut VP8Cat3: [uint8_t; 3] = [
    173 as libc::c_int as uint8_t,
    148 as libc::c_int as uint8_t,
    140 as libc::c_int as uint8_t,
];
#[no_mangle]
pub static mut VP8Cat4: [uint8_t; 4] = [
    176 as libc::c_int as uint8_t,
    155 as libc::c_int as uint8_t,
    140 as libc::c_int as uint8_t,
    135 as libc::c_int as uint8_t,
];
#[no_mangle]
pub static mut VP8Cat5: [uint8_t; 5] = [
    180 as libc::c_int as uint8_t,
    157 as libc::c_int as uint8_t,
    141 as libc::c_int as uint8_t,
    134 as libc::c_int as uint8_t,
    130 as libc::c_int as uint8_t,
];
#[no_mangle]
pub static mut VP8Cat6: [uint8_t; 11] = [
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
];
unsafe extern "C" fn ResetStats(enc: *mut VP8Encoder) {
    let proba: *mut VP8EncProba = &mut (*enc).proba_;
    VP8CalculateLevelCosts(proba);
    (*proba).nb_skip_ = 0 as libc::c_int;
}
unsafe extern "C" fn CalcSkipProba(
    mut nb: uint64_t,
    mut total: uint64_t,
) -> libc::c_int {
    return (if total != 0 {
        total
            .wrapping_sub(nb)
            .wrapping_mul(255 as libc::c_int as libc::c_ulong)
            .wrapping_div(total)
    } else {
        255 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
}
unsafe extern "C" fn FinalizeSkipProba(enc: *mut VP8Encoder) -> libc::c_int {
    let proba: *mut VP8EncProba = &mut (*enc).proba_;
    let nb_mbs: libc::c_int = (*enc).mb_w_ * (*enc).mb_h_;
    let nb_events: libc::c_int = (*proba).nb_skip_;
    let mut size: libc::c_int = 0;
    (*proba)
        .skip_proba_ = CalcSkipProba(nb_events as uint64_t, nb_mbs as uint64_t)
        as uint8_t;
    (*proba)
        .use_skip_proba_ = (((*proba).skip_proba_ as libc::c_int) < 250 as libc::c_int)
        as libc::c_int;
    size = 256 as libc::c_int;
    if (*proba).use_skip_proba_ != 0 {
        size
            += nb_events * VP8BitCost(1 as libc::c_int, (*proba).skip_proba_)
                + (nb_mbs - nb_events)
                    * VP8BitCost(0 as libc::c_int, (*proba).skip_proba_);
        size += 8 as libc::c_int * 256 as libc::c_int;
    }
    return size;
}
unsafe extern "C" fn CalcTokenProba(
    mut nb: libc::c_int,
    mut total: libc::c_int,
) -> libc::c_int {
    return if nb != 0 {
        255 as libc::c_int - nb * 255 as libc::c_int / total
    } else {
        255 as libc::c_int
    };
}
unsafe extern "C" fn BranchCost(
    mut nb: libc::c_int,
    mut total: libc::c_int,
    mut proba: libc::c_int,
) -> libc::c_int {
    return nb * VP8BitCost(1 as libc::c_int, proba as uint8_t)
        + (total - nb) * VP8BitCost(0 as libc::c_int, proba as uint8_t);
}
unsafe extern "C" fn ResetTokenStats(enc: *mut VP8Encoder) {
    let proba: *mut VP8EncProba = &mut (*enc).proba_;
    memset(
        ((*proba).stats_).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[StatsArray; 8]; 4]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn FinalizeTokenProbas(proba: *mut VP8EncProba) -> libc::c_int {
    let mut has_changed: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut t: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    t = 0 as libc::c_int;
    while t < NUM_TYPES as libc::c_int {
        b = 0 as libc::c_int;
        while b < NUM_BANDS as libc::c_int {
            c = 0 as libc::c_int;
            while c < NUM_CTX as libc::c_int {
                p = 0 as libc::c_int;
                while p < NUM_PROBAS as libc::c_int {
                    let stats: proba_t = (*proba)
                        .stats_[t as usize][b as usize][c as usize][p as usize];
                    let nb: libc::c_int = (stats >> 0 as libc::c_int
                        & 0xffff as libc::c_int as libc::c_uint) as libc::c_int;
                    let total: libc::c_int = (stats >> 16 as libc::c_int
                        & 0xffff as libc::c_int as libc::c_uint) as libc::c_int;
                    let update_proba: libc::c_int = VP8CoeffsUpdateProba[t
                        as usize][b as usize][c as usize][p as usize] as libc::c_int;
                    let old_p: libc::c_int = VP8CoeffsProba0[t
                        as usize][b as usize][c as usize][p as usize] as libc::c_int;
                    let new_p: libc::c_int = CalcTokenProba(nb, total);
                    let old_cost: libc::c_int = BranchCost(nb, total, old_p)
                        + VP8BitCost(0 as libc::c_int, update_proba as uint8_t);
                    let new_cost: libc::c_int = BranchCost(nb, total, new_p)
                        + VP8BitCost(1 as libc::c_int, update_proba as uint8_t)
                        + 8 as libc::c_int * 256 as libc::c_int;
                    let use_new_p: libc::c_int = (old_cost > new_cost) as libc::c_int;
                    size += VP8BitCost(use_new_p, update_proba as uint8_t);
                    if use_new_p != 0 {
                        (*proba)
                            .coeffs_[t
                            as usize][b
                            as usize][c as usize][p as usize] = new_p as uint8_t;
                        has_changed |= (new_p != old_p) as libc::c_int;
                        size += 8 as libc::c_int * 256 as libc::c_int;
                    } else {
                        (*proba)
                            .coeffs_[t
                            as usize][b
                            as usize][c as usize][p as usize] = old_p as uint8_t;
                    }
                    p += 1;
                    p;
                }
                c += 1;
                c;
            }
            b += 1;
            b;
        }
        t += 1;
        t;
    }
    (*proba).dirty_ = has_changed;
    return size;
}
unsafe extern "C" fn GetProba(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    let total: libc::c_int = a + b;
    return if total == 0 as libc::c_int {
        255 as libc::c_int
    } else {
        (255 as libc::c_int * a + total / 2 as libc::c_int) / total
    };
}
unsafe extern "C" fn ResetSegments(enc: *mut VP8Encoder) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        let ref mut fresh0 = *((*enc).mb_info_).offset(n as isize);
        (*fresh0).set_segment_(0 as libc::c_int as libc::c_uint);
        n += 1;
        n;
    }
}
unsafe extern "C" fn SetSegmentProbas(enc: *mut VP8Encoder) {
    let mut p: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        let mb: *const VP8MBInfo = &mut *((*enc).mb_info_).offset(n as isize)
            as *mut VP8MBInfo;
        p[(*mb).segment_() as usize] += 1;
        p[(*mb).segment_() as usize];
        n += 1;
        n;
    }
    if !((*(*enc).pic_).stats).is_null() {
        n = 0 as libc::c_int;
        while n < NUM_MB_SEGMENTS as libc::c_int {
            (*(*(*enc).pic_).stats).segment_size[n as usize] = p[n as usize];
            n += 1;
            n;
        }
    }
    if (*enc).segment_hdr_.num_segments_ > 1 as libc::c_int {
        let probas: *mut uint8_t = ((*enc).proba_.segments_).as_mut_ptr();
        *probas
            .offset(
                0 as libc::c_int as isize,
            ) = GetProba(
            p[0 as libc::c_int as usize] + p[1 as libc::c_int as usize],
            p[2 as libc::c_int as usize] + p[3 as libc::c_int as usize],
        ) as uint8_t;
        *probas
            .offset(
                1 as libc::c_int as isize,
            ) = GetProba(p[0 as libc::c_int as usize], p[1 as libc::c_int as usize])
            as uint8_t;
        *probas
            .offset(
                2 as libc::c_int as isize,
            ) = GetProba(p[2 as libc::c_int as usize], p[3 as libc::c_int as usize])
            as uint8_t;
        (*enc)
            .segment_hdr_
            .update_map_ = (*probas.offset(0 as libc::c_int as isize) as libc::c_int
            != 255 as libc::c_int
            || *probas.offset(1 as libc::c_int as isize) as libc::c_int
                != 255 as libc::c_int
            || *probas.offset(2 as libc::c_int as isize) as libc::c_int
                != 255 as libc::c_int) as libc::c_int;
        if (*enc).segment_hdr_.update_map_ == 0 {
            ResetSegments(enc);
        }
        (*enc)
            .segment_hdr_
            .size_ = p[0 as libc::c_int as usize]
            * (VP8BitCost(0 as libc::c_int, *probas.offset(0 as libc::c_int as isize))
                + VP8BitCost(
                    0 as libc::c_int,
                    *probas.offset(1 as libc::c_int as isize),
                ))
            + p[1 as libc::c_int as usize]
                * (VP8BitCost(
                    0 as libc::c_int,
                    *probas.offset(0 as libc::c_int as isize),
                )
                    + VP8BitCost(
                        1 as libc::c_int,
                        *probas.offset(1 as libc::c_int as isize),
                    ))
            + p[2 as libc::c_int as usize]
                * (VP8BitCost(
                    1 as libc::c_int,
                    *probas.offset(0 as libc::c_int as isize),
                )
                    + VP8BitCost(
                        0 as libc::c_int,
                        *probas.offset(2 as libc::c_int as isize),
                    ))
            + p[3 as libc::c_int as usize]
                * (VP8BitCost(
                    1 as libc::c_int,
                    *probas.offset(0 as libc::c_int as isize),
                )
                    + VP8BitCost(
                        1 as libc::c_int,
                        *probas.offset(2 as libc::c_int as isize),
                    ));
    } else {
        (*enc).segment_hdr_.update_map_ = 0 as libc::c_int;
        (*enc).segment_hdr_.size_ = 0 as libc::c_int;
    };
}
unsafe extern "C" fn PutCoeffs(
    bw: *mut VP8BitWriter,
    mut ctx: libc::c_int,
    mut res: *const VP8Residual,
) -> libc::c_int {
    let mut n: libc::c_int = (*res).first;
    let mut p: *const uint8_t = ((*((*res).prob).offset(n as isize))[ctx as usize])
        .as_mut_ptr();
    if VP8PutBit(
        bw,
        ((*res).last >= 0 as libc::c_int) as libc::c_int,
        *p.offset(0 as libc::c_int as isize) as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    while n < 16 as libc::c_int {
        let fresh1 = n;
        n = n + 1;
        let c: libc::c_int = *((*res).coeffs).offset(fresh1 as isize) as libc::c_int;
        let sign: libc::c_int = (c < 0 as libc::c_int) as libc::c_int;
        let mut v: libc::c_int = if sign != 0 { -c } else { c };
        if VP8PutBit(
            bw,
            (v != 0 as libc::c_int) as libc::c_int,
            *p.offset(1 as libc::c_int as isize) as libc::c_int,
        ) == 0
        {
            p = ((*((*res).prob)
                .offset(VP8EncBands[n as usize] as isize))[0 as libc::c_int as usize])
                .as_mut_ptr();
        } else {
            if VP8PutBit(
                bw,
                (v > 1 as libc::c_int) as libc::c_int,
                *p.offset(2 as libc::c_int as isize) as libc::c_int,
            ) == 0
            {
                p = ((*((*res).prob)
                    .offset(
                        VP8EncBands[n as usize] as isize,
                    ))[1 as libc::c_int as usize])
                    .as_mut_ptr();
            } else {
                if VP8PutBit(
                    bw,
                    (v > 4 as libc::c_int) as libc::c_int,
                    *p.offset(3 as libc::c_int as isize) as libc::c_int,
                ) == 0
                {
                    if VP8PutBit(
                        bw,
                        (v != 2 as libc::c_int) as libc::c_int,
                        *p.offset(4 as libc::c_int as isize) as libc::c_int,
                    ) != 0
                    {
                        VP8PutBit(
                            bw,
                            (v == 4 as libc::c_int) as libc::c_int,
                            *p.offset(5 as libc::c_int as isize) as libc::c_int,
                        );
                    }
                } else if VP8PutBit(
                    bw,
                    (v > 10 as libc::c_int) as libc::c_int,
                    *p.offset(6 as libc::c_int as isize) as libc::c_int,
                ) == 0
                {
                    if VP8PutBit(
                        bw,
                        (v > 6 as libc::c_int) as libc::c_int,
                        *p.offset(7 as libc::c_int as isize) as libc::c_int,
                    ) == 0
                    {
                        VP8PutBit(
                            bw,
                            (v == 6 as libc::c_int) as libc::c_int,
                            159 as libc::c_int,
                        );
                    } else {
                        VP8PutBit(
                            bw,
                            (v >= 9 as libc::c_int) as libc::c_int,
                            165 as libc::c_int,
                        );
                        VP8PutBit(
                            bw,
                            (v & 1 as libc::c_int == 0) as libc::c_int,
                            145 as libc::c_int,
                        );
                    }
                } else {
                    let mut mask: libc::c_int = 0;
                    let mut tab: *const uint8_t = 0 as *const uint8_t;
                    if v < 3 as libc::c_int + ((8 as libc::c_int) << 1 as libc::c_int) {
                        VP8PutBit(
                            bw,
                            0 as libc::c_int,
                            *p.offset(8 as libc::c_int as isize) as libc::c_int,
                        );
                        VP8PutBit(
                            bw,
                            0 as libc::c_int,
                            *p.offset(9 as libc::c_int as isize) as libc::c_int,
                        );
                        v -= 3 as libc::c_int + ((8 as libc::c_int) << 0 as libc::c_int);
                        mask = (1 as libc::c_int) << 2 as libc::c_int;
                        tab = VP8Cat3.as_ptr();
                    } else if v
                        < 3 as libc::c_int + ((8 as libc::c_int) << 2 as libc::c_int)
                    {
                        VP8PutBit(
                            bw,
                            0 as libc::c_int,
                            *p.offset(8 as libc::c_int as isize) as libc::c_int,
                        );
                        VP8PutBit(
                            bw,
                            1 as libc::c_int,
                            *p.offset(9 as libc::c_int as isize) as libc::c_int,
                        );
                        v -= 3 as libc::c_int + ((8 as libc::c_int) << 1 as libc::c_int);
                        mask = (1 as libc::c_int) << 3 as libc::c_int;
                        tab = VP8Cat4.as_ptr();
                    } else if v
                        < 3 as libc::c_int + ((8 as libc::c_int) << 3 as libc::c_int)
                    {
                        VP8PutBit(
                            bw,
                            1 as libc::c_int,
                            *p.offset(8 as libc::c_int as isize) as libc::c_int,
                        );
                        VP8PutBit(
                            bw,
                            0 as libc::c_int,
                            *p.offset(10 as libc::c_int as isize) as libc::c_int,
                        );
                        v -= 3 as libc::c_int + ((8 as libc::c_int) << 2 as libc::c_int);
                        mask = (1 as libc::c_int) << 4 as libc::c_int;
                        tab = VP8Cat5.as_ptr();
                    } else {
                        VP8PutBit(
                            bw,
                            1 as libc::c_int,
                            *p.offset(8 as libc::c_int as isize) as libc::c_int,
                        );
                        VP8PutBit(
                            bw,
                            1 as libc::c_int,
                            *p.offset(10 as libc::c_int as isize) as libc::c_int,
                        );
                        v -= 3 as libc::c_int + ((8 as libc::c_int) << 3 as libc::c_int);
                        mask = (1 as libc::c_int) << 10 as libc::c_int;
                        tab = VP8Cat6.as_ptr();
                    }
                    while mask != 0 {
                        let fresh2 = tab;
                        tab = tab.offset(1);
                        VP8PutBit(
                            bw,
                            (v & mask != 0) as libc::c_int,
                            *fresh2 as libc::c_int,
                        );
                        mask >>= 1 as libc::c_int;
                    }
                }
                p = ((*((*res).prob)
                    .offset(
                        VP8EncBands[n as usize] as isize,
                    ))[2 as libc::c_int as usize])
                    .as_mut_ptr();
            }
            VP8PutBitUniform(bw, sign);
            if n == 16 as libc::c_int
                || VP8PutBit(
                    bw,
                    (n <= (*res).last) as libc::c_int,
                    *p.offset(0 as libc::c_int as isize) as libc::c_int,
                ) == 0
            {
                return 1 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn CodeResiduals(
    bw: *mut VP8BitWriter,
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut res: VP8Residual = VP8Residual {
        first: 0,
        last: 0,
        coeffs: 0 as *const int16_t,
        coeff_type: 0,
        prob: 0 as *mut ProbaArray,
        stats: 0 as *mut StatsArray,
        costs: 0 as *mut [*const uint16_t; 3],
    };
    let mut pos1: uint64_t = 0;
    let mut pos2: uint64_t = 0;
    let mut pos3: uint64_t = 0;
    let i16: libc::c_int = ((*(*it).mb_).type_() as libc::c_int == 1 as libc::c_int)
        as libc::c_int;
    let segment: libc::c_int = (*(*it).mb_).segment_() as libc::c_int;
    let enc: *mut VP8Encoder = (*it).enc_;
    VP8IteratorNzToBytes(it);
    pos1 = VP8BitWriterPos(bw);
    if i16 != 0 {
        VP8InitResidual(0 as libc::c_int, 1 as libc::c_int, enc, &mut res);
        VP8SetResidualCoeffs
            .expect("non-null function pointer")(((*rd).y_dc_levels).as_ptr(), &mut res);
        (*it)
            .left_nz_[8 as libc::c_int
            as usize] = PutCoeffs(
            bw,
            (*it).top_nz_[8 as libc::c_int as usize]
                + (*it).left_nz_[8 as libc::c_int as usize],
            &mut res,
        );
        (*it)
            .top_nz_[8 as libc::c_int
            as usize] = (*it).left_nz_[8 as libc::c_int as usize];
        VP8InitResidual(1 as libc::c_int, 0 as libc::c_int, enc, &mut res);
    } else {
        VP8InitResidual(0 as libc::c_int, 3 as libc::c_int, enc, &mut res);
    }
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
            (*it).left_nz_[y as usize] = PutCoeffs(bw, ctx, &mut res);
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    pos2 = VP8BitWriterPos(bw);
    VP8InitResidual(0 as libc::c_int, 2 as libc::c_int, enc, &mut res);
    ch = 0 as libc::c_int;
    while ch <= 2 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 2 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 2 as libc::c_int {
                let ctx_0: libc::c_int = (*it)
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
                (*it)
                    .left_nz_[(4 as libc::c_int + ch + y)
                    as usize] = PutCoeffs(bw, ctx_0, &mut res);
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
    pos3 = VP8BitWriterPos(bw);
    (*it).luma_bits_ = pos2.wrapping_sub(pos1);
    (*it).uv_bits_ = pos3.wrapping_sub(pos2);
    (*it)
        .bit_count_[segment
        as usize][i16
        as usize] = ((*it).bit_count_[segment as usize][i16 as usize] as libc::c_ulong)
        .wrapping_add((*it).luma_bits_) as uint64_t as uint64_t;
    (*it)
        .bit_count_[segment
        as usize][2 as libc::c_int
        as usize] = ((*it).bit_count_[segment as usize][2 as libc::c_int as usize]
        as libc::c_ulong)
        .wrapping_add((*it).uv_bits_) as uint64_t as uint64_t;
    VP8IteratorBytesToNz(it);
}
unsafe extern "C" fn RecordResiduals(it: *mut VP8EncIterator, rd: *const VP8ModeScore) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
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
    VP8IteratorNzToBytes(it);
    if (*(*it).mb_).type_() as libc::c_int == 1 as libc::c_int {
        VP8InitResidual(0 as libc::c_int, 1 as libc::c_int, enc, &mut res);
        VP8SetResidualCoeffs
            .expect("non-null function pointer")(((*rd).y_dc_levels).as_ptr(), &mut res);
        (*it)
            .left_nz_[8 as libc::c_int
            as usize] = VP8RecordCoeffs(
            (*it).top_nz_[8 as libc::c_int as usize]
                + (*it).left_nz_[8 as libc::c_int as usize],
            &mut res,
        );
        (*it)
            .top_nz_[8 as libc::c_int
            as usize] = (*it).left_nz_[8 as libc::c_int as usize];
        VP8InitResidual(1 as libc::c_int, 0 as libc::c_int, enc, &mut res);
    } else {
        VP8InitResidual(0 as libc::c_int, 3 as libc::c_int, enc, &mut res);
    }
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
            (*it).left_nz_[y as usize] = VP8RecordCoeffs(ctx, &mut res);
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    VP8InitResidual(0 as libc::c_int, 2 as libc::c_int, enc, &mut res);
    ch = 0 as libc::c_int;
    while ch <= 2 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 2 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 2 as libc::c_int {
                let ctx_0: libc::c_int = (*it)
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
                (*it)
                    .left_nz_[(4 as libc::c_int + ch + y)
                    as usize] = VP8RecordCoeffs(ctx_0, &mut res);
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
    VP8IteratorBytesToNz(it);
}
unsafe extern "C" fn RecordTokens(
    it: *mut VP8EncIterator,
    rd: *const VP8ModeScore,
    tokens: *mut VP8TBuffer,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
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
    VP8IteratorNzToBytes(it);
    if (*(*it).mb_).type_() as libc::c_int == 1 as libc::c_int {
        let ctx: libc::c_int = (*it).top_nz_[8 as libc::c_int as usize]
            + (*it).left_nz_[8 as libc::c_int as usize];
        VP8InitResidual(0 as libc::c_int, 1 as libc::c_int, enc, &mut res);
        VP8SetResidualCoeffs
            .expect("non-null function pointer")(((*rd).y_dc_levels).as_ptr(), &mut res);
        (*it)
            .left_nz_[8 as libc::c_int
            as usize] = VP8RecordCoeffTokens(ctx, &mut res, tokens);
        (*it)
            .top_nz_[8 as libc::c_int
            as usize] = (*it).left_nz_[8 as libc::c_int as usize];
        VP8InitResidual(1 as libc::c_int, 0 as libc::c_int, enc, &mut res);
    } else {
        VP8InitResidual(0 as libc::c_int, 3 as libc::c_int, enc, &mut res);
    }
    y = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            let ctx_0: libc::c_int = (*it).top_nz_[x as usize]
                + (*it).left_nz_[y as usize];
            VP8SetResidualCoeffs
                .expect(
                    "non-null function pointer",
                )(
                ((*rd).y_ac_levels[(x + y * 4 as libc::c_int) as usize]).as_ptr(),
                &mut res,
            );
            (*it).left_nz_[y as usize] = VP8RecordCoeffTokens(ctx_0, &mut res, tokens);
            (*it).top_nz_[x as usize] = (*it).left_nz_[y as usize];
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    VP8InitResidual(0 as libc::c_int, 2 as libc::c_int, enc, &mut res);
    ch = 0 as libc::c_int;
    while ch <= 2 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 2 as libc::c_int {
            x = 0 as libc::c_int;
            while x < 2 as libc::c_int {
                let ctx_1: libc::c_int = (*it)
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
                (*it)
                    .left_nz_[(4 as libc::c_int + ch + y)
                    as usize] = VP8RecordCoeffTokens(ctx_1, &mut res, tokens);
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
    VP8IteratorBytesToNz(it);
    return ((*tokens).error_ == 0) as libc::c_int;
}
unsafe extern "C" fn ResetSSE(enc: *mut VP8Encoder) {
    (*enc).sse_[0 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*enc).sse_[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*enc).sse_[2 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*enc).sse_count_ = 0 as libc::c_int as uint64_t;
}
unsafe extern "C" fn StoreSSE(it: *const VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let in_0: *const uint8_t = (*it).yuv_in_;
    let out: *const uint8_t = (*it).yuv_out_;
    (*enc)
        .sse_[0 as libc::c_int
        as usize] = ((*enc).sse_[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            VP8SSE16x16
                .expect(
                    "non-null function pointer",
                )(
                in_0.offset(0 as libc::c_int as isize),
                out.offset(0 as libc::c_int as isize),
            ) as libc::c_ulong,
        ) as uint64_t as uint64_t;
    (*enc)
        .sse_[1 as libc::c_int
        as usize] = ((*enc).sse_[1 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            VP8SSE8x8
                .expect(
                    "non-null function pointer",
                )(
                in_0.offset(16 as libc::c_int as isize),
                out.offset(16 as libc::c_int as isize),
            ) as libc::c_ulong,
        ) as uint64_t as uint64_t;
    (*enc)
        .sse_[2 as libc::c_int
        as usize] = ((*enc).sse_[2 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            VP8SSE8x8
                .expect(
                    "non-null function pointer",
                )(
                in_0.offset((16 as libc::c_int + 8 as libc::c_int) as isize),
                out.offset((16 as libc::c_int + 8 as libc::c_int) as isize),
            ) as libc::c_ulong,
        ) as uint64_t as uint64_t;
    (*enc)
        .sse_count_ = ((*enc).sse_count_ as libc::c_ulong)
        .wrapping_add((16 as libc::c_int * 16 as libc::c_int) as libc::c_ulong)
        as uint64_t as uint64_t;
}
unsafe extern "C" fn StoreSideInfo(it: *const VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let mb: *const VP8MBInfo = (*it).mb_;
    let pic: *mut WebPPicture = (*enc).pic_;
    if !((*pic).stats).is_null() {
        StoreSSE(it);
        (*enc).block_count_[0 as libc::c_int as usize]
            += ((*mb).type_() as libc::c_int == 0 as libc::c_int) as libc::c_int;
        (*enc).block_count_[1 as libc::c_int as usize]
            += ((*mb).type_() as libc::c_int == 1 as libc::c_int) as libc::c_int;
        (*enc).block_count_[2 as libc::c_int as usize]
            += ((*mb).skip_() as libc::c_int != 0 as libc::c_int) as libc::c_int;
    }
    if !((*pic).extra_info).is_null() {
        let info: *mut uint8_t = &mut *((*pic).extra_info)
            .offset(((*it).x_ + (*it).y_ * (*enc).mb_w_) as isize) as *mut uint8_t;
        match (*pic).extra_info_type {
            1 => {
                *info = (*mb).type_() as uint8_t;
            }
            2 => {
                *info = (*mb).segment_() as uint8_t;
            }
            3 => {
                *info = (*enc).dqm_[(*mb).segment_() as usize].quant_ as uint8_t;
            }
            4 => {
                *info = (if (*mb).type_() as libc::c_int == 1 as libc::c_int {
                    *((*it).preds_).offset(0 as libc::c_int as isize) as libc::c_int
                } else {
                    0xff as libc::c_int
                }) as uint8_t;
            }
            5 => {
                *info = (*mb).uv_mode_() as uint8_t;
            }
            6 => {
                let b: libc::c_int = (((*it).luma_bits_)
                    .wrapping_add((*it).uv_bits_)
                    .wrapping_add(7 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int)
                    as libc::c_int;
                *info = (if b > 255 as libc::c_int { 255 as libc::c_int } else { b })
                    as uint8_t;
            }
            7 => {
                *info = (*mb).alpha_;
            }
            _ => {
                *info = 0 as libc::c_int as uint8_t;
            }
        }
    }
}
unsafe extern "C" fn ResetSideInfo(it: *const VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let pic: *mut WebPPicture = (*enc).pic_;
    if !((*pic).stats).is_null() {
        memset(
            ((*enc).block_count_).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong,
        );
    }
    ResetSSE(enc);
}
unsafe extern "C" fn GetPSNR(mut mse: uint64_t, mut size: uint64_t) -> libc::c_double {
    return if mse > 0 as libc::c_int as libc::c_ulong
        && size > 0 as libc::c_int as libc::c_ulong
    {
        10.0f64
            * log10(255.0f64 * 255.0f64 * size as libc::c_double / mse as libc::c_double)
    } else {
        99 as libc::c_int as libc::c_double
    };
}
unsafe extern "C" fn SetLoopParams(enc: *mut VP8Encoder, mut q: libc::c_float) {
    q = Clamp(q, 0.0f32, 100.0f32);
    VP8SetSegmentParams(enc, q);
    SetSegmentProbas(enc);
    ResetStats(enc);
    ResetSSE(enc);
}
unsafe extern "C" fn OneStatPass(
    enc: *mut VP8Encoder,
    mut rd_opt: VP8RDLevel,
    mut nb_mbs: libc::c_int,
    mut percent_delta: libc::c_int,
    s: *mut PassStats,
) -> uint64_t {
    let mut it: VP8EncIterator = VP8EncIterator {
        x_: 0,
        y_: 0,
        yuv_in_: 0 as *mut uint8_t,
        yuv_out_: 0 as *mut uint8_t,
        yuv_out2_: 0 as *mut uint8_t,
        yuv_p_: 0 as *mut uint8_t,
        enc_: 0 as *mut VP8Encoder,
        mb_: 0 as *mut VP8MBInfo,
        bw_: 0 as *mut VP8BitWriter,
        preds_: 0 as *mut uint8_t,
        nz_: 0 as *mut uint32_t,
        i4_boundary_: [0; 37],
        i4_top_: 0 as *mut uint8_t,
        i4_: 0,
        top_nz_: [0; 9],
        left_nz_: [0; 9],
        bit_count_: [[0; 3]; 4],
        luma_bits_: 0,
        uv_bits_: 0,
        lf_stats_: 0 as *mut LFStats,
        do_trellis_: 0,
        count_down_: 0,
        count_down0_: 0,
        percent0_: 0,
        left_derr_: [[0; 2]; 2],
        top_derr_: 0 as *mut DError,
        y_left_: 0 as *mut uint8_t,
        u_left_: 0 as *mut uint8_t,
        v_left_: 0 as *mut uint8_t,
        y_top_: 0 as *mut uint8_t,
        uv_top_: 0 as *mut uint8_t,
        yuv_left_mem_: [0; 88],
        yuv_mem_: [0; 3359],
    };
    let mut size: uint64_t = 0 as libc::c_int as uint64_t;
    let mut size_p0: uint64_t = 0 as libc::c_int as uint64_t;
    let mut distortion: uint64_t = 0 as libc::c_int as uint64_t;
    let pixel_count: uint64_t = (nb_mbs as uint64_t)
        .wrapping_mul(384 as libc::c_int as libc::c_ulong);
    VP8IteratorInit(enc, &mut it);
    SetLoopParams(enc, (*s).q);
    loop {
        let mut info: VP8ModeScore = VP8ModeScore {
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
        VP8IteratorImport(&mut it, 0 as *mut uint8_t);
        if VP8Decimate(&mut it, &mut info, rd_opt) != 0 {
            (*enc).proba_.nb_skip_ += 1;
            (*enc).proba_.nb_skip_;
        }
        RecordResiduals(&mut it, &mut info);
        size = (size as libc::c_ulong).wrapping_add((info.R + info.H) as libc::c_ulong)
            as uint64_t as uint64_t;
        size_p0 = (size_p0 as libc::c_ulong).wrapping_add(info.H as libc::c_ulong)
            as uint64_t as uint64_t;
        distortion = (distortion as libc::c_ulong).wrapping_add(info.D as libc::c_ulong)
            as uint64_t as uint64_t;
        if percent_delta != 0 && VP8IteratorProgress(&mut it, percent_delta) == 0 {
            return 0 as libc::c_int as uint64_t;
        }
        VP8IteratorSaveBoundary(&mut it);
        if !(VP8IteratorNext(&mut it) != 0
            && {
                nb_mbs -= 1;
                nb_mbs > 0 as libc::c_int
            })
        {
            break;
        }
    }
    size_p0 = (size_p0 as libc::c_ulong)
        .wrapping_add((*enc).segment_hdr_.size_ as libc::c_ulong) as uint64_t
        as uint64_t;
    if (*s).do_size_search != 0 {
        size = (size as libc::c_ulong)
            .wrapping_add(FinalizeSkipProba(enc) as libc::c_ulong) as uint64_t
            as uint64_t;
        size = (size as libc::c_ulong)
            .wrapping_add(FinalizeTokenProbas(&mut (*enc).proba_) as libc::c_ulong)
            as uint64_t as uint64_t;
        size = (size
            .wrapping_add(size_p0)
            .wrapping_add(1024 as libc::c_int as libc::c_ulong) >> 11 as libc::c_int)
            .wrapping_add(
                (12 as libc::c_int + 8 as libc::c_int + 10 as libc::c_int)
                    as libc::c_ulong,
            );
        (*s).value = size as libc::c_double;
    } else {
        (*s).value = GetPSNR(distortion, pixel_count);
    }
    return size_p0;
}
unsafe extern "C" fn StatLoop(enc: *mut VP8Encoder) -> libc::c_int {
    let method: libc::c_int = (*enc).method_;
    let do_search: libc::c_int = (*enc).do_search_;
    let fast_probe: libc::c_int = ((method == 0 as libc::c_int
        || method == 3 as libc::c_int) && do_search == 0) as libc::c_int;
    let mut num_pass_left: libc::c_int = (*(*enc).config_).pass;
    let task_percent: libc::c_int = 20 as libc::c_int;
    let percent_per_pass: libc::c_int = (task_percent + num_pass_left / 2 as libc::c_int)
        / num_pass_left;
    let final_percent: libc::c_int = (*enc).percent_ + task_percent;
    let rd_opt: VP8RDLevel = (if method >= 3 as libc::c_int || do_search != 0 {
        RD_OPT_BASIC as libc::c_int
    } else {
        RD_OPT_NONE as libc::c_int
    }) as VP8RDLevel;
    let mut nb_mbs: libc::c_int = (*enc).mb_w_ * (*enc).mb_h_;
    let mut stats: PassStats = PassStats {
        is_first: 0,
        dq: 0.,
        q: 0.,
        last_q: 0.,
        qmin: 0.,
        qmax: 0.,
        value: 0.,
        last_value: 0.,
        target: 0.,
        do_size_search: 0,
    };
    InitPassStats(enc, &mut stats);
    ResetTokenStats(enc);
    if fast_probe != 0 {
        if method == 3 as libc::c_int {
            nb_mbs = if nb_mbs > 200 as libc::c_int {
                nb_mbs >> 1 as libc::c_int
            } else {
                100 as libc::c_int
            };
        } else {
            nb_mbs = if nb_mbs > 200 as libc::c_int {
                nb_mbs >> 2 as libc::c_int
            } else {
                50 as libc::c_int
            };
        }
    }
    loop {
        let fresh3 = num_pass_left;
        num_pass_left = num_pass_left - 1;
        if !(fresh3 > 0 as libc::c_int) {
            break;
        }
        let is_last_pass: libc::c_int = (fabs(stats.dq as libc::c_double) <= 0.4f64
            || num_pass_left == 0 as libc::c_int
            || (*enc).max_i4_header_bits_ == 0 as libc::c_int) as libc::c_int;
        let size_p0: uint64_t = OneStatPass(
            enc,
            rd_opt,
            nb_mbs,
            percent_per_pass,
            &mut stats,
        );
        if size_p0 == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if (*enc).max_i4_header_bits_ > 0 as libc::c_int
            && size_p0 as libc::c_ulonglong
                > (((1 as libc::c_int) << 19 as libc::c_int) as libc::c_ulonglong)
                    .wrapping_sub(2048 as libc::c_ulonglong) << 11 as libc::c_int
        {
            num_pass_left += 1;
            num_pass_left;
            (*enc).max_i4_header_bits_ >>= 1 as libc::c_int;
        } else {
            if is_last_pass != 0 {
                break;
            }
            if !(do_search != 0) {
                continue;
            }
            ComputeNextQ(&mut stats);
            if fabs(stats.dq as libc::c_double) <= 0.4f64 {
                break;
            }
        }
    }
    if do_search == 0 || stats.do_size_search == 0 {
        FinalizeSkipProba(enc);
        FinalizeTokenProbas(&mut (*enc).proba_);
    }
    VP8CalculateLevelCosts(&mut (*enc).proba_);
    return WebPReportProgress((*enc).pic_, final_percent, &mut (*enc).percent_);
}
static mut kAverageBytesPerMB: [uint8_t; 8] = [
    50 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
];
unsafe extern "C" fn PreLoopInitialize(enc: *mut VP8Encoder) -> libc::c_int {
    let mut p: libc::c_int = 0;
    let mut ok: libc::c_int = 1 as libc::c_int;
    let average_bytes_per_MB: libc::c_int = kAverageBytesPerMB[((*enc).base_quant_
        >> 4 as libc::c_int) as usize] as libc::c_int;
    let bytes_per_parts: libc::c_int = (*enc).mb_w_ * (*enc).mb_h_ * average_bytes_per_MB
        / (*enc).num_parts_;
    p = 0 as libc::c_int;
    while ok != 0 && p < (*enc).num_parts_ {
        ok = VP8BitWriterInit(
            ((*enc).parts_).as_mut_ptr().offset(p as isize),
            bytes_per_parts as size_t,
        );
        p += 1;
        p;
    }
    if ok == 0 {
        VP8EncFreeBitWriters(enc);
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
unsafe extern "C" fn PostLoopFinalize(
    it: *mut VP8EncIterator,
    mut ok: libc::c_int,
) -> libc::c_int {
    let enc: *mut VP8Encoder = (*it).enc_;
    if ok != 0 {
        let mut p: libc::c_int = 0;
        p = 0 as libc::c_int;
        while p < (*enc).num_parts_ {
            VP8BitWriterFinish(((*enc).parts_).as_mut_ptr().offset(p as isize));
            ok &= ((*enc).parts_[p as usize].error_ == 0) as libc::c_int;
            p += 1;
            p;
        }
    }
    if ok != 0 {
        if !((*(*enc).pic_).stats).is_null() {
            let mut i: libc::c_int = 0;
            let mut s: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i <= 2 as libc::c_int {
                s = 0 as libc::c_int;
                while s < NUM_MB_SEGMENTS as libc::c_int {
                    (*enc)
                        .residual_bytes_[i
                        as usize][s
                        as usize] = (((*it).bit_count_[s as usize][i as usize])
                        .wrapping_add(7 as libc::c_int as libc::c_ulong)
                        >> 3 as libc::c_int) as libc::c_int;
                    s += 1;
                    s;
                }
                i += 1;
                i;
            }
        }
        VP8AdjustFilterStrength(it);
    } else {
        VP8EncFreeBitWriters(enc);
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
unsafe extern "C" fn ResetAfterSkip(it: *mut VP8EncIterator) {
    if (*(*it).mb_).type_() as libc::c_int == 1 as libc::c_int {
        *(*it).nz_ = 0 as libc::c_int as uint32_t;
        (*it).left_nz_[8 as libc::c_int as usize] = 0 as libc::c_int;
    } else {
        *(*it).nz_ &= ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncLoop(enc: *mut VP8Encoder) -> libc::c_int {
    let mut it: VP8EncIterator = VP8EncIterator {
        x_: 0,
        y_: 0,
        yuv_in_: 0 as *mut uint8_t,
        yuv_out_: 0 as *mut uint8_t,
        yuv_out2_: 0 as *mut uint8_t,
        yuv_p_: 0 as *mut uint8_t,
        enc_: 0 as *mut VP8Encoder,
        mb_: 0 as *mut VP8MBInfo,
        bw_: 0 as *mut VP8BitWriter,
        preds_: 0 as *mut uint8_t,
        nz_: 0 as *mut uint32_t,
        i4_boundary_: [0; 37],
        i4_top_: 0 as *mut uint8_t,
        i4_: 0,
        top_nz_: [0; 9],
        left_nz_: [0; 9],
        bit_count_: [[0; 3]; 4],
        luma_bits_: 0,
        uv_bits_: 0,
        lf_stats_: 0 as *mut LFStats,
        do_trellis_: 0,
        count_down_: 0,
        count_down0_: 0,
        percent0_: 0,
        left_derr_: [[0; 2]; 2],
        top_derr_: 0 as *mut DError,
        y_left_: 0 as *mut uint8_t,
        u_left_: 0 as *mut uint8_t,
        v_left_: 0 as *mut uint8_t,
        y_top_: 0 as *mut uint8_t,
        uv_top_: 0 as *mut uint8_t,
        yuv_left_mem_: [0; 88],
        yuv_mem_: [0; 3359],
    };
    let mut ok: libc::c_int = PreLoopInitialize(enc);
    if ok == 0 {
        return 0 as libc::c_int;
    }
    StatLoop(enc);
    VP8IteratorInit(enc, &mut it);
    VP8InitFilter(&mut it);
    loop {
        let mut info: VP8ModeScore = VP8ModeScore {
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
        let dont_use_skip: libc::c_int = ((*enc).proba_.use_skip_proba_ == 0)
            as libc::c_int;
        let rd_opt: VP8RDLevel = (*enc).rd_opt_level_;
        VP8IteratorImport(&mut it, 0 as *mut uint8_t);
        if VP8Decimate(&mut it, &mut info, rd_opt) == 0 || dont_use_skip != 0 {
            CodeResiduals(it.bw_, &mut it, &mut info);
            if (*it.bw_).error_ != 0 {
                ok = 0 as libc::c_int;
                break;
            }
        } else {
            ResetAfterSkip(&mut it);
        }
        StoreSideInfo(&mut it);
        VP8StoreFilterStats(&mut it);
        VP8IteratorExport(&mut it);
        ok = VP8IteratorProgress(&mut it, 20 as libc::c_int);
        VP8IteratorSaveBoundary(&mut it);
        if !(ok != 0 && VP8IteratorNext(&mut it) != 0) {
            break;
        }
    }
    return PostLoopFinalize(&mut it, ok);
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncTokenLoop(enc: *mut VP8Encoder) -> libc::c_int {
    let mut max_count: libc::c_int = (*enc).mb_w_ * (*enc).mb_h_ >> 3 as libc::c_int;
    let mut num_pass_left: libc::c_int = (*(*enc).config_).pass;
    let mut remaining_progress: libc::c_int = 40 as libc::c_int;
    let do_search: libc::c_int = (*enc).do_search_;
    let mut it: VP8EncIterator = VP8EncIterator {
        x_: 0,
        y_: 0,
        yuv_in_: 0 as *mut uint8_t,
        yuv_out_: 0 as *mut uint8_t,
        yuv_out2_: 0 as *mut uint8_t,
        yuv_p_: 0 as *mut uint8_t,
        enc_: 0 as *mut VP8Encoder,
        mb_: 0 as *mut VP8MBInfo,
        bw_: 0 as *mut VP8BitWriter,
        preds_: 0 as *mut uint8_t,
        nz_: 0 as *mut uint32_t,
        i4_boundary_: [0; 37],
        i4_top_: 0 as *mut uint8_t,
        i4_: 0,
        top_nz_: [0; 9],
        left_nz_: [0; 9],
        bit_count_: [[0; 3]; 4],
        luma_bits_: 0,
        uv_bits_: 0,
        lf_stats_: 0 as *mut LFStats,
        do_trellis_: 0,
        count_down_: 0,
        count_down0_: 0,
        percent0_: 0,
        left_derr_: [[0; 2]; 2],
        top_derr_: 0 as *mut DError,
        y_left_: 0 as *mut uint8_t,
        u_left_: 0 as *mut uint8_t,
        v_left_: 0 as *mut uint8_t,
        y_top_: 0 as *mut uint8_t,
        uv_top_: 0 as *mut uint8_t,
        yuv_left_mem_: [0; 88],
        yuv_mem_: [0; 3359],
    };
    let proba: *mut VP8EncProba = &mut (*enc).proba_;
    let rd_opt: VP8RDLevel = (*enc).rd_opt_level_;
    let pixel_count: uint64_t = ((*enc).mb_w_ as uint64_t)
        .wrapping_mul((*enc).mb_h_ as libc::c_ulong)
        .wrapping_mul(384 as libc::c_int as libc::c_ulong);
    let mut stats: PassStats = PassStats {
        is_first: 0,
        dq: 0.,
        q: 0.,
        last_q: 0.,
        qmin: 0.,
        qmax: 0.,
        value: 0.,
        last_value: 0.,
        target: 0.,
        do_size_search: 0,
    };
    let mut ok: libc::c_int = 0;
    InitPassStats(enc, &mut stats);
    ok = PreLoopInitialize(enc);
    if ok == 0 {
        return 0 as libc::c_int;
    }
    if max_count < 96 as libc::c_int {
        max_count = 96 as libc::c_int;
    }
    while ok != 0
        && {
            let fresh4 = num_pass_left;
            num_pass_left = num_pass_left - 1;
            fresh4 > 0 as libc::c_int
        }
    {
        let is_last_pass: libc::c_int = (fabs(stats.dq as libc::c_double) <= 0.4f64
            || num_pass_left == 0 as libc::c_int
            || (*enc).max_i4_header_bits_ == 0 as libc::c_int) as libc::c_int;
        let mut size_p0: uint64_t = 0 as libc::c_int as uint64_t;
        let mut distortion: uint64_t = 0 as libc::c_int as uint64_t;
        let mut cnt: libc::c_int = max_count;
        let pass_progress: libc::c_int = remaining_progress
            / (2 as libc::c_int + num_pass_left);
        remaining_progress -= pass_progress;
        VP8IteratorInit(enc, &mut it);
        SetLoopParams(enc, stats.q);
        if is_last_pass != 0 {
            ResetTokenStats(enc);
            VP8InitFilter(&mut it);
        }
        VP8TBufferClear(&mut (*enc).tokens_);
        loop {
            let mut info: VP8ModeScore = VP8ModeScore {
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
            VP8IteratorImport(&mut it, 0 as *mut uint8_t);
            cnt -= 1;
            if cnt < 0 as libc::c_int {
                FinalizeTokenProbas(proba);
                VP8CalculateLevelCosts(proba);
                cnt = max_count;
            }
            VP8Decimate(&mut it, &mut info, rd_opt);
            ok = RecordTokens(&mut it, &mut info, &mut (*enc).tokens_);
            if ok == 0 {
                WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
                break;
            } else {
                size_p0 = (size_p0 as libc::c_ulong)
                    .wrapping_add(info.H as libc::c_ulong) as uint64_t as uint64_t;
                distortion = (distortion as libc::c_ulong)
                    .wrapping_add(info.D as libc::c_ulong) as uint64_t as uint64_t;
                if is_last_pass != 0 {
                    StoreSideInfo(&mut it);
                    VP8StoreFilterStats(&mut it);
                    VP8IteratorExport(&mut it);
                    ok = VP8IteratorProgress(&mut it, pass_progress);
                }
                VP8IteratorSaveBoundary(&mut it);
                if !(ok != 0 && VP8IteratorNext(&mut it) != 0) {
                    break;
                }
            }
        }
        if ok == 0 {
            break;
        }
        size_p0 = (size_p0 as libc::c_ulong)
            .wrapping_add((*enc).segment_hdr_.size_ as libc::c_ulong) as uint64_t
            as uint64_t;
        if stats.do_size_search != 0 {
            let mut size: uint64_t = FinalizeTokenProbas(&mut (*enc).proba_) as uint64_t;
            size = (size as libc::c_ulong)
                .wrapping_add(
                    VP8EstimateTokenSize(
                        &mut (*enc).tokens_,
                        ((*proba).coeffs_).as_mut_ptr() as *const uint8_t,
                    ),
                ) as uint64_t as uint64_t;
            size = size
                .wrapping_add(size_p0)
                .wrapping_add(1024 as libc::c_int as libc::c_ulong) >> 11 as libc::c_int;
            size = (size as libc::c_ulong)
                .wrapping_add(
                    (12 as libc::c_int + 8 as libc::c_int + 10 as libc::c_int)
                        as libc::c_ulong,
                ) as uint64_t as uint64_t;
            stats.value = size as libc::c_double;
        } else {
            stats.value = GetPSNR(distortion, pixel_count);
        }
        if (*enc).max_i4_header_bits_ > 0 as libc::c_int
            && size_p0 as libc::c_ulonglong
                > (((1 as libc::c_int) << 19 as libc::c_int) as libc::c_ulonglong)
                    .wrapping_sub(2048 as libc::c_ulonglong) << 11 as libc::c_int
        {
            num_pass_left += 1;
            num_pass_left;
            (*enc).max_i4_header_bits_ >>= 1 as libc::c_int;
            if is_last_pass != 0 {
                ResetSideInfo(&mut it);
            }
        } else {
            if is_last_pass != 0 {
                break;
            }
            if do_search != 0 {
                ComputeNextQ(&mut stats);
            }
        }
    }
    if ok != 0 {
        if stats.do_size_search == 0 {
            FinalizeTokenProbas(&mut (*enc).proba_);
        }
        ok = VP8EmitTokens(
            &mut (*enc).tokens_,
            ((*enc).parts_).as_mut_ptr().offset(0 as libc::c_int as isize),
            ((*proba).coeffs_).as_mut_ptr() as *const uint8_t,
            1 as libc::c_int,
        );
    }
    ok = (ok != 0
        && WebPReportProgress(
            (*enc).pic_,
            (*enc).percent_ + remaining_progress,
            &mut (*enc).percent_,
        ) != 0) as libc::c_int;
    return PostLoopFinalize(&mut it, ok);
}
