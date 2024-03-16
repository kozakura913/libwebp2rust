use ::libc;

use super::alpha_dec::ALPHDecoder;
extern "C" {
    fn VP8GetValue(br: *mut VP8BitReader, num_bits: libc::c_int) -> uint32_t;
    fn VP8GetSignedValue(br: *mut VP8BitReader, num_bits: libc::c_int) -> int32_t;
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
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[inline]
unsafe extern "C" fn clip(mut v: libc::c_int, mut M: libc::c_int) -> libc::c_int {
    return if v < 0 as libc::c_int { 0 as libc::c_int } else if v > M { M } else { v };
}
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
#[no_mangle]
pub unsafe extern "C" fn VP8ParseQuant(dec: *mut VP8Decoder) {
    let br: *mut VP8BitReader = &mut (*dec).br_;
    let base_q0: libc::c_int = VP8GetValue(br, 7 as libc::c_int) as libc::c_int;
    let dqy1_dc: libc::c_int = if VP8GetValue(br, 1 as libc::c_int) != 0 {
        VP8GetSignedValue(br, 4 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let dqy2_dc: libc::c_int = if VP8GetValue(br, 1 as libc::c_int) != 0 {
        VP8GetSignedValue(br, 4 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let dqy2_ac: libc::c_int = if VP8GetValue(br, 1 as libc::c_int) != 0 {
        VP8GetSignedValue(br, 4 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let dquv_dc: libc::c_int = if VP8GetValue(br, 1 as libc::c_int) != 0 {
        VP8GetSignedValue(br, 4 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let dquv_ac: libc::c_int = if VP8GetValue(br, 1 as libc::c_int) != 0 {
        VP8GetSignedValue(br, 4 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let hdr: *const VP8SegmentHeader = &mut (*dec).segment_hdr_;
    let mut i: libc::c_int = 0;
    let mut current_block_18: u64;
    i = 0 as libc::c_int;
    while i < NUM_MB_SEGMENTS as libc::c_int {
        let mut q: libc::c_int = 0;
        if (*hdr).use_segment_ != 0 {
            q = (*hdr).quantizer_[i as usize] as libc::c_int;
            if (*hdr).absolute_delta_ == 0 {
                q += base_q0;
            }
            current_block_18 = 10048703153582371463;
        } else if i > 0 as libc::c_int {
            (*dec).dqm_[i as usize] = (*dec).dqm_[0 as libc::c_int as usize];
            current_block_18 = 17179679302217393232;
        } else {
            q = base_q0;
            current_block_18 = 10048703153582371463;
        }
        match current_block_18 {
            10048703153582371463 => {
                let m: *mut VP8QuantMatrix = &mut *((*dec).dqm_)
                    .as_mut_ptr()
                    .offset(i as isize) as *mut VP8QuantMatrix;
                (*m)
                    .y1_mat_[0 as libc::c_int
                    as usize] = kDcTable[clip(q + dqy1_dc, 127 as libc::c_int) as usize]
                    as libc::c_int;
                (*m)
                    .y1_mat_[1 as libc::c_int
                    as usize] = kAcTable[clip(q + 0 as libc::c_int, 127 as libc::c_int)
                    as usize] as libc::c_int;
                (*m)
                    .y2_mat_[0 as libc::c_int
                    as usize] = kDcTable[clip(q + dqy2_dc, 127 as libc::c_int) as usize]
                    as libc::c_int * 2 as libc::c_int;
                (*m)
                    .y2_mat_[1 as libc::c_int
                    as usize] = kAcTable[clip(q + dqy2_ac, 127 as libc::c_int) as usize]
                    as libc::c_int * 101581 as libc::c_int >> 16 as libc::c_int;
                if (*m).y2_mat_[1 as libc::c_int as usize] < 8 as libc::c_int {
                    (*m).y2_mat_[1 as libc::c_int as usize] = 8 as libc::c_int;
                }
                (*m)
                    .uv_mat_[0 as libc::c_int
                    as usize] = kDcTable[clip(q + dquv_dc, 117 as libc::c_int) as usize]
                    as libc::c_int;
                (*m)
                    .uv_mat_[1 as libc::c_int
                    as usize] = kAcTable[clip(q + dquv_ac, 127 as libc::c_int) as usize]
                    as libc::c_int;
                (*m).uv_quant_ = q + dquv_ac;
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
