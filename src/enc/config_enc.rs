use ::libc;

use crate::src::utils::types::{WebPConfig, WEBP_HINT_DEFAULT, WEBP_HINT_LAST};
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;

pub type WebPPreset = libc::c_uint;
pub const WEBP_PRESET_TEXT: WebPPreset = 5;
pub const WEBP_PRESET_ICON: WebPPreset = 4;
pub const WEBP_PRESET_DRAWING: WebPPreset = 3;
pub const WEBP_PRESET_PHOTO: WebPPreset = 2;
pub const WEBP_PRESET_PICTURE: WebPPreset = 1;
pub const WEBP_PRESET_DEFAULT: WebPPreset = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub method_: uint8_t,
    pub quality_: uint8_t,
}
#[no_mangle]
pub unsafe extern "C" fn WebPConfigInitInternal(
    mut config: *mut WebPConfig,
    mut preset: WebPPreset,
    mut quality: libc::c_float,
    mut version: libc::c_int,
) -> libc::c_int {
    if version >> 8 as libc::c_int != 0x20f as libc::c_int >> 8 as libc::c_int {
        return 0 as libc::c_int;
    }
    if config.is_null() {
        return 0 as libc::c_int;
    }
    (*config).quality = quality;
    (*config).target_size = 0 as libc::c_int;
    (*config).target_PSNR = 0.0f64 as libc::c_float;
    (*config).method = 4 as libc::c_int;
    (*config).sns_strength = 50 as libc::c_int;
    (*config).filter_strength = 60 as libc::c_int;
    (*config).filter_sharpness = 0 as libc::c_int;
    (*config).filter_type = 1 as libc::c_int;
    (*config).partitions = 0 as libc::c_int;
    (*config).segments = 4 as libc::c_int;
    (*config).pass = 1 as libc::c_int;
    (*config).qmin = 0 as libc::c_int;
    (*config).qmax = 100 as libc::c_int;
    (*config).show_compressed = 0 as libc::c_int;
    (*config).preprocessing = 0 as libc::c_int;
    (*config).autofilter = 0 as libc::c_int;
    (*config).partition_limit = 0 as libc::c_int;
    (*config).alpha_compression = 1 as libc::c_int;
    (*config).alpha_filtering = 1 as libc::c_int;
    (*config).alpha_quality = 100 as libc::c_int;
    (*config).lossless = 0 as libc::c_int;
    (*config).exact = 0 as libc::c_int;
    (*config).image_hint = WEBP_HINT_DEFAULT;
    (*config).emulate_jpeg_size = 0 as libc::c_int;
    (*config).thread_level = 0 as libc::c_int;
    (*config).low_memory = 0 as libc::c_int;
    (*config).near_lossless = 100 as libc::c_int;
    (*config).use_delta_palette = 0 as libc::c_int;
    (*config).use_sharp_yuv = 0 as libc::c_int;
    match preset as libc::c_uint {
        1 => {
            (*config).sns_strength = 80 as libc::c_int;
            (*config).filter_sharpness = 4 as libc::c_int;
            (*config).filter_strength = 35 as libc::c_int;
            (*config).preprocessing &= !(2 as libc::c_int);
        }
        2 => {
            (*config).sns_strength = 80 as libc::c_int;
            (*config).filter_sharpness = 3 as libc::c_int;
            (*config).filter_strength = 30 as libc::c_int;
            (*config).preprocessing |= 2 as libc::c_int;
        }
        3 => {
            (*config).sns_strength = 25 as libc::c_int;
            (*config).filter_sharpness = 6 as libc::c_int;
            (*config).filter_strength = 10 as libc::c_int;
        }
        4 => {
            (*config).sns_strength = 0 as libc::c_int;
            (*config).filter_strength = 0 as libc::c_int;
            (*config).preprocessing &= !(2 as libc::c_int);
        }
        5 => {
            (*config).sns_strength = 0 as libc::c_int;
            (*config).filter_strength = 0 as libc::c_int;
            (*config).preprocessing &= !(2 as libc::c_int);
            (*config).segments = 2 as libc::c_int;
        }
        0 | _ => {}
    }
    return WebPValidateConfig(config);
}
#[no_mangle]
pub unsafe extern "C" fn WebPValidateConfig(
    mut config: *const WebPConfig,
) -> libc::c_int {
    if config.is_null() {
        return 0 as libc::c_int;
    }
    if (*config).quality < 0 as libc::c_int as libc::c_float
        || (*config).quality > 100 as libc::c_int as libc::c_float
    {
        return 0 as libc::c_int;
    }
    if (*config).target_size < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).target_PSNR < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int;
    }
    if (*config).method < 0 as libc::c_int || (*config).method > 6 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).segments < 1 as libc::c_int || (*config).segments > 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).sns_strength < 0 as libc::c_int
        || (*config).sns_strength > 100 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).filter_strength < 0 as libc::c_int
        || (*config).filter_strength > 100 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).filter_sharpness < 0 as libc::c_int
        || (*config).filter_sharpness > 7 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).filter_type < 0 as libc::c_int
        || (*config).filter_type > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).autofilter < 0 as libc::c_int || (*config).autofilter > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).pass < 1 as libc::c_int || (*config).pass > 10 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).qmin < 0 as libc::c_int || (*config).qmax > 100 as libc::c_int
        || (*config).qmin > (*config).qmax
    {
        return 0 as libc::c_int;
    }
    if (*config).show_compressed < 0 as libc::c_int
        || (*config).show_compressed > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).preprocessing < 0 as libc::c_int
        || (*config).preprocessing > 7 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).partitions < 0 as libc::c_int || (*config).partitions > 3 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).partition_limit < 0 as libc::c_int
        || (*config).partition_limit > 100 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).alpha_compression < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).alpha_filtering < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).alpha_quality < 0 as libc::c_int
        || (*config).alpha_quality > 100 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).lossless < 0 as libc::c_int || (*config).lossless > 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).near_lossless < 0 as libc::c_int
        || (*config).near_lossless > 100 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).image_hint as libc::c_uint
        >= WEBP_HINT_LAST as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*config).emulate_jpeg_size < 0 as libc::c_int
        || (*config).emulate_jpeg_size > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).thread_level < 0 as libc::c_int
        || (*config).thread_level > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).low_memory < 0 as libc::c_int || (*config).low_memory > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).exact < 0 as libc::c_int || (*config).exact > 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*config).use_delta_palette < 0 as libc::c_int
        || (*config).use_delta_palette > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*config).use_sharp_yuv < 0 as libc::c_int
        || (*config).use_sharp_yuv > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
static mut kLosslessPresets: [C2RustUnnamed; 10] = [
    {
        let mut init = C2RustUnnamed {
            method_: 0 as libc::c_int as uint8_t,
            quality_: 0 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 1 as libc::c_int as uint8_t,
            quality_: 20 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 2 as libc::c_int as uint8_t,
            quality_: 25 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 3 as libc::c_int as uint8_t,
            quality_: 30 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 3 as libc::c_int as uint8_t,
            quality_: 50 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 4 as libc::c_int as uint8_t,
            quality_: 50 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 4 as libc::c_int as uint8_t,
            quality_: 75 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 4 as libc::c_int as uint8_t,
            quality_: 90 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 5 as libc::c_int as uint8_t,
            quality_: 90 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            method_: 6 as libc::c_int as uint8_t,
            quality_: 100 as libc::c_int as uint8_t,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn WebPConfigLosslessPreset(
    mut config: *mut WebPConfig,
    mut level: libc::c_int,
) -> libc::c_int {
    if config.is_null() || level < 0 as libc::c_int || level > 9 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*config).lossless = 1 as libc::c_int;
    (*config).method = kLosslessPresets[level as usize].method_ as libc::c_int;
    (*config).quality = kLosslessPresets[level as usize].quality_ as libc::c_float;
    return 1 as libc::c_int;
}
