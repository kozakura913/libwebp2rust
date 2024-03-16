use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type WEBP_FILTER_TYPE = libc::c_uint;
pub const WEBP_FILTER_FAST: WEBP_FILTER_TYPE = 6;
pub const WEBP_FILTER_BEST: WEBP_FILTER_TYPE = 5;
pub const WEBP_FILTER_LAST: WEBP_FILTER_TYPE = 4;
pub const WEBP_FILTER_GRADIENT: WEBP_FILTER_TYPE = 3;
pub const WEBP_FILTER_VERTICAL: WEBP_FILTER_TYPE = 2;
pub const WEBP_FILTER_HORIZONTAL: WEBP_FILTER_TYPE = 1;
pub const WEBP_FILTER_NONE: WEBP_FILTER_TYPE = 0;
#[inline]
unsafe extern "C" fn GradientPredictor(
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
#[no_mangle]
pub unsafe extern "C" fn WebPEstimateBestFilter(
    mut data: *const uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
) -> WEBP_FILTER_TYPE {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut bins: [[libc::c_int; 16]; 4] = [[0; 16]; 4];
    memset(
        bins.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[libc::c_int; 16]; 4]>() as libc::c_ulong,
    );
    j = 2 as libc::c_int;
    while j < height - 1 as libc::c_int {
        let p: *const uint8_t = data.offset((j * stride) as isize);
        let mut mean: libc::c_int = *p.offset(0 as libc::c_int as isize) as libc::c_int;
        i = 2 as libc::c_int;
        while i < width - 1 as libc::c_int {
            let diff0: libc::c_int = abs(*p.offset(i as isize) as libc::c_int - mean)
                >> 4 as libc::c_int;
            let diff1: libc::c_int = abs(
                *p.offset(i as isize) as libc::c_int
                    - *p.offset((i - 1 as libc::c_int) as isize) as libc::c_int,
            ) >> 4 as libc::c_int;
            let diff2: libc::c_int = abs(
                *p.offset(i as isize) as libc::c_int
                    - *p.offset((i - width) as isize) as libc::c_int,
            ) >> 4 as libc::c_int;
            let grad_pred: libc::c_int = GradientPredictor(
                *p.offset((i - 1 as libc::c_int) as isize),
                *p.offset((i - width) as isize),
                *p.offset((i - width - 1 as libc::c_int) as isize),
            );
            let diff3: libc::c_int = abs(
                *p.offset(i as isize) as libc::c_int - grad_pred,
            ) >> 4 as libc::c_int;
            bins[WEBP_FILTER_NONE as libc::c_int
                as usize][diff0 as usize] = 1 as libc::c_int;
            bins[WEBP_FILTER_HORIZONTAL as libc::c_int
                as usize][diff1 as usize] = 1 as libc::c_int;
            bins[WEBP_FILTER_VERTICAL as libc::c_int
                as usize][diff2 as usize] = 1 as libc::c_int;
            bins[WEBP_FILTER_GRADIENT as libc::c_int
                as usize][diff3 as usize] = 1 as libc::c_int;
            mean = 3 as libc::c_int * mean + *p.offset(i as isize) as libc::c_int
                + 2 as libc::c_int >> 2 as libc::c_int;
            i += 2 as libc::c_int;
        }
        j += 2 as libc::c_int;
    }
    let mut filter: libc::c_int = 0;
    let mut best_filter: WEBP_FILTER_TYPE = WEBP_FILTER_NONE;
    let mut best_score: libc::c_int = 0x7fffffff as libc::c_int;
    filter = WEBP_FILTER_NONE as libc::c_int;
    while filter < WEBP_FILTER_LAST as libc::c_int {
        let mut score: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if bins[filter as usize][i as usize] > 0 as libc::c_int {
                score += i;
            }
            i += 1;
            i;
        }
        if score < best_score {
            best_score = score;
            best_filter = filter as WEBP_FILTER_TYPE;
        }
        filter += 1;
        filter;
    }
    return best_filter;
}
