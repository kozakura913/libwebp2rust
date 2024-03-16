use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut SharpYuvGetCPUInfo: VP8CPUInfo;
}
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type CPUFeature = libc::c_uint;
pub const kMSA: CPUFeature = 9;
pub const kMIPSdspR2: CPUFeature = 8;
pub const kMIPS32: CPUFeature = 7;
pub const kNEON: CPUFeature = 6;
pub const kAVX2: CPUFeature = 5;
pub const kAVX: CPUFeature = 4;
pub const kSSE4_1: CPUFeature = 3;
pub const kSlowSSSE3: CPUFeature = 2;
pub const kSSE3: CPUFeature = 1;
pub const kSSE2: CPUFeature = 0;
pub type VP8CPUInfo = Option::<unsafe extern "C" fn(CPUFeature) -> libc::c_int>;
unsafe extern "C" fn clip(mut v: libc::c_int, mut max: libc::c_int) -> uint16_t {
    return (if v < 0 as libc::c_int {
        0 as libc::c_int
    } else if v > max {
        max
    } else {
        v as uint16_t as libc::c_int
    }) as uint16_t;
}
unsafe extern "C" fn SharpYuvUpdateY_C(
    mut ref_0: *const uint16_t,
    mut src: *const uint16_t,
    mut dst: *mut uint16_t,
    mut len: libc::c_int,
    mut bit_depth: libc::c_int,
) -> uint64_t {
    let mut diff: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_int = 0;
    let max_y: libc::c_int = ((1 as libc::c_int) << bit_depth) - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        let diff_y: libc::c_int = *ref_0.offset(i as isize) as libc::c_int
            - *src.offset(i as isize) as libc::c_int;
        let new_y: libc::c_int = *dst.offset(i as isize) as libc::c_int + diff_y;
        *dst.offset(i as isize) = clip(new_y, max_y);
        diff = (diff as libc::c_ulong).wrapping_add(abs(diff_y) as uint64_t) as uint64_t
            as uint64_t;
        i += 1;
        i;
    }
    return diff;
}
unsafe extern "C" fn SharpYuvUpdateRGB_C(
    mut ref_0: *const int16_t,
    mut src: *const int16_t,
    mut dst: *mut int16_t,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        let diff_uv: libc::c_int = *ref_0.offset(i as isize) as libc::c_int
            - *src.offset(i as isize) as libc::c_int;
        let ref mut fresh0 = *dst.offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_int + diff_uv) as int16_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn SharpYuvFilterRow_C(
    mut A: *const int16_t,
    mut B: *const int16_t,
    mut len: libc::c_int,
    mut best_y: *const uint16_t,
    mut out: *mut uint16_t,
    mut bit_depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let max_y: libc::c_int = ((1 as libc::c_int) << bit_depth) - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        let v0: libc::c_int = *A.offset(0 as libc::c_int as isize) as libc::c_int
            * 9 as libc::c_int
            + *A.offset(1 as libc::c_int as isize) as libc::c_int * 3 as libc::c_int
            + *B.offset(0 as libc::c_int as isize) as libc::c_int * 3 as libc::c_int
            + *B.offset(1 as libc::c_int as isize) as libc::c_int + 8 as libc::c_int
            >> 4 as libc::c_int;
        let v1: libc::c_int = *A.offset(1 as libc::c_int as isize) as libc::c_int
            * 9 as libc::c_int
            + *A.offset(0 as libc::c_int as isize) as libc::c_int * 3 as libc::c_int
            + *B.offset(1 as libc::c_int as isize) as libc::c_int * 3 as libc::c_int
            + *B.offset(0 as libc::c_int as isize) as libc::c_int + 8 as libc::c_int
            >> 4 as libc::c_int;
        *out
            .offset(
                (2 as libc::c_int * i + 0 as libc::c_int) as isize,
            ) = clip(
            *best_y.offset((2 as libc::c_int * i + 0 as libc::c_int) as isize)
                as libc::c_int + v0,
            max_y,
        );
        *out
            .offset(
                (2 as libc::c_int * i + 1 as libc::c_int) as isize,
            ) = clip(
            *best_y.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                as libc::c_int + v1,
            max_y,
        );
        i += 1;
        i;
        A = A.offset(1);
        A;
        B = B.offset(1);
        B;
    }
}
#[no_mangle]
pub static mut SharpYuvUpdateY: Option::<
    unsafe extern "C" fn(
        *const uint16_t,
        *const uint16_t,
        *mut uint16_t,
        libc::c_int,
        libc::c_int,
    ) -> uint64_t,
> = None;
#[no_mangle]
pub static mut SharpYuvUpdateRGB: Option::<
    unsafe extern "C" fn(*const int16_t, *const int16_t, *mut int16_t, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut SharpYuvFilterRow: Option::<
    unsafe extern "C" fn(
        *const int16_t,
        *const int16_t,
        libc::c_int,
        *const uint16_t,
        *mut uint16_t,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn SharpYuvInitDsp() {
    SharpYuvUpdateY = Some(
        SharpYuvUpdateY_C
            as unsafe extern "C" fn(
                *const uint16_t,
                *const uint16_t,
                *mut uint16_t,
                libc::c_int,
                libc::c_int,
            ) -> uint64_t,
    );
    SharpYuvUpdateRGB = Some(
        SharpYuvUpdateRGB_C
            as unsafe extern "C" fn(
                *const int16_t,
                *const int16_t,
                *mut int16_t,
                libc::c_int,
            ) -> (),
    );
    SharpYuvFilterRow = Some(
        SharpYuvFilterRow_C
            as unsafe extern "C" fn(
                *const int16_t,
                *const int16_t,
                libc::c_int,
                *const uint16_t,
                *mut uint16_t,
                libc::c_int,
            ) -> (),
    );
    SharpYuvGetCPUInfo.is_some();
}
