use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type bit_t = uint64_t;
pub type range_t = uint32_t;
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
pub type lbit_t = uint64_t;
pub type vp8l_val_t = uint64_t;
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
#[inline]
unsafe extern "C" fn WebPMemToUint32(ptr: *const uint8_t) -> uint32_t {
    let mut A: uint32_t = 0;
    memcpy(
        &mut A as *mut uint32_t as *mut libc::c_void,
        ptr as *const libc::c_void,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    return A;
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
unsafe extern "C" fn BitsLog2Floor(mut n: uint32_t) -> libc::c_int {
    return 31 as libc::c_int ^ n.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn VP8GetBit(
    br: *mut VP8BitReader,
    mut prob: libc::c_int,
) -> libc::c_int {
    let mut range: range_t = (*br).range_;
    if (*br).bits_ < 0 as libc::c_int {
        VP8LoadNewBytes(br);
    }
    let pos: libc::c_int = (*br).bits_;
    let split: range_t = range.wrapping_mul(prob as libc::c_uint) >> 8 as libc::c_int;
    let value: range_t = ((*br).value_ >> pos) as range_t;
    let bit: libc::c_int = (value > split) as libc::c_int;
    if bit != 0 {
        range = (range as libc::c_uint).wrapping_sub(split) as range_t as range_t;
        (*br)
            .value_ = ((*br).value_ as libc::c_ulong)
            .wrapping_sub(
                (split.wrapping_add(1 as libc::c_int as libc::c_uint) as bit_t) << pos,
            ) as bit_t as bit_t;
    } else {
        range = split.wrapping_add(1 as libc::c_int as libc::c_uint);
    }
    let shift: libc::c_int = 7 as libc::c_int ^ BitsLog2Floor(range);
    range <<= shift;
    (*br).bits_ -= shift;
    (*br).range_ = range.wrapping_sub(1 as libc::c_int as libc::c_uint);
    return bit;
}
#[inline]
unsafe extern "C" fn VP8LoadNewBytes(br: *mut VP8BitReader) {
    if (*br).buf_ < (*br).buf_max_ {
        let mut bits: bit_t = 0;
        let mut in_bits: lbit_t = 0;
        memcpy(
            &mut in_bits as *mut lbit_t as *mut libc::c_void,
            (*br).buf_ as *const libc::c_void,
            ::core::mem::size_of::<lbit_t>() as libc::c_ulong,
        );
        (*br)
            .buf_ = ((*br).buf_)
            .offset((56 as libc::c_int >> 3 as libc::c_int) as isize);
        bits = BSwap64(in_bits);
        bits >>= 64 as libc::c_int - 56 as libc::c_int;
        (*br).value_ = bits | (*br).value_ << 56 as libc::c_int;
        (*br).bits_ += 56 as libc::c_int;
    } else {
        VP8LoadFinalBytes(br);
    };
}
#[inline]
unsafe extern "C" fn BSwap64(mut x: uint64_t) -> uint64_t {
    return x.swap_bytes();
}
#[no_mangle]
pub unsafe extern "C" fn VP8BitReaderSetBuffer(
    br: *mut VP8BitReader,
    start: *const uint8_t,
    mut size: size_t,
) {
    (*br).buf_ = start;
    (*br).buf_end_ = start.offset(size as isize);
    (*br)
        .buf_max_ = if size >= ::core::mem::size_of::<lbit_t>() as libc::c_ulong {
        start
            .offset(size as isize)
            .offset(-(::core::mem::size_of::<lbit_t>() as libc::c_ulong as isize))
            .offset(1 as libc::c_int as isize)
    } else {
        start
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8InitBitReader(
    br: *mut VP8BitReader,
    start: *const uint8_t,
    mut size: size_t,
) {
    (*br).range_ = (255 as libc::c_int - 1 as libc::c_int) as range_t;
    (*br).value_ = 0 as libc::c_int as bit_t;
    (*br).bits_ = -(8 as libc::c_int);
    (*br).eof_ = 0 as libc::c_int;
    VP8BitReaderSetBuffer(br, start, size);
    VP8LoadNewBytes(br);
}
#[no_mangle]
pub unsafe extern "C" fn VP8RemapBitReader(
    br: *mut VP8BitReader,
    mut offset: ptrdiff_t,
) {
    if !((*br).buf_).is_null() {
        (*br).buf_ = ((*br).buf_).offset(offset as isize);
        (*br).buf_end_ = ((*br).buf_end_).offset(offset as isize);
        (*br).buf_max_ = ((*br).buf_max_).offset(offset as isize);
    }
}
#[no_mangle]
pub static mut kVP8Log2Range: [uint8_t; 128] = [
    7 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
#[no_mangle]
pub static mut kVP8NewRange: [uint8_t; 128] = [
    127 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    191 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    159 as libc::c_int as uint8_t,
    191 as libc::c_int as uint8_t,
    223 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    143 as libc::c_int as uint8_t,
    159 as libc::c_int as uint8_t,
    175 as libc::c_int as uint8_t,
    191 as libc::c_int as uint8_t,
    207 as libc::c_int as uint8_t,
    223 as libc::c_int as uint8_t,
    239 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    135 as libc::c_int as uint8_t,
    143 as libc::c_int as uint8_t,
    151 as libc::c_int as uint8_t,
    159 as libc::c_int as uint8_t,
    167 as libc::c_int as uint8_t,
    175 as libc::c_int as uint8_t,
    183 as libc::c_int as uint8_t,
    191 as libc::c_int as uint8_t,
    199 as libc::c_int as uint8_t,
    207 as libc::c_int as uint8_t,
    215 as libc::c_int as uint8_t,
    223 as libc::c_int as uint8_t,
    231 as libc::c_int as uint8_t,
    239 as libc::c_int as uint8_t,
    247 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    131 as libc::c_int as uint8_t,
    135 as libc::c_int as uint8_t,
    139 as libc::c_int as uint8_t,
    143 as libc::c_int as uint8_t,
    147 as libc::c_int as uint8_t,
    151 as libc::c_int as uint8_t,
    155 as libc::c_int as uint8_t,
    159 as libc::c_int as uint8_t,
    163 as libc::c_int as uint8_t,
    167 as libc::c_int as uint8_t,
    171 as libc::c_int as uint8_t,
    175 as libc::c_int as uint8_t,
    179 as libc::c_int as uint8_t,
    183 as libc::c_int as uint8_t,
    187 as libc::c_int as uint8_t,
    191 as libc::c_int as uint8_t,
    195 as libc::c_int as uint8_t,
    199 as libc::c_int as uint8_t,
    203 as libc::c_int as uint8_t,
    207 as libc::c_int as uint8_t,
    211 as libc::c_int as uint8_t,
    215 as libc::c_int as uint8_t,
    219 as libc::c_int as uint8_t,
    223 as libc::c_int as uint8_t,
    227 as libc::c_int as uint8_t,
    231 as libc::c_int as uint8_t,
    235 as libc::c_int as uint8_t,
    239 as libc::c_int as uint8_t,
    243 as libc::c_int as uint8_t,
    247 as libc::c_int as uint8_t,
    251 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    129 as libc::c_int as uint8_t,
    131 as libc::c_int as uint8_t,
    133 as libc::c_int as uint8_t,
    135 as libc::c_int as uint8_t,
    137 as libc::c_int as uint8_t,
    139 as libc::c_int as uint8_t,
    141 as libc::c_int as uint8_t,
    143 as libc::c_int as uint8_t,
    145 as libc::c_int as uint8_t,
    147 as libc::c_int as uint8_t,
    149 as libc::c_int as uint8_t,
    151 as libc::c_int as uint8_t,
    153 as libc::c_int as uint8_t,
    155 as libc::c_int as uint8_t,
    157 as libc::c_int as uint8_t,
    159 as libc::c_int as uint8_t,
    161 as libc::c_int as uint8_t,
    163 as libc::c_int as uint8_t,
    165 as libc::c_int as uint8_t,
    167 as libc::c_int as uint8_t,
    169 as libc::c_int as uint8_t,
    171 as libc::c_int as uint8_t,
    173 as libc::c_int as uint8_t,
    175 as libc::c_int as uint8_t,
    177 as libc::c_int as uint8_t,
    179 as libc::c_int as uint8_t,
    181 as libc::c_int as uint8_t,
    183 as libc::c_int as uint8_t,
    185 as libc::c_int as uint8_t,
    187 as libc::c_int as uint8_t,
    189 as libc::c_int as uint8_t,
    191 as libc::c_int as uint8_t,
    193 as libc::c_int as uint8_t,
    195 as libc::c_int as uint8_t,
    197 as libc::c_int as uint8_t,
    199 as libc::c_int as uint8_t,
    201 as libc::c_int as uint8_t,
    203 as libc::c_int as uint8_t,
    205 as libc::c_int as uint8_t,
    207 as libc::c_int as uint8_t,
    209 as libc::c_int as uint8_t,
    211 as libc::c_int as uint8_t,
    213 as libc::c_int as uint8_t,
    215 as libc::c_int as uint8_t,
    217 as libc::c_int as uint8_t,
    219 as libc::c_int as uint8_t,
    221 as libc::c_int as uint8_t,
    223 as libc::c_int as uint8_t,
    225 as libc::c_int as uint8_t,
    227 as libc::c_int as uint8_t,
    229 as libc::c_int as uint8_t,
    231 as libc::c_int as uint8_t,
    233 as libc::c_int as uint8_t,
    235 as libc::c_int as uint8_t,
    237 as libc::c_int as uint8_t,
    239 as libc::c_int as uint8_t,
    241 as libc::c_int as uint8_t,
    243 as libc::c_int as uint8_t,
    245 as libc::c_int as uint8_t,
    247 as libc::c_int as uint8_t,
    249 as libc::c_int as uint8_t,
    251 as libc::c_int as uint8_t,
    253 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8LoadFinalBytes(br: *mut VP8BitReader) {
    if (*br).buf_ < (*br).buf_end_ {
        (*br).bits_ += 8 as libc::c_int;
        let fresh0 = (*br).buf_;
        (*br).buf_ = ((*br).buf_).offset(1);
        (*br).value_ = *fresh0 as bit_t | (*br).value_ << 8 as libc::c_int;
    } else if (*br).eof_ == 0 {
        (*br).value_ <<= 8 as libc::c_int;
        (*br).bits_ += 8 as libc::c_int;
        (*br).eof_ = 1 as libc::c_int;
    } else {
        (*br).bits_ = 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetValue(
    br: *mut VP8BitReader,
    mut bits: libc::c_int,
) -> uint32_t {
    let mut v: uint32_t = 0 as libc::c_int as uint32_t;
    loop {
        let fresh1 = bits;
        bits = bits - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        v |= (VP8GetBit(br, 0x80 as libc::c_int) << bits) as libc::c_uint;
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn VP8GetSignedValue(
    br: *mut VP8BitReader,
    mut bits: libc::c_int,
) -> int32_t {
    let value: libc::c_int = VP8GetValue(br, bits) as libc::c_int;
    return if VP8GetValue(br, 1 as libc::c_int) != 0 { -value } else { value };
}
static mut kBitMask: [uint32_t; 25] = [
    0 as libc::c_int as uint32_t,
    0x1 as libc::c_int as uint32_t,
    0x3 as libc::c_int as uint32_t,
    0x7 as libc::c_int as uint32_t,
    0xf as libc::c_int as uint32_t,
    0x1f as libc::c_int as uint32_t,
    0x3f as libc::c_int as uint32_t,
    0x7f as libc::c_int as uint32_t,
    0xff as libc::c_int as uint32_t,
    0x1ff as libc::c_int as uint32_t,
    0x3ff as libc::c_int as uint32_t,
    0x7ff as libc::c_int as uint32_t,
    0xfff as libc::c_int as uint32_t,
    0x1fff as libc::c_int as uint32_t,
    0x3fff as libc::c_int as uint32_t,
    0x7fff as libc::c_int as uint32_t,
    0xffff as libc::c_int as uint32_t,
    0x1ffff as libc::c_int as uint32_t,
    0x3ffff as libc::c_int as uint32_t,
    0x7ffff as libc::c_int as uint32_t,
    0xfffff as libc::c_int as uint32_t,
    0x1fffff as libc::c_int as uint32_t,
    0x3fffff as libc::c_int as uint32_t,
    0x7fffff as libc::c_int as uint32_t,
    0xffffff as libc::c_int as uint32_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8LInitBitReader(
    br: *mut VP8LBitReader,
    start: *const uint8_t,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    let mut value: vp8l_val_t = 0 as libc::c_int as vp8l_val_t;
    (*br).len_ = length;
    (*br).val_ = 0 as libc::c_int as vp8l_val_t;
    (*br).bit_pos_ = 0 as libc::c_int;
    (*br).eos_ = 0 as libc::c_int;
    if length > ::core::mem::size_of::<vp8l_val_t>() as libc::c_ulong {
        length = ::core::mem::size_of::<vp8l_val_t>() as libc::c_ulong;
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        value
            |= (*start.offset(i as isize) as vp8l_val_t)
                << (8 as libc::c_int as libc::c_ulong).wrapping_mul(i);
        i = i.wrapping_add(1);
        i;
    }
    (*br).val_ = value;
    (*br).pos_ = length;
    (*br).buf_ = start;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitReaderSetBuffer(
    br: *mut VP8LBitReader,
    buf: *const uint8_t,
    mut len: size_t,
) {
    (*br).buf_ = buf;
    (*br).len_ = len;
    (*br).eos_ = ((*br).pos_ > (*br).len_ || VP8LIsEndOfStream(br) != 0) as libc::c_int;
}
unsafe extern "C" fn VP8LSetEndOfStream(br: *mut VP8LBitReader) {
    (*br).eos_ = 1 as libc::c_int;
    (*br).bit_pos_ = 0 as libc::c_int;
}
unsafe extern "C" fn ShiftBytes(br: *mut VP8LBitReader) {
    while (*br).bit_pos_ >= 8 as libc::c_int && (*br).pos_ < (*br).len_ {
        (*br).val_ >>= 8 as libc::c_int;
        (*br).val_
            |= (*((*br).buf_).offset((*br).pos_ as isize) as vp8l_val_t)
                << 64 as libc::c_int - 8 as libc::c_int;
        (*br).pos_ = ((*br).pos_).wrapping_add(1);
        (*br).pos_;
        (*br).bit_pos_ -= 8 as libc::c_int;
    }
    if VP8LIsEndOfStream(br) != 0 {
        VP8LSetEndOfStream(br);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LDoFillBitWindow(br: *mut VP8LBitReader) {
    if ((*br).pos_).wrapping_add(::core::mem::size_of::<vp8l_val_t>() as libc::c_ulong)
        < (*br).len_
    {
        (*br).val_ >>= 32 as libc::c_int;
        (*br).bit_pos_ -= 32 as libc::c_int;
        (*br).val_
            |= (WebPMemToUint32(((*br).buf_).offset((*br).pos_ as isize)) as vp8l_val_t)
                << 64 as libc::c_int - 32 as libc::c_int;
        (*br)
            .pos_ = ((*br).pos_ as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        return;
    }
    ShiftBytes(br);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LReadBits(
    br: *mut VP8LBitReader,
    mut n_bits: libc::c_int,
) -> uint32_t {
    if (*br).eos_ == 0 && n_bits <= 24 as libc::c_int {
        let val: uint32_t = VP8LPrefetchBits(br) & kBitMask[n_bits as usize];
        let new_bits: libc::c_int = (*br).bit_pos_ + n_bits;
        (*br).bit_pos_ = new_bits;
        ShiftBytes(br);
        return val;
    } else {
        VP8LSetEndOfStream(br);
        return 0 as libc::c_int as uint32_t;
    };
}
