use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8LColorCache {
    pub colors_: *mut uint32_t,
    pub hash_shift_: libc::c_int,
    pub hash_bits_: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorCacheInit(
    color_cache: *mut VP8LColorCache,
    mut hash_bits: libc::c_int,
) -> libc::c_int {
    let hash_size: libc::c_int = (1 as libc::c_int) << hash_bits;
    (*color_cache)
        .colors_ = WebPSafeCalloc(
        hash_size as uint64_t,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    if ((*color_cache).colors_).is_null() {
        return 0 as libc::c_int;
    }
    (*color_cache).hash_shift_ = 32 as libc::c_int - hash_bits;
    (*color_cache).hash_bits_ = hash_bits;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorCacheClear(color_cache: *mut VP8LColorCache) {
    if !color_cache.is_null() {
        WebPSafeFree((*color_cache).colors_ as *mut libc::c_void);
        (*color_cache).colors_ = 0 as *mut uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VP8LColorCacheCopy(
    src: *const VP8LColorCache,
    dst: *mut VP8LColorCache,
) {
    memcpy(
        (*dst).colors_ as *mut libc::c_void,
        (*src).colors_ as *const libc::c_void,
        ((1 as libc::c_uint as size_t) << (*dst).hash_bits_)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
}
