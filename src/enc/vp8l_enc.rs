use ::libc;

use super::backward_references_enc::PixOrCopyBlock;
extern "C" {
    fn WebPPictureHasTransparency(picture: *const WebPPicture) -> libc::c_int;
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
    fn WebPPictureInitInternal(_: *mut WebPPicture, _: libc::c_int) -> libc::c_int;
    fn WebPPictureView(
        src: *const WebPPicture,
        left: libc::c_int,
        top: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        dst: *mut WebPPicture,
    ) -> libc::c_int;
    fn VP8LHashChainInit(p: *mut VP8LHashChain, size: libc::c_int) -> libc::c_int;
    fn VP8LHashChainFill(
        p: *mut VP8LHashChain,
        quality: libc::c_int,
        argb: *const uint32_t,
        xsize: libc::c_int,
        ysize: libc::c_int,
        low_effort: libc::c_int,
        pic: *const WebPPicture,
        percent_range: libc::c_int,
        percent: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8LHashChainClear(p: *mut VP8LHashChain);
    fn VP8LBackwardRefsInit(refs: *mut VP8LBackwardRefs, block_size: libc::c_int);
    fn VP8LBackwardRefsClear(refs: *mut VP8LBackwardRefs);
    fn VP8LRefsCursorInit(refs: *const VP8LBackwardRefs) -> VP8LRefsCursor;
    fn VP8LRefsCursorNextBlock(c: *mut VP8LRefsCursor);
    fn VP8LGetBackwardReferences(
        width: libc::c_int,
        height: libc::c_int,
        argb: *const uint32_t,
        quality: libc::c_int,
        low_effort: libc::c_int,
        lz77_types_to_try: libc::c_int,
        cache_bits_max: libc::c_int,
        do_no_cache: libc::c_int,
        hash_chain: *const VP8LHashChain,
        refs: *mut VP8LBackwardRefs,
        cache_bits_best: *mut libc::c_int,
        pic: *const WebPPicture,
        percent_range: libc::c_int,
        percent: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8LHistogramStoreRefs(refs: *const VP8LBackwardRefs, histo: *mut VP8LHistogram);
    fn VP8LFreeHistogram(histo: *mut VP8LHistogram);
    fn VP8LFreeHistogramSet(histo: *mut VP8LHistogramSet);
    fn VP8LAllocateHistogramSet(
        size: libc::c_int,
        cache_bits: libc::c_int,
    ) -> *mut VP8LHistogramSet;
    fn VP8LHistogramSetClear(set: *mut VP8LHistogramSet);
    fn VP8LAllocateHistogram(cache_bits: libc::c_int) -> *mut VP8LHistogram;
    fn VP8LGetHistoImageSymbols(
        xsize: libc::c_int,
        ysize: libc::c_int,
        refs: *const VP8LBackwardRefs,
        quality: libc::c_int,
        low_effort: libc::c_int,
        histogram_bits: libc::c_int,
        cache_bits: libc::c_int,
        image_histo: *mut VP8LHistogramSet,
        tmp_histo: *mut VP8LHistogram,
        histogram_symbols: *mut uint16_t,
        pic: *const WebPPicture,
        percent_range: libc::c_int,
        percent: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8LBitsEntropy(array: *const uint32_t, n: libc::c_int) -> libc::c_float;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    static mut VP8LSubtractGreenFromBlueAndRed: VP8LProcessEncBlueAndRedFunc;
    static mut VP8LBundleColorMap: VP8LBundleColorMapFunc;
    fn VP8LEncDspInit();
    static kPrefixEncodeExtraBitsValue: [uint8_t; 512];
    static kLog2Table: [libc::c_float; 256];
    static mut VP8LFastLog2Slow: VP8LFastLog2SlowFunc;
    static kPrefixEncodeCode: [VP8LPrefixCode; 512];
    fn VP8LBitWriterInit(bw: *mut VP8LBitWriter, expected_size: size_t) -> libc::c_int;
    fn VP8LBitWriterClone(
        src: *const VP8LBitWriter,
        dst: *mut VP8LBitWriter,
    ) -> libc::c_int;
    fn VP8LBitWriterFinish(bw: *mut VP8LBitWriter) -> *mut uint8_t;
    fn VP8LBitWriterWipeOut(bw: *mut VP8LBitWriter);
    fn VP8LBitWriterReset(bw_init: *const VP8LBitWriter, bw: *mut VP8LBitWriter);
    fn VP8LBitWriterSwap(src: *mut VP8LBitWriter, dst: *mut VP8LBitWriter);
    fn VP8LPutBitsFlushBits(bw: *mut VP8LBitWriter);
    fn VP8LPutBitsInternal(bw: *mut VP8LBitWriter, bits: uint32_t, n_bits: libc::c_int);
    fn WebPGetWorkerInterface() -> *const WebPWorkerInterface;
    fn WebPEncodingSetError(
        pic: *const WebPPicture,
        error: WebPEncodingError,
    ) -> libc::c_int;
    fn WebPReportProgress(
        pic: *const WebPPicture,
        percent: libc::c_int,
        percent_store: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8ApplyNearLossless(
        picture: *const WebPPicture,
        quality: libc::c_int,
        argb_dst: *mut uint32_t,
    ) -> libc::c_int;
    fn VP8LResidualImage(
        width: libc::c_int,
        height: libc::c_int,
        bits: libc::c_int,
        low_effort: libc::c_int,
        argb: *mut uint32_t,
        argb_scratch: *mut uint32_t,
        image: *mut uint32_t,
        near_lossless: libc::c_int,
        exact: libc::c_int,
        used_subtract_green: libc::c_int,
        pic: *const WebPPicture,
        percent_range: libc::c_int,
        percent: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8LColorSpaceTransform(
        width: libc::c_int,
        height: libc::c_int,
        bits: libc::c_int,
        quality: libc::c_int,
        argb: *mut uint32_t,
        image: *mut uint32_t,
        pic: *const WebPPicture,
        percent_range: libc::c_int,
        percent: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8LCreateCompressedHuffmanTree(
        tree: *const HuffmanTreeCode,
        tokens: *mut HuffmanTreeToken,
        max_tokens: libc::c_int,
    ) -> libc::c_int;
    fn VP8LCreateHuffmanTree(
        histogram: *mut uint32_t,
        tree_depth_limit: libc::c_int,
        buf_rle: *mut uint8_t,
        huff_tree: *mut HuffmanTree,
        huff_code: *mut HuffmanTreeCode,
    );
    fn SearchColorNoIdx(
        sorted: *const uint32_t,
        color: uint32_t,
        num_colors: libc::c_int,
    ) -> libc::c_int;
    fn PrepareMapToPalette(
        palette: *const uint32_t,
        num_colors: uint32_t,
        sorted: *mut uint32_t,
        idx_map: *mut uint32_t,
    );
    fn GetColorPalette(pic: *const WebPPicture, palette: *mut uint32_t) -> libc::c_int;
    fn PaletteSort(
        method: PaletteSorting,
        pic: *const WebPPicture,
        palette_sorted: *const uint32_t,
        num_colors: uint32_t,
        palette: *mut uint32_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_uint;
pub const COLOR_INDEXING_TRANSFORM: C2RustUnnamed = 3;
pub const SUBTRACT_GREEN_TRANSFORM: C2RustUnnamed = 2;
pub const CROSS_COLOR_TRANSFORM: C2RustUnnamed = 1;
pub const PREDICTOR_TRANSFORM: C2RustUnnamed = 0;
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
pub struct VP8LHistogramSet {
    pub size: libc::c_int,
    pub max_size: libc::c_int,
    pub histograms: *mut *mut VP8LHistogram,
}
pub type VP8LProcessEncBlueAndRedFunc = Option::<
    unsafe extern "C" fn(*mut uint32_t, libc::c_int) -> (),
>;
pub type VP8LBundleColorMapFunc = Option::<
    unsafe extern "C" fn(*const uint8_t, libc::c_int, libc::c_int, *mut uint32_t) -> (),
>;
pub type VP8LFastLog2SlowFunc = Option::<
    unsafe extern "C" fn(uint32_t) -> libc::c_float,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
pub type vp8l_atype_t = uint64_t;
pub type vp8l_wtype_t = uint32_t;
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
pub type VP8LEncoderARGBContent = libc::c_uint;
pub const kEncoderPalette: VP8LEncoderARGBContent = 3;
pub const kEncoderNearLossless: VP8LEncoderARGBContent = 2;
pub const kEncoderARGB: VP8LEncoderARGBContent = 1;
pub const kEncoderNone: VP8LEncoderARGBContent = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LEncoder {
    pub config_: *const WebPConfig,
    pub pic_: *const WebPPicture,
    pub argb_: *mut uint32_t,
    pub argb_content_: VP8LEncoderARGBContent,
    pub argb_scratch_: *mut uint32_t,
    pub transform_data_: *mut uint32_t,
    pub transform_mem_: *mut uint32_t,
    pub transform_mem_size_: size_t,
    pub current_width_: libc::c_int,
    pub histo_bits_: libc::c_int,
    pub transform_bits_: libc::c_int,
    pub cache_bits_: libc::c_int,
    pub use_cross_color_: libc::c_int,
    pub use_subtract_green_: libc::c_int,
    pub use_predict_: libc::c_int,
    pub use_palette_: libc::c_int,
    pub palette_size_: libc::c_int,
    pub palette_: [uint32_t; 256],
    pub palette_sorted_: [uint32_t; 256],
    pub refs_: [VP8LBackwardRefs; 4],
    pub hash_chain_: VP8LHashChain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StreamEncodeContext {
    pub config_: *const WebPConfig,
    pub picture_: *const WebPPicture,
    pub bw_: *mut VP8LBitWriter,
    pub enc_: *mut VP8LEncoder,
    pub crunch_configs_: [CrunchConfig; 14],
    pub num_crunch_configs_: libc::c_int,
    pub red_and_blue_always_zero_: libc::c_int,
    pub stats_: *mut WebPAuxStats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrunchConfig {
    pub entropy_idx_: libc::c_int,
    pub palette_sorting_type_: PaletteSorting,
    pub sub_configs_: [CrunchSubConfig; 2],
    pub sub_configs_size_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CrunchSubConfig {
    pub lz77_: libc::c_int,
    pub do_no_cache_: libc::c_int,
}
pub type PaletteSorting = libc::c_uint;
pub const kPaletteSortingNum: PaletteSorting = 4;
pub const kUnusedPalette: PaletteSorting = 3;
pub const kModifiedZeng: PaletteSorting = 2;
pub const kMinimizeDelta: PaletteSorting = 1;
pub const kSortedDefault: PaletteSorting = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeCode {
    pub num_symbols: libc::c_int,
    pub code_lengths: *mut uint8_t,
    pub codes: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_: uint32_t,
    pub value_: libc::c_int,
    pub pool_index_left_: libc::c_int,
    pub pool_index_right_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeToken {
    pub code: uint8_t,
    pub extra_bits: uint8_t,
}
pub const kPaletteAndSpatial: EntropyIx = 5;
pub const kSpatialSubGreen: EntropyIx = 3;
pub const kSpatial: EntropyIx = 1;
pub const kSubGreen: EntropyIx = 2;
pub const kPalette: EntropyIx = 4;
pub type EntropyIx = libc::c_uint;
pub const kNumEntropyIx: EntropyIx = 6;
pub const kDirect: EntropyIx = 0;
pub const kHistoBlue: C2RustUnnamed_0 = 6;
pub const kHistoRed: C2RustUnnamed_0 = 4;
pub const kHistoBluePredSubGreen: C2RustUnnamed_0 = 11;
pub const kHistoRedPredSubGreen: C2RustUnnamed_0 = 9;
pub const kHistoBlueSubGreen: C2RustUnnamed_0 = 10;
pub const kHistoRedSubGreen: C2RustUnnamed_0 = 8;
pub const kHistoBluePred: C2RustUnnamed_0 = 7;
pub const kHistoRedPred: C2RustUnnamed_0 = 5;
pub const kHistoPalette: C2RustUnnamed_0 = 12;
pub const kHistoGreenPred: C2RustUnnamed_0 = 3;
pub const kHistoAlphaPred: C2RustUnnamed_0 = 1;
pub const kHistoGreen: C2RustUnnamed_0 = 2;
pub const kHistoAlpha: C2RustUnnamed_0 = 0;
pub const kHistoTotal: C2RustUnnamed_0 = 13;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn WebPPictureInit(mut picture: *mut WebPPicture) -> libc::c_int {
    return WebPPictureInitInternal(picture, 0x20f as libc::c_int);
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
unsafe extern "C" fn PutLE16(data: *mut uint8_t, mut val: libc::c_int) {
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (val >> 0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (val >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE32(data: *mut uint8_t, mut val: uint32_t) {
    PutLE16(data, (val & 0xffff as libc::c_int as libc::c_uint) as libc::c_int);
    PutLE16(
        data.offset(2 as libc::c_int as isize),
        (val >> 16 as libc::c_int) as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> libc::c_int {
    return 31 as libc::c_int ^ n.leading_zeros() as i32;
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
unsafe extern "C" fn VP8LFastLog2(mut v: uint32_t) -> libc::c_float {
    return if v < 256 as libc::c_int as libc::c_uint {
        kLog2Table[v as usize]
    } else {
        VP8LFastLog2Slow.expect("non-null function pointer")(v)
    };
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
#[inline]
unsafe extern "C" fn VP8LSubPixels(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let alpha_and_green: uint32_t = (0xff00ff as libc::c_uint)
        .wrapping_add(a & 0xff00ff00 as libc::c_uint)
        .wrapping_sub(b & 0xff00ff00 as libc::c_uint);
    let red_and_blue: uint32_t = (0xff00ff00 as libc::c_uint)
        .wrapping_add(a & 0xff00ff as libc::c_uint)
        .wrapping_sub(b & 0xff00ff as libc::c_uint);
    return alpha_and_green & 0xff00ff00 as libc::c_uint
        | red_and_blue & 0xff00ff as libc::c_uint;
}
#[inline]
unsafe extern "C" fn VP8LBitWriterNumBytes(bw: *const VP8LBitWriter) -> size_t {
    return (((*bw).cur_).offset_from((*bw).buf_) as libc::c_long
        + ((*bw).used_ + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_long)
        as size_t;
}
#[inline]
unsafe extern "C" fn VP8LPutBits(
    bw: *mut VP8LBitWriter,
    mut bits: uint32_t,
    mut n_bits: libc::c_int,
) {
    if ::core::mem::size_of::<vp8l_wtype_t>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong
    {
        if n_bits > 0 as libc::c_int {
            if (*bw).used_ >= 32 as libc::c_int {
                VP8LPutBitsFlushBits(bw);
            }
            (*bw).bits_ |= (bits as vp8l_atype_t) << (*bw).used_;
            (*bw).used_ += n_bits;
        }
    } else {
        VP8LPutBitsInternal(bw, bits, n_bits);
    };
}
unsafe extern "C" fn AddSingleSubGreen(
    mut p: uint32_t,
    r: *mut uint32_t,
    b: *mut uint32_t,
) {
    let green: libc::c_int = p as libc::c_int >> 8 as libc::c_int;
    let ref mut fresh0 = *r
        .offset(
            ((p as libc::c_int >> 16 as libc::c_int) - green & 0xff as libc::c_int)
                as isize,
        );
    *fresh0 = (*fresh0).wrapping_add(1);
    *fresh0;
    let ref mut fresh1 = *b
        .offset(
            ((p as libc::c_int >> 0 as libc::c_int) - green & 0xff as libc::c_int)
                as isize,
        );
    *fresh1 = (*fresh1).wrapping_add(1);
    *fresh1;
}
unsafe extern "C" fn AddSingle(
    mut p: uint32_t,
    a: *mut uint32_t,
    r: *mut uint32_t,
    g: *mut uint32_t,
    b: *mut uint32_t,
) {
    let ref mut fresh2 = *a
        .offset((p >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as isize);
    *fresh2 = (*fresh2).wrapping_add(1);
    *fresh2;
    let ref mut fresh3 = *r
        .offset((p >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as isize);
    *fresh3 = (*fresh3).wrapping_add(1);
    *fresh3;
    let ref mut fresh4 = *g
        .offset((p >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as isize);
    *fresh4 = (*fresh4).wrapping_add(1);
    *fresh4;
    let ref mut fresh5 = *b
        .offset((p >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as isize);
    *fresh5 = (*fresh5).wrapping_add(1);
    *fresh5;
}
#[inline]
unsafe extern "C" fn HashPix(mut pix: uint32_t) -> uint32_t {
    return ((((pix as uint64_t).wrapping_add((pix >> 19 as libc::c_int) as libc::c_ulong)
        as libc::c_ulonglong)
        .wrapping_mul(0x39c5fba7 as libc::c_ulonglong)
        & 0xffffffff as libc::c_uint as libc::c_ulonglong) >> 24 as libc::c_int)
        as uint32_t;
}
unsafe extern "C" fn AnalyzeEntropy(
    mut argb: *const uint32_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut argb_stride: libc::c_int,
    mut use_palette: libc::c_int,
    mut palette_size: libc::c_int,
    mut transform_bits: libc::c_int,
    min_entropy_ix: *mut EntropyIx,
    red_and_blue_always_zero: *mut libc::c_int,
) -> libc::c_int {
    let mut histo: *mut uint32_t = 0 as *mut uint32_t;
    if use_palette != 0 && palette_size <= 16 as libc::c_int {
        *min_entropy_ix = kPalette;
        *red_and_blue_always_zero = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    histo = WebPSafeCalloc(
        kHistoTotal as libc::c_int as uint64_t,
        (::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_mul(256 as libc::c_int as libc::c_ulong),
    ) as *mut uint32_t;
    if !histo.is_null() {
        let mut i: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut prev_row: *const uint32_t = 0 as *const uint32_t;
        let mut curr_row: *const uint32_t = argb;
        let mut pix_prev: uint32_t = *argb.offset(0 as libc::c_int as isize);
        y = 0 as libc::c_int;
        while y < height {
            x = 0 as libc::c_int;
            while x < width {
                let pix: uint32_t = *curr_row.offset(x as isize);
                let pix_diff: uint32_t = VP8LSubPixels(pix, pix_prev);
                pix_prev = pix;
                if !(pix_diff == 0 as libc::c_int as libc::c_uint
                    || !prev_row.is_null() && pix == *prev_row.offset(x as isize))
                {
                    AddSingle(
                        pix,
                        &mut *histo
                            .offset(
                                (kHistoAlpha as libc::c_int * 256 as libc::c_int) as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoRed as libc::c_int * 256 as libc::c_int) as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoGreen as libc::c_int * 256 as libc::c_int) as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoBlue as libc::c_int * 256 as libc::c_int) as isize,
                            ),
                    );
                    AddSingle(
                        pix_diff,
                        &mut *histo
                            .offset(
                                (kHistoAlphaPred as libc::c_int * 256 as libc::c_int)
                                    as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoRedPred as libc::c_int * 256 as libc::c_int) as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoGreenPred as libc::c_int * 256 as libc::c_int)
                                    as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoBluePred as libc::c_int * 256 as libc::c_int)
                                    as isize,
                            ),
                    );
                    AddSingleSubGreen(
                        pix,
                        &mut *histo
                            .offset(
                                (kHistoRedSubGreen as libc::c_int * 256 as libc::c_int)
                                    as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoBlueSubGreen as libc::c_int * 256 as libc::c_int)
                                    as isize,
                            ),
                    );
                    AddSingleSubGreen(
                        pix_diff,
                        &mut *histo
                            .offset(
                                (kHistoRedPredSubGreen as libc::c_int * 256 as libc::c_int)
                                    as isize,
                            ),
                        &mut *histo
                            .offset(
                                (kHistoBluePredSubGreen as libc::c_int * 256 as libc::c_int)
                                    as isize,
                            ),
                    );
                    let hash: uint32_t = HashPix(pix);
                    let ref mut fresh6 = *histo
                        .offset(
                            ((kHistoPalette as libc::c_int * 256 as libc::c_int)
                                as libc::c_uint)
                                .wrapping_add(hash) as isize,
                        );
                    *fresh6 = (*fresh6).wrapping_add(1);
                    *fresh6;
                }
                x += 1;
                x;
            }
            prev_row = curr_row;
            curr_row = curr_row.offset(argb_stride as isize);
            y += 1;
            y;
        }
        let mut entropy_comp: [libc::c_float; 13] = [0.; 13];
        let mut entropy: [libc::c_float; 6] = [0.; 6];
        let mut k: libc::c_int = 0;
        let mut last_mode_to_analyze: libc::c_int = if use_palette != 0 {
            kPalette as libc::c_int
        } else {
            kSpatialSubGreen as libc::c_int
        };
        let mut j: libc::c_int = 0;
        let ref mut fresh7 = *histo
            .offset(
                (kHistoRedPredSubGreen as libc::c_int * 256 as libc::c_int) as isize,
            );
        *fresh7 = (*fresh7).wrapping_add(1);
        *fresh7;
        let ref mut fresh8 = *histo
            .offset(
                (kHistoBluePredSubGreen as libc::c_int * 256 as libc::c_int) as isize,
            );
        *fresh8 = (*fresh8).wrapping_add(1);
        *fresh8;
        let ref mut fresh9 = *histo
            .offset((kHistoRedPred as libc::c_int * 256 as libc::c_int) as isize);
        *fresh9 = (*fresh9).wrapping_add(1);
        *fresh9;
        let ref mut fresh10 = *histo
            .offset((kHistoGreenPred as libc::c_int * 256 as libc::c_int) as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        *fresh10;
        let ref mut fresh11 = *histo
            .offset((kHistoBluePred as libc::c_int * 256 as libc::c_int) as isize);
        *fresh11 = (*fresh11).wrapping_add(1);
        *fresh11;
        let ref mut fresh12 = *histo
            .offset((kHistoAlphaPred as libc::c_int * 256 as libc::c_int) as isize);
        *fresh12 = (*fresh12).wrapping_add(1);
        *fresh12;
        j = 0 as libc::c_int;
        while j < kHistoTotal as libc::c_int {
            entropy_comp[j
                as usize] = VP8LBitsEntropy(
                &mut *histo.offset((j * 256 as libc::c_int) as isize),
                256 as libc::c_int,
            );
            j += 1;
            j;
        }
        entropy[kDirect as libc::c_int
            as usize] = entropy_comp[kHistoAlpha as libc::c_int as usize]
            + entropy_comp[kHistoRed as libc::c_int as usize]
            + entropy_comp[kHistoGreen as libc::c_int as usize]
            + entropy_comp[kHistoBlue as libc::c_int as usize];
        entropy[kSpatial as libc::c_int
            as usize] = entropy_comp[kHistoAlphaPred as libc::c_int as usize]
            + entropy_comp[kHistoRedPred as libc::c_int as usize]
            + entropy_comp[kHistoGreenPred as libc::c_int as usize]
            + entropy_comp[kHistoBluePred as libc::c_int as usize];
        entropy[kSubGreen as libc::c_int
            as usize] = entropy_comp[kHistoAlpha as libc::c_int as usize]
            + entropy_comp[kHistoRedSubGreen as libc::c_int as usize]
            + entropy_comp[kHistoGreen as libc::c_int as usize]
            + entropy_comp[kHistoBlueSubGreen as libc::c_int as usize];
        entropy[kSpatialSubGreen as libc::c_int
            as usize] = entropy_comp[kHistoAlphaPred as libc::c_int as usize]
            + entropy_comp[kHistoRedPredSubGreen as libc::c_int as usize]
            + entropy_comp[kHistoGreenPred as libc::c_int as usize]
            + entropy_comp[kHistoBluePredSubGreen as libc::c_int as usize];
        entropy[kPalette as libc::c_int
            as usize] = entropy_comp[kHistoPalette as libc::c_int as usize];
        entropy[kSpatial as libc::c_int as usize]
            += (VP8LSubSampleSize(width as uint32_t, transform_bits as uint32_t))
                .wrapping_mul(
                    VP8LSubSampleSize(height as uint32_t, transform_bits as uint32_t),
                ) as libc::c_float * VP8LFastLog2(14 as libc::c_int as uint32_t);
        entropy[kSpatialSubGreen as libc::c_int as usize]
            += (VP8LSubSampleSize(width as uint32_t, transform_bits as uint32_t))
                .wrapping_mul(
                    VP8LSubSampleSize(height as uint32_t, transform_bits as uint32_t),
                ) as libc::c_float * VP8LFastLog2(24 as libc::c_int as uint32_t);
        entropy[kPalette as libc::c_int as usize]
            += (palette_size * 8 as libc::c_int) as libc::c_float;
        *min_entropy_ix = kDirect;
        k = kDirect as libc::c_int + 1 as libc::c_int;
        while k <= last_mode_to_analyze {
            if entropy[*min_entropy_ix as usize] > entropy[k as usize] {
                *min_entropy_ix = k as EntropyIx;
            }
            k += 1;
            k;
        }
        *red_and_blue_always_zero = 1 as libc::c_int;
        static mut kHistoPairs: [[uint8_t; 2]; 5] = [
            [kHistoRed as libc::c_int as uint8_t, kHistoBlue as libc::c_int as uint8_t],
            [
                kHistoRedPred as libc::c_int as uint8_t,
                kHistoBluePred as libc::c_int as uint8_t,
            ],
            [
                kHistoRedSubGreen as libc::c_int as uint8_t,
                kHistoBlueSubGreen as libc::c_int as uint8_t,
            ],
            [
                kHistoRedPredSubGreen as libc::c_int as uint8_t,
                kHistoBluePredSubGreen as libc::c_int as uint8_t,
            ],
            [kHistoRed as libc::c_int as uint8_t, kHistoBlue as libc::c_int as uint8_t],
        ];
        let red_histo: *const uint32_t = &mut *histo
            .offset(
                (256 as libc::c_int
                    * *(*kHistoPairs.as_ptr().offset(*min_entropy_ix as isize))
                        .as_ptr()
                        .offset(0 as libc::c_int as isize) as libc::c_int) as isize,
            ) as *mut uint32_t;
        let blue_histo: *const uint32_t = &mut *histo
            .offset(
                (256 as libc::c_int
                    * *(*kHistoPairs.as_ptr().offset(*min_entropy_ix as isize))
                        .as_ptr()
                        .offset(1 as libc::c_int as isize) as libc::c_int) as isize,
            ) as *mut uint32_t;
        i = 1 as libc::c_int;
        while i < 256 as libc::c_int {
            if *red_histo.offset(i as isize) | *blue_histo.offset(i as isize)
                != 0 as libc::c_int as libc::c_uint
            {
                *red_and_blue_always_zero = 0 as libc::c_int;
                break;
            } else {
                i += 1;
                i;
            }
        }
        WebPSafeFree(histo as *mut libc::c_void);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn GetHistoBits(
    mut method: libc::c_int,
    mut use_palette: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut histo_bits: libc::c_int = (if use_palette != 0 {
        9 as libc::c_int
    } else {
        7 as libc::c_int
    }) - method;
    loop {
        let huff_image_size: libc::c_int = (VP8LSubSampleSize(
            width as uint32_t,
            histo_bits as uint32_t,
        ))
            .wrapping_mul(VP8LSubSampleSize(height as uint32_t, histo_bits as uint32_t))
            as libc::c_int;
        if huff_image_size <= 2600 as libc::c_int {
            break;
        }
        histo_bits += 1;
        histo_bits;
    }
    return if histo_bits < 2 as libc::c_int {
        2 as libc::c_int
    } else if histo_bits > 9 as libc::c_int {
        9 as libc::c_int
    } else {
        histo_bits
    };
}
unsafe extern "C" fn GetTransformBits(
    mut method: libc::c_int,
    mut histo_bits: libc::c_int,
) -> libc::c_int {
    let max_transform_bits: libc::c_int = if method < 4 as libc::c_int {
        6 as libc::c_int
    } else if method > 4 as libc::c_int {
        4 as libc::c_int
    } else {
        5 as libc::c_int
    };
    let res: libc::c_int = if histo_bits > max_transform_bits {
        max_transform_bits
    } else {
        histo_bits
    };
    return res;
}
unsafe extern "C" fn EncoderAnalyze(
    enc: *mut VP8LEncoder,
    mut crunch_configs: *mut CrunchConfig,
    crunch_configs_size: *mut libc::c_int,
    red_and_blue_always_zero: *mut libc::c_int,
) -> libc::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: libc::c_int = (*pic).width;
    let height: libc::c_int = (*pic).height;
    let config: *const WebPConfig = (*enc).config_;
    let method: libc::c_int = (*config).method;
    let low_effort: libc::c_int = ((*config).method == 0 as libc::c_int) as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut use_palette: libc::c_int = 0;
    let mut n_lz77s: libc::c_int = 0;
    let mut do_no_cache: libc::c_int = 0 as libc::c_int;
    (*enc).palette_size_ = GetColorPalette(pic, ((*enc).palette_sorted_).as_mut_ptr());
    use_palette = ((*enc).palette_size_ <= 256 as libc::c_int) as libc::c_int;
    if use_palette == 0 {
        (*enc).palette_size_ = 0 as libc::c_int;
    }
    (*enc).histo_bits_ = GetHistoBits(method, use_palette, (*pic).width, (*pic).height);
    (*enc).transform_bits_ = GetTransformBits(method, (*enc).histo_bits_);
    if low_effort != 0 {
        (*crunch_configs.offset(0 as libc::c_int as isize))
            .entropy_idx_ = if use_palette != 0 {
            kPalette as libc::c_int
        } else {
            kSpatialSubGreen as libc::c_int
        };
        (*crunch_configs.offset(0 as libc::c_int as isize))
            .palette_sorting_type_ = (if use_palette != 0 {
            kSortedDefault as libc::c_int
        } else {
            kUnusedPalette as libc::c_int
        }) as PaletteSorting;
        n_lz77s = 1 as libc::c_int;
        *crunch_configs_size = 1 as libc::c_int;
    } else {
        let mut min_entropy_ix: EntropyIx = kDirect;
        n_lz77s = if (*enc).palette_size_ > 0 as libc::c_int
            && (*enc).palette_size_ <= 16 as libc::c_int
        {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        if AnalyzeEntropy(
            (*pic).argb,
            width,
            height,
            (*pic).argb_stride,
            use_palette,
            (*enc).palette_size_,
            (*enc).transform_bits_,
            &mut min_entropy_ix,
            red_and_blue_always_zero,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if method == 6 as libc::c_int
            && (*config).quality == 100 as libc::c_int as libc::c_float
        {
            do_no_cache = 1 as libc::c_int;
            *crunch_configs_size = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < kNumEntropyIx as libc::c_int {
                if i != kPalette as libc::c_int && i != kPaletteAndSpatial as libc::c_int
                    || use_palette != 0
                {
                    if use_palette != 0
                        && (i == kPalette as libc::c_int
                            || i == kPaletteAndSpatial as libc::c_int)
                    {
                        let mut sorting_method: libc::c_int = 0;
                        sorting_method = 0 as libc::c_int;
                        while sorting_method < kPaletteSortingNum as libc::c_int {
                            let typed_sorting_method: PaletteSorting = sorting_method
                                as PaletteSorting;
                            if !(typed_sorting_method as libc::c_uint
                                == kUnusedPalette as libc::c_int as libc::c_uint
                                || typed_sorting_method as libc::c_uint
                                    == kSortedDefault as libc::c_int as libc::c_uint)
                            {
                                (*crunch_configs.offset(*crunch_configs_size as isize))
                                    .entropy_idx_ = i;
                                (*crunch_configs.offset(*crunch_configs_size as isize))
                                    .palette_sorting_type_ = typed_sorting_method;
                                *crunch_configs_size += 1;
                                *crunch_configs_size;
                            }
                            sorting_method += 1;
                            sorting_method;
                        }
                    } else {
                        (*crunch_configs.offset(*crunch_configs_size as isize))
                            .entropy_idx_ = i;
                        (*crunch_configs.offset(*crunch_configs_size as isize))
                            .palette_sorting_type_ = kUnusedPalette;
                        *crunch_configs_size += 1;
                        *crunch_configs_size;
                    }
                }
                i += 1;
                i;
            }
        } else {
            *crunch_configs_size = 1 as libc::c_int;
            (*crunch_configs.offset(0 as libc::c_int as isize))
                .entropy_idx_ = min_entropy_ix as libc::c_int;
            (*crunch_configs.offset(0 as libc::c_int as isize))
                .palette_sorting_type_ = (if use_palette != 0 {
                kMinimizeDelta as libc::c_int
            } else {
                kUnusedPalette as libc::c_int
            }) as PaletteSorting;
            if (*config).quality >= 75 as libc::c_int as libc::c_float
                && method == 5 as libc::c_int
            {
                do_no_cache = 1 as libc::c_int;
                if min_entropy_ix as libc::c_uint
                    == kPalette as libc::c_int as libc::c_uint
                {
                    *crunch_configs_size = 2 as libc::c_int;
                    (*crunch_configs.offset(1 as libc::c_int as isize))
                        .entropy_idx_ = kPaletteAndSpatial as libc::c_int;
                    (*crunch_configs.offset(1 as libc::c_int as isize))
                        .palette_sorting_type_ = kMinimizeDelta;
                }
            }
        }
    }
    i = 0 as libc::c_int;
    while i < *crunch_configs_size {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < n_lz77s {
            (*crunch_configs.offset(i as isize))
                .sub_configs_[j as usize]
                .lz77_ = if j == 0 as libc::c_int {
                kLZ77Standard as libc::c_int | kLZ77RLE as libc::c_int
            } else {
                kLZ77Box as libc::c_int
            };
            (*crunch_configs.offset(i as isize))
                .sub_configs_[j as usize]
                .do_no_cache_ = do_no_cache;
            j += 1;
            j;
        }
        (*crunch_configs.offset(i as isize)).sub_configs_size_ = n_lz77s;
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn EncoderInit(enc: *mut VP8LEncoder) -> libc::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: libc::c_int = (*pic).width;
    let height: libc::c_int = (*pic).height;
    let pix_cnt: libc::c_int = width * height;
    let refs_block_size: libc::c_int = (pix_cnt - 1 as libc::c_int) / 16 as libc::c_int
        + 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    if VP8LHashChainInit(&mut (*enc).hash_chain_, pix_cnt) == 0 {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        VP8LBackwardRefsInit(
            &mut *((*enc).refs_).as_mut_ptr().offset(i as isize),
            refs_block_size,
        );
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn GetHuffBitLengthsAndCodes(
    histogram_image: *const VP8LHistogramSet,
    huffman_codes: *mut HuffmanTreeCode,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut total_length_size: uint64_t = 0 as libc::c_int as uint64_t;
    let mut mem_buf: *mut uint8_t = 0 as *mut uint8_t;
    let histogram_image_size: libc::c_int = (*histogram_image).size;
    let mut max_num_symbols: libc::c_int = 0 as libc::c_int;
    let mut buf_rle: *mut uint8_t = 0 as *mut uint8_t;
    let mut huff_tree: *mut HuffmanTree = 0 as *mut HuffmanTree;
    i = 0 as libc::c_int;
    while i < histogram_image_size {
        let histo: *const VP8LHistogram = *((*histogram_image).histograms)
            .offset(i as isize);
        let codes: *mut HuffmanTreeCode = &mut *huffman_codes
            .offset((5 as libc::c_int * i) as isize) as *mut HuffmanTreeCode;
        k = 0 as libc::c_int;
        while k < 5 as libc::c_int {
            let num_symbols: libc::c_int = if k == 0 as libc::c_int {
                VP8LHistogramNumCodes((*histo).palette_code_bits_)
            } else if k == 4 as libc::c_int {
                40 as libc::c_int
            } else {
                256 as libc::c_int
            };
            (*codes.offset(k as isize)).num_symbols = num_symbols;
            total_length_size = (total_length_size as libc::c_ulong)
                .wrapping_add(num_symbols as libc::c_ulong) as uint64_t as uint64_t;
            k += 1;
            k;
        }
        i += 1;
        i;
    }
    let mut codes_0: *mut uint16_t = 0 as *mut uint16_t;
    let mut lengths: *mut uint8_t = 0 as *mut uint8_t;
    mem_buf = WebPSafeCalloc(
        total_length_size,
        (::core::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    ) as *mut uint8_t;
    if !mem_buf.is_null() {
        codes_0 = mem_buf as *mut uint16_t;
        lengths = &mut *codes_0.offset(total_length_size as isize) as *mut uint16_t
            as *mut uint8_t;
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int * histogram_image_size {
            let bit_length: libc::c_int = (*huffman_codes.offset(i as isize))
                .num_symbols;
            let ref mut fresh13 = (*huffman_codes.offset(i as isize)).codes;
            *fresh13 = codes_0;
            let ref mut fresh14 = (*huffman_codes.offset(i as isize)).code_lengths;
            *fresh14 = lengths;
            codes_0 = codes_0.offset(bit_length as isize);
            lengths = lengths.offset(bit_length as isize);
            if max_num_symbols < bit_length {
                max_num_symbols = bit_length;
            }
            i += 1;
            i;
        }
        buf_rle = WebPSafeMalloc(
            1 as libc::c_ulonglong as uint64_t,
            max_num_symbols as size_t,
        ) as *mut uint8_t;
        huff_tree = WebPSafeMalloc(
            (3 as libc::c_ulonglong).wrapping_mul(max_num_symbols as libc::c_ulonglong)
                as uint64_t,
            ::core::mem::size_of::<HuffmanTree>() as libc::c_ulong,
        ) as *mut HuffmanTree;
        if !(buf_rle.is_null() || huff_tree.is_null()) {
            i = 0 as libc::c_int;
            while i < histogram_image_size {
                let codes_1: *mut HuffmanTreeCode = &mut *huffman_codes
                    .offset((5 as libc::c_int * i) as isize) as *mut HuffmanTreeCode;
                let histo_0: *mut VP8LHistogram = *((*histogram_image).histograms)
                    .offset(i as isize);
                VP8LCreateHuffmanTree(
                    (*histo_0).literal_,
                    15 as libc::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(0 as libc::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    ((*histo_0).red_).as_mut_ptr(),
                    15 as libc::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(1 as libc::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    ((*histo_0).blue_).as_mut_ptr(),
                    15 as libc::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(2 as libc::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    ((*histo_0).alpha_).as_mut_ptr(),
                    15 as libc::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(3 as libc::c_int as isize),
                );
                VP8LCreateHuffmanTree(
                    ((*histo_0).distance_).as_mut_ptr(),
                    15 as libc::c_int,
                    buf_rle,
                    huff_tree,
                    codes_1.offset(4 as libc::c_int as isize),
                );
                i += 1;
                i;
            }
            ok = 1 as libc::c_int;
        }
    }
    WebPSafeFree(huff_tree as *mut libc::c_void);
    WebPSafeFree(buf_rle as *mut libc::c_void);
    if ok == 0 {
        WebPSafeFree(mem_buf as *mut libc::c_void);
        memset(
            huffman_codes as *mut libc::c_void,
            0 as libc::c_int,
            ((5 as libc::c_int * histogram_image_size) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<HuffmanTreeCode>() as libc::c_ulong),
        );
    }
    return ok;
}
unsafe extern "C" fn StoreHuffmanTreeOfHuffmanTreeToBitMask(
    bw: *mut VP8LBitWriter,
    mut code_length_bitdepth: *const uint8_t,
) {
    static mut kStorageOrder: [uint8_t; 19] = [
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
    let mut i: libc::c_int = 0;
    let mut codes_to_store: libc::c_int = 19 as libc::c_int;
    while codes_to_store > 4 as libc::c_int {
        if *code_length_bitdepth
            .offset(kStorageOrder[(codes_to_store - 1 as libc::c_int) as usize] as isize)
            as libc::c_int != 0 as libc::c_int
        {
            break;
        }
        codes_to_store -= 1;
        codes_to_store;
    }
    VP8LPutBits(bw, (codes_to_store - 4 as libc::c_int) as uint32_t, 4 as libc::c_int);
    i = 0 as libc::c_int;
    while i < codes_to_store {
        VP8LPutBits(
            bw,
            *code_length_bitdepth.offset(kStorageOrder[i as usize] as isize) as uint32_t,
            3 as libc::c_int,
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn ClearHuffmanTreeIfOnlyOneSymbol(
    huffman_code: *mut HuffmanTreeCode,
) {
    let mut k: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < (*huffman_code).num_symbols {
        if *((*huffman_code).code_lengths).offset(k as isize) as libc::c_int
            != 0 as libc::c_int
        {
            count += 1;
            count;
            if count > 1 as libc::c_int {
                return;
            }
        }
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k < (*huffman_code).num_symbols {
        *((*huffman_code).code_lengths).offset(k as isize) = 0 as libc::c_int as uint8_t;
        *((*huffman_code).codes).offset(k as isize) = 0 as libc::c_int as uint16_t;
        k += 1;
        k;
    }
}
unsafe extern "C" fn StoreHuffmanTreeToBitMask(
    bw: *mut VP8LBitWriter,
    tokens: *const HuffmanTreeToken,
    num_tokens: libc::c_int,
    huffman_code: *const HuffmanTreeCode,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_tokens {
        let ix: libc::c_int = (*tokens.offset(i as isize)).code as libc::c_int;
        let extra_bits: libc::c_int = (*tokens.offset(i as isize)).extra_bits
            as libc::c_int;
        VP8LPutBits(
            bw,
            *((*huffman_code).codes).offset(ix as isize) as uint32_t,
            *((*huffman_code).code_lengths).offset(ix as isize) as libc::c_int,
        );
        match ix {
            16 => {
                VP8LPutBits(bw, extra_bits as uint32_t, 2 as libc::c_int);
            }
            17 => {
                VP8LPutBits(bw, extra_bits as uint32_t, 3 as libc::c_int);
            }
            18 => {
                VP8LPutBits(bw, extra_bits as uint32_t, 7 as libc::c_int);
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn StoreFullHuffmanCode(
    bw: *mut VP8LBitWriter,
    huff_tree: *mut HuffmanTree,
    tokens: *mut HuffmanTreeToken,
    tree: *const HuffmanTreeCode,
) {
    let mut code_length_bitdepth: [uint8_t; 19] = [
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
    ];
    let mut code_length_bitdepth_symbols: [uint16_t; 19] = [
        0 as libc::c_int as uint16_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let max_tokens: libc::c_int = (*tree).num_symbols;
    let mut num_tokens: libc::c_int = 0;
    let mut huffman_code: HuffmanTreeCode = HuffmanTreeCode {
        num_symbols: 0,
        code_lengths: 0 as *mut uint8_t,
        codes: 0 as *mut uint16_t,
    };
    huffman_code.num_symbols = 19 as libc::c_int;
    huffman_code.code_lengths = code_length_bitdepth.as_mut_ptr();
    huffman_code.codes = code_length_bitdepth_symbols.as_mut_ptr();
    VP8LPutBits(bw, 0 as libc::c_int as uint32_t, 1 as libc::c_int);
    num_tokens = VP8LCreateCompressedHuffmanTree(tree, tokens, max_tokens);
    let mut histogram: [uint32_t; 19] = [
        0 as libc::c_int as uint32_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut buf_rle: [uint8_t; 19] = [
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
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_tokens {
        histogram[(*tokens.offset(i as isize)).code
            as usize] = (histogram[(*tokens.offset(i as isize)).code as usize])
            .wrapping_add(1);
        histogram[(*tokens.offset(i as isize)).code as usize];
        i += 1;
        i;
    }
    VP8LCreateHuffmanTree(
        histogram.as_mut_ptr(),
        7 as libc::c_int,
        buf_rle.as_mut_ptr(),
        huff_tree,
        &mut huffman_code,
    );
    StoreHuffmanTreeOfHuffmanTreeToBitMask(bw, code_length_bitdepth.as_mut_ptr());
    ClearHuffmanTreeIfOnlyOneSymbol(&mut huffman_code);
    let mut trailing_zero_bits: libc::c_int = 0 as libc::c_int;
    let mut trimmed_length: libc::c_int = num_tokens;
    let mut write_trimmed_length: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut i_0: libc::c_int = num_tokens;
    loop {
        let fresh15 = i_0;
        i_0 = i_0 - 1;
        if !(fresh15 > 0 as libc::c_int) {
            break;
        }
        let ix: libc::c_int = (*tokens.offset(i_0 as isize)).code as libc::c_int;
        if !(ix == 0 as libc::c_int || ix == 17 as libc::c_int
            || ix == 18 as libc::c_int)
        {
            break;
        }
        trimmed_length -= 1;
        trimmed_length;
        trailing_zero_bits += code_length_bitdepth[ix as usize] as libc::c_int;
        if ix == 17 as libc::c_int {
            trailing_zero_bits += 3 as libc::c_int;
        } else if ix == 18 as libc::c_int {
            trailing_zero_bits += 7 as libc::c_int;
        }
    }
    write_trimmed_length = (trimmed_length > 1 as libc::c_int
        && trailing_zero_bits > 12 as libc::c_int) as libc::c_int;
    length = if write_trimmed_length != 0 { trimmed_length } else { num_tokens };
    VP8LPutBits(bw, write_trimmed_length as uint32_t, 1 as libc::c_int);
    if write_trimmed_length != 0 {
        if trimmed_length == 2 as libc::c_int {
            VP8LPutBits(
                bw,
                0 as libc::c_int as uint32_t,
                3 as libc::c_int + 2 as libc::c_int,
            );
        } else {
            let nbits: libc::c_int = BitsLog2Floor(
                (trimmed_length - 2 as libc::c_int) as uint32_t,
            );
            let nbitpairs: libc::c_int = nbits / 2 as libc::c_int + 1 as libc::c_int;
            VP8LPutBits(
                bw,
                (nbitpairs - 1 as libc::c_int) as uint32_t,
                3 as libc::c_int,
            );
            VP8LPutBits(
                bw,
                (trimmed_length - 2 as libc::c_int) as uint32_t,
                nbitpairs * 2 as libc::c_int,
            );
        }
    }
    StoreHuffmanTreeToBitMask(bw, tokens, length, &mut huffman_code);
}
unsafe extern "C" fn StoreHuffmanCode(
    bw: *mut VP8LBitWriter,
    huff_tree: *mut HuffmanTree,
    tokens: *mut HuffmanTreeToken,
    huffman_code: *const HuffmanTreeCode,
) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut symbols: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
    let kMaxBits: libc::c_int = 8 as libc::c_int;
    let kMaxSymbol: libc::c_int = (1 as libc::c_int) << kMaxBits;
    i = 0 as libc::c_int;
    while i < (*huffman_code).num_symbols && count < 3 as libc::c_int {
        if *((*huffman_code).code_lengths).offset(i as isize) as libc::c_int
            != 0 as libc::c_int
        {
            if count < 2 as libc::c_int {
                symbols[count as usize] = i;
            }
            count += 1;
            count;
        }
        i += 1;
        i;
    }
    if count == 0 as libc::c_int {
        VP8LPutBits(bw, 0x1 as libc::c_int as uint32_t, 4 as libc::c_int);
    } else if count <= 2 as libc::c_int
        && symbols[0 as libc::c_int as usize] < kMaxSymbol
        && symbols[1 as libc::c_int as usize] < kMaxSymbol
    {
        VP8LPutBits(bw, 1 as libc::c_int as uint32_t, 1 as libc::c_int);
        VP8LPutBits(bw, (count - 1 as libc::c_int) as uint32_t, 1 as libc::c_int);
        if symbols[0 as libc::c_int as usize] <= 1 as libc::c_int {
            VP8LPutBits(bw, 0 as libc::c_int as uint32_t, 1 as libc::c_int);
            VP8LPutBits(
                bw,
                symbols[0 as libc::c_int as usize] as uint32_t,
                1 as libc::c_int,
            );
        } else {
            VP8LPutBits(bw, 1 as libc::c_int as uint32_t, 1 as libc::c_int);
            VP8LPutBits(
                bw,
                symbols[0 as libc::c_int as usize] as uint32_t,
                8 as libc::c_int,
            );
        }
        if count == 2 as libc::c_int {
            VP8LPutBits(
                bw,
                symbols[1 as libc::c_int as usize] as uint32_t,
                8 as libc::c_int,
            );
        }
    } else {
        StoreFullHuffmanCode(bw, huff_tree, tokens, huffman_code);
    };
}
#[inline]
unsafe extern "C" fn WriteHuffmanCode(
    bw: *mut VP8LBitWriter,
    code: *const HuffmanTreeCode,
    mut code_index: libc::c_int,
) {
    let depth: libc::c_int = *((*code).code_lengths).offset(code_index as isize)
        as libc::c_int;
    let symbol: libc::c_int = *((*code).codes).offset(code_index as isize)
        as libc::c_int;
    VP8LPutBits(bw, symbol as uint32_t, depth);
}
#[inline]
unsafe extern "C" fn WriteHuffmanCodeWithExtraBits(
    bw: *mut VP8LBitWriter,
    code: *const HuffmanTreeCode,
    mut code_index: libc::c_int,
    mut bits: libc::c_int,
    mut n_bits: libc::c_int,
) {
    let depth: libc::c_int = *((*code).code_lengths).offset(code_index as isize)
        as libc::c_int;
    let symbol: libc::c_int = *((*code).codes).offset(code_index as isize)
        as libc::c_int;
    VP8LPutBits(bw, (bits << depth | symbol) as uint32_t, depth + n_bits);
}
unsafe extern "C" fn StoreImageToBitMask(
    bw: *mut VP8LBitWriter,
    mut width: libc::c_int,
    mut histo_bits: libc::c_int,
    refs: *const VP8LBackwardRefs,
    mut histogram_symbols: *const uint16_t,
    huffman_codes: *const HuffmanTreeCode,
    pic: *const WebPPicture,
) -> libc::c_int {
    let histo_xsize: libc::c_int = (if histo_bits != 0 {
        VP8LSubSampleSize(width as uint32_t, histo_bits as uint32_t)
    } else {
        1 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    let tile_mask: libc::c_int = if histo_bits == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        -((1 as libc::c_int) << histo_bits)
    };
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut tile_x: libc::c_int = x & tile_mask;
    let mut tile_y: libc::c_int = y & tile_mask;
    let mut histogram_ix: libc::c_int = *histogram_symbols
        .offset(0 as libc::c_int as isize) as libc::c_int;
    let mut codes: *const HuffmanTreeCode = huffman_codes
        .offset((5 as libc::c_int * histogram_ix) as isize);
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    while VP8LRefsCursorOk(&mut c) != 0 {
        let v: *const PixOrCopy = c.cur_pos;
        if tile_x != x & tile_mask || tile_y != y & tile_mask {
            tile_x = x & tile_mask;
            tile_y = y & tile_mask;
            histogram_ix = *histogram_symbols
                .offset(((y >> histo_bits) * histo_xsize + (x >> histo_bits)) as isize)
                as libc::c_int;
            codes = huffman_codes.offset((5 as libc::c_int * histogram_ix) as isize);
        }
        if PixOrCopyIsLiteral(v) != 0 {
            static mut order: [uint8_t; 4] = [
                1 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
            ];
            let mut k: libc::c_int = 0;
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                let code: libc::c_int = PixOrCopyLiteral(
                    v,
                    order[k as usize] as libc::c_int,
                ) as libc::c_int;
                WriteHuffmanCode(bw, codes.offset(k as isize), code);
                k += 1;
                k;
            }
        } else if PixOrCopyIsCacheIdx(v) != 0 {
            let code_0: libc::c_int = PixOrCopyCacheIdx(v) as libc::c_int;
            let literal_ix: libc::c_int = 256 as libc::c_int + 24 as libc::c_int
                + code_0;
            WriteHuffmanCode(bw, codes, literal_ix);
        } else {
            let mut bits: libc::c_int = 0;
            let mut n_bits: libc::c_int = 0;
            let mut code_1: libc::c_int = 0;
            let distance: libc::c_int = PixOrCopyDistance(v) as libc::c_int;
            VP8LPrefixEncode(
                (*v).len as libc::c_int,
                &mut code_1,
                &mut n_bits,
                &mut bits,
            );
            WriteHuffmanCodeWithExtraBits(
                bw,
                codes,
                256 as libc::c_int + code_1,
                bits,
                n_bits,
            );
            VP8LPrefixEncode(distance, &mut code_1, &mut n_bits, &mut bits);
            WriteHuffmanCode(bw, codes.offset(4 as libc::c_int as isize), code_1);
            VP8LPutBits(bw, bits as uint32_t, n_bits);
        }
        x = (x as libc::c_uint).wrapping_add(PixOrCopyLength(v)) as libc::c_int
            as libc::c_int;
        while x >= width {
            x -= width;
            y += 1;
            y;
        }
        VP8LRefsCursorNext(&mut c);
    }
    if (*bw).error_ != 0 {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn EncodeImageNoHuffman(
    bw: *mut VP8LBitWriter,
    argb: *const uint32_t,
    hash_chain: *mut VP8LHashChain,
    refs_array: *mut VP8LBackwardRefs,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut quality: libc::c_int,
    mut low_effort: libc::c_int,
    pic: *const WebPPicture,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut max_tokens: libc::c_int = 0 as libc::c_int;
    let mut refs: *mut VP8LBackwardRefs = 0 as *mut VP8LBackwardRefs;
    let mut tokens: *mut HuffmanTreeToken = 0 as *mut HuffmanTreeToken;
    let mut huffman_codes: [HuffmanTreeCode; 5] = [
        {
            let mut init = HuffmanTreeCode {
                num_symbols: 0 as libc::c_int,
                code_lengths: 0 as *mut uint8_t,
                codes: 0 as *mut uint16_t,
            };
            init
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: 0 as *mut uint8_t,
            codes: 0 as *mut uint16_t,
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: 0 as *mut uint8_t,
            codes: 0 as *mut uint16_t,
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: 0 as *mut uint8_t,
            codes: 0 as *mut uint16_t,
        },
        HuffmanTreeCode {
            num_symbols: 0,
            code_lengths: 0 as *mut uint8_t,
            codes: 0 as *mut uint16_t,
        },
    ];
    let histogram_symbols: [uint16_t; 1] = [0 as libc::c_int as uint16_t];
    let mut cache_bits: libc::c_int = 0 as libc::c_int;
    let mut histogram_image: *mut VP8LHistogramSet = 0 as *mut VP8LHistogramSet;
    let huff_tree: *mut HuffmanTree = WebPSafeMalloc(
        (3 as libc::c_ulonglong).wrapping_mul(19 as libc::c_int as libc::c_ulonglong)
            as uint64_t,
        ::core::mem::size_of::<HuffmanTree>() as libc::c_ulong,
    ) as *mut HuffmanTree;
    if huff_tree.is_null() {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else if !(VP8LHashChainFill(
        hash_chain,
        quality,
        argb,
        width,
        height,
        low_effort,
        pic,
        percent_range / 2 as libc::c_int,
        percent,
    ) == 0)
    {
        if !(VP8LGetBackwardReferences(
            width,
            height,
            argb,
            quality,
            0 as libc::c_int,
            kLZ77Standard as libc::c_int | kLZ77RLE as libc::c_int,
            cache_bits,
            0 as libc::c_int,
            hash_chain,
            refs_array,
            &mut cache_bits,
            pic,
            percent_range - percent_range / 2 as libc::c_int,
            percent,
        ) == 0)
        {
            refs = &mut *refs_array.offset(0 as libc::c_int as isize)
                as *mut VP8LBackwardRefs;
            histogram_image = VP8LAllocateHistogramSet(1 as libc::c_int, cache_bits);
            if histogram_image.is_null() {
                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
            } else {
                VP8LHistogramSetClear(histogram_image);
                VP8LHistogramStoreRefs(
                    refs,
                    *((*histogram_image).histograms).offset(0 as libc::c_int as isize),
                );
                if GetHuffBitLengthsAndCodes(histogram_image, huffman_codes.as_mut_ptr())
                    == 0
                {
                    WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                } else {
                    VP8LPutBits(bw, 0 as libc::c_int as uint32_t, 1 as libc::c_int);
                    i = 0 as libc::c_int;
                    while i < 5 as libc::c_int {
                        let codes: *mut HuffmanTreeCode = &mut *huffman_codes
                            .as_mut_ptr()
                            .offset(i as isize) as *mut HuffmanTreeCode;
                        if max_tokens < (*codes).num_symbols {
                            max_tokens = (*codes).num_symbols;
                        }
                        i += 1;
                        i;
                    }
                    tokens = WebPSafeMalloc(
                        max_tokens as uint64_t,
                        ::core::mem::size_of::<HuffmanTreeToken>() as libc::c_ulong,
                    ) as *mut HuffmanTreeToken;
                    if tokens.is_null() {
                        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    } else {
                        i = 0 as libc::c_int;
                        while i < 5 as libc::c_int {
                            let codes_0: *mut HuffmanTreeCode = &mut *huffman_codes
                                .as_mut_ptr()
                                .offset(i as isize) as *mut HuffmanTreeCode;
                            StoreHuffmanCode(bw, huff_tree, tokens, codes_0);
                            ClearHuffmanTreeIfOnlyOneSymbol(codes_0);
                            i += 1;
                            i;
                        }
                        StoreImageToBitMask(
                            bw,
                            width,
                            0 as libc::c_int,
                            refs,
                            histogram_symbols.as_ptr(),
                            huffman_codes.as_mut_ptr(),
                            pic,
                        ) == 0;
                    }
                }
            }
        }
    }
    WebPSafeFree(tokens as *mut libc::c_void);
    WebPSafeFree(huff_tree as *mut libc::c_void);
    VP8LFreeHistogramSet(histogram_image);
    WebPSafeFree(huffman_codes[0 as libc::c_int as usize].codes as *mut libc::c_void);
    return ((*pic).error_code as libc::c_uint
        == VP8_ENC_OK as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn EncodeImageInternal(
    bw: *mut VP8LBitWriter,
    argb: *const uint32_t,
    hash_chain: *mut VP8LHashChain,
    mut refs_array: *mut VP8LBackwardRefs,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut quality: libc::c_int,
    mut low_effort: libc::c_int,
    config: *const CrunchConfig,
    mut cache_bits: *mut libc::c_int,
    mut histogram_bits: libc::c_int,
    mut init_byte_position: size_t,
    hdr_size: *mut libc::c_int,
    data_size: *mut libc::c_int,
    pic: *const WebPPicture,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let histogram_image_xysize: uint32_t = (VP8LSubSampleSize(
        width as uint32_t,
        histogram_bits as uint32_t,
    ))
        .wrapping_mul(VP8LSubSampleSize(height as uint32_t, histogram_bits as uint32_t));
    let mut remaining_percent: libc::c_int = percent_range;
    let mut percent_start: libc::c_int = *percent;
    let mut histogram_image: *mut VP8LHistogramSet = 0 as *mut VP8LHistogramSet;
    let mut tmp_histo: *mut VP8LHistogram = 0 as *mut VP8LHistogram;
    let mut histogram_image_size: libc::c_int = 0 as libc::c_int;
    let mut bit_array_size: size_t = 0 as libc::c_int as size_t;
    let huff_tree: *mut HuffmanTree = WebPSafeMalloc(
        (3 as libc::c_ulonglong).wrapping_mul(19 as libc::c_int as libc::c_ulonglong)
            as uint64_t,
        ::core::mem::size_of::<HuffmanTree>() as libc::c_ulong,
    ) as *mut HuffmanTree;
    let mut tokens: *mut HuffmanTreeToken = 0 as *mut HuffmanTreeToken;
    let mut huffman_codes: *mut HuffmanTreeCode = 0 as *mut HuffmanTreeCode;
    let histogram_symbols: *mut uint16_t = WebPSafeMalloc(
        histogram_image_xysize as uint64_t,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    ) as *mut uint16_t;
    let mut sub_configs_idx: libc::c_int = 0;
    let mut cache_bits_init: libc::c_int = 0;
    let mut write_histogram_image: libc::c_int = 0;
    let mut bw_init: VP8LBitWriter = *bw;
    let mut bw_best: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: 0 as *mut uint8_t,
        cur_: 0 as *mut uint8_t,
        end_: 0 as *mut uint8_t,
        error_: 0,
    };
    let mut hdr_size_tmp: libc::c_int = 0;
    let mut hash_chain_histogram: VP8LHashChain = VP8LHashChain {
        offset_length_: 0 as *mut uint32_t,
        size_: 0,
    };
    let mut bw_size_best: size_t = !(0 as libc::c_int as size_t);
    memset(
        &mut hash_chain_histogram as *mut VP8LHashChain as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8LHashChain>() as libc::c_ulong,
    );
    if VP8LBitWriterInit(&mut bw_best, 0 as libc::c_int as size_t) == 0 {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else if huff_tree.is_null() || histogram_symbols.is_null()
        || VP8LHashChainInit(
            &mut hash_chain_histogram,
            histogram_image_xysize as libc::c_int,
        ) == 0
    {
        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        percent_range = remaining_percent / 5 as libc::c_int;
        if !(VP8LHashChainFill(
            hash_chain,
            quality,
            argb,
            width,
            height,
            low_effort,
            pic,
            percent_range,
            percent,
        ) == 0)
        {
            percent_start += percent_range;
            remaining_percent -= percent_range;
            cache_bits_init = if *cache_bits == 0 as libc::c_int {
                10 as libc::c_int
            } else {
                *cache_bits
            };
            if ((*config).sub_configs_size_ > 1 as libc::c_int
                || (*config).sub_configs_[0 as libc::c_int as usize].do_no_cache_ != 0)
                && VP8LBitWriterClone(bw, &mut bw_best) == 0
            {
                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
            } else {
                sub_configs_idx = 0 as libc::c_int;
                's_100: loop {
                    if !(sub_configs_idx < (*config).sub_configs_size_) {
                        current_block = 12070711452894729854;
                        break;
                    }
                    let sub_config: *const CrunchSubConfig = &*((*config).sub_configs_)
                        .as_ptr()
                        .offset(sub_configs_idx as isize) as *const CrunchSubConfig;
                    let mut cache_bits_best: libc::c_int = 0;
                    let mut i_cache: libc::c_int = 0;
                    let mut i_remaining_percent: libc::c_int = remaining_percent
                        / (*config).sub_configs_size_;
                    let mut i_percent_range: libc::c_int = i_remaining_percent
                        / 4 as libc::c_int;
                    i_remaining_percent -= i_percent_range;
                    if VP8LGetBackwardReferences(
                        width,
                        height,
                        argb,
                        quality,
                        low_effort,
                        (*sub_config).lz77_,
                        cache_bits_init,
                        (*sub_config).do_no_cache_,
                        hash_chain,
                        &mut *refs_array.offset(0 as libc::c_int as isize),
                        &mut cache_bits_best,
                        pic,
                        i_percent_range,
                        percent,
                    ) == 0
                    {
                        current_block = 12728914781337687789;
                        break;
                    }
                    i_cache = 0 as libc::c_int;
                    while i_cache
                        < (if (*sub_config).do_no_cache_ != 0 {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })
                    {
                        let cache_bits_tmp: libc::c_int = if i_cache == 0 as libc::c_int
                        {
                            cache_bits_best
                        } else {
                            0 as libc::c_int
                        };
                        if i_cache == 1 as libc::c_int
                            && cache_bits_best == 0 as libc::c_int
                        {
                            break;
                        }
                        VP8LBitWriterReset(&mut bw_init, bw);
                        histogram_image = VP8LAllocateHistogramSet(
                            histogram_image_xysize as libc::c_int,
                            cache_bits_tmp,
                        );
                        tmp_histo = VP8LAllocateHistogram(cache_bits_tmp);
                        if histogram_image.is_null() || tmp_histo.is_null() {
                            WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                            current_block = 12728914781337687789;
                            break 's_100;
                        } else {
                            i_percent_range = i_remaining_percent / 3 as libc::c_int;
                            i_remaining_percent -= i_percent_range;
                            if VP8LGetHistoImageSymbols(
                                width,
                                height,
                                &mut *refs_array.offset(i_cache as isize),
                                quality,
                                low_effort,
                                histogram_bits,
                                cache_bits_tmp,
                                histogram_image,
                                tmp_histo,
                                histogram_symbols,
                                pic,
                                i_percent_range,
                                percent,
                            ) == 0
                            {
                                current_block = 12728914781337687789;
                                break 's_100;
                            }
                            histogram_image_size = (*histogram_image).size;
                            bit_array_size = (5 as libc::c_int * histogram_image_size)
                                as size_t;
                            huffman_codes = WebPSafeCalloc(
                                bit_array_size,
                                ::core::mem::size_of::<HuffmanTreeCode>() as libc::c_ulong,
                            ) as *mut HuffmanTreeCode;
                            if huffman_codes.is_null()
                                || GetHuffBitLengthsAndCodes(histogram_image, huffman_codes)
                                    == 0
                            {
                                WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                                current_block = 12728914781337687789;
                                break 's_100;
                            } else {
                                VP8LFreeHistogramSet(histogram_image);
                                histogram_image = 0 as *mut VP8LHistogramSet;
                                VP8LFreeHistogram(tmp_histo);
                                tmp_histo = 0 as *mut VP8LHistogram;
                                if cache_bits_tmp > 0 as libc::c_int {
                                    VP8LPutBits(
                                        bw,
                                        1 as libc::c_int as uint32_t,
                                        1 as libc::c_int,
                                    );
                                    VP8LPutBits(
                                        bw,
                                        cache_bits_tmp as uint32_t,
                                        4 as libc::c_int,
                                    );
                                } else {
                                    VP8LPutBits(
                                        bw,
                                        0 as libc::c_int as uint32_t,
                                        1 as libc::c_int,
                                    );
                                }
                                write_histogram_image = (histogram_image_size
                                    > 1 as libc::c_int) as libc::c_int;
                                VP8LPutBits(
                                    bw,
                                    write_histogram_image as uint32_t,
                                    1 as libc::c_int,
                                );
                                if write_histogram_image != 0 {
                                    let histogram_argb: *mut uint32_t = WebPSafeMalloc(
                                        histogram_image_xysize as uint64_t,
                                        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                                    ) as *mut uint32_t;
                                    let mut max_index: libc::c_int = 0 as libc::c_int;
                                    let mut i: uint32_t = 0;
                                    if histogram_argb.is_null() {
                                        WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                                        current_block = 12728914781337687789;
                                        break 's_100;
                                    } else {
                                        i = 0 as libc::c_int as uint32_t;
                                        while i < histogram_image_xysize {
                                            let symbol_index: libc::c_int = *histogram_symbols
                                                .offset(i as isize) as libc::c_int & 0xffff as libc::c_int;
                                            *histogram_argb
                                                .offset(
                                                    i as isize,
                                                ) = (symbol_index << 8 as libc::c_int) as uint32_t;
                                            if symbol_index >= max_index {
                                                max_index = symbol_index + 1 as libc::c_int;
                                            }
                                            i = i.wrapping_add(1);
                                            i;
                                        }
                                        histogram_image_size = max_index;
                                        VP8LPutBits(
                                            bw,
                                            (histogram_bits - 2 as libc::c_int) as uint32_t,
                                            3 as libc::c_int,
                                        );
                                        i_percent_range = i_remaining_percent / 2 as libc::c_int;
                                        i_remaining_percent -= i_percent_range;
                                        if EncodeImageNoHuffman(
                                            bw,
                                            histogram_argb,
                                            &mut hash_chain_histogram,
                                            &mut *refs_array.offset(2 as libc::c_int as isize),
                                            VP8LSubSampleSize(
                                                width as uint32_t,
                                                histogram_bits as uint32_t,
                                            ) as libc::c_int,
                                            VP8LSubSampleSize(
                                                height as uint32_t,
                                                histogram_bits as uint32_t,
                                            ) as libc::c_int,
                                            quality,
                                            low_effort,
                                            pic,
                                            i_percent_range,
                                            percent,
                                        ) == 0
                                        {
                                            WebPSafeFree(histogram_argb as *mut libc::c_void);
                                            current_block = 12728914781337687789;
                                            break 's_100;
                                        } else {
                                            WebPSafeFree(histogram_argb as *mut libc::c_void);
                                        }
                                    }
                                }
                                let mut i_0: libc::c_int = 0;
                                let mut max_tokens: libc::c_int = 0 as libc::c_int;
                                i_0 = 0 as libc::c_int;
                                while i_0 < 5 as libc::c_int * histogram_image_size {
                                    let codes: *mut HuffmanTreeCode = &mut *huffman_codes
                                        .offset(i_0 as isize) as *mut HuffmanTreeCode;
                                    if max_tokens < (*codes).num_symbols {
                                        max_tokens = (*codes).num_symbols;
                                    }
                                    i_0 += 1;
                                    i_0;
                                }
                                tokens = WebPSafeMalloc(
                                    max_tokens as uint64_t,
                                    ::core::mem::size_of::<HuffmanTreeToken>() as libc::c_ulong,
                                ) as *mut HuffmanTreeToken;
                                if tokens.is_null() {
                                    WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
                                    current_block = 12728914781337687789;
                                    break 's_100;
                                } else {
                                    i_0 = 0 as libc::c_int;
                                    while i_0 < 5 as libc::c_int * histogram_image_size {
                                        let codes_0: *mut HuffmanTreeCode = &mut *huffman_codes
                                            .offset(i_0 as isize) as *mut HuffmanTreeCode;
                                        StoreHuffmanCode(bw, huff_tree, tokens, codes_0);
                                        ClearHuffmanTreeIfOnlyOneSymbol(codes_0);
                                        i_0 += 1;
                                        i_0;
                                    }
                                    hdr_size_tmp = (VP8LBitWriterNumBytes(bw))
                                        .wrapping_sub(init_byte_position) as libc::c_int;
                                    if StoreImageToBitMask(
                                        bw,
                                        width,
                                        histogram_bits,
                                        &mut *refs_array.offset(i_cache as isize),
                                        histogram_symbols,
                                        huffman_codes,
                                        pic,
                                    ) == 0
                                    {
                                        current_block = 12728914781337687789;
                                        break 's_100;
                                    }
                                    if VP8LBitWriterNumBytes(bw) < bw_size_best {
                                        bw_size_best = VP8LBitWriterNumBytes(bw);
                                        *cache_bits = cache_bits_tmp;
                                        *hdr_size = hdr_size_tmp;
                                        *data_size = (VP8LBitWriterNumBytes(bw))
                                            .wrapping_sub(init_byte_position)
                                            .wrapping_sub(*hdr_size as libc::c_ulong) as libc::c_int;
                                        VP8LBitWriterSwap(bw, &mut bw_best);
                                    }
                                    WebPSafeFree(tokens as *mut libc::c_void);
                                    tokens = 0 as *mut HuffmanTreeToken;
                                    if !huffman_codes.is_null() {
                                        WebPSafeFree((*huffman_codes).codes as *mut libc::c_void);
                                        WebPSafeFree(huffman_codes as *mut libc::c_void);
                                        huffman_codes = 0 as *mut HuffmanTreeCode;
                                    }
                                    i_cache += 1;
                                    i_cache;
                                }
                            }
                        }
                    }
                    sub_configs_idx += 1;
                    sub_configs_idx;
                }
                match current_block {
                    12728914781337687789 => {}
                    _ => {
                        VP8LBitWriterSwap(bw, &mut bw_best);
                        WebPReportProgress(
                            pic,
                            percent_start + remaining_percent,
                            percent,
                        ) == 0;
                    }
                }
            }
        }
    }
    WebPSafeFree(tokens as *mut libc::c_void);
    WebPSafeFree(huff_tree as *mut libc::c_void);
    VP8LFreeHistogramSet(histogram_image);
    VP8LFreeHistogram(tmp_histo);
    VP8LHashChainClear(&mut hash_chain_histogram);
    if !huffman_codes.is_null() {
        WebPSafeFree((*huffman_codes).codes as *mut libc::c_void);
        WebPSafeFree(huffman_codes as *mut libc::c_void);
    }
    WebPSafeFree(histogram_symbols as *mut libc::c_void);
    VP8LBitWriterWipeOut(&mut bw_best);
    return ((*pic).error_code as libc::c_uint
        == VP8_ENC_OK as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn ApplySubtractGreen(
    enc: *mut VP8LEncoder,
    mut width: libc::c_int,
    mut height: libc::c_int,
    bw: *mut VP8LBitWriter,
) {
    VP8LPutBits(bw, 1 as libc::c_int as uint32_t, 1 as libc::c_int);
    VP8LPutBits(
        bw,
        SUBTRACT_GREEN_TRANSFORM as libc::c_int as uint32_t,
        2 as libc::c_int,
    );
    VP8LSubtractGreenFromBlueAndRed
        .expect("non-null function pointer")((*enc).argb_, width * height);
}
unsafe extern "C" fn ApplyPredictFilter(
    enc: *const VP8LEncoder,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut quality: libc::c_int,
    mut low_effort: libc::c_int,
    mut used_subtract_green: libc::c_int,
    bw: *mut VP8LBitWriter,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let pred_bits: libc::c_int = (*enc).transform_bits_;
    let transform_width: libc::c_int = VP8LSubSampleSize(
        width as uint32_t,
        pred_bits as uint32_t,
    ) as libc::c_int;
    let transform_height: libc::c_int = VP8LSubSampleSize(
        height as uint32_t,
        pred_bits as uint32_t,
    ) as libc::c_int;
    let near_lossless_strength: libc::c_int = if (*enc).use_palette_ != 0 {
        100 as libc::c_int
    } else {
        (*(*enc).config_).near_lossless
    };
    if VP8LResidualImage(
        width,
        height,
        pred_bits,
        low_effort,
        (*enc).argb_,
        (*enc).argb_scratch_,
        (*enc).transform_data_,
        near_lossless_strength,
        (*(*enc).config_).exact,
        used_subtract_green,
        (*enc).pic_,
        percent_range / 2 as libc::c_int,
        percent,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    VP8LPutBits(bw, 1 as libc::c_int as uint32_t, 1 as libc::c_int);
    VP8LPutBits(bw, PREDICTOR_TRANSFORM as libc::c_int as uint32_t, 2 as libc::c_int);
    VP8LPutBits(bw, (pred_bits - 2 as libc::c_int) as uint32_t, 3 as libc::c_int);
    return EncodeImageNoHuffman(
        bw,
        (*enc).transform_data_,
        &(*enc).hash_chain_ as *const VP8LHashChain as *mut VP8LHashChain,
        &*((*enc).refs_).as_ptr().offset(0 as libc::c_int as isize)
            as *const VP8LBackwardRefs as *mut VP8LBackwardRefs,
        transform_width,
        transform_height,
        quality,
        low_effort,
        (*enc).pic_,
        percent_range - percent_range / 2 as libc::c_int,
        percent,
    );
}
unsafe extern "C" fn ApplyCrossColorFilter(
    enc: *const VP8LEncoder,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut quality: libc::c_int,
    mut low_effort: libc::c_int,
    bw: *mut VP8LBitWriter,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> libc::c_int {
    let ccolor_transform_bits: libc::c_int = (*enc).transform_bits_;
    let transform_width: libc::c_int = VP8LSubSampleSize(
        width as uint32_t,
        ccolor_transform_bits as uint32_t,
    ) as libc::c_int;
    let transform_height: libc::c_int = VP8LSubSampleSize(
        height as uint32_t,
        ccolor_transform_bits as uint32_t,
    ) as libc::c_int;
    if VP8LColorSpaceTransform(
        width,
        height,
        ccolor_transform_bits,
        quality,
        (*enc).argb_,
        (*enc).transform_data_,
        (*enc).pic_,
        percent_range / 2 as libc::c_int,
        percent,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    VP8LPutBits(bw, 1 as libc::c_int as uint32_t, 1 as libc::c_int);
    VP8LPutBits(bw, CROSS_COLOR_TRANSFORM as libc::c_int as uint32_t, 2 as libc::c_int);
    VP8LPutBits(
        bw,
        (ccolor_transform_bits - 2 as libc::c_int) as uint32_t,
        3 as libc::c_int,
    );
    return EncodeImageNoHuffman(
        bw,
        (*enc).transform_data_,
        &(*enc).hash_chain_ as *const VP8LHashChain as *mut VP8LHashChain,
        &*((*enc).refs_).as_ptr().offset(0 as libc::c_int as isize)
            as *const VP8LBackwardRefs as *mut VP8LBackwardRefs,
        transform_width,
        transform_height,
        quality,
        low_effort,
        (*enc).pic_,
        percent_range - percent_range / 2 as libc::c_int,
        percent,
    );
}
unsafe extern "C" fn WriteRiffHeader(
    pic: *const WebPPicture,
    mut riff_size: size_t,
    mut vp8l_size: size_t,
) -> libc::c_int {
    let mut riff: [uint8_t; 21] = [
        'R' as i32 as uint8_t,
        'I' as i32 as uint8_t,
        'F' as i32 as uint8_t,
        'F' as i32 as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        'W' as i32 as uint8_t,
        'E' as i32 as uint8_t,
        'B' as i32 as uint8_t,
        'P' as i32 as uint8_t,
        'V' as i32 as uint8_t,
        'P' as i32 as uint8_t,
        '8' as i32 as uint8_t,
        'L' as i32 as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0x2f as libc::c_int as uint8_t,
    ];
    PutLE32(riff.as_mut_ptr().offset(4 as libc::c_int as isize), riff_size as uint32_t);
    PutLE32(
        riff
            .as_mut_ptr()
            .offset(12 as libc::c_int as isize)
            .offset(4 as libc::c_int as isize),
        vp8l_size as uint32_t,
    );
    return ((*pic).writer)
        .expect(
            "non-null function pointer",
        )(
        riff.as_mut_ptr(),
        ::core::mem::size_of::<[uint8_t; 21]>() as libc::c_ulong,
        pic,
    );
}
unsafe extern "C" fn WriteImageSize(
    pic: *const WebPPicture,
    bw: *mut VP8LBitWriter,
) -> libc::c_int {
    let width: libc::c_int = (*pic).width - 1 as libc::c_int;
    let height: libc::c_int = (*pic).height - 1 as libc::c_int;
    VP8LPutBits(bw, width as uint32_t, 14 as libc::c_int);
    VP8LPutBits(bw, height as uint32_t, 14 as libc::c_int);
    return ((*bw).error_ == 0) as libc::c_int;
}
unsafe extern "C" fn WriteRealAlphaAndVersion(
    bw: *mut VP8LBitWriter,
    mut has_alpha: libc::c_int,
) -> libc::c_int {
    VP8LPutBits(bw, has_alpha as uint32_t, 1 as libc::c_int);
    VP8LPutBits(bw, 0 as libc::c_int as uint32_t, 3 as libc::c_int);
    return ((*bw).error_ == 0) as libc::c_int;
}
unsafe extern "C" fn WriteImage(
    pic: *const WebPPicture,
    bw: *mut VP8LBitWriter,
    coded_size: *mut size_t,
) -> libc::c_int {
    let webpll_data: *const uint8_t = VP8LBitWriterFinish(bw);
    let webpll_size: size_t = VP8LBitWriterNumBytes(bw);
    let vp8l_size: size_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(webpll_size);
    let pad: size_t = vp8l_size & 1 as libc::c_int as libc::c_ulong;
    let riff_size: size_t = ((4 as libc::c_int + 8 as libc::c_int) as libc::c_ulong)
        .wrapping_add(vp8l_size)
        .wrapping_add(pad);
    *coded_size = 0 as libc::c_int as size_t;
    if (*bw).error_ != 0 {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    if WriteRiffHeader(pic, riff_size, vp8l_size) == 0
        || ((*pic).writer)
            .expect("non-null function pointer")(webpll_data, webpll_size, pic) == 0
    {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_BAD_WRITE);
    }
    if pad != 0 {
        let pad_byte: [uint8_t; 1] = [0 as libc::c_int as uint8_t];
        if ((*pic).writer)
            .expect(
                "non-null function pointer",
            )(pad_byte.as_ptr(), 1 as libc::c_int as size_t, pic) == 0
        {
            return WebPEncodingSetError(pic, VP8_ENC_ERROR_BAD_WRITE);
        }
    }
    *coded_size = (8 as libc::c_int as libc::c_ulong).wrapping_add(riff_size);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ClearTransformBuffer(enc: *mut VP8LEncoder) {
    WebPSafeFree((*enc).transform_mem_ as *mut libc::c_void);
    (*enc).transform_mem_ = 0 as *mut uint32_t;
    (*enc).transform_mem_size_ = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn AllocateTransformBuffer(
    enc: *mut VP8LEncoder,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let image_size: uint64_t = (width as uint64_t).wrapping_mul(height as libc::c_ulong);
    let argb_scratch_size: uint64_t = if (*enc).use_predict_ != 0 {
        (((width + 1 as libc::c_int) * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_add(
                ((width * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            )
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let transform_data_size: uint64_t = if (*enc).use_predict_ != 0
        || (*enc).use_cross_color_ != 0
    {
        (VP8LSubSampleSize(width as uint32_t, (*enc).transform_bits_ as uint32_t)
            as uint64_t)
            .wrapping_mul(
                VP8LSubSampleSize(height as uint32_t, (*enc).transform_bits_ as uint32_t)
                    as libc::c_ulong,
            )
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let max_alignment_in_words: uint64_t = (31 as libc::c_int as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<uint32_t>() as libc::c_ulong);
    let mem_size: uint64_t = image_size
        .wrapping_add(max_alignment_in_words)
        .wrapping_add(argb_scratch_size)
        .wrapping_add(max_alignment_in_words)
        .wrapping_add(transform_data_size);
    let mut mem: *mut uint32_t = (*enc).transform_mem_;
    if mem.is_null() || mem_size > (*enc).transform_mem_size_ {
        ClearTransformBuffer(enc);
        mem = WebPSafeMalloc(
            mem_size,
            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
        ) as *mut uint32_t;
        if mem.is_null() {
            return WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        (*enc).transform_mem_ = mem;
        (*enc).transform_mem_size_ = mem_size;
        (*enc).argb_content_ = kEncoderNone;
    }
    (*enc).argb_ = mem;
    mem = ((mem.offset(image_size as isize) as uintptr_t)
        .wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut uint32_t;
    (*enc).argb_scratch_ = mem;
    mem = ((mem.offset(argb_scratch_size as isize) as uintptr_t)
        .wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut uint32_t;
    (*enc).transform_data_ = mem;
    (*enc).current_width_ = width;
    return 1 as libc::c_int;
}
unsafe extern "C" fn MakeInputImageCopy(enc: *mut VP8LEncoder) -> libc::c_int {
    let picture: *const WebPPicture = (*enc).pic_;
    let width: libc::c_int = (*picture).width;
    let height: libc::c_int = (*picture).height;
    if AllocateTransformBuffer(enc, width, height) == 0 {
        return 0 as libc::c_int;
    }
    if (*enc).argb_content_ as libc::c_uint
        == kEncoderARGB as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    let mut dst: *mut uint32_t = (*enc).argb_;
    let mut src: *const uint32_t = (*picture).argb;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < height {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (width as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
        dst = dst.offset(width as isize);
        src = src.offset((*picture).argb_stride as isize);
        y += 1;
        y;
    }
    (*enc).argb_content_ = kEncoderARGB;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn SearchColorGreedy(
    mut palette: *const uint32_t,
    mut palette_size: libc::c_int,
    mut color: uint32_t,
) -> uint32_t {
    if color == *palette.offset(0 as libc::c_int as isize) {
        return 0 as libc::c_int as uint32_t;
    }
    if color == *palette.offset(1 as libc::c_int as isize) {
        return 1 as libc::c_int as uint32_t;
    }
    if color == *palette.offset(2 as libc::c_int as isize) {
        return 2 as libc::c_int as uint32_t;
    }
    return 3 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn ApplyPaletteHash0(mut color: uint32_t) -> uint32_t {
    return color >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ApplyPaletteHash1(mut color: uint32_t) -> uint32_t {
    return ((color & 0xffffff as libc::c_uint) as libc::c_ulonglong)
        .wrapping_mul(4222244071 as libc::c_ulonglong) as uint32_t
        >> 32 as libc::c_int - 11 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ApplyPaletteHash2(mut color: uint32_t) -> uint32_t {
    return ((color & 0xffffff as libc::c_uint) as libc::c_ulonglong)
        .wrapping_mul(
            ((1 as libc::c_ulonglong) << 31 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong),
        ) as uint32_t >> 32 as libc::c_int - 11 as libc::c_int;
}
unsafe extern "C" fn ApplyPalette(
    mut src: *const uint32_t,
    mut src_stride: uint32_t,
    mut dst: *mut uint32_t,
    mut dst_stride: uint32_t,
    mut palette: *const uint32_t,
    mut palette_size: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut xbits: libc::c_int,
    pic: *const WebPPicture,
) -> libc::c_int {
    let tmp_row: *mut uint8_t = WebPSafeMalloc(
        width as uint64_t,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if tmp_row.is_null() {
        return WebPEncodingSetError(pic, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    if palette_size < 4 as libc::c_int {
        let mut prev_pix: uint32_t = *palette.offset(0 as libc::c_int as isize);
        let mut prev_idx: uint32_t = 0 as libc::c_int as uint32_t;
        y = 0 as libc::c_int;
        while y < height {
            x = 0 as libc::c_int;
            while x < width {
                let pix: uint32_t = *src.offset(x as isize);
                if pix != prev_pix {
                    prev_idx = SearchColorGreedy(palette, palette_size, pix);
                    prev_pix = pix;
                }
                *tmp_row.offset(x as isize) = prev_idx as uint8_t;
                x += 1;
                x;
            }
            VP8LBundleColorMap
                .expect("non-null function pointer")(tmp_row, width, xbits, dst);
            src = src.offset(src_stride as isize);
            dst = dst.offset(dst_stride as isize);
            y += 1;
            y;
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut buffer: [uint16_t; 2048] = [0; 2048];
        let hash_functions: [Option::<unsafe extern "C" fn(uint32_t) -> uint32_t>; 3] = [
            Some(ApplyPaletteHash0 as unsafe extern "C" fn(uint32_t) -> uint32_t),
            Some(ApplyPaletteHash1 as unsafe extern "C" fn(uint32_t) -> uint32_t),
            Some(ApplyPaletteHash2 as unsafe extern "C" fn(uint32_t) -> uint32_t),
        ];
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            let mut use_LUT: libc::c_int = 1 as libc::c_int;
            memset(
                buffer.as_mut_ptr() as *mut libc::c_void,
                0xff as libc::c_int,
                ::core::mem::size_of::<[uint16_t; 2048]>() as libc::c_ulong,
            );
            j = 0 as libc::c_int;
            while j < palette_size {
                let ind: uint32_t = (hash_functions[i as usize])
                    .expect("non-null function pointer")(*palette.offset(j as isize));
                if buffer[ind as usize] as libc::c_uint != 0xffff as libc::c_uint {
                    use_LUT = 0 as libc::c_int;
                    break;
                } else {
                    buffer[ind as usize] = j as uint16_t;
                    j += 1;
                    j;
                }
            }
            if use_LUT != 0 {
                break;
            }
            i += 1;
            i;
        }
        if i == 0 as libc::c_int {
            let mut prev_pix_0: uint32_t = *palette.offset(0 as libc::c_int as isize);
            let mut prev_idx_0: uint32_t = 0 as libc::c_int as uint32_t;
            y = 0 as libc::c_int;
            while y < height {
                x = 0 as libc::c_int;
                while x < width {
                    let pix_0: uint32_t = *src.offset(x as isize);
                    if pix_0 != prev_pix_0 {
                        prev_idx_0 = buffer[ApplyPaletteHash0(pix_0) as usize]
                            as uint32_t;
                        prev_pix_0 = pix_0;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_0 as uint8_t;
                    x += 1;
                    x;
                }
                VP8LBundleColorMap
                    .expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
                y;
            }
        } else if i == 1 as libc::c_int {
            let mut prev_pix_1: uint32_t = *palette.offset(0 as libc::c_int as isize);
            let mut prev_idx_1: uint32_t = 0 as libc::c_int as uint32_t;
            y = 0 as libc::c_int;
            while y < height {
                x = 0 as libc::c_int;
                while x < width {
                    let pix_1: uint32_t = *src.offset(x as isize);
                    if pix_1 != prev_pix_1 {
                        prev_idx_1 = buffer[ApplyPaletteHash1(pix_1) as usize]
                            as uint32_t;
                        prev_pix_1 = pix_1;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_1 as uint8_t;
                    x += 1;
                    x;
                }
                VP8LBundleColorMap
                    .expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
                y;
            }
        } else if i == 2 as libc::c_int {
            let mut prev_pix_2: uint32_t = *palette.offset(0 as libc::c_int as isize);
            let mut prev_idx_2: uint32_t = 0 as libc::c_int as uint32_t;
            y = 0 as libc::c_int;
            while y < height {
                x = 0 as libc::c_int;
                while x < width {
                    let pix_2: uint32_t = *src.offset(x as isize);
                    if pix_2 != prev_pix_2 {
                        prev_idx_2 = buffer[ApplyPaletteHash2(pix_2) as usize]
                            as uint32_t;
                        prev_pix_2 = pix_2;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_2 as uint8_t;
                    x += 1;
                    x;
                }
                VP8LBundleColorMap
                    .expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
                y;
            }
        } else {
            let mut idx_map: [uint32_t; 256] = [0; 256];
            let mut palette_sorted: [uint32_t; 256] = [0; 256];
            PrepareMapToPalette(
                palette,
                palette_size as uint32_t,
                palette_sorted.as_mut_ptr(),
                idx_map.as_mut_ptr(),
            );
            let mut prev_pix_3: uint32_t = *palette.offset(0 as libc::c_int as isize);
            let mut prev_idx_3: uint32_t = 0 as libc::c_int as uint32_t;
            y = 0 as libc::c_int;
            while y < height {
                x = 0 as libc::c_int;
                while x < width {
                    let pix_3: uint32_t = *src.offset(x as isize);
                    if pix_3 != prev_pix_3 {
                        prev_idx_3 = idx_map[SearchColorNoIdx(
                            palette_sorted.as_mut_ptr() as *const uint32_t,
                            pix_3,
                            palette_size,
                        ) as usize];
                        prev_pix_3 = pix_3;
                    }
                    *tmp_row.offset(x as isize) = prev_idx_3 as uint8_t;
                    x += 1;
                    x;
                }
                VP8LBundleColorMap
                    .expect("non-null function pointer")(tmp_row, width, xbits, dst);
                src = src.offset(src_stride as isize);
                dst = dst.offset(dst_stride as isize);
                y += 1;
                y;
            }
        }
    }
    WebPSafeFree(tmp_row as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn MapImageFromPalette(
    enc: *mut VP8LEncoder,
    mut in_place: libc::c_int,
) -> libc::c_int {
    let pic: *const WebPPicture = (*enc).pic_;
    let width: libc::c_int = (*pic).width;
    let height: libc::c_int = (*pic).height;
    let palette: *const uint32_t = ((*enc).palette_).as_mut_ptr();
    let mut src: *const uint32_t = if in_place != 0 {
        (*enc).argb_
    } else {
        (*pic).argb
    };
    let src_stride: libc::c_int = if in_place != 0 {
        (*enc).current_width_
    } else {
        (*pic).argb_stride
    };
    let palette_size: libc::c_int = (*enc).palette_size_;
    let mut xbits: libc::c_int = 0;
    if palette_size <= 4 as libc::c_int {
        xbits = if palette_size <= 2 as libc::c_int {
            3 as libc::c_int
        } else {
            2 as libc::c_int
        };
    } else {
        xbits = if palette_size <= 16 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    if AllocateTransformBuffer(
        enc,
        VP8LSubSampleSize(width as uint32_t, xbits as uint32_t) as libc::c_int,
        height,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if ApplyPalette(
        src,
        src_stride as uint32_t,
        (*enc).argb_,
        (*enc).current_width_ as uint32_t,
        palette,
        palette_size,
        width,
        height,
        xbits,
        pic,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    (*enc).argb_content_ = kEncoderPalette;
    return 1 as libc::c_int;
}
unsafe extern "C" fn EncodePalette(
    bw: *mut VP8LBitWriter,
    mut low_effort: libc::c_int,
    enc: *mut VP8LEncoder,
    mut percent_range: libc::c_int,
    percent: *mut libc::c_int,
) -> WebPEncodingError {
    let mut i: libc::c_int = 0;
    let mut tmp_palette: [uint32_t; 256] = [0; 256];
    let palette_size: libc::c_int = (*enc).palette_size_;
    let palette: *const uint32_t = ((*enc).palette_).as_mut_ptr();
    VP8LPutBits(bw, 1 as libc::c_int as uint32_t, 1 as libc::c_int);
    VP8LPutBits(
        bw,
        COLOR_INDEXING_TRANSFORM as libc::c_int as uint32_t,
        2 as libc::c_int,
    );
    VP8LPutBits(bw, (palette_size - 1 as libc::c_int) as uint32_t, 8 as libc::c_int);
    i = palette_size - 1 as libc::c_int;
    while i >= 1 as libc::c_int {
        tmp_palette[i
            as usize] = VP8LSubPixels(
            *palette.offset(i as isize),
            *palette.offset((i - 1 as libc::c_int) as isize),
        );
        i -= 1;
        i;
    }
    tmp_palette[0 as libc::c_int as usize] = *palette.offset(0 as libc::c_int as isize);
    return EncodeImageNoHuffman(
        bw,
        tmp_palette.as_mut_ptr(),
        &mut (*enc).hash_chain_,
        &mut *((*enc).refs_).as_mut_ptr().offset(0 as libc::c_int as isize),
        palette_size,
        1 as libc::c_int,
        20 as libc::c_int,
        low_effort,
        (*enc).pic_,
        percent_range,
        percent,
    ) as WebPEncodingError;
}
unsafe extern "C" fn VP8LEncoderNew(
    config: *const WebPConfig,
    picture: *const WebPPicture,
) -> *mut VP8LEncoder {
    let enc: *mut VP8LEncoder = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<VP8LEncoder>() as libc::c_ulong,
    ) as *mut VP8LEncoder;
    if enc.is_null() {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        return 0 as *mut VP8LEncoder;
    }
    (*enc).config_ = config;
    (*enc).pic_ = picture;
    (*enc).argb_content_ = kEncoderNone;
    VP8LEncDspInit();
    return enc;
}
unsafe extern "C" fn VP8LEncoderDelete(mut enc: *mut VP8LEncoder) {
    if !enc.is_null() {
        let mut i: libc::c_int = 0;
        VP8LHashChainClear(&mut (*enc).hash_chain_);
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            VP8LBackwardRefsClear(&mut *((*enc).refs_).as_mut_ptr().offset(i as isize));
            i += 1;
            i;
        }
        ClearTransformBuffer(enc);
        WebPSafeFree(enc as *mut libc::c_void);
    }
}
unsafe extern "C" fn EncodeStreamHook(
    mut input: *mut libc::c_void,
    mut data2: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let params: *mut StreamEncodeContext = input as *mut StreamEncodeContext;
    let config: *const WebPConfig = (*params).config_;
    let picture: *const WebPPicture = (*params).picture_;
    let bw: *mut VP8LBitWriter = (*params).bw_;
    let enc: *mut VP8LEncoder = (*params).enc_;
    let crunch_configs: *const CrunchConfig = ((*params).crunch_configs_).as_mut_ptr();
    let num_crunch_configs: libc::c_int = (*params).num_crunch_configs_;
    let red_and_blue_always_zero: libc::c_int = (*params).red_and_blue_always_zero_;
    let stats: *mut WebPAuxStats = (*params).stats_;
    let quality: libc::c_int = (*config).quality as libc::c_int;
    let low_effort: libc::c_int = ((*config).method == 0 as libc::c_int) as libc::c_int;
    let width: libc::c_int = (*picture).width;
    let height: libc::c_int = (*picture).height;
    let byte_position: size_t = VP8LBitWriterNumBytes(bw);
    let mut percent: libc::c_int = 2 as libc::c_int;
    let mut use_near_lossless: libc::c_int = 0 as libc::c_int;
    let mut hdr_size: libc::c_int = 0 as libc::c_int;
    let mut data_size: libc::c_int = 0 as libc::c_int;
    let mut use_delta_palette: libc::c_int = 0 as libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut best_size: size_t = !(0 as libc::c_int as size_t);
    let mut bw_init: VP8LBitWriter = *bw;
    let mut bw_best: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: 0 as *mut uint8_t,
        cur_: 0 as *mut uint8_t,
        end_: 0 as *mut uint8_t,
        error_: 0,
    };
    if VP8LBitWriterInit(&mut bw_best, 0 as libc::c_int as size_t) == 0
        || num_crunch_configs > 1 as libc::c_int
            && VP8LBitWriterClone(bw, &mut bw_best) == 0
    {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        idx = 0 as libc::c_int;
        loop {
            if !(idx < num_crunch_configs) {
                current_block = 13910774313357589740;
                break;
            }
            let entropy_idx: libc::c_int = (*crunch_configs.offset(idx as isize))
                .entropy_idx_;
            let mut remaining_percent: libc::c_int = 97 as libc::c_int
                / num_crunch_configs;
            let mut percent_range: libc::c_int = 0;
            (*enc)
                .use_palette_ = (entropy_idx == kPalette as libc::c_int
                || entropy_idx == kPaletteAndSpatial as libc::c_int) as libc::c_int;
            (*enc)
                .use_subtract_green_ = (entropy_idx == kSubGreen as libc::c_int
                || entropy_idx == kSpatialSubGreen as libc::c_int) as libc::c_int;
            (*enc)
                .use_predict_ = (entropy_idx == kSpatial as libc::c_int
                || entropy_idx == kSpatialSubGreen as libc::c_int
                || entropy_idx == kPaletteAndSpatial as libc::c_int) as libc::c_int;
            if low_effort != 0 || (*enc).use_palette_ != 0 {
                (*enc).use_cross_color_ = 0 as libc::c_int;
            } else {
                (*enc)
                    .use_cross_color_ = if red_and_blue_always_zero != 0 {
                    0 as libc::c_int
                } else {
                    (*enc).use_predict_
                };
            }
            (*enc).cache_bits_ = 0 as libc::c_int;
            VP8LBackwardRefsClear(
                &mut *((*enc).refs_).as_mut_ptr().offset(0 as libc::c_int as isize),
            );
            VP8LBackwardRefsClear(
                &mut *((*enc).refs_).as_mut_ptr().offset(1 as libc::c_int as isize),
            );
            use_near_lossless = ((*config).near_lossless < 100 as libc::c_int
                && (*enc).use_palette_ == 0 && (*enc).use_predict_ == 0) as libc::c_int;
            if use_near_lossless != 0 {
                if AllocateTransformBuffer(enc, width, height) == 0 {
                    current_block = 12081408668797897407;
                    break;
                }
                if (*enc).argb_content_ as libc::c_uint
                    != kEncoderNearLossless as libc::c_int as libc::c_uint
                    && VP8ApplyNearLossless(
                        picture,
                        (*config).near_lossless,
                        (*enc).argb_,
                    ) == 0
                {
                    WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 12081408668797897407;
                    break;
                } else {
                    (*enc).argb_content_ = kEncoderNearLossless;
                }
            } else {
                (*enc).argb_content_ = kEncoderNone;
            }
            if (*enc).use_palette_ != 0 {
                if PaletteSort(
                    (*crunch_configs.offset(idx as isize)).palette_sorting_type_,
                    (*enc).pic_,
                    ((*enc).palette_sorted_).as_mut_ptr(),
                    (*enc).palette_size_ as uint32_t,
                    ((*enc).palette_).as_mut_ptr(),
                ) == 0
                {
                    WebPEncodingSetError((*enc).pic_, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 12081408668797897407;
                    break;
                } else {
                    percent_range = remaining_percent / 4 as libc::c_int;
                    if EncodePalette(bw, low_effort, enc, percent_range, &mut percent)
                        as u64 == 0
                    {
                        current_block = 12081408668797897407;
                        break;
                    }
                    remaining_percent -= percent_range;
                    if MapImageFromPalette(enc, use_delta_palette) == 0 {
                        current_block = 12081408668797897407;
                        break;
                    }
                    if (*enc).palette_size_ < (1 as libc::c_int) << 10 as libc::c_int {
                        (*enc)
                            .cache_bits_ = BitsLog2Floor(
                            (*enc).palette_size_ as uint32_t,
                        ) + 1 as libc::c_int;
                    }
                }
            }
            if use_delta_palette == 0 {
                if (*enc).argb_content_ as libc::c_uint
                    != kEncoderNearLossless as libc::c_int as libc::c_uint
                    && (*enc).argb_content_ as libc::c_uint
                        != kEncoderPalette as libc::c_int as libc::c_uint
                {
                    if MakeInputImageCopy(enc) == 0 {
                        current_block = 12081408668797897407;
                        break;
                    }
                }
                if (*enc).use_subtract_green_ != 0 {
                    ApplySubtractGreen(enc, (*enc).current_width_, height, bw);
                }
                if (*enc).use_predict_ != 0 {
                    percent_range = remaining_percent / 3 as libc::c_int;
                    if ApplyPredictFilter(
                        enc,
                        (*enc).current_width_,
                        height,
                        quality,
                        low_effort,
                        (*enc).use_subtract_green_,
                        bw,
                        percent_range,
                        &mut percent,
                    ) == 0
                    {
                        current_block = 12081408668797897407;
                        break;
                    }
                    remaining_percent -= percent_range;
                }
                if (*enc).use_cross_color_ != 0 {
                    percent_range = remaining_percent / 2 as libc::c_int;
                    if ApplyCrossColorFilter(
                        enc,
                        (*enc).current_width_,
                        height,
                        quality,
                        low_effort,
                        bw,
                        percent_range,
                        &mut percent,
                    ) == 0
                    {
                        current_block = 12081408668797897407;
                        break;
                    }
                    remaining_percent -= percent_range;
                }
            }
            VP8LPutBits(
                bw,
                (1 as libc::c_int == 0) as libc::c_int as uint32_t,
                1 as libc::c_int,
            );
            if EncodeImageInternal(
                bw,
                (*enc).argb_,
                &mut (*enc).hash_chain_,
                ((*enc).refs_).as_mut_ptr(),
                (*enc).current_width_,
                height,
                quality,
                low_effort,
                &*crunch_configs.offset(idx as isize),
                &mut (*enc).cache_bits_,
                (*enc).histo_bits_,
                byte_position,
                &mut hdr_size,
                &mut data_size,
                picture,
                remaining_percent,
                &mut percent,
            ) == 0
            {
                current_block = 12081408668797897407;
                break;
            }
            if VP8LBitWriterNumBytes(bw) < best_size {
                best_size = VP8LBitWriterNumBytes(bw);
                VP8LBitWriterSwap(bw, &mut bw_best);
                if !stats.is_null() {
                    (*stats).lossless_features = 0 as libc::c_int as uint32_t;
                    if (*enc).use_predict_ != 0 {
                        (*stats).lossless_features |= 1 as libc::c_int as libc::c_uint;
                    }
                    if (*enc).use_cross_color_ != 0 {
                        (*stats).lossless_features |= 2 as libc::c_int as libc::c_uint;
                    }
                    if (*enc).use_subtract_green_ != 0 {
                        (*stats).lossless_features |= 4 as libc::c_int as libc::c_uint;
                    }
                    if (*enc).use_palette_ != 0 {
                        (*stats).lossless_features |= 8 as libc::c_int as libc::c_uint;
                    }
                    (*stats).histogram_bits = (*enc).histo_bits_;
                    (*stats).transform_bits = (*enc).transform_bits_;
                    (*stats).cache_bits = (*enc).cache_bits_;
                    (*stats).palette_size = (*enc).palette_size_;
                    (*stats)
                        .lossless_size = best_size.wrapping_sub(byte_position)
                        as libc::c_int;
                    (*stats).lossless_hdr_size = hdr_size;
                    (*stats).lossless_data_size = data_size;
                }
            }
            if num_crunch_configs > 1 as libc::c_int {
                VP8LBitWriterReset(&mut bw_init, bw);
            }
            idx += 1;
            idx;
        }
        match current_block {
            12081408668797897407 => {}
            _ => {
                VP8LBitWriterSwap(&mut bw_best, bw);
            }
        }
    }
    VP8LBitWriterWipeOut(&mut bw_best);
    return ((*(*params).picture_).error_code as libc::c_uint
        == VP8_ENC_OK as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LEncodeStream(
    config: *const WebPConfig,
    picture: *const WebPPicture,
    bw_main: *mut VP8LBitWriter,
) -> libc::c_int {
    let mut current_block: u64;
    let enc_main: *mut VP8LEncoder = VP8LEncoderNew(config, picture);
    let mut enc_side: *mut VP8LEncoder = 0 as *mut VP8LEncoder;
    let mut crunch_configs: [CrunchConfig; 14] = [CrunchConfig {
        entropy_idx_: 0,
        palette_sorting_type_: kSortedDefault,
        sub_configs_: [CrunchSubConfig {
            lz77_: 0,
            do_no_cache_: 0,
        }; 2],
        sub_configs_size_: 0,
    }; 14];
    let mut num_crunch_configs_main: libc::c_int = 0;
    let mut num_crunch_configs_side: libc::c_int = 0 as libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut red_and_blue_always_zero: libc::c_int = 0 as libc::c_int;
    let mut worker_main: WebPWorker = WebPWorker {
        impl_: 0 as *mut libc::c_void,
        status_: NOT_OK,
        hook: None,
        data1: 0 as *mut libc::c_void,
        data2: 0 as *mut libc::c_void,
        had_error: 0,
    };
    let mut worker_side: WebPWorker = WebPWorker {
        impl_: 0 as *mut libc::c_void,
        status_: NOT_OK,
        hook: None,
        data1: 0 as *mut libc::c_void,
        data2: 0 as *mut libc::c_void,
        had_error: 0,
    };
    let mut params_main: StreamEncodeContext = StreamEncodeContext {
        config_: 0 as *const WebPConfig,
        picture_: 0 as *const WebPPicture,
        bw_: 0 as *mut VP8LBitWriter,
        enc_: 0 as *mut VP8LEncoder,
        crunch_configs_: [CrunchConfig {
            entropy_idx_: 0,
            palette_sorting_type_: kSortedDefault,
            sub_configs_: [CrunchSubConfig {
                lz77_: 0,
                do_no_cache_: 0,
            }; 2],
            sub_configs_size_: 0,
        }; 14],
        num_crunch_configs_: 0,
        red_and_blue_always_zero_: 0,
        stats_: 0 as *mut WebPAuxStats,
    };
    let mut params_side: StreamEncodeContext = StreamEncodeContext {
        config_: 0 as *const WebPConfig,
        picture_: 0 as *const WebPPicture,
        bw_: 0 as *mut VP8LBitWriter,
        enc_: 0 as *mut VP8LEncoder,
        crunch_configs_: [CrunchConfig {
            entropy_idx_: 0,
            palette_sorting_type_: kSortedDefault,
            sub_configs_: [CrunchSubConfig {
                lz77_: 0,
                do_no_cache_: 0,
            }; 2],
            sub_configs_size_: 0,
        }; 14],
        num_crunch_configs_: 0,
        red_and_blue_always_zero_: 0,
        stats_: 0 as *mut WebPAuxStats,
    };
    let mut stats_side: WebPAuxStats = WebPAuxStats {
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
    };
    let mut bw_side: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: 0 as *mut uint8_t,
        cur_: 0 as *mut uint8_t,
        end_: 0 as *mut uint8_t,
        error_: 0,
    };
    let mut picture_side: WebPPicture = WebPPicture {
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
    let worker_interface: *const WebPWorkerInterface = WebPGetWorkerInterface();
    let mut ok_main: libc::c_int = 0;
    if enc_main.is_null()
        || VP8LBitWriterInit(&mut bw_side, 0 as libc::c_int as size_t) == 0
    {
        VP8LEncoderDelete(enc_main);
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    if !(WebPPictureInit(&mut picture_side) == 0) {
        if EncoderAnalyze(
            enc_main,
            crunch_configs.as_mut_ptr(),
            &mut num_crunch_configs_main,
            &mut red_and_blue_always_zero,
        ) == 0 || EncoderInit(enc_main) == 0
        {
            WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
        } else {
            if (*config).thread_level > 0 as libc::c_int {
                num_crunch_configs_side = num_crunch_configs_main / 2 as libc::c_int;
                idx = 0 as libc::c_int;
                while idx < num_crunch_configs_side {
                    params_side
                        .crunch_configs_[idx
                        as usize] = crunch_configs[(num_crunch_configs_main
                        - num_crunch_configs_side + idx) as usize];
                    idx += 1;
                    idx;
                }
                params_side.num_crunch_configs_ = num_crunch_configs_side;
            }
            num_crunch_configs_main -= num_crunch_configs_side;
            idx = 0 as libc::c_int;
            while idx < num_crunch_configs_main {
                params_main.crunch_configs_[idx as usize] = crunch_configs[idx as usize];
                idx += 1;
                idx;
            }
            params_main.num_crunch_configs_ = num_crunch_configs_main;
            let params_size: libc::c_int = if num_crunch_configs_side > 0 as libc::c_int
            {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            idx = 0 as libc::c_int;
            loop {
                if !(idx < params_size) {
                    current_block = 11763295167351361500;
                    break;
                }
                let worker: *mut WebPWorker = if idx == 0 as libc::c_int {
                    &mut worker_main
                } else {
                    &mut worker_side
                };
                let param: *mut StreamEncodeContext = if idx == 0 as libc::c_int {
                    &mut params_main
                } else {
                    &mut params_side
                };
                (*param).config_ = config;
                (*param).red_and_blue_always_zero_ = red_and_blue_always_zero;
                if idx == 0 as libc::c_int {
                    (*param).picture_ = picture;
                    (*param).stats_ = (*picture).stats;
                    (*param).bw_ = bw_main;
                    (*param).enc_ = enc_main;
                } else {
                    WebPPictureView(
                        picture,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        (*picture).width,
                        (*picture).height,
                        &mut picture_side,
                    ) == 0;
                    picture_side.progress_hook = None;
                    (*param).picture_ = &mut picture_side;
                    (*param)
                        .stats_ = if ((*picture).stats).is_null() {
                        0 as *mut WebPAuxStats
                    } else {
                        &mut stats_side
                    };
                    if VP8LBitWriterClone(bw_main, &mut bw_side) == 0 {
                        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                        current_block = 12803755504025630281;
                        break;
                    } else {
                        (*param).bw_ = &mut bw_side;
                        enc_side = VP8LEncoderNew(config, &mut picture_side);
                        if enc_side.is_null() || EncoderInit(enc_side) == 0 {
                            WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                            current_block = 12803755504025630281;
                            break;
                        } else {
                            (*enc_side).histo_bits_ = (*enc_main).histo_bits_;
                            (*enc_side).transform_bits_ = (*enc_main).transform_bits_;
                            (*enc_side).palette_size_ = (*enc_main).palette_size_;
                            memcpy(
                                ((*enc_side).palette_).as_mut_ptr() as *mut libc::c_void,
                                ((*enc_main).palette_).as_mut_ptr() as *const libc::c_void,
                                ::core::mem::size_of::<[uint32_t; 256]>() as libc::c_ulong,
                            );
                            memcpy(
                                ((*enc_side).palette_sorted_).as_mut_ptr()
                                    as *mut libc::c_void,
                                ((*enc_main).palette_sorted_).as_mut_ptr()
                                    as *const libc::c_void,
                                ::core::mem::size_of::<[uint32_t; 256]>() as libc::c_ulong,
                            );
                            (*param).enc_ = enc_side;
                        }
                    }
                }
                ((*worker_interface).Init).expect("non-null function pointer")(worker);
                (*worker).data1 = param as *mut libc::c_void;
                (*worker).data2 = 0 as *mut libc::c_void;
                (*worker)
                    .hook = Some(
                    EncodeStreamHook
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                );
                idx += 1;
                idx;
            }
            match current_block {
                12803755504025630281 => {}
                _ => {
                    if num_crunch_configs_side != 0 as libc::c_int {
                        if ((*worker_interface).Reset)
                            .expect("non-null function pointer")(&mut worker_side) == 0
                        {
                            WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                            current_block = 12803755504025630281;
                        } else {
                            if !((*picture).stats).is_null() {
                                memcpy(
                                    &mut stats_side as *mut WebPAuxStats as *mut libc::c_void,
                                    (*picture).stats as *const libc::c_void,
                                    ::core::mem::size_of::<WebPAuxStats>() as libc::c_ulong,
                                );
                            }
                            ((*worker_interface).Launch)
                                .expect("non-null function pointer")(&mut worker_side);
                            current_block = 1854459640724737493;
                        }
                    } else {
                        current_block = 1854459640724737493;
                    }
                    match current_block {
                        12803755504025630281 => {}
                        _ => {
                            ((*worker_interface).Execute)
                                .expect("non-null function pointer")(&mut worker_main);
                            ok_main = ((*worker_interface).Sync_0)
                                .expect("non-null function pointer")(&mut worker_main);
                            ((*worker_interface).End)
                                .expect("non-null function pointer")(&mut worker_main);
                            if num_crunch_configs_side != 0 as libc::c_int {
                                let ok_side: libc::c_int = ((*worker_interface).Sync_0)
                                    .expect("non-null function pointer")(&mut worker_side);
                                ((*worker_interface).End)
                                    .expect("non-null function pointer")(&mut worker_side);
                                if ok_main == 0 || ok_side == 0 {
                                    if (*picture).error_code as libc::c_uint
                                        == VP8_ENC_OK as libc::c_int as libc::c_uint
                                    {
                                        WebPEncodingSetError(picture, picture_side.error_code);
                                    }
                                } else if VP8LBitWriterNumBytes(&mut bw_side)
                                    < VP8LBitWriterNumBytes(bw_main)
                                {
                                    VP8LBitWriterSwap(bw_main, &mut bw_side);
                                    if !((*picture).stats).is_null() {
                                        memcpy(
                                            (*picture).stats as *mut libc::c_void,
                                            &mut stats_side as *mut WebPAuxStats as *const libc::c_void,
                                            ::core::mem::size_of::<WebPAuxStats>() as libc::c_ulong,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    VP8LBitWriterWipeOut(&mut bw_side);
    VP8LEncoderDelete(enc_main);
    VP8LEncoderDelete(enc_side);
    return ((*picture).error_code as libc::c_uint
        == VP8_ENC_OK as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LEncodeImage(
    config: *const WebPConfig,
    picture: *const WebPPicture,
) -> libc::c_int {
    let mut current_block: u64;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut has_alpha: libc::c_int = 0;
    let mut coded_size: size_t = 0;
    let mut percent: libc::c_int = 0 as libc::c_int;
    let mut initial_size: libc::c_int = 0;
    let mut bw: VP8LBitWriter = VP8LBitWriter {
        bits_: 0,
        used_: 0,
        buf_: 0 as *mut uint8_t,
        cur_: 0 as *mut uint8_t,
        end_: 0 as *mut uint8_t,
        error_: 0,
    };
    if picture.is_null() {
        return 0 as libc::c_int;
    }
    if config.is_null() || ((*picture).argb).is_null() {
        return WebPEncodingSetError(picture, VP8_ENC_ERROR_NULL_PARAMETER);
    }
    width = (*picture).width;
    height = (*picture).height;
    initial_size = if (*config).image_hint as libc::c_uint
        == WEBP_HINT_GRAPH as libc::c_int as libc::c_uint
    {
        width * height
    } else {
        width * height * 2 as libc::c_int
    };
    if VP8LBitWriterInit(&mut bw, initial_size as size_t) == 0 {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    } else {
        if WebPReportProgress(picture, 1 as libc::c_int, &mut percent) == 0 {
            current_block = 18315848641197498845;
        } else {
            if !((*picture).stats).is_null() {
                let stats: *mut WebPAuxStats = (*picture).stats;
                memset(
                    stats as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<WebPAuxStats>() as libc::c_ulong,
                );
                (*stats).PSNR[0 as libc::c_int as usize] = 99.0f32;
                (*stats).PSNR[1 as libc::c_int as usize] = 99.0f32;
                (*stats).PSNR[2 as libc::c_int as usize] = 99.0f32;
                (*stats).PSNR[3 as libc::c_int as usize] = 99.0f32;
                (*stats).PSNR[4 as libc::c_int as usize] = 99.0f32;
            }
            if WriteImageSize(picture, &mut bw) == 0 {
                WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                current_block = 8385515747062866043;
            } else {
                has_alpha = WebPPictureHasTransparency(picture);
                if WriteRealAlphaAndVersion(&mut bw, has_alpha) == 0 {
                    WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
                    current_block = 8385515747062866043;
                } else if WebPReportProgress(picture, 2 as libc::c_int, &mut percent)
                    == 0
                {
                    current_block = 18315848641197498845;
                } else if VP8LEncodeStream(config, picture, &mut bw) == 0 {
                    current_block = 8385515747062866043;
                } else if WebPReportProgress(picture, 99 as libc::c_int, &mut percent)
                    == 0
                {
                    current_block = 18315848641197498845;
                } else if WriteImage(picture, &mut bw, &mut coded_size) == 0 {
                    current_block = 8385515747062866043;
                } else if WebPReportProgress(picture, 100 as libc::c_int, &mut percent)
                    == 0
                {
                    current_block = 18315848641197498845;
                } else {
                    if !((*picture).stats).is_null() {
                        (*(*picture).stats).coded_size += coded_size as libc::c_int;
                        (*(*picture).stats).lossless_size = coded_size as libc::c_int;
                    }
                    if !((*picture).extra_info).is_null() {
                        let mb_w: libc::c_int = width + 15 as libc::c_int
                            >> 4 as libc::c_int;
                        let mb_h: libc::c_int = height + 15 as libc::c_int
                            >> 4 as libc::c_int;
                        memset(
                            (*picture).extra_info as *mut libc::c_void,
                            0 as libc::c_int,
                            ((mb_w * mb_h) as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                                ),
                        );
                    }
                    current_block = 8385515747062866043;
                }
            }
        }
        match current_block {
            8385515747062866043 => {}
            _ => {
                WebPEncodingSetError(picture, VP8_ENC_ERROR_USER_ABORT);
            }
        }
    }
    if bw.error_ != 0 {
        WebPEncodingSetError(picture, VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    VP8LBitWriterWipeOut(&mut bw);
    return ((*picture).error_code as libc::c_uint
        == VP8_ENC_OK as libc::c_int as libc::c_uint) as libc::c_int;
}
