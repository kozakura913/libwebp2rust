use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Random {
    pub index1_: libc::c_int,
    pub index2_: libc::c_int,
    pub tab_: [uint32_t; 55],
    pub amp_: libc::c_int,
}
static mut kRandomTable: [uint32_t; 55] = [
    0xde15230 as libc::c_int as uint32_t,
    0x3b31886 as libc::c_int as uint32_t,
    0x775faccb as libc::c_int as uint32_t,
    0x1c88626a as libc::c_int as uint32_t,
    0x68385c55 as libc::c_int as uint32_t,
    0x14b3b828 as libc::c_int as uint32_t,
    0x4a85fef8 as libc::c_int as uint32_t,
    0x49ddb84b as libc::c_int as uint32_t,
    0x64fcf397 as libc::c_int as uint32_t,
    0x5c550289 as libc::c_int as uint32_t,
    0x4a290000 as libc::c_int as uint32_t,
    0xd7ec1da as libc::c_int as uint32_t,
    0x5940b7ab as libc::c_int as uint32_t,
    0x5492577d as libc::c_int as uint32_t,
    0x4e19ca72 as libc::c_int as uint32_t,
    0x38d38c69 as libc::c_int as uint32_t,
    0xc01ee65 as libc::c_int as uint32_t,
    0x32a1755f as libc::c_int as uint32_t,
    0x5437f652 as libc::c_int as uint32_t,
    0x5abb2c32 as libc::c_int as uint32_t,
    0xfaa57b1 as libc::c_int as uint32_t,
    0x73f533e7 as libc::c_int as uint32_t,
    0x685feeda as libc::c_int as uint32_t,
    0x7563cce2 as libc::c_int as uint32_t,
    0x6e990e83 as libc::c_int as uint32_t,
    0x4730a7ed as libc::c_int as uint32_t,
    0x4fc0d9c6 as libc::c_int as uint32_t,
    0x496b153c as libc::c_int as uint32_t,
    0x4f1403fa as libc::c_int as uint32_t,
    0x541afb0c as libc::c_int as uint32_t,
    0x73990b32 as libc::c_int as uint32_t,
    0x26d7cb1c as libc::c_int as uint32_t,
    0x6fcc3706 as libc::c_int as uint32_t,
    0x2cbb77d8 as libc::c_int as uint32_t,
    0x75762f2a as libc::c_int as uint32_t,
    0x6425ccdd as libc::c_int as uint32_t,
    0x24b35461 as libc::c_int as uint32_t,
    0xa7d8715 as libc::c_int as uint32_t,
    0x220414a8 as libc::c_int as uint32_t,
    0x141ebf67 as libc::c_int as uint32_t,
    0x56b41583 as libc::c_int as uint32_t,
    0x73e502e3 as libc::c_int as uint32_t,
    0x44cab16f as libc::c_int as uint32_t,
    0x28264d42 as libc::c_int as uint32_t,
    0x73baaefb as libc::c_int as uint32_t,
    0xa50ebed as libc::c_int as uint32_t,
    0x1d6ab6fb as libc::c_int as uint32_t,
    0xd3ad40b as libc::c_int as uint32_t,
    0x35db3b68 as libc::c_int as uint32_t,
    0x2b081e83 as libc::c_int as uint32_t,
    0x77ce6b95 as libc::c_int as uint32_t,
    0x5181e5f0 as libc::c_int as uint32_t,
    0x78853bbc as libc::c_int as uint32_t,
    0x9f9494 as libc::c_int as uint32_t,
    0x27e5ed3c as libc::c_int as uint32_t,
];
#[no_mangle]
pub unsafe extern "C" fn VP8InitRandom(
    rg: *mut VP8Random,
    mut dithering: libc::c_float,
) {
    memcpy(
        ((*rg).tab_).as_mut_ptr() as *mut libc::c_void,
        kRandomTable.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 55]>() as libc::c_ulong,
    );
    (*rg).index1_ = 0 as libc::c_int;
    (*rg).index2_ = 31 as libc::c_int;
    (*rg)
        .amp_ = (if (dithering as libc::c_double) < 0.0f64 {
        0 as libc::c_int as libc::c_uint
    } else if dithering as libc::c_double > 1.0f64 {
        ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint
    } else {
        (((1 as libc::c_int) << 8 as libc::c_int) as libc::c_float * dithering)
            as uint32_t
    }) as libc::c_int;
}
