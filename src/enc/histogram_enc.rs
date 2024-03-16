use ::libc;

use super::backward_references_enc::PixOrCopyBlock;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VP8LBitsEntropyUnrefined(
        array: *const uint32_t,
        n: libc::c_int,
        entropy: *mut VP8LBitEntropy,
    );
    static mut VP8LExtraCost: VP8LCostFunc;
    fn VP8LRefsCursorInit(refs: *const VP8LBackwardRefs) -> VP8LRefsCursor;
    fn VP8LRefsCursorNextBlock(c: *mut VP8LRefsCursor);
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn VP8LHistogramAdd(
        a: *const VP8LHistogram,
        b: *const VP8LHistogram,
        out: *mut VP8LHistogram,
    );
    static mut VP8LExtraCostCombined: VP8LCostCombinedFunc;
    fn VP8LBitEntropyInit(entropy: *mut VP8LBitEntropy);
    static mut VP8LGetEntropyUnrefined: VP8LGetEntropyUnrefinedFunc;
    static mut VP8LGetCombinedEntropyUnrefined: VP8LGetCombinedEntropyUnrefinedFunc;
    static kPrefixEncodeCode: [VP8LPrefixCode; 512];
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct VP8LHistogramSet {
    pub size: libc::c_int,
    pub max_size: libc::c_int,
    pub histograms: *mut *mut VP8LHistogram,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
pub type VP8LCostCombinedFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, *const uint32_t, libc::c_int) -> uint32_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LStreaks {
    pub counts: [libc::c_int; 2],
    pub streaks: [[libc::c_int; 2]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LBitEntropy {
    pub entropy: libc::c_float,
    pub sum: uint32_t,
    pub nonzeros: libc::c_int,
    pub max_val: uint32_t,
    pub nonzero_code: uint32_t,
}
pub type VP8LGetEntropyUnrefinedFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        libc::c_int,
        *mut VP8LBitEntropy,
        *mut VP8LStreaks,
    ) -> (),
>;
pub type VP8LGetCombinedEntropyUnrefinedFunc = Option::<
    unsafe extern "C" fn(
        *const uint32_t,
        *const uint32_t,
        libc::c_int,
        *mut VP8LBitEntropy,
        *mut VP8LStreaks,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistoQueue {
    pub queue: *mut HistogramPair,
    pub size: libc::c_int,
    pub max_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramPair {
    pub idx1: libc::c_int,
    pub idx2: libc::c_int,
    pub cost_diff: libc::c_float,
    pub cost_combo: libc::c_float,
}
pub type VP8LCostFunc = Option::<
    unsafe extern "C" fn(*const uint32_t, libc::c_int) -> uint32_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub first: int16_t,
    pub num_combine_failures: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DominantCostRange {
    pub literal_max_: libc::c_float,
    pub literal_min_: libc::c_float,
    pub red_max_: libc::c_float,
    pub red_min_: libc::c_float,
    pub blue_max_: libc::c_float,
    pub blue_min_: libc::c_float,
}
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsLiteral(p: *const PixOrCopy) -> libc::c_int {
    return ((*p).mode as libc::c_int == kLiteral as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyIsCacheIdx(p: *const PixOrCopy) -> libc::c_int {
    return ((*p).mode as libc::c_int == kCacheIdx as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn PixOrCopyLiteral(
    p: *const PixOrCopy,
    mut component: libc::c_int,
) -> uint32_t {
    return (*p).argb_or_distance >> component * 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn PixOrCopyLength(p: *const PixOrCopy) -> uint32_t {
    return (*p).len as uint32_t;
}
#[inline]
unsafe extern "C" fn PixOrCopyCacheIdx(p: *const PixOrCopy) -> uint32_t {
    return (*p).argb_or_distance;
}
#[inline]
unsafe extern "C" fn PixOrCopyDistance(p: *const PixOrCopy) -> uint32_t {
    return (*p).argb_or_distance;
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorOk(c: *const VP8LRefsCursor) -> libc::c_int {
    return ((*c).cur_pos != 0 as *mut libc::c_void as *mut PixOrCopy) as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8LRefsCursorNext(c: *mut VP8LRefsCursor) {
    (*c).cur_pos = ((*c).cur_pos).offset(1);
    if (*c).cur_pos == (*c).last_pos_ as *mut PixOrCopy {
        VP8LRefsCursorNextBlock(c);
    }
}
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> libc::c_int {
    return 31 as libc::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8LHistogramNumCodes(
    mut palette_code_bits: libc::c_int,
) -> libc::c_int {
    return 256 as libc::c_int + 24 as libc::c_int
        + (if palette_code_bits > 0 as libc::c_int {
            (1 as libc::c_int) << palette_code_bits
        } else {
            0 as libc::c_int
        });
}
#[inline]
unsafe extern "C" fn VP8LSubSampleSize(
    mut size: uint32_t,
    mut sampling_bits: uint32_t,
) -> uint32_t {
    return size
        .wrapping_add(((1 as libc::c_int) << sampling_bits) as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) >> sampling_bits;
}
#[inline]
unsafe extern "C" fn VP8LPrefixEncodeBitsNoLUT(
    mut distance: libc::c_int,
    code: *mut libc::c_int,
    extra_bits: *mut libc::c_int,
) {
    distance -= 1;
    let highest_bit: libc::c_int = BitsLog2Floor(distance as uint32_t);
    let second_highest_bit: libc::c_int = distance >> highest_bit - 1 as libc::c_int
        & 1 as libc::c_int;
    *extra_bits = highest_bit - 1 as libc::c_int;
    *code = 2 as libc::c_int * highest_bit + second_highest_bit;
}
#[inline]
unsafe extern "C" fn VP8LPrefixEncodeBits(
    mut distance: libc::c_int,
    code: *mut libc::c_int,
    extra_bits: *mut libc::c_int,
) {
    if distance < 512 as libc::c_int {
        let prefix_code: VP8LPrefixCode = kPrefixEncodeCode[distance as usize];
        *code = prefix_code.code_ as libc::c_int;
        *extra_bits = prefix_code.extra_bits_ as libc::c_int;
    } else {
        VP8LPrefixEncodeBitsNoLUT(distance, code, extra_bits);
    };
}
unsafe extern "C" fn HistogramClear(p: *mut VP8LHistogram) {
    let literal: *mut uint32_t = (*p).literal_;
    let cache_bits: libc::c_int = (*p).palette_code_bits_;
    let histo_size: libc::c_int = VP8LGetHistogramSize(cache_bits);
    memset(p as *mut libc::c_void, 0 as libc::c_int, histo_size as libc::c_ulong);
    (*p).palette_code_bits_ = cache_bits;
    (*p).literal_ = literal;
}
unsafe extern "C" fn HistogramSwap(
    A: *mut *mut VP8LHistogram,
    B: *mut *mut VP8LHistogram,
) {
    let tmp: *mut VP8LHistogram = *A;
    *A = *B;
    *B = tmp;
}
unsafe extern "C" fn HistogramCopy(src: *const VP8LHistogram, dst: *mut VP8LHistogram) {
    let dst_literal: *mut uint32_t = (*dst).literal_;
    let dst_cache_bits: libc::c_int = (*dst).palette_code_bits_;
    let literal_size: libc::c_int = VP8LHistogramNumCodes(dst_cache_bits);
    let histo_size: libc::c_int = VP8LGetHistogramSize(dst_cache_bits);
    memcpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        histo_size as libc::c_ulong,
    );
    (*dst).literal_ = dst_literal;
    memcpy(
        (*dst).literal_ as *mut libc::c_void,
        (*src).literal_ as *const libc::c_void,
        (literal_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetHistogramSize(
    mut cache_bits: libc::c_int,
) -> libc::c_int {
    let literal_size: libc::c_int = VP8LHistogramNumCodes(cache_bits);
    let total_size: size_t = (::core::mem::size_of::<VP8LHistogram>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(literal_size as libc::c_ulong),
        );
    return total_size as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LFreeHistogram(histo: *mut VP8LHistogram) {
    WebPSafeFree(histo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LFreeHistogramSet(histo: *mut VP8LHistogramSet) {
    WebPSafeFree(histo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramStoreRefs(
    refs: *const VP8LBackwardRefs,
    histo: *mut VP8LHistogram,
) {
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    while VP8LRefsCursorOk(&mut c) != 0 {
        VP8LHistogramAddSinglePixOrCopy(histo, c.cur_pos, None, 0 as libc::c_int);
        VP8LRefsCursorNext(&mut c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramCreate(
    p: *mut VP8LHistogram,
    refs: *const VP8LBackwardRefs,
    mut palette_code_bits: libc::c_int,
) {
    if palette_code_bits >= 0 as libc::c_int {
        (*p).palette_code_bits_ = palette_code_bits;
    }
    HistogramClear(p);
    VP8LHistogramStoreRefs(refs, p);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramInit(
    p: *mut VP8LHistogram,
    mut palette_code_bits: libc::c_int,
    mut init_arrays: libc::c_int,
) {
    (*p).palette_code_bits_ = palette_code_bits;
    if init_arrays != 0 {
        HistogramClear(p);
    } else {
        (*p).trivial_symbol_ = 0 as libc::c_int as uint32_t;
        (*p).bit_cost_ = 0.0f64 as libc::c_float;
        (*p).literal_cost_ = 0.0f64 as libc::c_float;
        (*p).red_cost_ = 0.0f64 as libc::c_float;
        (*p).blue_cost_ = 0.0f64 as libc::c_float;
        memset(
            ((*p).is_used_).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LAllocateHistogram(
    mut cache_bits: libc::c_int,
) -> *mut VP8LHistogram {
    let mut histo: *mut VP8LHistogram = 0 as *mut VP8LHistogram;
    let total_size: libc::c_int = VP8LGetHistogramSize(cache_bits);
    let memory: *mut uint8_t = WebPSafeMalloc(
        total_size as uint64_t,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    if memory.is_null() {
        return 0 as *mut VP8LHistogram;
    }
    histo = memory as *mut VP8LHistogram;
    (*histo)
        .literal_ = memory
        .offset(::core::mem::size_of::<VP8LHistogram>() as libc::c_ulong as isize)
        as *mut uint32_t;
    VP8LHistogramInit(histo, cache_bits, 0 as libc::c_int);
    return histo;
}
unsafe extern "C" fn HistogramSetResetPointers(
    set: *mut VP8LHistogramSet,
    mut cache_bits: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let histo_size: libc::c_int = VP8LGetHistogramSize(cache_bits);
    let mut memory: *mut uint8_t = (*set).histograms as *mut uint8_t;
    memory = memory
        .offset(
            ((*set).max_size as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut VP8LHistogram>() as libc::c_ulong,
                ) as isize,
        );
    i = 0 as libc::c_int;
    while i < (*set).max_size {
        memory = ((memory as uintptr_t).wrapping_add(31 as libc::c_int as libc::c_ulong)
            & !(31 as libc::c_int as uintptr_t)) as *mut uint8_t;
        let ref mut fresh0 = *((*set).histograms).offset(i as isize);
        *fresh0 = memory as *mut VP8LHistogram;
        let ref mut fresh1 = (**((*set).histograms).offset(i as isize)).literal_;
        *fresh1 = memory
            .offset(::core::mem::size_of::<VP8LHistogram>() as libc::c_ulong as isize)
            as *mut uint32_t;
        memory = memory.offset(histo_size as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn HistogramSetTotalSize(
    mut size: libc::c_int,
    mut cache_bits: libc::c_int,
) -> size_t {
    let histo_size: libc::c_int = VP8LGetHistogramSize(cache_bits);
    return (::core::mem::size_of::<VP8LHistogramSet>() as libc::c_ulong)
        .wrapping_add(
            (size as libc::c_ulong)
                .wrapping_mul(
                    (::core::mem::size_of::<*mut VP8LHistogram>() as libc::c_ulong)
                        .wrapping_add(histo_size as libc::c_ulong)
                        .wrapping_add(31 as libc::c_int as libc::c_ulong),
                ),
        );
}
#[no_mangle]
pub unsafe extern "C" fn VP8LAllocateHistogramSet(
    mut size: libc::c_int,
    mut cache_bits: libc::c_int,
) -> *mut VP8LHistogramSet {
    let mut i: libc::c_int = 0;
    let mut set: *mut VP8LHistogramSet = 0 as *mut VP8LHistogramSet;
    let total_size: size_t = HistogramSetTotalSize(size, cache_bits);
    let mut memory: *mut uint8_t = WebPSafeMalloc(
        total_size,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    if memory.is_null() {
        return 0 as *mut VP8LHistogramSet;
    }
    set = memory as *mut VP8LHistogramSet;
    memory = memory
        .offset(::core::mem::size_of::<VP8LHistogramSet>() as libc::c_ulong as isize);
    (*set).histograms = memory as *mut *mut VP8LHistogram;
    (*set).max_size = size;
    (*set).size = size;
    HistogramSetResetPointers(set, cache_bits);
    i = 0 as libc::c_int;
    while i < size {
        VP8LHistogramInit(
            *((*set).histograms).offset(i as isize),
            cache_bits,
            0 as libc::c_int,
        );
        i += 1;
        i;
    }
    return set;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramSetClear(set: *mut VP8LHistogramSet) {
    let mut i: libc::c_int = 0;
    let cache_bits: libc::c_int = (**((*set).histograms)
        .offset(0 as libc::c_int as isize))
        .palette_code_bits_;
    let size: libc::c_int = (*set).max_size;
    let total_size: size_t = HistogramSetTotalSize(size, cache_bits);
    let mut memory: *mut uint8_t = set as *mut uint8_t;
    memset(memory as *mut libc::c_void, 0 as libc::c_int, total_size);
    memory = memory
        .offset(::core::mem::size_of::<VP8LHistogramSet>() as libc::c_ulong as isize);
    (*set).histograms = memory as *mut *mut VP8LHistogram;
    (*set).max_size = size;
    (*set).size = size;
    HistogramSetResetPointers(set, cache_bits);
    i = 0 as libc::c_int;
    while i < size {
        (**((*set).histograms).offset(i as isize)).palette_code_bits_ = cache_bits;
        i += 1;
        i;
    }
}
unsafe extern "C" fn HistogramSetRemoveHistogram(
    set: *mut VP8LHistogramSet,
    mut i: libc::c_int,
    num_used: *mut libc::c_int,
) {
    let ref mut fresh2 = *((*set).histograms).offset(i as isize);
    *fresh2 = 0 as *mut VP8LHistogram;
    *num_used -= 1;
    *num_used;
    if i == (*set).size - 1 as libc::c_int {
        while (*set).size >= 1 as libc::c_int
            && (*((*set).histograms).offset(((*set).size - 1 as libc::c_int) as isize))
                .is_null()
        {
            (*set).size -= 1;
            (*set).size;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramAddSinglePixOrCopy(
    histo: *mut VP8LHistogram,
    v: *const PixOrCopy,
    distance_modifier: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    mut distance_modifier_arg0: libc::c_int,
) {
    if PixOrCopyIsLiteral(v) != 0 {
        (*histo)
            .alpha_[PixOrCopyLiteral(v, 3 as libc::c_int)
            as usize] = ((*histo).alpha_[PixOrCopyLiteral(v, 3 as libc::c_int) as usize])
            .wrapping_add(1);
        (*histo).alpha_[PixOrCopyLiteral(v, 3 as libc::c_int) as usize];
        (*histo)
            .red_[PixOrCopyLiteral(v, 2 as libc::c_int)
            as usize] = ((*histo).red_[PixOrCopyLiteral(v, 2 as libc::c_int) as usize])
            .wrapping_add(1);
        (*histo).red_[PixOrCopyLiteral(v, 2 as libc::c_int) as usize];
        let ref mut fresh3 = *((*histo).literal_)
            .offset(PixOrCopyLiteral(v, 1 as libc::c_int) as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        *fresh3;
        (*histo)
            .blue_[PixOrCopyLiteral(v, 0 as libc::c_int)
            as usize] = ((*histo).blue_[PixOrCopyLiteral(v, 0 as libc::c_int) as usize])
            .wrapping_add(1);
        (*histo).blue_[PixOrCopyLiteral(v, 0 as libc::c_int) as usize];
    } else if PixOrCopyIsCacheIdx(v) != 0 {
        let literal_ix: libc::c_int = ((256 as libc::c_int + 24 as libc::c_int)
            as libc::c_uint)
            .wrapping_add(PixOrCopyCacheIdx(v)) as libc::c_int;
        let ref mut fresh4 = *((*histo).literal_).offset(literal_ix as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *fresh4;
    } else {
        let mut code: libc::c_int = 0;
        let mut extra_bits: libc::c_int = 0;
        VP8LPrefixEncodeBits(
            PixOrCopyLength(v) as libc::c_int,
            &mut code,
            &mut extra_bits,
        );
        let ref mut fresh5 = *((*histo).literal_)
            .offset((256 as libc::c_int + code) as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        *fresh5;
        if distance_modifier.is_none() {
            VP8LPrefixEncodeBits(
                PixOrCopyDistance(v) as libc::c_int,
                &mut code,
                &mut extra_bits,
            );
        } else {
            VP8LPrefixEncodeBits(
                distance_modifier
                    .expect(
                        "non-null function pointer",
                    )(distance_modifier_arg0, PixOrCopyDistance(v) as libc::c_int),
                &mut code,
                &mut extra_bits,
            );
        }
        (*histo)
            .distance_[code
            as usize] = ((*histo).distance_[code as usize]).wrapping_add(1);
        (*histo).distance_[code as usize];
    };
}
#[inline]
unsafe extern "C" fn BitsEntropyRefine(
    mut entropy: *const VP8LBitEntropy,
) -> libc::c_float {
    let mut mix: libc::c_float = 0.;
    if (*entropy).nonzeros < 5 as libc::c_int {
        if (*entropy).nonzeros <= 1 as libc::c_int {
            return 0 as libc::c_int as libc::c_float;
        }
        if (*entropy).nonzeros == 2 as libc::c_int {
            return 0.99f32 * (*entropy).sum as libc::c_float
                + 0.01f32 * (*entropy).entropy;
        }
        if (*entropy).nonzeros == 3 as libc::c_int {
            mix = 0.95f32;
        } else {
            mix = 0.7f32;
        }
    } else {
        mix = 0.627f32;
    }
    let mut min_limit: libc::c_float = 2.0f32 * (*entropy).sum as libc::c_float
        - (*entropy).max_val as libc::c_float;
    min_limit = mix * min_limit + (1.0f32 - mix) * (*entropy).entropy;
    return if (*entropy).entropy < min_limit { min_limit } else { (*entropy).entropy };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitsEntropy(
    array: *const uint32_t,
    mut n: libc::c_int,
) -> libc::c_float {
    let mut entropy: VP8LBitEntropy = VP8LBitEntropy {
        entropy: 0.,
        sum: 0,
        nonzeros: 0,
        max_val: 0,
        nonzero_code: 0,
    };
    VP8LBitsEntropyUnrefined(array, n, &mut entropy);
    return BitsEntropyRefine(&mut entropy);
}
unsafe extern "C" fn InitialHuffmanCost() -> libc::c_float {
    static mut kHuffmanCodeOfHuffmanCodeSize: libc::c_int = 19 as libc::c_int
        * 3 as libc::c_int;
    static mut kSmallBias: libc::c_float = 9.1f32;
    return kHuffmanCodeOfHuffmanCodeSize as libc::c_float - kSmallBias;
}
unsafe extern "C" fn FinalHuffmanCost(stats: *const VP8LStreaks) -> libc::c_float {
    let mut retval: libc::c_float = InitialHuffmanCost();
    retval
        += (*stats).counts[0 as libc::c_int as usize] as libc::c_float * 1.5625f32
            + 0.234375f32
                * (*stats).streaks[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_float;
    retval
        += (*stats).counts[1 as libc::c_int as usize] as libc::c_float * 2.578125f32
            + 0.703125f32
                * (*stats).streaks[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_float;
    retval
        += 1.796875f32
            * (*stats).streaks[0 as libc::c_int as usize][0 as libc::c_int as usize]
                as libc::c_float;
    retval
        += 3.28125f32
            * (*stats).streaks[1 as libc::c_int as usize][0 as libc::c_int as usize]
                as libc::c_float;
    return retval;
}
unsafe extern "C" fn PopulationCost(
    population: *const uint32_t,
    mut length: libc::c_int,
    trivial_sym: *mut uint32_t,
    is_used: *mut uint8_t,
) -> libc::c_float {
    let mut bit_entropy: VP8LBitEntropy = VP8LBitEntropy {
        entropy: 0.,
        sum: 0,
        nonzeros: 0,
        max_val: 0,
        nonzero_code: 0,
    };
    let mut stats: VP8LStreaks = VP8LStreaks {
        counts: [0; 2],
        streaks: [[0; 2]; 2],
    };
    VP8LGetEntropyUnrefined
        .expect(
            "non-null function pointer",
        )(population, length, &mut bit_entropy, &mut stats);
    if !trivial_sym.is_null() {
        *trivial_sym = if bit_entropy.nonzeros == 1 as libc::c_int {
            bit_entropy.nonzero_code
        } else {
            0xffffffff as libc::c_uint
        };
    }
    *is_used = (stats.streaks[1 as libc::c_int as usize][0 as libc::c_int as usize]
        != 0 as libc::c_int
        || stats.streaks[1 as libc::c_int as usize][1 as libc::c_int as usize]
            != 0 as libc::c_int) as libc::c_int as uint8_t;
    return BitsEntropyRefine(&mut bit_entropy) + FinalHuffmanCost(&mut stats);
}
#[inline]
unsafe extern "C" fn GetCombinedEntropy(
    X: *const uint32_t,
    Y: *const uint32_t,
    mut length: libc::c_int,
    mut is_X_used: libc::c_int,
    mut is_Y_used: libc::c_int,
    mut trivial_at_end: libc::c_int,
) -> libc::c_float {
    let mut stats: VP8LStreaks = VP8LStreaks {
        counts: [0; 2],
        streaks: [[0; 2]; 2],
    };
    if trivial_at_end != 0 {
        memset(
            &mut stats as *mut VP8LStreaks as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<VP8LStreaks>() as libc::c_ulong,
        );
        stats
            .streaks[1 as libc::c_int
            as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
        stats.counts[0 as libc::c_int as usize] = 1 as libc::c_int;
        stats
            .streaks[0 as libc::c_int
            as usize][1 as libc::c_int as usize] = length - 1 as libc::c_int;
        return FinalHuffmanCost(&mut stats);
    } else {
        let mut bit_entropy: VP8LBitEntropy = VP8LBitEntropy {
            entropy: 0.,
            sum: 0,
            nonzeros: 0,
            max_val: 0,
            nonzero_code: 0,
        };
        if is_X_used != 0 {
            if is_Y_used != 0 {
                VP8LGetCombinedEntropyUnrefined
                    .expect(
                        "non-null function pointer",
                    )(X, Y, length, &mut bit_entropy, &mut stats);
            } else {
                VP8LGetEntropyUnrefined
                    .expect(
                        "non-null function pointer",
                    )(X, length, &mut bit_entropy, &mut stats);
            }
        } else if is_Y_used != 0 {
            VP8LGetEntropyUnrefined
                .expect(
                    "non-null function pointer",
                )(Y, length, &mut bit_entropy, &mut stats);
        } else {
            memset(
                &mut stats as *mut VP8LStreaks as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<VP8LStreaks>() as libc::c_ulong,
            );
            stats.counts[0 as libc::c_int as usize] = 1 as libc::c_int;
            stats
                .streaks[0 as libc::c_int
                as usize][(length > 3 as libc::c_int) as libc::c_int as usize] = length;
            VP8LBitEntropyInit(&mut bit_entropy);
        }
        return BitsEntropyRefine(&mut bit_entropy) + FinalHuffmanCost(&mut stats);
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHistogramEstimateBits(
    p: *mut VP8LHistogram,
) -> libc::c_float {
    return PopulationCost(
        (*p).literal_,
        VP8LHistogramNumCodes((*p).palette_code_bits_),
        0 as *mut uint32_t,
        &mut *((*p).is_used_).as_mut_ptr().offset(0 as libc::c_int as isize),
    )
        + PopulationCost(
            ((*p).red_).as_mut_ptr(),
            256 as libc::c_int,
            0 as *mut uint32_t,
            &mut *((*p).is_used_).as_mut_ptr().offset(1 as libc::c_int as isize),
        )
        + PopulationCost(
            ((*p).blue_).as_mut_ptr(),
            256 as libc::c_int,
            0 as *mut uint32_t,
            &mut *((*p).is_used_).as_mut_ptr().offset(2 as libc::c_int as isize),
        )
        + PopulationCost(
            ((*p).alpha_).as_mut_ptr(),
            256 as libc::c_int,
            0 as *mut uint32_t,
            &mut *((*p).is_used_).as_mut_ptr().offset(3 as libc::c_int as isize),
        )
        + PopulationCost(
            ((*p).distance_).as_mut_ptr(),
            40 as libc::c_int,
            0 as *mut uint32_t,
            &mut *((*p).is_used_).as_mut_ptr().offset(4 as libc::c_int as isize),
        )
        + VP8LExtraCost
            .expect(
                "non-null function pointer",
            )(((*p).literal_).offset(256 as libc::c_int as isize), 24 as libc::c_int)
            as libc::c_float
        + VP8LExtraCost
            .expect(
                "non-null function pointer",
            )(((*p).distance_).as_mut_ptr(), 40 as libc::c_int) as libc::c_float;
}
unsafe extern "C" fn GetCombinedHistogramEntropy(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    mut cost_threshold: libc::c_float,
    mut cost: *mut libc::c_float,
) -> libc::c_int {
    let palette_code_bits: libc::c_int = (*a).palette_code_bits_;
    let mut trivial_at_end: libc::c_int = 0 as libc::c_int;
    *cost
        += GetCombinedEntropy(
            (*a).literal_,
            (*b).literal_,
            VP8LHistogramNumCodes(palette_code_bits),
            (*a).is_used_[0 as libc::c_int as usize] as libc::c_int,
            (*b).is_used_[0 as libc::c_int as usize] as libc::c_int,
            0 as libc::c_int,
        );
    *cost
        += VP8LExtraCostCombined
            .expect(
                "non-null function pointer",
            )(
            ((*a).literal_).offset(256 as libc::c_int as isize),
            ((*b).literal_).offset(256 as libc::c_int as isize),
            24 as libc::c_int,
        ) as libc::c_float;
    if *cost > cost_threshold {
        return 0 as libc::c_int;
    }
    if (*a).trivial_symbol_ != 0xffffffff as libc::c_uint
        && (*a).trivial_symbol_ == (*b).trivial_symbol_
    {
        let color_a: uint32_t = (*a).trivial_symbol_ >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint;
        let color_r: uint32_t = (*a).trivial_symbol_ >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint;
        let color_b: uint32_t = (*a).trivial_symbol_ >> 0 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint;
        if (color_a == 0 as libc::c_int as libc::c_uint
            || color_a == 0xff as libc::c_int as libc::c_uint)
            && (color_r == 0 as libc::c_int as libc::c_uint
                || color_r == 0xff as libc::c_int as libc::c_uint)
            && (color_b == 0 as libc::c_int as libc::c_uint
                || color_b == 0xff as libc::c_int as libc::c_uint)
        {
            trivial_at_end = 1 as libc::c_int;
        }
    }
    *cost
        += GetCombinedEntropy(
            ((*a).red_).as_ptr(),
            ((*b).red_).as_ptr(),
            256 as libc::c_int,
            (*a).is_used_[1 as libc::c_int as usize] as libc::c_int,
            (*b).is_used_[1 as libc::c_int as usize] as libc::c_int,
            trivial_at_end,
        );
    if *cost > cost_threshold {
        return 0 as libc::c_int;
    }
    *cost
        += GetCombinedEntropy(
            ((*a).blue_).as_ptr(),
            ((*b).blue_).as_ptr(),
            256 as libc::c_int,
            (*a).is_used_[2 as libc::c_int as usize] as libc::c_int,
            (*b).is_used_[2 as libc::c_int as usize] as libc::c_int,
            trivial_at_end,
        );
    if *cost > cost_threshold {
        return 0 as libc::c_int;
    }
    *cost
        += GetCombinedEntropy(
            ((*a).alpha_).as_ptr(),
            ((*b).alpha_).as_ptr(),
            256 as libc::c_int,
            (*a).is_used_[3 as libc::c_int as usize] as libc::c_int,
            (*b).is_used_[3 as libc::c_int as usize] as libc::c_int,
            trivial_at_end,
        );
    if *cost > cost_threshold {
        return 0 as libc::c_int;
    }
    *cost
        += GetCombinedEntropy(
            ((*a).distance_).as_ptr(),
            ((*b).distance_).as_ptr(),
            40 as libc::c_int,
            (*a).is_used_[4 as libc::c_int as usize] as libc::c_int,
            (*b).is_used_[4 as libc::c_int as usize] as libc::c_int,
            0 as libc::c_int,
        );
    *cost
        += VP8LExtraCostCombined
            .expect(
                "non-null function pointer",
            )(((*a).distance_).as_ptr(), ((*b).distance_).as_ptr(), 40 as libc::c_int)
            as libc::c_float;
    if *cost > cost_threshold {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn HistogramAdd(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    out: *mut VP8LHistogram,
) {
    VP8LHistogramAdd(a, b, out);
    (*out)
        .trivial_symbol_ = if (*a).trivial_symbol_ == (*b).trivial_symbol_ {
        (*a).trivial_symbol_
    } else {
        0xffffffff as libc::c_uint
    };
}
unsafe extern "C" fn HistogramAddEval(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    out: *mut VP8LHistogram,
    mut cost_threshold: libc::c_float,
) -> libc::c_float {
    let mut cost: libc::c_float = 0 as libc::c_int as libc::c_float;
    let sum_cost: libc::c_float = (*a).bit_cost_ + (*b).bit_cost_;
    cost_threshold += sum_cost;
    if GetCombinedHistogramEntropy(a, b, cost_threshold, &mut cost) != 0 {
        HistogramAdd(a, b, out);
        (*out).bit_cost_ = cost;
        (*out).palette_code_bits_ = (*a).palette_code_bits_;
    }
    return cost - sum_cost;
}
unsafe extern "C" fn HistogramAddThresh(
    a: *const VP8LHistogram,
    b: *const VP8LHistogram,
    mut cost_threshold: libc::c_float,
) -> libc::c_float {
    let mut cost: libc::c_float = 0.;
    cost = -(*a).bit_cost_;
    GetCombinedHistogramEntropy(a, b, cost_threshold, &mut cost);
    return cost;
}
unsafe extern "C" fn DominantCostRangeInit(c: *mut DominantCostRange) {
    (*c).literal_max_ = 0.0f64 as libc::c_float;
    (*c).literal_min_ = 3.40282347e+38f32;
    (*c).red_max_ = 0.0f64 as libc::c_float;
    (*c).red_min_ = 3.40282347e+38f32;
    (*c).blue_max_ = 0.0f64 as libc::c_float;
    (*c).blue_min_ = 3.40282347e+38f32;
}
unsafe extern "C" fn UpdateDominantCostRange(
    h: *const VP8LHistogram,
    c: *mut DominantCostRange,
) {
    if (*c).literal_max_ < (*h).literal_cost_ {
        (*c).literal_max_ = (*h).literal_cost_;
    }
    if (*c).literal_min_ > (*h).literal_cost_ {
        (*c).literal_min_ = (*h).literal_cost_;
    }
    if (*c).red_max_ < (*h).red_cost_ {
        (*c).red_max_ = (*h).red_cost_;
    }
    if (*c).red_min_ > (*h).red_cost_ {
        (*c).red_min_ = (*h).red_cost_;
    }
    if (*c).blue_max_ < (*h).blue_cost_ {
        (*c).blue_max_ = (*h).blue_cost_;
    }
    if (*c).blue_min_ > (*h).blue_cost_ {
        (*c).blue_min_ = (*h).blue_cost_;
    }
}
unsafe extern "C" fn UpdateHistogramCost(h: *mut VP8LHistogram) {
    let mut alpha_sym: uint32_t = 0;
    let mut red_sym: uint32_t = 0;
    let mut blue_sym: uint32_t = 0;
    let alpha_cost: libc::c_float = PopulationCost(
        ((*h).alpha_).as_mut_ptr(),
        256 as libc::c_int,
        &mut alpha_sym,
        &mut *((*h).is_used_).as_mut_ptr().offset(3 as libc::c_int as isize),
    );
    let distance_cost: libc::c_float = PopulationCost(
        ((*h).distance_).as_mut_ptr(),
        40 as libc::c_int,
        0 as *mut uint32_t,
        &mut *((*h).is_used_).as_mut_ptr().offset(4 as libc::c_int as isize),
    )
        + VP8LExtraCost
            .expect(
                "non-null function pointer",
            )(((*h).distance_).as_mut_ptr(), 40 as libc::c_int) as libc::c_float;
    let num_codes: libc::c_int = VP8LHistogramNumCodes((*h).palette_code_bits_);
    (*h)
        .literal_cost_ = PopulationCost(
        (*h).literal_,
        num_codes,
        0 as *mut uint32_t,
        &mut *((*h).is_used_).as_mut_ptr().offset(0 as libc::c_int as isize),
    )
        + VP8LExtraCost
            .expect(
                "non-null function pointer",
            )(((*h).literal_).offset(256 as libc::c_int as isize), 24 as libc::c_int)
            as libc::c_float;
    (*h)
        .red_cost_ = PopulationCost(
        ((*h).red_).as_mut_ptr(),
        256 as libc::c_int,
        &mut red_sym,
        &mut *((*h).is_used_).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    (*h)
        .blue_cost_ = PopulationCost(
        ((*h).blue_).as_mut_ptr(),
        256 as libc::c_int,
        &mut blue_sym,
        &mut *((*h).is_used_).as_mut_ptr().offset(2 as libc::c_int as isize),
    );
    (*h)
        .bit_cost_ = (*h).literal_cost_ + (*h).red_cost_ + (*h).blue_cost_ + alpha_cost
        + distance_cost;
    if alpha_sym | red_sym | blue_sym == 0xffffffff as libc::c_uint {
        (*h).trivial_symbol_ = 0xffffffff as libc::c_uint;
    } else {
        (*h)
            .trivial_symbol_ = alpha_sym << 24 as libc::c_int
            | red_sym << 16 as libc::c_int | blue_sym << 0 as libc::c_int;
    };
}
unsafe extern "C" fn GetBinIdForEntropy(
    mut min: libc::c_float,
    mut max: libc::c_float,
    mut val: libc::c_float,
) -> libc::c_int {
    let range: libc::c_float = max - min;
    if range as libc::c_double > 0.0f64 {
        let delta: libc::c_float = val - min;
        return ((4 as libc::c_int as libc::c_double - 1e-6f64) * delta as libc::c_double
            / range as libc::c_double) as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn GetHistoBinIndex(
    h: *const VP8LHistogram,
    c: *const DominantCostRange,
    mut low_effort: libc::c_int,
) -> libc::c_int {
    let mut bin_id: libc::c_int = GetBinIdForEntropy(
        (*c).literal_min_,
        (*c).literal_max_,
        (*h).literal_cost_,
    );
    if low_effort == 0 {
        bin_id = bin_id * 4 as libc::c_int
            + GetBinIdForEntropy((*c).red_min_, (*c).red_max_, (*h).red_cost_);
        bin_id = bin_id * 4 as libc::c_int
            + GetBinIdForEntropy((*c).blue_min_, (*c).blue_max_, (*h).blue_cost_);
    }
    return bin_id;
}
unsafe extern "C" fn HistogramBuild(
    mut xsize: libc::c_int,
    mut histo_bits: libc::c_int,
    backward_refs: *const VP8LBackwardRefs,
    image_histo: *mut VP8LHistogramSet,
) {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let histo_xsize: libc::c_int = VP8LSubSampleSize(
        xsize as uint32_t,
        histo_bits as uint32_t,
    ) as libc::c_int;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(backward_refs);
    VP8LHistogramSetClear(image_histo);
    while VP8LRefsCursorOk(&mut c) != 0 {
        let v: *const PixOrCopy = c.cur_pos;
        let ix: libc::c_int = (y >> histo_bits) * histo_xsize + (x >> histo_bits);
        VP8LHistogramAddSinglePixOrCopy(
            *histograms.offset(ix as isize),
            v,
            None,
            0 as libc::c_int,
        );
        x = (x as libc::c_uint).wrapping_add(PixOrCopyLength(v)) as libc::c_int
            as libc::c_int;
        while x >= xsize {
            x -= xsize;
            y += 1;
            y;
        }
        VP8LRefsCursorNext(&mut c);
    }
}
static mut kInvalidHistogramSymbol: uint16_t = -(1 as libc::c_int) as uint16_t;
unsafe extern "C" fn HistogramCopyAndAnalyze(
    orig_histo: *mut VP8LHistogramSet,
    image_histo: *mut VP8LHistogramSet,
    num_used: *mut libc::c_int,
    histogram_symbols: *mut uint16_t,
) {
    let mut i: libc::c_int = 0;
    let mut cluster_id: libc::c_int = 0;
    let mut num_used_orig: libc::c_int = *num_used;
    let orig_histograms: *mut *mut VP8LHistogram = (*orig_histo).histograms;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    cluster_id = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*orig_histo).max_size {
        let histo: *mut VP8LHistogram = *orig_histograms.offset(i as isize);
        UpdateHistogramCost(histo);
        if (*histo).is_used_[0 as libc::c_int as usize] == 0
            && (*histo).is_used_[1 as libc::c_int as usize] == 0
            && (*histo).is_used_[2 as libc::c_int as usize] == 0
            && (*histo).is_used_[3 as libc::c_int as usize] == 0
            && (*histo).is_used_[4 as libc::c_int as usize] == 0
        {
            HistogramSetRemoveHistogram(image_histo, i, num_used);
            HistogramSetRemoveHistogram(orig_histo, i, &mut num_used_orig);
            *histogram_symbols.offset(i as isize) = kInvalidHistogramSymbol;
        } else {
            HistogramCopy(histo, *histograms.offset(i as isize));
            let fresh6 = cluster_id;
            cluster_id = cluster_id + 1;
            *histogram_symbols.offset(i as isize) = fresh6 as uint16_t;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn HistogramAnalyzeEntropyBin(
    image_histo: *mut VP8LHistogramSet,
    bin_map: *mut uint16_t,
    mut low_effort: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let histo_size: libc::c_int = (*image_histo).size;
    let mut cost_range: DominantCostRange = DominantCostRange {
        literal_max_: 0.,
        literal_min_: 0.,
        red_max_: 0.,
        red_min_: 0.,
        blue_max_: 0.,
        blue_min_: 0.,
    };
    DominantCostRangeInit(&mut cost_range);
    i = 0 as libc::c_int;
    while i < histo_size {
        if !(*histograms.offset(i as isize)).is_null() {
            UpdateDominantCostRange(*histograms.offset(i as isize), &mut cost_range);
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < histo_size {
        if !(*histograms.offset(i as isize)).is_null() {
            *bin_map
                .offset(
                    i as isize,
                ) = GetHistoBinIndex(
                *histograms.offset(i as isize),
                &mut cost_range,
                low_effort,
            ) as uint16_t;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn HistogramCombineEntropyBin(
    image_histo: *mut VP8LHistogramSet,
    mut num_used: *mut libc::c_int,
    clusters: *const uint16_t,
    cluster_mappings: *mut uint16_t,
    mut cur_combo: *mut VP8LHistogram,
    bin_map: *const uint16_t,
    mut num_bins: libc::c_int,
    mut combine_cost_factor: libc::c_float,
    mut low_effort: libc::c_int,
) {
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut idx: libc::c_int = 0;
    let mut bin_info: [C2RustUnnamed; 64] = [C2RustUnnamed {
        first: 0,
        num_combine_failures: 0,
    }; 64];
    idx = 0 as libc::c_int;
    while idx < num_bins {
        bin_info[idx as usize].first = -(1 as libc::c_int) as int16_t;
        bin_info[idx as usize].num_combine_failures = 0 as libc::c_int as uint16_t;
        idx += 1;
        idx;
    }
    idx = 0 as libc::c_int;
    while idx < *num_used {
        *cluster_mappings.offset(idx as isize) = idx as uint16_t;
        idx += 1;
        idx;
    }
    idx = 0 as libc::c_int;
    while idx < (*image_histo).size {
        let mut bin_id: libc::c_int = 0;
        let mut first: libc::c_int = 0;
        if !(*histograms.offset(idx as isize)).is_null() {
            bin_id = *bin_map.offset(idx as isize) as libc::c_int;
            first = bin_info[bin_id as usize].first as libc::c_int;
            if first == -(1 as libc::c_int) {
                bin_info[bin_id as usize].first = idx as int16_t;
            } else if low_effort != 0 {
                HistogramAdd(
                    *histograms.offset(idx as isize),
                    *histograms.offset(first as isize),
                    *histograms.offset(first as isize),
                );
                HistogramSetRemoveHistogram(image_histo, idx, num_used);
                *cluster_mappings
                    .offset(
                        *clusters.offset(idx as isize) as isize,
                    ) = *clusters.offset(first as isize);
            } else {
                let bit_cost: libc::c_float = (**histograms.offset(idx as isize))
                    .bit_cost_;
                let bit_cost_thresh: libc::c_float = -bit_cost * combine_cost_factor;
                let curr_cost_diff: libc::c_float = HistogramAddEval(
                    *histograms.offset(first as isize),
                    *histograms.offset(idx as isize),
                    cur_combo,
                    bit_cost_thresh,
                );
                if curr_cost_diff < bit_cost_thresh {
                    let try_combine: libc::c_int = ((*cur_combo).trivial_symbol_
                        != 0xffffffff as libc::c_uint
                        || (**histograms.offset(idx as isize)).trivial_symbol_
                            == 0xffffffff as libc::c_uint
                            && (**histograms.offset(first as isize)).trivial_symbol_
                                == 0xffffffff as libc::c_uint) as libc::c_int;
                    let max_combine_failures: libc::c_int = 32 as libc::c_int;
                    if try_combine != 0
                        || bin_info[bin_id as usize].num_combine_failures as libc::c_int
                            >= max_combine_failures
                    {
                        HistogramSwap(
                            &mut cur_combo,
                            &mut *histograms.offset(first as isize),
                        );
                        HistogramSetRemoveHistogram(image_histo, idx, num_used);
                        *cluster_mappings
                            .offset(
                                *clusters.offset(idx as isize) as isize,
                            ) = *clusters.offset(first as isize);
                    } else {
                        bin_info[bin_id as usize]
                            .num_combine_failures = (bin_info[bin_id as usize]
                            .num_combine_failures)
                            .wrapping_add(1);
                        bin_info[bin_id as usize].num_combine_failures;
                    }
                }
            }
        }
        idx += 1;
        idx;
    }
    if low_effort != 0 {
        idx = 0 as libc::c_int;
        while idx < (*image_histo).size {
            if !(*histograms.offset(idx as isize)).is_null() {
                UpdateHistogramCost(*histograms.offset(idx as isize));
            }
            idx += 1;
            idx;
        }
    }
}
unsafe extern "C" fn MyRand(seed: *mut uint32_t) -> uint32_t {
    *seed = (*seed as uint64_t)
        .wrapping_mul(48271 as libc::c_uint as libc::c_ulong)
        .wrapping_rem(2147483647 as libc::c_uint as libc::c_ulong) as uint32_t;
    return *seed;
}
unsafe extern "C" fn HistoQueueInit(
    histo_queue: *mut HistoQueue,
    max_size: libc::c_int,
) -> libc::c_int {
    (*histo_queue).size = 0 as libc::c_int;
    (*histo_queue).max_size = max_size;
    (*histo_queue)
        .queue = WebPSafeMalloc(
        ((*histo_queue).max_size + 1 as libc::c_int) as uint64_t,
        ::core::mem::size_of::<HistogramPair>() as libc::c_ulong,
    ) as *mut HistogramPair;
    return ((*histo_queue).queue != 0 as *mut libc::c_void as *mut HistogramPair)
        as libc::c_int;
}
unsafe extern "C" fn HistoQueueClear(histo_queue: *mut HistoQueue) {
    WebPSafeFree((*histo_queue).queue as *mut libc::c_void);
    (*histo_queue).size = 0 as libc::c_int;
    (*histo_queue).max_size = 0 as libc::c_int;
}
unsafe extern "C" fn HistoQueuePopPair(
    histo_queue: *mut HistoQueue,
    pair: *mut HistogramPair,
) {
    *pair = *((*histo_queue).queue)
        .offset(((*histo_queue).size - 1 as libc::c_int) as isize);
    (*histo_queue).size -= 1;
    (*histo_queue).size;
}
unsafe extern "C" fn HistoQueueUpdateHead(
    histo_queue: *mut HistoQueue,
    pair: *mut HistogramPair,
) {
    if (*pair).cost_diff
        < (*((*histo_queue).queue).offset(0 as libc::c_int as isize)).cost_diff
    {
        let tmp: HistogramPair = *((*histo_queue).queue)
            .offset(0 as libc::c_int as isize);
        *((*histo_queue).queue).offset(0 as libc::c_int as isize) = *pair;
        *pair = tmp;
    }
}
unsafe extern "C" fn HistoQueueUpdatePair(
    h1: *const VP8LHistogram,
    h2: *const VP8LHistogram,
    mut threshold: libc::c_float,
    pair: *mut HistogramPair,
) {
    let sum_cost: libc::c_float = (*h1).bit_cost_ + (*h2).bit_cost_;
    (*pair).cost_combo = 0.0f64 as libc::c_float;
    GetCombinedHistogramEntropy(h1, h2, sum_cost + threshold, &mut (*pair).cost_combo);
    (*pair).cost_diff = (*pair).cost_combo - sum_cost;
}
unsafe extern "C" fn HistoQueuePush(
    histo_queue: *mut HistoQueue,
    histograms: *mut *mut VP8LHistogram,
    mut idx1: libc::c_int,
    mut idx2: libc::c_int,
    mut threshold: libc::c_float,
) -> libc::c_float {
    let mut h1: *const VP8LHistogram = 0 as *const VP8LHistogram;
    let mut h2: *const VP8LHistogram = 0 as *const VP8LHistogram;
    let mut pair: HistogramPair = HistogramPair {
        idx1: 0,
        idx2: 0,
        cost_diff: 0.,
        cost_combo: 0.,
    };
    if (*histo_queue).size == (*histo_queue).max_size {
        return 0.0f64 as libc::c_float;
    }
    if idx1 > idx2 {
        let tmp: libc::c_int = idx2;
        idx2 = idx1;
        idx1 = tmp;
    }
    pair.idx1 = idx1;
    pair.idx2 = idx2;
    h1 = *histograms.offset(idx1 as isize);
    h2 = *histograms.offset(idx2 as isize);
    HistoQueueUpdatePair(h1, h2, threshold, &mut pair);
    if pair.cost_diff >= threshold {
        return 0.0f64 as libc::c_float;
    }
    let fresh7 = (*histo_queue).size;
    (*histo_queue).size = (*histo_queue).size + 1;
    *((*histo_queue).queue).offset(fresh7 as isize) = pair;
    HistoQueueUpdateHead(
        histo_queue,
        &mut *((*histo_queue).queue)
            .offset(((*histo_queue).size - 1 as libc::c_int) as isize),
    );
    return pair.cost_diff;
}
unsafe extern "C" fn HistogramCombineGreedy(
    image_histo: *mut VP8LHistogramSet,
    num_used: *mut libc::c_int,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let image_histo_size: libc::c_int = (*image_histo).size;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut histo_queue: HistoQueue = HistoQueue {
        queue: 0 as *mut HistogramPair,
        size: 0,
        max_size: 0,
    };
    if !(HistoQueueInit(&mut histo_queue, image_histo_size * image_histo_size) == 0) {
        i = 0 as libc::c_int;
        while i < image_histo_size {
            if !(*((*image_histo).histograms).offset(i as isize)).is_null() {
                j = i + 1 as libc::c_int;
                while j < image_histo_size {
                    if !(*((*image_histo).histograms).offset(j as isize)).is_null() {
                        HistoQueuePush(
                            &mut histo_queue,
                            histograms,
                            i,
                            j,
                            0.0f64 as libc::c_float,
                        );
                    }
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
        while histo_queue.size > 0 as libc::c_int {
            let idx1: libc::c_int = (*(histo_queue.queue)
                .offset(0 as libc::c_int as isize))
                .idx1;
            let idx2: libc::c_int = (*(histo_queue.queue)
                .offset(0 as libc::c_int as isize))
                .idx2;
            HistogramAdd(
                *histograms.offset(idx2 as isize),
                *histograms.offset(idx1 as isize),
                *histograms.offset(idx1 as isize),
            );
            (**histograms.offset(idx1 as isize))
                .bit_cost_ = (*(histo_queue.queue).offset(0 as libc::c_int as isize))
                .cost_combo;
            HistogramSetRemoveHistogram(image_histo, idx2, num_used);
            i = 0 as libc::c_int;
            while i < histo_queue.size {
                let p: *mut HistogramPair = (histo_queue.queue).offset(i as isize);
                if (*p).idx1 == idx1 || (*p).idx2 == idx1 || (*p).idx1 == idx2
                    || (*p).idx2 == idx2
                {
                    HistoQueuePopPair(&mut histo_queue, p);
                } else {
                    HistoQueueUpdateHead(&mut histo_queue, p);
                    i += 1;
                    i;
                }
            }
            i = 0 as libc::c_int;
            while i < (*image_histo).size {
                if !(i == idx1
                    || (*((*image_histo).histograms).offset(i as isize)).is_null())
                {
                    HistoQueuePush(
                        &mut histo_queue,
                        (*image_histo).histograms,
                        idx1,
                        i,
                        0.0f64 as libc::c_float,
                    );
                }
                i += 1;
                i;
            }
        }
        ok = 1 as libc::c_int;
    }
    HistoQueueClear(&mut histo_queue);
    return ok;
}
unsafe extern "C" fn PairComparison(
    mut idx1: *const libc::c_void,
    mut idx2: *const libc::c_void,
) -> libc::c_int {
    return *(idx1 as *mut libc::c_int) - *(idx2 as *mut libc::c_int);
}
unsafe extern "C" fn HistogramCombineStochastic(
    image_histo: *mut VP8LHistogramSet,
    num_used: *mut libc::c_int,
    mut min_cluster_size: libc::c_int,
    do_greedy: *mut libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut seed: uint32_t = 1 as libc::c_int as uint32_t;
    let mut tries_with_no_success: libc::c_int = 0 as libc::c_int;
    let outer_iters: libc::c_int = *num_used;
    let num_tries_no_success: libc::c_int = outer_iters / 2 as libc::c_int;
    let histograms: *mut *mut VP8LHistogram = (*image_histo).histograms;
    let mut histo_queue: HistoQueue = HistoQueue {
        queue: 0 as *mut HistogramPair,
        size: 0,
        max_size: 0,
    };
    let kHistoQueueSize: libc::c_int = 9 as libc::c_int;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut mappings: *mut libc::c_int = 0 as *mut libc::c_int;
    if *num_used < min_cluster_size {
        *do_greedy = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    mappings = WebPSafeMalloc(
        *num_used as uint64_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if mappings.is_null() {
        return 0 as libc::c_int;
    }
    if !(HistoQueueInit(&mut histo_queue, kHistoQueueSize) == 0) {
        j = 0 as libc::c_int;
        iter = 0 as libc::c_int;
        while iter < (*image_histo).size {
            if !(*histograms.offset(iter as isize)).is_null() {
                let fresh8 = j;
                j = j + 1;
                *mappings.offset(fresh8 as isize) = iter;
            }
            iter += 1;
            iter;
        }
        iter = 0 as libc::c_int;
        while iter < outer_iters && *num_used >= min_cluster_size
            && {
                tries_with_no_success += 1;
                tries_with_no_success < num_tries_no_success
            }
        {
            let mut mapping_index: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut best_cost: libc::c_float = if histo_queue.size == 0 as libc::c_int {
                0.0f32
            } else {
                (*(histo_queue.queue).offset(0 as libc::c_int as isize)).cost_diff
            };
            let mut best_idx1: libc::c_int = -(1 as libc::c_int);
            let mut best_idx2: libc::c_int = 1 as libc::c_int;
            let rand_range: uint32_t = ((*num_used - 1 as libc::c_int) * *num_used)
                as uint32_t;
            let num_tries: libc::c_int = *num_used / 2 as libc::c_int;
            j = 0 as libc::c_int;
            while *num_used >= 2 as libc::c_int && j < num_tries {
                let mut curr_cost: libc::c_float = 0.;
                let tmp: uint32_t = (MyRand(&mut seed)).wrapping_rem(rand_range);
                let mut idx1: uint32_t = tmp
                    .wrapping_div((*num_used - 1 as libc::c_int) as libc::c_uint);
                let mut idx2: uint32_t = tmp
                    .wrapping_rem((*num_used - 1 as libc::c_int) as libc::c_uint);
                if idx2 >= idx1 {
                    idx2 = idx2.wrapping_add(1);
                    idx2;
                }
                idx1 = *mappings.offset(idx1 as isize) as uint32_t;
                idx2 = *mappings.offset(idx2 as isize) as uint32_t;
                curr_cost = HistoQueuePush(
                    &mut histo_queue,
                    histograms,
                    idx1 as libc::c_int,
                    idx2 as libc::c_int,
                    best_cost,
                );
                if curr_cost < 0 as libc::c_int as libc::c_float {
                    best_cost = curr_cost;
                    if histo_queue.size == histo_queue.max_size {
                        break;
                    }
                }
                j += 1;
                j;
            }
            if !(histo_queue.size == 0 as libc::c_int) {
                best_idx1 = (*(histo_queue.queue).offset(0 as libc::c_int as isize))
                    .idx1;
                best_idx2 = (*(histo_queue.queue).offset(0 as libc::c_int as isize))
                    .idx2;
                mapping_index = bsearch(
                    &mut best_idx2 as *mut libc::c_int as *const libc::c_void,
                    mappings as *const libc::c_void,
                    *num_used as size_t,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    Some(
                        PairComparison
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                ) as *mut libc::c_int;
                memmove(
                    mapping_index as *mut libc::c_void,
                    mapping_index.offset(1 as libc::c_int as isize)
                        as *const libc::c_void,
                    (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(
                            (*num_used as libc::c_long
                                - mapping_index.offset_from(mappings) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                        ),
                );
                HistogramAdd(
                    *histograms.offset(best_idx2 as isize),
                    *histograms.offset(best_idx1 as isize),
                    *histograms.offset(best_idx1 as isize),
                );
                (**histograms.offset(best_idx1 as isize))
                    .bit_cost_ = (*(histo_queue.queue).offset(0 as libc::c_int as isize))
                    .cost_combo;
                HistogramSetRemoveHistogram(image_histo, best_idx2, num_used);
                j = 0 as libc::c_int;
                while j < histo_queue.size {
                    let p: *mut HistogramPair = (histo_queue.queue).offset(j as isize);
                    let is_idx1_best: libc::c_int = ((*p).idx1 == best_idx1
                        || (*p).idx1 == best_idx2) as libc::c_int;
                    let is_idx2_best: libc::c_int = ((*p).idx2 == best_idx1
                        || (*p).idx2 == best_idx2) as libc::c_int;
                    let mut do_eval: libc::c_int = 0 as libc::c_int;
                    if is_idx1_best != 0 && is_idx2_best != 0 {
                        HistoQueuePopPair(&mut histo_queue, p);
                    } else {
                        if is_idx1_best != 0 {
                            (*p).idx1 = best_idx1;
                            do_eval = 1 as libc::c_int;
                        } else if is_idx2_best != 0 {
                            (*p).idx2 = best_idx1;
                            do_eval = 1 as libc::c_int;
                        }
                        if (*p).idx1 > (*p).idx2 {
                            let tmp_0: libc::c_int = (*p).idx2;
                            (*p).idx2 = (*p).idx1;
                            (*p).idx1 = tmp_0;
                        }
                        if do_eval != 0 {
                            HistoQueueUpdatePair(
                                *histograms.offset((*p).idx1 as isize),
                                *histograms.offset((*p).idx2 as isize),
                                0.0f64 as libc::c_float,
                                p,
                            );
                            if (*p).cost_diff as libc::c_double >= 0.0f64 {
                                HistoQueuePopPair(&mut histo_queue, p);
                                continue;
                            }
                        }
                        HistoQueueUpdateHead(&mut histo_queue, p);
                        j += 1;
                        j;
                    }
                }
                tries_with_no_success = 0 as libc::c_int;
            }
            iter += 1;
            iter;
        }
        *do_greedy = (*num_used <= min_cluster_size) as libc::c_int;
        ok = 1 as libc::c_int;
    }
    HistoQueueClear(&mut histo_queue);
    WebPSafeFree(mappings as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn HistogramRemap(
    in_0: *const VP8LHistogramSet,
    out: *mut VP8LHistogramSet,
    symbols: *mut uint16_t,
) {
    let mut i: libc::c_int = 0;
    let in_histo: *mut *mut VP8LHistogram = (*in_0).histograms;
    let out_histo: *mut *mut VP8LHistogram = (*out).histograms;
    let in_size: libc::c_int = (*out).max_size;
    let out_size: libc::c_int = (*out).size;
    if out_size > 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < in_size {
            let mut best_out: libc::c_int = 0 as libc::c_int;
            let mut best_bits: libc::c_float = 3.40282347e+38f32;
            let mut k: libc::c_int = 0;
            if (*in_histo.offset(i as isize)).is_null() {
                *symbols
                    .offset(
                        i as isize,
                    ) = *symbols.offset((i - 1 as libc::c_int) as isize);
            } else {
                k = 0 as libc::c_int;
                while k < out_size {
                    let mut cur_bits: libc::c_float = 0.;
                    cur_bits = HistogramAddThresh(
                        *out_histo.offset(k as isize),
                        *in_histo.offset(i as isize),
                        best_bits,
                    );
                    if k == 0 as libc::c_int || cur_bits < best_bits {
                        best_bits = cur_bits;
                        best_out = k;
                    }
                    k += 1;
                    k;
                }
                *symbols.offset(i as isize) = best_out as uint16_t;
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < in_size {
            *symbols.offset(i as isize) = 0 as libc::c_int as uint16_t;
            i += 1;
            i;
        }
    }
    VP8LHistogramSetClear(out);
    (*out).size = out_size;
    i = 0 as libc::c_int;
    while i < in_size {
        let mut idx: libc::c_int = 0;
        if !(*in_histo.offset(i as isize)).is_null() {
            idx = *symbols.offset(i as isize) as libc::c_int;
            HistogramAdd(
                *in_histo.offset(i as isize),
                *out_histo.offset(idx as isize),
                *out_histo.offset(idx as isize),
            );
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn GetCombineCostFactor(
    mut histo_size: libc::c_int,
    mut quality: libc::c_int,
) -> libc::c_float {
    let mut combine_cost_factor: libc::c_float = 0.16f32;
    if quality < 90 as libc::c_int {
        if histo_size > 256 as libc::c_int {
            combine_cost_factor /= 2.0f32;
        }
        if histo_size > 512 as libc::c_int {
            combine_cost_factor /= 2.0f32;
        }
        if histo_size > 1024 as libc::c_int {
            combine_cost_factor /= 2.0f32;
        }
        if quality <= 50 as libc::c_int {
            combine_cost_factor /= 2.0f32;
        }
    }
    return combine_cost_factor;
}
unsafe extern "C" fn OptimizeHistogramSymbols(
    set: *const VP8LHistogramSet,
    cluster_mappings: *mut uint16_t,
    mut num_clusters: libc::c_int,
    cluster_mappings_tmp: *mut uint16_t,
    symbols: *mut uint16_t,
) {
    let mut i: libc::c_int = 0;
    let mut cluster_max: libc::c_int = 0;
    let mut do_continue: libc::c_int = 1 as libc::c_int;
    while do_continue != 0 {
        do_continue = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < num_clusters {
            let mut k: libc::c_int = 0;
            k = *cluster_mappings.offset(i as isize) as libc::c_int;
            while k != *cluster_mappings.offset(k as isize) as libc::c_int {
                *cluster_mappings
                    .offset(
                        k as isize,
                    ) = *cluster_mappings
                    .offset(*cluster_mappings.offset(k as isize) as isize);
                k = *cluster_mappings.offset(k as isize) as libc::c_int;
            }
            if k != *cluster_mappings.offset(i as isize) as libc::c_int {
                do_continue = 1 as libc::c_int;
                *cluster_mappings.offset(i as isize) = k as uint16_t;
            }
            i += 1;
            i;
        }
    }
    cluster_max = 0 as libc::c_int;
    memset(
        cluster_mappings_tmp as *mut libc::c_void,
        0 as libc::c_int,
        ((*set).max_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*set).max_size {
        let mut cluster: libc::c_int = 0;
        if !(*symbols.offset(i as isize) as libc::c_int
            == kInvalidHistogramSymbol as libc::c_int)
        {
            cluster = *cluster_mappings.offset(*symbols.offset(i as isize) as isize)
                as libc::c_int;
            if cluster > 0 as libc::c_int
                && *cluster_mappings_tmp.offset(cluster as isize) as libc::c_int
                    == 0 as libc::c_int
            {
                cluster_max += 1;
                cluster_max;
                *cluster_mappings_tmp.offset(cluster as isize) = cluster_max as uint16_t;
            }
            *symbols.offset(i as isize) = *cluster_mappings_tmp.offset(cluster as isize);
        }
        i += 1;
        i;
    }
    cluster_max = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*set).max_size {
        if !(*symbols.offset(i as isize) as libc::c_int
            == kInvalidHistogramSymbol as libc::c_int)
        {
            if !(*symbols.offset(i as isize) as libc::c_int <= cluster_max) {
                cluster_max += 1;
                cluster_max;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn RemoveEmptyHistograms(image_histo: *mut VP8LHistogramSet) {
    let mut size: uint32_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    size = 0 as libc::c_int as uint32_t;
    while i < (*image_histo).size {
        if !(*((*image_histo).histograms).offset(i as isize)).is_null() {
            let fresh9 = size;
            size = size.wrapping_add(1);
            let ref mut fresh10 = *((*image_histo).histograms).offset(fresh9 as isize);
            *fresh10 = *((*image_histo).histograms).offset(i as isize);
        }
        i += 1;
        i;
    }
    (*image_histo).size = size as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetHistoImageSymbols(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    refs: *const VP8LBackwardRefs,
    mut quality: libc::c_int,
    mut low_effort: libc::c_int,
    mut histogram_bits: libc::c_int,
    mut cache_bits: libc::c_int,
    image_histo: *mut VP8LHistogramSet,
    tmp_histo: *mut VP8LHistogram,
    histogram_symbols: *mut uint16_t,
    pic: *const WebPPicture,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let histo_xsize: libc::c_int = (if histogram_bits != 0 {
        VP8LSubSampleSize(xsize as uint32_t, histogram_bits as uint32_t)
    } else {
        1 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    let histo_ysize: libc::c_int = (if histogram_bits != 0 {
        VP8LSubSampleSize(ysize as uint32_t, histogram_bits as uint32_t)
    } else {
        1 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    let image_histo_raw_size: libc::c_int = histo_xsize * histo_ysize;
    let orig_histo: *mut VP8LHistogramSet = VP8LAllocateHistogramSet(
        image_histo_raw_size,
        cache_bits,
    );
    let entropy_combine_num_bins: libc::c_int = if low_effort != 0 {
        4 as libc::c_int
    } else {
        4 as libc::c_int * 4 as libc::c_int * 4 as libc::c_int
    };
    let mut entropy_combine: libc::c_int = 0;
    let map_tmp: *mut uint16_t = WebPSafeMalloc(
        (2 as libc::c_int * image_histo_raw_size) as uint64_t,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    ) as *mut uint16_t;
    let cluster_mappings: *mut uint16_t = map_tmp.offset(image_histo_raw_size as isize);
    let mut num_used: libc::c_int = image_histo_raw_size;
    if orig_histo.is_null() || map_tmp.is_null() {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        HistogramBuild(xsize, histogram_bits, refs, orig_histo);
        HistogramCopyAndAnalyze(
            orig_histo,
            image_histo,
            &mut num_used,
            histogram_symbols,
        );
        entropy_combine = (num_used > entropy_combine_num_bins * 2 as libc::c_int
            && quality < 100 as libc::c_int) as libc::c_int;
        if entropy_combine != 0 {
            let bin_map: *mut uint16_t = map_tmp;
            let combine_cost_factor: libc::c_float = GetCombineCostFactor(
                image_histo_raw_size,
                quality,
            );
            let num_clusters: uint32_t = num_used as uint32_t;
            HistogramAnalyzeEntropyBin(image_histo, bin_map, low_effort);
            HistogramCombineEntropyBin(
                image_histo,
                &mut num_used,
                histogram_symbols,
                cluster_mappings,
                tmp_histo,
                bin_map,
                entropy_combine_num_bins,
                combine_cost_factor,
                low_effort,
            );
            OptimizeHistogramSymbols(
                image_histo,
                cluster_mappings,
                num_clusters as libc::c_int,
                map_tmp,
                histogram_symbols,
            );
        }
        if low_effort == 0 || entropy_combine == 0 {
            let x: libc::c_float = quality as libc::c_float / 100.0f32;
            let threshold_size: libc::c_int = (1 as libc::c_int as libc::c_float
                + x * x * x * (100 as libc::c_int - 1 as libc::c_int) as libc::c_float)
                as libc::c_int;
            let mut do_greedy: libc::c_int = 0;
            if HistogramCombineStochastic(
                image_histo,
                &mut num_used,
                threshold_size,
                &mut do_greedy,
            ) == 0
            {
                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                current_block = 3475037311606592594;
            } else if do_greedy != 0 {
                RemoveEmptyHistograms(image_histo);
                if HistogramCombineGreedy(image_histo, &mut num_used) == 0 {
                    WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 3475037311606592594;
                } else {
                    current_block = 11584701595673473500;
                }
            } else {
                current_block = 11584701595673473500;
            }
        } else {
            current_block = 11584701595673473500;
        }
        match current_block {
            3475037311606592594 => {}
            _ => {
                RemoveEmptyHistograms(image_histo);
                HistogramRemap(orig_histo, image_histo, histogram_symbols);
                WebPReportProgress(pic, *percent + percent_range, percent) == 0;
            }
        }
    }
    VP8LFreeHistogramSet(orig_histo);
    WebPSafeFree(map_tmp as *mut libc::c_void);
    return ((*pic).error_code as libc::c_uint
        == VP8_ENC_OK as libc::c_int as libc::c_uint) as libc::c_int;
}
