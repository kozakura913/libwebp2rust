use ::libc;
extern "C" {
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
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
#[inline]
unsafe extern "C" fn VP8LNearLosslessBits(
    mut near_lossless_quality: libc::c_int,
) -> libc::c_int {
    return 5 as libc::c_int - near_lossless_quality / 20 as libc::c_int;
}
unsafe extern "C" fn FindClosestDiscretized(
    mut a: uint32_t,
    mut bits: libc::c_int,
) -> uint32_t {
    let mask: uint32_t = ((1 as libc::c_uint) << bits)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let biased: uint32_t = a
        .wrapping_add(mask >> 1 as libc::c_int)
        .wrapping_add(a >> bits & 1 as libc::c_int as libc::c_uint);
    if biased > 0xff as libc::c_int as libc::c_uint {
        return 0xff as libc::c_int as uint32_t;
    }
    return biased & !mask;
}
unsafe extern "C" fn ClosestDiscretizedArgb(
    mut a: uint32_t,
    mut bits: libc::c_int,
) -> uint32_t {
    return FindClosestDiscretized(a >> 24 as libc::c_int, bits) << 24 as libc::c_int
        | FindClosestDiscretized(
            a >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
            bits,
        ) << 16 as libc::c_int
        | FindClosestDiscretized(
            a >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
            bits,
        ) << 8 as libc::c_int
        | FindClosestDiscretized(a & 0xff as libc::c_int as libc::c_uint, bits);
}
unsafe extern "C" fn IsNear(
    mut a: uint32_t,
    mut b: uint32_t,
    mut limit: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int {
        let delta: libc::c_int = (a >> k * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_int
            - (b >> k * 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_int;
        if delta >= limit || delta <= -limit {
            return 0 as libc::c_int;
        }
        k += 1;
        k;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn IsSmooth(
    prev_row: *const uint32_t,
    curr_row: *const uint32_t,
    next_row: *const uint32_t,
    mut ix: libc::c_int,
    mut limit: libc::c_int,
) -> libc::c_int {
    return (IsNear(
        *curr_row.offset(ix as isize),
        *curr_row.offset((ix - 1 as libc::c_int) as isize),
        limit,
    ) != 0
        && IsNear(
            *curr_row.offset(ix as isize),
            *curr_row.offset((ix + 1 as libc::c_int) as isize),
            limit,
        ) != 0
        && IsNear(*curr_row.offset(ix as isize), *prev_row.offset(ix as isize), limit)
            != 0
        && IsNear(*curr_row.offset(ix as isize), *next_row.offset(ix as isize), limit)
            != 0) as libc::c_int;
}
unsafe extern "C" fn NearLossless(
    mut xsize: libc::c_int,
    mut ysize: libc::c_int,
    mut argb_src: *const uint32_t,
    mut stride: libc::c_int,
    mut limit_bits: libc::c_int,
    mut copy_buffer: *mut uint32_t,
    mut argb_dst: *mut uint32_t,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let limit: libc::c_int = (1 as libc::c_int) << limit_bits;
    let mut prev_row: *mut uint32_t = copy_buffer;
    let mut curr_row: *mut uint32_t = prev_row.offset(xsize as isize);
    let mut next_row: *mut uint32_t = curr_row.offset(xsize as isize);
    memcpy(
        curr_row as *mut libc::c_void,
        argb_src as *const libc::c_void,
        (xsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    memcpy(
        next_row as *mut libc::c_void,
        argb_src.offset(stride as isize) as *const libc::c_void,
        (xsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    y = 0 as libc::c_int;
    while y < ysize {
        if y == 0 as libc::c_int || y == ysize - 1 as libc::c_int {
            memcpy(
                argb_dst as *mut libc::c_void,
                argb_src as *const libc::c_void,
                (xsize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
        } else {
            memcpy(
                next_row as *mut libc::c_void,
                argb_src.offset(stride as isize) as *const libc::c_void,
                (xsize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
            *argb_dst
                .offset(
                    0 as libc::c_int as isize,
                ) = *argb_src.offset(0 as libc::c_int as isize);
            *argb_dst
                .offset(
                    (xsize - 1 as libc::c_int) as isize,
                ) = *argb_src.offset((xsize - 1 as libc::c_int) as isize);
            x = 1 as libc::c_int;
            while x < xsize - 1 as libc::c_int {
                if IsSmooth(prev_row, curr_row, next_row, x, limit) != 0 {
                    *argb_dst.offset(x as isize) = *curr_row.offset(x as isize);
                } else {
                    *argb_dst
                        .offset(
                            x as isize,
                        ) = ClosestDiscretizedArgb(
                        *curr_row.offset(x as isize),
                        limit_bits,
                    );
                }
                x += 1;
                x;
            }
        }
        let temp: *mut uint32_t = prev_row;
        prev_row = curr_row;
        curr_row = next_row;
        next_row = temp;
        y += 1;
        y;
        argb_src = argb_src.offset(stride as isize);
        argb_dst = argb_dst.offset(xsize as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8ApplyNearLossless(
    picture: *const WebPPicture,
    mut quality: libc::c_int,
    argb_dst: *mut uint32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let xsize: libc::c_int = (*picture).width;
    let ysize: libc::c_int = (*picture).height;
    let stride: libc::c_int = (*picture).argb_stride;
    let copy_buffer: *mut uint32_t = WebPSafeMalloc(
        (xsize * 3 as libc::c_int) as uint64_t,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    let limit_bits: libc::c_int = VP8LNearLosslessBits(quality);
    if copy_buffer.is_null() {
        return 0 as libc::c_int;
    }
    if xsize < 64 as libc::c_int && ysize < 64 as libc::c_int || ysize < 3 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < ysize {
            memcpy(
                argb_dst.offset((i * xsize) as isize) as *mut libc::c_void,
                ((*picture).argb).offset((i * (*picture).argb_stride) as isize)
                    as *const libc::c_void,
                (xsize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
            i += 1;
            i;
        }
        WebPSafeFree(copy_buffer as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    NearLossless(
        xsize,
        ysize,
        (*picture).argb,
        stride,
        limit_bits,
        copy_buffer,
        argb_dst,
    );
    i = limit_bits - 1 as libc::c_int;
    while i != 0 as libc::c_int {
        NearLossless(xsize, ysize, argb_dst, xsize, i, copy_buffer, argb_dst);
        i -= 1;
        i;
    }
    WebPSafeFree(copy_buffer as *mut libc::c_void);
    return 1 as libc::c_int;
}
