use ::libc;
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
    fn VP8LHistogramInit(
        p: *mut VP8LHistogram,
        palette_code_bits: libc::c_int,
        init_arrays: libc::c_int,
    );
    fn VP8LAllocateHistogram(cache_bits: libc::c_int) -> *mut VP8LHistogram;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn VP8LFreeHistogram(histo: *mut VP8LHistogram);
    fn VP8LHistogramEstimateBits(p: *mut VP8LHistogram) -> libc::c_float;
    fn VP8LHistogramCreate(
        p: *mut VP8LHistogram,
        refs: *const VP8LBackwardRefs,
        palette_code_bits: libc::c_int,
    );
    static mut VP8LVectorMismatch: VP8LVectorMismatchFunc;
    static kPrefixEncodeCode: [VP8LPrefixCode; 512];
    static kPrefixEncodeExtraBitsValue: [uint8_t; 512];
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn VP8LColorCacheInit(
        color_cache: *mut VP8LColorCache,
        hash_bits: libc::c_int,
    ) -> libc::c_int;
    fn VP8LColorCacheClear(color_cache: *mut VP8LColorCache);
    fn VP8LBackwardReferencesTraceBackwards(
        xsize: libc::c_int,
        ysize: libc::c_int,
        argb: *const uint32_t,
        cache_bits: libc::c_int,
        hash_chain: *const VP8LHashChain,
        refs_src: *const VP8LBackwardRefs,
        refs_dst: *mut VP8LBackwardRefs,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type Mode = libc::c_uint;
pub const kNone: Mode = 3;
pub const kCopy: Mode = 2;
pub const kCacheIdx: Mode = 1;
pub const kLiteral: Mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PixOrCopy {
    pub mode: uint8_t,
    pub len: uint16_t,
    pub argb_or_distance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHashChain {
    pub offset_length_: *mut uint32_t,
    pub size_: libc::c_int,
}
pub type VP8LVectorMismatchFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, *const uint32_t, libc::c_int) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PixOrCopyBlock {
    pub next_: *mut PixOrCopyBlock,
    pub start_: *mut PixOrCopy,
    pub size_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBackwardRefs {
    pub block_size_: libc::c_int,
    pub error_: libc::c_int,
    pub refs_: *mut PixOrCopyBlock,
    pub tail_: *mut *mut PixOrCopyBlock,
    pub free_blocks_: *mut PixOrCopyBlock,
    pub last_block_: *mut PixOrCopyBlock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LRefsCursor {
    pub cur_pos: *mut PixOrCopy,
    pub cur_block_: *mut PixOrCopyBlock,
    pub last_pos_: *const PixOrCopy,
}
pub type VP8LLZ77Type = libc::c_uint;
pub const kLZ77Box: VP8LLZ77Type = 4;
pub const kLZ77RLE: VP8LLZ77Type = 2;
pub const kLZ77Standard: VP8LLZ77Type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LHistogram {
    pub literal_: *mut uint32_t,
    pub red_: [uint32_t; 256],
    pub blue_: [uint32_t; 256],
    pub alpha_: [uint32_t; 256],
    pub distance_: [uint32_t; 40],
    pub palette_code_bits_: libc::c_int,
    pub trivial_symbol_: uint32_t,
    pub bit_cost_: libc::c_float,
    pub literal_cost_: libc::c_float,
    pub red_cost_: libc::c_float,
    pub blue_cost_: libc::c_float,
    pub is_used_: [uint8_t; 5],
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
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorNext(c: *mut VP8LRefsCursor) {
    (*c).cur_pos = ((*c).cur_pos).offset(1);
    if (*c).cur_pos == (*c).last_pos_ as *mut PixOrCopy {
        VP8LRefsCursorNextBlock(c);
    }
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorOk(c: *const VP8LRefsCursor) -> libc::c_int {
    return ((*c).cur_pos != 0 as *mut libc::c_void as *mut PixOrCopy) as libc::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyCreateCopy(
    mut distance: uint32_t,
    mut len: uint16_t,
) -> PixOrCopy {
    let mut retval: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    retval.mode = kCopy as libc::c_int as uint8_t;
    retval.argb_or_distance = distance;
    retval.len = len;
    return retval;
}
#[inline]
unsafe extern "C" fn PixOrCopyCreateCacheIdx(mut idx: libc::c_int) -> PixOrCopy {
    let mut retval: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    retval.mode = kCacheIdx as libc::c_int as uint8_t;
    retval.argb_or_distance = idx as uint32_t;
    retval.len = 1 as libc::c_int as uint16_t;
    return retval;
}
#[inline]
unsafe extern "C" fn PixOrCopyCreateLiteral(mut argb: uint32_t) -> PixOrCopy {
    let mut retval: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    retval.mode = kLiteral as libc::c_int as uint8_t;
    retval.argb_or_distance = argb;
    retval.len = 1 as libc::c_int as uint16_t;
    return retval;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsLiteral(p: *const PixOrCopy) -> libc::c_int {
    return ((*p).mode as libc::c_int == kLiteral as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsCopy(p: *const PixOrCopy) -> libc::c_int {
    return ((*p).mode as libc::c_int == kCopy as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyLength(p: *const PixOrCopy) -> uint32_t {
    return (*p).len as uint32_t;
}
#[inline]
unsafe extern "C" fn VP8LHashChainFindCopy(
    p: *const VP8LHashChain,
    mut base_position: libc::c_int,
    offset_ptr: *mut libc::c_int,
    length_ptr: *mut libc::c_int,
) {
    *offset_ptr = VP8LHashChainFindOffset(p, base_position);
    *length_ptr = VP8LHashChainFindLength(p, base_position);
}
#[inline]
unsafe extern "C" fn VP8LHashChainFindLength(
    p: *const VP8LHashChain,
    base_position: libc::c_int,
) -> libc::c_int {
    return (*((*p).offset_length_).offset(base_position as isize)
        & ((1 as libc::c_uint) << 12 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)) as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8LHashChainFindOffset(
    p: *const VP8LHashChain,
    base_position: libc::c_int,
) -> libc::c_int {
    return (*((*p).offset_length_).offset(base_position as isize) >> 12 as libc::c_int)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> libc::c_int {
    return 31 as libc::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8LPrefixEncodeNoLUT(
    mut distance: libc::c_int,
    code: *mut libc::c_int,
    extra_bits: *mut libc::c_int,
    extra_bits_value: *mut libc::c_int,
) {
    distance -= 1;
    let highest_bit: libc::c_int = BitsLog2Floor(distance as uint32_t);
    let second_highest_bit: libc::c_int = distance >> highest_bit - 1 as libc::c_int
        & 1 as libc::c_int;
    *extra_bits = highest_bit - 1 as libc::c_int;
    *extra_bits_value = distance
        & ((1 as libc::c_int) << *extra_bits) - 1 as libc::c_int;
    *code = 2 as libc::c_int * highest_bit + second_highest_bit;
}
#[inline]
unsafe extern "C" fn VP8LPrefixEncode(
    mut distance: libc::c_int,
    code: *mut libc::c_int,
    extra_bits: *mut libc::c_int,
    extra_bits_value: *mut libc::c_int,
) {
    if distance < 512 as libc::c_int {
        let prefix_code: VP8LPrefixCode = kPrefixEncodeCode[distance as usize];
        *code = prefix_code.code_ as libc::c_int;
        *extra_bits = prefix_code.extra_bits_ as libc::c_int;
        *extra_bits_value = kPrefixEncodeExtraBitsValue[distance as usize]
            as libc::c_int;
    } else {
        VP8LPrefixEncodeNoLUT(distance, code, extra_bits, extra_bits_value);
    };
}
static mut kHashMul: uint32_t = 0x1e35a7bd as libc::c_uint;
#[inline]
unsafe extern "C" fn VP8LHashPix(
    mut argb: uint32_t,
    mut shift: libc::c_int,
) -> libc::c_int {
    return (argb.wrapping_mul(kHashMul) >> shift) as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8LColorCacheLookup(
    cc: *const VP8LColorCache,
    mut key: uint32_t,
) -> uint32_t {
    return *((*cc).colors_).offset(key as isize);
}
#[inline]
unsafe extern "C" fn VP8LColorCacheSet(
    cc: *const VP8LColorCache,
    mut key: uint32_t,
    mut argb: uint32_t,
) {
    *((*cc).colors_).offset(key as isize) = argb;
}
#[inline]
unsafe extern "C" fn VP8LColorCacheInsert(
    cc: *const VP8LColorCache,
    mut argb: uint32_t,
) {
    let key: libc::c_int = VP8LHashPix(argb, (*cc).hash_shift_);
    *((*cc).colors_).offset(key as isize) = argb;
}
#[inline]
unsafe extern "C" fn VP8LColorCacheGetIndex(
    cc: *const VP8LColorCache,
    mut argb: uint32_t,
) -> libc::c_int {
    return VP8LHashPix(argb, (*cc).hash_shift_);
}
#[inline]
unsafe extern "C" fn VP8LColorCacheContains(
    cc: *const VP8LColorCache,
    mut argb: uint32_t,
) -> libc::c_int {
    let key: libc::c_int = VP8LHashPix(argb, (*cc).hash_shift_);
    return if *((*cc).colors_).offset(key as isize) == argb {
        key
    } else {
        -(1 as libc::c_int)
    };
}
static mut plane_to_code_lut: [uint8_t; 128] = [
    96 as libc::c_int as uint8_t,
    73 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    101 as libc::c_int as uint8_t,
    78 as libc::c_int as uint8_t,
    58 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    59 as libc::c_int as uint8_t,
    79 as libc::c_int as uint8_t,
    102 as libc::c_int as uint8_t,
    86 as libc::c_int as uint8_t,
    62 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    63 as libc::c_int as uint8_t,
    87 as libc::c_int as uint8_t,
    105 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    70 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    71 as libc::c_int as uint8_t,
    91 as libc::c_int as uint8_t,
    110 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    82 as libc::c_int as uint8_t,
    66 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    67 as libc::c_int as uint8_t,
    83 as libc::c_int as uint8_t,
    100 as libc::c_int as uint8_t,
    115 as libc::c_int as uint8_t,
    108 as libc::c_int as uint8_t,
    94 as libc::c_int as uint8_t,
    76 as libc::c_int as uint8_t,
    64 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    65 as libc::c_int as uint8_t,
    77 as libc::c_int as uint8_t,
    95 as libc::c_int as uint8_t,
    109 as libc::c_int as uint8_t,
    118 as libc::c_int as uint8_t,
    113 as libc::c_int as uint8_t,
    103 as libc::c_int as uint8_t,
    92 as libc::c_int as uint8_t,
    80 as libc::c_int as uint8_t,
    68 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    56 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    57 as libc::c_int as uint8_t,
    61 as libc::c_int as uint8_t,
    69 as libc::c_int as uint8_t,
    81 as libc::c_int as uint8_t,
    93 as libc::c_int as uint8_t,
    104 as libc::c_int as uint8_t,
    114 as libc::c_int as uint8_t,
    119 as libc::c_int as uint8_t,
    116 as libc::c_int as uint8_t,
    111 as libc::c_int as uint8_t,
    106 as libc::c_int as uint8_t,
    97 as libc::c_int as uint8_t,
    88 as libc::c_int as uint8_t,
    84 as libc::c_int as uint8_t,
    74 as libc::c_int as uint8_t,
    72 as libc::c_int as uint8_t,
    75 as libc::c_int as uint8_t,
    85 as libc::c_int as uint8_t,
    89 as libc::c_int as uint8_t,
    98 as libc::c_int as uint8_t,
    107 as libc::c_int as uint8_t,
    112 as libc::c_int as uint8_t,
    117 as libc::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8LDistanceToPlaneCode(
    mut xsize: libc::c_int,
    mut dist: libc::c_int,
) -> libc::c_int {
    let yoffset: libc::c_int = dist / xsize;
    let xoffset: libc::c_int = dist - yoffset * xsize;
    if xoffset <= 8 as libc::c_int && yoffset < 8 as libc::c_int {
        return plane_to_code_lut[(yoffset * 16 as libc::c_int + 8 as libc::c_int
            - xoffset) as usize] as libc::c_int + 1 as libc::c_int
    } else if xoffset > xsize - 8 as libc::c_int && yoffset < 7 as libc::c_int {
        return plane_to_code_lut[((yoffset + 1 as libc::c_int) * 16 as libc::c_int
            + 8 as libc::c_int + (xsize - xoffset)) as usize] as libc::c_int
            + 1 as libc::c_int
    }
    return dist + 120 as libc::c_int;
}
#[inline]
unsafe extern "C" fn FindMatchLength(
    array1: *const uint32_t,
    array2: *const uint32_t,
    mut best_len_match: libc::c_int,
    mut max_limit: libc::c_int,
) -> libc::c_int {
    if *array1.offset(best_len_match as isize) != *array2.offset(best_len_match as isize)
    {
        return 0 as libc::c_int;
    }
    return VP8LVectorMismatch
        .expect("non-null function pointer")(array1, array2, max_limit);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LClearBackwardRefs(refs: *mut VP8LBackwardRefs) {
    if !((*refs).tail_).is_null() {
        *(*refs).tail_ = (*refs).free_blocks_;
    }
    (*refs).free_blocks_ = (*refs).refs_;
    (*refs).tail_ = &mut (*refs).refs_;
    (*refs).last_block_ = 0 as *mut PixOrCopyBlock;
    (*refs).refs_ = 0 as *mut PixOrCopyBlock;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBackwardRefsClear(refs: *mut VP8LBackwardRefs) {
    VP8LClearBackwardRefs(refs);
    while !((*refs).free_blocks_).is_null() {
        let next: *mut PixOrCopyBlock = (*(*refs).free_blocks_).next_;
        WebPSafeFree((*refs).free_blocks_ as *mut libc::c_void);
        (*refs).free_blocks_ = next;
    }
}
unsafe extern "C" fn BackwardRefsSwap(
    refs1: *mut VP8LBackwardRefs,
    refs2: *mut VP8LBackwardRefs,
) {
    let point_to_refs1: libc::c_int = (!((*refs1).tail_).is_null()
        && (*refs1).tail_ == &mut (*refs1).refs_ as *mut *mut PixOrCopyBlock)
        as libc::c_int;
    let point_to_refs2: libc::c_int = (!((*refs2).tail_).is_null()
        && (*refs2).tail_ == &mut (*refs2).refs_ as *mut *mut PixOrCopyBlock)
        as libc::c_int;
    let tmp: VP8LBackwardRefs = *refs1;
    *refs1 = *refs2;
    *refs2 = tmp;
    if point_to_refs2 != 0 {
        (*refs1).tail_ = &mut (*refs1).refs_;
    }
    if point_to_refs1 != 0 {
        (*refs2).tail_ = &mut (*refs2).refs_;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBackwardRefsInit(
    refs: *mut VP8LBackwardRefs,
    mut block_size: libc::c_int,
) {
    memset(
        refs as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8LBackwardRefs>() as libc::c_ulong,
    );
    (*refs).tail_ = &mut (*refs).refs_;
    (*refs)
        .block_size_ = if block_size < 256 as libc::c_int {
        256 as libc::c_int
    } else {
        block_size
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LRefsCursorInit(
    refs: *const VP8LBackwardRefs,
) -> VP8LRefsCursor {
    let mut c: VP8LRefsCursor = VP8LRefsCursor {
        cur_pos: 0 as *mut PixOrCopy,
        cur_block_: 0 as *mut PixOrCopyBlock,
        last_pos_: 0 as *const PixOrCopy,
    };
    c.cur_block_ = (*refs).refs_;
    if !((*refs).refs_).is_null() {
        c.cur_pos = (*c.cur_block_).start_;
        c.last_pos_ = (c.cur_pos).offset((*c.cur_block_).size_ as isize);
    } else {
        c.cur_pos = 0 as *mut PixOrCopy;
        c.last_pos_ = 0 as *const PixOrCopy;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LRefsCursorNextBlock(c: *mut VP8LRefsCursor) {
    let b: *mut PixOrCopyBlock = (*(*c).cur_block_).next_;
    (*c).cur_pos = if b.is_null() { 0 as *mut PixOrCopy } else { (*b).start_ };
    (*c)
        .last_pos_ = if b.is_null() {
        0 as *mut PixOrCopy
    } else {
        ((*b).start_).offset((*b).size_ as isize)
    };
    (*c).cur_block_ = b;
}
unsafe extern "C" fn BackwardRefsNewBlock(
    refs: *mut VP8LBackwardRefs,
) -> *mut PixOrCopyBlock {
    let mut b: *mut PixOrCopyBlock = (*refs).free_blocks_;
    if b.is_null() {
        let total_size: size_t = (::core::mem::size_of::<PixOrCopyBlock>()
            as libc::c_ulong)
            .wrapping_add(
                ((*refs).block_size_ as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<PixOrCopy>() as libc::c_ulong),
            );
        b = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, total_size)
            as *mut PixOrCopyBlock;
        if b.is_null() {
            (*refs).error_ |= 1 as libc::c_int;
            return 0 as *mut PixOrCopyBlock;
        }
        (*b)
            .start_ = (b as *mut uint8_t)
            .offset(::core::mem::size_of::<PixOrCopyBlock>() as libc::c_ulong as isize)
            as *mut PixOrCopy;
    } else {
        (*refs).free_blocks_ = (*b).next_;
    }
    *(*refs).tail_ = b;
    (*refs).tail_ = &mut (*b).next_;
    (*refs).last_block_ = b;
    (*b).next_ = 0 as *mut PixOrCopyBlock;
    (*b).size_ = 0 as libc::c_int;
    return b;
}
unsafe extern "C" fn BackwardRefsClone(
    from: *const VP8LBackwardRefs,
    to: *mut VP8LBackwardRefs,
) -> libc::c_int {
    let mut block_from: *const PixOrCopyBlock = (*from).refs_;
    VP8LClearBackwardRefs(to);
    while !block_from.is_null() {
        let block_to: *mut PixOrCopyBlock = BackwardRefsNewBlock(to);
        if block_to.is_null() {
            return 0 as libc::c_int;
        }
        memcpy(
            (*block_to).start_ as *mut libc::c_void,
            (*block_from).start_ as *const libc::c_void,
            ((*block_from).size_ as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<PixOrCopy>() as libc::c_ulong),
        );
        (*block_to).size_ = (*block_from).size_;
        block_from = (*block_from).next_;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBackwardRefsCursorAdd(
    refs: *mut VP8LBackwardRefs,
    v: PixOrCopy,
) {
    let mut b: *mut PixOrCopyBlock = (*refs).last_block_;
    if b.is_null() || (*b).size_ == (*refs).block_size_ {
        b = BackwardRefsNewBlock(refs);
        if b.is_null() {
            return;
        }
    }
    let fresh0 = (*b).size_;
    (*b).size_ = (*b).size_ + 1;
    *((*b).start_).offset(fresh0 as isize) = v;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHashChainInit(
    p: *mut VP8LHashChain,
    mut size: libc::c_int,
) -> libc::c_int {
    (*p)
        .offset_length_ = WebPSafeMalloc(
        size as uint64_t,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    if ((*p).offset_length_).is_null() {
        return 0 as libc::c_int;
    }
    (*p).size_ = size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHashChainClear(p: *mut VP8LHashChain) {
    WebPSafeFree((*p).offset_length_ as *mut libc::c_void);
    (*p).size_ = 0 as libc::c_int;
    (*p).offset_length_ = 0 as *mut uint32_t;
}
static mut kHashMultiplierHi: uint32_t = 0xc6a4a793 as libc::c_uint;
static mut kHashMultiplierLo: uint32_t = 0x5bd1e996 as libc::c_uint;
#[inline]
unsafe extern "C" fn GetPixPairHash64(argb: *const uint32_t) -> uint32_t {
    let mut key: uint32_t = 0;
    key = (*argb.offset(1 as libc::c_int as isize)).wrapping_mul(kHashMultiplierHi);
    key = (key as libc::c_uint)
        .wrapping_add(
            (*argb.offset(0 as libc::c_int as isize)).wrapping_mul(kHashMultiplierLo),
        ) as uint32_t as uint32_t;
    key = key >> 32 as libc::c_int - 18 as libc::c_int;
    return key;
}
unsafe extern "C" fn GetMaxItersForQuality(mut quality: libc::c_int) -> libc::c_int {
    return 8 as libc::c_int + quality * quality / 128 as libc::c_int;
}
unsafe extern "C" fn GetWindowSizeForHashChain(
    mut quality: libc::c_int,
    mut xsize: libc::c_int,
) -> libc::c_int {
    let max_window_size: libc::c_int = if quality > 75 as libc::c_int {
        ((1 as libc::c_int) << 20 as libc::c_int) - 120 as libc::c_int
    } else if quality > 50 as libc::c_int {
        xsize << 8 as libc::c_int
    } else if quality > 25 as libc::c_int {
        xsize << 6 as libc::c_int
    } else {
        xsize << 4 as libc::c_int
    };
    return if max_window_size
        > ((1 as libc::c_int) << 20 as libc::c_int) - 120 as libc::c_int
    {
        ((1 as libc::c_int) << 20 as libc::c_int) - 120 as libc::c_int
    } else {
        max_window_size
    };
}
#[inline]
unsafe extern "C" fn MaxFindCopyLength(mut len: libc::c_int) -> libc::c_int {
    return if len < ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int {
        len
    } else {
        ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHashChainFill(
    p: *mut VP8LHashChain,
    mut quality: libc::c_int,
    argb: *const uint32_t,
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    mut low_effort: libc::c_int,
    pic: *const WebPPicture,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let size: libc::c_int = xsize * ysize;
    let iter_max: libc::c_int = GetMaxItersForQuality(quality);
    let window_size: uint32_t = GetWindowSizeForHashChain(quality, xsize) as uint32_t;
    let mut remaining_percent: libc::c_int = percent_range;
    let mut percent_start: libc::c_int = *percent;
    let mut pos: libc::c_int = 0;
    let mut argb_comp: libc::c_int = 0;
    let mut base_position: uint32_t = 0;
    let mut hash_to_first_index: *mut int32_t = 0 as *mut int32_t;
    let mut chain: *mut int32_t = (*p).offset_length_ as *mut int32_t;
    if size <= 2 as libc::c_int {
        let ref mut fresh1 = *((*p).offset_length_)
            .offset((size - 1 as libc::c_int) as isize);
        *fresh1 = 0 as libc::c_int as uint32_t;
        *((*p).offset_length_).offset(0 as libc::c_int as isize) = *fresh1;
        return 1 as libc::c_int;
    }
    hash_to_first_index = WebPSafeMalloc(
        ((1 as libc::c_int) << 18 as libc::c_int) as uint64_t,
        ::core::mem::size_of::<int32_t>() as libc::c_ulong,
    ) as *mut int32_t;
    if hash_to_first_index.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    percent_range = remaining_percent / 2 as libc::c_int;
    remaining_percent -= percent_range;
    memset(
        hash_to_first_index as *mut libc::c_void,
        0xff as libc::c_int,
        (((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    argb_comp = (*argb.offset(0 as libc::c_int as isize)
        == *argb.offset(1 as libc::c_int as isize)) as libc::c_int;
    pos = 0 as libc::c_int;
    while pos < size - 2 as libc::c_int {
        let mut hash_code: uint32_t = 0;
        let argb_comp_next: libc::c_int = (*argb
            .offset((pos + 1 as libc::c_int) as isize)
            == *argb.offset((pos + 2 as libc::c_int) as isize)) as libc::c_int;
        if argb_comp != 0 && argb_comp_next != 0 {
            let mut tmp: [uint32_t; 2] = [0; 2];
            let mut len: uint32_t = 1 as libc::c_int as uint32_t;
            tmp[0 as libc::c_int as usize] = *argb.offset(pos as isize);
            while (pos + len as libc::c_int + 2 as libc::c_int) < size
                && *argb
                    .offset(
                        (pos as libc::c_uint)
                            .wrapping_add(len)
                            .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                    ) == *argb.offset(pos as isize)
            {
                len = len.wrapping_add(1);
                len;
            }
            if len
                > (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint
            {
                memset(
                    chain.offset(pos as isize) as *mut libc::c_void,
                    0xff as libc::c_int,
                    (len
                        .wrapping_sub(
                            (((1 as libc::c_int) << 12 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint,
                        ) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<int32_t>() as libc::c_ulong),
                );
                pos = (pos as libc::c_uint)
                    .wrapping_add(
                        len
                            .wrapping_sub(
                                (((1 as libc::c_int) << 12 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint,
                            ),
                    ) as libc::c_int as libc::c_int;
                len = (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int)
                    as uint32_t;
            }
            while len != 0 {
                let fresh2 = len;
                len = len.wrapping_sub(1);
                tmp[1 as libc::c_int as usize] = fresh2;
                hash_code = GetPixPairHash64(tmp.as_mut_ptr());
                *chain
                    .offset(
                        pos as isize,
                    ) = *hash_to_first_index.offset(hash_code as isize);
                let fresh3 = pos;
                pos = pos + 1;
                *hash_to_first_index.offset(hash_code as isize) = fresh3;
            }
            argb_comp = 0 as libc::c_int;
        } else {
            hash_code = GetPixPairHash64(argb.offset(pos as isize));
            *chain
                .offset(pos as isize) = *hash_to_first_index.offset(hash_code as isize);
            let fresh4 = pos;
            pos = pos + 1;
            *hash_to_first_index.offset(hash_code as isize) = fresh4;
            argb_comp = argb_comp_next;
        }
        if WebPReportProgress(
            pic,
            percent_start + percent_range * pos / (size - 2 as libc::c_int),
            percent,
        ) == 0
        {
            WebPSafeFree(hash_to_first_index as *mut libc::c_void);
            return 0 as libc::c_int;
        }
    }
    *chain
        .offset(
            pos as isize,
        ) = *hash_to_first_index
        .offset(GetPixPairHash64(argb.offset(pos as isize)) as isize);
    WebPSafeFree(hash_to_first_index as *mut libc::c_void);
    percent_start += percent_range;
    if WebPReportProgress(pic, percent_start, percent) == 0 {
        return 0 as libc::c_int;
    }
    percent_range = remaining_percent;
    let ref mut fresh5 = *((*p).offset_length_)
        .offset((size - 1 as libc::c_int) as isize);
    *fresh5 = 0 as libc::c_int as uint32_t;
    *((*p).offset_length_).offset(0 as libc::c_int as isize) = *fresh5;
    base_position = (size - 2 as libc::c_int) as uint32_t;
    while base_position > 0 as libc::c_int as libc::c_uint {
        let max_len: libc::c_int = MaxFindCopyLength(
            ((size - 1 as libc::c_int) as libc::c_uint).wrapping_sub(base_position)
                as libc::c_int,
        );
        let argb_start: *const uint32_t = argb.offset(base_position as isize);
        let mut iter: libc::c_int = iter_max;
        let mut best_length: libc::c_int = 0 as libc::c_int;
        let mut best_distance: uint32_t = 0 as libc::c_int as uint32_t;
        let mut best_argb: uint32_t = 0;
        let min_pos: libc::c_int = (if base_position > window_size {
            base_position.wrapping_sub(window_size)
        } else {
            0 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        let length_max: libc::c_int = if max_len < 256 as libc::c_int {
            max_len
        } else {
            256 as libc::c_int
        };
        let mut max_base_position: uint32_t = 0;
        pos = *chain.offset(base_position as isize);
        if low_effort == 0 {
            let mut curr_length: libc::c_int = 0;
            if base_position >= xsize as uint32_t {
                curr_length = FindMatchLength(
                    argb_start.offset(-(xsize as isize)),
                    argb_start,
                    best_length,
                    max_len,
                );
                if curr_length > best_length {
                    best_length = curr_length;
                    best_distance = xsize as uint32_t;
                }
                iter -= 1;
                iter;
            }
            curr_length = FindMatchLength(
                argb_start.offset(-(1 as libc::c_int as isize)),
                argb_start,
                best_length,
                max_len,
            );
            if curr_length > best_length {
                best_length = curr_length;
                best_distance = 1 as libc::c_int as uint32_t;
            }
            iter -= 1;
            iter;
            if best_length
                == ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int
            {
                pos = min_pos - 1 as libc::c_int;
            }
        }
        best_argb = *argb_start.offset(best_length as isize);
        while pos >= min_pos
            && {
                iter -= 1;
                iter != 0
            }
        {
            let mut curr_length_0: libc::c_int = 0;
            if !(*argb.offset((pos + best_length) as isize) != best_argb) {
                curr_length_0 = VP8LVectorMismatch
                    .expect(
                        "non-null function pointer",
                    )(argb.offset(pos as isize), argb_start, max_len);
                if best_length < curr_length_0 {
                    best_length = curr_length_0;
                    best_distance = base_position.wrapping_sub(pos as libc::c_uint);
                    best_argb = *argb_start.offset(best_length as isize);
                    if best_length >= length_max {
                        break;
                    }
                }
            }
            pos = *chain.offset(pos as isize);
        }
        max_base_position = base_position;
        loop {
            *((*p).offset_length_)
                .offset(
                    base_position as isize,
                ) = best_distance << 12 as libc::c_int | best_length as uint32_t;
            base_position = base_position.wrapping_sub(1);
            base_position;
            if best_distance == 0 as libc::c_int as libc::c_uint
                || base_position == 0 as libc::c_int as libc::c_uint
            {
                break;
            }
            if base_position < best_distance
                || *argb.offset(base_position.wrapping_sub(best_distance) as isize)
                    != *argb.offset(base_position as isize)
            {
                break;
            }
            if best_length
                == ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int
                && best_distance != 1 as libc::c_int as libc::c_uint
                && base_position
                    .wrapping_add(
                        (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint,
                    ) < max_base_position
            {
                break;
            }
            if best_length < ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int
            {
                best_length += 1;
                best_length;
                max_base_position = base_position;
            }
        }
        if WebPReportProgress(
            pic,
            (percent_start as libc::c_uint)
                .wrapping_add(
                    (percent_range as libc::c_uint)
                        .wrapping_mul(
                            ((size - 2 as libc::c_int) as libc::c_uint)
                                .wrapping_sub(base_position),
                        )
                        .wrapping_div((size - 2 as libc::c_int) as libc::c_uint),
                ) as libc::c_int,
            percent,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    return WebPReportProgress(pic, percent_start + percent_range, percent);
}
#[inline]
unsafe extern "C" fn AddSingleLiteral(
    mut pixel: uint32_t,
    mut use_color_cache: libc::c_int,
    hashers: *mut VP8LColorCache,
    refs: *mut VP8LBackwardRefs,
) {
    let mut v: PixOrCopy = PixOrCopy {
        mode: 0,
        len: 0,
        argb_or_distance: 0,
    };
    if use_color_cache != 0 {
        let key: uint32_t = VP8LColorCacheGetIndex(hashers, pixel) as uint32_t;
        if VP8LColorCacheLookup(hashers, key) == pixel {
            v = PixOrCopyCreateCacheIdx(key as libc::c_int);
        } else {
            v = PixOrCopyCreateLiteral(pixel);
            VP8LColorCacheSet(hashers, key, pixel);
        }
    } else {
        v = PixOrCopyCreateLiteral(pixel);
    }
    VP8LBackwardRefsCursorAdd(refs, v);
}
unsafe extern "C" fn BackwardReferencesRle(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    argb: *const uint32_t,
    mut cache_bits: libc::c_int,
    refs: *mut VP8LBackwardRefs,
) -> libc::c_int {
    let pix_count: libc::c_int = xsize * ysize;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let use_color_cache: libc::c_int = (cache_bits > 0 as libc::c_int) as libc::c_int;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: 0 as *mut uint32_t,
        hash_shift_: 0,
        hash_bits_: 0,
    };
    if use_color_cache != 0 && VP8LColorCacheInit(&mut hashers, cache_bits) == 0 {
        return 0 as libc::c_int;
    }
    VP8LClearBackwardRefs(refs);
    AddSingleLiteral(
        *argb.offset(0 as libc::c_int as isize),
        use_color_cache,
        &mut hashers,
        refs,
    );
    i = 1 as libc::c_int;
    while i < pix_count {
        let max_len: libc::c_int = MaxFindCopyLength(pix_count - i);
        let rle_len: libc::c_int = FindMatchLength(
            argb.offset(i as isize),
            argb.offset(i as isize).offset(-(1 as libc::c_int as isize)),
            0 as libc::c_int,
            max_len,
        );
        let prev_row_len: libc::c_int = if i < xsize {
            0 as libc::c_int
        } else {
            FindMatchLength(
                argb.offset(i as isize),
                argb.offset(i as isize).offset(-(xsize as isize)),
                0 as libc::c_int,
                max_len,
            )
        };
        if rle_len >= prev_row_len && rle_len >= 4 as libc::c_int {
            VP8LBackwardRefsCursorAdd(
                refs,
                PixOrCopyCreateCopy(1 as libc::c_int as uint32_t, rle_len as uint16_t),
            );
            i += rle_len;
        } else if prev_row_len >= 4 as libc::c_int {
            VP8LBackwardRefsCursorAdd(
                refs,
                PixOrCopyCreateCopy(xsize as uint32_t, prev_row_len as uint16_t),
            );
            if use_color_cache != 0 {
                k = 0 as libc::c_int;
                while k < prev_row_len {
                    VP8LColorCacheInsert(&mut hashers, *argb.offset((i + k) as isize));
                    k += 1;
                    k;
                }
            }
            i += prev_row_len;
        } else {
            AddSingleLiteral(
                *argb.offset(i as isize),
                use_color_cache,
                &mut hashers,
                refs,
            );
            i += 1;
            i;
        }
    }
    if use_color_cache != 0 {
        VP8LColorCacheClear(&mut hashers);
    }
    return ((*refs).error_ == 0) as libc::c_int;
}
unsafe extern "C" fn BackwardReferencesLz77(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    argb: *const uint32_t,
    mut cache_bits: libc::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut i_last_check: libc::c_int = -(1 as libc::c_int);
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut cc_init: libc::c_int = 0 as libc::c_int;
    let use_color_cache: libc::c_int = (cache_bits > 0 as libc::c_int) as libc::c_int;
    let pix_count: libc::c_int = xsize * ysize;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: 0 as *mut uint32_t,
        hash_shift_: 0,
        hash_bits_: 0,
    };
    if use_color_cache != 0 {
        cc_init = VP8LColorCacheInit(&mut hashers, cache_bits);
        if cc_init == 0 {
            current_block = 16789425564425565956;
        } else {
            current_block = 15427931788582360902;
        }
    } else {
        current_block = 15427931788582360902;
    }
    match current_block {
        15427931788582360902 => {
            VP8LClearBackwardRefs(refs);
            i = 0 as libc::c_int;
            while i < pix_count {
                let mut offset: libc::c_int = 0 as libc::c_int;
                let mut len: libc::c_int = 0 as libc::c_int;
                let mut j: libc::c_int = 0;
                VP8LHashChainFindCopy(hash_chain, i, &mut offset, &mut len);
                if len >= 4 as libc::c_int {
                    let len_ini: libc::c_int = len;
                    let mut max_reach: libc::c_int = 0 as libc::c_int;
                    let j_max: libc::c_int = if i + len_ini >= pix_count {
                        pix_count - 1 as libc::c_int
                    } else {
                        i + len_ini
                    };
                    i_last_check = if i > i_last_check { i } else { i_last_check };
                    j = i_last_check + 1 as libc::c_int;
                    while j <= j_max {
                        let len_j: libc::c_int = VP8LHashChainFindLength(hash_chain, j);
                        let reach: libc::c_int = j
                            + (if len_j >= 4 as libc::c_int {
                                len_j
                            } else {
                                1 as libc::c_int
                            });
                        if reach > max_reach {
                            len = j - i;
                            max_reach = reach;
                            if max_reach >= pix_count {
                                break;
                            }
                        }
                        j += 1;
                        j;
                    }
                } else {
                    len = 1 as libc::c_int;
                }
                if len == 1 as libc::c_int {
                    AddSingleLiteral(
                        *argb.offset(i as isize),
                        use_color_cache,
                        &mut hashers,
                        refs,
                    );
                } else {
                    VP8LBackwardRefsCursorAdd(
                        refs,
                        PixOrCopyCreateCopy(offset as uint32_t, len as uint16_t),
                    );
                    if use_color_cache != 0 {
                        j = i;
                        while j < i + len {
                            VP8LColorCacheInsert(&mut hashers, *argb.offset(j as isize));
                            j += 1;
                            j;
                        }
                    }
                }
                i += len;
            }
            ok = ((*refs).error_ == 0) as libc::c_int;
        }
        _ => {}
    }
    if cc_init != 0 {
        VP8LColorCacheClear(&mut hashers);
    }
    return ok;
}
unsafe extern "C" fn BackwardReferencesLz77Box(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    argb: *const uint32_t,
    mut cache_bits: libc::c_int,
    hash_chain_best: *const VP8LHashChain,
    mut hash_chain: *mut VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let pix_count: libc::c_int = xsize * ysize;
    let mut counts: *mut uint16_t = 0 as *mut uint16_t;
    let mut window_offsets: [libc::c_int; 32] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut window_offsets_new: [libc::c_int; 32] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut window_offsets_size: libc::c_int = 0 as libc::c_int;
    let mut window_offsets_new_size: libc::c_int = 0 as libc::c_int;
    let counts_ini: *mut uint16_t = WebPSafeMalloc(
        (xsize * ysize) as uint64_t,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    ) as *mut uint16_t;
    let mut best_offset_prev: libc::c_int = -(1 as libc::c_int);
    let mut best_length_prev: libc::c_int = -(1 as libc::c_int);
    if counts_ini.is_null() {
        return 0 as libc::c_int;
    }
    i = pix_count - 2 as libc::c_int;
    counts = counts_ini.offset(i as isize);
    *counts.offset(1 as libc::c_int as isize) = 1 as libc::c_int as uint16_t;
    while i >= 0 as libc::c_int {
        if *argb.offset(i as isize) == *argb.offset((i + 1 as libc::c_int) as isize) {
            *counts
                .offset(
                    0 as libc::c_int as isize,
                ) = (*counts.offset(1 as libc::c_int as isize) as libc::c_int
                + (*counts.offset(1 as libc::c_int as isize) as libc::c_int
                    != ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_int) as uint16_t;
        } else {
            *counts.offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint16_t;
        }
        i -= 1;
        i;
        counts = counts.offset(-1);
        counts;
    }
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y <= 6 as libc::c_int {
        x = -(6 as libc::c_int);
        while x <= 6 as libc::c_int {
            let offset: libc::c_int = y * xsize + x;
            let mut plane_code: libc::c_int = 0;
            if !(offset <= 0 as libc::c_int) {
                plane_code = VP8LDistanceToPlaneCode(xsize, offset) - 1 as libc::c_int;
                if !(plane_code >= 32 as libc::c_int) {
                    window_offsets[plane_code as usize] = offset;
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !(window_offsets[i as usize] == 0 as libc::c_int) {
            let fresh6 = window_offsets_size;
            window_offsets_size = window_offsets_size + 1;
            window_offsets[fresh6 as usize] = window_offsets[i as usize];
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < window_offsets_size {
        let mut j: libc::c_int = 0;
        let mut is_reachable: libc::c_int = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < window_offsets_size && is_reachable == 0 {
            is_reachable
                |= (window_offsets[i as usize]
                    == window_offsets[j as usize] + 1 as libc::c_int) as libc::c_int;
            j += 1;
            j;
        }
        if is_reachable == 0 {
            window_offsets_new[window_offsets_new_size
                as usize] = window_offsets[i as usize];
            window_offsets_new_size += 1;
            window_offsets_new_size;
        }
        i += 1;
        i;
    }
    *((*hash_chain).offset_length_)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint32_t;
    i = 1 as libc::c_int;
    while i < pix_count {
        let mut ind: libc::c_int = 0;
        let mut best_length: libc::c_int = VP8LHashChainFindLength(hash_chain_best, i);
        let mut best_offset: libc::c_int = 0;
        let mut do_compute: libc::c_int = 1 as libc::c_int;
        if best_length >= ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int {
            best_offset = VP8LHashChainFindOffset(hash_chain_best, i);
            ind = 0 as libc::c_int;
            while ind < window_offsets_size {
                if best_offset == window_offsets[ind as usize] {
                    do_compute = 0 as libc::c_int;
                    break;
                } else {
                    ind += 1;
                    ind;
                }
            }
        }
        if do_compute != 0 {
            let use_prev: libc::c_int = (best_length_prev > 1 as libc::c_int
                && best_length_prev
                    < ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int)
                as libc::c_int;
            let num_ind: libc::c_int = if use_prev != 0 {
                window_offsets_new_size
            } else {
                window_offsets_size
            };
            best_length = if use_prev != 0 {
                best_length_prev - 1 as libc::c_int
            } else {
                0 as libc::c_int
            };
            best_offset = if use_prev != 0 {
                best_offset_prev
            } else {
                0 as libc::c_int
            };
            ind = 0 as libc::c_int;
            while ind < num_ind {
                let mut curr_length: libc::c_int = 0 as libc::c_int;
                let mut j_0: libc::c_int = i;
                let mut j_offset: libc::c_int = if use_prev != 0 {
                    i - window_offsets_new[ind as usize]
                } else {
                    i - window_offsets[ind as usize]
                };
                if !(j_offset < 0 as libc::c_int
                    || *argb.offset(j_offset as isize) != *argb.offset(i as isize))
                {
                    loop {
                        let counts_j_offset: libc::c_int = *counts_ini
                            .offset(j_offset as isize) as libc::c_int;
                        let counts_j: libc::c_int = *counts_ini.offset(j_0 as isize)
                            as libc::c_int;
                        if counts_j_offset != counts_j {
                            curr_length
                                += if counts_j_offset < counts_j {
                                    counts_j_offset
                                } else {
                                    counts_j
                                };
                            break;
                        } else {
                            curr_length += counts_j_offset;
                            j_offset += counts_j_offset;
                            j_0 += counts_j_offset;
                            if !(curr_length
                                <= ((1 as libc::c_int) << 12 as libc::c_int)
                                    - 1 as libc::c_int && j_0 < pix_count
                                && *argb.offset(j_offset as isize)
                                    == *argb.offset(j_0 as isize))
                            {
                                break;
                            }
                        }
                    }
                    if best_length < curr_length {
                        best_offset = if use_prev != 0 {
                            window_offsets_new[ind as usize]
                        } else {
                            window_offsets[ind as usize]
                        };
                        if curr_length
                            >= ((1 as libc::c_int) << 12 as libc::c_int)
                                - 1 as libc::c_int
                        {
                            best_length = ((1 as libc::c_int) << 12 as libc::c_int)
                                - 1 as libc::c_int;
                            break;
                        } else {
                            best_length = curr_length;
                        }
                    }
                }
                ind += 1;
                ind;
            }
        }
        if best_length <= 4 as libc::c_int {
            *((*hash_chain).offset_length_)
                .offset(i as isize) = 0 as libc::c_int as uint32_t;
            best_offset_prev = 0 as libc::c_int;
            best_length_prev = 0 as libc::c_int;
        } else {
            *((*hash_chain).offset_length_)
                .offset(
                    i as isize,
                ) = (best_offset << 12 as libc::c_int) as libc::c_uint
                | best_length as uint32_t;
            best_offset_prev = best_offset;
            best_length_prev = best_length;
        }
        i += 1;
        i;
    }
    *((*hash_chain).offset_length_)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint32_t;
    WebPSafeFree(counts_ini as *mut libc::c_void);
    return BackwardReferencesLz77(xsize, ysize, argb, cache_bits, hash_chain, refs);
}
unsafe extern "C" fn BackwardReferences2DLocality(
    mut xsize: libc::c_int,
    refs: *const VP8LBackwardRefs,
) {
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    while VP8LRefsCursorOk(&mut c) != 0 {
        if PixOrCopyIsCopy(c.cur_pos) != 0 {
            let dist: libc::c_int = (*c.cur_pos).argb_or_distance as libc::c_int;
            let transformed_dist: libc::c_int = VP8LDistanceToPlaneCode(xsize, dist);
            (*c.cur_pos).argb_or_distance = transformed_dist as uint32_t;
        }
        VP8LRefsCursorNext(&mut c);
    }
}
unsafe extern "C" fn CalculateBestCacheSize(
    mut argb: *const uint32_t,
    mut quality: libc::c_int,
    refs: *const VP8LBackwardRefs,
    best_cache_bits: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let cache_bits_max: libc::c_int = if quality <= 25 as libc::c_int {
        0 as libc::c_int
    } else {
        *best_cache_bits
    };
    let mut entropy_min: libc::c_float = 1e30f32;
    let mut cc_init: [libc::c_int; 11] = [
        0 as libc::c_int,
        0,
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
    let mut hashers: [VP8LColorCache; 11] = [VP8LColorCache {
        colors_: 0 as *mut uint32_t,
        hash_shift_: 0,
        hash_bits_: 0,
    }; 11];
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    let mut histos: [*mut VP8LHistogram; 11] = [
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
        0 as *mut VP8LHistogram,
    ];
    let mut ok: libc::c_int = 0 as libc::c_int;
    if cache_bits_max == 0 as libc::c_int {
        *best_cache_bits = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    loop {
        if !(i <= cache_bits_max) {
            current_block = 10599921512955367680;
            break;
        }
        histos[i as usize] = VP8LAllocateHistogram(i);
        if (histos[i as usize]).is_null() {
            current_block = 8117612481852030665;
            break;
        }
        VP8LHistogramInit(histos[i as usize], i, 1 as libc::c_int);
        if !(i == 0 as libc::c_int) {
            cc_init[i
                as usize] = VP8LColorCacheInit(
                &mut *hashers.as_mut_ptr().offset(i as isize),
                i,
            );
            if cc_init[i as usize] == 0 {
                current_block = 8117612481852030665;
                break;
            }
        }
        i += 1;
        i;
    }
    match current_block {
        10599921512955367680 => {
            while VP8LRefsCursorOk(&mut c) != 0 {
                let v: *const PixOrCopy = c.cur_pos;
                if PixOrCopyIsLiteral(v) != 0 {
                    let fresh7 = argb;
                    argb = argb.offset(1);
                    let pix: uint32_t = *fresh7;
                    let a: uint32_t = pix >> 24 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint;
                    let r: uint32_t = pix >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint;
                    let g: uint32_t = pix >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint;
                    let b: uint32_t = pix >> 0 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint;
                    let mut key: libc::c_int = VP8LHashPix(
                        pix,
                        32 as libc::c_int - cache_bits_max,
                    );
                    (*histos[0 as libc::c_int as usize])
                        .blue_[b
                        as usize] = ((*histos[0 as libc::c_int as usize])
                        .blue_[b as usize])
                        .wrapping_add(1);
                    (*histos[0 as libc::c_int as usize]).blue_[b as usize];
                    let ref mut fresh8 = *((*histos[0 as libc::c_int as usize]).literal_)
                        .offset(g as isize);
                    *fresh8 = (*fresh8).wrapping_add(1);
                    *fresh8;
                    (*histos[0 as libc::c_int as usize])
                        .red_[r
                        as usize] = ((*histos[0 as libc::c_int as usize])
                        .red_[r as usize])
                        .wrapping_add(1);
                    (*histos[0 as libc::c_int as usize]).red_[r as usize];
                    (*histos[0 as libc::c_int as usize])
                        .alpha_[a
                        as usize] = ((*histos[0 as libc::c_int as usize])
                        .alpha_[a as usize])
                        .wrapping_add(1);
                    (*histos[0 as libc::c_int as usize]).alpha_[a as usize];
                    i = cache_bits_max;
                    while i >= 1 as libc::c_int {
                        if VP8LColorCacheLookup(
                            &mut *hashers.as_mut_ptr().offset(i as isize),
                            key as uint32_t,
                        ) == pix
                        {
                            let ref mut fresh9 = *((*histos[i as usize]).literal_)
                                .offset(
                                    (256 as libc::c_int + 24 as libc::c_int + key) as isize,
                                );
                            *fresh9 = (*fresh9).wrapping_add(1);
                            *fresh9;
                        } else {
                            VP8LColorCacheSet(
                                &mut *hashers.as_mut_ptr().offset(i as isize),
                                key as uint32_t,
                                pix,
                            );
                            (*histos[i as usize])
                                .blue_[b
                                as usize] = ((*histos[i as usize]).blue_[b as usize])
                                .wrapping_add(1);
                            (*histos[i as usize]).blue_[b as usize];
                            let ref mut fresh10 = *((*histos[i as usize]).literal_)
                                .offset(g as isize);
                            *fresh10 = (*fresh10).wrapping_add(1);
                            *fresh10;
                            (*histos[i as usize])
                                .red_[r
                                as usize] = ((*histos[i as usize]).red_[r as usize])
                                .wrapping_add(1);
                            (*histos[i as usize]).red_[r as usize];
                            (*histos[i as usize])
                                .alpha_[a
                                as usize] = ((*histos[i as usize]).alpha_[a as usize])
                                .wrapping_add(1);
                            (*histos[i as usize]).alpha_[a as usize];
                        }
                        i -= 1;
                        i;
                        key >>= 1 as libc::c_int;
                    }
                } else {
                    let mut code: libc::c_int = 0;
                    let mut extra_bits: libc::c_int = 0;
                    let mut extra_bits_value: libc::c_int = 0;
                    let mut len: libc::c_int = PixOrCopyLength(v) as libc::c_int;
                    let mut argb_prev: uint32_t = *argb ^ 0xffffffff as libc::c_uint;
                    VP8LPrefixEncode(
                        len,
                        &mut code,
                        &mut extra_bits,
                        &mut extra_bits_value,
                    );
                    i = 0 as libc::c_int;
                    while i <= cache_bits_max {
                        let ref mut fresh11 = *((*histos[i as usize]).literal_)
                            .offset((256 as libc::c_int + code) as isize);
                        *fresh11 = (*fresh11).wrapping_add(1);
                        *fresh11;
                        i += 1;
                        i;
                    }
                    loop {
                        if *argb != argb_prev {
                            let mut key_0: libc::c_int = VP8LHashPix(
                                *argb,
                                32 as libc::c_int - cache_bits_max,
                            );
                            i = cache_bits_max;
                            while i >= 1 as libc::c_int {
                                *(hashers[i as usize].colors_)
                                    .offset(key_0 as isize) = *argb;
                                i -= 1;
                                i;
                                key_0 >>= 1 as libc::c_int;
                            }
                            argb_prev = *argb;
                        }
                        argb = argb.offset(1);
                        argb;
                        len -= 1;
                        if !(len != 0 as libc::c_int) {
                            break;
                        }
                    }
                }
                VP8LRefsCursorNext(&mut c);
            }
            i = 0 as libc::c_int;
            while i <= cache_bits_max {
                let entropy: libc::c_float = VP8LHistogramEstimateBits(
                    histos[i as usize],
                );
                if i == 0 as libc::c_int || entropy < entropy_min {
                    entropy_min = entropy;
                    *best_cache_bits = i;
                }
                i += 1;
                i;
            }
            ok = 1 as libc::c_int;
        }
        _ => {}
    }
    i = 0 as libc::c_int;
    while i <= cache_bits_max {
        if cc_init[i as usize] != 0 {
            VP8LColorCacheClear(&mut *hashers.as_mut_ptr().offset(i as isize));
        }
        VP8LFreeHistogram(histos[i as usize]);
        i += 1;
        i;
    }
    return ok;
}
unsafe extern "C" fn BackwardRefsWithLocalCache(
    argb: *const uint32_t,
    mut cache_bits: libc::c_int,
    refs: *mut VP8LBackwardRefs,
) -> libc::c_int {
    let mut pixel_index: libc::c_int = 0 as libc::c_int;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: 0 as *mut uint32_t,
        hash_shift_: 0,
        hash_bits_: 0,
    };
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    if VP8LColorCacheInit(&mut hashers, cache_bits) == 0 {
        return 0 as libc::c_int;
    }
    while VP8LRefsCursorOk(&mut c) != 0 {
        let v: *mut PixOrCopy = c.cur_pos;
        if PixOrCopyIsLiteral(v) != 0 {
            let argb_literal: uint32_t = (*v).argb_or_distance;
            let ix: libc::c_int = VP8LColorCacheContains(&mut hashers, argb_literal);
            if ix >= 0 as libc::c_int {
                *v = PixOrCopyCreateCacheIdx(ix);
            } else {
                VP8LColorCacheInsert(&mut hashers, argb_literal);
            }
            pixel_index += 1;
            pixel_index;
        } else {
            let mut k: libc::c_int = 0;
            k = 0 as libc::c_int;
            while k < (*v).len as libc::c_int {
                let fresh12 = pixel_index;
                pixel_index = pixel_index + 1;
                VP8LColorCacheInsert(&mut hashers, *argb.offset(fresh12 as isize));
                k += 1;
                k;
            }
        }
        VP8LRefsCursorNext(&mut c);
    }
    VP8LColorCacheClear(&mut hashers);
    return 1 as libc::c_int;
}
unsafe extern "C" fn GetBackwardReferencesLowEffort(
    mut width: libc::c_int,
    mut height: libc::c_int,
    argb: *const uint32_t,
    cache_bits: *mut libc::c_int,
    hash_chain: *const VP8LHashChain,
    refs_lz77: *mut VP8LBackwardRefs,
) -> *mut VP8LBackwardRefs {
    *cache_bits = 0 as libc::c_int;
    if BackwardReferencesLz77(
        width,
        height,
        argb,
        0 as libc::c_int,
        hash_chain,
        refs_lz77,
    ) == 0
    {
        return 0 as *mut VP8LBackwardRefs;
    }
    BackwardReferences2DLocality(width, refs_lz77);
    return refs_lz77;
}
unsafe extern "C" fn GetBackwardReferences(
    mut width: libc::c_int,
    mut height: libc::c_int,
    argb: *const uint32_t,
    mut quality: libc::c_int,
    mut lz77_types_to_try: libc::c_int,
    mut cache_bits_max: libc::c_int,
    mut do_no_cache: libc::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
    cache_bits_best: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut histo: *mut VP8LHistogram = 0 as *mut VP8LHistogram;
    let mut i: libc::c_int = 0;
    let mut lz77_type: libc::c_int = 0;
    let mut lz77_types_best: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
    let mut bit_costs_best: [libc::c_float; 2] = [3.40282347e+38f32, 3.40282347e+38f32];
    let mut hash_chain_box: VP8LHashChain = VP8LHashChain {
        offset_length_: 0 as *mut uint32_t,
        size_: 0,
    };
    let refs_tmp: *mut VP8LBackwardRefs = &mut *refs
        .offset(
            (if do_no_cache != 0 { 2 as libc::c_int } else { 1 as libc::c_int }) as isize,
        ) as *mut VP8LBackwardRefs;
    let mut status: libc::c_int = 0 as libc::c_int;
    memset(
        &mut hash_chain_box as *mut VP8LHashChain as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8LHashChain>() as libc::c_ulong,
    );
    histo = VP8LAllocateHistogram(10 as libc::c_int);
    if !histo.is_null() {
        lz77_type = 1 as libc::c_int;
        's_27: loop {
            if !(lz77_types_to_try != 0) {
                current_block = 10891380440665537214;
                break;
            }
            let mut res: libc::c_int = 0 as libc::c_int;
            let mut bit_cost: libc::c_float = 0.0f32;
            if !(lz77_types_to_try & lz77_type == 0 as libc::c_int) {
                match lz77_type {
                    2 => {
                        res = BackwardReferencesRle(
                            width,
                            height,
                            argb,
                            0 as libc::c_int,
                            refs_tmp,
                        );
                    }
                    1 => {
                        res = BackwardReferencesLz77(
                            width,
                            height,
                            argb,
                            0 as libc::c_int,
                            hash_chain,
                            refs_tmp,
                        );
                    }
                    4 => {
                        if VP8LHashChainInit(&mut hash_chain_box, width * height) == 0 {
                            current_block = 14107288705473113101;
                            break;
                        }
                        res = BackwardReferencesLz77Box(
                            width,
                            height,
                            argb,
                            0 as libc::c_int,
                            hash_chain,
                            &mut hash_chain_box,
                            refs_tmp,
                        );
                    }
                    _ => {}
                }
                if res == 0 {
                    current_block = 14107288705473113101;
                    break;
                }
                i = 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    let mut cache_bits: libc::c_int = if i == 1 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        cache_bits_max
                    };
                    if !(i == 1 as libc::c_int && do_no_cache == 0) {
                        if i == 0 as libc::c_int {
                            if CalculateBestCacheSize(
                                argb,
                                quality,
                                refs_tmp,
                                &mut cache_bits,
                            ) == 0
                            {
                                current_block = 14107288705473113101;
                                break 's_27;
                            }
                            if cache_bits > 0 as libc::c_int {
                                if BackwardRefsWithLocalCache(argb, cache_bits, refs_tmp)
                                    == 0
                                {
                                    current_block = 14107288705473113101;
                                    break 's_27;
                                }
                            }
                        }
                        if !(i == 0 as libc::c_int && do_no_cache != 0
                            && cache_bits == 0 as libc::c_int)
                        {
                            VP8LHistogramCreate(histo, refs_tmp, cache_bits);
                            bit_cost = VP8LHistogramEstimateBits(histo);
                        }
                        if bit_cost < bit_costs_best[i as usize] {
                            if i == 1 as libc::c_int {
                                if BackwardRefsClone(
                                    refs_tmp,
                                    &mut *refs.offset(1 as libc::c_int as isize),
                                ) == 0
                                {
                                    current_block = 14107288705473113101;
                                    break 's_27;
                                }
                            } else {
                                BackwardRefsSwap(
                                    refs_tmp,
                                    &mut *refs.offset(0 as libc::c_int as isize),
                                );
                            }
                            bit_costs_best[i as usize] = bit_cost;
                            lz77_types_best[i as usize] = lz77_type;
                            if i == 0 as libc::c_int {
                                *cache_bits_best = cache_bits;
                            }
                        }
                    }
                    i -= 1;
                    i;
                }
            }
            lz77_types_to_try &= !lz77_type;
            lz77_type <<= 1 as libc::c_int;
        }
        match current_block {
            14107288705473113101 => {}
            _ => {
                i = 1 as libc::c_int;
                loop {
                    if !(i >= 0 as libc::c_int) {
                        current_block = 9353995356876505083;
                        break;
                    }
                    if !(i == 1 as libc::c_int && do_no_cache == 0) {
                        if (lz77_types_best[i as usize] == kLZ77Standard as libc::c_int
                            || lz77_types_best[i as usize] == kLZ77Box as libc::c_int)
                            && quality >= 25 as libc::c_int
                        {
                            let hash_chain_tmp: *const VP8LHashChain = if lz77_types_best[i
                                as usize] == kLZ77Standard as libc::c_int
                            {
                                hash_chain
                            } else {
                                &mut hash_chain_box as *mut VP8LHashChain
                                    as *const VP8LHashChain
                            };
                            let cache_bits_0: libc::c_int = if i == 1 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                *cache_bits_best
                            };
                            let mut bit_cost_trace: libc::c_float = 0.;
                            if VP8LBackwardReferencesTraceBackwards(
                                width,
                                height,
                                argb,
                                cache_bits_0,
                                hash_chain_tmp,
                                &mut *refs.offset(i as isize),
                                refs_tmp,
                            ) == 0
                            {
                                current_block = 14107288705473113101;
                                break;
                            }
                            VP8LHistogramCreate(histo, refs_tmp, cache_bits_0);
                            bit_cost_trace = VP8LHistogramEstimateBits(histo);
                            if bit_cost_trace < bit_costs_best[i as usize] {
                                BackwardRefsSwap(refs_tmp, &mut *refs.offset(i as isize));
                            }
                        }
                        BackwardReferences2DLocality(
                            width,
                            &mut *refs.offset(i as isize),
                        );
                        if i == 1 as libc::c_int
                            && lz77_types_best[0 as libc::c_int as usize]
                                == lz77_types_best[1 as libc::c_int as usize]
                            && *cache_bits_best == 0 as libc::c_int
                        {
                            if BackwardRefsClone(
                                &mut *refs.offset(1 as libc::c_int as isize),
                                &mut *refs.offset(0 as libc::c_int as isize),
                            ) == 0
                            {
                                current_block = 14107288705473113101;
                                break;
                            } else {
                                current_block = 9353995356876505083;
                                break;
                            }
                        }
                    }
                    i -= 1;
                    i;
                }
                match current_block {
                    14107288705473113101 => {}
                    _ => {
                        status = 1 as libc::c_int;
                    }
                }
            }
        }
    }
    VP8LHashChainClear(&mut hash_chain_box);
    VP8LFreeHistogram(histo);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetBackwardReferences(
    mut width: libc::c_int,
    mut height: libc::c_int,
    argb: *const uint32_t,
    mut quality: libc::c_int,
    mut low_effort: libc::c_int,
    mut lz77_types_to_try: libc::c_int,
    mut cache_bits_max: libc::c_int,
    mut do_no_cache: libc::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
    cache_bits_best: *mut libc::c_int,
    pic: *const WebPPicture,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    if low_effort != 0 {
        let mut refs_best: *mut VP8LBackwardRefs = 0 as *mut VP8LBackwardRefs;
        *cache_bits_best = cache_bits_max;
        refs_best = GetBackwardReferencesLowEffort(
            width,
            height,
            argb,
            cache_bits_best,
            hash_chain,
            refs,
        );
        if refs_best.is_null() {
            return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        BackwardRefsSwap(refs_best, &mut *refs.offset(0 as libc::c_int as isize));
    } else if GetBackwardReferences(
        width,
        height,
        argb,
        quality,
        lz77_types_to_try,
        cache_bits_max,
        do_no_cache,
        hash_chain,
        refs,
        cache_bits_best,
    ) == 0
    {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY)
    }
    return WebPReportProgress(pic, *percent + percent_range, percent);
}
