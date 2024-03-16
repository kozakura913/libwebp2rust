use ::libc;
extern "C" {
    fn expf(_: libc::c_float) -> libc::c_float;
    fn logf(_: libc::c_float) -> libc::c_float;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
}
pub type SharpYuvTransferFunctionType = libc::c_uint;
pub const kSharpYuvTransferFunctionNum: SharpYuvTransferFunctionType = 19;
pub const kSharpYuvTransferFunctionHlg: SharpYuvTransferFunctionType = 18;
pub const kSharpYuvTransferFunctionSmpte428: SharpYuvTransferFunctionType = 17;
pub const kSharpYuvTransferFunctionSmpte2084: SharpYuvTransferFunctionType = 16;
pub const kSharpYuvTransferFunctionBt2020_12Bit: SharpYuvTransferFunctionType = 15;
pub const kSharpYuvTransferFunctionBt2020_10Bit: SharpYuvTransferFunctionType = 14;
pub const kSharpYuvTransferFunctionSrgb: SharpYuvTransferFunctionType = 13;
pub const kSharpYuvTransferFunctionBt1361: SharpYuvTransferFunctionType = 12;
pub const kSharpYuvTransferFunctionIec61966: SharpYuvTransferFunctionType = 11;
pub const kSharpYuvTransferFunctionLog100_Sqrt10: SharpYuvTransferFunctionType = 10;
pub const kSharpYuvTransferFunctionLog100: SharpYuvTransferFunctionType = 9;
pub const kSharpYuvTransferFunctionLinear: SharpYuvTransferFunctionType = 8;
pub const kSharpYuvTransferFunctionSmpte240: SharpYuvTransferFunctionType = 7;
pub const kSharpYuvTransferFunctionBt601: SharpYuvTransferFunctionType = 6;
pub const kSharpYuvTransferFunctionBt470Bg: SharpYuvTransferFunctionType = 5;
pub const kSharpYuvTransferFunctionBt470M: SharpYuvTransferFunctionType = 4;
pub const kSharpYuvTransferFunctionBt709: SharpYuvTransferFunctionType = 1;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
static mut kGammaToLinearTabS: [uint32_t; 1026] = [0; 1026];
static mut kLinearToGammaTabS: [uint32_t; 514] = [0; 514];
static mut kGammaF: libc::c_double = 1.0f64 / 0.45f64;
static mut kGammaTablesSOk: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn SharpYuvInitGammaTables() {
    if kGammaTablesSOk == 0 {
        let mut v: libc::c_int = 0;
        let a: libc::c_double = 0.09929682680944f64;
        let thresh: libc::c_double = 0.018053968510807f64;
        let final_scale: libc::c_double = ((1 as libc::c_int) << 16 as libc::c_int)
            as libc::c_double;
        let norm: libc::c_double = 1.0f64
            / ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_double;
        let a_rec: libc::c_double = 1.0f64 / (1.0f64 + a);
        v = 0 as libc::c_int;
        while v <= (1 as libc::c_int) << 10 as libc::c_int {
            let g: libc::c_double = norm * v as libc::c_double;
            let mut value: libc::c_double = 0.;
            if g <= thresh * 4.5f64 {
                value = g / 4.5f64;
            } else {
                value = pow(a_rec * (g + a), kGammaF);
            }
            kGammaToLinearTabS[v as usize] = (value * final_scale + 0.5f64) as uint32_t;
            v += 1;
            v;
        }
        kGammaToLinearTabS[(((1 as libc::c_int) << 10 as libc::c_int) + 1 as libc::c_int)
            as usize] = kGammaToLinearTabS[((1 as libc::c_int) << 10 as libc::c_int)
            as usize];
        let scale: libc::c_double = 1.0f64
            / ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_double;
        v = 0 as libc::c_int;
        while v <= (1 as libc::c_int) << 9 as libc::c_int {
            let g_0: libc::c_double = scale * v as libc::c_double;
            let mut value_0: libc::c_double = 0.;
            if g_0 <= thresh {
                value_0 = 4.5f64 * g_0;
            } else {
                value_0 = (1.0f64 + a) * pow(g_0, 1.0f64 / kGammaF) - a;
            }
            kLinearToGammaTabS[v
                as usize] = (final_scale * value_0 + 0.5f64) as uint32_t;
            v += 1;
            v;
        }
        kLinearToGammaTabS[(((1 as libc::c_int) << 9 as libc::c_int) + 1 as libc::c_int)
            as usize] = kLinearToGammaTabS[((1 as libc::c_int) << 9 as libc::c_int)
            as usize];
        ::core::ptr::write_volatile(
            &mut kGammaTablesSOk as *mut libc::c_int,
            1 as libc::c_int,
        );
    }
}
#[inline]
unsafe extern "C" fn Shift(mut v: libc::c_int, mut shift: libc::c_int) -> libc::c_int {
    return if shift >= 0 as libc::c_int { v << shift } else { v >> -shift };
}
#[inline]
unsafe extern "C" fn FixedPointInterpolation(
    mut v: libc::c_int,
    mut tab: *mut uint32_t,
    mut tab_pos_shift_right: libc::c_int,
    mut tab_value_shift: libc::c_int,
) -> uint32_t {
    let tab_pos: uint32_t = Shift(v, -tab_pos_shift_right) as uint32_t;
    let x: uint32_t = (v as libc::c_uint).wrapping_sub(tab_pos << tab_pos_shift_right);
    let v0: uint32_t = Shift(
        *tab.offset(tab_pos.wrapping_add(0 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int,
        tab_value_shift,
    ) as uint32_t;
    let v1: uint32_t = Shift(
        *tab.offset(tab_pos.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int,
        tab_value_shift,
    ) as uint32_t;
    let v2: uint32_t = v1.wrapping_sub(v0).wrapping_mul(x);
    let half: libc::c_int = if tab_pos_shift_right > 0 as libc::c_int {
        (1 as libc::c_int) << tab_pos_shift_right - 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let result: uint32_t = v0
        .wrapping_add(v2.wrapping_add(half as libc::c_uint) >> tab_pos_shift_right);
    return result;
}
unsafe extern "C" fn ToLinearSrgb(
    mut v: uint16_t,
    mut bit_depth: libc::c_int,
) -> uint32_t {
    let shift: libc::c_int = 10 as libc::c_int - bit_depth;
    if shift > 0 as libc::c_int {
        return kGammaToLinearTabS[((v as libc::c_int) << shift) as usize];
    }
    return FixedPointInterpolation(
        v as libc::c_int,
        kGammaToLinearTabS.as_mut_ptr(),
        -shift,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn FromLinearSrgb(
    mut value: uint32_t,
    mut bit_depth: libc::c_int,
) -> uint16_t {
    return FixedPointInterpolation(
        value as libc::c_int,
        kLinearToGammaTabS.as_mut_ptr(),
        16 as libc::c_int - 9 as libc::c_int,
        bit_depth - 16 as libc::c_int,
    ) as uint16_t;
}
#[inline]
unsafe extern "C" fn Roundf(mut x: libc::c_float) -> libc::c_float {
    if x < 0 as libc::c_int as libc::c_float {
        return ceil((x - 0.5f32) as libc::c_double) as libc::c_float
    } else {
        return floor((x + 0.5f32) as libc::c_double) as libc::c_float
    };
}
#[inline]
unsafe extern "C" fn Powf(
    mut base: libc::c_float,
    mut exp: libc::c_float,
) -> libc::c_float {
    return pow(base as libc::c_double, exp as libc::c_double) as libc::c_float;
}
#[inline]
unsafe extern "C" fn Log10f(mut x: libc::c_float) -> libc::c_float {
    return log10(x as libc::c_double) as libc::c_float;
}
unsafe extern "C" fn ToLinear709(mut gamma: libc::c_float) -> libc::c_float {
    if gamma < 0.0f32 {
        return 0.0f32
    } else if gamma < 4.5f32 * 0.018053968510807f32 {
        return gamma / 4.5f32
    } else if gamma < 1.0f32 {
        return Powf(
            (gamma + 0.09929682680944f32) / 1.09929682680944f32,
            1.0f32 / 0.45f32,
        )
    }
    return 1.0f32;
}
unsafe extern "C" fn FromLinear709(mut linear: libc::c_float) -> libc::c_float {
    if linear < 0.0f32 {
        return 0.0f32
    } else if linear < 0.018053968510807f32 {
        return linear * 4.5f32
    } else if linear < 1.0f32 {
        return 1.09929682680944f32 * Powf(linear, 0.45f32) - 0.09929682680944f32
    }
    return 1.0f32;
}
unsafe extern "C" fn ToLinear470M(mut gamma: libc::c_float) -> libc::c_float {
    return Powf(
        if gamma < 0.0f32 { 0.0f32 } else if 1.0f32 < gamma { 1.0f32 } else { gamma },
        2.2f32,
    );
}
unsafe extern "C" fn FromLinear470M(mut linear: libc::c_float) -> libc::c_float {
    return Powf(
        if linear < 0.0f32 { 0.0f32 } else if 1.0f32 < linear { 1.0f32 } else { linear },
        1.0f32 / 2.2f32,
    );
}
unsafe extern "C" fn ToLinear470Bg(mut gamma: libc::c_float) -> libc::c_float {
    return Powf(
        if gamma < 0.0f32 { 0.0f32 } else if 1.0f32 < gamma { 1.0f32 } else { gamma },
        2.8f32,
    );
}
unsafe extern "C" fn FromLinear470Bg(mut linear: libc::c_float) -> libc::c_float {
    return Powf(
        if linear < 0.0f32 { 0.0f32 } else if 1.0f32 < linear { 1.0f32 } else { linear },
        1.0f32 / 2.8f32,
    );
}
unsafe extern "C" fn ToLinearSmpte240(mut gamma: libc::c_float) -> libc::c_float {
    if gamma < 0.0f32 {
        return 0.0f32
    } else if gamma < 4.0f32 * 0.022821585529445f32 {
        return gamma / 4.0f32
    } else if gamma < 1.0f32 {
        return Powf(
            (gamma + 0.111572195921731f32) / 1.111572195921731f32,
            1.0f32 / 0.45f32,
        )
    }
    return 1.0f32;
}
unsafe extern "C" fn FromLinearSmpte240(mut linear: libc::c_float) -> libc::c_float {
    if linear < 0.0f32 {
        return 0.0f32
    } else if linear < 0.022821585529445f32 {
        return linear * 4.0f32
    } else if linear < 1.0f32 {
        return 1.111572195921731f32 * Powf(linear, 0.45f32) - 0.111572195921731f32
    }
    return 1.0f32;
}
unsafe extern "C" fn ToLinearLog100(mut gamma: libc::c_float) -> libc::c_float {
    let mid_interval: libc::c_float = 0.01f32 / 2.0f32;
    return if gamma <= 0.0f32 {
        mid_interval
    } else {
        Powf(10.0f32, 2.0f32 * ((if gamma < 1.0f32 { gamma } else { 1.0f32 }) - 1.0f32))
    };
}
unsafe extern "C" fn FromLinearLog100(mut linear: libc::c_float) -> libc::c_float {
    return if linear < 0.01f32 {
        0.0f32
    } else {
        1.0f32 + Log10f((if linear < 1.0f32 { linear } else { 1.0f32 })) / 2.0f32
    };
}
unsafe extern "C" fn ToLinearLog100Sqrt10(mut gamma: libc::c_float) -> libc::c_float {
    let mid_interval: libc::c_float = 0.00316227766f32 / 2.0f32;
    return if gamma <= 0.0f32 {
        mid_interval
    } else {
        Powf(10.0f32, 2.5f32 * ((if gamma < 1.0f32 { gamma } else { 1.0f32 }) - 1.0f32))
    };
}
unsafe extern "C" fn FromLinearLog100Sqrt10(mut linear: libc::c_float) -> libc::c_float {
    return if linear < 0.00316227766f32 {
        0.0f32
    } else {
        1.0f32 + Log10f((if linear < 1.0f32 { linear } else { 1.0f32 })) / 2.5f32
    };
}
unsafe extern "C" fn ToLinearIec61966(mut gamma: libc::c_float) -> libc::c_float {
    if gamma <= -4.5f32 * 0.018053968510807f32 {
        return Powf(
            (-gamma + 0.09929682680944f32) / -1.09929682680944f32,
            1.0f32 / 0.45f32,
        )
    } else if gamma < 4.5f32 * 0.018053968510807f32 {
        return gamma / 4.5f32
    }
    return Powf((gamma + 0.09929682680944f32) / 1.09929682680944f32, 1.0f32 / 0.45f32);
}
unsafe extern "C" fn FromLinearIec61966(mut linear: libc::c_float) -> libc::c_float {
    if linear <= -0.018053968510807f32 {
        return -1.09929682680944f32 * Powf(-linear, 0.45f32) + 0.09929682680944f32
    } else if linear < 0.018053968510807f32 {
        return linear * 4.5f32
    }
    return 1.09929682680944f32 * Powf(linear, 0.45f32) - 0.09929682680944f32;
}
unsafe extern "C" fn ToLinearBt1361(mut gamma: libc::c_float) -> libc::c_float {
    if gamma < -0.25f32 {
        return -0.25f32
    } else if gamma < 0.0f32 {
        return Powf(
            (gamma - 0.02482420670236f32) / -0.27482420670236f32,
            1.0f32 / 0.45f32,
        ) / -4.0f32
    } else if gamma < 4.5f32 * 0.018053968510807f32 {
        return gamma / 4.5f32
    } else if gamma < 1.0f32 {
        return Powf(
            (gamma + 0.09929682680944f32) / 1.09929682680944f32,
            1.0f32 / 0.45f32,
        )
    }
    return 1.0f32;
}
unsafe extern "C" fn FromLinearBt1361(mut linear: libc::c_float) -> libc::c_float {
    if linear < -0.25f32 {
        return -0.25f32
    } else if linear < 0.0f32 {
        return -0.27482420670236f32 * Powf(-4.0f32 * linear, 0.45f32)
            + 0.02482420670236f32
    } else if linear < 0.018053968510807f32 {
        return linear * 4.5f32
    } else if linear < 1.0f32 {
        return 1.09929682680944f32 * Powf(linear, 0.45f32) - 0.09929682680944f32
    }
    return 1.0f32;
}
unsafe extern "C" fn ToLinearPq(mut gamma: libc::c_float) -> libc::c_float {
    if gamma > 0.0f32 {
        let pow_gamma: libc::c_float = Powf(gamma, 32.0f32 / 2523.0f32);
        let num: libc::c_float = if pow_gamma - 107.0f32 / 128.0f32 > 0.0f32 {
            pow_gamma - 107.0f32 / 128.0f32
        } else {
            0.0f32
        };
        let den: libc::c_float = if 2413.0f32 / 128.0f32
            - 2392.0f32 / 128.0f32 * pow_gamma > 1.17549435e-38f32
        {
            2413.0f32 / 128.0f32 - 2392.0f32 / 128.0f32 * pow_gamma
        } else {
            1.17549435e-38f32
        };
        return Powf(num / den, 4096.0f32 / 653.0f32);
    }
    return 0.0f32;
}
unsafe extern "C" fn FromLinearPq(mut linear: libc::c_float) -> libc::c_float {
    if linear > 0.0f32 {
        let pow_linear: libc::c_float = Powf(linear, 653.0f32 / 4096.0f32);
        let num: libc::c_float = 107.0f32 / 128.0f32 + 2413.0f32 / 128.0f32 * pow_linear;
        let den: libc::c_float = 1.0f32 + 2392.0f32 / 128.0f32 * pow_linear;
        return Powf(num / den, 2523.0f32 / 32.0f32);
    }
    return 0.0f32;
}
unsafe extern "C" fn ToLinearSmpte428(mut gamma: libc::c_float) -> libc::c_float {
    return Powf((if gamma > 0.0f32 { gamma } else { 0.0f32 }), 2.6f32)
        / 0.91655527974030934f32;
}
unsafe extern "C" fn FromLinearSmpte428(mut linear: libc::c_float) -> libc::c_float {
    return Powf(
        0.91655527974030934f32 * (if linear > 0.0f32 { linear } else { 0.0f32 }),
        1.0f32 / 2.6f32,
    );
}
unsafe extern "C" fn ToLinearHlg(mut gamma: libc::c_float) -> libc::c_float {
    if gamma < 0.0f32 {
        return 0.0f32
    } else if gamma <= 0.5f32 {
        return Powf(gamma * gamma * (1.0f32 / 3.0f32), 1.2f32)
    }
    return Powf(
        (expf((gamma - 0.55991073f32) / 0.17883277f32) + 0.28466892f32) / 12.0f32,
        1.2f32,
    );
}
unsafe extern "C" fn FromLinearHlg(mut linear: libc::c_float) -> libc::c_float {
    linear = Powf(linear, 1.0f32 / 1.2f32);
    if linear < 0.0f32 {
        return 0.0f32
    } else if linear <= 1.0f32 / 12.0f32 {
        return sqrtf(3.0f32 * linear)
    }
    return 0.17883277f32 * logf(12.0f32 * linear - 0.28466892f32) + 0.55991073f32;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvGammaToLinear(
    mut v: uint16_t,
    mut bit_depth: libc::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) -> uint32_t {
    let mut v_float: libc::c_float = 0.;
    let mut linear: libc::c_float = 0.;
    if transfer_type as libc::c_uint
        == kSharpYuvTransferFunctionSrgb as libc::c_int as libc::c_uint
    {
        return ToLinearSrgb(v, bit_depth);
    }
    v_float = v as libc::c_float
        / (((1 as libc::c_int) << bit_depth) - 1 as libc::c_int) as libc::c_float;
    match transfer_type as libc::c_uint {
        1 | 6 | 14 | 15 => {
            linear = ToLinear709(v_float);
        }
        4 => {
            linear = ToLinear470M(v_float);
        }
        5 => {
            linear = ToLinear470Bg(v_float);
        }
        7 => {
            linear = ToLinearSmpte240(v_float);
        }
        8 => return v as uint32_t,
        9 => {
            linear = ToLinearLog100(v_float);
        }
        10 => {
            linear = ToLinearLog100Sqrt10(v_float);
        }
        11 => {
            linear = ToLinearIec61966(v_float);
        }
        12 => {
            linear = ToLinearBt1361(v_float);
        }
        16 => {
            linear = ToLinearPq(v_float);
        }
        17 => {
            linear = ToLinearSmpte428(v_float);
        }
        18 => {
            linear = ToLinearHlg(v_float);
        }
        _ => {
            linear = 0 as libc::c_int as libc::c_float;
        }
    }
    return Roundf(
        linear
            * (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                as libc::c_float,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvLinearToGamma(
    mut v: uint32_t,
    mut bit_depth: libc::c_int,
    mut transfer_type: SharpYuvTransferFunctionType,
) -> uint16_t {
    let mut v_float: libc::c_float = 0.;
    let mut linear: libc::c_float = 0.;
    if transfer_type as libc::c_uint
        == kSharpYuvTransferFunctionSrgb as libc::c_int as libc::c_uint
    {
        return FromLinearSrgb(v, bit_depth);
    }
    v_float = v as libc::c_float
        / (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
            as libc::c_float;
    match transfer_type as libc::c_uint {
        1 | 6 | 14 | 15 => {
            linear = FromLinear709(v_float);
        }
        4 => {
            linear = FromLinear470M(v_float);
        }
        5 => {
            linear = FromLinear470Bg(v_float);
        }
        7 => {
            linear = FromLinearSmpte240(v_float);
        }
        8 => return v as uint16_t,
        9 => {
            linear = FromLinearLog100(v_float);
        }
        10 => {
            linear = FromLinearLog100Sqrt10(v_float);
        }
        11 => {
            linear = FromLinearIec61966(v_float);
        }
        12 => {
            linear = FromLinearBt1361(v_float);
        }
        16 => {
            linear = FromLinearPq(v_float);
        }
        17 => {
            linear = FromLinearSmpte428(v_float);
        }
        18 => {
            linear = FromLinearHlg(v_float);
        }
        _ => {
            linear = 0 as libc::c_int as libc::c_float;
        }
    }
    return Roundf(
        linear * (((1 as libc::c_int) << bit_depth) - 1 as libc::c_int) as libc::c_float,
    ) as uint16_t;
}
