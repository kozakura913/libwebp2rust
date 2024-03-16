use ::libc;
extern "C" {
    fn floor(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [libc::c_int; 4],
    pub rgb_to_u: [libc::c_int; 4],
    pub rgb_to_v: [libc::c_int; 4],
}
pub type SharpYuvRange = libc::c_uint;
pub const kSharpYuvRangeLimited: SharpYuvRange = 1;
pub const kSharpYuvRangeFull: SharpYuvRange = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvColorSpace {
    pub kr: libc::c_float,
    pub kb: libc::c_float,
    pub bit_depth: libc::c_int,
    pub range: SharpYuvRange,
}
pub type SharpYuvMatrixType = libc::c_uint;
pub const kSharpYuvMatrixNum: SharpYuvMatrixType = 5;
pub const kSharpYuvMatrixRec709Full: SharpYuvMatrixType = 4;
pub const kSharpYuvMatrixRec709Limited: SharpYuvMatrixType = 3;
pub const kSharpYuvMatrixRec601Full: SharpYuvMatrixType = 2;
pub const kSharpYuvMatrixRec601Limited: SharpYuvMatrixType = 1;
pub const kSharpYuvMatrixWebp: SharpYuvMatrixType = 0;
unsafe extern "C" fn ToFixed16(mut f: libc::c_float) -> libc::c_int {
    return floor(
        (f * ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_float + 0.5f32)
            as libc::c_double,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvComputeConversionMatrix(
    mut yuv_color_space: *const SharpYuvColorSpace,
    mut matrix: *mut SharpYuvConversionMatrix,
) {
    let kr: libc::c_float = (*yuv_color_space).kr;
    let kb: libc::c_float = (*yuv_color_space).kb;
    let kg: libc::c_float = 1.0f32 - kr - kb;
    let cr: libc::c_float = 0.5f32 / (1.0f32 - kb);
    let cb: libc::c_float = 0.5f32 / (1.0f32 - kr);
    let shift: libc::c_int = (*yuv_color_space).bit_depth - 8 as libc::c_int;
    let denom: libc::c_float = (((1 as libc::c_int) << (*yuv_color_space).bit_depth)
        - 1 as libc::c_int) as libc::c_float;
    let mut scale_y: libc::c_float = 1.0f32;
    let mut add_y: libc::c_float = 0.0f32;
    let mut scale_u: libc::c_float = cr;
    let mut scale_v: libc::c_float = cb;
    let mut add_uv: libc::c_float = ((128 as libc::c_int) << shift) as libc::c_float;
    if (*yuv_color_space).range as libc::c_uint
        == kSharpYuvRangeLimited as libc::c_int as libc::c_uint
    {
        scale_y *= ((219 as libc::c_int) << shift) as libc::c_float / denom;
        scale_u *= ((224 as libc::c_int) << shift) as libc::c_float / denom;
        scale_v *= ((224 as libc::c_int) << shift) as libc::c_float / denom;
        add_y = ((16 as libc::c_int) << shift) as libc::c_float;
    }
    (*matrix).rgb_to_y[0 as libc::c_int as usize] = ToFixed16(kr * scale_y);
    (*matrix).rgb_to_y[1 as libc::c_int as usize] = ToFixed16(kg * scale_y);
    (*matrix).rgb_to_y[2 as libc::c_int as usize] = ToFixed16(kb * scale_y);
    (*matrix).rgb_to_y[3 as libc::c_int as usize] = ToFixed16(add_y);
    (*matrix).rgb_to_u[0 as libc::c_int as usize] = ToFixed16(-kr * scale_u);
    (*matrix).rgb_to_u[1 as libc::c_int as usize] = ToFixed16(-kg * scale_u);
    (*matrix)
        .rgb_to_u[2 as libc::c_int
        as usize] = ToFixed16((1 as libc::c_int as libc::c_float - kb) * scale_u);
    (*matrix).rgb_to_u[3 as libc::c_int as usize] = ToFixed16(add_uv);
    (*matrix)
        .rgb_to_v[0 as libc::c_int
        as usize] = ToFixed16((1 as libc::c_int as libc::c_float - kr) * scale_v);
    (*matrix).rgb_to_v[1 as libc::c_int as usize] = ToFixed16(-kg * scale_v);
    (*matrix).rgb_to_v[2 as libc::c_int as usize] = ToFixed16(-kb * scale_v);
    (*matrix).rgb_to_v[3 as libc::c_int as usize] = ToFixed16(add_uv);
}
static mut kWebpMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            16839 as libc::c_int,
            33059 as libc::c_int,
            6420 as libc::c_int,
            (16 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_u: [
            -(9719 as libc::c_int),
            -(19081 as libc::c_int),
            28800 as libc::c_int,
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_v: [
            28800 as libc::c_int,
            -(24116 as libc::c_int),
            -(4684 as libc::c_int),
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
    };
    init
};
static mut kRec601LimitedMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            16829 as libc::c_int,
            33039 as libc::c_int,
            6416 as libc::c_int,
            (16 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_u: [
            -(9714 as libc::c_int),
            -(19071 as libc::c_int),
            28784 as libc::c_int,
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_v: [
            28784 as libc::c_int,
            -(24103 as libc::c_int),
            -(4681 as libc::c_int),
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
    };
    init
};
static mut kRec601FullMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            19595 as libc::c_int,
            38470 as libc::c_int,
            7471 as libc::c_int,
            0 as libc::c_int,
        ],
        rgb_to_u: [
            -(11058 as libc::c_int),
            -(21710 as libc::c_int),
            32768 as libc::c_int,
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_v: [
            32768 as libc::c_int,
            -(27439 as libc::c_int),
            -(5329 as libc::c_int),
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
    };
    init
};
static mut kRec709LimitedMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            11966 as libc::c_int,
            40254 as libc::c_int,
            4064 as libc::c_int,
            (16 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_u: [
            -(6596 as libc::c_int),
            -(22189 as libc::c_int),
            28784 as libc::c_int,
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_v: [
            28784 as libc::c_int,
            -(26145 as libc::c_int),
            -(2639 as libc::c_int),
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
    };
    init
};
static mut kRec709FullMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            13933 as libc::c_int,
            46871 as libc::c_int,
            4732 as libc::c_int,
            0 as libc::c_int,
        ],
        rgb_to_u: [
            -(7509 as libc::c_int),
            -(25259 as libc::c_int),
            32768 as libc::c_int,
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
        rgb_to_v: [
            32768 as libc::c_int,
            -(29763 as libc::c_int),
            -(3005 as libc::c_int),
            (128 as libc::c_int) << 16 as libc::c_int,
        ],
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn SharpYuvGetConversionMatrix(
    mut matrix_type: SharpYuvMatrixType,
) -> *const SharpYuvConversionMatrix {
    match matrix_type as libc::c_uint {
        0 => return &kWebpMatrix,
        1 => return &kRec601LimitedMatrix,
        2 => return &kRec601FullMatrix,
        3 => return &kRec709LimitedMatrix,
        4 => return &kRec709FullMatrix,
        5 => return 0 as *const SharpYuvConversionMatrix,
        _ => {}
    }
    return 0 as *const SharpYuvConversionMatrix;
}
