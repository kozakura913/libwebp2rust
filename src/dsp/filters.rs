use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const WEBP_FILTER_FAST: C2RustUnnamed_0 = 6;
pub const WEBP_FILTER_BEST: C2RustUnnamed_0 = 5;
pub const WEBP_FILTER_LAST: C2RustUnnamed_0 = 4;
pub const WEBP_FILTER_GRADIENT: C2RustUnnamed_0 = 3;
pub const WEBP_FILTER_VERTICAL: C2RustUnnamed_0 = 2;
pub const WEBP_FILTER_HORIZONTAL: C2RustUnnamed_0 = 1;
pub const WEBP_FILTER_NONE: C2RustUnnamed_0 = 0;
pub type WebPFilterFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
    ) -> (),
>;
pub type WebPUnfilterFunc = Option::<
    unsafe extern "C" fn(*const uint8_t, *const uint8_t, *mut uint8_t, libc::c_int) -> (),
>;
#[inline]
unsafe extern "C" fn PredictLine_C(
    mut src: *const uint8_t,
    mut pred: *const uint8_t,
    mut dst: *mut uint8_t,
    mut length: libc::c_int,
    mut inverse: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if inverse != 0 {
        i = 0 as libc::c_int;
        while i < length {
            *dst
                .offset(
                    i as isize,
                ) = (*src.offset(i as isize) as libc::c_int
                + *pred.offset(i as isize) as libc::c_int) as uint8_t;
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < length {
            *dst
                .offset(
                    i as isize,
                ) = (*src.offset(i as isize) as libc::c_int
                - *pred.offset(i as isize) as libc::c_int) as uint8_t;
            i += 1;
            i;
        }
    };
}
#[inline]
unsafe extern "C" fn DoHorizontalFilter_C(
    mut in_0: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut row: libc::c_int,
    mut num_rows: libc::c_int,
    mut inverse: libc::c_int,
    mut out: *mut uint8_t,
) {
    let mut preds: *const uint8_t = 0 as *const uint8_t;
    let start_offset: size_t = (row * stride) as size_t;
    let last_row: libc::c_int = row + num_rows;
    in_0 = in_0.offset(start_offset as isize);
    out = out.offset(start_offset as isize);
    preds = if inverse != 0 { out as *const uint8_t } else { in_0 };
    if row == 0 as libc::c_int {
        *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize);
        PredictLine_C(
            in_0.offset(1 as libc::c_int as isize),
            preds,
            out.offset(1 as libc::c_int as isize),
            width - 1 as libc::c_int,
            inverse,
        );
        row = 1 as libc::c_int;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
    while row < last_row {
        PredictLine_C(
            in_0,
            preds.offset(-(stride as isize)),
            out,
            1 as libc::c_int,
            inverse,
        );
        PredictLine_C(
            in_0.offset(1 as libc::c_int as isize),
            preds,
            out.offset(1 as libc::c_int as isize),
            width - 1 as libc::c_int,
            inverse,
        );
        row += 1;
        row;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
}
#[inline]
unsafe extern "C" fn DoVerticalFilter_C(
    mut in_0: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut row: libc::c_int,
    mut num_rows: libc::c_int,
    mut inverse: libc::c_int,
    mut out: *mut uint8_t,
) {
    let mut preds: *const uint8_t = 0 as *const uint8_t;
    let start_offset: size_t = (row * stride) as size_t;
    let last_row: libc::c_int = row + num_rows;
    in_0 = in_0.offset(start_offset as isize);
    out = out.offset(start_offset as isize);
    preds = if inverse != 0 { out as *const uint8_t } else { in_0 };
    if row == 0 as libc::c_int {
        *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize);
        PredictLine_C(
            in_0.offset(1 as libc::c_int as isize),
            preds,
            out.offset(1 as libc::c_int as isize),
            width - 1 as libc::c_int,
            inverse,
        );
        row = 1 as libc::c_int;
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    } else {
        preds = preds.offset(-(stride as isize));
    }
    while row < last_row {
        PredictLine_C(in_0, preds, out, width, inverse);
        row += 1;
        row;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
}
#[inline]
unsafe extern "C" fn GradientPredictor_C(
    mut a: uint8_t,
    mut b: uint8_t,
    mut c: uint8_t,
) -> libc::c_int {
    let g: libc::c_int = a as libc::c_int + b as libc::c_int - c as libc::c_int;
    return if g & !(0xff as libc::c_int) == 0 as libc::c_int {
        g
    } else if g < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        255 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn DoGradientFilter_C(
    mut in_0: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut row: libc::c_int,
    mut num_rows: libc::c_int,
    mut inverse: libc::c_int,
    mut out: *mut uint8_t,
) {
    let mut preds: *const uint8_t = 0 as *const uint8_t;
    let start_offset: size_t = (row * stride) as size_t;
    let last_row: libc::c_int = row + num_rows;
    in_0 = in_0.offset(start_offset as isize);
    out = out.offset(start_offset as isize);
    preds = if inverse != 0 { out as *const uint8_t } else { in_0 };
    if row == 0 as libc::c_int {
        *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize);
        PredictLine_C(
            in_0.offset(1 as libc::c_int as isize),
            preds,
            out.offset(1 as libc::c_int as isize),
            width - 1 as libc::c_int,
            inverse,
        );
        row = 1 as libc::c_int;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
    while row < last_row {
        let mut w: libc::c_int = 0;
        PredictLine_C(
            in_0,
            preds.offset(-(stride as isize)),
            out,
            1 as libc::c_int,
            inverse,
        );
        w = 1 as libc::c_int;
        while w < width {
            let pred: libc::c_int = GradientPredictor_C(
                *preds.offset((w - 1 as libc::c_int) as isize),
                *preds.offset((w - stride) as isize),
                *preds.offset((w - stride - 1 as libc::c_int) as isize),
            );
            *out
                .offset(
                    w as isize,
                ) = (*in_0.offset(w as isize) as libc::c_int
                + (if inverse != 0 { pred } else { -pred })) as uint8_t;
            w += 1;
            w;
        }
        row += 1;
        row;
        preds = preds.offset(stride as isize);
        in_0 = in_0.offset(stride as isize);
        out = out.offset(stride as isize);
    }
}
unsafe extern "C" fn HorizontalFilter_C(
    mut data: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut filtered_data: *mut uint8_t,
) {
    DoHorizontalFilter_C(
        data,
        width,
        height,
        stride,
        0 as libc::c_int,
        height,
        0 as libc::c_int,
        filtered_data,
    );
}
unsafe extern "C" fn VerticalFilter_C(
    mut data: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut filtered_data: *mut uint8_t,
) {
    DoVerticalFilter_C(
        data,
        width,
        height,
        stride,
        0 as libc::c_int,
        height,
        0 as libc::c_int,
        filtered_data,
    );
}
unsafe extern "C" fn GradientFilter_C(
    mut data: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut filtered_data: *mut uint8_t,
) {
    DoGradientFilter_C(
        data,
        width,
        height,
        stride,
        0 as libc::c_int,
        height,
        0 as libc::c_int,
        filtered_data,
    );
}
unsafe extern "C" fn NoneUnfilter_C(
    mut prev: *const uint8_t,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
    mut width: libc::c_int,
) {
    if out != in_0 as *mut uint8_t {
        memcpy(
            out as *mut libc::c_void,
            in_0 as *const libc::c_void,
            (width as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
    }
}
unsafe extern "C" fn HorizontalUnfilter_C(
    mut prev: *const uint8_t,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
    mut width: libc::c_int,
) {
    let mut pred: uint8_t = (if prev.is_null() {
        0 as libc::c_int
    } else {
        *prev.offset(0 as libc::c_int as isize) as libc::c_int
    }) as uint8_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < width {
        *out
            .offset(
                i as isize,
            ) = (pred as libc::c_int + *in_0.offset(i as isize) as libc::c_int)
            as uint8_t;
        pred = *out.offset(i as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn VerticalUnfilter_C(
    mut prev: *const uint8_t,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
    mut width: libc::c_int,
) {
    if prev.is_null() {
        HorizontalUnfilter_C(0 as *const uint8_t, in_0, out, width);
    } else {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < width {
            *out
                .offset(
                    i as isize,
                ) = (*prev.offset(i as isize) as libc::c_int
                + *in_0.offset(i as isize) as libc::c_int) as uint8_t;
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn GradientUnfilter_C(
    mut prev: *const uint8_t,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
    mut width: libc::c_int,
) {
    if prev.is_null() {
        HorizontalUnfilter_C(0 as *const uint8_t, in_0, out, width);
    } else {
        let mut top: uint8_t = *prev.offset(0 as libc::c_int as isize);
        let mut top_left: uint8_t = top;
        let mut left: uint8_t = top;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < width {
            top = *prev.offset(i as isize);
            left = (*in_0.offset(i as isize) as libc::c_int
                + GradientPredictor_C(left, top, top_left)) as uint8_t;
            top_left = top;
            *out.offset(i as isize) = left;
            i += 1;
            i;
        }
    };
}
#[no_mangle]
pub static mut WebPFilters: [WebPFilterFunc; 4] = [None; 4];
#[no_mangle]
pub static mut WebPUnfilters: [WebPUnfilterFunc; 4] = [None; 4];
#[no_mangle]
pub unsafe extern "C" fn VP8FiltersInit() {
    static mut VP8FiltersInit_body_lock: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int as libc::c_short,
                __elision: 0 as libc::c_int as libc::c_short,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    if !(pthread_mutex_lock(&mut VP8FiltersInit_body_lock) != 0) {
        VP8FiltersInit_body();
        pthread_mutex_unlock(&mut VP8FiltersInit_body_lock);
    }
}
unsafe extern "C" fn VP8FiltersInit_body() {
    WebPUnfilters[WEBP_FILTER_NONE as libc::c_int
        as usize] = Some(
        NoneUnfilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUnfilters[WEBP_FILTER_HORIZONTAL as libc::c_int
        as usize] = Some(
        HorizontalUnfilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUnfilters[WEBP_FILTER_VERTICAL as libc::c_int
        as usize] = Some(
        VerticalUnfilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPUnfilters[WEBP_FILTER_GRADIENT as libc::c_int
        as usize] = Some(
        GradientUnfilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *const uint8_t,
                *mut uint8_t,
                libc::c_int,
            ) -> (),
    );
    WebPFilters[WEBP_FILTER_NONE as libc::c_int as usize] = None;
    WebPFilters[WEBP_FILTER_HORIZONTAL as libc::c_int
        as usize] = Some(
        HorizontalFilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
            ) -> (),
    );
    WebPFilters[WEBP_FILTER_VERTICAL as libc::c_int
        as usize] = Some(
        VerticalFilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
            ) -> (),
    );
    WebPFilters[WEBP_FILTER_GRADIENT as libc::c_int
        as usize] = Some(
        GradientFilter_C
            as unsafe extern "C" fn(
                *const uint8_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
            ) -> (),
    );
}
