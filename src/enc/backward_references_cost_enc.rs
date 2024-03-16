use ::libc;

use super::backward_references_enc::PixOrCopyBlock;
extern "C" {
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static kPrefixEncodeCode: [VP8LPrefixCode; 512];
    static mut VP8LFastLog2Slow: VP8LFastLog2SlowFunc;
    static kLog2Table: [libc::c_float; 256];
    fn VP8LRefsCursorInit(refs: *const VP8LBackwardRefs) -> VP8LRefsCursor;
    fn VP8LRefsCursorNextBlock(c: *mut VP8LRefsCursor);
    fn VP8LHistogramInit(
        p: *mut VP8LHistogram,
        palette_code_bits: libc::c_int,
        init_arrays: libc::c_int,
    );
    fn VP8LFreeHistogram(histo: *mut VP8LHistogram);
    fn VP8LAllocateHistogram(cache_bits: libc::c_int) -> *mut VP8LHistogram;
    fn VP8LHistogramAddSinglePixOrCopy(
        histo: *mut VP8LHistogram,
        v: *const PixOrCopy,
        distance_modifier: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        >,
        distance_modifier_arg0: libc::c_int,
    );
    fn VP8LColorCacheInit(
        color_cache: *mut VP8LColorCache,
        hash_bits: libc::c_int,
    ) -> libc::c_int;
    fn VP8LColorCacheClear(color_cache: *mut VP8LColorCache);
    fn VP8LClearBackwardRefs(refs: *mut VP8LBackwardRefs);
    fn VP8LDistanceToPlaneCode(xsize: libc::c_int, dist: libc::c_int) -> libc::c_int;
    fn VP8LBackwardRefsCursorAdd(refs: *mut VP8LBackwardRefs, v: PixOrCopy);
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
pub type VP8LFastLog2SlowFunc = Option::<
    unsafe extern "C" fn(uint32_t) -> libc::c_float,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LPrefixCode {
    pub code_: int8_t,
    pub extra_bits_: int8_t,
}
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
pub struct CostModel {
    pub alpha_: [libc::c_float; 256],
    pub red_: [libc::c_float; 256],
    pub blue_: [libc::c_float; 256],
    pub distance_: [libc::c_float; 40],
    pub literal_: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CostInterval {
    pub cost_: libc::c_float,
    pub start_: libc::c_int,
    pub end_: libc::c_int,
    pub index_: libc::c_int,
    pub previous_: *mut CostInterval,
    pub next_: *mut CostInterval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CostCacheInterval {
    pub cost_: libc::c_float,
    pub start_: libc::c_int,
    pub end_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CostManager {
    pub head_: *mut CostInterval,
    pub count_: libc::c_int,
    pub cache_intervals_: *mut CostCacheInterval,
    pub cache_intervals_size_: size_t,
    pub cost_cache_: [libc::c_float; 4095],
    pub costs_: *mut libc::c_float,
    pub dist_array_: *mut uint16_t,
    pub intervals_: [CostInterval; 10],
    pub free_intervals_: *mut CostInterval,
    pub recycled_intervals_: *mut CostInterval,
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
#[inline]
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> libc::c_int {
    return 31 as libc::c_int ^ n.leading_zeros() as i32;
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
unsafe extern "C" fn VP8LFastLog2(mut v: uint32_t) -> libc::c_float {
    return if v < 256 as libc::c_int as libc::c_uint {
        kLog2Table[v as usize]
    } else {
        VP8LFastLog2Slow.expect("non-null function pointer")(v)
    };
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
unsafe extern "C" fn VP8LHashChainFindLength(
    p: *const VP8LHashChain,
    base_position: libc::c_int,
) -> libc::c_int {
    return (*((*p).offset_length_).offset(base_position as isize)
        & ((1 as libc::c_uint) << 12 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)) as libc::c_int;
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
static mut kHashMul: uint32_t = 0x1e35a7bd as libc::c_uint;
#[inline]
unsafe extern "C" fn VP8LHashPix(
    mut argb: uint32_t,
    mut shift: libc::c_int,
) -> libc::c_int {
    return (argb.wrapping_mul(kHashMul) >> shift) as libc::c_int;
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
unsafe extern "C" fn ConvertPopulationCountTableToBitEstimates(
    mut num_symbols: libc::c_int,
    mut population_counts: *const uint32_t,
    mut output: *mut libc::c_float,
) {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut nonzeros: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_symbols {
        sum = (sum as libc::c_uint).wrapping_add(*population_counts.offset(i as isize))
            as uint32_t as uint32_t;
        if *population_counts.offset(i as isize) > 0 as libc::c_int as libc::c_uint {
            nonzeros += 1;
            nonzeros;
        }
        i += 1;
        i;
    }
    if nonzeros <= 1 as libc::c_int {
        memset(
            output as *mut libc::c_void,
            0 as libc::c_int,
            (num_symbols as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
    } else {
        let logsum: libc::c_float = VP8LFastLog2(sum);
        i = 0 as libc::c_int;
        while i < num_symbols {
            *output
                .offset(
                    i as isize,
                ) = logsum - VP8LFastLog2(*population_counts.offset(i as isize));
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn CostModelBuild(
    m: *mut CostModel,
    mut xsize: libc::c_int,
    mut cache_bits: libc::c_int,
    refs: *const VP8LBackwardRefs,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut c: VP8LRefsCursor = VP8LRefsCursorInit(refs);
    let histo: *mut VP8LHistogram = VP8LAllocateHistogram(cache_bits);
    if !histo.is_null() {
        VP8LHistogramInit(histo, cache_bits, 1 as libc::c_int);
        while VP8LRefsCursorOk(&mut c) != 0 {
            VP8LHistogramAddSinglePixOrCopy(
                histo,
                c.cur_pos,
                Some(
                    VP8LDistanceToPlaneCode
                        as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
                ),
                xsize,
            );
            VP8LRefsCursorNext(&mut c);
        }
        ConvertPopulationCountTableToBitEstimates(
            VP8LHistogramNumCodes((*histo).palette_code_bits_),
            (*histo).literal_ as *const uint32_t,
            (*m).literal_,
        );
        ConvertPopulationCountTableToBitEstimates(
            256 as libc::c_int,
            ((*histo).red_).as_mut_ptr() as *const uint32_t,
            ((*m).red_).as_mut_ptr(),
        );
        ConvertPopulationCountTableToBitEstimates(
            256 as libc::c_int,
            ((*histo).blue_).as_mut_ptr() as *const uint32_t,
            ((*m).blue_).as_mut_ptr(),
        );
        ConvertPopulationCountTableToBitEstimates(
            256 as libc::c_int,
            ((*histo).alpha_).as_mut_ptr() as *const uint32_t,
            ((*m).alpha_).as_mut_ptr(),
        );
        ConvertPopulationCountTableToBitEstimates(
            40 as libc::c_int,
            ((*histo).distance_).as_mut_ptr() as *const uint32_t,
            ((*m).distance_).as_mut_ptr(),
        );
        ok = 1 as libc::c_int;
    }
    VP8LFreeHistogram(histo);
    return ok;
}
#[inline]
unsafe extern "C" fn GetLiteralCost(
    m: *const CostModel,
    mut v: uint32_t,
) -> libc::c_float {
    return (*m).alpha_[(v >> 24 as libc::c_int) as usize]
        + (*m)
            .red_[(v >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as usize]
        + *((*m).literal_)
            .offset(
                (v >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as isize,
            ) + (*m).blue_[(v & 0xff as libc::c_int as libc::c_uint) as usize];
}
#[inline]
unsafe extern "C" fn GetCacheCost(
    m: *const CostModel,
    mut idx: uint32_t,
) -> libc::c_float {
    let literal_idx: libc::c_int = ((256 as libc::c_int + 24 as libc::c_int)
        as libc::c_uint)
        .wrapping_add(idx) as libc::c_int;
    return *((*m).literal_).offset(literal_idx as isize);
}
#[inline]
unsafe extern "C" fn GetLengthCost(
    m: *const CostModel,
    mut length: uint32_t,
) -> libc::c_float {
    let mut code: libc::c_int = 0;
    let mut extra_bits: libc::c_int = 0;
    VP8LPrefixEncodeBits(length as libc::c_int, &mut code, &mut extra_bits);
    return *((*m).literal_).offset((256 as libc::c_int + code) as isize)
        + extra_bits as libc::c_float;
}
#[inline]
unsafe extern "C" fn GetDistanceCost(
    m: *const CostModel,
    mut distance: uint32_t,
) -> libc::c_float {
    let mut code: libc::c_int = 0;
    let mut extra_bits: libc::c_int = 0;
    VP8LPrefixEncodeBits(distance as libc::c_int, &mut code, &mut extra_bits);
    return (*m).distance_[code as usize] + extra_bits as libc::c_float;
}
#[inline]
unsafe extern "C" fn AddSingleLiteralWithCostModel(
    argb: *const uint32_t,
    hashers: *mut VP8LColorCache,
    cost_model: *const CostModel,
    mut idx: libc::c_int,
    mut use_color_cache: libc::c_int,
    mut prev_cost: libc::c_float,
    cost: *mut libc::c_float,
    dist_array: *mut uint16_t,
) {
    let mut cost_val: libc::c_float = prev_cost;
    let color: uint32_t = *argb.offset(idx as isize);
    let ix: libc::c_int = if use_color_cache != 0 {
        VP8LColorCacheContains(hashers, color)
    } else {
        -(1 as libc::c_int)
    };
    if ix >= 0 as libc::c_int {
        let mul0: libc::c_float = 0.68f32;
        cost_val += GetCacheCost(cost_model, ix as uint32_t) * mul0;
    } else {
        let mul1: libc::c_float = 0.82f32;
        if use_color_cache != 0 {
            VP8LColorCacheInsert(hashers, color);
        }
        cost_val += GetLiteralCost(cost_model, color) * mul1;
    }
    if *cost.offset(idx as isize) > cost_val {
        *cost.offset(idx as isize) = cost_val;
        *dist_array.offset(idx as isize) = 1 as libc::c_int as uint16_t;
    }
}
unsafe extern "C" fn CostIntervalAddToFreeList(
    manager: *mut CostManager,
    interval: *mut CostInterval,
) {
    (*interval).next_ = (*manager).free_intervals_;
    (*manager).free_intervals_ = interval;
}
unsafe extern "C" fn CostIntervalIsInFreeList(
    manager: *const CostManager,
    interval: *const CostInterval,
) -> libc::c_int {
    return (interval
        >= &*((*manager).intervals_).as_ptr().offset(0 as libc::c_int as isize)
            as *const CostInterval
        && interval
            <= &*((*manager).intervals_)
                .as_ptr()
                .offset((10 as libc::c_int - 1 as libc::c_int) as isize)
                as *const CostInterval) as libc::c_int;
}
unsafe extern "C" fn CostManagerInitFreeList(manager: *mut CostManager) {
    let mut i: libc::c_int = 0;
    (*manager).free_intervals_ = 0 as *mut CostInterval;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        CostIntervalAddToFreeList(
            manager,
            &mut *((*manager).intervals_).as_mut_ptr().offset(i as isize),
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn DeleteIntervalList(
    manager: *mut CostManager,
    mut interval: *const CostInterval,
) {
    while !interval.is_null() {
        let next: *const CostInterval = (*interval).next_;
        if CostIntervalIsInFreeList(manager, interval) == 0 {
            WebPSafeFree(interval as *mut libc::c_void);
        }
        interval = next;
    }
}
unsafe extern "C" fn CostManagerClear(manager: *mut CostManager) {
    if manager.is_null() {
        return;
    }
    WebPSafeFree((*manager).costs_ as *mut libc::c_void);
    WebPSafeFree((*manager).cache_intervals_ as *mut libc::c_void);
    DeleteIntervalList(manager, (*manager).head_);
    (*manager).head_ = 0 as *mut CostInterval;
    DeleteIntervalList(manager, (*manager).recycled_intervals_);
    (*manager).recycled_intervals_ = 0 as *mut CostInterval;
    memset(
        manager as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<CostManager>() as libc::c_ulong,
    );
    CostManagerInitFreeList(manager);
}
unsafe extern "C" fn CostManagerInit(
    manager: *mut CostManager,
    dist_array: *mut uint16_t,
    mut pix_count: libc::c_int,
    cost_model: *const CostModel,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let cost_cache_size: libc::c_int = if pix_count
        > ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int
    {
        ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int
    } else {
        pix_count
    };
    (*manager).costs_ = 0 as *mut libc::c_float;
    (*manager).cache_intervals_ = 0 as *mut CostCacheInterval;
    (*manager).head_ = 0 as *mut CostInterval;
    (*manager).recycled_intervals_ = 0 as *mut CostInterval;
    (*manager).count_ = 0 as libc::c_int;
    (*manager).dist_array_ = dist_array;
    CostManagerInitFreeList(manager);
    i = 0 as libc::c_int;
    while i < cost_cache_size {
        (*manager).cost_cache_[i as usize] = GetLengthCost(cost_model, i as uint32_t);
        i += 1;
        i;
    }
    (*manager).cache_intervals_size_ = 1 as libc::c_int as size_t;
    i = 1 as libc::c_int;
    while i < cost_cache_size {
        if (*manager).cost_cache_[i as usize]
            != (*manager).cost_cache_[(i - 1 as libc::c_int) as usize]
        {
            (*manager)
                .cache_intervals_size_ = ((*manager).cache_intervals_size_)
                .wrapping_add(1);
            (*manager).cache_intervals_size_;
        }
        i += 1;
        i;
    }
    (*manager)
        .cache_intervals_ = WebPSafeMalloc(
        (*manager).cache_intervals_size_,
        ::core::mem::size_of::<CostCacheInterval>() as libc::c_ulong,
    ) as *mut CostCacheInterval;
    if ((*manager).cache_intervals_).is_null() {
        CostManagerClear(manager);
        return 0 as libc::c_int;
    }
    let mut cur: *mut CostCacheInterval = (*manager).cache_intervals_;
    (*cur).start_ = 0 as libc::c_int;
    (*cur).end_ = 1 as libc::c_int;
    (*cur).cost_ = (*manager).cost_cache_[0 as libc::c_int as usize];
    i = 1 as libc::c_int;
    while i < cost_cache_size {
        let cost_val: libc::c_float = (*manager).cost_cache_[i as usize];
        if cost_val != (*cur).cost_ {
            cur = cur.offset(1);
            cur;
            (*cur).start_ = i;
            (*cur).cost_ = cost_val;
        }
        (*cur).end_ = i + 1 as libc::c_int;
        i += 1;
        i;
    }
    (*manager)
        .costs_ = WebPSafeMalloc(
        pix_count as uint64_t,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    if ((*manager).costs_).is_null() {
        CostManagerClear(manager);
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < pix_count {
        *((*manager).costs_).offset(i as isize) = 3.40282347e+38f32;
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn UpdateCost(
    manager: *mut CostManager,
    mut i: libc::c_int,
    mut position: libc::c_int,
    mut cost: libc::c_float,
) {
    let k: libc::c_int = i - position;
    if *((*manager).costs_).offset(i as isize) > cost {
        *((*manager).costs_).offset(i as isize) = cost;
        *((*manager).dist_array_)
            .offset(i as isize) = (k + 1 as libc::c_int) as uint16_t;
    }
}
#[inline]
unsafe extern "C" fn UpdateCostPerInterval(
    manager: *mut CostManager,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut position: libc::c_int,
    mut cost: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = start;
    while i < end {
        UpdateCost(manager, i, position, cost);
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn ConnectIntervals(
    manager: *mut CostManager,
    prev: *mut CostInterval,
    next: *mut CostInterval,
) {
    if !prev.is_null() {
        (*prev).next_ = next;
    } else {
        (*manager).head_ = next;
    }
    if !next.is_null() {
        (*next).previous_ = prev;
    }
}
#[inline]
unsafe extern "C" fn PopInterval(
    manager: *mut CostManager,
    interval: *mut CostInterval,
) {
    if interval.is_null() {
        return;
    }
    ConnectIntervals(manager, (*interval).previous_, (*interval).next_);
    if CostIntervalIsInFreeList(manager, interval) != 0 {
        CostIntervalAddToFreeList(manager, interval);
    } else {
        (*interval).next_ = (*manager).recycled_intervals_;
        (*manager).recycled_intervals_ = interval;
    }
    (*manager).count_ -= 1;
    (*manager).count_;
}
#[inline]
unsafe extern "C" fn UpdateCostAtIndex(
    manager: *mut CostManager,
    mut i: libc::c_int,
    mut do_clean_intervals: libc::c_int,
) {
    let mut current: *mut CostInterval = (*manager).head_;
    while !current.is_null() && (*current).start_ <= i {
        let next: *mut CostInterval = (*current).next_;
        if (*current).end_ <= i {
            if do_clean_intervals != 0 {
                PopInterval(manager, current);
            }
        } else {
            UpdateCost(manager, i, (*current).index_, (*current).cost_);
        }
        current = next;
    }
}
#[inline]
unsafe extern "C" fn PositionOrphanInterval(
    manager: *mut CostManager,
    current: *mut CostInterval,
    mut previous: *mut CostInterval,
) {
    if previous.is_null() {
        previous = (*manager).head_;
    }
    while !previous.is_null() && (*current).start_ < (*previous).start_ {
        previous = (*previous).previous_;
    }
    while !previous.is_null() && !((*previous).next_).is_null()
        && (*(*previous).next_).start_ < (*current).start_
    {
        previous = (*previous).next_;
    }
    if !previous.is_null() {
        ConnectIntervals(manager, current, (*previous).next_);
    } else {
        ConnectIntervals(manager, current, (*manager).head_);
    }
    ConnectIntervals(manager, previous, current);
}
#[inline]
unsafe extern "C" fn InsertInterval(
    manager: *mut CostManager,
    interval_in: *mut CostInterval,
    mut cost: libc::c_float,
    mut position: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
) {
    let mut interval_new: *mut CostInterval = 0 as *mut CostInterval;
    if start >= end {
        return;
    }
    if (*manager).count_ >= 500 as libc::c_int {
        UpdateCostPerInterval(manager, start, end, position, cost);
        return;
    }
    if !((*manager).free_intervals_).is_null() {
        interval_new = (*manager).free_intervals_;
        (*manager).free_intervals_ = (*interval_new).next_;
    } else if !((*manager).recycled_intervals_).is_null() {
        interval_new = (*manager).recycled_intervals_;
        (*manager).recycled_intervals_ = (*interval_new).next_;
    } else {
        interval_new = WebPSafeMalloc(
            1 as libc::c_int as uint64_t,
            ::core::mem::size_of::<CostInterval>() as libc::c_ulong,
        ) as *mut CostInterval;
        if interval_new.is_null() {
            UpdateCostPerInterval(manager, start, end, position, cost);
            return;
        }
    }
    (*interval_new).cost_ = cost;
    (*interval_new).index_ = position;
    (*interval_new).start_ = start;
    (*interval_new).end_ = end;
    PositionOrphanInterval(manager, interval_new, interval_in);
    (*manager).count_ += 1;
    (*manager).count_;
}
#[inline]
unsafe extern "C" fn PushInterval(
    manager: *mut CostManager,
    mut distance_cost: libc::c_float,
    mut position: libc::c_int,
    mut len: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut interval: *mut CostInterval = (*manager).head_;
    let mut interval_next: *mut CostInterval = 0 as *mut CostInterval;
    let cost_cache_intervals: *const CostCacheInterval = (*manager).cache_intervals_;
    let kSkipDistance: libc::c_int = 10 as libc::c_int;
    if len < kSkipDistance {
        let mut j: libc::c_int = 0;
        j = position;
        while j < position + len {
            let k: libc::c_int = j - position;
            let mut cost_tmp: libc::c_float = 0.;
            cost_tmp = distance_cost + (*manager).cost_cache_[k as usize];
            if *((*manager).costs_).offset(j as isize) > cost_tmp {
                *((*manager).costs_).offset(j as isize) = cost_tmp;
                *((*manager).dist_array_)
                    .offset(j as isize) = (k + 1 as libc::c_int) as uint16_t;
            }
            j += 1;
            j;
        }
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*manager).cache_intervals_size_
        && (*cost_cache_intervals.offset(i as isize)).start_ < len
    {
        let mut start: libc::c_int = position
            + (*cost_cache_intervals.offset(i as isize)).start_;
        let end: libc::c_int = position
            + (if (*cost_cache_intervals.offset(i as isize)).end_ > len {
                len
            } else {
                (*cost_cache_intervals.offset(i as isize)).end_
            });
        let cost: libc::c_float = distance_cost
            + (*cost_cache_intervals.offset(i as isize)).cost_;
        while !interval.is_null() && (*interval).start_ < end {
            interval_next = (*interval).next_;
            if !(start >= (*interval).end_) {
                if cost >= (*interval).cost_ {
                    let start_new: libc::c_int = (*interval).end_;
                    InsertInterval(
                        manager,
                        interval,
                        cost,
                        position,
                        start,
                        (*interval).start_,
                    );
                    start = start_new;
                    if start >= end {
                        break;
                    }
                } else if start <= (*interval).start_ {
                    if (*interval).end_ <= end {
                        PopInterval(manager, interval);
                    } else {
                        (*interval).start_ = end;
                        break;
                    }
                } else if end < (*interval).end_ {
                    let end_original: libc::c_int = (*interval).end_;
                    (*interval).end_ = start;
                    InsertInterval(
                        manager,
                        interval,
                        (*interval).cost_,
                        (*interval).index_,
                        end,
                        end_original,
                    );
                    interval = (*interval).next_;
                    break;
                } else {
                    (*interval).end_ = start;
                }
            }
            interval = interval_next;
        }
        InsertInterval(manager, interval, cost, position, start, end);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn BackwardReferencesHashChainDistanceOnly(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    argb: *const uint32_t,
    mut cache_bits: libc::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *const VP8LBackwardRefs,
    dist_array: *mut uint16_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut cc_init: libc::c_int = 0 as libc::c_int;
    let pix_count: libc::c_int = xsize * ysize;
    let use_color_cache: libc::c_int = (cache_bits > 0 as libc::c_int) as libc::c_int;
    let literal_array_size: size_t = (::core::mem::size_of::<libc::c_float>()
        as libc::c_ulong)
        .wrapping_mul(VP8LHistogramNumCodes(cache_bits) as libc::c_ulong);
    let cost_model_size: size_t = (::core::mem::size_of::<CostModel>() as libc::c_ulong)
        .wrapping_add(literal_array_size);
    let cost_model: *mut CostModel = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        cost_model_size,
    ) as *mut CostModel;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: 0 as *mut uint32_t,
        hash_shift_: 0,
        hash_bits_: 0,
    };
    let mut cost_manager: *mut CostManager = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<CostManager>() as libc::c_ulong,
    ) as *mut CostManager;
    let mut offset_prev: libc::c_int = -(1 as libc::c_int);
    let mut len_prev: libc::c_int = -(1 as libc::c_int);
    let mut offset_cost: libc::c_float = -1.0f32;
    let mut first_offset_is_constant: libc::c_int = -(1 as libc::c_int);
    let mut reach: libc::c_int = 0 as libc::c_int;
    if !(cost_model.is_null() || cost_manager.is_null()) {
        (*cost_model)
            .literal_ = cost_model.offset(1 as libc::c_int as isize)
            as *mut libc::c_float;
        if use_color_cache != 0 {
            cc_init = VP8LColorCacheInit(&mut hashers, cache_bits);
            if cc_init == 0 {
                current_block = 9906763305750574563;
            } else {
                current_block = 11812396948646013369;
            }
        } else {
            current_block = 11812396948646013369;
        }
        match current_block {
            9906763305750574563 => {}
            _ => {
                if !(CostModelBuild(cost_model, xsize, cache_bits, refs) == 0) {
                    if !(CostManagerInit(cost_manager, dist_array, pix_count, cost_model)
                        == 0)
                    {
                        *dist_array
                            .offset(
                                0 as libc::c_int as isize,
                            ) = 0 as libc::c_int as uint16_t;
                        AddSingleLiteralWithCostModel(
                            argb,
                            &mut hashers,
                            cost_model,
                            0 as libc::c_int,
                            use_color_cache,
                            0.0f32,
                            (*cost_manager).costs_,
                            dist_array,
                        );
                        i = 1 as libc::c_int;
                        while i < pix_count {
                            let prev_cost: libc::c_float = *((*cost_manager).costs_)
                                .offset((i - 1 as libc::c_int) as isize);
                            let mut offset: libc::c_int = 0;
                            let mut len: libc::c_int = 0;
                            VP8LHashChainFindCopy(hash_chain, i, &mut offset, &mut len);
                            AddSingleLiteralWithCostModel(
                                argb,
                                &mut hashers,
                                cost_model,
                                i,
                                use_color_cache,
                                prev_cost,
                                (*cost_manager).costs_,
                                dist_array,
                            );
                            if len >= 2 as libc::c_int {
                                if offset != offset_prev {
                                    let code: libc::c_int = VP8LDistanceToPlaneCode(
                                        xsize,
                                        offset,
                                    );
                                    offset_cost = GetDistanceCost(cost_model, code as uint32_t);
                                    first_offset_is_constant = 1 as libc::c_int;
                                    PushInterval(cost_manager, prev_cost + offset_cost, i, len);
                                } else {
                                    if first_offset_is_constant != 0 {
                                        reach = i - 1 as libc::c_int + len_prev - 1 as libc::c_int;
                                        first_offset_is_constant = 0 as libc::c_int;
                                    }
                                    if i + len - 1 as libc::c_int > reach {
                                        let mut offset_j: libc::c_int = 0;
                                        let mut len_j: libc::c_int = 0 as libc::c_int;
                                        let mut j: libc::c_int = 0;
                                        j = i;
                                        while j <= reach {
                                            VP8LHashChainFindCopy(
                                                hash_chain,
                                                j + 1 as libc::c_int,
                                                &mut offset_j,
                                                &mut len_j,
                                            );
                                            if offset_j != offset {
                                                VP8LHashChainFindCopy(
                                                    hash_chain,
                                                    j,
                                                    &mut offset_j,
                                                    &mut len_j,
                                                );
                                                break;
                                            } else {
                                                j += 1;
                                                j;
                                            }
                                        }
                                        UpdateCostAtIndex(
                                            cost_manager,
                                            j - 1 as libc::c_int,
                                            0 as libc::c_int,
                                        );
                                        UpdateCostAtIndex(cost_manager, j, 0 as libc::c_int);
                                        PushInterval(
                                            cost_manager,
                                            *((*cost_manager).costs_)
                                                .offset((j - 1 as libc::c_int) as isize) + offset_cost,
                                            j,
                                            len_j,
                                        );
                                        reach = j + len_j - 1 as libc::c_int;
                                    }
                                }
                            }
                            UpdateCostAtIndex(cost_manager, i, 1 as libc::c_int);
                            offset_prev = offset;
                            len_prev = len;
                            i += 1;
                            i;
                        }
                        ok = ((*refs).error_ == 0) as libc::c_int;
                    }
                }
            }
        }
    }
    if cc_init != 0 {
        VP8LColorCacheClear(&mut hashers);
    }
    CostManagerClear(cost_manager);
    WebPSafeFree(cost_model as *mut libc::c_void);
    WebPSafeFree(cost_manager as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn TraceBackwards(
    dist_array: *mut uint16_t,
    mut dist_array_size: libc::c_int,
    chosen_path: *mut *mut uint16_t,
    chosen_path_size: *mut libc::c_int,
) {
    let mut path: *mut uint16_t = dist_array.offset(dist_array_size as isize);
    let mut cur: *mut uint16_t = dist_array
        .offset(dist_array_size as isize)
        .offset(-(1 as libc::c_int as isize));
    while cur >= dist_array {
        let k: libc::c_int = *cur as libc::c_int;
        path = path.offset(-1);
        path;
        *path = k as uint16_t;
        cur = cur.offset(-(k as isize));
    }
    *chosen_path = path;
    *chosen_path_size = dist_array.offset(dist_array_size as isize).offset_from(path)
        as libc::c_long as libc::c_int;
}
unsafe extern "C" fn BackwardReferencesHashChainFollowChosenPath(
    argb: *const uint32_t,
    mut cache_bits: libc::c_int,
    chosen_path: *const uint16_t,
    mut chosen_path_size: libc::c_int,
    hash_chain: *const VP8LHashChain,
    refs: *mut VP8LBackwardRefs,
) -> libc::c_int {
    let mut current_block: u64;
    let use_color_cache: libc::c_int = (cache_bits > 0 as libc::c_int) as libc::c_int;
    let mut ix: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut cc_init: libc::c_int = 0 as libc::c_int;
    let mut hashers: VP8LColorCache = VP8LColorCache {
        colors_: 0 as *mut uint32_t,
        hash_shift_: 0,
        hash_bits_: 0,
    };
    if use_color_cache != 0 {
        cc_init = VP8LColorCacheInit(&mut hashers, cache_bits);
        if cc_init == 0 {
            current_block = 15904598633091914272;
        } else {
            current_block = 7095457783677275021;
        }
    } else {
        current_block = 7095457783677275021;
    }
    match current_block {
        7095457783677275021 => {
            VP8LClearBackwardRefs(refs);
            ix = 0 as libc::c_int;
            while ix < chosen_path_size {
                let len: libc::c_int = *chosen_path.offset(ix as isize) as libc::c_int;
                if len != 1 as libc::c_int {
                    let mut k: libc::c_int = 0;
                    let offset: libc::c_int = VP8LHashChainFindOffset(hash_chain, i);
                    VP8LBackwardRefsCursorAdd(
                        refs,
                        PixOrCopyCreateCopy(offset as uint32_t, len as uint16_t),
                    );
                    if use_color_cache != 0 {
                        k = 0 as libc::c_int;
                        while k < len {
                            VP8LColorCacheInsert(
                                &mut hashers,
                                *argb.offset((i + k) as isize),
                            );
                            k += 1;
                            k;
                        }
                    }
                    i += len;
                } else {
                    let mut v: PixOrCopy = PixOrCopy {
                        mode: 0,
                        len: 0,
                        argb_or_distance: 0,
                    };
                    let idx: libc::c_int = if use_color_cache != 0 {
                        VP8LColorCacheContains(&mut hashers, *argb.offset(i as isize))
                    } else {
                        -(1 as libc::c_int)
                    };
                    if idx >= 0 as libc::c_int {
                        v = PixOrCopyCreateCacheIdx(idx);
                    } else {
                        if use_color_cache != 0 {
                            VP8LColorCacheInsert(&mut hashers, *argb.offset(i as isize));
                        }
                        v = PixOrCopyCreateLiteral(*argb.offset(i as isize));
                    }
                    VP8LBackwardRefsCursorAdd(refs, v);
                    i += 1;
                    i;
                }
                ix += 1;
                ix;
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
#[no_mangle]
pub unsafe extern "C" fn VP8LBackwardReferencesTraceBackwards(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    argb: *const uint32_t,
    mut cache_bits: libc::c_int,
    hash_chain: *const VP8LHashChain,
    refs_src: *const VP8LBackwardRefs,
    refs_dst: *mut VP8LBackwardRefs,
) -> libc::c_int {
    let mut ok: libc::c_int = 0 as libc::c_int;
    let dist_array_size: libc::c_int = xsize * ysize;
    let mut chosen_path: *mut uint16_t = 0 as *mut uint16_t;
    let mut chosen_path_size: libc::c_int = 0 as libc::c_int;
    let mut dist_array: *mut uint16_t = WebPSafeMalloc(
        dist_array_size as uint64_t,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    ) as *mut uint16_t;
    if !dist_array.is_null() {
        if !(BackwardReferencesHashChainDistanceOnly(
            xsize,
            ysize,
            argb,
            cache_bits,
            hash_chain,
            refs_src,
            dist_array,
        ) == 0)
        {
            TraceBackwards(
                dist_array,
                dist_array_size,
                &mut chosen_path,
                &mut chosen_path_size,
            );
            if !(BackwardReferencesHashChainFollowChosenPath(
                argb,
                cache_bits,
                chosen_path,
                chosen_path_size,
                hash_chain,
                refs_dst,
            ) == 0)
            {
                ok = 1 as libc::c_int;
            }
        }
    }
    WebPSafeFree(dist_array as *mut libc::c_void);
    return ok;
}
