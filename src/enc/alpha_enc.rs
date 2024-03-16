use ::libc;
use ::c2rust_bitfields;

use super::token_enc::VP8Tokens;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut WebPDispatchAlphaToGreen: Option::<
        unsafe extern "C" fn(
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut uint32_t,
            libc::c_int,
        ) -> (),
    >;
    fn WebPInitAlphaProcessing();
    static mut WebPFilters: [WebPFilterFunc; 4];
    fn VP8FiltersInit();
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPCopyPlane(
        src: *const uint8_t,
        src_stride: libc::c_int,
        dst: *mut uint8_t,
        dst_stride: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );
    fn WebPConfigInitInternal(
        _: *mut WebPConfig,
        _: WebPPreset,
        _: libc::c_float,
        _: libc::c_int,
    ) -> libc::c_int;
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: libc::c_int) -> libc::c_int;
    fn WebPPictureAlloc(picture: *mut WebPPicture) -> libc::c_int;
    fn WebPPictureFree(picture: *mut WebPPicture);
    fn WebPPictureHasTransparency(picture: *const WebPPicture) -> libc::c_int;
    fn VP8BitWriterWipeOut(bw: *mut VP8BitWriter);
    fn VP8BitWriterAppend(
        bw: *mut VP8BitWriter,
        data: *const uint8_t,
        size: size_t,
    ) -> libc::c_int;
    fn VP8BitWriterInit(bw: *mut VP8BitWriter, expected_size: size_t) -> libc::c_int;
    fn VP8LBitWriterInit(bw: *mut VP8LBitWriter, expected_size: size_t) -> libc::c_int;
    fn VP8LBitWriterFinish(bw: *mut VP8LBitWriter) -> *mut uint8_t;
    fn VP8LBitWriterWipeOut(bw: *mut VP8LBitWriter);
    fn WebPEstimateBestFilter(
        data: *const uint8_t,
        width: libc::c_int,
        height: libc::c_int,
        stride: libc::c_int,
    ) -> WEBP_FILTER_TYPE;
    fn QuantizeLevels(
        data: *mut uint8_t,
        width: libc::c_int,
        height: libc::c_int,
        num_levels: libc::c_int,
        sse: *mut uint64_t,
    ) -> libc::c_int;
    fn VP8LEncodeStream(
        config: *const WebPConfig,
        picture: *const WebPPicture,
        bw: *mut VP8LBitWriter,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Matrix {
    pub q_: [uint16_t; 16],
    pub iq_: [uint16_t; 16],
    pub bias_: [uint32_t; 16],
    pub zthresh_: [uint32_t; 16],
    pub sharpen_: [uint16_t; 16],
}
pub type WEBP_FILTER_TYPE = libc::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
pub type WebPFilterFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
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
pub type vp8l_atype_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitWriter {
    pub bits_: vp8l_atype_t,
    pub used_: libc::c_int,
    pub buf_: *mut uint8_t,
    pub cur_: *mut uint8_t,
    pub end_: *mut uint8_t,
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
pub type WebPPreset = libc::c_uint;
pub const WEBP_PRESET_TEXT: WebPPreset = 5;
pub const WEBP_PRESET_ICON: WebPPreset = 4;
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
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
pub struct FilterTrial {
    pub score: size_t,
    pub bw: VP8BitWriter,
    pub stats: WebPAuxStats,
}
#[inline]
unsafe extern "C" fn WebPConfigInit(mut config: *mut WebPConfig) -> libc::c_int {
    return WebPConfigInitInternal(
        config,
        WEBP_PRESET_DEFAULT,
        75.0f32,
        0x20f as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> libc::c_int {
    return WebPPictureInitInternal(picture, 0x20f as libc::c_int);
}
#[inline]
unsafe extern "C" fn VP8LBitWriterNumBytes(bw: *const VP8LBitWriter) -> size_t {
    return (((*bw).cur_).offset_from((*bw).buf_) as libc::c_long
        + ((*bw).used_ + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_long)
        as size_t;
}
#[inline]
unsafe extern "C" fn VP8BitWriterSize(bw: *const VP8BitWriter) -> size_t {
    return (*bw).pos_;
}
#[inline]
unsafe extern "C" fn VP8BitWriterBuf(bw: *const VP8BitWriter) -> *mut uint8_t {
    return (*bw).buf_;
}
unsafe extern "C" fn EncodeLossless(
    data: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut effort_level: libc::c_int,
    mut use_quality_100: libc::c_int,
    bw: *mut VP8LBitWriter,
    stats: *mut WebPAuxStats,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut config: WebPConfig = WebPConfig {
        lossless: 0,
        quality: 0.,
        method: 0,
        image_hint: WEBP_HINT_DEFAULT,
        target_size: 0,
        target_PSNR: 0.,
        segments: 0,
        sns_strength: 0,
        filter_strength: 0,
        filter_sharpness: 0,
        filter_type: 0,
        autofilter: 0,
        alpha_compression: 0,
        alpha_filtering: 0,
        alpha_quality: 0,
        pass: 0,
        show_compressed: 0,
        preprocessing: 0,
        partitions: 0,
        partition_limit: 0,
        emulate_jpeg_size: 0,
        thread_level: 0,
        low_memory: 0,
        near_lossless: 0,
        exact: 0,
        use_delta_palette: 0,
        use_sharp_yuv: 0,
        qmin: 0,
        qmax: 0,
    };
    let mut picture: WebPPicture = WebPPicture {
        use_argb: 0,
        colorspace: WEBP_YUV420,
        width: 0,
        height: 0,
        y: 0 as *mut uint8_t,
        u: 0 as *mut uint8_t,
        v: 0 as *mut uint8_t,
        y_stride: 0,
        uv_stride: 0,
        a: 0 as *mut uint8_t,
        a_stride: 0,
        pad1: [0; 2],
        argb: 0 as *mut uint32_t,
        argb_stride: 0,
        pad2: [0; 3],
        writer: None,
        custom_ptr: 0 as *mut libc::c_void,
        extra_info_type: 0,
        extra_info: 0 as *mut uint8_t,
        stats: 0 as *mut WebPAuxStats,
        error_code: VP8_ENC_OK,
        progress_hook: None,
        user_data: 0 as *mut libc::c_void,
        pad3: [0; 3],
        pad4: 0 as *mut uint8_t,
        pad5: 0 as *mut uint8_t,
        pad6: [0; 8],
        memory_: 0 as *mut libc::c_void,
        memory_argb_: 0 as *mut libc::c_void,
        pad7: [0 as *mut libc::c_void; 2],
    };
    if WebPPictureInit(&mut picture) == 0 {
        return 0 as libc::c_int;
    }
    picture.width = width;
    picture.height = height;
    picture.use_argb = 1 as libc::c_int;
    picture.stats = stats;
    if WebPPictureAlloc(&mut picture) == 0 {
        return 0 as libc::c_int;
    }
    WebPDispatchAlphaToGreen
        .expect(
            "non-null function pointer",
        )(data, width, picture.width, picture.height, picture.argb, picture.argb_stride);
    if WebPConfigInit(&mut config) == 0 {
        return 0 as libc::c_int;
    }
    config.lossless = 1 as libc::c_int;
    config.exact = 1 as libc::c_int;
    config.method = effort_level;
    config
        .quality = if use_quality_100 != 0 && effort_level == 6 as libc::c_int {
        100 as libc::c_int as libc::c_float
    } else {
        8.0f32 * effort_level as libc::c_float
    };
    ok = VP8LEncodeStream(&mut config, &mut picture, bw);
    WebPPictureFree(&mut picture);
    ok = (ok != 0 && (*bw).error_ == 0) as libc::c_int;
    if ok == 0 {
        VP8LBitWriterWipeOut(bw);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn EncodeAlphaInternal(
    data: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut method: libc::c_int,
    mut filter: libc::c_int,
    mut reduce_levels: libc::c_int,
    mut effort_level: libc::c_int,
    tmp_alpha: *mut uint8_t,
    mut result: *mut FilterTrial,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut alpha_src: *const uint8_t = 0 as *const uint8_t;
    let mut filter_func: WebPFilterFunc = None;
    let mut header: uint8_t = 0;
    let data_size: size_t = (width * height) as size_t;
    let mut output: *const uint8_t = 0 as *const uint8_t;
    let mut output_size: size_t = 0 as libc::c_int as size_t;
    let mut tmp_bw: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: 0 as *mut uint8_t,
        cur_: 0 as *mut uint8_t,
        end_: 0 as *mut uint8_t,
        error_: 0,
    };
    filter_func = WebPFilters[filter as usize];
    if filter_func.is_some() {
        filter_func
            .expect("non-null function pointer")(data, width, height, width, tmp_alpha);
        alpha_src = tmp_alpha;
    } else {
        alpha_src = data;
    }
    if method != 0 as libc::c_int {
        ok = VP8LBitWriterInit(&mut tmp_bw, data_size >> 3 as libc::c_int);
        ok = (ok != 0
            && EncodeLossless(
                alpha_src,
                width,
                height,
                effort_level,
                (reduce_levels == 0) as libc::c_int,
                &mut tmp_bw,
                &mut (*result).stats,
            ) != 0) as libc::c_int;
        if ok != 0 {
            output = VP8LBitWriterFinish(&mut tmp_bw);
            if tmp_bw.error_ != 0 {
                VP8LBitWriterWipeOut(&mut tmp_bw);
                memset(
                    &mut (*result).bw as *mut VP8BitWriter as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<VP8BitWriter>() as libc::c_ulong,
                );
                return 0 as libc::c_int;
            }
            output_size = VP8LBitWriterNumBytes(&mut tmp_bw);
            if output_size > data_size {
                method = 0 as libc::c_int;
                VP8LBitWriterWipeOut(&mut tmp_bw);
            }
        } else {
            VP8LBitWriterWipeOut(&mut tmp_bw);
            memset(
                &mut (*result).bw as *mut VP8BitWriter as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<VP8BitWriter>() as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
    }
    if method == 0 as libc::c_int {
        output = alpha_src;
        output_size = data_size;
        ok = 1 as libc::c_int;
    }
    header = (method | filter << 2 as libc::c_int) as uint8_t;
    if reduce_levels != 0 {
        header = (header as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
            as uint8_t;
    }
    if VP8BitWriterInit(
        &mut (*result).bw,
        (1 as libc::c_int as libc::c_ulong).wrapping_add(output_size),
    ) == 0
    {
        ok = 0 as libc::c_int;
    }
    ok = (ok != 0
        && VP8BitWriterAppend(&mut (*result).bw, &mut header, 1 as libc::c_int as size_t)
            != 0) as libc::c_int;
    ok = (ok != 0 && VP8BitWriterAppend(&mut (*result).bw, output, output_size) != 0)
        as libc::c_int;
    if method != 0 as libc::c_int {
        VP8LBitWriterWipeOut(&mut tmp_bw);
    }
    ok = (ok != 0 && (*result).bw.error_ == 0) as libc::c_int;
    (*result).score = VP8BitWriterSize(&mut (*result).bw);
    return ok;
}
unsafe extern "C" fn GetNumColors(
    mut data: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut colors: libc::c_int = 0 as libc::c_int;
    let mut color: [uint8_t; 256] = [
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
    j = 0 as libc::c_int;
    while j < height {
        let mut i: libc::c_int = 0;
        let p: *const uint8_t = data.offset((j * stride) as isize);
        i = 0 as libc::c_int;
        while i < width {
            color[*p.offset(i as isize) as usize] = 1 as libc::c_int as uint8_t;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < 256 as libc::c_int {
        if color[j as usize] as libc::c_int > 0 as libc::c_int {
            colors += 1;
            colors;
        }
        j += 1;
        j;
    }
    return colors;
}
unsafe extern "C" fn GetFilterMap(
    mut alpha: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut filter: libc::c_int,
    mut effort_level: libc::c_int,
) -> uint32_t {
    let mut bit_map: uint32_t = 0 as libc::c_uint;
    if filter == WEBP_FILTER_FAST as libc::c_int {
        let mut try_filter_none: libc::c_int = (effort_level > 3 as libc::c_int)
            as libc::c_int;
        let kMinColorsForFilterNone: libc::c_int = 16 as libc::c_int;
        let kMaxColorsForFilterNone: libc::c_int = 192 as libc::c_int;
        let num_colors: libc::c_int = GetNumColors(alpha, width, height, width);
        filter = (if num_colors <= kMinColorsForFilterNone {
            WEBP_FILTER_NONE as libc::c_int as libc::c_uint
        } else {
            WebPEstimateBestFilter(alpha, width, height, width) as libc::c_uint
        }) as libc::c_int;
        bit_map |= ((1 as libc::c_int) << filter) as libc::c_uint;
        if try_filter_none != 0 || num_colors > kMaxColorsForFilterNone {
            bit_map
                |= ((1 as libc::c_int) << WEBP_FILTER_NONE as libc::c_int)
                    as libc::c_uint;
        }
    } else if filter == WEBP_FILTER_NONE as libc::c_int {
        bit_map = ((1 as libc::c_int) << WEBP_FILTER_NONE as libc::c_int) as uint32_t;
    } else {
        bit_map = (((1 as libc::c_int) << WEBP_FILTER_LAST as libc::c_int)
            - 1 as libc::c_int) as uint32_t;
    }
    return bit_map;
}
unsafe extern "C" fn InitFilterTrial(score: *mut FilterTrial) {
    (*score).score = !(0 as libc::c_uint) as size_t;
    VP8BitWriterInit(&mut (*score).bw, 0 as libc::c_int as size_t);
}
unsafe extern "C" fn ApplyFiltersAndEncode(
    mut alpha: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut data_size: size_t,
    mut method: libc::c_int,
    mut filter: libc::c_int,
    mut reduce_levels: libc::c_int,
    mut effort_level: libc::c_int,
    output: *mut *mut uint8_t,
    output_size: *mut size_t,
    stats: *mut WebPAuxStats,
) -> libc::c_int {
    let mut ok: libc::c_int = 1 as libc::c_int;
    let mut best: FilterTrial = FilterTrial {
        score: 0,
        bw: VP8BitWriter {
            range_: 0,
            value_: 0,
            run_: 0,
            nb_bits_: 0,
            buf_: 0 as *mut uint8_t,
            pos_: 0,
            max_pos_: 0,
            error_: 0,
        },
        stats: WebPAuxStats {
            coded_size: 0,
            PSNR: [0.; 5],
            block_count: [0; 3],
            header_bytes: [0; 2],
            residual_bytes: [[0; 4]; 3],
            segment_size: [0; 4],
            segment_quant: [0; 4],
            segment_level: [0; 4],
            alpha_data_size: 0,
            layer_data_size: 0,
            lossless_features: 0,
            histogram_bits: 0,
            transform_bits: 0,
            cache_bits: 0,
            palette_size: 0,
            lossless_size: 0,
            lossless_hdr_size: 0,
            lossless_data_size: 0,
            pad: [0; 2],
        },
    };
    let mut try_map: uint32_t = GetFilterMap(alpha, width, height, filter, effort_level);
    InitFilterTrial(&mut best);
    if try_map != ((1 as libc::c_int) << WEBP_FILTER_NONE as libc::c_int) as libc::c_uint
    {
        let mut filtered_alpha: *mut uint8_t = WebPSafeMalloc(
            1 as libc::c_ulonglong as uint64_t,
            data_size,
        ) as *mut uint8_t;
        if filtered_alpha.is_null() {
            return 0 as libc::c_int;
        }
        filter = WEBP_FILTER_NONE as libc::c_int;
        while ok != 0 && try_map != 0 {
            if try_map & 1 as libc::c_int as libc::c_uint != 0 {
                let mut trial: FilterTrial = FilterTrial {
                    score: 0,
                    bw: VP8BitWriter {
                        range_: 0,
                        value_: 0,
                        run_: 0,
                        nb_bits_: 0,
                        buf_: 0 as *mut uint8_t,
                        pos_: 0,
                        max_pos_: 0,
                        error_: 0,
                    },
                    stats: WebPAuxStats {
                        coded_size: 0,
                        PSNR: [0.; 5],
                        block_count: [0; 3],
                        header_bytes: [0; 2],
                        residual_bytes: [[0; 4]; 3],
                        segment_size: [0; 4],
                        segment_quant: [0; 4],
                        segment_level: [0; 4],
                        alpha_data_size: 0,
                        layer_data_size: 0,
                        lossless_features: 0,
                        histogram_bits: 0,
                        transform_bits: 0,
                        cache_bits: 0,
                        palette_size: 0,
                        lossless_size: 0,
                        lossless_hdr_size: 0,
                        lossless_data_size: 0,
                        pad: [0; 2],
                    },
                };
                ok = EncodeAlphaInternal(
                    alpha,
                    width,
                    height,
                    method,
                    filter,
                    reduce_levels,
                    effort_level,
                    filtered_alpha,
                    &mut trial,
                );
                if ok != 0 && trial.score < best.score {
                    VP8BitWriterWipeOut(&mut best.bw);
                    best = trial;
                } else {
                    VP8BitWriterWipeOut(&mut trial.bw);
                }
            }
            filter += 1;
            filter;
            try_map >>= 1 as libc::c_int;
        }
        WebPSafeFree(filtered_alpha as *mut libc::c_void);
    } else {
        ok = EncodeAlphaInternal(
            alpha,
            width,
            height,
            method,
            WEBP_FILTER_NONE as libc::c_int,
            reduce_levels,
            effort_level,
            0 as *mut uint8_t,
            &mut best,
        );
    }
    if ok != 0 {
        if !stats.is_null() {
            (*stats).lossless_features = best.stats.lossless_features;
            (*stats).histogram_bits = best.stats.histogram_bits;
            (*stats).transform_bits = best.stats.transform_bits;
            (*stats).cache_bits = best.stats.cache_bits;
            (*stats).palette_size = best.stats.palette_size;
            (*stats).lossless_size = best.stats.lossless_size;
            (*stats).lossless_hdr_size = best.stats.lossless_hdr_size;
            (*stats).lossless_data_size = best.stats.lossless_data_size;
        }
        *output_size = VP8BitWriterSize(&mut best.bw);
        *output = VP8BitWriterBuf(&mut best.bw);
    } else {
        VP8BitWriterWipeOut(&mut best.bw);
    }
    return ok;
}
unsafe extern "C" fn EncodeAlpha(
    enc: *mut VP8Encoder,
    mut quality: libc::c_int,
    mut method: libc::c_int,
    mut filter: libc::c_int,
    mut effort_level: libc::c_int,
    output: *mut *mut uint8_t,
    output_size: *mut size_t,
) -> libc::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: libc::c_int = (*pic).width;
    let height: libc::c_int = (*pic).height;
    let mut quant_alpha: *mut uint8_t = 0 as *mut uint8_t;
    let data_size: size_t = (width * height) as size_t;
    let mut sse: uint64_t = 0 as libc::c_int as uint64_t;
    let mut ok: libc::c_int = 1 as libc::c_int;
    let reduce_levels: libc::c_int = (quality < 100 as libc::c_int) as libc::c_int;
    if quality < 0 as libc::c_int || quality > 100 as libc::c_int {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    if method < 0 as libc::c_int || method > 1 as libc::c_int {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_INVALID_CONFIGURATION);
    }
    if method == 0 as libc::c_int {
        filter = WEBP_FILTER_NONE as libc::c_int;
    }
    quant_alpha = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, data_size)
        as *mut uint8_t;
    if quant_alpha.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    WebPCopyPlane((*pic).a, (*pic).a_stride, quant_alpha, width, width, height);
    if reduce_levels != 0 {
        let alpha_levels: libc::c_int = if quality <= 70 as libc::c_int {
            2 as libc::c_int + quality / 5 as libc::c_int
        } else {
            16 as libc::c_int + (quality - 70 as libc::c_int) * 8 as libc::c_int
        };
        ok = QuantizeLevels(quant_alpha, width, height, alpha_levels, &mut sse);
    }
    if ok != 0 {
        VP8FiltersInit();
        ok = ApplyFiltersAndEncode(
            quant_alpha,
            width,
            height,
            data_size,
            method,
            filter,
            reduce_levels,
            effort_level,
            output,
            output_size,
            (*pic).stats,
        );
        if ok == 0 {
            WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        if !((*pic).stats).is_null() {
            (*(*pic).stats).coded_size += *output_size as libc::c_int;
            (*enc).sse_[3 as libc::c_int as usize] = sse;
        }
    }
    WebPSafeFree(quant_alpha as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn CompressAlphaJob(
    mut arg1: *mut libc::c_void,
    mut unused: *mut libc::c_void,
) -> libc::c_int {
    let enc: *mut VP8Encoder = arg1 as *mut VP8Encoder;
    let mut config: *const WebPConfig = (*enc).config_;
    let mut alpha_data: *mut uint8_t = 0 as *mut uint8_t;
    let mut alpha_size: size_t = 0 as libc::c_int as size_t;
    let effort_level: libc::c_int = (*config).method;
    let filter: WEBP_FILTER_TYPE = (if (*config).alpha_filtering == 0 as libc::c_int {
        WEBP_FILTER_NONE as libc::c_int
    } else if (*config).alpha_filtering == 1 as libc::c_int {
        WEBP_FILTER_FAST as libc::c_int
    } else {
        WEBP_FILTER_BEST as libc::c_int
    }) as WEBP_FILTER_TYPE;
    if EncodeAlpha(
        enc,
        (*config).alpha_quality,
        (*config).alpha_compression,
        filter as libc::c_int,
        effort_level,
        &mut alpha_data,
        &mut alpha_size,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if alpha_size != alpha_size as uint32_t as libc::c_ulong {
        WebPSafeFree(alpha_data as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    (*enc).alpha_data_size_ = alpha_size as uint32_t;
    (*enc).alpha_data_ = alpha_data;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncInitAlpha(enc: *mut VP8Encoder) {
    WebPInitAlphaProcessing();
    (*enc).has_alpha_ = WebPPictureHasTransparency((*enc).pic_);
    (*enc).alpha_data_ = 0 as *mut uint8_t;
    (*enc).alpha_data_size_ = 0 as libc::c_int as uint32_t;
    if (*enc).thread_level_ > 0 as libc::c_int {
        let worker: *mut WebPWorker = &mut (*enc).alpha_worker_;
        ((*WebPGetWorkerInterface()).Init).expect("non-null function pointer")(worker);
        (*worker).data1 = enc as *mut libc::c_void;
        (*worker).data2 = 0 as *mut libc::c_void;
        (*worker)
            .hook = Some(
            CompressAlphaJob
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncStartAlpha(enc: *mut VP8Encoder) -> libc::c_int {
    if (*enc).has_alpha_ != 0 {
        if (*enc).thread_level_ > 0 as libc::c_int {
            let worker: *mut WebPWorker = &mut (*enc).alpha_worker_;
            if ((*WebPGetWorkerInterface()).Reset)
                .expect("non-null function pointer")(worker) == 0
            {
                return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
            }
            ((*WebPGetWorkerInterface()).Launch)
                .expect("non-null function pointer")(worker);
            return 1 as libc::c_int;
        } else {
            return CompressAlphaJob(enc as *mut libc::c_void, 0 as *mut libc::c_void)
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncFinishAlpha(enc: *mut VP8Encoder) -> libc::c_int {
    if (*enc).has_alpha_ != 0 {
        if (*enc).thread_level_ > 0 as libc::c_int {
            let worker: *mut WebPWorker = &mut (*enc).alpha_worker_;
            if ((*WebPGetWorkerInterface()).Sync_0)
                .expect("non-null function pointer")(worker) == 0
            {
                return 0 as libc::c_int;
            }
        }
    }
    return WebPReportProgress(
        (*enc).pic_,
        (*enc).percent_ + 20 as libc::c_int,
        &mut (*enc).percent_,
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8EncDeleteAlpha(enc: *mut VP8Encoder) -> libc::c_int {
    let mut ok: libc::c_int = 1 as libc::c_int;
    if (*enc).thread_level_ > 0 as libc::c_int {
        let worker: *mut WebPWorker = &mut (*enc).alpha_worker_;
        ok = ((*WebPGetWorkerInterface()).Sync_0)
            .expect("non-null function pointer")(worker);
        ((*WebPGetWorkerInterface()).End).expect("non-null function pointer")(worker);
    }
    WebPSafeFree((*enc).alpha_data_ as *mut libc::c_void);
    (*enc).alpha_data_ = 0 as *mut uint8_t;
    (*enc).alpha_data_size_ = 0 as libc::c_int as uint32_t;
    (*enc).has_alpha_ = 0 as libc::c_int;
    return ok;
}
