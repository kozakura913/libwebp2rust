use ::libc;
extern "C" {
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
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SmoothParams {
    pub width_: libc::c_int,
    pub height_: libc::c_int,
    pub stride_: libc::c_int,
    pub row_: libc::c_int,
    pub src_: *mut uint8_t,
    pub dst_: *mut uint8_t,
    pub radius_: libc::c_int,
    pub scale_: libc::c_int,
    pub mem_: *mut libc::c_void,
    pub start_: *mut uint16_t,
    pub cur_: *mut uint16_t,
    pub end_: *mut uint16_t,
    pub top_: *mut uint16_t,
    pub average_: *mut uint16_t,
    pub num_levels_: libc::c_int,
    pub min_: libc::c_int,
    pub max_: libc::c_int,
    pub min_level_dist_: libc::c_int,
    pub correction_: *mut int16_t,
}
#[inline]
unsafe extern "C" fn clip_8b(mut v: libc::c_int) -> uint8_t {
    return (if v
        & (!(0 as libc::c_uint) << 8 as libc::c_int + 0 as libc::c_int) as libc::c_int
        == 0
    {
        (v >> 0 as libc::c_int) as uint8_t as libc::c_uint
    } else if v < 0 as libc::c_int {
        0 as libc::c_uint
    } else {
        255 as libc::c_uint
    }) as uint8_t;
}
unsafe extern "C" fn VFilter(p: *mut SmoothParams) {
    let mut src: *const uint8_t = (*p).src_;
    let w: libc::c_int = (*p).width_;
    let cur: *mut uint16_t = (*p).cur_;
    let top: *const uint16_t = (*p).top_;
    let out: *mut uint16_t = (*p).end_;
    let mut sum: uint16_t = 0 as libc::c_int as uint16_t;
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < w {
        let mut new_value: uint16_t = 0;
        sum = (sum as libc::c_int + *src.offset(x as isize) as libc::c_int) as uint16_t;
        new_value = (*top.offset(x as isize) as libc::c_int + sum as libc::c_int)
            as uint16_t;
        *out
            .offset(
                x as isize,
            ) = (new_value as libc::c_int - *cur.offset(x as isize) as libc::c_int)
            as uint16_t;
        *cur.offset(x as isize) = new_value;
        x += 1;
        x;
    }
    (*p).top_ = (*p).cur_;
    (*p).cur_ = ((*p).cur_).offset(w as isize);
    if (*p).cur_ == (*p).end_ {
        (*p).cur_ = (*p).start_;
    }
    if (*p).row_ >= 0 as libc::c_int && (*p).row_ < (*p).height_ - 1 as libc::c_int {
        (*p).src_ = ((*p).src_).offset((*p).stride_ as isize);
    }
}
unsafe extern "C" fn HFilter(p: *mut SmoothParams) {
    let in_0: *const uint16_t = (*p).end_;
    let out: *mut uint16_t = (*p).average_;
    let scale: uint32_t = (*p).scale_ as uint32_t;
    let w: libc::c_int = (*p).width_;
    let r: libc::c_int = (*p).radius_;
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x <= r {
        let delta: uint16_t = (*in_0.offset((x + r - 1 as libc::c_int) as isize)
            as libc::c_int + *in_0.offset((r - x) as isize) as libc::c_int) as uint16_t;
        *out
            .offset(
                x as isize,
            ) = ((delta as libc::c_uint).wrapping_mul(scale) >> 16 as libc::c_int)
            as uint16_t;
        x += 1;
        x;
    }
    while x < w - r {
        let delta_0: uint16_t = (*in_0.offset((x + r) as isize) as libc::c_int
            - *in_0.offset((x - r - 1 as libc::c_int) as isize) as libc::c_int)
            as uint16_t;
        *out
            .offset(
                x as isize,
            ) = ((delta_0 as libc::c_uint).wrapping_mul(scale) >> 16 as libc::c_int)
            as uint16_t;
        x += 1;
        x;
    }
    while x < w {
        let delta_1: uint16_t = (2 as libc::c_int
            * *in_0.offset((w - 1 as libc::c_int) as isize) as libc::c_int
            - *in_0.offset((2 as libc::c_int * w - 2 as libc::c_int - r - x) as isize)
                as libc::c_int
            - *in_0.offset((x - r - 1 as libc::c_int) as isize) as libc::c_int)
            as uint16_t;
        *out
            .offset(
                x as isize,
            ) = ((delta_1 as libc::c_uint).wrapping_mul(scale) >> 16 as libc::c_int)
            as uint16_t;
        x += 1;
        x;
    }
}
unsafe extern "C" fn ApplyFilter(p: *mut SmoothParams) {
    let average: *const uint16_t = (*p).average_;
    let w: libc::c_int = (*p).width_;
    let correction: *const int16_t = (*p).correction_;
    let dst: *mut uint8_t = (*p).dst_;
    let mut x: libc::c_int = 0;
    x = 0 as libc::c_int;
    while x < w {
        let v: libc::c_int = *dst.offset(x as isize) as libc::c_int;
        if v < (*p).max_ && v > (*p).min_ {
            let c: libc::c_int = (v << 0 as libc::c_int)
                + *correction
                    .offset(
                        (*average.offset(x as isize) as libc::c_int
                            - (v << 2 as libc::c_int)) as isize,
                    ) as libc::c_int;
            *dst.offset(x as isize) = clip_8b(c);
        }
        x += 1;
        x;
    }
    (*p).dst_ = ((*p).dst_).offset((*p).stride_ as isize);
}
unsafe extern "C" fn InitCorrectionLUT(lut: *mut int16_t, mut min_dist: libc::c_int) {
    let threshold1: libc::c_int = min_dist << 2 as libc::c_int;
    let threshold2: libc::c_int = 3 as libc::c_int * threshold1 >> 2 as libc::c_int;
    let max_threshold: libc::c_int = threshold2 << 0 as libc::c_int;
    let delta: libc::c_int = threshold1 - threshold2;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i
        <= ((1 as libc::c_int) << 8 as libc::c_int + 2 as libc::c_int) - 1 as libc::c_int
    {
        let mut c: libc::c_int = if i <= threshold2 {
            i << 0 as libc::c_int
        } else if i < threshold1 {
            max_threshold * (threshold1 - i) / delta
        } else {
            0 as libc::c_int
        };
        c >>= 2 as libc::c_int;
        *lut.offset(i as isize) = c as int16_t;
        *lut.offset(-i as isize) = -c as int16_t;
        i += 1;
        i;
    }
    *lut.offset(0 as libc::c_int as isize) = 0 as libc::c_int as int16_t;
}
unsafe extern "C" fn CountLevels(p: *mut SmoothParams) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut last_level: libc::c_int = 0;
    let mut used_levels: [uint8_t; 256] = [
        0 as libc::c_int as uint8_t,
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
    let mut data: *const uint8_t = (*p).src_;
    (*p).min_ = 255 as libc::c_int;
    (*p).max_ = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < (*p).height_ {
        i = 0 as libc::c_int;
        while i < (*p).width_ {
            let v: libc::c_int = *data.offset(i as isize) as libc::c_int;
            if v < (*p).min_ {
                (*p).min_ = v;
            }
            if v > (*p).max_ {
                (*p).max_ = v;
            }
            used_levels[v as usize] = 1 as libc::c_int as uint8_t;
            i += 1;
            i;
        }
        data = data.offset((*p).stride_ as isize);
        j += 1;
        j;
    }
    (*p).min_level_dist_ = (*p).max_ - (*p).min_;
    last_level = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if used_levels[i as usize] != 0 {
            (*p).num_levels_ += 1;
            (*p).num_levels_;
            if last_level >= 0 as libc::c_int {
                let level_dist: libc::c_int = i - last_level;
                if level_dist < (*p).min_level_dist_ {
                    (*p).min_level_dist_ = level_dist;
                }
            }
            last_level = i;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn InitParams(
    data: *mut uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut radius: libc::c_int,
    p: *mut SmoothParams,
) -> libc::c_int {
    let R: libc::c_int = 2 as libc::c_int * radius + 1 as libc::c_int;
    let size_scratch_m: size_t = (((R + 1 as libc::c_int) * width) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong);
    let size_m: size_t = (width as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong);
    let size_lut: size_t = ((1 as libc::c_int
        + 2 as libc::c_int
            * (((1 as libc::c_int) << 8 as libc::c_int + 2 as libc::c_int)
                - 1 as libc::c_int)) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong);
    let total_size: size_t = size_scratch_m.wrapping_add(size_m).wrapping_add(size_lut);
    let mut mem: *mut uint8_t = WebPSafeMalloc(1 as libc::c_uint as uint64_t, total_size)
        as *mut uint8_t;
    if mem.is_null() {
        return 0 as libc::c_int;
    }
    (*p).mem_ = mem as *mut libc::c_void;
    (*p).start_ = mem as *mut uint16_t;
    (*p).cur_ = (*p).start_;
    (*p).end_ = ((*p).start_).offset((R * width) as isize);
    (*p).top_ = ((*p).end_).offset(-(width as isize));
    memset(
        (*p).top_ as *mut libc::c_void,
        0 as libc::c_int,
        (width as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    );
    mem = mem.offset(size_scratch_m as isize);
    (*p).average_ = mem as *mut uint16_t;
    mem = mem.offset(size_m as isize);
    (*p).width_ = width;
    (*p).height_ = height;
    (*p).stride_ = stride;
    (*p).src_ = data;
    (*p).dst_ = data;
    (*p).radius_ = radius;
    (*p).scale_ = ((1 as libc::c_int) << 16 as libc::c_int + 2 as libc::c_int) / (R * R);
    (*p).row_ = -radius;
    CountLevels(p);
    (*p)
        .correction_ = (mem as *mut int16_t)
        .offset(
            (((1 as libc::c_int) << 8 as libc::c_int + 2 as libc::c_int)
                - 1 as libc::c_int) as isize,
        );
    InitCorrectionLUT((*p).correction_, (*p).min_level_dist_);
    return 1 as libc::c_int;
}
unsafe extern "C" fn CleanupParams(p: *mut SmoothParams) {
    WebPSafeFree((*p).mem_);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDequantizeLevels(
    data: *mut uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut stride: libc::c_int,
    mut strength: libc::c_int,
) -> libc::c_int {
    let mut radius: libc::c_int = 4 as libc::c_int * strength / 100 as libc::c_int;
    if strength < 0 as libc::c_int || strength > 100 as libc::c_int {
        return 0 as libc::c_int;
    }
    if data.is_null() || width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if 2 as libc::c_int * radius + 1 as libc::c_int > width {
        radius = width - 1 as libc::c_int >> 1 as libc::c_int;
    }
    if 2 as libc::c_int * radius + 1 as libc::c_int > height {
        radius = height - 1 as libc::c_int >> 1 as libc::c_int;
    }
    if radius > 0 as libc::c_int {
        let mut p: SmoothParams = SmoothParams {
            width_: 0,
            height_: 0,
            stride_: 0,
            row_: 0,
            src_: 0 as *mut uint8_t,
            dst_: 0 as *mut uint8_t,
            radius_: 0,
            scale_: 0,
            mem_: 0 as *mut libc::c_void,
            start_: 0 as *mut uint16_t,
            cur_: 0 as *mut uint16_t,
            end_: 0 as *mut uint16_t,
            top_: 0 as *mut uint16_t,
            average_: 0 as *mut uint16_t,
            num_levels_: 0,
            min_: 0,
            max_: 0,
            min_level_dist_: 0,
            correction_: 0 as *mut int16_t,
        };
        memset(
            &mut p as *mut SmoothParams as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SmoothParams>() as libc::c_ulong,
        );
        if InitParams(data, width, height, stride, radius, &mut p) == 0 {
            return 0 as libc::c_int;
        }
        if p.num_levels_ > 2 as libc::c_int {
            while p.row_ < p.height_ {
                VFilter(&mut p);
                if p.row_ >= p.radius_ {
                    HFilter(&mut p);
                    ApplyFilter(&mut p);
                }
                p.row_ += 1;
                p.row_;
            }
        }
        CleanupParams(&mut p);
    }
    return 1 as libc::c_int;
}
