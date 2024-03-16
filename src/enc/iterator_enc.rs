use ::libc;
use ::c2rust_bitfields;

use super::token_enc::VP8Tokens;
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
    static VP8Scan: [uint16_t; 16];
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
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
unsafe extern "C" fn InitLeft(it: *mut VP8EncIterator) {
    let ref mut fresh0 = *((*it).v_left_).offset(-(1 as libc::c_int) as isize);
    *fresh0 = (if (*it).y_ > 0 as libc::c_int {
        129 as libc::c_int
    } else {
        127 as libc::c_int
    }) as uint8_t;
    let ref mut fresh1 = *((*it).u_left_).offset(-(1 as libc::c_int) as isize);
    *fresh1 = *fresh0;
    *((*it).y_left_).offset(-(1 as libc::c_int) as isize) = *fresh1;
    memset(
        (*it).y_left_ as *mut libc::c_void,
        129 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    memset(
        (*it).u_left_ as *mut libc::c_void,
        129 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    memset(
        (*it).v_left_ as *mut libc::c_void,
        129 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    (*it).left_nz_[8 as libc::c_int as usize] = 0 as libc::c_int;
    if !((*it).top_derr_).is_null() {
        memset(
            &mut (*it).left_derr_ as *mut DError as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<DError>() as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn InitTop(it: *mut VP8EncIterator) {
    let enc: *const VP8Encoder = (*it).enc_;
    let top_size: size_t = ((*enc).mb_w_ * 16 as libc::c_int) as size_t;
    memset(
        (*enc).y_top_ as *mut libc::c_void,
        127 as libc::c_int,
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(top_size),
    );
    memset(
        (*enc).nz_ as *mut libc::c_void,
        0 as libc::c_int,
        ((*enc).mb_w_ as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    if !((*enc).top_derr_).is_null() {
        memset(
            (*enc).top_derr_ as *mut libc::c_void,
            0 as libc::c_int,
            ((*enc).mb_w_ as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<DError>() as libc::c_ulong),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorSetRow(it: *mut VP8EncIterator, mut y: libc::c_int) {
    let enc: *mut VP8Encoder = (*it).enc_;
    (*it).x_ = 0 as libc::c_int;
    (*it).y_ = y;
    (*it)
        .bw_ = &mut *((*enc).parts_)
        .as_mut_ptr()
        .offset((y & (*enc).num_parts_ - 1 as libc::c_int) as isize)
        as *mut VP8BitWriter;
    (*it)
        .preds_ = ((*enc).preds_)
        .offset((y * 4 as libc::c_int * (*enc).preds_w_) as isize);
    (*it).nz_ = (*enc).nz_;
    (*it).mb_ = ((*enc).mb_info_).offset((y * (*enc).mb_w_) as isize);
    (*it).y_top_ = (*enc).y_top_;
    (*it).uv_top_ = (*enc).uv_top_;
    InitLeft(it);
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorReset(it: *mut VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    VP8IteratorSetRow(it, 0 as libc::c_int);
    VP8IteratorSetCountDown(it, (*enc).mb_w_ * (*enc).mb_h_);
    InitTop(it);
    memset(
        ((*it).bit_count_).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[uint64_t; 3]; 4]>() as libc::c_ulong,
    );
    (*it).do_trellis_ = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorSetCountDown(
    it: *mut VP8EncIterator,
    mut count_down: libc::c_int,
) {
    (*it).count_down0_ = count_down;
    (*it).count_down_ = (*it).count_down0_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorIsDone(it: *const VP8EncIterator) -> libc::c_int {
    return ((*it).count_down_ <= 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorInit(enc: *mut VP8Encoder, it: *mut VP8EncIterator) {
    (*it).enc_ = enc;
    (*it)
        .yuv_in_ = ((((*it).yuv_mem_).as_mut_ptr() as uintptr_t)
        .wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut uint8_t;
    (*it)
        .yuv_out_ = ((*it).yuv_in_)
        .offset((32 as libc::c_int * 16 as libc::c_int) as isize);
    (*it)
        .yuv_out2_ = ((*it).yuv_out_)
        .offset((32 as libc::c_int * 16 as libc::c_int) as isize);
    (*it)
        .yuv_p_ = ((*it).yuv_out2_)
        .offset((32 as libc::c_int * 16 as libc::c_int) as isize);
    (*it).lf_stats_ = (*enc).lf_stats_;
    (*it).percent0_ = (*enc).percent_;
    (*it)
        .y_left_ = ((((*it).yuv_left_mem_).as_mut_ptr().offset(1 as libc::c_int as isize)
        as uintptr_t)
        .wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut uint8_t;
    (*it)
        .u_left_ = ((*it).y_left_)
        .offset(16 as libc::c_int as isize)
        .offset(16 as libc::c_int as isize);
    (*it).v_left_ = ((*it).u_left_).offset(16 as libc::c_int as isize);
    (*it).top_derr_ = (*enc).top_derr_;
    VP8IteratorReset(it);
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorProgress(
    it: *const VP8EncIterator,
    mut delta: libc::c_int,
) -> libc::c_int {
    let enc: *mut VP8Encoder = (*it).enc_;
    if delta != 0 && ((*(*enc).pic_).progress_hook).is_some() {
        let done: libc::c_int = (*it).count_down0_ - (*it).count_down_;
        let percent: libc::c_int = if (*it).count_down0_ <= 0 as libc::c_int {
            (*it).percent0_
        } else {
            (*it).percent0_ + delta * done / (*it).count_down0_
        };
        return WebPReportProgress((*enc).pic_, percent, &mut (*enc).percent_);
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MinSize(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn ImportBlock(
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
    mut dst: *mut uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < h {
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void, w as libc::c_ulong);
        if w < size {
            memset(
                dst.offset(w as isize) as *mut libc::c_void,
                *dst.offset((w - 1 as libc::c_int) as isize) as libc::c_int,
                (size - w) as libc::c_ulong,
            );
        }
        dst = dst.offset(32 as libc::c_int as isize);
        src = src.offset(src_stride as isize);
        i += 1;
        i;
    }
    i = h;
    while i < size {
        memcpy(
            dst as *mut libc::c_void,
            dst.offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            size as libc::c_ulong,
        );
        dst = dst.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn ImportLine(
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
    mut dst: *mut uint8_t,
    mut len: libc::c_int,
    mut total_len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        *dst.offset(i as isize) = *src;
        i += 1;
        i;
        src = src.offset(src_stride as isize);
    }
    while i < total_len {
        *dst.offset(i as isize) = *dst.offset((len - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorImport(
    it: *mut VP8EncIterator,
    tmp_32: *mut uint8_t,
) {
    let enc: *const VP8Encoder = (*it).enc_;
    let x: libc::c_int = (*it).x_;
    let y: libc::c_int = (*it).y_;
    let pic: *const WebPPicture = (*enc).pic_;
    let ysrc: *const uint8_t = ((*pic).y)
        .offset(((y * (*pic).y_stride + x) * 16 as libc::c_int) as isize);
    let usrc: *const uint8_t = ((*pic).u)
        .offset(((y * (*pic).uv_stride + x) * 8 as libc::c_int) as isize);
    let vsrc: *const uint8_t = ((*pic).v)
        .offset(((y * (*pic).uv_stride + x) * 8 as libc::c_int) as isize);
    let w: libc::c_int = MinSize(
        (*pic).width - x * 16 as libc::c_int,
        16 as libc::c_int,
    );
    let h: libc::c_int = MinSize(
        (*pic).height - y * 16 as libc::c_int,
        16 as libc::c_int,
    );
    let uv_w: libc::c_int = w + 1 as libc::c_int >> 1 as libc::c_int;
    let uv_h: libc::c_int = h + 1 as libc::c_int >> 1 as libc::c_int;
    ImportBlock(
        ysrc,
        (*pic).y_stride,
        ((*it).yuv_in_).offset(0 as libc::c_int as isize),
        w,
        h,
        16 as libc::c_int,
    );
    ImportBlock(
        usrc,
        (*pic).uv_stride,
        ((*it).yuv_in_).offset(16 as libc::c_int as isize),
        uv_w,
        uv_h,
        8 as libc::c_int,
    );
    ImportBlock(
        vsrc,
        (*pic).uv_stride,
        ((*it).yuv_in_).offset((16 as libc::c_int + 8 as libc::c_int) as isize),
        uv_w,
        uv_h,
        8 as libc::c_int,
    );
    if tmp_32.is_null() {
        return;
    }
    if x == 0 as libc::c_int {
        InitLeft(it);
    } else {
        if y == 0 as libc::c_int {
            let ref mut fresh2 = *((*it).v_left_).offset(-(1 as libc::c_int) as isize);
            *fresh2 = 127 as libc::c_int as uint8_t;
            let ref mut fresh3 = *((*it).u_left_).offset(-(1 as libc::c_int) as isize);
            *fresh3 = *fresh2;
            *((*it).y_left_).offset(-(1 as libc::c_int) as isize) = *fresh3;
        } else {
            *((*it).y_left_)
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = *ysrc.offset((-(1 as libc::c_int) - (*pic).y_stride) as isize);
            *((*it).u_left_)
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = *usrc.offset((-(1 as libc::c_int) - (*pic).uv_stride) as isize);
            *((*it).v_left_)
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = *vsrc.offset((-(1 as libc::c_int) - (*pic).uv_stride) as isize);
        }
        ImportLine(
            ysrc.offset(-(1 as libc::c_int as isize)),
            (*pic).y_stride,
            (*it).y_left_,
            h,
            16 as libc::c_int,
        );
        ImportLine(
            usrc.offset(-(1 as libc::c_int as isize)),
            (*pic).uv_stride,
            (*it).u_left_,
            uv_h,
            8 as libc::c_int,
        );
        ImportLine(
            vsrc.offset(-(1 as libc::c_int as isize)),
            (*pic).uv_stride,
            (*it).v_left_,
            uv_h,
            8 as libc::c_int,
        );
    }
    (*it).y_top_ = tmp_32.offset(0 as libc::c_int as isize);
    (*it).uv_top_ = tmp_32.offset(16 as libc::c_int as isize);
    if y == 0 as libc::c_int {
        memset(
            tmp_32 as *mut libc::c_void,
            127 as libc::c_int,
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
    } else {
        ImportLine(
            ysrc.offset(-((*pic).y_stride as isize)),
            1 as libc::c_int,
            tmp_32,
            w,
            16 as libc::c_int,
        );
        ImportLine(
            usrc.offset(-((*pic).uv_stride as isize)),
            1 as libc::c_int,
            tmp_32.offset(16 as libc::c_int as isize),
            uv_w,
            8 as libc::c_int,
        );
        ImportLine(
            vsrc.offset(-((*pic).uv_stride as isize)),
            1 as libc::c_int,
            tmp_32.offset(16 as libc::c_int as isize).offset(8 as libc::c_int as isize),
            uv_w,
            8 as libc::c_int,
        );
    };
}
unsafe extern "C" fn ExportBlock(
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
    mut dst_stride: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    loop {
        let fresh4 = h;
        h = h - 1;
        if !(fresh4 > 0 as libc::c_int) {
            break;
        }
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void, w as libc::c_ulong);
        dst = dst.offset(dst_stride as isize);
        src = src.offset(32 as libc::c_int as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorExport(it: *const VP8EncIterator) {
    let enc: *const VP8Encoder = (*it).enc_;
    if (*(*enc).config_).show_compressed != 0 {
        let x: libc::c_int = (*it).x_;
        let y: libc::c_int = (*it).y_;
        let ysrc: *const uint8_t = ((*it).yuv_out_).offset(0 as libc::c_int as isize);
        let usrc: *const uint8_t = ((*it).yuv_out_).offset(16 as libc::c_int as isize);
        let vsrc: *const uint8_t = ((*it).yuv_out_)
            .offset((16 as libc::c_int + 8 as libc::c_int) as isize);
        let pic: *const WebPPicture = (*enc).pic_;
        let ydst: *mut uint8_t = ((*pic).y)
            .offset(((y * (*pic).y_stride + x) * 16 as libc::c_int) as isize);
        let udst: *mut uint8_t = ((*pic).u)
            .offset(((y * (*pic).uv_stride + x) * 8 as libc::c_int) as isize);
        let vdst: *mut uint8_t = ((*pic).v)
            .offset(((y * (*pic).uv_stride + x) * 8 as libc::c_int) as isize);
        let mut w: libc::c_int = (*pic).width - x * 16 as libc::c_int;
        let mut h: libc::c_int = (*pic).height - y * 16 as libc::c_int;
        if w > 16 as libc::c_int {
            w = 16 as libc::c_int;
        }
        if h > 16 as libc::c_int {
            h = 16 as libc::c_int;
        }
        ExportBlock(ysrc, ydst, (*pic).y_stride, w, h);
        let uv_w: libc::c_int = w + 1 as libc::c_int >> 1 as libc::c_int;
        let uv_h: libc::c_int = h + 1 as libc::c_int >> 1 as libc::c_int;
        ExportBlock(usrc, udst, (*pic).uv_stride, uv_w, uv_h);
        ExportBlock(vsrc, vdst, (*pic).uv_stride, uv_w, uv_h);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorNzToBytes(it: *mut VP8EncIterator) {
    let tnz: libc::c_int = *((*it).nz_).offset(0 as libc::c_int as isize) as libc::c_int;
    let lnz: libc::c_int = *((*it).nz_).offset(-(1 as libc::c_int) as isize)
        as libc::c_int;
    let top_nz: *mut libc::c_int = ((*it).top_nz_).as_mut_ptr();
    let left_nz: *mut libc::c_int = ((*it).left_nz_).as_mut_ptr();
    *top_nz
        .offset(
            0 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 12 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            1 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 13 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            2 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 14 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            3 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 15 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            4 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 18 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            5 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 19 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            6 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 22 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            7 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 23 as libc::c_int != 0) as libc::c_int;
    *top_nz
        .offset(
            8 as libc::c_int as isize,
        ) = (tnz & (1 as libc::c_int) << 24 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            0 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 3 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            1 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 7 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            2 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 11 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            3 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 15 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            4 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 17 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            5 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 19 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            6 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 21 as libc::c_int != 0) as libc::c_int;
    *left_nz
        .offset(
            7 as libc::c_int as isize,
        ) = (lnz & (1 as libc::c_int) << 23 as libc::c_int != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorBytesToNz(it: *mut VP8EncIterator) {
    let mut nz: uint32_t = 0 as libc::c_int as uint32_t;
    let top_nz: *const libc::c_int = ((*it).top_nz_).as_mut_ptr();
    let left_nz: *const libc::c_int = ((*it).left_nz_).as_mut_ptr();
    nz
        |= (*top_nz.offset(0 as libc::c_int as isize) << 12 as libc::c_int
            | *top_nz.offset(1 as libc::c_int as isize) << 13 as libc::c_int)
            as libc::c_uint;
    nz
        |= (*top_nz.offset(2 as libc::c_int as isize) << 14 as libc::c_int
            | *top_nz.offset(3 as libc::c_int as isize) << 15 as libc::c_int)
            as libc::c_uint;
    nz
        |= (*top_nz.offset(4 as libc::c_int as isize) << 18 as libc::c_int
            | *top_nz.offset(5 as libc::c_int as isize) << 19 as libc::c_int)
            as libc::c_uint;
    nz
        |= (*top_nz.offset(6 as libc::c_int as isize) << 22 as libc::c_int
            | *top_nz.offset(7 as libc::c_int as isize) << 23 as libc::c_int)
            as libc::c_uint;
    nz
        |= (*top_nz.offset(8 as libc::c_int as isize) << 24 as libc::c_int)
            as libc::c_uint;
    nz
        |= (*left_nz.offset(0 as libc::c_int as isize) << 3 as libc::c_int
            | *left_nz.offset(1 as libc::c_int as isize) << 7 as libc::c_int)
            as libc::c_uint;
    nz
        |= (*left_nz.offset(2 as libc::c_int as isize) << 11 as libc::c_int)
            as libc::c_uint;
    nz
        |= (*left_nz.offset(4 as libc::c_int as isize) << 17 as libc::c_int
            | *left_nz.offset(6 as libc::c_int as isize) << 21 as libc::c_int)
            as libc::c_uint;
    *(*it).nz_ = nz;
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorSaveBoundary(it: *mut VP8EncIterator) {
    let enc: *mut VP8Encoder = (*it).enc_;
    let x: libc::c_int = (*it).x_;
    let y: libc::c_int = (*it).y_;
    let ysrc: *const uint8_t = ((*it).yuv_out_).offset(0 as libc::c_int as isize);
    let uvsrc: *const uint8_t = ((*it).yuv_out_).offset(16 as libc::c_int as isize);
    if x < (*enc).mb_w_ - 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            *((*it).y_left_)
                .offset(
                    i as isize,
                ) = *ysrc.offset((15 as libc::c_int + i * 32 as libc::c_int) as isize);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            *((*it).u_left_)
                .offset(
                    i as isize,
                ) = *uvsrc.offset((7 as libc::c_int + i * 32 as libc::c_int) as isize);
            *((*it).v_left_)
                .offset(
                    i as isize,
                ) = *uvsrc.offset((15 as libc::c_int + i * 32 as libc::c_int) as isize);
            i += 1;
            i;
        }
        *((*it).y_left_)
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *((*it).y_top_).offset(15 as libc::c_int as isize);
        *((*it).u_left_)
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *((*it).uv_top_).offset((0 as libc::c_int + 7 as libc::c_int) as isize);
        *((*it).v_left_)
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *((*it).uv_top_).offset((8 as libc::c_int + 7 as libc::c_int) as isize);
    }
    if y < (*enc).mb_h_ - 1 as libc::c_int {
        memcpy(
            (*it).y_top_ as *mut libc::c_void,
            ysrc.offset((15 as libc::c_int * 32 as libc::c_int) as isize)
                as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            (*it).uv_top_ as *mut libc::c_void,
            uvsrc.offset((7 as libc::c_int * 32 as libc::c_int) as isize)
                as *const libc::c_void,
            (8 as libc::c_int + 8 as libc::c_int) as libc::c_ulong,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorNext(it: *mut VP8EncIterator) -> libc::c_int {
    (*it).x_ += 1;
    if (*it).x_ == (*(*it).enc_).mb_w_ {
        (*it).y_ += 1;
        VP8IteratorSetRow(it, (*it).y_);
    } else {
        (*it).preds_ = ((*it).preds_).offset(4 as libc::c_int as isize);
        (*it).mb_ = ((*it).mb_).offset(1 as libc::c_int as isize);
        (*it).nz_ = ((*it).nz_).offset(1 as libc::c_int as isize);
        (*it).y_top_ = ((*it).y_top_).offset(16 as libc::c_int as isize);
        (*it).uv_top_ = ((*it).uv_top_).offset(16 as libc::c_int as isize);
    }
    (*it).count_down_ -= 1;
    return ((0 as libc::c_int) < (*it).count_down_) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetIntra16Mode(
    it: *const VP8EncIterator,
    mut mode: libc::c_int,
) {
    let mut preds: *mut uint8_t = (*it).preds_;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        memset(preds as *mut libc::c_void, mode, 4 as libc::c_int as libc::c_ulong);
        preds = preds.offset((*(*it).enc_).preds_w_ as isize);
        y += 1;
        y;
    }
    (*(*it).mb_).set_type_(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetIntra4Mode(
    it: *const VP8EncIterator,
    mut modes: *const uint8_t,
) {
    let mut preds: *mut uint8_t = (*it).preds_;
    let mut y: libc::c_int = 0;
    y = 4 as libc::c_int;
    while y > 0 as libc::c_int {
        memcpy(
            preds as *mut libc::c_void,
            modes as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
        preds = preds.offset((*(*it).enc_).preds_w_ as isize);
        modes = modes.offset(4 as libc::c_int as isize);
        y -= 1;
        y;
    }
    (*(*it).mb_).set_type_(0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetIntraUVMode(
    it: *const VP8EncIterator,
    mut mode: libc::c_int,
) {
    (*(*it).mb_).set_uv_mode_(mode as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetSkip(it: *const VP8EncIterator, mut skip: libc::c_int) {
    (*(*it).mb_).set_skip_(skip as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn VP8SetSegment(
    it: *const VP8EncIterator,
    mut segment: libc::c_int,
) {
    (*(*it).mb_).set_segment_(segment as libc::c_uint);
}
static mut VP8TopLeftI4: [uint8_t; 16] = [
    17 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorStartI4(it: *mut VP8EncIterator) {
    let enc: *const VP8Encoder = (*it).enc_;
    let mut i: libc::c_int = 0;
    (*it).i4_ = 0 as libc::c_int;
    (*it)
        .i4_top_ = ((*it).i4_boundary_)
        .as_mut_ptr()
        .offset(VP8TopLeftI4[0 as libc::c_int as usize] as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 17 as libc::c_int {
        (*it)
            .i4_boundary_[i
            as usize] = *((*it).y_left_).offset((15 as libc::c_int - i) as isize);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*it)
            .i4_boundary_[(17 as libc::c_int + i)
            as usize] = *((*it).y_top_).offset(i as isize);
        i += 1;
        i;
    }
    if (*it).x_ < (*enc).mb_w_ - 1 as libc::c_int {
        i = 16 as libc::c_int;
        while i < 16 as libc::c_int + 4 as libc::c_int {
            (*it)
                .i4_boundary_[(17 as libc::c_int + i)
                as usize] = *((*it).y_top_).offset(i as isize);
            i += 1;
            i;
        }
    } else {
        i = 16 as libc::c_int;
        while i < 16 as libc::c_int + 4 as libc::c_int {
            (*it)
                .i4_boundary_[(17 as libc::c_int + i)
                as usize] = (*it)
                .i4_boundary_[(17 as libc::c_int + 15 as libc::c_int) as usize];
            i += 1;
            i;
        }
    }
    VP8IteratorNzToBytes(it);
}
#[no_mangle]
pub unsafe extern "C" fn VP8IteratorRotateI4(
    it: *mut VP8EncIterator,
    yuv_out: *const uint8_t,
) -> libc::c_int {
    let blk: *const uint8_t = yuv_out
        .offset(VP8Scan[(*it).i4_ as usize] as libc::c_int as isize);
    let top: *mut uint8_t = (*it).i4_top_;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 3 as libc::c_int {
        *top
            .offset(
                (-(4 as libc::c_int) + i) as isize,
            ) = *blk.offset((i + 3 as libc::c_int * 32 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if (*it).i4_ & 3 as libc::c_int != 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= 2 as libc::c_int {
            *top
                .offset(
                    i as isize,
                ) = *blk
                .offset(
                    (3 as libc::c_int + (2 as libc::c_int - i) * 32 as libc::c_int)
                        as isize,
                );
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i <= 3 as libc::c_int {
            *top.offset(i as isize) = *top.offset((i + 4 as libc::c_int) as isize);
            i += 1;
            i;
        }
    }
    (*it).i4_ += 1;
    (*it).i4_;
    if (*it).i4_ == 16 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*it)
        .i4_top_ = ((*it).i4_boundary_)
        .as_mut_ptr()
        .offset(VP8TopLeftI4[(*it).i4_ as usize] as libc::c_int as isize);
    return 1 as libc::c_int;
}
