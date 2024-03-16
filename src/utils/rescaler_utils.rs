use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPRescalerImportRow(wrk: *mut WebPRescaler, src: *const uint8_t);
    fn WebPRescalerExportRow(wrk: *mut WebPRescaler);
    fn WebPRescalerDspInit();
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
pub type rescaler_t = uint32_t;
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
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> libc::c_int {
    return (size == size) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerInit(
    rescaler: *mut WebPRescaler,
    mut src_width: libc::c_int,
    mut src_height: libc::c_int,
    dst: *mut uint8_t,
    mut dst_width: libc::c_int,
    mut dst_height: libc::c_int,
    mut dst_stride: libc::c_int,
    mut num_channels: libc::c_int,
    work: *mut rescaler_t,
) -> libc::c_int {
    let x_add: libc::c_int = src_width;
    let x_sub: libc::c_int = dst_width;
    let y_add: libc::c_int = src_height;
    let y_sub: libc::c_int = dst_height;
    let total_size: uint64_t = (2 as libc::c_ulonglong)
        .wrapping_mul(dst_width as libc::c_ulonglong)
        .wrapping_mul(num_channels as libc::c_ulonglong)
        .wrapping_mul(
            ::core::mem::size_of::<rescaler_t>() as libc::c_ulong as libc::c_ulonglong,
        ) as uint64_t;
    if CheckSizeOverflow(total_size) == 0 {
        return 0 as libc::c_int;
    }
    (*rescaler).x_expand = (src_width < dst_width) as libc::c_int;
    (*rescaler).y_expand = (src_height < dst_height) as libc::c_int;
    (*rescaler).src_width = src_width;
    (*rescaler).src_height = src_height;
    (*rescaler).dst_width = dst_width;
    (*rescaler).dst_height = dst_height;
    (*rescaler).src_y = 0 as libc::c_int;
    (*rescaler).dst_y = 0 as libc::c_int;
    (*rescaler).dst = dst;
    (*rescaler).dst_stride = dst_stride;
    (*rescaler).num_channels = num_channels;
    (*rescaler)
        .x_add = if (*rescaler).x_expand != 0 {
        x_sub - 1 as libc::c_int
    } else {
        x_add
    };
    (*rescaler)
        .x_sub = if (*rescaler).x_expand != 0 {
        x_add - 1 as libc::c_int
    } else {
        x_sub
    };
    if (*rescaler).x_expand == 0 {
        (*rescaler)
            .fx_scale = ((1 as libc::c_int as uint64_t) << 32 as libc::c_int)
            .wrapping_div((*rescaler).x_sub as libc::c_ulong) as uint32_t;
    }
    (*rescaler)
        .y_add = if (*rescaler).y_expand != 0 {
        y_add - 1 as libc::c_int
    } else {
        y_add
    };
    (*rescaler)
        .y_sub = if (*rescaler).y_expand != 0 {
        y_sub - 1 as libc::c_int
    } else {
        y_sub
    };
    (*rescaler)
        .y_accum = if (*rescaler).y_expand != 0 {
        (*rescaler).y_sub
    } else {
        (*rescaler).y_add
    };
    if (*rescaler).y_expand == 0 {
        let num: uint64_t = (dst_height as uint64_t as libc::c_ulonglong)
            .wrapping_mul((1 as libc::c_ulonglong) << 32 as libc::c_int) as uint64_t;
        let den: uint64_t = ((*rescaler).x_add as uint64_t)
            .wrapping_mul((*rescaler).y_add as libc::c_ulong);
        let ratio: uint64_t = num.wrapping_div(den);
        if ratio != ratio as uint32_t as libc::c_ulong {
            (*rescaler).fxy_scale = 0 as libc::c_int as uint32_t;
        } else {
            (*rescaler).fxy_scale = ratio as uint32_t;
        }
        (*rescaler)
            .fy_scale = ((1 as libc::c_int as uint64_t) << 32 as libc::c_int)
            .wrapping_div((*rescaler).y_sub as libc::c_ulong) as uint32_t;
    } else {
        (*rescaler)
            .fy_scale = ((1 as libc::c_int as uint64_t) << 32 as libc::c_int)
            .wrapping_div((*rescaler).x_add as libc::c_ulong) as uint32_t;
    }
    (*rescaler).irow = work;
    (*rescaler).frow = work.offset((num_channels * dst_width) as isize);
    memset(work as *mut libc::c_void, 0 as libc::c_int, total_size);
    WebPRescalerDspInit();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerGetScaledDimensions(
    mut src_width: libc::c_int,
    mut src_height: libc::c_int,
    scaled_width: *mut libc::c_int,
    scaled_height: *mut libc::c_int,
) -> libc::c_int {
    let mut width: libc::c_int = *scaled_width;
    let mut height: libc::c_int = *scaled_height;
    let max_size: libc::c_int = 2147483647 as libc::c_int / 2 as libc::c_int;
    if width == 0 as libc::c_int && src_height > 0 as libc::c_int {
        width = (src_width as uint64_t)
            .wrapping_mul(height as libc::c_ulong)
            .wrapping_add(src_height as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(src_height as libc::c_ulong) as libc::c_int;
    }
    if height == 0 as libc::c_int && src_width > 0 as libc::c_int {
        height = (src_height as uint64_t)
            .wrapping_mul(width as libc::c_ulong)
            .wrapping_add(src_width as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(src_width as libc::c_ulong) as libc::c_int;
    }
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int || width > max_size
        || height > max_size
    {
        return 0 as libc::c_int;
    }
    *scaled_width = width;
    *scaled_height = height;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescaleNeededLines(
    rescaler: *const WebPRescaler,
    mut max_num_lines: libc::c_int,
) -> libc::c_int {
    let num_lines: libc::c_int = ((*rescaler).y_accum + (*rescaler).y_sub
        - 1 as libc::c_int) / (*rescaler).y_sub;
    return if num_lines > max_num_lines { max_num_lines } else { num_lines };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImport(
    rescaler: *mut WebPRescaler,
    mut num_lines: libc::c_int,
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
) -> libc::c_int {
    let mut total_imported: libc::c_int = 0 as libc::c_int;
    while total_imported < num_lines && WebPRescalerHasPendingOutput(rescaler) == 0 {
        if (*rescaler).y_expand != 0 {
            let tmp: *mut rescaler_t = (*rescaler).irow;
            (*rescaler).irow = (*rescaler).frow;
            (*rescaler).frow = tmp;
        }
        WebPRescalerImportRow(rescaler, src);
        if (*rescaler).y_expand == 0 {
            let mut x: libc::c_int = 0;
            x = 0 as libc::c_int;
            while x < (*rescaler).num_channels * (*rescaler).dst_width {
                let ref mut fresh0 = *((*rescaler).irow).offset(x as isize);
                *fresh0 = (*fresh0 as libc::c_uint)
                    .wrapping_add(*((*rescaler).frow).offset(x as isize)) as rescaler_t
                    as rescaler_t;
                x += 1;
                x;
            }
        }
        (*rescaler).src_y += 1;
        (*rescaler).src_y;
        src = src.offset(src_stride as isize);
        total_imported += 1;
        total_imported;
        (*rescaler).y_accum -= (*rescaler).y_sub;
    }
    return total_imported;
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExport(rescaler: *mut WebPRescaler) -> libc::c_int {
    let mut total_exported: libc::c_int = 0 as libc::c_int;
    while WebPRescalerHasPendingOutput(rescaler) != 0 {
        WebPRescalerExportRow(rescaler);
        total_exported += 1;
        total_exported;
    }
    return total_exported;
}
