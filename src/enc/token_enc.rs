use ::libc;
extern "C" {
    static VP8EntropyCost: [uint16_t; 256];
    static VP8EncBands: [uint8_t; 17];
    static VP8Cat3: [uint8_t; 0];
    fn VP8PutBit(
        bw: *mut VP8BitWriter,
        bit: libc::c_int,
        prob: libc::c_int,
    ) -> libc::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    static VP8Cat4: [uint8_t; 0];
    static VP8Cat5: [uint8_t; 0];
    static VP8Cat6: [uint8_t; 0];
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Residual {
    pub first: libc::c_int,
    pub last: libc::c_int,
    pub coeffs: *const int16_t,
    pub coeff_type: libc::c_int,
    pub prob: *mut ProbaArray,
    pub stats: *mut StatsArray,
    pub costs: CostArrayPtr,
}
pub type CostArrayPtr = *mut [*const uint16_t; 3];
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
pub struct VP8Tokens {
    pub next_: *mut VP8Tokens,
}
pub type token_t = uint16_t;
#[inline]
unsafe extern "C" fn VP8BitCost(
    mut bit: libc::c_int,
    mut proba: uint8_t,
) -> libc::c_int {
    return if bit == 0 {
        VP8EntropyCost[proba as usize] as libc::c_int
    } else {
        VP8EntropyCost[(255 as libc::c_int - proba as libc::c_int) as usize]
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn VP8RecordStats(
    mut bit: libc::c_int,
    stats: *mut proba_t,
) -> libc::c_int {
    let mut p: proba_t = *stats;
    if p >= 0xfffe0000 as libc::c_uint {
        p = p.wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int
            & 0x7fff7fff as libc::c_uint;
    }
    p = (p as libc::c_uint)
        .wrapping_add((0x10000 as libc::c_uint).wrapping_add(bit as libc::c_uint))
        as proba_t as proba_t;
    *stats = p;
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn VP8TBufferInit(b: *mut VP8TBuffer, mut page_size: libc::c_int) {
    (*b).tokens_ = 0 as *mut uint16_t;
    (*b).pages_ = 0 as *mut VP8Tokens;
    (*b).last_page_ = &mut (*b).pages_;
    (*b).left_ = 0 as libc::c_int;
    (*b)
        .page_size_ = if page_size < 8192 as libc::c_int {
        8192 as libc::c_int
    } else {
        page_size
    };
    (*b).error_ = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8TBufferClear(b: *mut VP8TBuffer) {
    if !b.is_null() {
        let mut p: *mut VP8Tokens = (*b).pages_;
        while !p.is_null() {
            let next: *mut VP8Tokens = (*p).next_;
            WebPSafeFree(p as *mut libc::c_void);
            p = next;
        }
        VP8TBufferInit(b, (*b).page_size_);
    }
}
unsafe extern "C" fn TBufferNewPage(b: *mut VP8TBuffer) -> libc::c_int {
    let mut page: *mut VP8Tokens = 0 as *mut VP8Tokens;
    if (*b).error_ == 0 {
        let size: size_t = (::core::mem::size_of::<VP8Tokens>() as libc::c_ulong)
            .wrapping_add(
                ((*b).page_size_ as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<token_t>() as libc::c_ulong),
            );
        page = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, size)
            as *mut VP8Tokens;
    }
    if page.is_null() {
        (*b).error_ = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*page).next_ = 0 as *mut VP8Tokens;
    *(*b).last_page_ = page;
    (*b).last_page_ = &mut (*page).next_;
    (*b).left_ = (*b).page_size_;
    (*b)
        .tokens_ = &mut *page.offset(1 as libc::c_int as isize) as *mut VP8Tokens
        as *const token_t as *mut token_t;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn AddToken(
    b: *mut VP8TBuffer,
    mut bit: uint32_t,
    mut proba_idx: uint32_t,
    stats: *mut proba_t,
) -> uint32_t {
    if (*b).left_ > 0 as libc::c_int || TBufferNewPage(b) != 0 {
        (*b).left_ -= 1;
        let slot: libc::c_int = (*b).left_;
        *((*b).tokens_)
            .offset(slot as isize) = (bit << 15 as libc::c_int | proba_idx) as uint16_t;
    }
    VP8RecordStats(bit as libc::c_int, stats);
    return bit;
}
#[inline]
unsafe extern "C" fn AddConstantToken(
    b: *mut VP8TBuffer,
    mut bit: uint32_t,
    mut proba: uint32_t,
) {
    if (*b).left_ > 0 as libc::c_int || TBufferNewPage(b) != 0 {
        (*b).left_ -= 1;
        let slot: libc::c_int = (*b).left_;
        *((*b).tokens_)
            .offset(
                slot as isize,
            ) = (bit << 15 as libc::c_int | (1 as libc::c_uint) << 14 as libc::c_int
            | proba) as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8RecordCoeffTokens(
    mut ctx: libc::c_int,
    res: *const VP8Residual,
    tokens: *mut VP8TBuffer,
) -> libc::c_int {
    let coeffs: *const int16_t = (*res).coeffs;
    let coeff_type: libc::c_int = (*res).coeff_type;
    let last: libc::c_int = (*res).last;
    let mut n: libc::c_int = (*res).first;
    let mut base_id: uint32_t = (NUM_PROBAS as libc::c_int
        * (ctx + NUM_CTX as libc::c_int * (n + NUM_BANDS as libc::c_int * coeff_type)))
        as uint32_t;
    let mut s: *mut proba_t = ((*((*res).stats).offset(n as isize))[ctx as usize])
        .as_mut_ptr();
    if AddToken(
        tokens,
        (last >= 0 as libc::c_int) as libc::c_int as uint32_t,
        base_id.wrapping_add(0 as libc::c_int as libc::c_uint),
        s.offset(0 as libc::c_int as isize),
    ) == 0
    {
        return 0 as libc::c_int;
    }
    while n < 16 as libc::c_int {
        let fresh0 = n;
        n = n + 1;
        let c: libc::c_int = *coeffs.offset(fresh0 as isize) as libc::c_int;
        let sign: libc::c_int = (c < 0 as libc::c_int) as libc::c_int;
        let v: uint32_t = (if sign != 0 { -c } else { c }) as uint32_t;
        if AddToken(
            tokens,
            (v != 0 as libc::c_int as libc::c_uint) as libc::c_int as uint32_t,
            base_id.wrapping_add(1 as libc::c_int as libc::c_uint),
            s.offset(1 as libc::c_int as isize),
        ) == 0
        {
            base_id = (NUM_PROBAS as libc::c_int
                * (0 as libc::c_int
                    + NUM_CTX as libc::c_int
                        * (VP8EncBands[n as usize] as libc::c_int
                            + NUM_BANDS as libc::c_int * coeff_type))) as uint32_t;
            s = ((*((*res).stats)
                .offset(VP8EncBands[n as usize] as isize))[0 as libc::c_int as usize])
                .as_mut_ptr();
        } else {
            if AddToken(
                tokens,
                (v > 1 as libc::c_int as libc::c_uint) as libc::c_int as uint32_t,
                base_id.wrapping_add(2 as libc::c_int as libc::c_uint),
                s.offset(2 as libc::c_int as isize),
            ) == 0
            {
                base_id = (NUM_PROBAS as libc::c_int
                    * (1 as libc::c_int
                        + NUM_CTX as libc::c_int
                            * (VP8EncBands[n as usize] as libc::c_int
                                + NUM_BANDS as libc::c_int * coeff_type))) as uint32_t;
                s = ((*((*res).stats)
                    .offset(
                        VP8EncBands[n as usize] as isize,
                    ))[1 as libc::c_int as usize])
                    .as_mut_ptr();
            } else {
                if AddToken(
                    tokens,
                    (v > 4 as libc::c_int as libc::c_uint) as libc::c_int as uint32_t,
                    base_id.wrapping_add(3 as libc::c_int as libc::c_uint),
                    s.offset(3 as libc::c_int as isize),
                ) == 0
                {
                    if AddToken(
                        tokens,
                        (v != 2 as libc::c_int as libc::c_uint) as libc::c_int
                            as uint32_t,
                        base_id.wrapping_add(4 as libc::c_int as libc::c_uint),
                        s.offset(4 as libc::c_int as isize),
                    ) != 0
                    {
                        AddToken(
                            tokens,
                            (v == 4 as libc::c_int as libc::c_uint) as libc::c_int
                                as uint32_t,
                            base_id.wrapping_add(5 as libc::c_int as libc::c_uint),
                            s.offset(5 as libc::c_int as isize),
                        );
                    }
                } else if AddToken(
                    tokens,
                    (v > 10 as libc::c_int as libc::c_uint) as libc::c_int as uint32_t,
                    base_id.wrapping_add(6 as libc::c_int as libc::c_uint),
                    s.offset(6 as libc::c_int as isize),
                ) == 0
                {
                    if AddToken(
                        tokens,
                        (v > 6 as libc::c_int as libc::c_uint) as libc::c_int
                            as uint32_t,
                        base_id.wrapping_add(7 as libc::c_int as libc::c_uint),
                        s.offset(7 as libc::c_int as isize),
                    ) == 0
                    {
                        AddConstantToken(
                            tokens,
                            (v == 6 as libc::c_int as libc::c_uint) as libc::c_int
                                as uint32_t,
                            159 as libc::c_int as uint32_t,
                        );
                    } else {
                        AddConstantToken(
                            tokens,
                            (v >= 9 as libc::c_int as libc::c_uint) as libc::c_int
                                as uint32_t,
                            165 as libc::c_int as uint32_t,
                        );
                        AddConstantToken(
                            tokens,
                            (v & 1 as libc::c_int as libc::c_uint == 0) as libc::c_int
                                as uint32_t,
                            145 as libc::c_int as uint32_t,
                        );
                    }
                } else {
                    let mut mask: libc::c_int = 0;
                    let mut tab: *const uint8_t = 0 as *const uint8_t;
                    let mut residue: uint32_t = v
                        .wrapping_sub(3 as libc::c_int as libc::c_uint);
                    if residue < ((8 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
                    {
                        AddToken(
                            tokens,
                            0 as libc::c_int as uint32_t,
                            base_id.wrapping_add(8 as libc::c_int as libc::c_uint),
                            s.offset(8 as libc::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            0 as libc::c_int as uint32_t,
                            base_id.wrapping_add(9 as libc::c_int as libc::c_uint),
                            s.offset(9 as libc::c_int as isize),
                        );
                        residue = (residue as libc::c_uint)
                            .wrapping_sub(
                                ((8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint,
                            ) as uint32_t as uint32_t;
                        mask = (1 as libc::c_int) << 2 as libc::c_int;
                        tab = VP8Cat3.as_ptr();
                    } else if residue
                        < ((8 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
                    {
                        AddToken(
                            tokens,
                            0 as libc::c_int as uint32_t,
                            base_id.wrapping_add(8 as libc::c_int as libc::c_uint),
                            s.offset(8 as libc::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            1 as libc::c_int as uint32_t,
                            base_id.wrapping_add(9 as libc::c_int as libc::c_uint),
                            s.offset(9 as libc::c_int as isize),
                        );
                        residue = (residue as libc::c_uint)
                            .wrapping_sub(
                                ((8 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                            ) as uint32_t as uint32_t;
                        mask = (1 as libc::c_int) << 3 as libc::c_int;
                        tab = VP8Cat4.as_ptr();
                    } else if residue
                        < ((8 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                    {
                        AddToken(
                            tokens,
                            1 as libc::c_int as uint32_t,
                            base_id.wrapping_add(8 as libc::c_int as libc::c_uint),
                            s.offset(8 as libc::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            0 as libc::c_int as uint32_t,
                            base_id.wrapping_add(10 as libc::c_int as libc::c_uint),
                            s.offset(9 as libc::c_int as isize),
                        );
                        residue = (residue as libc::c_uint)
                            .wrapping_sub(
                                ((8 as libc::c_int) << 2 as libc::c_int) as libc::c_uint,
                            ) as uint32_t as uint32_t;
                        mask = (1 as libc::c_int) << 4 as libc::c_int;
                        tab = VP8Cat5.as_ptr();
                    } else {
                        AddToken(
                            tokens,
                            1 as libc::c_int as uint32_t,
                            base_id.wrapping_add(8 as libc::c_int as libc::c_uint),
                            s.offset(8 as libc::c_int as isize),
                        );
                        AddToken(
                            tokens,
                            1 as libc::c_int as uint32_t,
                            base_id.wrapping_add(10 as libc::c_int as libc::c_uint),
                            s.offset(9 as libc::c_int as isize),
                        );
                        residue = (residue as libc::c_uint)
                            .wrapping_sub(
                                ((8 as libc::c_int) << 3 as libc::c_int) as libc::c_uint,
                            ) as uint32_t as uint32_t;
                        mask = (1 as libc::c_int) << 10 as libc::c_int;
                        tab = VP8Cat6.as_ptr();
                    }
                    while mask != 0 {
                        let fresh1 = tab;
                        tab = tab.offset(1);
                        AddConstantToken(
                            tokens,
                            (residue & mask as libc::c_uint != 0) as libc::c_int
                                as uint32_t,
                            *fresh1 as uint32_t,
                        );
                        mask >>= 1 as libc::c_int;
                    }
                }
                base_id = (NUM_PROBAS as libc::c_int
                    * (2 as libc::c_int
                        + NUM_CTX as libc::c_int
                            * (VP8EncBands[n as usize] as libc::c_int
                                + NUM_BANDS as libc::c_int * coeff_type))) as uint32_t;
                s = ((*((*res).stats)
                    .offset(
                        VP8EncBands[n as usize] as isize,
                    ))[2 as libc::c_int as usize])
                    .as_mut_ptr();
            }
            AddConstantToken(tokens, sign as uint32_t, 128 as libc::c_int as uint32_t);
            if n == 16 as libc::c_int
                || AddToken(
                    tokens,
                    (n <= last) as libc::c_int as uint32_t,
                    base_id.wrapping_add(0 as libc::c_int as libc::c_uint),
                    s.offset(0 as libc::c_int as isize),
                ) == 0
            {
                return 1 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EmitTokens(
    b: *mut VP8TBuffer,
    bw: *mut VP8BitWriter,
    probas: *const uint8_t,
    mut final_pass: libc::c_int,
) -> libc::c_int {
    let mut p: *const VP8Tokens = (*b).pages_;
    while !p.is_null() {
        let next: *const VP8Tokens = (*p).next_;
        let N: libc::c_int = if next.is_null() { (*b).left_ } else { 0 as libc::c_int };
        let mut n: libc::c_int = (*b).page_size_;
        let tokens: *const token_t = &*p.offset(1 as libc::c_int as isize)
            as *const VP8Tokens as *const token_t;
        loop {
            let fresh2 = n;
            n = n - 1;
            if !(fresh2 > N) {
                break;
            }
            let token: token_t = *tokens.offset(n as isize);
            let bit: libc::c_int = token as libc::c_int >> 15 as libc::c_int
                & 1 as libc::c_int;
            if token as libc::c_uint & (1 as libc::c_uint) << 14 as libc::c_int != 0 {
                VP8PutBit(
                    bw,
                    bit,
                    (token as libc::c_uint & 0xff as libc::c_uint) as libc::c_int,
                );
            } else {
                VP8PutBit(
                    bw,
                    bit,
                    *probas
                        .offset(
                            (token as libc::c_uint & 0x3fff as libc::c_uint) as isize,
                        ) as libc::c_int,
                );
            }
        }
        if final_pass != 0 {
            WebPSafeFree(p as *mut libc::c_void);
        }
        p = next;
    }
    if final_pass != 0 {
        (*b).pages_ = 0 as *mut VP8Tokens;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8EstimateTokenSize(
    b: *mut VP8TBuffer,
    probas: *const uint8_t,
) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut p: *const VP8Tokens = (*b).pages_;
    while !p.is_null() {
        let next: *const VP8Tokens = (*p).next_;
        let N: libc::c_int = if next.is_null() { (*b).left_ } else { 0 as libc::c_int };
        let mut n: libc::c_int = (*b).page_size_;
        let tokens: *const token_t = &*p.offset(1 as libc::c_int as isize)
            as *const VP8Tokens as *const token_t;
        loop {
            let fresh3 = n;
            n = n - 1;
            if !(fresh3 > N) {
                break;
            }
            let token: token_t = *tokens.offset(n as isize);
            let bit: libc::c_int = token as libc::c_int
                & (1 as libc::c_int) << 15 as libc::c_int;
            if token as libc::c_uint & (1 as libc::c_uint) << 14 as libc::c_int != 0 {
                size = (size as libc::c_ulong)
                    .wrapping_add(
                        VP8BitCost(
                            bit,
                            (token as libc::c_uint & 0xff as libc::c_uint) as uint8_t,
                        ) as libc::c_ulong,
                    ) as size_t as size_t;
            } else {
                size = (size as libc::c_ulong)
                    .wrapping_add(
                        VP8BitCost(
                            bit,
                            *probas
                                .offset(
                                    (token as libc::c_uint & 0x3fff as libc::c_uint) as isize,
                                ),
                        ) as libc::c_ulong,
                    ) as size_t as size_t;
            }
        }
        p = next;
    }
    return size;
}
