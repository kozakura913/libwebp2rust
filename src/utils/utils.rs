use ::libc;

use super::types::WebPPicture;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn GetColorPalette(pic: *const WebPPicture, palette: *mut uint32_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

#[inline]
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> libc::c_int {
    return (size == size) as libc::c_int;
}
unsafe extern "C" fn CheckSizeArgumentsOverflow(
    mut nmemb: uint64_t,
    mut size: size_t,
) -> libc::c_int {
    let total_size: uint64_t = nmemb.wrapping_mul(size);
    if nmemb == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if size as libc::c_ulonglong
        > ((1 as libc::c_ulonglong) << 34 as libc::c_int)
            .wrapping_div(nmemb as libc::c_ulonglong)
    {
        return 0 as libc::c_int;
    }
    if CheckSizeOverflow(total_size) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPSafeMalloc(
    mut nmemb: uint64_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if CheckSizeArgumentsOverflow(nmemb, size) == 0 {
        return 0 as *mut libc::c_void;
    }
    ptr = malloc(nmemb.wrapping_mul(size));
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn WebPSafeCalloc(
    mut nmemb: uint64_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if CheckSizeArgumentsOverflow(nmemb, size) == 0 {
        return 0 as *mut libc::c_void;
    }
    ptr = calloc(nmemb, size);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn WebPSafeFree(ptr: *mut libc::c_void) {
    !ptr.is_null();
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMalloc(mut size: size_t) -> *mut libc::c_void {
    return WebPSafeMalloc(1 as libc::c_int as uint64_t, size);
}
#[no_mangle]
pub unsafe extern "C" fn WebPFree(mut ptr: *mut libc::c_void) {
    WebPSafeFree(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn WebPCopyPlane(
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    loop {
        let fresh0 = height;
        height = height - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            width as libc::c_ulong,
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPCopyPixels(src: *const WebPPicture, dst: *mut WebPPicture) {
    WebPCopyPlane(
        (*src).argb as *mut uint8_t,
        4 as libc::c_int * (*src).argb_stride,
        (*dst).argb as *mut uint8_t,
        4 as libc::c_int * (*dst).argb_stride,
        4 as libc::c_int * (*src).width,
        (*src).height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPGetColorPalette(
    pic: *const WebPPicture,
    palette: *mut uint32_t,
) -> libc::c_int {
    return GetColorPalette(pic, palette);
}
