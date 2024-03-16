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
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[inline]
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> libc::c_int {
    return (size == size) as libc::c_int;
}
unsafe extern "C" fn BitWriterResize(
    bw: *mut VP8BitWriter,
    mut extra_size: size_t,
) -> libc::c_int {
    let mut new_buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut new_size: size_t = 0;
    let needed_size_64b: uint64_t = ((*bw).pos_).wrapping_add(extra_size);
    let needed_size: size_t = needed_size_64b;
    if needed_size_64b != needed_size {
        (*bw).error_ = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if needed_size <= (*bw).max_pos_ {
        return 1 as libc::c_int;
    }
    new_size = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*bw).max_pos_);
    if new_size < needed_size {
        new_size = needed_size;
    }
    if new_size < 1024 as libc::c_int as libc::c_ulong {
        new_size = 1024 as libc::c_int as size_t;
    }
    new_buf = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, new_size)
        as *mut uint8_t;
    if new_buf.is_null() {
        (*bw).error_ = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*bw).pos_ > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            new_buf as *mut libc::c_void,
            (*bw).buf_ as *const libc::c_void,
            (*bw).pos_,
        );
    }
    WebPSafeFree((*bw).buf_ as *mut libc::c_void);
    (*bw).buf_ = new_buf;
    (*bw).max_pos_ = new_size;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Flush(bw: *mut VP8BitWriter) {
    let s: libc::c_int = 8 as libc::c_int + (*bw).nb_bits_;
    let bits: int32_t = (*bw).value_ >> s;
    (*bw).value_ -= bits << s;
    (*bw).nb_bits_ -= 8 as libc::c_int;
    if bits & 0xff as libc::c_int != 0xff as libc::c_int {
        let mut pos: size_t = (*bw).pos_;
        if BitWriterResize(bw, ((*bw).run_ + 1 as libc::c_int) as size_t) == 0 {
            return;
        }
        if bits & 0x100 as libc::c_int != 0 {
            if pos > 0 as libc::c_int as libc::c_ulong {
                let ref mut fresh0 = *((*bw).buf_)
                    .offset(
                        pos.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                *fresh0 = (*fresh0).wrapping_add(1);
                *fresh0;
            }
        }
        if (*bw).run_ > 0 as libc::c_int {
            let value: libc::c_int = if bits & 0x100 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                0xff as libc::c_int
            };
            while (*bw).run_ > 0 as libc::c_int {
                let fresh1 = pos;
                pos = pos.wrapping_add(1);
                *((*bw).buf_).offset(fresh1 as isize) = value as uint8_t;
                (*bw).run_ -= 1;
                (*bw).run_;
            }
        }
        let fresh2 = pos;
        pos = pos.wrapping_add(1);
        *((*bw).buf_).offset(fresh2 as isize) = (bits & 0xff as libc::c_int) as uint8_t;
        (*bw).pos_ = pos;
    } else {
        (*bw).run_ += 1;
        (*bw).run_;
    };
}
static mut kNorm: [uint8_t; 128] = [
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
static mut kNewRange: [uint8_t; 128] = [
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
pub unsafe extern "C" fn VP8PutBit(
    bw: *mut VP8BitWriter,
    mut bit: libc::c_int,
    mut prob: libc::c_int,
) -> libc::c_int {
    let split: libc::c_int = (*bw).range_ * prob >> 8 as libc::c_int;
    if bit != 0 {
        (*bw).value_ += split + 1 as libc::c_int;
        (*bw).range_ -= split + 1 as libc::c_int;
    } else {
        (*bw).range_ = split;
    }
    if (*bw).range_ < 127 as libc::c_int {
        let shift: libc::c_int = kNorm[(*bw).range_ as usize] as libc::c_int;
        (*bw).range_ = kNewRange[(*bw).range_ as usize] as int32_t;
        (*bw).value_ <<= shift;
        (*bw).nb_bits_ += shift;
        if (*bw).nb_bits_ > 0 as libc::c_int {
            Flush(bw);
        }
    }
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn VP8PutBitUniform(
    bw: *mut VP8BitWriter,
    mut bit: libc::c_int,
) -> libc::c_int {
    let split: libc::c_int = (*bw).range_ >> 1 as libc::c_int;
    if bit != 0 {
        (*bw).value_ += split + 1 as libc::c_int;
        (*bw).range_ -= split + 1 as libc::c_int;
    } else {
        (*bw).range_ = split;
    }
    if (*bw).range_ < 127 as libc::c_int {
        (*bw).range_ = kNewRange[(*bw).range_ as usize] as int32_t;
        (*bw).value_ <<= 1 as libc::c_int;
        (*bw).nb_bits_ += 1 as libc::c_int;
        if (*bw).nb_bits_ > 0 as libc::c_int {
            Flush(bw);
        }
    }
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn VP8PutBits(
    bw: *mut VP8BitWriter,
    mut value: uint32_t,
    mut nb_bits: libc::c_int,
) {
    let mut mask: uint32_t = 0;
    mask = (1 as libc::c_uint) << nb_bits - 1 as libc::c_int;
    while mask != 0 {
        VP8PutBitUniform(bw, (value & mask) as libc::c_int);
        mask >>= 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8PutSignedBits(
    bw: *mut VP8BitWriter,
    mut value: libc::c_int,
    mut nb_bits: libc::c_int,
) {
    if VP8PutBitUniform(bw, (value != 0 as libc::c_int) as libc::c_int) == 0 {
        return;
    }
    if value < 0 as libc::c_int {
        VP8PutBits(
            bw,
            (-value << 1 as libc::c_int | 1 as libc::c_int) as uint32_t,
            nb_bits + 1 as libc::c_int,
        );
    } else {
        VP8PutBits(
            bw,
            (value << 1 as libc::c_int) as uint32_t,
            nb_bits + 1 as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8BitWriterInit(
    bw: *mut VP8BitWriter,
    mut expected_size: size_t,
) -> libc::c_int {
    (*bw).range_ = 255 as libc::c_int - 1 as libc::c_int;
    (*bw).value_ = 0 as libc::c_int;
    (*bw).run_ = 0 as libc::c_int;
    (*bw).nb_bits_ = -(8 as libc::c_int);
    (*bw).pos_ = 0 as libc::c_int as size_t;
    (*bw).max_pos_ = 0 as libc::c_int as size_t;
    (*bw).error_ = 0 as libc::c_int;
    (*bw).buf_ = 0 as *mut uint8_t;
    return if expected_size > 0 as libc::c_int as libc::c_ulong {
        BitWriterResize(bw, expected_size)
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn VP8BitWriterFinish(bw: *mut VP8BitWriter) -> *mut uint8_t {
    VP8PutBits(bw, 0 as libc::c_int as uint32_t, 9 as libc::c_int - (*bw).nb_bits_);
    (*bw).nb_bits_ = 0 as libc::c_int;
    Flush(bw);
    return (*bw).buf_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8BitWriterAppend(
    bw: *mut VP8BitWriter,
    mut data: *const uint8_t,
    mut size: size_t,
) -> libc::c_int {
    if (*bw).nb_bits_ != -(8 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if BitWriterResize(bw, size) == 0 {
        return 0 as libc::c_int;
    }
    memcpy(
        ((*bw).buf_).offset((*bw).pos_ as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        size,
    );
    (*bw).pos_ = ((*bw).pos_ as libc::c_ulong).wrapping_add(size) as size_t as size_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8BitWriterWipeOut(bw: *mut VP8BitWriter) {
    if !bw.is_null() {
        WebPSafeFree((*bw).buf_ as *mut libc::c_void);
        memset(
            bw as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<VP8BitWriter>() as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn VP8LBitWriterResize(
    bw: *mut VP8LBitWriter,
    mut extra_size: size_t,
) -> libc::c_int {
    let mut allocated_buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut allocated_size: size_t = 0;
    let max_bytes: size_t = ((*bw).end_).offset_from((*bw).buf_) as libc::c_long
        as size_t;
    let current_size: size_t = ((*bw).cur_).offset_from((*bw).buf_) as libc::c_long
        as size_t;
    let size_required_64b: uint64_t = current_size.wrapping_add(extra_size);
    let size_required: size_t = size_required_64b;
    if size_required != size_required_64b {
        (*bw).error_ = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if max_bytes > 0 as libc::c_int as libc::c_ulong && size_required <= max_bytes {
        return 1 as libc::c_int;
    }
    allocated_size = (3 as libc::c_int as libc::c_ulong).wrapping_mul(max_bytes)
        >> 1 as libc::c_int;
    if allocated_size < size_required {
        allocated_size = size_required;
    }
    allocated_size = (allocated_size >> 10 as libc::c_int)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) << 10 as libc::c_int;
    allocated_buf = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, allocated_size)
        as *mut uint8_t;
    if allocated_buf.is_null() {
        (*bw).error_ = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if current_size > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            allocated_buf as *mut libc::c_void,
            (*bw).buf_ as *const libc::c_void,
            current_size,
        );
    }
    WebPSafeFree((*bw).buf_ as *mut libc::c_void);
    (*bw).buf_ = allocated_buf;
    (*bw).cur_ = ((*bw).buf_).offset(current_size as isize);
    (*bw).end_ = ((*bw).buf_).offset(allocated_size as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitWriterInit(
    bw: *mut VP8LBitWriter,
    mut expected_size: size_t,
) -> libc::c_int {
    memset(
        bw as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<VP8LBitWriter>() as libc::c_ulong,
    );
    return VP8LBitWriterResize(bw, expected_size);
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitWriterClone(
    src: *const VP8LBitWriter,
    dst: *mut VP8LBitWriter,
) -> libc::c_int {
    let current_size: size_t = ((*src).cur_).offset_from((*src).buf_) as libc::c_long
        as size_t;
    if VP8LBitWriterResize(dst, current_size) == 0 {
        return 0 as libc::c_int;
    }
    memcpy(
        (*dst).buf_ as *mut libc::c_void,
        (*src).buf_ as *const libc::c_void,
        current_size,
    );
    (*dst).bits_ = (*src).bits_;
    (*dst).used_ = (*src).used_;
    (*dst).error_ = (*src).error_;
    (*dst).cur_ = ((*dst).buf_).offset(current_size as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitWriterWipeOut(bw: *mut VP8LBitWriter) {
    if !bw.is_null() {
        WebPSafeFree((*bw).buf_ as *mut libc::c_void);
        memset(
            bw as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<VP8LBitWriter>() as libc::c_ulong,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitWriterReset(
    bw_init: *const VP8LBitWriter,
    bw: *mut VP8LBitWriter,
) {
    (*bw).bits_ = (*bw_init).bits_;
    (*bw).used_ = (*bw_init).used_;
    (*bw)
        .cur_ = ((*bw).buf_)
        .offset(((*bw_init).cur_).offset_from((*bw_init).buf_) as libc::c_long as isize);
    (*bw).error_ = (*bw_init).error_;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitWriterSwap(
    src: *mut VP8LBitWriter,
    dst: *mut VP8LBitWriter,
) {
    let tmp: VP8LBitWriter = *src;
    *src = *dst;
    *dst = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPutBitsFlushBits(bw: *mut VP8LBitWriter) {
    if ((*bw).cur_).offset(4 as libc::c_int as isize) > (*bw).end_ {
        let extra_size: uint64_t = (((*bw).end_).offset_from((*bw).buf_) as libc::c_long
            as libc::c_ulonglong)
            .wrapping_add(32768 as libc::c_ulonglong) as uint64_t;
        if CheckSizeOverflow(extra_size) == 0 || VP8LBitWriterResize(bw, extra_size) == 0
        {
            (*bw).cur_ = (*bw).buf_;
            (*bw).error_ = 1 as libc::c_int;
            return;
        }
    }
    *((*bw).cur_ as *mut vp8l_wtype_t) = (*bw).bits_ as vp8l_wtype_t;
    (*bw).cur_ = ((*bw).cur_).offset(4 as libc::c_int as isize);
    (*bw).bits_ >>= 32 as libc::c_int;
    (*bw).used_ -= 32 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LPutBitsInternal(
    bw: *mut VP8LBitWriter,
    mut bits: uint32_t,
    mut n_bits: libc::c_int,
) {
    if n_bits > 0 as libc::c_int {
        let mut lbits: vp8l_atype_t = (*bw).bits_;
        let mut used: libc::c_int = (*bw).used_;
        while used >= 32 as libc::c_int {
            if ((*bw).cur_).offset(4 as libc::c_int as isize) > (*bw).end_ {
                let extra_size: uint64_t = (((*bw).end_).offset_from((*bw).buf_)
                    as libc::c_long as libc::c_ulonglong)
                    .wrapping_add(32768 as libc::c_ulonglong) as uint64_t;
                if CheckSizeOverflow(extra_size) == 0
                    || VP8LBitWriterResize(bw, extra_size) == 0
                {
                    (*bw).cur_ = (*bw).buf_;
                    (*bw).error_ = 1 as libc::c_int;
                    return;
                }
            }
            *((*bw).cur_ as *mut vp8l_wtype_t) = lbits as vp8l_wtype_t;
            (*bw).cur_ = ((*bw).cur_).offset(4 as libc::c_int as isize);
            lbits >>= 32 as libc::c_int;
            used -= 32 as libc::c_int;
        }
        (*bw).bits_ = lbits | (bits as vp8l_atype_t) << used;
        (*bw).used_ = used + n_bits;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LBitWriterFinish(bw: *mut VP8LBitWriter) -> *mut uint8_t {
    if VP8LBitWriterResize(
        bw,
        ((*bw).used_ + 7 as libc::c_int >> 3 as libc::c_int) as size_t,
    ) != 0
    {
        while (*bw).used_ > 0 as libc::c_int {
            let fresh3 = (*bw).cur_;
            (*bw).cur_ = ((*bw).cur_).offset(1);
            *fresh3 = (*bw).bits_ as uint8_t;
            (*bw).bits_ >>= 8 as libc::c_int;
            (*bw).used_ -= 8 as libc::c_int;
        }
        (*bw).used_ = 0 as libc::c_int;
    }
    return (*bw).buf_;
}
