use ::libc;
extern "C" {
    fn WebPRescalerInit(
        rescaler: *mut WebPRescaler,
        src_width: libc::c_int,
        src_height: libc::c_int,
        dst: *mut uint8_t,
        dst_width: libc::c_int,
        dst_height: libc::c_int,
        dst_stride: libc::c_int,
        num_channels: libc::c_int,
        work: *mut rescaler_t,
    ) -> libc::c_int;
    fn WebPRescaleNeededLines(
        rescaler: *const WebPRescaler,
        max_num_lines: libc::c_int,
    ) -> libc::c_int;
    fn WebPRescalerImport(
        rescaler: *mut WebPRescaler,
        num_rows: libc::c_int,
        src: *const uint8_t,
        src_stride: libc::c_int,
    ) -> libc::c_int;
    static mut WebPUnfilters: [WebPUnfilterFunc; 4];
    fn WebPIoInitFromOptions(
        options: *const WebPDecoderOptions,
        io: *mut VP8Io,
        src_colorspace: WEBP_CSP_MODE,
    ) -> libc::c_int;
    fn WebPInitAlphaProcessing();
    fn WebPMultARGBRows(
        ptr: *mut uint8_t,
        stride: libc::c_int,
        width: libc::c_int,
        num_rows: libc::c_int,
        inverse: libc::c_int,
    );
    static mut WebPMultARGBRow: Option::<
        unsafe extern "C" fn(*mut uint32_t, libc::c_int, libc::c_int) -> (),
    >;
    static mut WebPExtractGreen: Option::<
        unsafe extern "C" fn(*const uint32_t, *mut uint8_t, libc::c_int) -> (),
    >;
    static mut WebPExtractAlpha: Option::<
        unsafe extern "C" fn(
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >;
    fn WebPRescalerExportRow(wrk: *mut WebPRescaler);
    fn WebPInitConvertARGBToYUV();
    static mut WebPConvertARGBToUV: Option::<
        unsafe extern "C" fn(
            *const uint32_t,
            *mut uint8_t,
            *mut uint8_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >;
    static mut WebPConvertARGBToY: Option::<
        unsafe extern "C" fn(*const uint32_t, *mut uint8_t, libc::c_int) -> (),
    >;
    fn VP8LInitBitReader(br: *mut VP8LBitReader, start: *const uint8_t, length: size_t);
    fn VP8LReadBits(br: *mut VP8LBitReader, n_bits: libc::c_int) -> uint32_t;
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
    fn VP8LDoFillBitWindow(br: *mut VP8LBitReader);
    fn VP8LColorCacheInit(
        color_cache: *mut VP8LColorCache,
        hash_bits: libc::c_int,
    ) -> libc::c_int;
    fn VP8LColorCacheCopy(src: *const VP8LColorCache, dst: *mut VP8LColorCache);
    fn VP8LColorCacheClear(color_cache: *mut VP8LColorCache);
    fn VP8LHuffmanTablesAllocate(
        size: libc::c_int,
        huffman_tables: *mut HuffmanTables,
    ) -> libc::c_int;
    fn VP8LHuffmanTablesDeallocate(huffman_tables: *mut HuffmanTables);
    fn VP8LHtreeGroupsNew(num_htree_groups: libc::c_int) -> *mut HTreeGroup;
    fn VP8LHtreeGroupsFree(htree_groups: *mut HTreeGroup);
    fn VP8LBuildHuffmanTable(
        root_table: *mut HuffmanTables,
        root_bits: libc::c_int,
        code_lengths: *const libc::c_int,
        code_lengths_size: libc::c_int,
    ) -> libc::c_int;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn VP8LDspInit();
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn VP8LInverseTransform(
        transform: *const VP8LTransform,
        row_start: libc::c_int,
        row_end: libc::c_int,
        in_0: *const uint32_t,
        out: *mut uint32_t,
    );
    fn VP8LColorIndexInverseTransformAlpha(
        transform: *const VP8LTransform,
        y_start: libc::c_int,
        y_end: libc::c_int,
        src: *const uint8_t,
        dst: *mut uint8_t,
    );
    fn VP8LConvertFromBGRA(
        in_data: *const uint32_t,
        num_pixels: libc::c_int,
        out_colorspace: WEBP_CSP_MODE,
        rgba: *mut uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type ptrdiff_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type rescaler_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRescaler {
    pub x_expand: libc::c_int,
    pub y_expand: libc::c_int,
    pub num_channels: libc::c_int,
    pub fx_scale: uint32_t,
    pub fy_scale: uint32_t,
    pub fxy_scale: uint32_t,
    pub y_accum: libc::c_int,
    pub y_add: libc::c_int,
    pub y_sub: libc::c_int,
    pub x_add: libc::c_int,
    pub x_sub: libc::c_int,
    pub src_width: libc::c_int,
    pub src_height: libc::c_int,
    pub dst_width: libc::c_int,
    pub dst_height: libc::c_int,
    pub src_y: libc::c_int,
    pub dst_y: libc::c_int,
    pub dst: *mut uint8_t,
    pub dst_stride: libc::c_int,
    pub irow: *mut rescaler_t,
    pub frow: *mut rescaler_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRGBABuffer {
    pub rgba: *mut uint8_t,
    pub stride: libc::c_int,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPYUVABuffer {
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub a: *mut uint8_t,
    pub y_stride: libc::c_int,
    pub u_stride: libc::c_int,
    pub v_stride: libc::c_int,
    pub a_stride: libc::c_int,
    pub y_size: size_t,
    pub u_size: size_t,
    pub v_size: size_t,
    pub a_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub is_external_memory: libc::c_int,
    pub u: C2RustUnnamed,
    pub pad: [uint32_t; 4],
    pub private_memory: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}
pub type WEBP_CSP_MODE = libc::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const MODE_Argb: WEBP_CSP_MODE = 9;
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub const MODE_RGB: WEBP_CSP_MODE = 0;
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
pub struct VP8LBitReader {
    pub val_: vp8l_val_t,
    pub buf_: *const uint8_t,
    pub len_: size_t,
    pub pos_: size_t,
    pub bit_pos_: libc::c_int,
    pub eos_: libc::c_int,
}
pub type vp8l_val_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecParams {
    pub output: *mut WebPDecBuffer,
    pub tmp_y: *mut uint8_t,
    pub tmp_u: *mut uint8_t,
    pub tmp_v: *mut uint8_t,
    pub last_y: libc::c_int,
    pub options: *const WebPDecoderOptions,
    pub scaler_y: *mut WebPRescaler,
    pub scaler_u: *mut WebPRescaler,
    pub scaler_v: *mut WebPRescaler,
    pub scaler_a: *mut WebPRescaler,
    pub memory: *mut libc::c_void,
    pub emit: OutputFunc,
    pub emit_alpha: OutputAlphaFunc,
    pub emit_alpha_row: OutputRowFunc,
}
pub type OutputRowFunc = Option::<
    unsafe extern "C" fn(*mut WebPDecParams, libc::c_int, libc::c_int) -> libc::c_int,
>;
pub type OutputAlphaFunc = Option::<
    unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams, libc::c_int) -> libc::c_int,
>;
pub type OutputFunc = Option::<
    unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> libc::c_int,
>;
pub type WEBP_FILTER_TYPE = libc::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
pub type WebPUnfilterFunc = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut uint8_t, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LDecoder {
    pub status_: VP8StatusCode,
    pub state_: VP8LDecodeState,
    pub io_: *mut VP8Io,
    pub output_: *const WebPDecBuffer,
    pub pixels_: *mut uint32_t,
    pub argb_cache_: *mut uint32_t,
    pub br_: VP8LBitReader,
    pub incremental_: libc::c_int,
    pub saved_br_: VP8LBitReader,
    pub saved_last_pixel_: libc::c_int,
    pub width_: libc::c_int,
    pub height_: libc::c_int,
    pub last_row_: libc::c_int,
    pub last_pixel_: libc::c_int,
    pub last_out_row_: libc::c_int,
    pub hdr_: VP8LMetadata,
    pub next_transform_: libc::c_int,
    pub transforms_: [VP8LTransform; 4],
    pub transforms_seen_: uint32_t,
    pub rescaler_memory: *mut uint8_t,
    pub rescaler: *mut WebPRescaler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LTransform {
    pub type_: VP8LImageTransformType,
    pub bits_: libc::c_int,
    pub xsize_: libc::c_int,
    pub ysize_: libc::c_int,
    pub data_: *mut uint32_t,
}
pub type VP8LImageTransformType = libc::c_uint;
pub const COLOR_INDEXING_TRANSFORM: VP8LImageTransformType = 3;
pub const SUBTRACT_GREEN_TRANSFORM: VP8LImageTransformType = 2;
pub const CROSS_COLOR_TRANSFORM: VP8LImageTransformType = 1;
pub const PREDICTOR_TRANSFORM: VP8LImageTransformType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LMetadata {
    pub color_cache_size_: libc::c_int,
    pub color_cache_: VP8LColorCache,
    pub saved_color_cache_: VP8LColorCache,
    pub huffman_mask_: libc::c_int,
    pub huffman_subsample_bits_: libc::c_int,
    pub huffman_xsize_: libc::c_int,
    pub huffman_image_: *mut uint32_t,
    pub num_htree_groups_: libc::c_int,
    pub htree_groups_: *mut HTreeGroup,
    pub huffman_tables_: HuffmanTables,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTables {
    pub root: HuffmanTablesSegment,
    pub curr_segment: *mut HuffmanTablesSegment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTablesSegment {
    pub start: *mut HuffmanCode,
    pub curr_table: *mut HuffmanCode,
    pub next: *mut HuffmanTablesSegment,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTreeGroup {
    pub htrees: [*mut HuffmanCode; 5],
    pub is_trivial_literal: libc::c_int,
    pub literal_arb: uint32_t,
    pub is_trivial_code: libc::c_int,
    pub use_packed_table: libc::c_int,
    pub packed_table: [HuffmanCode32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode32 {
    pub bits: libc::c_int,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LColorCache {
    pub colors_: *mut uint32_t,
    pub hash_shift_: libc::c_int,
    pub hash_bits_: libc::c_int,
}
pub type VP8LDecodeState = libc::c_uint;
pub const READ_DIM: VP8LDecodeState = 2;
pub const READ_HDR: VP8LDecodeState = 1;
pub const READ_DATA: VP8LDecodeState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ALPHDecoder {
    pub width_: libc::c_int,
    pub height_: libc::c_int,
    pub method_: libc::c_int,
    pub filter_: WEBP_FILTER_TYPE,
    pub pre_processing_: libc::c_int,
    pub vp8l_dec_: *mut VP8LDecoder,
    pub io_: VP8Io,
    pub use_8b_decode_: libc::c_int,
    pub output_: *mut uint8_t,
    pub prev_line_: *const uint8_t,
}
pub const ALPHA: C2RustUnnamed_0 = 3;
pub const BLUE: C2RustUnnamed_0 = 2;
pub const RED: C2RustUnnamed_0 = 1;
pub type ProcessRowsFunc = Option::<
    unsafe extern "C" fn(*mut VP8LDecoder, libc::c_int) -> (),
>;
pub const DIST: C2RustUnnamed_0 = 4;
pub const GREEN: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn WebPRescalerOutputDone(
    rescaler: *const WebPRescaler,
) -> libc::c_int {
    return ((*rescaler).dst_y >= (*rescaler).dst_height) as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPRescalerHasPendingOutput(
    rescaler: *const WebPRescaler,
) -> libc::c_int {
    return (WebPRescalerOutputDone(rescaler) == 0
        && (*rescaler).y_accum <= 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsPremultipliedMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return (mode as libc::c_uint == MODE_rgbA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_bgrA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_Argb as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_rgbA_4444 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsRGBMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return ((mode as libc::c_uint) < MODE_YUV as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8LPrefetchBits(br: *mut VP8LBitReader) -> uint32_t {
    return ((*br).val_ >> ((*br).bit_pos_ & 64 as libc::c_int - 1 as libc::c_int))
        as uint32_t;
}
#[inline]
unsafe extern "C" fn VP8LIsEndOfStream(br: *const VP8LBitReader) -> libc::c_int {
    return ((*br).eos_ != 0
        || (*br).pos_ == (*br).len_ && (*br).bit_pos_ > 64 as libc::c_int)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn VP8LSetBitPos(br: *mut VP8LBitReader, mut val: libc::c_int) {
    (*br).bit_pos_ = val;
}
#[inline]
unsafe extern "C" fn VP8LFillBitWindow(br: *mut VP8LBitReader) {
    if (*br).bit_pos_ >= 32 as libc::c_int {
        VP8LDoFillBitWindow(br);
    }
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
unsafe extern "C" fn VP8LColorCacheInsert(
    cc: *const VP8LColorCache,
    mut argb: uint32_t,
) {
    let key: libc::c_int = VP8LHashPix(argb, (*cc).hash_shift_);
    *((*cc).colors_).offset(key as isize) = argb;
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
static mut kCodeLengthLiterals: libc::c_int = 16 as libc::c_int;
static mut kCodeLengthRepeatCode: libc::c_int = 16 as libc::c_int;
static mut kCodeLengthExtraBits: [uint8_t; 3] = [
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
];
static mut kCodeLengthRepeatOffsets: [uint8_t; 3] = [
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
];
static mut kAlphabetSize: [uint16_t; 5] = [
    (256 as libc::c_int + 24 as libc::c_int) as uint16_t,
    256 as libc::c_int as uint16_t,
    256 as libc::c_int as uint16_t,
    256 as libc::c_int as uint16_t,
    40 as libc::c_int as uint16_t,
];
static mut kLiteralMap: [uint8_t; 5] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
static mut kCodeLengthCodeOrder: [uint8_t; 19] = [
    17 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
static mut kCodeToPlane: [uint8_t; 120] = [
    0x18 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x27 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x37 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x15 as libc::c_int as uint8_t,
    0x1b as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x3a as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0x1c as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0x3b as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x2c as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x4b as libc::c_int as uint8_t,
    0x34 as libc::c_int as uint8_t,
    0x3c as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x57 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0x1d as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0x5a as libc::c_int as uint8_t,
    0x23 as libc::c_int as uint8_t,
    0x2d as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x4c as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x5b as libc::c_int as uint8_t,
    0x33 as libc::c_int as uint8_t,
    0x3d as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0x1e as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0x6a as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x5c as libc::c_int as uint8_t,
    0x43 as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0x6b as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x53 as libc::c_int as uint8_t,
    0x5d as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0x1f as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0x4e as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0x2f as libc::c_int as uint8_t,
    0x75 as libc::c_int as uint8_t,
    0x7b as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x3f as libc::c_int as uint8_t,
    0x63 as libc::c_int as uint8_t,
    0x6d as libc::c_int as uint8_t,
    0x52 as libc::c_int as uint8_t,
    0x5e as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x74 as libc::c_int as uint8_t,
    0x7c as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x4f as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0x62 as libc::c_int as uint8_t,
    0x6e as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0x73 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x5f as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0x7e as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0x6f as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x7f as libc::c_int as uint8_t,
    0x60 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
];
static mut kTableSize: [uint16_t; 12] = [
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 654 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 656 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 658 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 662 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 670 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 686 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 718 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 782 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 912 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 1168 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 1680 as libc::c_int)
        as uint16_t,
    (630 as libc::c_int * 3 as libc::c_int + 410 as libc::c_int + 2704 as libc::c_int)
        as uint16_t,
];
unsafe extern "C" fn VP8LSetError(
    dec: *mut VP8LDecoder,
    mut error: VP8StatusCode,
) -> libc::c_int {
    if (*dec).status_ as libc::c_uint == VP8_STATUS_OK as libc::c_int as libc::c_uint
        || (*dec).status_ as libc::c_uint
            == VP8_STATUS_SUSPENDED as libc::c_int as libc::c_uint
    {
        (*dec).status_ = error;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LCheckSignature(
    data: *const uint8_t,
    mut size: size_t,
) -> libc::c_int {
    return (size >= 5 as libc::c_int as libc::c_ulong
        && *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x2f as libc::c_int
        && *data.offset(4 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int
            == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ReadImageInfo(
    br: *mut VP8LBitReader,
    width: *mut libc::c_int,
    height: *mut libc::c_int,
    has_alpha: *mut libc::c_int,
) -> libc::c_int {
    if VP8LReadBits(br, 8 as libc::c_int) != 0x2f as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    *width = (VP8LReadBits(br, 14 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    *height = (VP8LReadBits(br, 14 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    *has_alpha = VP8LReadBits(br, 1 as libc::c_int) as libc::c_int;
    if VP8LReadBits(br, 3 as libc::c_int) != 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    return ((*br).eos_ == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LGetInfo(
    mut data: *const uint8_t,
    mut data_size: size_t,
    width: *mut libc::c_int,
    height: *mut libc::c_int,
    has_alpha: *mut libc::c_int,
) -> libc::c_int {
    if data.is_null() || data_size < 5 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    } else if VP8LCheckSignature(data, data_size) == 0 {
        return 0 as libc::c_int
    } else {
        let mut w: libc::c_int = 0;
        let mut h: libc::c_int = 0;
        let mut a: libc::c_int = 0;
        let mut br: VP8LBitReader = VP8LBitReader {
            val_: 0,
            buf_: 0 as *const uint8_t,
            len_: 0,
            pos_: 0,
            bit_pos_: 0,
            eos_: 0,
        };
        VP8LInitBitReader(&mut br, data, data_size);
        if ReadImageInfo(&mut br, &mut w, &mut h, &mut a) == 0 {
            return 0 as libc::c_int;
        }
        if !width.is_null() {
            *width = w;
        }
        if !height.is_null() {
            *height = h;
        }
        if !has_alpha.is_null() {
            *has_alpha = a;
        }
        return 1 as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn GetCopyDistance(
    mut distance_symbol: libc::c_int,
    br: *mut VP8LBitReader,
) -> libc::c_int {
    let mut extra_bits: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    if distance_symbol < 4 as libc::c_int {
        return distance_symbol + 1 as libc::c_int;
    }
    extra_bits = distance_symbol - 2 as libc::c_int >> 1 as libc::c_int;
    offset = 2 as libc::c_int + (distance_symbol & 1 as libc::c_int) << extra_bits;
    return (offset as libc::c_uint)
        .wrapping_add(VP8LReadBits(br, extra_bits))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn GetCopyLength(
    mut length_symbol: libc::c_int,
    br: *mut VP8LBitReader,
) -> libc::c_int {
    return GetCopyDistance(length_symbol, br);
}
#[inline]
unsafe extern "C" fn PlaneCodeToDistance(
    mut xsize: libc::c_int,
    mut plane_code: libc::c_int,
) -> libc::c_int {
    if plane_code > 120 as libc::c_int {
        return plane_code - 120 as libc::c_int
    } else {
        let dist_code: libc::c_int = kCodeToPlane[(plane_code - 1 as libc::c_int)
            as usize] as libc::c_int;
        let yoffset: libc::c_int = dist_code >> 4 as libc::c_int;
        let xoffset: libc::c_int = 8 as libc::c_int - (dist_code & 0xf as libc::c_int);
        let dist: libc::c_int = yoffset * xsize + xoffset;
        return if dist >= 1 as libc::c_int { dist } else { 1 as libc::c_int };
    };
}
#[inline]
unsafe extern "C" fn ReadSymbol(
    mut table: *const HuffmanCode,
    br: *mut VP8LBitReader,
) -> libc::c_int {
    let mut nbits: libc::c_int = 0;
    let mut val: uint32_t = VP8LPrefetchBits(br);
    table = table
        .offset(
            (val
                & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint) as isize,
        );
    nbits = (*table).bits as libc::c_int - 8 as libc::c_int;
    if nbits > 0 as libc::c_int {
        VP8LSetBitPos(br, (*br).bit_pos_ + 8 as libc::c_int);
        val = VP8LPrefetchBits(br);
        table = table.offset((*table).value as libc::c_int as isize);
        table = table
            .offset(
                (val
                    & (((1 as libc::c_int) << nbits) - 1 as libc::c_int) as libc::c_uint)
                    as isize,
            );
    }
    VP8LSetBitPos(br, (*br).bit_pos_ + (*table).bits as libc::c_int);
    return (*table).value as libc::c_int;
}
#[inline]
unsafe extern "C" fn ReadPackedSymbols(
    mut group: *const HTreeGroup,
    br: *mut VP8LBitReader,
    dst: *mut uint32_t,
) -> libc::c_int {
    let val: uint32_t = VP8LPrefetchBits(br)
        & ((1 as libc::c_uint) << 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let code: HuffmanCode32 = (*group).packed_table[val as usize];
    if code.bits < 0x100 as libc::c_int {
        VP8LSetBitPos(br, (*br).bit_pos_ + code.bits);
        *dst = code.value;
        return 0 as libc::c_int;
    } else {
        VP8LSetBitPos(br, (*br).bit_pos_ + code.bits - 0x100 as libc::c_int);
        return code.value as libc::c_int;
    };
}
unsafe extern "C" fn AccumulateHCode(
    mut hcode: HuffmanCode,
    mut shift: libc::c_int,
    huff: *mut HuffmanCode32,
) -> libc::c_int {
    (*huff).bits += hcode.bits as libc::c_int;
    (*huff).value |= (hcode.value as uint32_t) << shift;
    return hcode.bits as libc::c_int;
}
unsafe extern "C" fn BuildPackedTable(htree_group: *mut HTreeGroup) {
    let mut code: uint32_t = 0;
    code = 0 as libc::c_int as uint32_t;
    while code < (1 as libc::c_uint) << 6 as libc::c_int {
        let mut bits: uint32_t = code;
        let huff: *mut HuffmanCode32 = &mut *((*htree_group).packed_table)
            .as_mut_ptr()
            .offset(bits as isize) as *mut HuffmanCode32;
        let mut hcode: HuffmanCode = *((*htree_group)
            .htrees[GREEN as libc::c_int as usize])
            .offset(bits as isize);
        if hcode.value as libc::c_int >= 256 as libc::c_int {
            (*huff).bits = hcode.bits as libc::c_int + 0x100 as libc::c_int;
            (*huff).value = hcode.value as uint32_t;
        } else {
            (*huff).bits = 0 as libc::c_int;
            (*huff).value = 0 as libc::c_int as uint32_t;
            bits >>= AccumulateHCode(hcode, 8 as libc::c_int, huff);
            bits
                >>= AccumulateHCode(
                    *((*htree_group).htrees[RED as libc::c_int as usize])
                        .offset(bits as isize),
                    16 as libc::c_int,
                    huff,
                );
            bits
                >>= AccumulateHCode(
                    *((*htree_group).htrees[BLUE as libc::c_int as usize])
                        .offset(bits as isize),
                    0 as libc::c_int,
                    huff,
                );
            bits
                >>= AccumulateHCode(
                    *((*htree_group).htrees[ALPHA as libc::c_int as usize])
                        .offset(bits as isize),
                    24 as libc::c_int,
                    huff,
                );
        }
        code = code.wrapping_add(1);
        code;
    }
}
unsafe extern "C" fn ReadHuffmanCodeLengths(
    dec: *mut VP8LDecoder,
    code_length_code_lengths: *const libc::c_int,
    mut num_symbols: libc::c_int,
    code_lengths: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let br: *mut VP8LBitReader = &mut (*dec).br_;
    let mut symbol: libc::c_int = 0;
    let mut max_symbol: libc::c_int = 0;
    let mut prev_code_len: libc::c_int = 8 as libc::c_int;
    let mut tables: HuffmanTables = HuffmanTables {
        root: HuffmanTablesSegment {
            start: 0 as *mut HuffmanCode,
            curr_table: 0 as *mut HuffmanCode,
            next: 0 as *mut HuffmanTablesSegment,
            size: 0,
        },
        curr_segment: 0 as *mut HuffmanTablesSegment,
    };
    if !(VP8LHuffmanTablesAllocate((1 as libc::c_int) << 7 as libc::c_int, &mut tables)
        == 0
        || VP8LBuildHuffmanTable(
            &mut tables,
            7 as libc::c_int,
            code_length_code_lengths,
            19 as libc::c_int,
        ) == 0)
    {
        if VP8LReadBits(br, 1 as libc::c_int) != 0 {
            let length_nbits: libc::c_int = (2 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(VP8LReadBits(br, 3 as libc::c_int)),
                ) as libc::c_int;
            max_symbol = (2 as libc::c_int as libc::c_uint)
                .wrapping_add(VP8LReadBits(br, length_nbits)) as libc::c_int;
            if max_symbol > num_symbols {
                current_block = 15127221173810008931;
            } else {
                current_block = 5720623009719927633;
            }
        } else {
            max_symbol = num_symbols;
            current_block = 5720623009719927633;
        }
        match current_block {
            15127221173810008931 => {}
            _ => {
                symbol = 0 as libc::c_int;
                loop {
                    if !(symbol < num_symbols) {
                        current_block = 7056779235015430508;
                        break;
                    }
                    let mut p: *const HuffmanCode = 0 as *const HuffmanCode;
                    let mut code_len: libc::c_int = 0;
                    let fresh0 = max_symbol;
                    max_symbol = max_symbol - 1;
                    if fresh0 == 0 as libc::c_int {
                        current_block = 7056779235015430508;
                        break;
                    }
                    VP8LFillBitWindow(br);
                    p = &mut *((*tables.curr_segment).start)
                        .offset(
                            ((VP8LPrefetchBits
                                as unsafe extern "C" fn(*mut VP8LBitReader) -> uint32_t)(br)
                                & (((1 as libc::c_int) << 7 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as isize,
                        ) as *mut HuffmanCode;
                    VP8LSetBitPos(br, (*br).bit_pos_ + (*p).bits as libc::c_int);
                    code_len = (*p).value as libc::c_int;
                    if code_len < kCodeLengthLiterals {
                        let fresh1 = symbol;
                        symbol = symbol + 1;
                        *code_lengths.offset(fresh1 as isize) = code_len;
                        if code_len != 0 as libc::c_int {
                            prev_code_len = code_len;
                        }
                    } else {
                        let use_prev: libc::c_int = (code_len == kCodeLengthRepeatCode)
                            as libc::c_int;
                        let slot: libc::c_int = code_len - kCodeLengthLiterals;
                        let extra_bits: libc::c_int = kCodeLengthExtraBits[slot as usize]
                            as libc::c_int;
                        let repeat_offset: libc::c_int = kCodeLengthRepeatOffsets[slot
                            as usize] as libc::c_int;
                        let mut repeat: libc::c_int = (VP8LReadBits(br, extra_bits))
                            .wrapping_add(repeat_offset as libc::c_uint) as libc::c_int;
                        if symbol + repeat > num_symbols {
                            current_block = 15127221173810008931;
                            break;
                        }
                        let length: libc::c_int = if use_prev != 0 {
                            prev_code_len
                        } else {
                            0 as libc::c_int
                        };
                        loop {
                            let fresh2 = repeat;
                            repeat = repeat - 1;
                            if !(fresh2 > 0 as libc::c_int) {
                                break;
                            }
                            let fresh3 = symbol;
                            symbol = symbol + 1;
                            *code_lengths.offset(fresh3 as isize) = length;
                        }
                    }
                }
                match current_block {
                    15127221173810008931 => {}
                    _ => {
                        ok = 1 as libc::c_int;
                    }
                }
            }
        }
    }
    VP8LHuffmanTablesDeallocate(&mut tables);
    if ok == 0 {
        return VP8LSetError(dec, VP8_STATUS_BITSTREAM_ERROR);
    }
    return ok;
}
unsafe extern "C" fn ReadHuffmanCode(
    mut alphabet_size: libc::c_int,
    dec: *mut VP8LDecoder,
    code_lengths: *mut libc::c_int,
    table: *mut HuffmanTables,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 0 as libc::c_int;
    let br: *mut VP8LBitReader = &mut (*dec).br_;
    let simple_code: libc::c_int = VP8LReadBits(br, 1 as libc::c_int) as libc::c_int;
    memset(
        code_lengths as *mut libc::c_void,
        0 as libc::c_int,
        (alphabet_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if simple_code != 0 {
        let num_symbols: libc::c_int = (VP8LReadBits(br, 1 as libc::c_int))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
        let first_symbol_len_code: libc::c_int = VP8LReadBits(br, 1 as libc::c_int)
            as libc::c_int;
        let mut symbol: libc::c_int = VP8LReadBits(
            br,
            if first_symbol_len_code == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                8 as libc::c_int
            },
        ) as libc::c_int;
        *code_lengths.offset(symbol as isize) = 1 as libc::c_int;
        if num_symbols == 2 as libc::c_int {
            symbol = VP8LReadBits(br, 8 as libc::c_int) as libc::c_int;
            *code_lengths.offset(symbol as isize) = 1 as libc::c_int;
        }
        ok = 1 as libc::c_int;
    } else {
        let mut i: libc::c_int = 0;
        let mut code_length_code_lengths: [libc::c_int; 19] = [
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
        ];
        let num_codes: libc::c_int = (VP8LReadBits(br, 4 as libc::c_int))
            .wrapping_add(4 as libc::c_int as libc::c_uint) as libc::c_int;
        i = 0 as libc::c_int;
        while i < num_codes {
            code_length_code_lengths[kCodeLengthCodeOrder[i as usize]
                as usize] = VP8LReadBits(br, 3 as libc::c_int) as libc::c_int;
            i += 1;
            i;
        }
        ok = ReadHuffmanCodeLengths(
            dec,
            code_length_code_lengths.as_mut_ptr(),
            alphabet_size,
            code_lengths,
        );
    }
    ok = (ok != 0 && (*br).eos_ == 0) as libc::c_int;
    if ok != 0 {
        size = VP8LBuildHuffmanTable(
            table,
            8 as libc::c_int,
            code_lengths as *const libc::c_int,
            alphabet_size,
        );
    }
    if ok == 0 || size == 0 as libc::c_int {
        return VP8LSetError(dec, VP8_STATUS_BITSTREAM_ERROR);
    }
    return size;
}
unsafe extern "C" fn ReadHuffmanCodes(
    dec: *mut VP8LDecoder,
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    mut color_cache_bits: libc::c_int,
    mut allow_recursion: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let br: *mut VP8LBitReader = &mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &mut (*dec).hdr_;
    let mut huffman_image: *mut uint32_t = 0 as *mut uint32_t;
    let mut htree_groups: *mut HTreeGroup = 0 as *mut HTreeGroup;
    let mut huffman_tables: *mut HuffmanTables = &mut (*hdr).huffman_tables_;
    let mut num_htree_groups: libc::c_int = 1 as libc::c_int;
    let mut num_htree_groups_max: libc::c_int = 1 as libc::c_int;
    let mut mapping: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ok: libc::c_int = 0 as libc::c_int;
    if allow_recursion != 0 && VP8LReadBits(br, 1 as libc::c_int) != 0 {
        let huffman_precision: libc::c_int = (VP8LReadBits(br, 3 as libc::c_int))
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_int;
        let huffman_xsize: libc::c_int = VP8LSubSampleSize(
            xsize as uint32_t,
            huffman_precision as uint32_t,
        ) as libc::c_int;
        let huffman_ysize: libc::c_int = VP8LSubSampleSize(
            ysize as uint32_t,
            huffman_precision as uint32_t,
        ) as libc::c_int;
        let huffman_pixs: libc::c_int = huffman_xsize * huffman_ysize;
        if DecodeImageStream(
            huffman_xsize,
            huffman_ysize,
            0 as libc::c_int,
            dec,
            &mut huffman_image,
        ) == 0
        {
            current_block = 4943015495968876630;
        } else {
            (*hdr).huffman_subsample_bits_ = huffman_precision;
            i = 0 as libc::c_int;
            while i < huffman_pixs {
                let group: libc::c_int = (*huffman_image.offset(i as isize)
                    >> 8 as libc::c_int & 0xffff as libc::c_int as libc::c_uint)
                    as libc::c_int;
                *huffman_image.offset(i as isize) = group as uint32_t;
                if group >= num_htree_groups_max {
                    num_htree_groups_max = group + 1 as libc::c_int;
                }
                i += 1;
                i;
            }
            if num_htree_groups_max > 1000 as libc::c_int
                || num_htree_groups_max > xsize * ysize
            {
                mapping = WebPSafeMalloc(
                    num_htree_groups_max as uint64_t,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                ) as *mut libc::c_int;
                if mapping.is_null() {
                    VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
                    current_block = 4943015495968876630;
                } else {
                    memset(
                        mapping as *mut libc::c_void,
                        0xff as libc::c_int,
                        (num_htree_groups_max as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    );
                    num_htree_groups = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < huffman_pixs {
                        let mapped_group: *mut libc::c_int = &mut *mapping
                            .offset(*huffman_image.offset(i as isize) as isize)
                            as *mut libc::c_int;
                        if *mapped_group == -(1 as libc::c_int) {
                            let fresh4 = num_htree_groups;
                            num_htree_groups = num_htree_groups + 1;
                            *mapped_group = fresh4;
                        }
                        *huffman_image.offset(i as isize) = *mapped_group as uint32_t;
                        i += 1;
                        i;
                    }
                    current_block = 14648156034262866959;
                }
            } else {
                num_htree_groups = num_htree_groups_max;
                current_block = 14648156034262866959;
            }
        }
    } else {
        current_block = 14648156034262866959;
    }
    match current_block {
        14648156034262866959 => {
            if !((*br).eos_ != 0) {
                if !(ReadHuffmanCodesHelper(
                    color_cache_bits,
                    num_htree_groups,
                    num_htree_groups_max,
                    mapping,
                    dec,
                    huffman_tables,
                    &mut htree_groups,
                ) == 0)
                {
                    ok = 1 as libc::c_int;
                    (*hdr).huffman_image_ = huffman_image;
                    (*hdr).num_htree_groups_ = num_htree_groups;
                    (*hdr).htree_groups_ = htree_groups;
                }
            }
        }
        _ => {}
    }
    WebPSafeFree(mapping as *mut libc::c_void);
    if ok == 0 {
        WebPSafeFree(huffman_image as *mut libc::c_void);
        VP8LHuffmanTablesDeallocate(huffman_tables);
        VP8LHtreeGroupsFree(htree_groups);
    }
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn ReadHuffmanCodesHelper(
    mut color_cache_bits: libc::c_int,
    mut num_htree_groups: libc::c_int,
    mut num_htree_groups_max: libc::c_int,
    mapping: *const libc::c_int,
    dec: *mut VP8LDecoder,
    huffman_tables: *mut HuffmanTables,
    htree_groups: *mut *mut HTreeGroup,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let max_alphabet_size: libc::c_int = kAlphabetSize[0 as libc::c_int as usize]
        as libc::c_int
        + (if color_cache_bits > 0 as libc::c_int {
            (1 as libc::c_int) << color_cache_bits
        } else {
            0 as libc::c_int
        });
    let table_size: libc::c_int = kTableSize[color_cache_bits as usize] as libc::c_int;
    let mut code_lengths: *mut libc::c_int = 0 as *mut libc::c_int;
    if !(mapping.is_null() && num_htree_groups != num_htree_groups_max
        || num_htree_groups > num_htree_groups_max)
    {
        code_lengths = WebPSafeCalloc(
            max_alphabet_size as uint64_t,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        *htree_groups = VP8LHtreeGroupsNew(num_htree_groups);
        if (*htree_groups).is_null() || code_lengths.is_null()
            || VP8LHuffmanTablesAllocate(num_htree_groups * table_size, huffman_tables)
                == 0
        {
            VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
        } else {
            i = 0 as libc::c_int;
            's_33: loop {
                if !(i < num_htree_groups_max) {
                    current_block = 1423531122933789233;
                    break;
                }
                if !mapping.is_null()
                    && *mapping.offset(i as isize) == -(1 as libc::c_int)
                {
                    j = 0 as libc::c_int;
                    while j < 5 as libc::c_int {
                        let mut alphabet_size: libc::c_int = kAlphabetSize[j as usize]
                            as libc::c_int;
                        if j == 0 as libc::c_int && color_cache_bits > 0 as libc::c_int {
                            alphabet_size += (1 as libc::c_int) << color_cache_bits;
                        }
                        if ReadHuffmanCode(
                            alphabet_size,
                            dec,
                            code_lengths,
                            0 as *mut HuffmanTables,
                        ) == 0
                        {
                            current_block = 7432026986410637432;
                            break 's_33;
                        }
                        j += 1;
                        j;
                    }
                } else {
                    let htree_group: *mut HTreeGroup = &mut *(*htree_groups)
                        .offset(
                            (if mapping.is_null() {
                                i
                            } else {
                                *mapping.offset(i as isize)
                            }) as isize,
                        ) as *mut HTreeGroup;
                    let htrees: *mut *mut HuffmanCode = ((*htree_group).htrees)
                        .as_mut_ptr();
                    let mut size: libc::c_int = 0;
                    let mut total_size: libc::c_int = 0 as libc::c_int;
                    let mut is_trivial_literal: libc::c_int = 1 as libc::c_int;
                    let mut max_bits: libc::c_int = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while j < 5 as libc::c_int {
                        let mut alphabet_size_0: libc::c_int = kAlphabetSize[j as usize]
                            as libc::c_int;
                        if j == 0 as libc::c_int && color_cache_bits > 0 as libc::c_int {
                            alphabet_size_0 += (1 as libc::c_int) << color_cache_bits;
                        }
                        size = ReadHuffmanCode(
                            alphabet_size_0,
                            dec,
                            code_lengths,
                            huffman_tables,
                        );
                        let ref mut fresh5 = *htrees.offset(j as isize);
                        *fresh5 = (*(*huffman_tables).curr_segment).curr_table;
                        if size == 0 as libc::c_int {
                            current_block = 7432026986410637432;
                            break 's_33;
                        }
                        if is_trivial_literal != 0
                            && kLiteralMap[j as usize] as libc::c_int == 1 as libc::c_int
                        {
                            is_trivial_literal = ((**htrees.offset(j as isize)).bits
                                as libc::c_int == 0 as libc::c_int) as libc::c_int;
                        }
                        total_size += (**htrees.offset(j as isize)).bits as libc::c_int;
                        (*(*huffman_tables).curr_segment)
                            .curr_table = ((*(*huffman_tables).curr_segment).curr_table)
                            .offset(size as isize);
                        if j <= ALPHA as libc::c_int {
                            let mut local_max_bits: libc::c_int = *code_lengths
                                .offset(0 as libc::c_int as isize);
                            let mut k: libc::c_int = 0;
                            k = 1 as libc::c_int;
                            while k < alphabet_size_0 {
                                if *code_lengths.offset(k as isize) > local_max_bits {
                                    local_max_bits = *code_lengths.offset(k as isize);
                                }
                                k += 1;
                                k;
                            }
                            max_bits += local_max_bits;
                        }
                        j += 1;
                        j;
                    }
                    (*htree_group).is_trivial_literal = is_trivial_literal;
                    (*htree_group).is_trivial_code = 0 as libc::c_int;
                    if is_trivial_literal != 0 {
                        let red: libc::c_int = (*(*htrees
                            .offset(RED as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize))
                            .value as libc::c_int;
                        let blue: libc::c_int = (*(*htrees
                            .offset(BLUE as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize))
                            .value as libc::c_int;
                        let alpha: libc::c_int = (*(*htrees
                            .offset(ALPHA as libc::c_int as isize))
                            .offset(0 as libc::c_int as isize))
                            .value as libc::c_int;
                        (*htree_group)
                            .literal_arb = (alpha as uint32_t) << 24 as libc::c_int
                            | (red << 16 as libc::c_int) as libc::c_uint
                            | blue as libc::c_uint;
                        if total_size == 0 as libc::c_int
                            && ((*(*htrees.offset(GREEN as libc::c_int as isize))
                                .offset(0 as libc::c_int as isize))
                                .value as libc::c_int) < 256 as libc::c_int
                        {
                            (*htree_group).is_trivial_code = 1 as libc::c_int;
                            (*htree_group).literal_arb
                                |= (((*(*htrees.offset(GREEN as libc::c_int as isize))
                                    .offset(0 as libc::c_int as isize))
                                    .value as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
                        }
                    }
                    (*htree_group)
                        .use_packed_table = ((*htree_group).is_trivial_code == 0
                        && max_bits < 6 as libc::c_int) as libc::c_int;
                    if (*htree_group).use_packed_table != 0 {
                        BuildPackedTable(htree_group);
                    }
                }
                i += 1;
                i;
            }
            match current_block {
                7432026986410637432 => {}
                _ => {
                    ok = 1 as libc::c_int;
                }
            }
        }
    }
    WebPSafeFree(code_lengths as *mut libc::c_void);
    if ok == 0 {
        VP8LHuffmanTablesDeallocate(huffman_tables);
        VP8LHtreeGroupsFree(*htree_groups);
        *htree_groups = 0 as *mut HTreeGroup;
    }
    return ok;
}
unsafe extern "C" fn AllocateAndInitRescaler(
    dec: *mut VP8LDecoder,
    io: *mut VP8Io,
) -> libc::c_int {
    let num_channels: libc::c_int = 4 as libc::c_int;
    let in_width: libc::c_int = (*io).mb_w;
    let out_width: libc::c_int = (*io).scaled_width;
    let in_height: libc::c_int = (*io).mb_h;
    let out_height: libc::c_int = (*io).scaled_height;
    let work_size: uint64_t = ((2 as libc::c_int * num_channels) as libc::c_ulong)
        .wrapping_mul(out_width as uint64_t);
    let mut work: *mut rescaler_t = 0 as *mut rescaler_t;
    let scaled_data_size: uint64_t = out_width as uint64_t;
    let mut scaled_data: *mut uint32_t = 0 as *mut uint32_t;
    let memory_size: uint64_t = (::core::mem::size_of::<WebPRescaler>() as libc::c_ulong)
        .wrapping_add(
            work_size.wrapping_mul(::core::mem::size_of::<rescaler_t>() as libc::c_ulong),
        )
        .wrapping_add(
            scaled_data_size
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
    let mut memory: *mut uint8_t = WebPSafeMalloc(
        memory_size,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    if memory.is_null() {
        return VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
    }
    (*dec).rescaler_memory = memory;
    (*dec).rescaler = memory as *mut WebPRescaler;
    memory = memory
        .offset(::core::mem::size_of::<WebPRescaler>() as libc::c_ulong as isize);
    work = memory as *mut rescaler_t;
    memory = memory
        .offset(
            work_size.wrapping_mul(::core::mem::size_of::<rescaler_t>() as libc::c_ulong)
                as isize,
        );
    scaled_data = memory as *mut uint32_t;
    if WebPRescalerInit(
        (*dec).rescaler,
        in_width,
        in_height,
        scaled_data as *mut uint8_t,
        out_width,
        out_height,
        0 as libc::c_int,
        num_channels,
        work,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Export(
    rescaler: *mut WebPRescaler,
    mut colorspace: WEBP_CSP_MODE,
    mut rgba_stride: libc::c_int,
    rgba: *mut uint8_t,
) -> libc::c_int {
    let src: *mut uint32_t = (*rescaler).dst as *mut uint32_t;
    let mut dst: *mut uint8_t = rgba;
    let dst_width: libc::c_int = (*rescaler).dst_width;
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    while WebPRescalerHasPendingOutput(rescaler) != 0 {
        WebPRescalerExportRow(rescaler);
        WebPMultARGBRow
            .expect("non-null function pointer")(src, dst_width, 1 as libc::c_int);
        VP8LConvertFromBGRA(src, dst_width, colorspace, dst);
        dst = dst.offset(rgba_stride as isize);
        num_lines_out += 1;
        num_lines_out;
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledRowsRGBA(
    dec: *const VP8LDecoder,
    mut in_0: *mut uint8_t,
    mut in_stride: libc::c_int,
    mut mb_h: libc::c_int,
    out: *mut uint8_t,
    mut out_stride: libc::c_int,
) -> libc::c_int {
    let colorspace: WEBP_CSP_MODE = (*(*dec).output_).colorspace;
    let mut num_lines_in: libc::c_int = 0 as libc::c_int;
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    while num_lines_in < mb_h {
        let row_in: *mut uint8_t = in_0
            .offset(
                (num_lines_in as uint64_t).wrapping_mul(in_stride as libc::c_ulong)
                    as isize,
            );
        let row_out: *mut uint8_t = out
            .offset(
                (num_lines_out as uint64_t).wrapping_mul(out_stride as libc::c_ulong)
                    as isize,
            );
        let lines_left: libc::c_int = mb_h - num_lines_in;
        let needed_lines: libc::c_int = WebPRescaleNeededLines(
            (*dec).rescaler,
            lines_left,
        );
        let mut lines_imported: libc::c_int = 0;
        WebPMultARGBRows(
            row_in,
            in_stride,
            (*(*dec).rescaler).src_width,
            needed_lines,
            0 as libc::c_int,
        );
        lines_imported = WebPRescalerImport(
            (*dec).rescaler,
            lines_left,
            row_in,
            in_stride,
        );
        num_lines_in += lines_imported;
        num_lines_out += Export((*dec).rescaler, colorspace, out_stride, row_out);
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRows(
    mut colorspace: WEBP_CSP_MODE,
    mut row_in: *const uint8_t,
    mut in_stride: libc::c_int,
    mut mb_w: libc::c_int,
    mut mb_h: libc::c_int,
    out: *mut uint8_t,
    mut out_stride: libc::c_int,
) -> libc::c_int {
    let mut lines: libc::c_int = mb_h;
    let mut row_out: *mut uint8_t = out;
    loop {
        let fresh6 = lines;
        lines = lines - 1;
        if !(fresh6 > 0 as libc::c_int) {
            break;
        }
        VP8LConvertFromBGRA(row_in as *const uint32_t, mb_w, colorspace, row_out);
        row_in = row_in.offset(in_stride as isize);
        row_out = row_out.offset(out_stride as isize);
    }
    return mb_h;
}
unsafe extern "C" fn ConvertToYUVA(
    src: *const uint32_t,
    mut width: libc::c_int,
    mut y_pos: libc::c_int,
    output: *const WebPDecBuffer,
) {
    let buf: *const WebPYUVABuffer = &(*output).u.YUVA;
    WebPConvertARGBToY
        .expect(
            "non-null function pointer",
        )(src, ((*buf).y).offset((y_pos * (*buf).y_stride) as isize), width);
    let u: *mut uint8_t = ((*buf).u)
        .offset(((y_pos >> 1 as libc::c_int) * (*buf).u_stride) as isize);
    let v: *mut uint8_t = ((*buf).v)
        .offset(((y_pos >> 1 as libc::c_int) * (*buf).v_stride) as isize);
    WebPConvertARGBToUV
        .expect(
            "non-null function pointer",
        )(src, u, v, width, (y_pos & 1 as libc::c_int == 0) as libc::c_int);
    if !((*buf).a).is_null() {
        let a: *mut uint8_t = ((*buf).a).offset((y_pos * (*buf).a_stride) as isize);
        WebPExtractAlpha
            .expect(
                "non-null function pointer",
            )(
            (src as *mut uint8_t).offset(3 as libc::c_int as isize),
            0 as libc::c_int,
            width,
            1 as libc::c_int,
            a,
            0 as libc::c_int,
        );
    }
}
unsafe extern "C" fn ExportYUVA(
    dec: *const VP8LDecoder,
    mut y_pos: libc::c_int,
) -> libc::c_int {
    let rescaler: *mut WebPRescaler = (*dec).rescaler;
    let src: *mut uint32_t = (*rescaler).dst as *mut uint32_t;
    let dst_width: libc::c_int = (*rescaler).dst_width;
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    while WebPRescalerHasPendingOutput(rescaler) != 0 {
        WebPRescalerExportRow(rescaler);
        WebPMultARGBRow
            .expect("non-null function pointer")(src, dst_width, 1 as libc::c_int);
        ConvertToYUVA(src, dst_width, y_pos, (*dec).output_);
        y_pos += 1;
        y_pos;
        num_lines_out += 1;
        num_lines_out;
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledRowsYUVA(
    dec: *const VP8LDecoder,
    mut in_0: *mut uint8_t,
    mut in_stride: libc::c_int,
    mut mb_h: libc::c_int,
) -> libc::c_int {
    let mut num_lines_in: libc::c_int = 0 as libc::c_int;
    let mut y_pos: libc::c_int = (*dec).last_out_row_;
    while num_lines_in < mb_h {
        let lines_left: libc::c_int = mb_h - num_lines_in;
        let needed_lines: libc::c_int = WebPRescaleNeededLines(
            (*dec).rescaler,
            lines_left,
        );
        let mut lines_imported: libc::c_int = 0;
        WebPMultARGBRows(
            in_0,
            in_stride,
            (*(*dec).rescaler).src_width,
            needed_lines,
            0 as libc::c_int,
        );
        lines_imported = WebPRescalerImport(
            (*dec).rescaler,
            lines_left,
            in_0,
            in_stride,
        );
        num_lines_in += lines_imported;
        in_0 = in_0.offset((needed_lines * in_stride) as isize);
        y_pos += ExportYUVA(dec, y_pos);
    }
    return y_pos;
}
unsafe extern "C" fn EmitRowsYUVA(
    dec: *const VP8LDecoder,
    mut in_0: *const uint8_t,
    mut in_stride: libc::c_int,
    mut mb_w: libc::c_int,
    mut num_rows: libc::c_int,
) -> libc::c_int {
    let mut y_pos: libc::c_int = (*dec).last_out_row_;
    loop {
        let fresh7 = num_rows;
        num_rows = num_rows - 1;
        if !(fresh7 > 0 as libc::c_int) {
            break;
        }
        ConvertToYUVA(in_0 as *const uint32_t, mb_w, y_pos, (*dec).output_);
        in_0 = in_0.offset(in_stride as isize);
        y_pos += 1;
        y_pos;
    }
    return y_pos;
}
unsafe extern "C" fn SetCropWindow(
    io: *mut VP8Io,
    mut y_start: libc::c_int,
    mut y_end: libc::c_int,
    in_data: *mut *mut uint8_t,
    mut pixel_stride: libc::c_int,
) -> libc::c_int {
    if y_end > (*io).crop_bottom {
        y_end = (*io).crop_bottom;
    }
    if y_start < (*io).crop_top {
        let delta: libc::c_int = (*io).crop_top - y_start;
        y_start = (*io).crop_top;
        *in_data = (*in_data).offset((delta * pixel_stride) as isize);
    }
    if y_start >= y_end {
        return 0 as libc::c_int;
    }
    *in_data = (*in_data)
        .offset(
            ((*io).crop_left as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                as isize,
        );
    (*io).mb_y = y_start - (*io).crop_top;
    (*io).mb_w = (*io).crop_right - (*io).crop_left;
    (*io).mb_h = y_end - y_start;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn GetMetaIndex(
    image: *const uint32_t,
    mut xsize: libc::c_int,
    mut bits: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    if bits == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return *image.offset((xsize * (y >> bits) + (x >> bits)) as isize) as libc::c_int;
}
#[inline]
unsafe extern "C" fn GetHtreeGroupForPos(
    hdr: *mut VP8LMetadata,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> *mut HTreeGroup {
    let meta_index: libc::c_int = GetMetaIndex(
        (*hdr).huffman_image_,
        (*hdr).huffman_xsize_,
        (*hdr).huffman_subsample_bits_,
        x,
        y,
    );
    return ((*hdr).htree_groups_).offset(meta_index as isize);
}
unsafe extern "C" fn ApplyInverseTransforms(
    dec: *mut VP8LDecoder,
    mut start_row: libc::c_int,
    mut num_rows: libc::c_int,
    rows: *const uint32_t,
) {
    let mut n: libc::c_int = (*dec).next_transform_;
    let cache_pixs: libc::c_int = (*dec).width_ * num_rows;
    let end_row: libc::c_int = start_row + num_rows;
    let mut rows_in: *const uint32_t = rows;
    let rows_out: *mut uint32_t = (*dec).argb_cache_;
    loop {
        let fresh8 = n;
        n = n - 1;
        if !(fresh8 > 0 as libc::c_int) {
            break;
        }
        let transform: *mut VP8LTransform = &mut *((*dec).transforms_)
            .as_mut_ptr()
            .offset(n as isize) as *mut VP8LTransform;
        VP8LInverseTransform(transform, start_row, end_row, rows_in, rows_out);
        rows_in = rows_out;
    }
    if rows_in != rows_out as *const uint32_t {
        memcpy(
            rows_out as *mut libc::c_void,
            rows_in as *const libc::c_void,
            (cache_pixs as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn ProcessRows(dec: *mut VP8LDecoder, mut row: libc::c_int) {
    let rows: *const uint32_t = ((*dec).pixels_)
        .offset(((*dec).width_ * (*dec).last_row_) as isize);
    let num_rows: libc::c_int = row - (*dec).last_row_;
    if num_rows > 0 as libc::c_int {
        let io: *mut VP8Io = (*dec).io_;
        let mut rows_data: *mut uint8_t = (*dec).argb_cache_ as *mut uint8_t;
        let in_stride: libc::c_int = ((*io).width as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_int;
        ApplyInverseTransforms(dec, (*dec).last_row_, num_rows, rows);
        if !(SetCropWindow(io, (*dec).last_row_, row, &mut rows_data, in_stride) == 0) {
            let output: *const WebPDecBuffer = (*dec).output_;
            if WebPIsRGBMode((*output).colorspace) != 0 {
                let buf: *const WebPRGBABuffer = &(*output).u.RGBA;
                let rgba: *mut uint8_t = ((*buf).rgba)
                    .offset(
                        ((*dec).last_out_row_ as int64_t * (*buf).stride as libc::c_long)
                            as isize,
                    );
                let num_rows_out: libc::c_int = if (*io).use_scaling != 0 {
                    EmitRescaledRowsRGBA(
                        dec,
                        rows_data,
                        in_stride,
                        (*io).mb_h,
                        rgba,
                        (*buf).stride,
                    )
                } else {
                    EmitRows(
                        (*output).colorspace,
                        rows_data,
                        in_stride,
                        (*io).mb_w,
                        (*io).mb_h,
                        rgba,
                        (*buf).stride,
                    )
                };
                (*dec).last_out_row_ += num_rows_out;
            } else {
                (*dec)
                    .last_out_row_ = if (*io).use_scaling != 0 {
                    EmitRescaledRowsYUVA(dec, rows_data, in_stride, (*io).mb_h)
                } else {
                    EmitRowsYUVA(dec, rows_data, in_stride, (*io).mb_w, (*io).mb_h)
                };
            }
        }
    }
    (*dec).last_row_ = row;
}
unsafe extern "C" fn Is8bOptimizable(hdr: *const VP8LMetadata) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*hdr).color_cache_size_ > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*hdr).num_htree_groups_ {
        let htrees: *mut *mut HuffmanCode = ((*((*hdr).htree_groups_).offset(i as isize))
            .htrees)
            .as_mut_ptr();
        if (*(*htrees.offset(RED as libc::c_int as isize))
            .offset(0 as libc::c_int as isize))
            .bits as libc::c_int > 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if (*(*htrees.offset(BLUE as libc::c_int as isize))
            .offset(0 as libc::c_int as isize))
            .bits as libc::c_int > 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if (*(*htrees.offset(ALPHA as libc::c_int as isize))
            .offset(0 as libc::c_int as isize))
            .bits as libc::c_int > 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn AlphaApplyFilter(
    alph_dec: *mut ALPHDecoder,
    mut first_row: libc::c_int,
    mut last_row: libc::c_int,
    mut out: *mut uint8_t,
    mut stride: libc::c_int,
) {
    if (*alph_dec).filter_ as libc::c_uint
        != WEBP_FILTER_NONE as libc::c_int as libc::c_uint
    {
        let mut y: libc::c_int = 0;
        let mut prev_line: *const uint8_t = (*alph_dec).prev_line_;
        y = first_row;
        while y < last_row {
            (WebPUnfilters[(*alph_dec).filter_ as usize])
                .expect("non-null function pointer")(prev_line, out, out, stride);
            prev_line = out;
            out = out.offset(stride as isize);
            y += 1;
            y;
        }
        (*alph_dec).prev_line_ = prev_line;
    }
}
unsafe extern "C" fn ExtractPalettedAlphaRows(
    dec: *mut VP8LDecoder,
    mut last_row: libc::c_int,
) {
    let alph_dec: *mut ALPHDecoder = (*(*dec).io_).opaque as *mut ALPHDecoder;
    let top_row: libc::c_int = if (*alph_dec).filter_ as libc::c_uint
        == WEBP_FILTER_NONE as libc::c_int as libc::c_uint
        || (*alph_dec).filter_ as libc::c_uint
            == WEBP_FILTER_HORIZONTAL as libc::c_int as libc::c_uint
    {
        (*(*dec).io_).crop_top
    } else {
        (*dec).last_row_
    };
    let first_row: libc::c_int = if (*dec).last_row_ < top_row {
        top_row
    } else {
        (*dec).last_row_
    };
    if last_row > first_row {
        let width: libc::c_int = (*(*dec).io_).width;
        let mut out: *mut uint8_t = ((*alph_dec).output_)
            .offset((width * first_row) as isize);
        let in_0: *const uint8_t = ((*dec).pixels_ as *mut uint8_t)
            .offset(((*dec).width_ * first_row) as isize);
        let transform: *mut VP8LTransform = &mut *((*dec).transforms_)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut VP8LTransform;
        VP8LColorIndexInverseTransformAlpha(transform, first_row, last_row, in_0, out);
        AlphaApplyFilter(alph_dec, first_row, last_row, out, width);
    }
    (*dec).last_out_row_ = last_row;
    (*dec).last_row_ = (*dec).last_out_row_;
}
#[inline]
unsafe extern "C" fn Rotate8b(mut V: uint32_t) -> uint32_t {
    return (V & 0xff as libc::c_uint) << 24 as libc::c_int | V >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn CopySmallPattern8b(
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
    mut length: libc::c_int,
    mut pattern: uint32_t,
) {
    let mut i: libc::c_int = 0;
    while dst as uintptr_t & 3 as libc::c_int as libc::c_ulong != 0 {
        let fresh9 = src;
        src = src.offset(1);
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = *fresh9;
        pattern = Rotate8b(pattern);
        length -= 1;
        length;
    }
    i = 0 as libc::c_int;
    while i < length >> 2 as libc::c_int {
        *(dst as *mut uint32_t).offset(i as isize) = pattern;
        i += 1;
        i;
    }
    i <<= 2 as libc::c_int;
    while i < length {
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn CopyBlock8b(
    dst: *mut uint8_t,
    mut dist: libc::c_int,
    mut length: libc::c_int,
) {
    let mut current_block: u64;
    let mut src: *const uint8_t = dst.offset(-(dist as isize));
    if length >= 8 as libc::c_int {
        let mut pattern: uint32_t = 0 as libc::c_int as uint32_t;
        match dist {
            1 => {
                current_block = 11856292385005058703;
                match current_block {
                    9857270992107966733 => {
                        memcpy(
                            &mut pattern as *mut uint32_t as *mut libc::c_void,
                            src as *const libc::c_void,
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        );
                    }
                    11856292385005058703 => {
                        pattern = *src.offset(0 as libc::c_int as isize) as uint32_t;
                        pattern = (0x1010101 as libc::c_uint).wrapping_mul(pattern);
                    }
                    _ => {
                        memcpy(
                            &mut pattern as *mut uint32_t as *mut libc::c_void,
                            src as *const libc::c_void,
                            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        );
                        pattern = (0x10001 as libc::c_uint).wrapping_mul(pattern);
                    }
                }
                CopySmallPattern8b(src, dst, length, pattern);
                return;
            }
            2 => {
                current_block = 12766785672648116837;
                match current_block {
                    9857270992107966733 => {
                        memcpy(
                            &mut pattern as *mut uint32_t as *mut libc::c_void,
                            src as *const libc::c_void,
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        );
                    }
                    11856292385005058703 => {
                        pattern = *src.offset(0 as libc::c_int as isize) as uint32_t;
                        pattern = (0x1010101 as libc::c_uint).wrapping_mul(pattern);
                    }
                    _ => {
                        memcpy(
                            &mut pattern as *mut uint32_t as *mut libc::c_void,
                            src as *const libc::c_void,
                            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        );
                        pattern = (0x10001 as libc::c_uint).wrapping_mul(pattern);
                    }
                }
                CopySmallPattern8b(src, dst, length, pattern);
                return;
            }
            4 => {
                current_block = 9857270992107966733;
                match current_block {
                    9857270992107966733 => {
                        memcpy(
                            &mut pattern as *mut uint32_t as *mut libc::c_void,
                            src as *const libc::c_void,
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        );
                    }
                    11856292385005058703 => {
                        pattern = *src.offset(0 as libc::c_int as isize) as uint32_t;
                        pattern = (0x1010101 as libc::c_uint).wrapping_mul(pattern);
                    }
                    _ => {
                        memcpy(
                            &mut pattern as *mut uint32_t as *mut libc::c_void,
                            src as *const libc::c_void,
                            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        );
                        pattern = (0x10001 as libc::c_uint).wrapping_mul(pattern);
                    }
                }
                CopySmallPattern8b(src, dst, length, pattern);
                return;
            }
            _ => {}
        }
    }
    if dist >= length {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (length as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
    } else {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < length {
            *dst.offset(i as isize) = *src.offset(i as isize);
            i += 1;
            i;
        }
    };
}
#[inline]
unsafe extern "C" fn CopySmallPattern32b(
    mut src: *const uint32_t,
    mut dst: *mut uint32_t,
    mut length: libc::c_int,
    mut pattern: uint64_t,
) {
    let mut i: libc::c_int = 0;
    if dst as uintptr_t & 4 as libc::c_int as libc::c_ulong != 0 {
        let fresh11 = src;
        src = src.offset(1);
        let fresh12 = dst;
        dst = dst.offset(1);
        *fresh12 = *fresh11;
        pattern = pattern >> 32 as libc::c_int | pattern << 32 as libc::c_int;
        length -= 1;
        length;
    }
    i = 0 as libc::c_int;
    while i < length >> 1 as libc::c_int {
        *(dst as *mut uint64_t).offset(i as isize) = pattern;
        i += 1;
        i;
    }
    if length & 1 as libc::c_int != 0 {
        *dst
            .offset(
                (i << 1 as libc::c_int) as isize,
            ) = *src.offset((i << 1 as libc::c_int) as isize);
    }
}
#[inline]
unsafe extern "C" fn CopyBlock32b(
    dst: *mut uint32_t,
    mut dist: libc::c_int,
    mut length: libc::c_int,
) {
    let src: *const uint32_t = dst.offset(-(dist as isize));
    if dist <= 2 as libc::c_int && length >= 4 as libc::c_int
        && dst as uintptr_t & 3 as libc::c_int as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
    {
        let mut pattern: uint64_t = 0;
        if dist == 1 as libc::c_int {
            pattern = *src.offset(0 as libc::c_int as isize) as uint64_t;
            pattern |= pattern << 32 as libc::c_int;
        } else {
            memcpy(
                &mut pattern as *mut uint64_t as *mut libc::c_void,
                src as *const libc::c_void,
                ::core::mem::size_of::<uint64_t>() as libc::c_ulong,
            );
        }
        CopySmallPattern32b(src, dst, length, pattern);
    } else if dist >= length {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (length as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
    } else {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < length {
            *dst.offset(i as isize) = *src.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn DecodeAlphaData(
    dec: *mut VP8LDecoder,
    data: *mut uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut last_row: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ok: libc::c_int = 1 as libc::c_int;
    let mut row: libc::c_int = (*dec).last_pixel_ / width;
    let mut col: libc::c_int = (*dec).last_pixel_ % width;
    let br: *mut VP8LBitReader = &mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &mut (*dec).hdr_;
    let mut pos: libc::c_int = (*dec).last_pixel_;
    let end: libc::c_int = width * height;
    let last: libc::c_int = width * last_row;
    let len_code_limit: libc::c_int = 256 as libc::c_int + 24 as libc::c_int;
    let mask: libc::c_int = (*hdr).huffman_mask_;
    let mut htree_group: *const HTreeGroup = if pos < last {
        GetHtreeGroupForPos(hdr, col, row)
    } else {
        0 as *mut HTreeGroup
    };
    loop {
        if !((*br).eos_ == 0 && pos < last) {
            current_block = 7427571413727699167;
            break;
        }
        let mut code: libc::c_int = 0;
        if col & mask == 0 as libc::c_int {
            htree_group = GetHtreeGroupForPos(hdr, col, row);
        }
        VP8LFillBitWindow(br);
        code = ReadSymbol((*htree_group).htrees[GREEN as libc::c_int as usize], br);
        if code < 256 as libc::c_int {
            *data.offset(pos as isize) = code as uint8_t;
            pos += 1;
            pos;
            col += 1;
            col;
            if col >= width {
                col = 0 as libc::c_int;
                row += 1;
                row;
                if row <= last_row && row % 16 as libc::c_int == 0 as libc::c_int {
                    ExtractPalettedAlphaRows(dec, row);
                }
            }
        } else if code < len_code_limit {
            let mut dist_code: libc::c_int = 0;
            let mut dist: libc::c_int = 0;
            let length_sym: libc::c_int = code - 256 as libc::c_int;
            let length: libc::c_int = GetCopyLength(length_sym, br);
            let dist_symbol: libc::c_int = ReadSymbol(
                (*htree_group).htrees[DIST as libc::c_int as usize],
                br,
            );
            VP8LFillBitWindow(br);
            dist_code = GetCopyDistance(dist_symbol, br);
            dist = PlaneCodeToDistance(width, dist_code);
            if pos >= dist && end - pos >= length {
                CopyBlock8b(data.offset(pos as isize), dist, length);
                pos += length;
                col += length;
                while col >= width {
                    col -= width;
                    row += 1;
                    row;
                    if row <= last_row && row % 16 as libc::c_int == 0 as libc::c_int {
                        ExtractPalettedAlphaRows(dec, row);
                    }
                }
                if pos < last && col & mask != 0 {
                    htree_group = GetHtreeGroupForPos(hdr, col, row);
                }
            } else {
                ok = 0 as libc::c_int;
                current_block = 14936558846990409905;
                break;
            }
        } else {
            ok = 0 as libc::c_int;
            current_block = 14936558846990409905;
            break;
        }
        (*br).eos_ = VP8LIsEndOfStream(br);
    }
    match current_block {
        7427571413727699167 => {
            ExtractPalettedAlphaRows(dec, if row > last_row { last_row } else { row });
        }
        _ => {}
    }
    (*br).eos_ = VP8LIsEndOfStream(br);
    if ok == 0 || (*br).eos_ != 0 && pos < end {
        return VP8LSetError(
            dec,
            (if (*br).eos_ != 0 {
                VP8_STATUS_SUSPENDED as libc::c_int
            } else {
                VP8_STATUS_BITSTREAM_ERROR as libc::c_int
            }) as VP8StatusCode,
        );
    }
    (*dec).last_pixel_ = pos;
    return ok;
}
unsafe extern "C" fn SaveState(dec: *mut VP8LDecoder, mut last_pixel: libc::c_int) {
    (*dec).saved_br_ = (*dec).br_;
    (*dec).saved_last_pixel_ = last_pixel;
    if (*dec).hdr_.color_cache_size_ > 0 as libc::c_int {
        VP8LColorCacheCopy(
            &mut (*dec).hdr_.color_cache_,
            &mut (*dec).hdr_.saved_color_cache_,
        );
    }
}
unsafe extern "C" fn RestoreState(dec: *mut VP8LDecoder) {
    (*dec).status_ = VP8_STATUS_SUSPENDED;
    (*dec).br_ = (*dec).saved_br_;
    (*dec).last_pixel_ = (*dec).saved_last_pixel_;
    if (*dec).hdr_.color_cache_size_ > 0 as libc::c_int {
        VP8LColorCacheCopy(
            &mut (*dec).hdr_.saved_color_cache_,
            &mut (*dec).hdr_.color_cache_,
        );
    }
}
unsafe extern "C" fn DecodeImageData(
    dec: *mut VP8LDecoder,
    data: *mut uint32_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut last_row: libc::c_int,
    mut process_func: ProcessRowsFunc,
) -> libc::c_int {
    let mut current_block: u64;
    let mut row: libc::c_int = (*dec).last_pixel_ / width;
    let mut col: libc::c_int = (*dec).last_pixel_ % width;
    let br: *mut VP8LBitReader = &mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &mut (*dec).hdr_;
    let mut src: *mut uint32_t = data.offset((*dec).last_pixel_ as isize);
    let mut last_cached: *mut uint32_t = src;
    let src_end: *mut uint32_t = data.offset((width * height) as isize);
    let src_last: *mut uint32_t = data.offset((width * last_row) as isize);
    let len_code_limit: libc::c_int = 256 as libc::c_int + 24 as libc::c_int;
    let color_cache_limit: libc::c_int = len_code_limit + (*hdr).color_cache_size_;
    let mut next_sync_row: libc::c_int = if (*dec).incremental_ != 0 {
        row
    } else {
        (1 as libc::c_int) << 24 as libc::c_int
    };
    let color_cache: *mut VP8LColorCache = if (*hdr).color_cache_size_ > 0 as libc::c_int
    {
        &mut (*hdr).color_cache_
    } else {
        0 as *mut VP8LColorCache
    };
    let mask: libc::c_int = (*hdr).huffman_mask_;
    let mut htree_group: *const HTreeGroup = if src < src_last {
        GetHtreeGroupForPos(hdr, col, row)
    } else {
        0 as *mut HTreeGroup
    };
    loop {
        if !(src < src_last) {
            current_block = 10435735846551762309;
            break;
        }
        let mut code: libc::c_int = 0;
        if row >= next_sync_row {
            SaveState(dec, src.offset_from(data) as libc::c_long as libc::c_int);
            next_sync_row = row + 8 as libc::c_int;
        }
        if col & mask == 0 as libc::c_int {
            htree_group = GetHtreeGroupForPos(hdr, col, row);
        }
        if (*htree_group).is_trivial_code != 0 {
            *src = (*htree_group).literal_arb;
        } else {
            VP8LFillBitWindow(br);
            if (*htree_group).use_packed_table != 0 {
                code = ReadPackedSymbols(htree_group, br, src);
                if VP8LIsEndOfStream(br) != 0 {
                    current_block = 10435735846551762309;
                    break;
                }
                if code == 0 as libc::c_int {
                    current_block = 11933603412437072862;
                } else {
                    current_block = 2668756484064249700;
                }
            } else {
                code = ReadSymbol(
                    (*htree_group).htrees[GREEN as libc::c_int as usize],
                    br,
                );
                current_block = 2668756484064249700;
            }
            match current_block {
                11933603412437072862 => {}
                _ => {
                    if VP8LIsEndOfStream(br) != 0 {
                        current_block = 10435735846551762309;
                        break;
                    }
                    if code < 256 as libc::c_int {
                        if (*htree_group).is_trivial_literal != 0 {
                            *src = (*htree_group).literal_arb
                                | (code << 8 as libc::c_int) as libc::c_uint;
                        } else {
                            let mut red: libc::c_int = 0;
                            let mut blue: libc::c_int = 0;
                            let mut alpha: libc::c_int = 0;
                            red = ReadSymbol(
                                (*htree_group).htrees[RED as libc::c_int as usize],
                                br,
                            );
                            VP8LFillBitWindow(br);
                            blue = ReadSymbol(
                                (*htree_group).htrees[BLUE as libc::c_int as usize],
                                br,
                            );
                            alpha = ReadSymbol(
                                (*htree_group).htrees[ALPHA as libc::c_int as usize],
                                br,
                            );
                            if VP8LIsEndOfStream(br) != 0 {
                                current_block = 10435735846551762309;
                                break;
                            }
                            *src = (alpha as uint32_t) << 24 as libc::c_int
                                | (red << 16 as libc::c_int) as libc::c_uint
                                | (code << 8 as libc::c_int) as libc::c_uint
                                | blue as libc::c_uint;
                        }
                    } else if code < len_code_limit {
                        let mut dist_code: libc::c_int = 0;
                        let mut dist: libc::c_int = 0;
                        let length_sym: libc::c_int = code - 256 as libc::c_int;
                        let length: libc::c_int = GetCopyLength(length_sym, br);
                        let dist_symbol: libc::c_int = ReadSymbol(
                            (*htree_group).htrees[DIST as libc::c_int as usize],
                            br,
                        );
                        VP8LFillBitWindow(br);
                        dist_code = GetCopyDistance(dist_symbol, br);
                        dist = PlaneCodeToDistance(width, dist_code);
                        if VP8LIsEndOfStream(br) != 0 {
                            current_block = 10435735846551762309;
                            break;
                        }
                        if (src.offset_from(data) as libc::c_long) < dist as ptrdiff_t
                            || (src_end.offset_from(src) as libc::c_long)
                                < length as ptrdiff_t
                        {
                            current_block = 14685741201792387029;
                            break;
                        }
                        CopyBlock32b(src, dist, length);
                        src = src.offset(length as isize);
                        col += length;
                        while col >= width {
                            col -= width;
                            row += 1;
                            row;
                            if process_func.is_some() {
                                if row <= last_row
                                    && row % 16 as libc::c_int == 0 as libc::c_int
                                {
                                    process_func.expect("non-null function pointer")(dec, row);
                                }
                            }
                        }
                        if col & mask != 0 {
                            htree_group = GetHtreeGroupForPos(hdr, col, row);
                        }
                        if !color_cache.is_null() {
                            while last_cached < src {
                                let fresh14 = last_cached;
                                last_cached = last_cached.offset(1);
                                VP8LColorCacheInsert(color_cache, *fresh14);
                            }
                        }
                        continue;
                    } else {
                        if !(code < color_cache_limit) {
                            current_block = 14685741201792387029;
                            break;
                        }
                        let key: libc::c_int = code - len_code_limit;
                        while last_cached < src {
                            let fresh15 = last_cached;
                            last_cached = last_cached.offset(1);
                            VP8LColorCacheInsert(color_cache, *fresh15);
                        }
                        *src = VP8LColorCacheLookup(color_cache, key as uint32_t);
                    }
                }
            }
        }
        src = src.offset(1);
        src;
        col += 1;
        col;
        if col >= width {
            col = 0 as libc::c_int;
            row += 1;
            row;
            if process_func.is_some() {
                if row <= last_row && row % 16 as libc::c_int == 0 as libc::c_int {
                    process_func.expect("non-null function pointer")(dec, row);
                }
            }
            if !color_cache.is_null() {
                while last_cached < src {
                    let fresh13 = last_cached;
                    last_cached = last_cached.offset(1);
                    VP8LColorCacheInsert(color_cache, *fresh13);
                }
            }
        }
    }
    match current_block {
        10435735846551762309 => {
            (*br).eos_ = VP8LIsEndOfStream(br);
            if (*dec).incremental_ != 0 && (*br).eos_ != 0 && src < src_last {
                RestoreState(dec);
                current_block = 8062065914618164218;
            } else if (*dec).incremental_ != 0 && src >= src_last || (*br).eos_ == 0 {
                if process_func.is_some() {
                    process_func
                        .expect(
                            "non-null function pointer",
                        )(dec, if row > last_row { last_row } else { row });
                }
                (*dec).status_ = VP8_STATUS_OK;
                (*dec)
                    .last_pixel_ = src.offset_from(data) as libc::c_long as libc::c_int;
                current_block = 8062065914618164218;
            } else {
                current_block = 14685741201792387029;
            }
            match current_block {
                14685741201792387029 => {}
                _ => return 1 as libc::c_int,
            }
        }
        _ => {}
    }
    return VP8LSetError(dec, VP8_STATUS_BITSTREAM_ERROR);
}
unsafe extern "C" fn ClearTransform(transform: *mut VP8LTransform) {
    WebPSafeFree((*transform).data_ as *mut libc::c_void);
    (*transform).data_ = 0 as *mut uint32_t;
}
unsafe extern "C" fn ExpandColorMap(
    mut num_colors: libc::c_int,
    transform: *mut VP8LTransform,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let final_num_colors: libc::c_int = (1 as libc::c_int)
        << (8 as libc::c_int >> (*transform).bits_);
    let new_color_map: *mut uint32_t = WebPSafeMalloc(
        final_num_colors as uint64_t,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    if new_color_map.is_null() {
        return 0 as libc::c_int
    } else {
        let data: *mut uint8_t = (*transform).data_ as *mut uint8_t;
        let new_data: *mut uint8_t = new_color_map as *mut uint8_t;
        *new_color_map
            .offset(
                0 as libc::c_int as isize,
            ) = *((*transform).data_).offset(0 as libc::c_int as isize);
        i = 4 as libc::c_int;
        while i < 4 as libc::c_int * num_colors {
            *new_data
                .offset(
                    i as isize,
                ) = (*data.offset(i as isize) as libc::c_int
                + *new_data.offset((i - 4 as libc::c_int) as isize) as libc::c_int
                & 0xff as libc::c_int) as uint8_t;
            i += 1;
            i;
        }
        while i < 4 as libc::c_int * final_num_colors {
            *new_data.offset(i as isize) = 0 as libc::c_int as uint8_t;
            i += 1;
            i;
        }
        WebPSafeFree((*transform).data_ as *mut libc::c_void);
        (*transform).data_ = new_color_map;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ReadTransform(
    xsize: *mut libc::c_int,
    mut ysize: *const libc::c_int,
    dec: *mut VP8LDecoder,
) -> libc::c_int {
    let mut ok: libc::c_int = 1 as libc::c_int;
    let br: *mut VP8LBitReader = &mut (*dec).br_;
    let mut transform: *mut VP8LTransform = &mut *((*dec).transforms_)
        .as_mut_ptr()
        .offset((*dec).next_transform_ as isize) as *mut VP8LTransform;
    let type_0: VP8LImageTransformType = VP8LReadBits(br, 2 as libc::c_int)
        as VP8LImageTransformType;
    if (*dec).transforms_seen_ & (1 as libc::c_uint) << type_0 as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    (*dec).transforms_seen_ |= (1 as libc::c_uint) << type_0 as libc::c_uint;
    (*transform).type_ = type_0;
    (*transform).xsize_ = *xsize;
    (*transform).ysize_ = *ysize;
    (*transform).data_ = 0 as *mut uint32_t;
    (*dec).next_transform_ += 1;
    (*dec).next_transform_;
    match type_0 as libc::c_uint {
        0 | 1 => {
            (*transform)
                .bits_ = (VP8LReadBits(br, 3 as libc::c_int))
                .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_int;
            ok = DecodeImageStream(
                VP8LSubSampleSize(
                    (*transform).xsize_ as uint32_t,
                    (*transform).bits_ as uint32_t,
                ) as libc::c_int,
                VP8LSubSampleSize(
                    (*transform).ysize_ as uint32_t,
                    (*transform).bits_ as uint32_t,
                ) as libc::c_int,
                0 as libc::c_int,
                dec,
                &mut (*transform).data_,
            );
        }
        3 => {
            let num_colors: libc::c_int = (VP8LReadBits(br, 8 as libc::c_int))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            let bits: libc::c_int = if num_colors > 16 as libc::c_int {
                0 as libc::c_int
            } else if num_colors > 4 as libc::c_int {
                1 as libc::c_int
            } else if num_colors > 2 as libc::c_int {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            *xsize = VP8LSubSampleSize((*transform).xsize_ as uint32_t, bits as uint32_t)
                as libc::c_int;
            (*transform).bits_ = bits;
            ok = DecodeImageStream(
                num_colors,
                1 as libc::c_int,
                0 as libc::c_int,
                dec,
                &mut (*transform).data_,
            );
            if ok != 0 && ExpandColorMap(num_colors, transform) == 0 {
                return VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
            }
        }
        2 | _ => {}
    }
    return ok;
}
unsafe extern "C" fn InitMetadata(hdr: *mut VP8LMetadata) {
    memset(
        hdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8LMetadata>() as libc::c_ulong,
    );
}
unsafe extern "C" fn ClearMetadata(hdr: *mut VP8LMetadata) {
    WebPSafeFree((*hdr).huffman_image_ as *mut libc::c_void);
    VP8LHuffmanTablesDeallocate(&mut (*hdr).huffman_tables_);
    VP8LHtreeGroupsFree((*hdr).htree_groups_);
    VP8LColorCacheClear(&mut (*hdr).color_cache_);
    VP8LColorCacheClear(&mut (*hdr).saved_color_cache_);
    InitMetadata(hdr);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LNew() -> *mut VP8LDecoder {
    let dec: *mut VP8LDecoder = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<VP8LDecoder>() as libc::c_ulong,
    ) as *mut VP8LDecoder;
    if dec.is_null() {
        return 0 as *mut VP8LDecoder;
    }
    (*dec).status_ = VP8_STATUS_OK;
    (*dec).state_ = READ_DIM;
    VP8LDspInit();
    return dec;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LClear(dec: *mut VP8LDecoder) {
    let mut i: libc::c_int = 0;
    if dec.is_null() {
        return;
    }
    ClearMetadata(&mut (*dec).hdr_);
    WebPSafeFree((*dec).pixels_ as *mut libc::c_void);
    (*dec).pixels_ = 0 as *mut uint32_t;
    i = 0 as libc::c_int;
    while i < (*dec).next_transform_ {
        ClearTransform(&mut *((*dec).transforms_).as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
    (*dec).next_transform_ = 0 as libc::c_int;
    (*dec).transforms_seen_ = 0 as libc::c_int as uint32_t;
    WebPSafeFree((*dec).rescaler_memory as *mut libc::c_void);
    (*dec).rescaler_memory = 0 as *mut uint8_t;
    (*dec).output_ = 0 as *const WebPDecBuffer;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDelete(dec: *mut VP8LDecoder) {
    if !dec.is_null() {
        VP8LClear(dec);
        WebPSafeFree(dec as *mut libc::c_void);
    }
}
unsafe extern "C" fn UpdateDecoder(
    dec: *mut VP8LDecoder,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let hdr: *mut VP8LMetadata = &mut (*dec).hdr_;
    let num_bits: libc::c_int = (*hdr).huffman_subsample_bits_;
    (*dec).width_ = width;
    (*dec).height_ = height;
    (*hdr)
        .huffman_xsize_ = VP8LSubSampleSize(width as uint32_t, num_bits as uint32_t)
        as libc::c_int;
    (*hdr)
        .huffman_mask_ = if num_bits == 0 as libc::c_int {
        !(0 as libc::c_int)
    } else {
        ((1 as libc::c_int) << num_bits) - 1 as libc::c_int
    };
}
unsafe extern "C" fn DecodeImageStream(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    mut is_level0: libc::c_int,
    dec: *mut VP8LDecoder,
    decoded_data: *mut *mut uint32_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ok: libc::c_int = 1 as libc::c_int;
    let mut transform_xsize: libc::c_int = xsize;
    let mut transform_ysize: libc::c_int = ysize;
    let br: *mut VP8LBitReader = &mut (*dec).br_;
    let hdr: *mut VP8LMetadata = &mut (*dec).hdr_;
    let mut data: *mut uint32_t = 0 as *mut uint32_t;
    let mut color_cache_bits: libc::c_int = 0 as libc::c_int;
    if is_level0 != 0 {
        while ok != 0 && VP8LReadBits(br, 1 as libc::c_int) != 0 {
            ok = ReadTransform(&mut transform_xsize, &mut transform_ysize, dec);
        }
    }
    if ok != 0 && VP8LReadBits(br, 1 as libc::c_int) != 0 {
        color_cache_bits = VP8LReadBits(br, 4 as libc::c_int) as libc::c_int;
        ok = (color_cache_bits >= 1 as libc::c_int
            && color_cache_bits <= 11 as libc::c_int) as libc::c_int;
        if ok == 0 {
            VP8LSetError(dec, VP8_STATUS_BITSTREAM_ERROR);
            current_block = 13015265850560828554;
        } else {
            current_block = 2968425633554183086;
        }
    } else {
        current_block = 2968425633554183086;
    }
    match current_block {
        2968425633554183086 => {
            ok = (ok != 0
                && ReadHuffmanCodes(
                    dec,
                    transform_xsize,
                    transform_ysize,
                    color_cache_bits,
                    is_level0,
                ) != 0) as libc::c_int;
            if ok == 0 {
                VP8LSetError(dec, VP8_STATUS_BITSTREAM_ERROR);
            } else {
                if color_cache_bits > 0 as libc::c_int {
                    (*hdr).color_cache_size_ = (1 as libc::c_int) << color_cache_bits;
                    if VP8LColorCacheInit(&mut (*hdr).color_cache_, color_cache_bits)
                        == 0
                    {
                        ok = VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
                        current_block = 13015265850560828554;
                    } else {
                        current_block = 2838571290723028321;
                    }
                } else {
                    (*hdr).color_cache_size_ = 0 as libc::c_int;
                    current_block = 2838571290723028321;
                }
                match current_block {
                    13015265850560828554 => {}
                    _ => {
                        UpdateDecoder(dec, transform_xsize, transform_ysize);
                        if is_level0 != 0 {
                            (*dec).state_ = READ_HDR;
                        } else {
                            let total_size: uint64_t = (transform_xsize as uint64_t)
                                .wrapping_mul(transform_ysize as libc::c_ulong);
                            data = WebPSafeMalloc(
                                total_size,
                                ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                            ) as *mut uint32_t;
                            if data.is_null() {
                                ok = VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
                            } else {
                                ok = DecodeImageData(
                                    dec,
                                    data,
                                    transform_xsize,
                                    transform_ysize,
                                    transform_ysize,
                                    None,
                                );
                                ok = (ok != 0 && (*br).eos_ == 0) as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if ok == 0 {
        WebPSafeFree(data as *mut libc::c_void);
        ClearMetadata(hdr);
    } else {
        if !decoded_data.is_null() {
            *decoded_data = data;
        }
        (*dec).last_pixel_ = 0 as libc::c_int;
        if is_level0 == 0 {
            ClearMetadata(hdr);
        }
    }
    return ok;
}
unsafe extern "C" fn AllocateInternalBuffers32b(
    dec: *mut VP8LDecoder,
    mut final_width: libc::c_int,
) -> libc::c_int {
    let num_pixels: uint64_t = ((*dec).width_ as uint64_t)
        .wrapping_mul((*dec).height_ as libc::c_ulong);
    let cache_top_pixels: uint64_t = final_width as uint16_t as uint64_t;
    let cache_pixels: uint64_t = (final_width as uint64_t)
        .wrapping_mul(16 as libc::c_int as libc::c_ulong);
    let total_num_pixels: uint64_t = num_pixels
        .wrapping_add(cache_top_pixels)
        .wrapping_add(cache_pixels);
    (*dec)
        .pixels_ = WebPSafeMalloc(
        total_num_pixels,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    if ((*dec).pixels_).is_null() {
        (*dec).argb_cache_ = 0 as *mut uint32_t;
        return VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
    }
    (*dec)
        .argb_cache_ = ((*dec).pixels_)
        .offset(num_pixels as isize)
        .offset(cache_top_pixels as isize);
    return 1 as libc::c_int;
}
unsafe extern "C" fn AllocateInternalBuffers8b(dec: *mut VP8LDecoder) -> libc::c_int {
    let total_num_pixels: uint64_t = ((*dec).width_ as uint64_t)
        .wrapping_mul((*dec).height_ as libc::c_ulong);
    (*dec).argb_cache_ = 0 as *mut uint32_t;
    (*dec)
        .pixels_ = WebPSafeMalloc(
        total_num_pixels,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    if ((*dec).pixels_).is_null() {
        return VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ExtractAlphaRows(dec: *mut VP8LDecoder, mut last_row: libc::c_int) {
    let mut cur_row: libc::c_int = (*dec).last_row_;
    let mut num_rows: libc::c_int = last_row - cur_row;
    let mut in_0: *const uint32_t = ((*dec).pixels_)
        .offset(((*dec).width_ * cur_row) as isize);
    while num_rows > 0 as libc::c_int {
        let num_rows_to_process: libc::c_int = if num_rows > 16 as libc::c_int {
            16 as libc::c_int
        } else {
            num_rows
        };
        let alph_dec: *mut ALPHDecoder = (*(*dec).io_).opaque as *mut ALPHDecoder;
        let output: *mut uint8_t = (*alph_dec).output_;
        let width: libc::c_int = (*(*dec).io_).width;
        let cache_pixs: libc::c_int = width * num_rows_to_process;
        let dst: *mut uint8_t = output.offset((width * cur_row) as isize);
        let src: *const uint32_t = (*dec).argb_cache_;
        ApplyInverseTransforms(dec, cur_row, num_rows_to_process, in_0);
        WebPExtractGreen.expect("non-null function pointer")(src, dst, cache_pixs);
        AlphaApplyFilter(alph_dec, cur_row, cur_row + num_rows_to_process, dst, width);
        num_rows -= num_rows_to_process;
        in_0 = in_0.offset((num_rows_to_process * (*dec).width_) as isize);
        cur_row += num_rows_to_process;
    }
    (*dec).last_out_row_ = last_row;
    (*dec).last_row_ = (*dec).last_out_row_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeAlphaHeader(
    alph_dec: *mut ALPHDecoder,
    data: *const uint8_t,
    mut data_size: size_t,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut dec: *mut VP8LDecoder = VP8LNew();
    if dec.is_null() {
        return 0 as libc::c_int;
    }
    (*dec).width_ = (*alph_dec).width_;
    (*dec).height_ = (*alph_dec).height_;
    (*dec).io_ = &mut (*alph_dec).io_;
    (*(*dec).io_).opaque = alph_dec as *mut libc::c_void;
    (*(*dec).io_).width = (*alph_dec).width_;
    (*(*dec).io_).height = (*alph_dec).height_;
    (*dec).status_ = VP8_STATUS_OK;
    VP8LInitBitReader(&mut (*dec).br_, data, data_size);
    if !(DecodeImageStream(
        (*alph_dec).width_,
        (*alph_dec).height_,
        1 as libc::c_int,
        dec,
        0 as *mut *mut uint32_t,
    ) == 0)
    {
        if (*dec).next_transform_ == 1 as libc::c_int
            && (*dec).transforms_[0 as libc::c_int as usize].type_ as libc::c_uint
                == COLOR_INDEXING_TRANSFORM as libc::c_int as libc::c_uint
            && Is8bOptimizable(&mut (*dec).hdr_) != 0
        {
            (*alph_dec).use_8b_decode_ = 1 as libc::c_int;
            ok = AllocateInternalBuffers8b(dec);
        } else {
            (*alph_dec).use_8b_decode_ = 0 as libc::c_int;
            ok = AllocateInternalBuffers32b(dec, (*alph_dec).width_);
        }
        if !(ok == 0) {
            (*alph_dec).vp8l_dec_ = dec;
            return 1 as libc::c_int;
        }
    }
    VP8LDelete(dec);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeAlphaImageStream(
    alph_dec: *mut ALPHDecoder,
    mut last_row: libc::c_int,
) -> libc::c_int {
    let dec: *mut VP8LDecoder = (*alph_dec).vp8l_dec_;
    if (*dec).last_row_ >= last_row {
        return 1 as libc::c_int;
    }
    if (*alph_dec).use_8b_decode_ == 0 {
        WebPInitAlphaProcessing();
    }
    return if (*alph_dec).use_8b_decode_ != 0 {
        DecodeAlphaData(
            dec,
            (*dec).pixels_ as *mut uint8_t,
            (*dec).width_,
            (*dec).height_,
            last_row,
        )
    } else {
        DecodeImageData(
            dec,
            (*dec).pixels_,
            (*dec).width_,
            (*dec).height_,
            last_row,
            Some(
                ExtractAlphaRows
                    as unsafe extern "C" fn(*mut VP8LDecoder, libc::c_int) -> (),
            ),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeHeader(
    dec: *mut VP8LDecoder,
    io: *mut VP8Io,
) -> libc::c_int {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut has_alpha: libc::c_int = 0;
    if dec.is_null() {
        return 0 as libc::c_int;
    }
    if io.is_null() {
        return VP8LSetError(dec, VP8_STATUS_INVALID_PARAM);
    }
    (*dec).io_ = io;
    (*dec).status_ = VP8_STATUS_OK;
    VP8LInitBitReader(&mut (*dec).br_, (*io).data, (*io).data_size);
    if ReadImageInfo(&mut (*dec).br_, &mut width, &mut height, &mut has_alpha) == 0 {
        VP8LSetError(dec, VP8_STATUS_BITSTREAM_ERROR);
    } else {
        (*dec).state_ = READ_DIM;
        (*io).width = width;
        (*io).height = height;
        if !(DecodeImageStream(
            width,
            height,
            1 as libc::c_int,
            dec,
            0 as *mut *mut uint32_t,
        ) == 0)
        {
            return 1 as libc::c_int;
        }
    }
    VP8LClear(dec);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDecodeImage(dec: *mut VP8LDecoder) -> libc::c_int {
    let mut current_block: u64;
    let mut io: *mut VP8Io = 0 as *mut VP8Io;
    let mut params: *mut WebPDecParams = 0 as *mut WebPDecParams;
    if dec.is_null() {
        return 0 as libc::c_int;
    }
    io = (*dec).io_;
    params = (*io).opaque as *mut WebPDecParams;
    if (*dec).state_ as libc::c_uint != READ_DATA as libc::c_int as libc::c_uint {
        (*dec).output_ = (*params).output;
        if WebPIoInitFromOptions((*params).options, io, MODE_BGRA) == 0 {
            VP8LSetError(dec, VP8_STATUS_INVALID_PARAM);
            current_block = 1602694546607873112;
        } else if AllocateInternalBuffers32b(dec, (*io).width) == 0 {
            current_block = 1602694546607873112;
        } else if (*io).use_scaling != 0 && AllocateAndInitRescaler(dec, io) == 0 {
            current_block = 1602694546607873112;
        } else {
            if (*io).use_scaling != 0
                || WebPIsPremultipliedMode((*(*dec).output_).colorspace) != 0
            {
                WebPInitAlphaProcessing();
            }
            if WebPIsRGBMode((*(*dec).output_).colorspace) == 0 {
                WebPInitConvertARGBToYUV();
                if !((*(*dec).output_).u.YUVA.a).is_null() {
                    WebPInitAlphaProcessing();
                }
            }
            if (*dec).incremental_ != 0 {
                if (*dec).hdr_.color_cache_size_ > 0 as libc::c_int
                    && ((*dec).hdr_.saved_color_cache_.colors_).is_null()
                {
                    if VP8LColorCacheInit(
                        &mut (*dec).hdr_.saved_color_cache_,
                        (*dec).hdr_.color_cache_.hash_bits_,
                    ) == 0
                    {
                        VP8LSetError(dec, VP8_STATUS_OUT_OF_MEMORY);
                        current_block = 1602694546607873112;
                    } else {
                        current_block = 5634871135123216486;
                    }
                } else {
                    current_block = 5634871135123216486;
                }
            } else {
                current_block = 5634871135123216486;
            }
            match current_block {
                1602694546607873112 => {}
                _ => {
                    (*dec).state_ = READ_DATA;
                    current_block = 17478428563724192186;
                }
            }
        }
    } else {
        current_block = 17478428563724192186;
    }
    match current_block {
        17478428563724192186 => {
            if !(DecodeImageData(
                dec,
                (*dec).pixels_,
                (*dec).width_,
                (*dec).height_,
                (*io).crop_bottom,
                Some(
                    ProcessRows
                        as unsafe extern "C" fn(*mut VP8LDecoder, libc::c_int) -> (),
                ),
            ) == 0)
            {
                (*params).last_y = (*dec).last_out_row_;
                return 1 as libc::c_int;
            }
        }
        _ => {}
    }
    VP8LClear(dec);
    return 0 as libc::c_int;
}
