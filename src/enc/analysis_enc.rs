use ::libc;
use ::c2rust_bitfields;

use super::token_enc::VP8Tokens;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut VP8Mean16x4: VP8MeanMetric;
    static mut VP8CollectHistogram: VP8CHisto;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn VP8MakeChroma8Preds(it: *const VP8EncIterator);
    fn VP8MakeLuma16Preds(it: *const VP8EncIterator);
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    static VP8UVModeOffsets: [uint16_t; 4];
    static VP8I16ModeOffsets: [uint16_t; 4];
    fn VP8IteratorInit(enc: *mut VP8Encoder, it: *mut VP8EncIterator);
    fn VP8IteratorSetRow(it: *mut VP8EncIterator, y: libc::c_int);
    fn VP8IteratorSetCountDown(it: *mut VP8EncIterator, count_down: libc::c_int);
    fn VP8IteratorIsDone(it: *const VP8EncIterator) -> libc::c_int;
    fn VP8IteratorImport(it: *mut VP8EncIterator, tmp_32: *mut uint8_t);
    fn VP8IteratorNext(it: *mut VP8EncIterator) -> libc::c_int;
    fn VP8IteratorProgress(it: *const VP8EncIterator, delta: libc::c_int) -> libc::c_int;
    fn VP8SetIntra16Mode(it: *const VP8EncIterator, mode: libc::c_int);
    fn VP8SetIntra4Mode(it: *const VP8EncIterator, modes: *const uint8_t);
    fn VP8SetIntraUVMode(it: *const VP8EncIterator, mode: libc::c_int);
    fn VP8SetSkip(it: *const VP8EncIterator, skip: libc::c_int);
    fn VP8SetSegment(it: *const VP8EncIterator, segment: libc::c_int);
}
pub type size_t = libc::c_ulong;
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
pub type uintptr_t = libc::c_ulong;
pub type VP8MeanMetric = Option::<
    unsafe extern "C" fn(*const uint8_t, *mut uint32_t) -> (),
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
pub struct VP8Histogram {
    pub max_value: libc::c_int,
    pub last_non_zero: libc::c_int,
}
pub type VP8CHisto = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        *mut VP8Histogram,
    ) -> (),
>;
pub type StatsArray = [[proba_t; 11]; 3];
pub type proba_t = uint32_t;
pub type ProbaArray = [[uint8_t; 11]; 3];
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
pub struct SegmentJob {
    pub worker: WebPWorker,
    pub alphas: [libc::c_int; 256],
    pub alpha: libc::c_int,
    pub uv_alpha: libc::c_int,
    pub it: VP8EncIterator,
    pub delta_progress: libc::c_int,
}
unsafe extern "C" fn SmoothSegmentMap(enc: *mut VP8Encoder) {
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let w: libc::c_int = (*enc).mb_w_;
    let h: libc::c_int = (*enc).mb_h_;
    let majority_cnt_3_x_3_grid: libc::c_int = 5 as libc::c_int;
    let tmp: *mut uint8_t = WebPSafeMalloc(
        (w * h) as uint64_t,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    if tmp.is_null() {
        return;
    }
    y = 1 as libc::c_int;
    while y < h - 1 as libc::c_int {
        x = 1 as libc::c_int;
        while x < w - 1 as libc::c_int {
            let mut cnt: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
            let mb: *const VP8MBInfo = &mut *((*enc).mb_info_)
                .offset((x + w * y) as isize) as *mut VP8MBInfo;
            let mut majority_seg: libc::c_int = (*mb).segment_() as libc::c_int;
            cnt[(*mb.offset((-w - 1 as libc::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((-w - 1 as libc::c_int) as isize)).segment_() as usize];
            cnt[(*mb.offset((-w + 0 as libc::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((-w + 0 as libc::c_int) as isize)).segment_() as usize];
            cnt[(*mb.offset((-w + 1 as libc::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((-w + 1 as libc::c_int) as isize)).segment_() as usize];
            cnt[(*mb.offset(-(1 as libc::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset(-(1 as libc::c_int) as isize)).segment_() as usize];
            cnt[(*mb.offset(1 as libc::c_int as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset(1 as libc::c_int as isize)).segment_() as usize];
            cnt[(*mb.offset((w - 1 as libc::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((w - 1 as libc::c_int) as isize)).segment_() as usize];
            cnt[(*mb.offset((w + 0 as libc::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((w + 0 as libc::c_int) as isize)).segment_() as usize];
            cnt[(*mb.offset((w + 1 as libc::c_int) as isize)).segment_() as usize] += 1;
            cnt[(*mb.offset((w + 1 as libc::c_int) as isize)).segment_() as usize];
            n = 0 as libc::c_int;
            while n < NUM_MB_SEGMENTS as libc::c_int {
                if cnt[n as usize] >= majority_cnt_3_x_3_grid {
                    majority_seg = n;
                    break;
                } else {
                    n += 1;
                    n;
                }
            }
            *tmp.offset((x + y * w) as isize) = majority_seg as uint8_t;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    y = 1 as libc::c_int;
    while y < h - 1 as libc::c_int {
        x = 1 as libc::c_int;
        while x < w - 1 as libc::c_int {
            let mb_0: *mut VP8MBInfo = &mut *((*enc).mb_info_)
                .offset((x + w * y) as isize) as *mut VP8MBInfo;
            (*mb_0).set_segment_(*tmp.offset((x + y * w) as isize) as libc::c_uint);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    WebPSafeFree(tmp as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn clip(
    mut v: libc::c_int,
    mut m: libc::c_int,
    mut M: libc::c_int,
) -> libc::c_int {
    return if v < m { m } else if v > M { M } else { v };
}
unsafe extern "C" fn SetSegmentAlphas(
    enc: *mut VP8Encoder,
    mut centers: *const libc::c_int,
    mut mid: libc::c_int,
) {
    let nb: libc::c_int = (*enc).segment_hdr_.num_segments_;
    let mut min: libc::c_int = *centers.offset(0 as libc::c_int as isize);
    let mut max: libc::c_int = *centers.offset(0 as libc::c_int as isize);
    let mut n: libc::c_int = 0;
    if nb > 1 as libc::c_int {
        n = 0 as libc::c_int;
        while n < nb {
            if min > *centers.offset(n as isize) {
                min = *centers.offset(n as isize);
            }
            if max < *centers.offset(n as isize) {
                max = *centers.offset(n as isize);
            }
            n += 1;
            n;
        }
    }
    if max == min {
        max = min + 1 as libc::c_int;
    }
    n = 0 as libc::c_int;
    while n < nb {
        let alpha: libc::c_int = 255 as libc::c_int * (*centers.offset(n as isize) - mid)
            / (max - min);
        let beta: libc::c_int = 255 as libc::c_int * (*centers.offset(n as isize) - min)
            / (max - min);
        (*enc)
            .dqm_[n as usize]
            .alpha_ = clip(alpha, -(127 as libc::c_int), 127 as libc::c_int);
        (*enc).dqm_[n as usize].beta_ = clip(beta, 0 as libc::c_int, 255 as libc::c_int);
        n += 1;
        n;
    }
}
unsafe extern "C" fn FinalAlphaValue(mut alpha: libc::c_int) -> libc::c_int {
    alpha = 255 as libc::c_int - alpha;
    return clip(alpha, 0 as libc::c_int, 255 as libc::c_int);
}
unsafe extern "C" fn GetAlpha(histo: *const VP8Histogram) -> libc::c_int {
    let max_value: libc::c_int = (*histo).max_value;
    let last_non_zero: libc::c_int = (*histo).last_non_zero;
    let alpha: libc::c_int = if max_value > 1 as libc::c_int {
        2 as libc::c_int * 255 as libc::c_int * last_non_zero / max_value
    } else {
        0 as libc::c_int
    };
    return alpha;
}
unsafe extern "C" fn InitHistogram(histo: *mut VP8Histogram) {
    (*histo).max_value = 0 as libc::c_int;
    (*histo).last_non_zero = 1 as libc::c_int;
}
unsafe extern "C" fn AssignSegments(
    enc: *mut VP8Encoder,
    mut alphas: *const libc::c_int,
) {
    let nb: libc::c_int = if (*enc).segment_hdr_.num_segments_
        < NUM_MB_SEGMENTS as libc::c_int
    {
        (*enc).segment_hdr_.num_segments_
    } else {
        NUM_MB_SEGMENTS as libc::c_int
    };
    let mut centers: [libc::c_int; 4] = [0; 4];
    let mut weighted_average: libc::c_int = 0 as libc::c_int;
    let mut map: [libc::c_int; 256] = [0; 256];
    let mut a: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut min_a: libc::c_int = 0 as libc::c_int;
    let mut max_a: libc::c_int = 255 as libc::c_int;
    let mut range_a: libc::c_int = 0;
    let mut accum: [libc::c_int; 4] = [0; 4];
    let mut dist_accum: [libc::c_int; 4] = [0; 4];
    n = 0 as libc::c_int;
    while n <= 255 as libc::c_int && *alphas.offset(n as isize) == 0 as libc::c_int {
        n += 1;
        n;
    }
    min_a = n;
    n = 255 as libc::c_int;
    while n > min_a && *alphas.offset(n as isize) == 0 as libc::c_int {
        n -= 1;
        n;
    }
    max_a = n;
    range_a = max_a - min_a;
    k = 0 as libc::c_int;
    n = 1 as libc::c_int;
    while k < nb {
        centers[k as usize] = min_a + n * range_a / (2 as libc::c_int * nb);
        k += 1;
        k;
        n += 2 as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < 6 as libc::c_int {
        let mut total_weight: libc::c_int = 0;
        let mut displaced: libc::c_int = 0;
        n = 0 as libc::c_int;
        while n < nb {
            accum[n as usize] = 0 as libc::c_int;
            dist_accum[n as usize] = 0 as libc::c_int;
            n += 1;
            n;
        }
        n = 0 as libc::c_int;
        a = min_a;
        while a <= max_a {
            if *alphas.offset(a as isize) != 0 {
                while (n + 1 as libc::c_int) < nb
                    && abs(a - centers[(n + 1 as libc::c_int) as usize])
                        < abs(a - centers[n as usize])
                {
                    n += 1;
                    n;
                }
                map[a as usize] = n;
                dist_accum[n as usize] += a * *alphas.offset(a as isize);
                accum[n as usize] += *alphas.offset(a as isize);
            }
            a += 1;
            a;
        }
        displaced = 0 as libc::c_int;
        weighted_average = 0 as libc::c_int;
        total_weight = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while n < nb {
            if accum[n as usize] != 0 {
                let new_center: libc::c_int = (dist_accum[n as usize]
                    + accum[n as usize] / 2 as libc::c_int) / accum[n as usize];
                displaced += abs(centers[n as usize] - new_center);
                centers[n as usize] = new_center;
                weighted_average += new_center * accum[n as usize];
                total_weight += accum[n as usize];
            }
            n += 1;
            n;
        }
        weighted_average = (weighted_average + total_weight / 2 as libc::c_int)
            / total_weight;
        if displaced < 5 as libc::c_int {
            break;
        }
        k += 1;
        k;
    }
    n = 0 as libc::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        let mb: *mut VP8MBInfo = &mut *((*enc).mb_info_).offset(n as isize)
            as *mut VP8MBInfo;
        let alpha: libc::c_int = (*mb).alpha_ as libc::c_int;
        (*mb).set_segment_(map[alpha as usize] as libc::c_uint);
        (*mb).alpha_ = centers[map[alpha as usize] as usize] as uint8_t;
        n += 1;
        n;
    }
    if nb > 1 as libc::c_int {
        let smooth: libc::c_int = (*(*enc).config_).preprocessing & 1 as libc::c_int;
        if smooth != 0 {
            SmoothSegmentMap(enc);
        }
    }
    SetSegmentAlphas(enc, centers.as_mut_ptr() as *const libc::c_int, weighted_average);
}
unsafe extern "C" fn MBAnalyzeBestIntra16Mode(it: *mut VP8EncIterator) -> libc::c_int {
    let max_mode: libc::c_int = 2 as libc::c_int;
    let mut mode: libc::c_int = 0;
    let mut best_alpha: libc::c_int = -(1 as libc::c_int);
    let mut best_mode: libc::c_int = 0 as libc::c_int;
    VP8MakeLuma16Preds(it);
    mode = 0 as libc::c_int;
    while mode < max_mode {
        let mut histo: VP8Histogram = VP8Histogram {
            max_value: 0,
            last_non_zero: 0,
        };
        let mut alpha: libc::c_int = 0;
        InitHistogram(&mut histo);
        VP8CollectHistogram
            .expect(
                "non-null function pointer",
            )(
            ((*it).yuv_in_).offset(0 as libc::c_int as isize),
            ((*it).yuv_p_)
                .offset(VP8I16ModeOffsets[mode as usize] as libc::c_int as isize),
            0 as libc::c_int,
            16 as libc::c_int,
            &mut histo,
        );
        alpha = GetAlpha(&mut histo);
        if alpha > best_alpha {
            best_alpha = alpha;
            best_mode = mode;
        }
        mode += 1;
        mode;
    }
    VP8SetIntra16Mode(it, best_mode);
    return best_alpha;
}
unsafe extern "C" fn FastMBAnalyze(it: *mut VP8EncIterator) -> libc::c_int {
    let q: libc::c_int = (*(*(*it).enc_).config_).quality as libc::c_int;
    let kThreshold: uint32_t = (8 as libc::c_int
        + (17 as libc::c_int - 8 as libc::c_int) * q / 100 as libc::c_int) as uint32_t;
    let mut k: libc::c_int = 0;
    let mut dc: [uint32_t; 16] = [0; 16];
    let mut m: uint32_t = 0;
    let mut m2: uint32_t = 0;
    k = 0 as libc::c_int;
    while k < 16 as libc::c_int {
        VP8Mean16x4
            .expect(
                "non-null function pointer",
            )(
            ((*it).yuv_in_)
                .offset(0 as libc::c_int as isize)
                .offset((k * 32 as libc::c_int) as isize),
            &mut *dc.as_mut_ptr().offset(k as isize),
        );
        k += 4 as libc::c_int;
    }
    m = 0 as libc::c_int as uint32_t;
    m2 = 0 as libc::c_int as uint32_t;
    k = 0 as libc::c_int;
    while k < 16 as libc::c_int {
        m = (m as libc::c_uint).wrapping_add(dc[k as usize]) as uint32_t as uint32_t;
        m2 = (m2 as libc::c_uint)
            .wrapping_add((dc[k as usize]).wrapping_mul(dc[k as usize])) as uint32_t
            as uint32_t;
        k += 1;
        k;
    }
    if kThreshold.wrapping_mul(m2) < m.wrapping_mul(m) {
        VP8SetIntra16Mode(it, 0 as libc::c_int);
    } else {
        let modes: [uint8_t; 16] = [
            0 as libc::c_int as uint8_t,
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
        VP8SetIntra4Mode(it, modes.as_ptr());
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn MBAnalyzeBestUVMode(it: *mut VP8EncIterator) -> libc::c_int {
    let mut best_alpha: libc::c_int = -(1 as libc::c_int);
    let mut smallest_alpha: libc::c_int = 0 as libc::c_int;
    let mut best_mode: libc::c_int = 0 as libc::c_int;
    let max_mode: libc::c_int = 2 as libc::c_int;
    let mut mode: libc::c_int = 0;
    VP8MakeChroma8Preds(it);
    mode = 0 as libc::c_int;
    while mode < max_mode {
        let mut histo: VP8Histogram = VP8Histogram {
            max_value: 0,
            last_non_zero: 0,
        };
        let mut alpha: libc::c_int = 0;
        InitHistogram(&mut histo);
        VP8CollectHistogram
            .expect(
                "non-null function pointer",
            )(
            ((*it).yuv_in_).offset(16 as libc::c_int as isize),
            ((*it).yuv_p_)
                .offset(VP8UVModeOffsets[mode as usize] as libc::c_int as isize),
            16 as libc::c_int,
            16 as libc::c_int + 4 as libc::c_int + 4 as libc::c_int,
            &mut histo,
        );
        alpha = GetAlpha(&mut histo);
        if alpha > best_alpha {
            best_alpha = alpha;
        }
        if mode == 0 as libc::c_int || alpha < smallest_alpha {
            smallest_alpha = alpha;
            best_mode = mode;
        }
        mode += 1;
        mode;
    }
    VP8SetIntraUVMode(it, best_mode);
    return best_alpha;
}
unsafe extern "C" fn MBAnalyze(
    it: *mut VP8EncIterator,
    mut alphas: *mut libc::c_int,
    alpha: *mut libc::c_int,
    uv_alpha: *mut libc::c_int,
) {
    let enc: *const VP8Encoder = (*it).enc_;
    let mut best_alpha: libc::c_int = 0;
    let mut best_uv_alpha: libc::c_int = 0;
    VP8SetIntra16Mode(it, 0 as libc::c_int);
    VP8SetSkip(it, 0 as libc::c_int);
    VP8SetSegment(it, 0 as libc::c_int);
    if (*enc).method_ <= 1 as libc::c_int {
        best_alpha = FastMBAnalyze(it);
    } else {
        best_alpha = MBAnalyzeBestIntra16Mode(it);
    }
    best_uv_alpha = MBAnalyzeBestUVMode(it);
    best_alpha = 3 as libc::c_int * best_alpha + best_uv_alpha + 2 as libc::c_int
        >> 2 as libc::c_int;
    best_alpha = FinalAlphaValue(best_alpha);
    let ref mut fresh0 = *alphas.offset(best_alpha as isize);
    *fresh0 += 1;
    *fresh0;
    (*(*it).mb_).alpha_ = best_alpha as uint8_t;
    *alpha += best_alpha;
    *uv_alpha += best_uv_alpha;
}
unsafe extern "C" fn DefaultMBInfo(mb: *mut VP8MBInfo) {
    (*mb).set_type_(1 as libc::c_int as libc::c_uint);
    (*mb).set_uv_mode_(0 as libc::c_int as libc::c_uint);
    (*mb).set_skip_(0 as libc::c_int as libc::c_uint);
    (*mb).set_segment_(0 as libc::c_int as libc::c_uint);
    (*mb).alpha_ = 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn ResetAllMBInfo(enc: *mut VP8Encoder) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < (*enc).mb_w_ * (*enc).mb_h_ {
        DefaultMBInfo(&mut *((*enc).mb_info_).offset(n as isize));
        n += 1;
        n;
    }
    (*enc).dqm_[0 as libc::c_int as usize].alpha_ = 0 as libc::c_int;
    (*enc).dqm_[0 as libc::c_int as usize].beta_ = 0 as libc::c_int;
    (*enc).alpha_ = 0 as libc::c_int;
    (*enc).uv_alpha_ = 0 as libc::c_int;
    WebPReportProgress(
        (*enc).pic_,
        (*enc).percent_ + 20 as libc::c_int,
        &mut (*enc).percent_,
    );
}
unsafe extern "C" fn DoSegmentsJob(
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
) -> libc::c_int {
    let job: *mut SegmentJob = arg1 as *mut SegmentJob;
    let it: *mut VP8EncIterator = arg2 as *mut VP8EncIterator;
    let mut ok: libc::c_int = 1 as libc::c_int;
    if VP8IteratorIsDone(it) == 0 {
        let mut tmp: [uint8_t; 63] = [0; 63];
        let scratch: *mut uint8_t = ((tmp.as_mut_ptr() as uintptr_t)
            .wrapping_add(31 as libc::c_int as libc::c_ulong)
            & !(31 as libc::c_int as uintptr_t)) as *mut uint8_t;
        loop {
            VP8IteratorImport(it, scratch);
            MBAnalyze(
                it,
                ((*job).alphas).as_mut_ptr(),
                &mut (*job).alpha,
                &mut (*job).uv_alpha,
            );
            ok = VP8IteratorProgress(it, (*job).delta_progress);
            if !(ok != 0 && VP8IteratorNext(it) != 0) {
                break;
            }
        }
    }
    return ok;
}
unsafe extern "C" fn MergeJobs(src: *const SegmentJob, dst: *mut SegmentJob) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 255 as libc::c_int {
        (*dst).alphas[i as usize] += (*src).alphas[i as usize];
        i += 1;
        i;
    }
    (*dst).alpha += (*src).alpha;
    (*dst).uv_alpha += (*src).uv_alpha;
}
unsafe extern "C" fn InitSegmentJob(
    enc: *mut VP8Encoder,
    job: *mut SegmentJob,
    mut start_row: libc::c_int,
    mut end_row: libc::c_int,
) {
    ((*WebPGetWorkerInterface()).Init)
        .expect("non-null function pointer")(&mut (*job).worker);
    (*job).worker.data1 = job as *mut libc::c_void;
    (*job).worker.data2 = &mut (*job).it as *mut VP8EncIterator as *mut libc::c_void;
    (*job)
        .worker
        .hook = Some(
        DoSegmentsJob
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    );
    VP8IteratorInit(enc, &mut (*job).it);
    VP8IteratorSetRow(&mut (*job).it, start_row);
    VP8IteratorSetCountDown(&mut (*job).it, (end_row - start_row) * (*enc).mb_w_);
    memset(
        ((*job).alphas).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong,
    );
    (*job).alpha = 0 as libc::c_int;
    (*job).uv_alpha = 0 as libc::c_int;
    (*job)
        .delta_progress = if start_row == 0 as libc::c_int {
        20 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncAnalyze(enc: *mut VP8Encoder) -> libc::c_int {
    let mut ok: libc::c_int = 1 as libc::c_int;
    let do_segments: libc::c_int = ((*(*enc).config_).emulate_jpeg_size != 0
        || (*enc).segment_hdr_.num_segments_ > 1 as libc::c_int
        || (*enc).method_ <= 1 as libc::c_int) as libc::c_int;
    if do_segments != 0 {
        let last_row: libc::c_int = (*enc).mb_h_;
        let total_mb: libc::c_int = last_row * (*enc).mb_w_;
        let split_row: libc::c_int = 9 as libc::c_int * last_row + 15 as libc::c_int
            >> 4 as libc::c_int;
        let kMinSplitRow: libc::c_int = 2 as libc::c_int;
        let do_mt: libc::c_int = ((*enc).thread_level_ > 0 as libc::c_int
            && split_row >= kMinSplitRow) as libc::c_int;
        let worker_interface: *const WebPWorkerInterface = WebPGetWorkerInterface();
        let mut main_job: SegmentJob = SegmentJob {
            worker: WebPWorker {
                impl_: 0 as *mut libc::c_void,
                status_: NOT_OK,
                hook: None,
                data1: 0 as *mut libc::c_void,
                data2: 0 as *mut libc::c_void,
                had_error: 0,
            },
            alphas: [0; 256],
            alpha: 0,
            uv_alpha: 0,
            it: VP8EncIterator {
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
            },
            delta_progress: 0,
        };
        if do_mt != 0 {
            let mut side_job: SegmentJob = SegmentJob {
                worker: WebPWorker {
                    impl_: 0 as *mut libc::c_void,
                    status_: NOT_OK,
                    hook: None,
                    data1: 0 as *mut libc::c_void,
                    data2: 0 as *mut libc::c_void,
                    had_error: 0,
                },
                alphas: [0; 256],
                alpha: 0,
                uv_alpha: 0,
                it: VP8EncIterator {
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
                },
                delta_progress: 0,
            };
            InitSegmentJob(enc, &mut main_job, 0 as libc::c_int, split_row);
            InitSegmentJob(enc, &mut side_job, split_row, last_row);
            ok
                &= ((*worker_interface).Reset)
                    .expect("non-null function pointer")(&mut side_job.worker);
            if ok != 0 {
                ((*worker_interface).Launch)
                    .expect("non-null function pointer")(&mut side_job.worker);
                ((*worker_interface).Execute)
                    .expect("non-null function pointer")(&mut main_job.worker);
                ok
                    &= ((*worker_interface).Sync_0)
                        .expect("non-null function pointer")(&mut side_job.worker);
                ok
                    &= ((*worker_interface).Sync_0)
                        .expect("non-null function pointer")(&mut main_job.worker);
            }
            ((*worker_interface).End)
                .expect("non-null function pointer")(&mut side_job.worker);
            if ok != 0 {
                MergeJobs(&mut side_job, &mut main_job);
            }
        } else {
            InitSegmentJob(enc, &mut main_job, 0 as libc::c_int, last_row);
            ((*worker_interface).Execute)
                .expect("non-null function pointer")(&mut main_job.worker);
            ok
                &= ((*worker_interface).Sync_0)
                    .expect("non-null function pointer")(&mut main_job.worker);
        }
        ((*worker_interface).End)
            .expect("non-null function pointer")(&mut main_job.worker);
        if ok != 0 {
            (*enc).alpha_ = main_job.alpha / total_mb;
            (*enc).uv_alpha_ = main_job.uv_alpha / total_mb;
            AssignSegments(enc, (main_job.alphas).as_mut_ptr() as *const libc::c_int);
        }
    } else {
        ResetAllMBInfo(enc);
    }
    if ok == 0 {
        return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return ok;
}
