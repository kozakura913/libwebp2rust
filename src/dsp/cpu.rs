use ::libc;
use core::arch::asm;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
#[inline]
unsafe extern "C" fn GetCPUInfo(
    mut cpu_info: *mut libc::c_int,
    mut info_type: libc::c_int,
) {
    asm!(
        "cpuid\n\nmov {restmp0:x}, %bx", restmp0 = lateout(reg) * cpu_info.offset(1 as
        libc::c_int as isize), inlateout("ax") info_type => * cpu_info.offset(0 as
        libc::c_int as isize), inlateout("cx") 0 as libc::c_int => * cpu_info.offset(2 as
        libc::c_int as isize), lateout("dx") * cpu_info.offset(3 as libc::c_int as
        isize), options(preserves_flags, att_syntax)
    );
}
#[inline]
unsafe extern "C" fn xgetbv() -> uint64_t {
    let ecx: uint32_t = 0 as libc::c_int as uint32_t;
    let mut eax: uint32_t = 0;
    let mut edx: uint32_t = 0;
    asm!(
        ".byte 0x0f, 0x01, 0xd0\n", lateout("ax") eax, lateout("dx") edx, inlateout("cx")
        ecx => _, options(preserves_flags, att_syntax)
    );
    return (edx as uint64_t) << 32 as libc::c_int | eax as libc::c_ulong;
}
unsafe extern "C" fn CheckSlowModel(mut info: libc::c_int) -> libc::c_int {
    static mut kSlowModels: [uint8_t; 6] = [
        0x37 as libc::c_int as uint8_t,
        0x4a as libc::c_int as uint8_t,
        0x4d as libc::c_int as uint8_t,
        0x1c as libc::c_int as uint8_t,
        0x26 as libc::c_int as uint8_t,
        0x27 as libc::c_int as uint8_t,
    ];
    let model: uint32_t = ((info & 0xf0000 as libc::c_int) >> 12 as libc::c_int
        | info >> 4 as libc::c_int & 0xf as libc::c_int) as uint32_t;
    let family: uint32_t = (info >> 8 as libc::c_int & 0xf as libc::c_int) as uint32_t;
    if family == 0x6 as libc::c_int as libc::c_uint {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i
            < (::core::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<uint8_t>() as libc::c_ulong)
        {
            if model == kSlowModels[i as usize] as libc::c_uint {
                return 1 as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn x86CPUInfo(mut feature: CPUFeature) -> libc::c_int {
    let mut max_cpuid_value: libc::c_int = 0;
    let mut cpu_info: [libc::c_int; 4] = [0; 4];
    let mut is_intel: libc::c_int = 0 as libc::c_int;
    GetCPUInfo(cpu_info.as_mut_ptr(), 0 as libc::c_int);
    max_cpuid_value = cpu_info[0 as libc::c_int as usize];
    if max_cpuid_value < 1 as libc::c_int {
        return 0 as libc::c_int
    } else {
        let VENDOR_ID_INTEL_EBX: libc::c_int = 0x756e6547 as libc::c_int;
        let VENDOR_ID_INTEL_EDX: libc::c_int = 0x49656e69 as libc::c_int;
        let VENDOR_ID_INTEL_ECX: libc::c_int = 0x6c65746e as libc::c_int;
        is_intel = (cpu_info[1 as libc::c_int as usize] == VENDOR_ID_INTEL_EBX
            && cpu_info[2 as libc::c_int as usize] == VENDOR_ID_INTEL_ECX
            && cpu_info[3 as libc::c_int as usize] == VENDOR_ID_INTEL_EDX)
            as libc::c_int;
    }
    GetCPUInfo(cpu_info.as_mut_ptr(), 1 as libc::c_int);
    if feature as libc::c_uint == kSSE2 as libc::c_int as libc::c_uint {
        return (cpu_info[3 as libc::c_int as usize]
            & (1 as libc::c_int) << 26 as libc::c_int != 0) as libc::c_int;
    }
    if feature as libc::c_uint == kSSE3 as libc::c_int as libc::c_uint {
        return (cpu_info[2 as libc::c_int as usize]
            & (1 as libc::c_int) << 0 as libc::c_int != 0) as libc::c_int;
    }
    if feature as libc::c_uint == kSlowSSSE3 as libc::c_int as libc::c_uint {
        if is_intel != 0
            && cpu_info[2 as libc::c_int as usize]
                & (1 as libc::c_int) << 9 as libc::c_int != 0
        {
            return CheckSlowModel(cpu_info[0 as libc::c_int as usize]);
        }
        return 0 as libc::c_int;
    }
    if feature as libc::c_uint == kSSE4_1 as libc::c_int as libc::c_uint {
        return (cpu_info[2 as libc::c_int as usize]
            & (1 as libc::c_int) << 19 as libc::c_int != 0) as libc::c_int;
    }
    if feature as libc::c_uint == kAVX as libc::c_int as libc::c_uint {
        if cpu_info[2 as libc::c_int as usize] & 0x18000000 as libc::c_int
            == 0x18000000 as libc::c_int
        {
            return (xgetbv() & 0x6 as libc::c_int as libc::c_ulong
                == 0x6 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
    }
    if feature as libc::c_uint == kAVX2 as libc::c_int as libc::c_uint {
        if x86CPUInfo(kAVX) != 0 && max_cpuid_value >= 7 as libc::c_int {
            GetCPUInfo(cpu_info.as_mut_ptr(), 7 as libc::c_int);
            return (cpu_info[1 as libc::c_int as usize]
                & (1 as libc::c_int) << 5 as libc::c_int != 0) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut VP8GetCPUInfo: VP8CPUInfo = unsafe {
    Some(x86CPUInfo as unsafe extern "C" fn(CPUFeature) -> libc::c_int)
};
