use ::libc;
extern "C" {
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: uint8_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode32 {
    pub bits: libc::c_int,
    pub value: uint32_t,
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
pub struct HuffmanTables {
    pub root: HuffmanTablesSegment,
    pub curr_segment: *mut HuffmanTablesSegment,
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
#[no_mangle]
pub unsafe extern "C" fn VP8LHtreeGroupsNew(
    mut num_htree_groups: libc::c_int,
) -> *mut HTreeGroup {
    let htree_groups: *mut HTreeGroup = WebPSafeMalloc(
        num_htree_groups as uint64_t,
        ::core::mem::size_of::<HTreeGroup>() as libc::c_ulong,
    ) as *mut HTreeGroup;
    if htree_groups.is_null() {
        return 0 as *mut HTreeGroup;
    }
    return htree_groups;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHtreeGroupsFree(htree_groups: *mut HTreeGroup) {
    if !htree_groups.is_null() {
        WebPSafeFree(htree_groups as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn GetNextKey(mut key: uint32_t, mut len: libc::c_int) -> uint32_t {
    let mut step: uint32_t = ((1 as libc::c_int) << len - 1 as libc::c_int) as uint32_t;
    while key & step != 0 {
        step >>= 1 as libc::c_int;
    }
    return if step != 0 {
        (key & step.wrapping_sub(1 as libc::c_int as libc::c_uint)).wrapping_add(step)
    } else {
        key
    };
}
#[inline]
unsafe extern "C" fn ReplicateValue(
    mut table: *mut HuffmanCode,
    mut step: libc::c_int,
    mut end: libc::c_int,
    mut code: HuffmanCode,
) {
    loop {
        end -= step;
        *table.offset(end as isize) = code;
        if !(end > 0 as libc::c_int) {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn NextTableBitSize(
    count: *const libc::c_int,
    mut len: libc::c_int,
    mut root_bits: libc::c_int,
) -> libc::c_int {
    let mut left: libc::c_int = (1 as libc::c_int) << len - root_bits;
    while len < 15 as libc::c_int {
        left -= *count.offset(len as isize);
        if left <= 0 as libc::c_int {
            break;
        }
        len += 1;
        len;
        left <<= 1 as libc::c_int;
    }
    return len - root_bits;
}
unsafe extern "C" fn BuildHuffmanTable(
    root_table: *mut HuffmanCode,
    mut root_bits: libc::c_int,
    mut code_lengths: *const libc::c_int,
    mut code_lengths_size: libc::c_int,
    mut sorted: *mut uint16_t,
) -> libc::c_int {
    let mut table: *mut HuffmanCode = root_table;
    let mut total_size: libc::c_int = (1 as libc::c_int) << root_bits;
    let mut len: libc::c_int = 0;
    let mut symbol: libc::c_int = 0;
    let mut count: [libc::c_int; 16] = [
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
    let mut offset: [libc::c_int; 16] = [0; 16];
    symbol = 0 as libc::c_int;
    while symbol < code_lengths_size {
        if *code_lengths.offset(symbol as isize) > 15 as libc::c_int {
            return 0 as libc::c_int;
        }
        count[*code_lengths.offset(symbol as isize) as usize] += 1;
        count[*code_lengths.offset(symbol as isize) as usize];
        symbol += 1;
        symbol;
    }
    if count[0 as libc::c_int as usize] == code_lengths_size {
        return 0 as libc::c_int;
    }
    offset[1 as libc::c_int as usize] = 0 as libc::c_int;
    len = 1 as libc::c_int;
    while len < 15 as libc::c_int {
        if count[len as usize] > (1 as libc::c_int) << len {
            return 0 as libc::c_int;
        }
        offset[(len + 1 as libc::c_int)
            as usize] = offset[len as usize] + count[len as usize];
        len += 1;
        len;
    }
    symbol = 0 as libc::c_int;
    while symbol < code_lengths_size {
        let symbol_code_length: libc::c_int = *code_lengths.offset(symbol as isize);
        if *code_lengths.offset(symbol as isize) > 0 as libc::c_int {
            if !sorted.is_null() {
                if offset[symbol_code_length as usize] >= code_lengths_size {
                    return 0 as libc::c_int;
                }
                let fresh0 = offset[symbol_code_length as usize];
                offset[symbol_code_length
                    as usize] = offset[symbol_code_length as usize] + 1;
                *sorted.offset(fresh0 as isize) = symbol as uint16_t;
            } else {
                offset[symbol_code_length as usize] += 1;
                offset[symbol_code_length as usize];
            }
        }
        symbol += 1;
        symbol;
    }
    if offset[15 as libc::c_int as usize] == 1 as libc::c_int {
        if !sorted.is_null() {
            let mut code: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
            code.bits = 0 as libc::c_int as uint8_t;
            code.value = *sorted.offset(0 as libc::c_int as isize);
            ReplicateValue(table, 1 as libc::c_int, total_size, code);
        }
        return total_size;
    }
    let mut step: libc::c_int = 0;
    let mut low: uint32_t = 0xffffffff as libc::c_uint;
    let mut mask: uint32_t = (total_size - 1 as libc::c_int) as uint32_t;
    let mut key: uint32_t = 0 as libc::c_int as uint32_t;
    let mut num_nodes: libc::c_int = 1 as libc::c_int;
    let mut num_open: libc::c_int = 1 as libc::c_int;
    let mut table_bits: libc::c_int = root_bits;
    let mut table_size: libc::c_int = (1 as libc::c_int) << table_bits;
    symbol = 0 as libc::c_int;
    len = 1 as libc::c_int;
    step = 2 as libc::c_int;
    while len <= root_bits {
        num_open <<= 1 as libc::c_int;
        num_nodes += num_open;
        num_open -= count[len as usize];
        if num_open < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if !root_table.is_null() {
            while count[len as usize] > 0 as libc::c_int {
                let mut code_0: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
                code_0.bits = len as uint8_t;
                let fresh1 = symbol;
                symbol = symbol + 1;
                code_0.value = *sorted.offset(fresh1 as isize);
                ReplicateValue(
                    &mut *table.offset(key as isize),
                    step,
                    table_size,
                    code_0,
                );
                key = GetNextKey(key, len);
                count[len as usize] -= 1;
                count[len as usize];
            }
        }
        len += 1;
        len;
        step <<= 1 as libc::c_int;
    }
    len = root_bits + 1 as libc::c_int;
    step = 2 as libc::c_int;
    while len <= 15 as libc::c_int {
        num_open <<= 1 as libc::c_int;
        num_nodes += num_open;
        num_open -= count[len as usize];
        if num_open < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        while count[len as usize] > 0 as libc::c_int {
            let mut code_1: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
            if key & mask != low {
                if !root_table.is_null() {
                    table = table.offset(table_size as isize);
                }
                table_bits = NextTableBitSize(count.as_mut_ptr(), len, root_bits);
                table_size = (1 as libc::c_int) << table_bits;
                total_size += table_size;
                low = key & mask;
                if !root_table.is_null() {
                    (*root_table.offset(low as isize))
                        .bits = (table_bits + root_bits) as uint8_t;
                    (*root_table.offset(low as isize))
                        .value = (table.offset_from(root_table) as libc::c_long
                        - low as libc::c_long) as uint16_t;
                }
            }
            if !root_table.is_null() {
                code_1.bits = (len - root_bits) as uint8_t;
                let fresh2 = symbol;
                symbol = symbol + 1;
                code_1.value = *sorted.offset(fresh2 as isize);
                ReplicateValue(
                    &mut *table.offset((key >> root_bits) as isize),
                    step,
                    table_size,
                    code_1,
                );
            }
            key = GetNextKey(key, len);
            count[len as usize] -= 1;
            count[len as usize];
        }
        len += 1;
        len;
        step <<= 1 as libc::c_int;
    }
    if num_nodes
        != 2 as libc::c_int * offset[15 as libc::c_int as usize] - 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return total_size;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBuildHuffmanTable(
    root_table: *mut HuffmanTables,
    mut root_bits: libc::c_int,
    mut code_lengths: *const libc::c_int,
    mut code_lengths_size: libc::c_int,
) -> libc::c_int {
    let total_size: libc::c_int = BuildHuffmanTable(
        0 as *mut HuffmanCode,
        root_bits,
        code_lengths,
        code_lengths_size,
        0 as *mut uint16_t,
    );
    if total_size == 0 as libc::c_int || root_table.is_null() {
        return total_size;
    }
    if ((*(*root_table).curr_segment).curr_table).offset(total_size as isize)
        >= ((*(*root_table).curr_segment).start)
            .offset((*(*root_table).curr_segment).size as isize)
    {
        let segment_size: libc::c_int = (*(*root_table).curr_segment).size;
        let mut next: *mut HuffmanTablesSegment = WebPSafeMalloc(
            1 as libc::c_int as uint64_t,
            ::core::mem::size_of::<HuffmanTablesSegment>() as libc::c_ulong,
        ) as *mut HuffmanTablesSegment;
        if next.is_null() {
            return 0 as libc::c_int;
        }
        (*next).size = if total_size > segment_size { total_size } else { segment_size };
        (*next)
            .start = WebPSafeMalloc(
            (*next).size as uint64_t,
            ::core::mem::size_of::<HuffmanCode>() as libc::c_ulong,
        ) as *mut HuffmanCode;
        if ((*next).start).is_null() {
            WebPSafeFree(next as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        (*next).curr_table = (*next).start;
        (*next).next = 0 as *mut HuffmanTablesSegment;
        (*(*root_table).curr_segment).next = next;
        (*root_table).curr_segment = next;
    }
    if code_lengths_size <= 512 as libc::c_int {
        let mut sorted: [uint16_t; 512] = [0; 512];
        BuildHuffmanTable(
            (*(*root_table).curr_segment).curr_table,
            root_bits,
            code_lengths,
            code_lengths_size,
            sorted.as_mut_ptr(),
        );
    } else {
        let sorted_0: *mut uint16_t = WebPSafeMalloc(
            code_lengths_size as uint64_t,
            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
        ) as *mut uint16_t;
        if sorted_0.is_null() {
            return 0 as libc::c_int;
        }
        BuildHuffmanTable(
            (*(*root_table).curr_segment).curr_table,
            root_bits,
            code_lengths,
            code_lengths_size,
            sorted_0,
        );
        WebPSafeFree(sorted_0 as *mut libc::c_void);
    }
    return total_size;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHuffmanTablesAllocate(
    mut size: libc::c_int,
    mut huffman_tables: *mut HuffmanTables,
) -> libc::c_int {
    let root: *mut HuffmanTablesSegment = &mut (*huffman_tables).root;
    (*huffman_tables).curr_segment = root;
    (*root).next = 0 as *mut HuffmanTablesSegment;
    (*root)
        .start = WebPSafeMalloc(
        size as uint64_t,
        ::core::mem::size_of::<HuffmanCode>() as libc::c_ulong,
    ) as *mut HuffmanCode;
    if ((*root).start).is_null() {
        return 0 as libc::c_int;
    }
    (*root).curr_table = (*root).start;
    (*root).size = size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LHuffmanTablesDeallocate(
    huffman_tables: *mut HuffmanTables,
) {
    let mut current: *mut HuffmanTablesSegment = 0 as *mut HuffmanTablesSegment;
    let mut next: *mut HuffmanTablesSegment = 0 as *mut HuffmanTablesSegment;
    if huffman_tables.is_null() {
        return;
    }
    current = &mut (*huffman_tables).root;
    next = (*current).next;
    WebPSafeFree((*current).start as *mut libc::c_void);
    (*current).start = 0 as *mut HuffmanCode;
    (*current).next = 0 as *mut HuffmanTablesSegment;
    current = next;
    while !current.is_null() {
        next = (*current).next;
        WebPSafeFree((*current).start as *mut libc::c_void);
        WebPSafeFree(current as *mut libc::c_void);
        current = next;
    }
}
