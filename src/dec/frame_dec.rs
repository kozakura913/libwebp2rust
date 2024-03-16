use ::libc;

use super::alpha_dec::ALPHDecoder;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VP8DecompressAlphaRows(
        dec: *mut VP8Decoder,
        io: *const VP8Io,
        row: libc::c_int,
        num_rows: libc::c_int,
    ) -> *const uint8_t;
    fn VP8InitScanline(dec: *mut VP8Decoder);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VP8SetError(
        dec: *mut VP8Decoder,
        error: VP8StatusCode,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn VP8InitRandom(rg: *mut VP8Random, dithering: libc::c_float);
    static mut VP8Transform: VP8DecIdct2;
    static mut VP8TransformAC3: VP8DecIdct;
    static mut VP8TransformUV: VP8DecIdct;
    static mut VP8TransformDC: VP8DecIdct;
    static mut VP8TransformDCUV: VP8DecIdct;
    static mut VP8PredLuma16: [VP8PredFunc; 0];
    static mut VP8PredChroma8: [VP8PredFunc; 0];
    static mut VP8PredLuma4: [VP8PredFunc; 0];
    static mut VP8SimpleVFilter16: VP8SimpleFilterFunc;
    static mut VP8SimpleHFilter16: VP8SimpleFilterFunc;
    static mut VP8SimpleVFilter16i: VP8SimpleFilterFunc;
    static mut VP8SimpleHFilter16i: VP8SimpleFilterFunc;
    static mut VP8VFilter16: VP8LumaFilterFunc;
    static mut VP8HFilter16: VP8LumaFilterFunc;
    static mut VP8VFilter8: VP8ChromaFilterFunc;
    static mut VP8HFilter8: VP8ChromaFilterFunc;
    static mut VP8VFilter16i: VP8LumaFilterFunc;
    static mut VP8HFilter16i: VP8LumaFilterFunc;
    static mut VP8VFilter8i: VP8ChromaFilterFunc;
    static mut VP8HFilter8i: VP8ChromaFilterFunc;
    static mut VP8DitherCombine8x8: Option::<
        unsafe extern "C" fn(*const uint8_t, *mut uint8_t, libc::c_int) -> (),
    >;
    fn VP8DspInit();
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
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
pub type VP8DecIdct = Option::<unsafe extern "C" fn(*const int16_t, *mut uint8_t) -> ()>;
pub type VP8DecIdct2 = Option::<
    unsafe extern "C" fn(*const int16_t, *mut uint8_t, libc::c_int) -> (),
>;
pub type VP8PredFunc = Option::<unsafe extern "C" fn(*mut uint8_t) -> ()>;
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
pub struct WebPWorkerInterface {
    pub Init: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Reset: Option::<unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int>,
    pub Sync_0: Option::<unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int>,
    pub Launch: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Execute: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub End: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
}
#[inline]
unsafe extern "C" fn VP8RandomBits2(
    rg: *mut VP8Random,
    mut num_bits: libc::c_int,
    mut amp: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    diff = ((*rg).tab_[(*rg).index1_ as usize])
        .wrapping_sub((*rg).tab_[(*rg).index2_ as usize]) as libc::c_int;
    if diff < 0 as libc::c_int {
        diff = (diff as libc::c_uint)
            .wrapping_add((1 as libc::c_uint) << 31 as libc::c_int) as libc::c_int
            as libc::c_int;
    }
    (*rg).tab_[(*rg).index1_ as usize] = diff as uint32_t;
    (*rg).index1_ += 1;
    if (*rg).index1_ == 55 as libc::c_int {
        (*rg).index1_ = 0 as libc::c_int;
    }
    (*rg).index2_ += 1;
    if (*rg).index2_ == 55 as libc::c_int {
        (*rg).index2_ = 0 as libc::c_int;
    }
    diff = ((diff as uint32_t) << 1 as libc::c_int) as libc::c_int
        >> 32 as libc::c_int - num_bits;
    diff = diff * amp >> 8 as libc::c_int;
    diff += (1 as libc::c_int) << num_bits - 1 as libc::c_int;
    return diff;
}
#[inline]
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> libc::c_int {
    return (size == size) as libc::c_int;
}
static mut kScan: [uint16_t; 16] = [
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
unsafe extern "C" fn CheckMode(
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    if mode == B_DC_PRED as libc::c_int {
        if mb_x == 0 as libc::c_int {
            return if mb_y == 0 as libc::c_int {
                B_DC_PRED_NOTOPLEFT as libc::c_int
            } else {
                B_DC_PRED_NOLEFT as libc::c_int
            }
        } else {
            return if mb_y == 0 as libc::c_int {
                B_DC_PRED_NOTOP as libc::c_int
            } else {
                B_DC_PRED as libc::c_int
            }
        }
    }
    return mode;
}
unsafe extern "C" fn Copy32b(dst: *mut uint8_t, src: *const uint8_t) {
    memcpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn DoTransform(
    mut bits: uint32_t,
    src: *const int16_t,
    dst: *mut uint8_t,
) {
    match bits >> 30 as libc::c_int {
        3 => {
            VP8Transform.expect("non-null function pointer")(src, dst, 0 as libc::c_int);
        }
        2 => {
            VP8TransformAC3.expect("non-null function pointer")(src, dst);
        }
        1 => {
            VP8TransformDC.expect("non-null function pointer")(src, dst);
        }
        _ => {}
    };
}
unsafe extern "C" fn DoUVTransform(
    mut bits: uint32_t,
    src: *const int16_t,
    dst: *mut uint8_t,
) {
    if bits & 0xff as libc::c_int as libc::c_uint != 0 {
        if bits & 0xaa as libc::c_int as libc::c_uint != 0 {
            VP8TransformUV.expect("non-null function pointer")(src, dst);
        } else {
            VP8TransformDCUV.expect("non-null function pointer")(src, dst);
        }
    }
}
unsafe extern "C" fn ReconstructRow(
    dec: *const VP8Decoder,
    mut ctx: *const VP8ThreadContext,
) {
    let mut j: libc::c_int = 0;
    let mut mb_x: libc::c_int = 0;
    let mb_y: libc::c_int = (*ctx).mb_y_;
    let cache_id: libc::c_int = (*ctx).id_;
    let y_dst: *mut uint8_t = ((*dec).yuv_b_)
        .offset((32 as libc::c_int * 1 as libc::c_int + 8 as libc::c_int) as isize);
    let u_dst: *mut uint8_t = ((*dec).yuv_b_)
        .offset(
            (32 as libc::c_int * 1 as libc::c_int + 8 as libc::c_int
                + 32 as libc::c_int * 16 as libc::c_int + 32 as libc::c_int) as isize,
        );
    let v_dst: *mut uint8_t = ((*dec).yuv_b_)
        .offset(
            (32 as libc::c_int * 1 as libc::c_int + 8 as libc::c_int
                + 32 as libc::c_int * 16 as libc::c_int + 32 as libc::c_int
                + 16 as libc::c_int) as isize,
        );
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        *y_dst
            .offset(
                (j * 32 as libc::c_int - 1 as libc::c_int) as isize,
            ) = 129 as libc::c_int as uint8_t;
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        *u_dst
            .offset(
                (j * 32 as libc::c_int - 1 as libc::c_int) as isize,
            ) = 129 as libc::c_int as uint8_t;
        *v_dst
            .offset(
                (j * 32 as libc::c_int - 1 as libc::c_int) as isize,
            ) = 129 as libc::c_int as uint8_t;
        j += 1;
        j;
    }
    if mb_y > 0 as libc::c_int {
        let ref mut fresh0 = *v_dst
            .offset((-(1 as libc::c_int) - 32 as libc::c_int) as isize);
        *fresh0 = 129 as libc::c_int as uint8_t;
        let ref mut fresh1 = *u_dst
            .offset((-(1 as libc::c_int) - 32 as libc::c_int) as isize);
        *fresh1 = *fresh0;
        *y_dst.offset((-(1 as libc::c_int) - 32 as libc::c_int) as isize) = *fresh1;
    } else {
        memset(
            y_dst
                .offset(-(32 as libc::c_int as isize))
                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            127 as libc::c_int,
            (16 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        memset(
            u_dst
                .offset(-(32 as libc::c_int as isize))
                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            127 as libc::c_int,
            (8 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
        memset(
            v_dst
                .offset(-(32 as libc::c_int as isize))
                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            127 as libc::c_int,
            (8 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
    }
    mb_x = 0 as libc::c_int;
    while mb_x < (*dec).mb_w_ {
        let block: *const VP8MBData = ((*ctx).mb_data_).offset(mb_x as isize);
        if mb_x > 0 as libc::c_int {
            j = -(1 as libc::c_int);
            while j < 16 as libc::c_int {
                Copy32b(
                    &mut *y_dst
                        .offset((j * 32 as libc::c_int - 4 as libc::c_int) as isize),
                    &mut *y_dst
                        .offset((j * 32 as libc::c_int + 12 as libc::c_int) as isize),
                );
                j += 1;
                j;
            }
            j = -(1 as libc::c_int);
            while j < 8 as libc::c_int {
                Copy32b(
                    &mut *u_dst
                        .offset((j * 32 as libc::c_int - 4 as libc::c_int) as isize),
                    &mut *u_dst
                        .offset((j * 32 as libc::c_int + 4 as libc::c_int) as isize),
                );
                Copy32b(
                    &mut *v_dst
                        .offset((j * 32 as libc::c_int - 4 as libc::c_int) as isize),
                    &mut *v_dst
                        .offset((j * 32 as libc::c_int + 4 as libc::c_int) as isize),
                );
                j += 1;
                j;
            }
        }
        let top_yuv: *mut VP8TopSamples = ((*dec).yuv_t_).offset(mb_x as isize);
        let coeffs: *const int16_t = ((*block).coeffs_).as_ptr();
        let mut bits: uint32_t = (*block).non_zero_y_;
        let mut n: libc::c_int = 0;
        if mb_y > 0 as libc::c_int {
            memcpy(
                y_dst.offset(-(32 as libc::c_int as isize)) as *mut libc::c_void,
                ((*top_yuv.offset(0 as libc::c_int as isize)).y).as_mut_ptr()
                    as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                u_dst.offset(-(32 as libc::c_int as isize)) as *mut libc::c_void,
                ((*top_yuv.offset(0 as libc::c_int as isize)).u).as_mut_ptr()
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                v_dst.offset(-(32 as libc::c_int as isize)) as *mut libc::c_void,
                ((*top_yuv.offset(0 as libc::c_int as isize)).v).as_mut_ptr()
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
        }
        if (*block).is_i4x4_ != 0 {
            let top_right: *mut uint32_t = y_dst
                .offset(-(32 as libc::c_int as isize))
                .offset(16 as libc::c_int as isize) as *mut uint32_t;
            if mb_y > 0 as libc::c_int {
                if mb_x >= (*dec).mb_w_ - 1 as libc::c_int {
                    memset(
                        top_right as *mut libc::c_void,
                        (*top_yuv.offset(0 as libc::c_int as isize))
                            .y[15 as libc::c_int as usize] as libc::c_int,
                        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                    );
                } else {
                    memcpy(
                        top_right as *mut libc::c_void,
                        ((*top_yuv.offset(1 as libc::c_int as isize)).y).as_mut_ptr()
                            as *const libc::c_void,
                        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                    );
                }
            }
            let ref mut fresh2 = *top_right
                .offset((3 as libc::c_int * 32 as libc::c_int) as isize);
            *fresh2 = *top_right.offset(0 as libc::c_int as isize);
            let ref mut fresh3 = *top_right
                .offset((2 as libc::c_int * 32 as libc::c_int) as isize);
            *fresh3 = *fresh2;
            *top_right.offset(32 as libc::c_int as isize) = *fresh3;
            n = 0 as libc::c_int;
            while n < 16 as libc::c_int {
                let dst: *mut uint8_t = y_dst
                    .offset(kScan[n as usize] as libc::c_int as isize);
                (*VP8PredLuma4
                    .as_mut_ptr()
                    .offset((*block).imodes_[n as usize] as isize))
                    .expect("non-null function pointer")(dst);
                DoTransform(bits, coeffs.offset((n * 16 as libc::c_int) as isize), dst);
                n += 1;
                n;
                bits <<= 2 as libc::c_int;
            }
        } else {
            let pred_func: libc::c_int = CheckMode(
                mb_x,
                mb_y,
                (*block).imodes_[0 as libc::c_int as usize] as libc::c_int,
            );
            (*VP8PredLuma16.as_mut_ptr().offset(pred_func as isize))
                .expect("non-null function pointer")(y_dst);
            if bits != 0 as libc::c_int as libc::c_uint {
                n = 0 as libc::c_int;
                while n < 16 as libc::c_int {
                    DoTransform(
                        bits,
                        coeffs.offset((n * 16 as libc::c_int) as isize),
                        y_dst.offset(kScan[n as usize] as libc::c_int as isize),
                    );
                    n += 1;
                    n;
                    bits <<= 2 as libc::c_int;
                }
            }
        }
        let bits_uv: uint32_t = (*block).non_zero_uv_;
        let pred_func_0: libc::c_int = CheckMode(
            mb_x,
            mb_y,
            (*block).uvmode_ as libc::c_int,
        );
        (*VP8PredChroma8.as_mut_ptr().offset(pred_func_0 as isize))
            .expect("non-null function pointer")(u_dst);
        (*VP8PredChroma8.as_mut_ptr().offset(pred_func_0 as isize))
            .expect("non-null function pointer")(v_dst);
        DoUVTransform(
            bits_uv >> 0 as libc::c_int,
            coeffs.offset((16 as libc::c_int * 16 as libc::c_int) as isize),
            u_dst,
        );
        DoUVTransform(
            bits_uv >> 8 as libc::c_int,
            coeffs.offset((20 as libc::c_int * 16 as libc::c_int) as isize),
            v_dst,
        );
        if mb_y < (*dec).mb_h_ - 1 as libc::c_int {
            memcpy(
                ((*top_yuv.offset(0 as libc::c_int as isize)).y).as_mut_ptr()
                    as *mut libc::c_void,
                y_dst.offset((15 as libc::c_int * 32 as libc::c_int) as isize)
                    as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                ((*top_yuv.offset(0 as libc::c_int as isize)).u).as_mut_ptr()
                    as *mut libc::c_void,
                u_dst.offset((7 as libc::c_int * 32 as libc::c_int) as isize)
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                ((*top_yuv.offset(0 as libc::c_int as isize)).v).as_mut_ptr()
                    as *mut libc::c_void,
                v_dst.offset((7 as libc::c_int * 32 as libc::c_int) as isize)
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
        }
        let y_offset: libc::c_int = cache_id * 16 as libc::c_int
            * (*dec).cache_y_stride_;
        let uv_offset: libc::c_int = cache_id * 8 as libc::c_int
            * (*dec).cache_uv_stride_;
        let y_out: *mut uint8_t = ((*dec).cache_y_)
            .offset((mb_x * 16 as libc::c_int) as isize)
            .offset(y_offset as isize);
        let u_out: *mut uint8_t = ((*dec).cache_u_)
            .offset((mb_x * 8 as libc::c_int) as isize)
            .offset(uv_offset as isize);
        let v_out: *mut uint8_t = ((*dec).cache_v_)
            .offset((mb_x * 8 as libc::c_int) as isize)
            .offset(uv_offset as isize);
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            memcpy(
                y_out.offset((j * (*dec).cache_y_stride_) as isize) as *mut libc::c_void,
                y_dst.offset((j * 32 as libc::c_int) as isize) as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            );
            j += 1;
            j;
        }
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            memcpy(
                u_out.offset((j * (*dec).cache_uv_stride_) as isize)
                    as *mut libc::c_void,
                u_dst.offset((j * 32 as libc::c_int) as isize) as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                v_out.offset((j * (*dec).cache_uv_stride_) as isize)
                    as *mut libc::c_void,
                v_dst.offset((j * 32 as libc::c_int) as isize) as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            j += 1;
            j;
        }
        mb_x += 1;
        mb_x;
    }
}
static mut kFilterExtraRows: [uint8_t; 3] = [
    0 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
];
unsafe extern "C" fn DoFilter(
    dec: *const VP8Decoder,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) {
    let ctx: *const VP8ThreadContext = &(*dec).thread_ctx_;
    let cache_id: libc::c_int = (*ctx).id_;
    let y_bps: libc::c_int = (*dec).cache_y_stride_;
    let f_info: *const VP8FInfo = ((*ctx).f_info_).offset(mb_x as isize);
    let y_dst: *mut uint8_t = ((*dec).cache_y_)
        .offset((cache_id * 16 as libc::c_int * y_bps) as isize)
        .offset((mb_x * 16 as libc::c_int) as isize);
    let ilevel: libc::c_int = (*f_info).f_ilevel_ as libc::c_int;
    let limit: libc::c_int = (*f_info).f_limit_ as libc::c_int;
    if limit == 0 as libc::c_int {
        return;
    }
    if (*dec).filter_type_ == 1 as libc::c_int {
        if mb_x > 0 as libc::c_int {
            VP8SimpleHFilter16
                .expect(
                    "non-null function pointer",
                )(y_dst, y_bps, limit + 4 as libc::c_int);
        }
        if (*f_info).f_inner_ != 0 {
            VP8SimpleHFilter16i.expect("non-null function pointer")(y_dst, y_bps, limit);
        }
        if mb_y > 0 as libc::c_int {
            VP8SimpleVFilter16
                .expect(
                    "non-null function pointer",
                )(y_dst, y_bps, limit + 4 as libc::c_int);
        }
        if (*f_info).f_inner_ != 0 {
            VP8SimpleVFilter16i.expect("non-null function pointer")(y_dst, y_bps, limit);
        }
    } else {
        let uv_bps: libc::c_int = (*dec).cache_uv_stride_;
        let u_dst: *mut uint8_t = ((*dec).cache_u_)
            .offset((cache_id * 8 as libc::c_int * uv_bps) as isize)
            .offset((mb_x * 8 as libc::c_int) as isize);
        let v_dst: *mut uint8_t = ((*dec).cache_v_)
            .offset((cache_id * 8 as libc::c_int * uv_bps) as isize)
            .offset((mb_x * 8 as libc::c_int) as isize);
        let hev_thresh: libc::c_int = (*f_info).hev_thresh_ as libc::c_int;
        if mb_x > 0 as libc::c_int {
            VP8HFilter16
                .expect(
                    "non-null function pointer",
                )(y_dst, y_bps, limit + 4 as libc::c_int, ilevel, hev_thresh);
            VP8HFilter8
                .expect(
                    "non-null function pointer",
                )(u_dst, v_dst, uv_bps, limit + 4 as libc::c_int, ilevel, hev_thresh);
        }
        if (*f_info).f_inner_ != 0 {
            VP8HFilter16i
                .expect(
                    "non-null function pointer",
                )(y_dst, y_bps, limit, ilevel, hev_thresh);
            VP8HFilter8i
                .expect(
                    "non-null function pointer",
                )(u_dst, v_dst, uv_bps, limit, ilevel, hev_thresh);
        }
        if mb_y > 0 as libc::c_int {
            VP8VFilter16
                .expect(
                    "non-null function pointer",
                )(y_dst, y_bps, limit + 4 as libc::c_int, ilevel, hev_thresh);
            VP8VFilter8
                .expect(
                    "non-null function pointer",
                )(u_dst, v_dst, uv_bps, limit + 4 as libc::c_int, ilevel, hev_thresh);
        }
        if (*f_info).f_inner_ != 0 {
            VP8VFilter16i
                .expect(
                    "non-null function pointer",
                )(y_dst, y_bps, limit, ilevel, hev_thresh);
            VP8VFilter8i
                .expect(
                    "non-null function pointer",
                )(u_dst, v_dst, uv_bps, limit, ilevel, hev_thresh);
        }
    };
}
unsafe extern "C" fn FilterRow(dec: *const VP8Decoder) {
    let mut mb_x: libc::c_int = 0;
    let mb_y: libc::c_int = (*dec).thread_ctx_.mb_y_;
    mb_x = (*dec).tl_mb_x_;
    while mb_x < (*dec).br_mb_x_ {
        DoFilter(dec, mb_x, mb_y);
        mb_x += 1;
        mb_x;
    }
}
unsafe extern "C" fn PrecomputeFilterStrengths(dec: *mut VP8Decoder) {
    if (*dec).filter_type_ > 0 as libc::c_int {
        let mut s: libc::c_int = 0;
        let hdr: *const VP8FilterHeader = &mut (*dec).filter_hdr_;
        s = 0 as libc::c_int;
        while s < NUM_MB_SEGMENTS as libc::c_int {
            let mut i4x4: libc::c_int = 0;
            let mut base_level: libc::c_int = 0;
            if (*dec).segment_hdr_.use_segment_ != 0 {
                base_level = (*dec).segment_hdr_.filter_strength_[s as usize]
                    as libc::c_int;
                if (*dec).segment_hdr_.absolute_delta_ == 0 {
                    base_level += (*hdr).level_;
                }
            } else {
                base_level = (*hdr).level_;
            }
            i4x4 = 0 as libc::c_int;
            while i4x4 <= 1 as libc::c_int {
                let info: *mut VP8FInfo = &mut *(*((*dec).fstrengths_)
                    .as_mut_ptr()
                    .offset(s as isize))
                    .as_mut_ptr()
                    .offset(i4x4 as isize) as *mut VP8FInfo;
                let mut level: libc::c_int = base_level;
                if (*hdr).use_lf_delta_ != 0 {
                    level += (*hdr).ref_lf_delta_[0 as libc::c_int as usize];
                    if i4x4 != 0 {
                        level += (*hdr).mode_lf_delta_[0 as libc::c_int as usize];
                    }
                }
                level = if level < 0 as libc::c_int {
                    0 as libc::c_int
                } else if level > 63 as libc::c_int {
                    63 as libc::c_int
                } else {
                    level
                };
                if level > 0 as libc::c_int {
                    let mut ilevel: libc::c_int = level;
                    if (*hdr).sharpness_ > 0 as libc::c_int {
                        if (*hdr).sharpness_ > 4 as libc::c_int {
                            ilevel >>= 2 as libc::c_int;
                        } else {
                            ilevel >>= 1 as libc::c_int;
                        }
                        if ilevel > 9 as libc::c_int - (*hdr).sharpness_ {
                            ilevel = 9 as libc::c_int - (*hdr).sharpness_;
                        }
                    }
                    if ilevel < 1 as libc::c_int {
                        ilevel = 1 as libc::c_int;
                    }
                    (*info).f_ilevel_ = ilevel as uint8_t;
                    (*info).f_limit_ = (2 as libc::c_int * level + ilevel) as uint8_t;
                    (*info)
                        .hev_thresh_ = (if level >= 40 as libc::c_int {
                        2 as libc::c_int
                    } else if level >= 15 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as uint8_t;
                } else {
                    (*info).f_limit_ = 0 as libc::c_int as uint8_t;
                }
                (*info).f_inner_ = i4x4 as uint8_t;
                i4x4 += 1;
                i4x4;
            }
            s += 1;
            s;
        }
    }
}
static mut kQuantToDitherAmp: [uint8_t; 12] = [
    8 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8InitDithering(
    options: *const WebPDecoderOptions,
    dec: *mut VP8Decoder,
) {
    if !options.is_null() {
        let d: libc::c_int = (*options).dithering_strength;
        let max_amp: libc::c_int = ((1 as libc::c_int) << 8 as libc::c_int)
            - 1 as libc::c_int;
        let f: libc::c_int = if d < 0 as libc::c_int {
            0 as libc::c_int
        } else if d > 100 as libc::c_int {
            max_amp
        } else {
            d * max_amp / 100 as libc::c_int
        };
        if f > 0 as libc::c_int {
            let mut s: libc::c_int = 0;
            let mut all_amp: libc::c_int = 0 as libc::c_int;
            s = 0 as libc::c_int;
            while s < NUM_MB_SEGMENTS as libc::c_int {
                let dqm: *mut VP8QuantMatrix = &mut *((*dec).dqm_)
                    .as_mut_ptr()
                    .offset(s as isize) as *mut VP8QuantMatrix;
                if (*dqm).uv_quant_ < 12 as libc::c_int {
                    let idx: libc::c_int = if (*dqm).uv_quant_ < 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        (*dqm).uv_quant_
                    };
                    (*dqm)
                        .dither_ = f * kQuantToDitherAmp[idx as usize] as libc::c_int
                        >> 3 as libc::c_int;
                }
                all_amp |= (*dqm).dither_;
                s += 1;
                s;
            }
            if all_amp != 0 as libc::c_int {
                VP8InitRandom(&mut (*dec).dithering_rg_, 1.0f32);
                (*dec).dither_ = 1 as libc::c_int;
            }
        }
        (*dec).alpha_dithering_ = (*options).alpha_dithering_strength;
        if (*dec).alpha_dithering_ > 100 as libc::c_int {
            (*dec).alpha_dithering_ = 100 as libc::c_int;
        } else if (*dec).alpha_dithering_ < 0 as libc::c_int {
            (*dec).alpha_dithering_ = 0 as libc::c_int;
        }
    }
}
unsafe extern "C" fn Dither8x8(
    rg: *mut VP8Random,
    mut dst: *mut uint8_t,
    mut bps: libc::c_int,
    mut amp: libc::c_int,
) {
    let mut dither: [uint8_t; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int * 8 as libc::c_int {
        dither[i
            as usize] = VP8RandomBits2(rg, 7 as libc::c_int + 1 as libc::c_int, amp)
            as uint8_t;
        i += 1;
        i;
    }
    VP8DitherCombine8x8
        .expect("non-null function pointer")(dither.as_mut_ptr(), dst, bps);
}
unsafe extern "C" fn DitherRow(dec: *mut VP8Decoder) {
    let mut mb_x: libc::c_int = 0;
    mb_x = (*dec).tl_mb_x_;
    while mb_x < (*dec).br_mb_x_ {
        let ctx: *const VP8ThreadContext = &mut (*dec).thread_ctx_;
        let data: *const VP8MBData = ((*ctx).mb_data_).offset(mb_x as isize);
        let cache_id: libc::c_int = (*ctx).id_;
        let uv_bps: libc::c_int = (*dec).cache_uv_stride_;
        if (*data).dither_ as libc::c_int >= 4 as libc::c_int {
            let u_dst: *mut uint8_t = ((*dec).cache_u_)
                .offset((cache_id * 8 as libc::c_int * uv_bps) as isize)
                .offset((mb_x * 8 as libc::c_int) as isize);
            let v_dst: *mut uint8_t = ((*dec).cache_v_)
                .offset((cache_id * 8 as libc::c_int * uv_bps) as isize)
                .offset((mb_x * 8 as libc::c_int) as isize);
            Dither8x8(
                &mut (*dec).dithering_rg_,
                u_dst,
                uv_bps,
                (*data).dither_ as libc::c_int,
            );
            Dither8x8(
                &mut (*dec).dithering_rg_,
                v_dst,
                uv_bps,
                (*data).dither_ as libc::c_int,
            );
        }
        mb_x += 1;
        mb_x;
    }
}
unsafe extern "C" fn FinishRow(
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
) -> libc::c_int {
    let dec: *mut VP8Decoder = arg1 as *mut VP8Decoder;
    let io: *mut VP8Io = arg2 as *mut VP8Io;
    let mut ok: libc::c_int = 1 as libc::c_int;
    let ctx: *const VP8ThreadContext = &mut (*dec).thread_ctx_;
    let cache_id: libc::c_int = (*ctx).id_;
    let extra_y_rows: libc::c_int = kFilterExtraRows[(*dec).filter_type_ as usize]
        as libc::c_int;
    let ysize: libc::c_int = extra_y_rows * (*dec).cache_y_stride_;
    let uvsize: libc::c_int = extra_y_rows / 2 as libc::c_int * (*dec).cache_uv_stride_;
    let y_offset: libc::c_int = cache_id * 16 as libc::c_int * (*dec).cache_y_stride_;
    let uv_offset: libc::c_int = cache_id * 8 as libc::c_int * (*dec).cache_uv_stride_;
    let ydst: *mut uint8_t = ((*dec).cache_y_)
        .offset(-(ysize as isize))
        .offset(y_offset as isize);
    let udst: *mut uint8_t = ((*dec).cache_u_)
        .offset(-(uvsize as isize))
        .offset(uv_offset as isize);
    let vdst: *mut uint8_t = ((*dec).cache_v_)
        .offset(-(uvsize as isize))
        .offset(uv_offset as isize);
    let mb_y: libc::c_int = (*ctx).mb_y_;
    let is_first_row: libc::c_int = (mb_y == 0 as libc::c_int) as libc::c_int;
    let is_last_row: libc::c_int = (mb_y >= (*dec).br_mb_y_ - 1 as libc::c_int)
        as libc::c_int;
    if (*dec).mt_method_ == 2 as libc::c_int {
        ReconstructRow(dec, ctx);
    }
    if (*ctx).filter_row_ != 0 {
        FilterRow(dec);
    }
    if (*dec).dither_ != 0 {
        DitherRow(dec);
    }
    if ((*io).put).is_some() {
        let mut y_start: libc::c_int = mb_y * 16 as libc::c_int;
        let mut y_end: libc::c_int = (mb_y + 1 as libc::c_int) * 16 as libc::c_int;
        if is_first_row == 0 {
            y_start -= extra_y_rows;
            (*io).y = ydst;
            (*io).u = udst;
            (*io).v = vdst;
        } else {
            (*io).y = ((*dec).cache_y_).offset(y_offset as isize);
            (*io).u = ((*dec).cache_u_).offset(uv_offset as isize);
            (*io).v = ((*dec).cache_v_).offset(uv_offset as isize);
        }
        if is_last_row == 0 {
            y_end -= extra_y_rows;
        }
        if y_end > (*io).crop_bottom {
            y_end = (*io).crop_bottom;
        }
        (*io).a = 0 as *const uint8_t;
        if !((*dec).alpha_data_).is_null() && y_start < y_end {
            (*io).a = VP8DecompressAlphaRows(dec, io, y_start, y_end - y_start);
            if ((*io).a).is_null() {
                return VP8SetError(
                    dec,
                    VP8_STATUS_BITSTREAM_ERROR,
                    b"Could not decode alpha data.\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if y_start < (*io).crop_top {
            let delta_y: libc::c_int = (*io).crop_top - y_start;
            y_start = (*io).crop_top;
            (*io).y = ((*io).y).offset(((*dec).cache_y_stride_ * delta_y) as isize);
            (*io)
                .u = ((*io).u)
                .offset(
                    ((*dec).cache_uv_stride_ * (delta_y >> 1 as libc::c_int)) as isize,
                );
            (*io)
                .v = ((*io).v)
                .offset(
                    ((*dec).cache_uv_stride_ * (delta_y >> 1 as libc::c_int)) as isize,
                );
            if !((*io).a).is_null() {
                (*io).a = ((*io).a).offset(((*io).width * delta_y) as isize);
            }
        }
        if y_start < y_end {
            (*io).y = ((*io).y).offset((*io).crop_left as isize);
            (*io).u = ((*io).u).offset(((*io).crop_left >> 1 as libc::c_int) as isize);
            (*io).v = ((*io).v).offset(((*io).crop_left >> 1 as libc::c_int) as isize);
            if !((*io).a).is_null() {
                (*io).a = ((*io).a).offset((*io).crop_left as isize);
            }
            (*io).mb_y = y_start - (*io).crop_top;
            (*io).mb_w = (*io).crop_right - (*io).crop_left;
            (*io).mb_h = y_end - y_start;
            ok = ((*io).put).expect("non-null function pointer")(io);
        }
    }
    if cache_id + 1 as libc::c_int == (*dec).num_caches_ {
        if is_last_row == 0 {
            memcpy(
                ((*dec).cache_y_).offset(-(ysize as isize)) as *mut libc::c_void,
                ydst.offset((16 as libc::c_int * (*dec).cache_y_stride_) as isize)
                    as *const libc::c_void,
                ysize as libc::c_ulong,
            );
            memcpy(
                ((*dec).cache_u_).offset(-(uvsize as isize)) as *mut libc::c_void,
                udst.offset((8 as libc::c_int * (*dec).cache_uv_stride_) as isize)
                    as *const libc::c_void,
                uvsize as libc::c_ulong,
            );
            memcpy(
                ((*dec).cache_v_).offset(-(uvsize as isize)) as *mut libc::c_void,
                vdst.offset((8 as libc::c_int * (*dec).cache_uv_stride_) as isize)
                    as *const libc::c_void,
                uvsize as libc::c_ulong,
            );
        }
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn VP8ProcessRow(
    dec: *mut VP8Decoder,
    io: *mut VP8Io,
) -> libc::c_int {
    let mut ok: libc::c_int = 1 as libc::c_int;
    let ctx: *mut VP8ThreadContext = &mut (*dec).thread_ctx_;
    let filter_row: libc::c_int = ((*dec).filter_type_ > 0 as libc::c_int
        && (*dec).mb_y_ >= (*dec).tl_mb_y_ && (*dec).mb_y_ <= (*dec).br_mb_y_)
        as libc::c_int;
    if (*dec).mt_method_ == 0 as libc::c_int {
        (*ctx).mb_y_ = (*dec).mb_y_;
        (*ctx).filter_row_ = filter_row;
        ReconstructRow(dec, ctx);
        ok = FinishRow(dec as *mut libc::c_void, io as *mut libc::c_void);
    } else {
        let worker: *mut WebPWorker = &mut (*dec).worker_;
        ok
            &= ((*WebPGetWorkerInterface()).Sync_0)
                .expect("non-null function pointer")(worker);
        if ok != 0 {
            (*ctx).io_ = *io;
            (*ctx).id_ = (*dec).cache_id_;
            (*ctx).mb_y_ = (*dec).mb_y_;
            (*ctx).filter_row_ = filter_row;
            if (*dec).mt_method_ == 2 as libc::c_int {
                let tmp: *mut VP8MBData = (*ctx).mb_data_;
                (*ctx).mb_data_ = (*dec).mb_data_;
                (*dec).mb_data_ = tmp;
            } else {
                ReconstructRow(dec, ctx);
            }
            if filter_row != 0 {
                let tmp_0: *mut VP8FInfo = (*ctx).f_info_;
                (*ctx).f_info_ = (*dec).f_info_;
                (*dec).f_info_ = tmp_0;
            }
            ((*WebPGetWorkerInterface()).Launch)
                .expect("non-null function pointer")(worker);
            (*dec).cache_id_ += 1;
            if (*dec).cache_id_ == (*dec).num_caches_ {
                (*dec).cache_id_ = 0 as libc::c_int;
            }
        }
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EnterCritical(
    dec: *mut VP8Decoder,
    io: *mut VP8Io,
) -> VP8StatusCode {
    if ((*io).setup).is_some()
        && ((*io).setup).expect("non-null function pointer")(io) == 0
    {
        VP8SetError(
            dec,
            VP8_STATUS_USER_ABORT,
            b"Frame setup failed\0" as *const u8 as *const libc::c_char,
        );
        return (*dec).status_;
    }
    if (*io).bypass_filtering != 0 {
        (*dec).filter_type_ = 0 as libc::c_int;
    }
    let extra_pixels: libc::c_int = kFilterExtraRows[(*dec).filter_type_ as usize]
        as libc::c_int;
    if (*dec).filter_type_ == 2 as libc::c_int {
        (*dec).tl_mb_x_ = 0 as libc::c_int;
        (*dec).tl_mb_y_ = 0 as libc::c_int;
    } else {
        (*dec).tl_mb_x_ = (*io).crop_left - extra_pixels >> 4 as libc::c_int;
        (*dec).tl_mb_y_ = (*io).crop_top - extra_pixels >> 4 as libc::c_int;
        if (*dec).tl_mb_x_ < 0 as libc::c_int {
            (*dec).tl_mb_x_ = 0 as libc::c_int;
        }
        if (*dec).tl_mb_y_ < 0 as libc::c_int {
            (*dec).tl_mb_y_ = 0 as libc::c_int;
        }
    }
    (*dec)
        .br_mb_y_ = (*io).crop_bottom + 15 as libc::c_int + extra_pixels
        >> 4 as libc::c_int;
    (*dec)
        .br_mb_x_ = (*io).crop_right + 15 as libc::c_int + extra_pixels
        >> 4 as libc::c_int;
    if (*dec).br_mb_x_ > (*dec).mb_w_ {
        (*dec).br_mb_x_ = (*dec).mb_w_;
    }
    if (*dec).br_mb_y_ > (*dec).mb_h_ {
        (*dec).br_mb_y_ = (*dec).mb_h_;
    }
    PrecomputeFilterStrengths(dec);
    return VP8_STATUS_OK;
}
#[no_mangle]
pub unsafe extern "C" fn VP8ExitCritical(
    dec: *mut VP8Decoder,
    io: *mut VP8Io,
) -> libc::c_int {
    let mut ok: libc::c_int = 1 as libc::c_int;
    if (*dec).mt_method_ > 0 as libc::c_int {
        ok = ((*WebPGetWorkerInterface()).Sync_0)
            .expect("non-null function pointer")(&mut (*dec).worker_);
    }
    if ((*io).teardown).is_some() {
        ((*io).teardown).expect("non-null function pointer")(io);
    }
    return ok;
}
unsafe extern "C" fn InitThreadContext(dec: *mut VP8Decoder) -> libc::c_int {
    (*dec).cache_id_ = 0 as libc::c_int;
    if (*dec).mt_method_ > 0 as libc::c_int {
        let worker: *mut WebPWorker = &mut (*dec).worker_;
        if ((*WebPGetWorkerInterface()).Reset)
            .expect("non-null function pointer")(worker) == 0
        {
            return VP8SetError(
                dec,
                VP8_STATUS_OUT_OF_MEMORY,
                b"thread initialization failed.\0" as *const u8 as *const libc::c_char,
            );
        }
        (*worker).data1 = dec as *mut libc::c_void;
        (*worker).data2 = &mut (*dec).thread_ctx_.io_ as *mut VP8Io as *mut libc::c_void;
        (*worker)
            .hook = Some(
            FinishRow
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        );
        (*dec)
            .num_caches_ = if (*dec).filter_type_ > 0 as libc::c_int {
            3 as libc::c_int
        } else {
            3 as libc::c_int - 1 as libc::c_int
        };
    } else {
        (*dec).num_caches_ = 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetThreadMethod(
    options: *const WebPDecoderOptions,
    headers: *const WebPHeaderStructure,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    if options.is_null() || (*options).use_threads == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if width >= 512 as libc::c_int {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn AllocateMemory(dec: *mut VP8Decoder) -> libc::c_int {
    let num_caches: libc::c_int = (*dec).num_caches_;
    let mb_w: libc::c_int = (*dec).mb_w_;
    let intra_pred_mode_size: size_t = ((4 as libc::c_int * mb_w) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong);
    let top_size: size_t = (::core::mem::size_of::<VP8TopSamples>() as libc::c_ulong)
        .wrapping_mul(mb_w as libc::c_ulong);
    let mb_info_size: size_t = ((mb_w + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<VP8MB>() as libc::c_ulong);
    let f_info_size: size_t = if (*dec).filter_type_ > 0 as libc::c_int {
        ((mb_w
            * (if (*dec).mt_method_ > 0 as libc::c_int {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            })) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<VP8FInfo>() as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let yuv_size: size_t = ((32 as libc::c_int * 17 as libc::c_int
        + 32 as libc::c_int * 9 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong);
    let mb_data_size: size_t = (((if (*dec).mt_method_ == 2 as libc::c_int {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    }) * mb_w) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<VP8MBData>() as libc::c_ulong);
    let cache_height: size_t = ((16 as libc::c_int * num_caches
        + kFilterExtraRows[(*dec).filter_type_ as usize] as libc::c_int)
        * 3 as libc::c_int / 2 as libc::c_int) as size_t;
    let cache_size: size_t = top_size.wrapping_mul(cache_height);
    let alpha_size: uint64_t = (if !((*dec).alpha_data_).is_null() {
        ((*dec).pic_hdr_.width_ as uint64_t)
            .wrapping_mul((*dec).pic_hdr_.height_ as libc::c_ulong) as libc::c_ulonglong
    } else {
        0 as libc::c_ulonglong
    }) as uint64_t;
    let needed: uint64_t = intra_pred_mode_size
        .wrapping_add(top_size)
        .wrapping_add(mb_info_size)
        .wrapping_add(f_info_size)
        .wrapping_add(yuv_size)
        .wrapping_add(mb_data_size)
        .wrapping_add(cache_size)
        .wrapping_add(alpha_size)
        .wrapping_add(31 as libc::c_int as libc::c_ulong);
    let mut mem: *mut uint8_t = 0 as *mut uint8_t;
    if CheckSizeOverflow(needed) == 0 {
        return 0 as libc::c_int;
    }
    if needed > (*dec).mem_size_ {
        WebPSafeFree((*dec).mem_);
        (*dec).mem_size_ = 0 as libc::c_int as size_t;
        (*dec)
            .mem_ = WebPSafeMalloc(
            needed,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        if ((*dec).mem_).is_null() {
            return VP8SetError(
                dec,
                VP8_STATUS_OUT_OF_MEMORY,
                b"no memory during frame initialization.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*dec).mem_size_ = needed;
    }
    mem = (*dec).mem_ as *mut uint8_t;
    (*dec).intra_t_ = mem;
    mem = mem.offset(intra_pred_mode_size as isize);
    (*dec).yuv_t_ = mem as *mut VP8TopSamples;
    mem = mem.offset(top_size as isize);
    (*dec).mb_info_ = (mem as *mut VP8MB).offset(1 as libc::c_int as isize);
    mem = mem.offset(mb_info_size as isize);
    (*dec)
        .f_info_ = if f_info_size != 0 {
        mem as *mut VP8FInfo
    } else {
        0 as *mut VP8FInfo
    };
    mem = mem.offset(f_info_size as isize);
    (*dec).thread_ctx_.id_ = 0 as libc::c_int;
    (*dec).thread_ctx_.f_info_ = (*dec).f_info_;
    if (*dec).filter_type_ > 0 as libc::c_int && (*dec).mt_method_ > 0 as libc::c_int {
        (*dec).thread_ctx_.f_info_ = ((*dec).thread_ctx_.f_info_).offset(mb_w as isize);
    }
    mem = ((mem as uintptr_t).wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut uint8_t;
    (*dec).yuv_b_ = mem;
    mem = mem.offset(yuv_size as isize);
    (*dec).mb_data_ = mem as *mut VP8MBData;
    (*dec).thread_ctx_.mb_data_ = mem as *mut VP8MBData;
    if (*dec).mt_method_ == 2 as libc::c_int {
        (*dec)
            .thread_ctx_
            .mb_data_ = ((*dec).thread_ctx_.mb_data_).offset(mb_w as isize);
    }
    mem = mem.offset(mb_data_size as isize);
    (*dec).cache_y_stride_ = 16 as libc::c_int * mb_w;
    (*dec).cache_uv_stride_ = 8 as libc::c_int * mb_w;
    let extra_rows: libc::c_int = kFilterExtraRows[(*dec).filter_type_ as usize]
        as libc::c_int;
    let extra_y: libc::c_int = extra_rows * (*dec).cache_y_stride_;
    let extra_uv: libc::c_int = extra_rows / 2 as libc::c_int * (*dec).cache_uv_stride_;
    (*dec).cache_y_ = mem.offset(extra_y as isize);
    (*dec)
        .cache_u_ = ((*dec).cache_y_)
        .offset((16 as libc::c_int * num_caches * (*dec).cache_y_stride_) as isize)
        .offset(extra_uv as isize);
    (*dec)
        .cache_v_ = ((*dec).cache_u_)
        .offset((8 as libc::c_int * num_caches * (*dec).cache_uv_stride_) as isize)
        .offset(extra_uv as isize);
    (*dec).cache_id_ = 0 as libc::c_int;
    mem = mem.offset(cache_size as isize);
    (*dec).alpha_plane_ = if alpha_size != 0 { mem } else { 0 as *mut uint8_t };
    mem = mem.offset(alpha_size as isize);
    memset(
        ((*dec).mb_info_).offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
        0 as libc::c_int,
        mb_info_size,
    );
    VP8InitScanline(dec);
    memset(
        (*dec).intra_t_ as *mut libc::c_void,
        B_DC_PRED as libc::c_int,
        intra_pred_mode_size,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn InitIo(dec: *mut VP8Decoder, mut io: *mut VP8Io) {
    (*io).mb_y = 0 as libc::c_int;
    (*io).y = (*dec).cache_y_;
    (*io).u = (*dec).cache_u_;
    (*io).v = (*dec).cache_v_;
    (*io).y_stride = (*dec).cache_y_stride_;
    (*io).uv_stride = (*dec).cache_uv_stride_;
    (*io).a = 0 as *const uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitFrame(
    dec: *mut VP8Decoder,
    io: *mut VP8Io,
) -> libc::c_int {
    if InitThreadContext(dec) == 0 {
        return 0 as libc::c_int;
    }
    if AllocateMemory(dec) == 0 {
        return 0 as libc::c_int;
    }
    InitIo(dec, io);
    VP8DspInit();
    return 1 as libc::c_int;
}
