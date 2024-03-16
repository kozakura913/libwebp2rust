use ::libc;
extern "C" {
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
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
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeToken {
    pub code: uint8_t,
    pub extra_bits: uint8_t,
}
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
unsafe extern "C" fn ValuesShouldBeCollapsedToStrideAverage(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return (abs(a - b) < 4 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn OptimizeHuffmanForRle(
    mut length: libc::c_int,
    good_for_rle: *mut uint8_t,
    counts: *mut uint32_t,
) {
    let mut i: libc::c_int = 0;
    while length >= 0 as libc::c_int {
        if length == 0 as libc::c_int {
            return;
        }
        if *counts.offset((length - 1 as libc::c_int) as isize)
            != 0 as libc::c_int as libc::c_uint
        {
            break;
        }
        length -= 1;
        length;
    }
    let mut symbol: uint32_t = *counts.offset(0 as libc::c_int as isize);
    let mut stride: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length + 1 as libc::c_int {
        if i == length || *counts.offset(i as isize) != symbol {
            if symbol == 0 as libc::c_int as libc::c_uint && stride >= 5 as libc::c_int
                || symbol != 0 as libc::c_int as libc::c_uint
                    && stride >= 7 as libc::c_int
            {
                let mut k: libc::c_int = 0;
                k = 0 as libc::c_int;
                while k < stride {
                    *good_for_rle
                        .offset(
                            (i - k - 1 as libc::c_int) as isize,
                        ) = 1 as libc::c_int as uint8_t;
                    k += 1;
                    k;
                }
            }
            stride = 1 as libc::c_int;
            if i != length {
                symbol = *counts.offset(i as isize);
            }
        } else {
            stride += 1;
            stride;
        }
        i += 1;
        i;
    }
    let mut stride_0: uint32_t = 0 as libc::c_int as uint32_t;
    let mut limit: uint32_t = *counts.offset(0 as libc::c_int as isize);
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int;
    while i < length + 1 as libc::c_int {
        if i == length || *good_for_rle.offset(i as isize) as libc::c_int != 0
            || i != 0 as libc::c_int
                && *good_for_rle.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                    != 0
            || ValuesShouldBeCollapsedToStrideAverage(
                *counts.offset(i as isize) as libc::c_int,
                limit as libc::c_int,
            ) == 0
        {
            if stride_0 >= 4 as libc::c_int as libc::c_uint
                || stride_0 >= 3 as libc::c_int as libc::c_uint
                    && sum == 0 as libc::c_int as libc::c_uint
            {
                let mut k_0: uint32_t = 0;
                let mut count: uint32_t = sum
                    .wrapping_add(
                        stride_0.wrapping_div(2 as libc::c_int as libc::c_uint),
                    )
                    .wrapping_div(stride_0);
                if count < 1 as libc::c_int as libc::c_uint {
                    count = 1 as libc::c_int as uint32_t;
                }
                if sum == 0 as libc::c_int as libc::c_uint {
                    count = 0 as libc::c_int as uint32_t;
                }
                k_0 = 0 as libc::c_int as uint32_t;
                while k_0 < stride_0 {
                    *counts
                        .offset(
                            (i as libc::c_uint)
                                .wrapping_sub(k_0)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) = count;
                    k_0 = k_0.wrapping_add(1);
                    k_0;
                }
            }
            stride_0 = 0 as libc::c_int as uint32_t;
            sum = 0 as libc::c_int as uint32_t;
            if i < length - 3 as libc::c_int {
                limit = (*counts.offset(i as isize))
                    .wrapping_add(*counts.offset((i + 1 as libc::c_int) as isize))
                    .wrapping_add(*counts.offset((i + 2 as libc::c_int) as isize))
                    .wrapping_add(*counts.offset((i + 3 as libc::c_int) as isize))
                    .wrapping_add(2 as libc::c_int as libc::c_uint)
                    .wrapping_div(4 as libc::c_int as libc::c_uint);
            } else if i < length {
                limit = *counts.offset(i as isize);
            } else {
                limit = 0 as libc::c_int as uint32_t;
            }
        }
        stride_0 = stride_0.wrapping_add(1);
        stride_0;
        if i != length {
            sum = (sum as libc::c_uint).wrapping_add(*counts.offset(i as isize))
                as uint32_t as uint32_t;
            if stride_0 >= 4 as libc::c_int as libc::c_uint {
                limit = sum
                    .wrapping_add(
                        stride_0.wrapping_div(2 as libc::c_int as libc::c_uint),
                    )
                    .wrapping_div(stride_0);
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn CompareHuffmanTrees(
    mut ptr1: *const libc::c_void,
    mut ptr2: *const libc::c_void,
) -> libc::c_int {
    let t1: *const HuffmanTree = ptr1 as *const HuffmanTree;
    let t2: *const HuffmanTree = ptr2 as *const HuffmanTree;
    if (*t1).total_count_ > (*t2).total_count_ {
        return -(1 as libc::c_int)
    } else if (*t1).total_count_ < (*t2).total_count_ {
        return 1 as libc::c_int
    } else {
        return if (*t1).value_ < (*t2).value_ {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }
    };
}
unsafe extern "C" fn SetBitDepths(
    tree: *const HuffmanTree,
    pool: *const HuffmanTree,
    bit_depths: *mut uint8_t,
    mut level: libc::c_int,
) {
    if (*tree).pool_index_left_ >= 0 as libc::c_int {
        SetBitDepths(
            &*pool.offset((*tree).pool_index_left_ as isize),
            pool,
            bit_depths,
            level + 1 as libc::c_int,
        );
        SetBitDepths(
            &*pool.offset((*tree).pool_index_right_ as isize),
            pool,
            bit_depths,
            level + 1 as libc::c_int,
        );
    } else {
        *bit_depths.offset((*tree).value_ as isize) = level as uint8_t;
    };
}
unsafe extern "C" fn GenerateOptimalTree(
    histogram: *const uint32_t,
    mut histogram_size: libc::c_int,
    mut tree: *mut HuffmanTree,
    mut tree_depth_limit: libc::c_int,
    bit_depths: *mut uint8_t,
) {
    let mut count_min: uint32_t = 0;
    let mut tree_pool: *mut HuffmanTree = 0 as *mut HuffmanTree;
    let mut tree_size_orig: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < histogram_size {
        if *histogram.offset(i as isize) != 0 as libc::c_int as libc::c_uint {
            tree_size_orig += 1;
            tree_size_orig;
        }
        i += 1;
        i;
    }
    if tree_size_orig == 0 as libc::c_int {
        return;
    }
    tree_pool = tree.offset(tree_size_orig as isize);
    count_min = 1 as libc::c_int as uint32_t;
    loop {
        let mut tree_size: libc::c_int = tree_size_orig;
        let mut idx: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < histogram_size {
            if *histogram.offset(j as isize) != 0 as libc::c_int as libc::c_uint {
                let count: uint32_t = if *histogram.offset(j as isize) < count_min {
                    count_min
                } else {
                    *histogram.offset(j as isize)
                };
                (*tree.offset(idx as isize)).total_count_ = count;
                (*tree.offset(idx as isize)).value_ = j;
                (*tree.offset(idx as isize)).pool_index_left_ = -(1 as libc::c_int);
                (*tree.offset(idx as isize)).pool_index_right_ = -(1 as libc::c_int);
                idx += 1;
                idx;
            }
            j += 1;
            j;
        }
        qsort(
            tree as *mut libc::c_void,
            tree_size as size_t,
            ::core::mem::size_of::<HuffmanTree>() as libc::c_ulong,
            Some(
                CompareHuffmanTrees
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        if tree_size > 1 as libc::c_int {
            let mut tree_pool_size: libc::c_int = 0 as libc::c_int;
            while tree_size > 1 as libc::c_int {
                let mut count_0: uint32_t = 0;
                let fresh0 = tree_pool_size;
                tree_pool_size = tree_pool_size + 1;
                *tree_pool
                    .offset(
                        fresh0 as isize,
                    ) = *tree.offset((tree_size - 1 as libc::c_int) as isize);
                let fresh1 = tree_pool_size;
                tree_pool_size = tree_pool_size + 1;
                *tree_pool
                    .offset(
                        fresh1 as isize,
                    ) = *tree.offset((tree_size - 2 as libc::c_int) as isize);
                count_0 = ((*tree_pool
                    .offset((tree_pool_size - 1 as libc::c_int) as isize))
                    .total_count_)
                    .wrapping_add(
                        (*tree_pool.offset((tree_pool_size - 2 as libc::c_int) as isize))
                            .total_count_,
                    );
                tree_size -= 2 as libc::c_int;
                let mut k: libc::c_int = 0;
                k = 0 as libc::c_int;
                while k < tree_size {
                    if (*tree.offset(k as isize)).total_count_ <= count_0 {
                        break;
                    }
                    k += 1;
                    k;
                }
                memmove(
                    tree.offset((k + 1 as libc::c_int) as isize) as *mut libc::c_void,
                    tree.offset(k as isize) as *const libc::c_void,
                    ((tree_size - k) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<HuffmanTree>() as libc::c_ulong,
                        ),
                );
                (*tree.offset(k as isize)).total_count_ = count_0;
                (*tree.offset(k as isize)).value_ = -(1 as libc::c_int);
                (*tree.offset(k as isize))
                    .pool_index_left_ = tree_pool_size - 1 as libc::c_int;
                (*tree.offset(k as isize))
                    .pool_index_right_ = tree_pool_size - 2 as libc::c_int;
                tree_size = tree_size + 1 as libc::c_int;
            }
            SetBitDepths(
                &mut *tree.offset(0 as libc::c_int as isize),
                tree_pool,
                bit_depths,
                0 as libc::c_int,
            );
        } else if tree_size == 1 as libc::c_int {
            *bit_depths
                .offset(
                    (*tree.offset(0 as libc::c_int as isize)).value_ as isize,
                ) = 1 as libc::c_int as uint8_t;
        }
        let mut max_depth: libc::c_int = *bit_depths.offset(0 as libc::c_int as isize)
            as libc::c_int;
        j = 1 as libc::c_int;
        while j < histogram_size {
            if max_depth < *bit_depths.offset(j as isize) as libc::c_int {
                max_depth = *bit_depths.offset(j as isize) as libc::c_int;
            }
            j += 1;
            j;
        }
        if max_depth <= tree_depth_limit {
            break;
        }
        count_min = (count_min as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    };
}
unsafe extern "C" fn CodeRepeatedValues(
    mut repetitions: libc::c_int,
    mut tokens: *mut HuffmanTreeToken,
    mut value: libc::c_int,
    mut prev_value: libc::c_int,
) -> *mut HuffmanTreeToken {
    if value != prev_value {
        (*tokens).code = value as uint8_t;
        (*tokens).extra_bits = 0 as libc::c_int as uint8_t;
        tokens = tokens.offset(1);
        tokens;
        repetitions -= 1;
        repetitions;
    }
    while repetitions >= 1 as libc::c_int {
        if repetitions < 3 as libc::c_int {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < repetitions {
                (*tokens).code = value as uint8_t;
                (*tokens).extra_bits = 0 as libc::c_int as uint8_t;
                tokens = tokens.offset(1);
                tokens;
                i += 1;
                i;
            }
            break;
        } else if repetitions < 7 as libc::c_int {
            (*tokens).code = 16 as libc::c_int as uint8_t;
            (*tokens).extra_bits = (repetitions - 3 as libc::c_int) as uint8_t;
            tokens = tokens.offset(1);
            tokens;
            break;
        } else {
            (*tokens).code = 16 as libc::c_int as uint8_t;
            (*tokens).extra_bits = 3 as libc::c_int as uint8_t;
            tokens = tokens.offset(1);
            tokens;
            repetitions -= 6 as libc::c_int;
        }
    }
    return tokens;
}
unsafe extern "C" fn CodeRepeatedZeros(
    mut repetitions: libc::c_int,
    mut tokens: *mut HuffmanTreeToken,
) -> *mut HuffmanTreeToken {
    while repetitions >= 1 as libc::c_int {
        if repetitions < 3 as libc::c_int {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < repetitions {
                (*tokens).code = 0 as libc::c_int as uint8_t;
                (*tokens).extra_bits = 0 as libc::c_int as uint8_t;
                tokens = tokens.offset(1);
                tokens;
                i += 1;
                i;
            }
            break;
        } else if repetitions < 11 as libc::c_int {
            (*tokens).code = 17 as libc::c_int as uint8_t;
            (*tokens).extra_bits = (repetitions - 3 as libc::c_int) as uint8_t;
            tokens = tokens.offset(1);
            tokens;
            break;
        } else if repetitions < 139 as libc::c_int {
            (*tokens).code = 18 as libc::c_int as uint8_t;
            (*tokens).extra_bits = (repetitions - 11 as libc::c_int) as uint8_t;
            tokens = tokens.offset(1);
            tokens;
            break;
        } else {
            (*tokens).code = 18 as libc::c_int as uint8_t;
            (*tokens).extra_bits = 0x7f as libc::c_int as uint8_t;
            tokens = tokens.offset(1);
            tokens;
            repetitions -= 138 as libc::c_int;
        }
    }
    return tokens;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LCreateCompressedHuffmanTree(
    tree: *const HuffmanTreeCode,
    mut tokens: *mut HuffmanTreeToken,
    mut max_tokens: libc::c_int,
) -> libc::c_int {
    let starting_token: *mut HuffmanTreeToken = tokens;
    let ending_token: *mut HuffmanTreeToken = tokens.offset(max_tokens as isize);
    let depth_size: libc::c_int = (*tree).num_symbols;
    let mut prev_value: libc::c_int = 8 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < depth_size {
        let value: libc::c_int = *((*tree).code_lengths).offset(i as isize)
            as libc::c_int;
        let mut k: libc::c_int = i + 1 as libc::c_int;
        let mut runs: libc::c_int = 0;
        while k < depth_size
            && *((*tree).code_lengths).offset(k as isize) as libc::c_int == value
        {
            k += 1;
            k;
        }
        runs = k - i;
        if value == 0 as libc::c_int {
            tokens = CodeRepeatedZeros(runs, tokens);
        } else {
            tokens = CodeRepeatedValues(runs, tokens, value, prev_value);
            prev_value = value;
        }
        i += runs;
    }
    return tokens.offset_from(starting_token) as libc::c_long as libc::c_int;
}
static mut kReversedBits: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0xc as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0xe as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0xd as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0xf as libc::c_int as uint8_t,
];
unsafe extern "C" fn ReverseBits(
    mut num_bits: libc::c_int,
    mut bits: uint32_t,
) -> uint32_t {
    let mut retval: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_bits {
        i += 4 as libc::c_int;
        retval
            |= ((kReversedBits[(bits & 0xf as libc::c_int as libc::c_uint) as usize]
                as libc::c_int) << 15 as libc::c_int + 1 as libc::c_int - i)
                as libc::c_uint;
        bits >>= 4 as libc::c_int;
    }
    retval >>= 15 as libc::c_int + 1 as libc::c_int - num_bits;
    return retval;
}
unsafe extern "C" fn ConvertBitDepthsToSymbols(tree: *mut HuffmanTreeCode) {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut next_code: [uint32_t; 16] = [0; 16];
    let mut depth_count: [libc::c_int; 16] = [
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
    ];
    len = (*tree).num_symbols;
    i = 0 as libc::c_int;
    while i < len {
        let code_length: libc::c_int = *((*tree).code_lengths).offset(i as isize)
            as libc::c_int;
        depth_count[code_length as usize] += 1;
        depth_count[code_length as usize];
        i += 1;
        i;
    }
    depth_count[0 as libc::c_int as usize] = 0 as libc::c_int;
    next_code[0 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    let mut code: uint32_t = 0 as libc::c_int as uint32_t;
    i = 1 as libc::c_int;
    while i <= 15 as libc::c_int {
        code = code
            .wrapping_add(depth_count[(i - 1 as libc::c_int) as usize] as libc::c_uint)
            << 1 as libc::c_int;
        next_code[i as usize] = code;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < len {
        let code_length_0: libc::c_int = *((*tree).code_lengths).offset(i as isize)
            as libc::c_int;
        let fresh2 = next_code[code_length_0 as usize];
        next_code[code_length_0
            as usize] = (next_code[code_length_0 as usize]).wrapping_add(1);
        *((*tree).codes)
            .offset(i as isize) = ReverseBits(code_length_0, fresh2) as uint16_t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LCreateHuffmanTree(
    histogram: *mut uint32_t,
    mut tree_depth_limit: libc::c_int,
    buf_rle: *mut uint8_t,
    huff_tree: *mut HuffmanTree,
    huff_code: *mut HuffmanTreeCode,
) {
    let num_symbols: libc::c_int = (*huff_code).num_symbols;
    memset(
        buf_rle as *mut libc::c_void,
        0 as libc::c_int,
        (num_symbols as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
    );
    OptimizeHuffmanForRle(num_symbols, buf_rle, histogram);
    GenerateOptimalTree(
        histogram,
        num_symbols,
        huff_tree,
        tree_depth_limit,
        (*huff_code).code_lengths,
    );
    ConvertBitDepthsToSymbols(huff_code);
}
