use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type WebPRescalerImportRowFunc = Option::<
    unsafe extern "C" fn(*mut WebPRescaler, *const uint8_t) -> (),
>;
pub type WebPRescalerExportRowFunc = Option::<
    unsafe extern "C" fn(*mut WebPRescaler) -> (),
>;
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImportRowExpand_C(
    wrk: *mut WebPRescaler,
    mut src: *const uint8_t,
) {
    let x_stride: libc::c_int = (*wrk).num_channels;
    let x_out_max: libc::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let mut channel: libc::c_int = 0;
    channel = 0 as libc::c_int;
    while channel < x_stride {
        let mut x_in: libc::c_int = channel;
        let mut x_out: libc::c_int = channel;
        let mut accum: libc::c_int = (*wrk).x_add;
        let mut left: rescaler_t = *src.offset(x_in as isize) as rescaler_t;
        let mut right: rescaler_t = if (*wrk).src_width > 1 as libc::c_int {
            *src.offset((x_in + x_stride) as isize) as rescaler_t
        } else {
            left
        };
        x_in += x_stride;
        loop {
            *((*wrk).frow)
                .offset(
                    x_out as isize,
                ) = right
                .wrapping_mul((*wrk).x_add as libc::c_uint)
                .wrapping_add(
                    left.wrapping_sub(right).wrapping_mul(accum as libc::c_uint),
                );
            x_out += x_stride;
            if x_out >= x_out_max {
                break;
            }
            accum -= (*wrk).x_sub;
            if accum < 0 as libc::c_int {
                left = right;
                x_in += x_stride;
                right = *src.offset(x_in as isize) as rescaler_t;
                accum += (*wrk).x_add;
            }
        }
        channel += 1;
        channel;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImportRowShrink_C(
    wrk: *mut WebPRescaler,
    mut src: *const uint8_t,
) {
    let x_stride: libc::c_int = (*wrk).num_channels;
    let x_out_max: libc::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let mut channel: libc::c_int = 0;
    channel = 0 as libc::c_int;
    while channel < x_stride {
        let mut x_in: libc::c_int = channel;
        let mut x_out: libc::c_int = channel;
        let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
        let mut accum: libc::c_int = 0 as libc::c_int;
        while x_out < x_out_max {
            let mut base: uint32_t = 0 as libc::c_int as uint32_t;
            accum += (*wrk).x_add;
            while accum > 0 as libc::c_int {
                accum -= (*wrk).x_sub;
                base = *src.offset(x_in as isize) as uint32_t;
                sum = (sum as libc::c_uint).wrapping_add(base) as uint32_t as uint32_t;
                x_in += x_stride;
            }
            let frac: rescaler_t = base.wrapping_mul(-accum as libc::c_uint);
            *((*wrk).frow)
                .offset(
                    x_out as isize,
                ) = sum.wrapping_mul((*wrk).x_sub as libc::c_uint).wrapping_sub(frac);
            sum = (((frac as uint64_t).wrapping_mul((*wrk).fx_scale as libc::c_ulong)
                as libc::c_ulonglong)
                .wrapping_add(
                    (1 as libc::c_ulonglong) << 32 as libc::c_int >> 1 as libc::c_int,
                ) >> 32 as libc::c_int) as libc::c_int as uint32_t;
            x_out += x_stride;
        }
        channel += 1;
        channel;
    }
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExportRowExpand_C(wrk: *mut WebPRescaler) {
    let mut x_out: libc::c_int = 0;
    let dst: *mut uint8_t = (*wrk).dst;
    let irow: *mut rescaler_t = (*wrk).irow;
    let x_out_max: libc::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let frow: *const rescaler_t = (*wrk).frow;
    if (*wrk).y_accum == 0 as libc::c_int {
        x_out = 0 as libc::c_int;
        while x_out < x_out_max {
            let J: uint32_t = *frow.offset(x_out as isize);
            let v: libc::c_int = (((J as uint64_t)
                .wrapping_mul((*wrk).fy_scale as libc::c_ulong) as libc::c_ulonglong)
                .wrapping_add(
                    (1 as libc::c_ulonglong) << 32 as libc::c_int >> 1 as libc::c_int,
                ) >> 32 as libc::c_int) as libc::c_int;
            *dst
                .offset(
                    x_out as isize,
                ) = (if v > 255 as libc::c_int {
                255 as libc::c_uint
            } else {
                v as uint8_t as libc::c_uint
            }) as uint8_t;
            x_out += 1;
            x_out;
        }
    } else {
        let B: uint32_t = ((-(*wrk).y_accum as uint64_t) << 32 as libc::c_int)
            .wrapping_div((*wrk).y_sub as libc::c_ulong) as uint32_t;
        let A: uint32_t = ((1 as libc::c_ulonglong) << 32 as libc::c_int)
            .wrapping_sub(B as libc::c_ulonglong) as uint32_t;
        x_out = 0 as libc::c_int;
        while x_out < x_out_max {
            let I: uint64_t = (A as uint64_t)
                .wrapping_mul(*frow.offset(x_out as isize) as libc::c_ulong)
                .wrapping_add(
                    (B as uint64_t)
                        .wrapping_mul(*irow.offset(x_out as isize) as libc::c_ulong),
                );
            let J_0: uint32_t = ((I as libc::c_ulonglong)
                .wrapping_add(
                    (1 as libc::c_ulonglong) << 32 as libc::c_int >> 1 as libc::c_int,
                ) >> 32 as libc::c_int) as uint32_t;
            let v_0: libc::c_int = (((J_0 as uint64_t)
                .wrapping_mul((*wrk).fy_scale as libc::c_ulong) as libc::c_ulonglong)
                .wrapping_add(
                    (1 as libc::c_ulonglong) << 32 as libc::c_int >> 1 as libc::c_int,
                ) >> 32 as libc::c_int) as libc::c_int;
            *dst
                .offset(
                    x_out as isize,
                ) = (if v_0 > 255 as libc::c_int {
                255 as libc::c_uint
            } else {
                v_0 as uint8_t as libc::c_uint
            }) as uint8_t;
            x_out += 1;
            x_out;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExportRowShrink_C(wrk: *mut WebPRescaler) {
    let mut x_out: libc::c_int = 0;
    let dst: *mut uint8_t = (*wrk).dst;
    let irow: *mut rescaler_t = (*wrk).irow;
    let x_out_max: libc::c_int = (*wrk).dst_width * (*wrk).num_channels;
    let frow: *const rescaler_t = (*wrk).frow;
    let yscale: uint32_t = ((*wrk).fy_scale)
        .wrapping_mul(-(*wrk).y_accum as libc::c_uint);
    if yscale != 0 {
        x_out = 0 as libc::c_int;
        while x_out < x_out_max {
            let frac: uint32_t = ((*frow.offset(x_out as isize) as uint64_t)
                .wrapping_mul(yscale as libc::c_ulong) >> 32 as libc::c_int) as uint32_t;
            let v: libc::c_int = ((((*irow.offset(x_out as isize)).wrapping_sub(frac)
                as uint64_t)
                .wrapping_mul((*wrk).fxy_scale as libc::c_ulong) as libc::c_ulonglong)
                .wrapping_add(
                    (1 as libc::c_ulonglong) << 32 as libc::c_int >> 1 as libc::c_int,
                ) >> 32 as libc::c_int) as libc::c_int;
            *dst
                .offset(
                    x_out as isize,
                ) = (if v > 255 as libc::c_int {
                255 as libc::c_uint
            } else {
                v as uint8_t as libc::c_uint
            }) as uint8_t;
            *irow.offset(x_out as isize) = frac;
            x_out += 1;
            x_out;
        }
    } else {
        x_out = 0 as libc::c_int;
        while x_out < x_out_max {
            let v_0: libc::c_int = (((*irow.offset(x_out as isize) as uint64_t)
                .wrapping_mul((*wrk).fxy_scale as libc::c_ulong) as libc::c_ulonglong)
                .wrapping_add(
                    (1 as libc::c_ulonglong) << 32 as libc::c_int >> 1 as libc::c_int,
                ) >> 32 as libc::c_int) as libc::c_int;
            *dst
                .offset(
                    x_out as isize,
                ) = (if v_0 > 255 as libc::c_int {
                255 as libc::c_uint
            } else {
                v_0 as uint8_t as libc::c_uint
            }) as uint8_t;
            *irow.offset(x_out as isize) = 0 as libc::c_int as rescaler_t;
            x_out += 1;
            x_out;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerImportRow(
    wrk: *mut WebPRescaler,
    mut src: *const uint8_t,
) {
    if (*wrk).x_expand == 0 {
        WebPRescalerImportRowShrink.expect("non-null function pointer")(wrk, src);
    } else {
        WebPRescalerImportRowExpand.expect("non-null function pointer")(wrk, src);
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerExportRow(wrk: *mut WebPRescaler) {
    if (*wrk).y_accum <= 0 as libc::c_int {
        if (*wrk).y_expand != 0 {
            WebPRescalerExportRowExpand.expect("non-null function pointer")(wrk);
        } else if (*wrk).fxy_scale != 0 {
            WebPRescalerExportRowShrink.expect("non-null function pointer")(wrk);
        } else {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*wrk).num_channels * (*wrk).dst_width {
                *((*wrk).dst)
                    .offset(i as isize) = *((*wrk).irow).offset(i as isize) as uint8_t;
                *((*wrk).irow).offset(i as isize) = 0 as libc::c_int as rescaler_t;
                i += 1;
                i;
            }
        }
        (*wrk).y_accum += (*wrk).y_add;
        (*wrk).dst = ((*wrk).dst).offset((*wrk).dst_stride as isize);
        (*wrk).dst_y += 1;
        (*wrk).dst_y;
    }
}
#[no_mangle]
pub static mut WebPRescalerImportRowExpand: WebPRescalerImportRowFunc = None;
#[no_mangle]
pub static mut WebPRescalerImportRowShrink: WebPRescalerImportRowFunc = None;
#[no_mangle]
pub static mut WebPRescalerExportRowExpand: WebPRescalerExportRowFunc = None;
#[no_mangle]
pub static mut WebPRescalerExportRowShrink: WebPRescalerExportRowFunc = None;
unsafe extern "C" fn WebPRescalerDspInit_body() {
    WebPRescalerExportRowExpand = Some(
        WebPRescalerExportRowExpand_C as unsafe extern "C" fn(*mut WebPRescaler) -> (),
    );
    WebPRescalerExportRowShrink = Some(
        WebPRescalerExportRowShrink_C as unsafe extern "C" fn(*mut WebPRescaler) -> (),
    );
    WebPRescalerImportRowExpand = Some(
        WebPRescalerImportRowExpand_C
            as unsafe extern "C" fn(*mut WebPRescaler, *const uint8_t) -> (),
    );
    WebPRescalerImportRowShrink = Some(
        WebPRescalerImportRowShrink_C
            as unsafe extern "C" fn(*mut WebPRescaler, *const uint8_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPRescalerDspInit() {
    static mut WebPRescalerDspInit_body_lock: pthread_mutex_t = pthread_mutex_t {
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
    if !(pthread_mutex_lock(&mut WebPRescalerDspInit_body_lock) != 0) {
        WebPRescalerDspInit_body();
        pthread_mutex_unlock(&mut WebPRescalerDspInit_body_lock);
    }
}
