use ::libc;
extern "C" {
    fn WebPRescalerInit(
        rescaler: *mut WebPRescaler,
        src_width: libc::c_int,
        src_height: libc::c_int,
        dst: *mut uint8_t,
        dst_width: libc::c_int,
        dst_height: libc::c_int,
        dst_stride: libc::c_int,
        num_channels: libc::c_int,
        work: *mut rescaler_t,
    ) -> libc::c_int;
    fn WebPRescaleNeededLines(
        rescaler: *const WebPRescaler,
        max_num_lines: libc::c_int,
    ) -> libc::c_int;
    fn WebPRescalerImport(
        rescaler: *mut WebPRescaler,
        num_rows: libc::c_int,
        src: *const uint8_t,
        src_stride: libc::c_int,
    ) -> libc::c_int;
    fn WebPRescalerExport(rescaler: *mut WebPRescaler) -> libc::c_int;
    fn WebPInitAlphaProcessing();
    static mut WebPApplyAlphaMultiply: Option::<
        unsafe extern "C" fn(
            *mut uint8_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >;
    static mut WebPDispatchAlpha: Option::<
        unsafe extern "C" fn(
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut uint8_t,
            libc::c_int,
        ) -> libc::c_int,
    >;
    static mut WebPApplyAlphaMultiply4444: Option::<
        unsafe extern "C" fn(*mut uint8_t, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    fn WebPInitUpsamplers();
    static mut WebPUpsamplers: [WebPUpsampleLinePairFunc; 0];
    static mut WebPSamplers: [WebPSamplerRowFunc; 0];
    fn WebPSamplerProcessPlane(
        y: *const uint8_t,
        y_stride: libc::c_int,
        u: *const uint8_t,
        v: *const uint8_t,
        uv_stride: libc::c_int,
        dst: *mut uint8_t,
        dst_stride: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        func: WebPSamplerRowFunc,
    );
    fn WebPInitSamplers();
    fn WebPMultRows(
        ptr: *mut uint8_t,
        stride: libc::c_int,
        alpha: *const uint8_t,
        alpha_stride: libc::c_int,
        width: libc::c_int,
        num_rows: libc::c_int,
        inverse: libc::c_int,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPRescalerExportRow(wrk: *mut WebPRescaler);
    fn WebPInitYUV444Converters();
    static mut WebPYUV444Converters: [WebPYUV444Converter; 0];
    fn WebPIoInitFromOptions(
        options: *const WebPDecoderOptions,
        io: *mut VP8Io,
        src_colorspace: WEBP_CSP_MODE,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPCopyPlane(
        src: *const uint8_t,
        src_stride: libc::c_int,
        dst: *mut uint8_t,
        dst_stride: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type rescaler_t = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPRGBABuffer {
    pub rgba: *mut uint8_t,
    pub stride: libc::c_int,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPYUVABuffer {
    pub y: *mut uint8_t,
    pub u: *mut uint8_t,
    pub v: *mut uint8_t,
    pub a: *mut uint8_t,
    pub y_stride: libc::c_int,
    pub u_stride: libc::c_int,
    pub v_stride: libc::c_int,
    pub a_stride: libc::c_int,
    pub y_size: size_t,
    pub u_size: size_t,
    pub v_size: size_t,
    pub a_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecBuffer {
    pub colorspace: WEBP_CSP_MODE,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub is_external_memory: libc::c_int,
    pub u: C2RustUnnamed,
    pub pad: [uint32_t; 4],
    pub private_memory: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub RGBA: WebPRGBABuffer,
    pub YUVA: WebPYUVABuffer,
}
pub type WEBP_CSP_MODE = libc::c_uint;
pub const MODE_LAST: WEBP_CSP_MODE = 13;
pub const MODE_YUVA: WEBP_CSP_MODE = 12;
pub const MODE_YUV: WEBP_CSP_MODE = 11;
pub const MODE_rgbA_4444: WEBP_CSP_MODE = 10;
pub const MODE_Argb: WEBP_CSP_MODE = 9;
pub const MODE_bgrA: WEBP_CSP_MODE = 8;
pub const MODE_rgbA: WEBP_CSP_MODE = 7;
pub const MODE_RGB_565: WEBP_CSP_MODE = 6;
pub const MODE_RGBA_4444: WEBP_CSP_MODE = 5;
pub const MODE_ARGB: WEBP_CSP_MODE = 4;
pub const MODE_BGRA: WEBP_CSP_MODE = 3;
pub const MODE_BGR: WEBP_CSP_MODE = 2;
pub const MODE_RGBA: WEBP_CSP_MODE = 1;
pub const MODE_RGB: WEBP_CSP_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecoderOptions {
    pub bypass_filtering: libc::c_int,
    pub no_fancy_upsampling: libc::c_int,
    pub use_cropping: libc::c_int,
    pub crop_left: libc::c_int,
    pub crop_top: libc::c_int,
    pub crop_width: libc::c_int,
    pub crop_height: libc::c_int,
    pub use_scaling: libc::c_int,
    pub scaled_width: libc::c_int,
    pub scaled_height: libc::c_int,
    pub use_threads: libc::c_int,
    pub dithering_strength: libc::c_int,
    pub flip: libc::c_int,
    pub alpha_dithering_strength: libc::c_int,
    pub pad: [uint32_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VP8Io {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mb_y: libc::c_int,
    pub mb_w: libc::c_int,
    pub mb_h: libc::c_int,
    pub y: *const uint8_t,
    pub u: *const uint8_t,
    pub v: *const uint8_t,
    pub y_stride: libc::c_int,
    pub uv_stride: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub put: VP8IoPutHook,
    pub setup: VP8IoSetupHook,
    pub teardown: VP8IoTeardownHook,
    pub fancy_upsampling: libc::c_int,
    pub data_size: size_t,
    pub data: *const uint8_t,
    pub bypass_filtering: libc::c_int,
    pub use_cropping: libc::c_int,
    pub crop_left: libc::c_int,
    pub crop_right: libc::c_int,
    pub crop_top: libc::c_int,
    pub crop_bottom: libc::c_int,
    pub use_scaling: libc::c_int,
    pub scaled_width: libc::c_int,
    pub scaled_height: libc::c_int,
    pub a: *const uint8_t,
}
pub type VP8IoTeardownHook = Option::<unsafe extern "C" fn(*const VP8Io) -> ()>;
pub type VP8IoSetupHook = Option::<unsafe extern "C" fn(*mut VP8Io) -> libc::c_int>;
pub type VP8IoPutHook = Option::<unsafe extern "C" fn(*const VP8Io) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDecParams {
    pub output: *mut WebPDecBuffer,
    pub tmp_y: *mut uint8_t,
    pub tmp_u: *mut uint8_t,
    pub tmp_v: *mut uint8_t,
    pub last_y: libc::c_int,
    pub options: *const WebPDecoderOptions,
    pub scaler_y: *mut WebPRescaler,
    pub scaler_u: *mut WebPRescaler,
    pub scaler_v: *mut WebPRescaler,
    pub scaler_a: *mut WebPRescaler,
    pub memory: *mut libc::c_void,
    pub emit: OutputFunc,
    pub emit_alpha: OutputAlphaFunc,
    pub emit_alpha_row: OutputRowFunc,
}
pub type OutputRowFunc = Option::<
    unsafe extern "C" fn(*mut WebPDecParams, libc::c_int, libc::c_int) -> libc::c_int,
>;
pub type OutputAlphaFunc = Option::<
    unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams, libc::c_int) -> libc::c_int,
>;
pub type OutputFunc = Option::<
    unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> libc::c_int,
>;
pub type WebPUpsampleLinePairFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type WebPSamplerRowFunc = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type WebPYUV444Converter = Option::<
    unsafe extern "C" fn(
        *const uint8_t,
        *const uint8_t,
        *const uint8_t,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
#[inline]
unsafe extern "C" fn WebPRescalerOutputDone(
    rescaler: *const WebPRescaler,
) -> libc::c_int {
    return ((*rescaler).dst_y >= (*rescaler).dst_height) as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPRescalerHasPendingOutput(
    rescaler: *const WebPRescaler,
) -> libc::c_int {
    return (WebPRescalerOutputDone(rescaler) == 0
        && (*rescaler).y_accum <= 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsPremultipliedMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return (mode as libc::c_uint == MODE_rgbA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_bgrA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_Argb as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_rgbA_4444 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsAlphaMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return (mode as libc::c_uint == MODE_RGBA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_BGRA as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_ARGB as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_RGBA_4444 as libc::c_int as libc::c_uint
        || mode as libc::c_uint == MODE_YUVA as libc::c_int as libc::c_uint
        || WebPIsPremultipliedMode(mode) != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn WebPIsRGBMode(mut mode: WEBP_CSP_MODE) -> libc::c_int {
    return ((mode as libc::c_uint) < MODE_YUV as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn CheckSizeOverflow(mut size: uint64_t) -> libc::c_int {
    return (size == size) as libc::c_int;
}
unsafe extern "C" fn EmitYUV(io: *const VP8Io, p: *mut WebPDecParams) -> libc::c_int {
    let mut output: *mut WebPDecBuffer = (*p).output;
    let buf: *const WebPYUVABuffer = &mut (*output).u.YUVA;
    let y_dst: *mut uint8_t = ((*buf).y)
        .offset(
            ((*io).mb_y as size_t).wrapping_mul((*buf).y_stride as libc::c_ulong)
                as isize,
        );
    let u_dst: *mut uint8_t = ((*buf).u)
        .offset(
            (((*io).mb_y >> 1 as libc::c_int) as size_t)
                .wrapping_mul((*buf).u_stride as libc::c_ulong) as isize,
        );
    let v_dst: *mut uint8_t = ((*buf).v)
        .offset(
            (((*io).mb_y >> 1 as libc::c_int) as size_t)
                .wrapping_mul((*buf).v_stride as libc::c_ulong) as isize,
        );
    let mb_w: libc::c_int = (*io).mb_w;
    let mb_h: libc::c_int = (*io).mb_h;
    let uv_w: libc::c_int = (mb_w + 1 as libc::c_int) / 2 as libc::c_int;
    let uv_h: libc::c_int = (mb_h + 1 as libc::c_int) / 2 as libc::c_int;
    WebPCopyPlane((*io).y, (*io).y_stride, y_dst, (*buf).y_stride, mb_w, mb_h);
    WebPCopyPlane((*io).u, (*io).uv_stride, u_dst, (*buf).u_stride, uv_w, uv_h);
    WebPCopyPlane((*io).v, (*io).uv_stride, v_dst, (*buf).v_stride, uv_w, uv_h);
    return (*io).mb_h;
}
unsafe extern "C" fn EmitSampledRGB(
    io: *const VP8Io,
    p: *mut WebPDecParams,
) -> libc::c_int {
    let output: *mut WebPDecBuffer = (*p).output;
    let buf: *mut WebPRGBABuffer = &mut (*output).u.RGBA;
    let dst: *mut uint8_t = ((*buf).rgba)
        .offset(
            ((*io).mb_y as size_t).wrapping_mul((*buf).stride as libc::c_ulong) as isize,
        );
    WebPSamplerProcessPlane(
        (*io).y,
        (*io).y_stride,
        (*io).u,
        (*io).v,
        (*io).uv_stride,
        dst,
        (*buf).stride,
        (*io).mb_w,
        (*io).mb_h,
        *WebPSamplers.as_mut_ptr().offset((*output).colorspace as isize),
    );
    return (*io).mb_h;
}
unsafe extern "C" fn EmitFancyRGB(
    io: *const VP8Io,
    p: *mut WebPDecParams,
) -> libc::c_int {
    let mut num_lines_out: libc::c_int = (*io).mb_h;
    let buf: *const WebPRGBABuffer = &mut (*(*p).output).u.RGBA;
    let mut dst: *mut uint8_t = ((*buf).rgba)
        .offset(
            ((*io).mb_y as size_t).wrapping_mul((*buf).stride as libc::c_ulong) as isize,
        );
    let mut upsample: WebPUpsampleLinePairFunc = *WebPUpsamplers
        .as_mut_ptr()
        .offset((*(*p).output).colorspace as isize);
    let mut cur_y: *const uint8_t = (*io).y;
    let mut cur_u: *const uint8_t = (*io).u;
    let mut cur_v: *const uint8_t = (*io).v;
    let mut top_u: *const uint8_t = (*p).tmp_u;
    let mut top_v: *const uint8_t = (*p).tmp_v;
    let mut y: libc::c_int = (*io).mb_y;
    let y_end: libc::c_int = (*io).mb_y + (*io).mb_h;
    let mb_w: libc::c_int = (*io).mb_w;
    let uv_w: libc::c_int = (mb_w + 1 as libc::c_int) / 2 as libc::c_int;
    if y == 0 as libc::c_int {
        upsample
            .expect(
                "non-null function pointer",
            )(
            cur_y,
            0 as *const uint8_t,
            cur_u,
            cur_v,
            cur_u,
            cur_v,
            dst,
            0 as *mut uint8_t,
            mb_w,
        );
    } else {
        upsample
            .expect(
                "non-null function pointer",
            )(
            (*p).tmp_y,
            cur_y,
            top_u,
            top_v,
            cur_u,
            cur_v,
            dst.offset(-((*buf).stride as isize)),
            dst,
            mb_w,
        );
        num_lines_out += 1;
        num_lines_out;
    }
    while (y + 2 as libc::c_int) < y_end {
        top_u = cur_u;
        top_v = cur_v;
        cur_u = cur_u.offset((*io).uv_stride as isize);
        cur_v = cur_v.offset((*io).uv_stride as isize);
        dst = dst.offset((2 as libc::c_int * (*buf).stride) as isize);
        cur_y = cur_y.offset((2 as libc::c_int * (*io).y_stride) as isize);
        upsample
            .expect(
                "non-null function pointer",
            )(
            cur_y.offset(-((*io).y_stride as isize)),
            cur_y,
            top_u,
            top_v,
            cur_u,
            cur_v,
            dst.offset(-((*buf).stride as isize)),
            dst,
            mb_w,
        );
        y += 2 as libc::c_int;
    }
    cur_y = cur_y.offset((*io).y_stride as isize);
    if (*io).crop_top + y_end < (*io).crop_bottom {
        memcpy(
            (*p).tmp_y as *mut libc::c_void,
            cur_y as *const libc::c_void,
            (mb_w as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
        memcpy(
            (*p).tmp_u as *mut libc::c_void,
            cur_u as *const libc::c_void,
            (uv_w as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
        memcpy(
            (*p).tmp_v as *mut libc::c_void,
            cur_v as *const libc::c_void,
            (uv_w as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
        num_lines_out -= 1;
        num_lines_out;
    } else if y_end & 1 as libc::c_int == 0 {
        upsample
            .expect(
                "non-null function pointer",
            )(
            cur_y,
            0 as *const uint8_t,
            cur_u,
            cur_v,
            cur_u,
            cur_v,
            dst.offset((*buf).stride as isize),
            0 as *mut uint8_t,
            mb_w,
        );
    }
    return num_lines_out;
}
unsafe extern "C" fn FillAlphaPlane(
    mut dst: *mut uint8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut stride: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        memset(
            dst as *mut libc::c_void,
            0xff as libc::c_int,
            (w as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
        dst = dst.offset(stride as isize);
        j += 1;
        j;
    }
}
unsafe extern "C" fn EmitAlphaYUV(
    io: *const VP8Io,
    p: *mut WebPDecParams,
    mut expected_num_lines_out: libc::c_int,
) -> libc::c_int {
    let mut alpha: *const uint8_t = (*io).a;
    let buf: *const WebPYUVABuffer = &mut (*(*p).output).u.YUVA;
    let mb_w: libc::c_int = (*io).mb_w;
    let mb_h: libc::c_int = (*io).mb_h;
    let mut dst: *mut uint8_t = ((*buf).a)
        .offset(
            ((*io).mb_y as size_t).wrapping_mul((*buf).a_stride as libc::c_ulong)
                as isize,
        );
    let mut j: libc::c_int = 0;
    if !alpha.is_null() {
        j = 0 as libc::c_int;
        while j < mb_h {
            memcpy(
                dst as *mut libc::c_void,
                alpha as *const libc::c_void,
                (mb_w as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
            );
            alpha = alpha.offset((*io).width as isize);
            dst = dst.offset((*buf).a_stride as isize);
            j += 1;
            j;
        }
    } else if !((*buf).a).is_null() {
        FillAlphaPlane(dst, mb_w, mb_h, (*buf).a_stride);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn GetAlphaSourceRow(
    io: *const VP8Io,
    mut alpha: *mut *const uint8_t,
    num_rows: *mut libc::c_int,
) -> libc::c_int {
    let mut start_y: libc::c_int = (*io).mb_y;
    *num_rows = (*io).mb_h;
    if (*io).fancy_upsampling != 0 {
        if start_y == 0 as libc::c_int {
            *num_rows -= 1;
            *num_rows;
        } else {
            start_y -= 1;
            start_y;
            *alpha = (*alpha).offset(-((*io).width as isize));
        }
        if (*io).crop_top + (*io).mb_y + (*io).mb_h == (*io).crop_bottom {
            *num_rows = (*io).crop_bottom - (*io).crop_top - start_y;
        }
    }
    return start_y;
}
unsafe extern "C" fn EmitAlphaRGB(
    io: *const VP8Io,
    p: *mut WebPDecParams,
    mut expected_num_lines_out: libc::c_int,
) -> libc::c_int {
    let mut alpha: *const uint8_t = (*io).a;
    if !alpha.is_null() {
        let mb_w: libc::c_int = (*io).mb_w;
        let colorspace: WEBP_CSP_MODE = (*(*p).output).colorspace;
        let alpha_first: libc::c_int = (colorspace as libc::c_uint
            == MODE_ARGB as libc::c_int as libc::c_uint
            || colorspace as libc::c_uint == MODE_Argb as libc::c_int as libc::c_uint)
            as libc::c_int;
        let buf: *const WebPRGBABuffer = &mut (*(*p).output).u.RGBA;
        let mut num_rows: libc::c_int = 0;
        let start_y: size_t = GetAlphaSourceRow(io, &mut alpha, &mut num_rows) as size_t;
        let base_rgba: *mut uint8_t = ((*buf).rgba)
            .offset(start_y.wrapping_mul((*buf).stride as libc::c_ulong) as isize);
        let dst: *mut uint8_t = base_rgba
            .offset(
                (if alpha_first != 0 { 0 as libc::c_int } else { 3 as libc::c_int })
                    as isize,
            );
        let has_alpha: libc::c_int = WebPDispatchAlpha
            .expect(
                "non-null function pointer",
            )(alpha, (*io).width, mb_w, num_rows, dst, (*buf).stride);
        if has_alpha != 0 && WebPIsPremultipliedMode(colorspace) != 0 {
            WebPApplyAlphaMultiply
                .expect(
                    "non-null function pointer",
                )(base_rgba, alpha_first, mb_w, num_rows, (*buf).stride);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn EmitAlphaRGBA4444(
    io: *const VP8Io,
    p: *mut WebPDecParams,
    mut expected_num_lines_out: libc::c_int,
) -> libc::c_int {
    let mut alpha: *const uint8_t = (*io).a;
    if !alpha.is_null() {
        let mb_w: libc::c_int = (*io).mb_w;
        let colorspace: WEBP_CSP_MODE = (*(*p).output).colorspace;
        let buf: *const WebPRGBABuffer = &mut (*(*p).output).u.RGBA;
        let mut num_rows: libc::c_int = 0;
        let start_y: size_t = GetAlphaSourceRow(io, &mut alpha, &mut num_rows) as size_t;
        let base_rgba: *mut uint8_t = ((*buf).rgba)
            .offset(start_y.wrapping_mul((*buf).stride as libc::c_ulong) as isize);
        let mut alpha_dst: *mut uint8_t = base_rgba.offset(1 as libc::c_int as isize);
        let mut alpha_mask: uint32_t = 0xf as libc::c_int as uint32_t;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < num_rows {
            i = 0 as libc::c_int;
            while i < mb_w {
                let alpha_value: uint32_t = (*alpha.offset(i as isize) as libc::c_int
                    >> 4 as libc::c_int) as uint32_t;
                *alpha_dst
                    .offset(
                        (2 as libc::c_int * i) as isize,
                    ) = ((*alpha_dst.offset((2 as libc::c_int * i) as isize)
                    as libc::c_int & 0xf0 as libc::c_int) as libc::c_uint | alpha_value)
                    as uint8_t;
                alpha_mask &= alpha_value;
                i += 1;
                i;
            }
            alpha = alpha.offset((*io).width as isize);
            alpha_dst = alpha_dst.offset((*buf).stride as isize);
            j += 1;
            j;
        }
        if alpha_mask != 0xf as libc::c_int as libc::c_uint
            && WebPIsPremultipliedMode(colorspace) != 0
        {
            WebPApplyAlphaMultiply4444
                .expect(
                    "non-null function pointer",
                )(base_rgba, mb_w, num_rows, (*buf).stride);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Rescale(
    mut src: *const uint8_t,
    mut src_stride: libc::c_int,
    mut new_lines: libc::c_int,
    wrk: *mut WebPRescaler,
) -> libc::c_int {
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    while new_lines > 0 as libc::c_int {
        let lines_in: libc::c_int = WebPRescalerImport(wrk, new_lines, src, src_stride);
        src = src.offset((lines_in * src_stride) as isize);
        new_lines -= lines_in;
        num_lines_out += WebPRescalerExport(wrk);
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledYUV(
    io: *const VP8Io,
    p: *mut WebPDecParams,
) -> libc::c_int {
    let mb_h: libc::c_int = (*io).mb_h;
    let uv_mb_h: libc::c_int = mb_h + 1 as libc::c_int >> 1 as libc::c_int;
    let scaler: *mut WebPRescaler = (*p).scaler_y;
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    if WebPIsAlphaMode((*(*p).output).colorspace) != 0 && !((*io).a).is_null() {
        WebPMultRows(
            (*io).y as *mut uint8_t,
            (*io).y_stride,
            (*io).a,
            (*io).width,
            (*io).mb_w,
            mb_h,
            0 as libc::c_int,
        );
    }
    num_lines_out = Rescale((*io).y, (*io).y_stride, mb_h, scaler);
    Rescale((*io).u, (*io).uv_stride, uv_mb_h, (*p).scaler_u);
    Rescale((*io).v, (*io).uv_stride, uv_mb_h, (*p).scaler_v);
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledAlphaYUV(
    io: *const VP8Io,
    p: *mut WebPDecParams,
    mut expected_num_lines_out: libc::c_int,
) -> libc::c_int {
    let buf: *const WebPYUVABuffer = &mut (*(*p).output).u.YUVA;
    let dst_a: *mut uint8_t = ((*buf).a)
        .offset(
            ((*p).last_y as size_t).wrapping_mul((*buf).a_stride as libc::c_ulong)
                as isize,
        );
    if !((*io).a).is_null() {
        let dst_y: *mut uint8_t = ((*buf).y)
            .offset(
                ((*p).last_y as size_t).wrapping_mul((*buf).y_stride as libc::c_ulong)
                    as isize,
            );
        let num_lines_out: libc::c_int = Rescale(
            (*io).a,
            (*io).width,
            (*io).mb_h,
            (*p).scaler_a,
        );
        if num_lines_out > 0 as libc::c_int {
            WebPMultRows(
                dst_y,
                (*buf).y_stride,
                dst_a,
                (*buf).a_stride,
                (*(*p).scaler_a).dst_width,
                num_lines_out,
                1 as libc::c_int,
            );
        }
    } else if !((*buf).a).is_null() {
        FillAlphaPlane(
            dst_a,
            (*io).scaled_width,
            expected_num_lines_out,
            (*buf).a_stride,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn InitYUVRescaler(
    io: *const VP8Io,
    p: *mut WebPDecParams,
) -> libc::c_int {
    let has_alpha: libc::c_int = WebPIsAlphaMode((*(*p).output).colorspace);
    let buf: *const WebPYUVABuffer = &mut (*(*p).output).u.YUVA;
    let out_width: libc::c_int = (*io).scaled_width;
    let out_height: libc::c_int = (*io).scaled_height;
    let uv_out_width: libc::c_int = out_width + 1 as libc::c_int >> 1 as libc::c_int;
    let uv_out_height: libc::c_int = out_height + 1 as libc::c_int >> 1 as libc::c_int;
    let uv_in_width: libc::c_int = (*io).mb_w + 1 as libc::c_int >> 1 as libc::c_int;
    let uv_in_height: libc::c_int = (*io).mb_h + 1 as libc::c_int >> 1 as libc::c_int;
    let work_size: size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(out_width as size_t);
    let uv_work_size: size_t = (2 as libc::c_int * uv_out_width) as size_t;
    let mut total_size: uint64_t = 0;
    let mut rescaler_size: size_t = 0;
    let mut work: *mut rescaler_t = 0 as *mut rescaler_t;
    let mut scalers: *mut WebPRescaler = 0 as *mut WebPRescaler;
    let num_rescalers: libc::c_int = if has_alpha != 0 {
        4 as libc::c_int
    } else {
        3 as libc::c_int
    };
    total_size = work_size
        .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(uv_work_size))
        .wrapping_mul(::core::mem::size_of::<rescaler_t>() as libc::c_ulong);
    if has_alpha != 0 {
        total_size = (total_size as libc::c_ulong)
            .wrapping_add(
                work_size
                    .wrapping_mul(::core::mem::size_of::<rescaler_t>() as libc::c_ulong),
            ) as uint64_t as uint64_t;
    }
    rescaler_size = (num_rescalers as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<WebPRescaler>() as libc::c_ulong)
        .wrapping_add(31 as libc::c_int as libc::c_ulong);
    total_size = (total_size as libc::c_ulong).wrapping_add(rescaler_size) as uint64_t
        as uint64_t;
    if CheckSizeOverflow(total_size) == 0 {
        return 0 as libc::c_int;
    }
    (*p).memory = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, total_size);
    if ((*p).memory).is_null() {
        return 0 as libc::c_int;
    }
    work = (*p).memory as *mut rescaler_t;
    scalers = (((work as *const uint8_t)
        .offset(total_size as isize)
        .offset(-(rescaler_size as isize)) as uintptr_t)
        .wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut WebPRescaler;
    (*p).scaler_y = &mut *scalers.offset(0 as libc::c_int as isize) as *mut WebPRescaler;
    (*p).scaler_u = &mut *scalers.offset(1 as libc::c_int as isize) as *mut WebPRescaler;
    (*p).scaler_v = &mut *scalers.offset(2 as libc::c_int as isize) as *mut WebPRescaler;
    (*p)
        .scaler_a = if has_alpha != 0 {
        &mut *scalers.offset(3 as libc::c_int as isize) as *mut WebPRescaler
    } else {
        0 as *mut WebPRescaler
    };
    if WebPRescalerInit(
        (*p).scaler_y,
        (*io).mb_w,
        (*io).mb_h,
        (*buf).y,
        out_width,
        out_height,
        (*buf).y_stride,
        1 as libc::c_int,
        work,
    ) == 0
        || WebPRescalerInit(
            (*p).scaler_u,
            uv_in_width,
            uv_in_height,
            (*buf).u,
            uv_out_width,
            uv_out_height,
            (*buf).u_stride,
            1 as libc::c_int,
            work.offset(work_size as isize),
        ) == 0
        || WebPRescalerInit(
            (*p).scaler_v,
            uv_in_width,
            uv_in_height,
            (*buf).v,
            uv_out_width,
            uv_out_height,
            (*buf).v_stride,
            1 as libc::c_int,
            work.offset(work_size as isize).offset(uv_work_size as isize),
        ) == 0
    {
        return 0 as libc::c_int;
    }
    (*p)
        .emit = Some(
        EmitRescaledYUV
            as unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> libc::c_int,
    );
    if has_alpha != 0 {
        if WebPRescalerInit(
            (*p).scaler_a,
            (*io).mb_w,
            (*io).mb_h,
            (*buf).a,
            out_width,
            out_height,
            (*buf).a_stride,
            1 as libc::c_int,
            work
                .offset(work_size as isize)
                .offset(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(uv_work_size)
                        as isize,
                ),
        ) == 0
        {
            return 0 as libc::c_int;
        }
        (*p)
            .emit_alpha = Some(
            EmitRescaledAlphaYUV
                as unsafe extern "C" fn(
                    *const VP8Io,
                    *mut WebPDecParams,
                    libc::c_int,
                ) -> libc::c_int,
        );
        WebPInitAlphaProcessing();
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ExportRGB(
    p: *mut WebPDecParams,
    mut y_pos: libc::c_int,
) -> libc::c_int {
    let convert: WebPYUV444Converter = *WebPYUV444Converters
        .as_mut_ptr()
        .offset((*(*p).output).colorspace as isize);
    let buf: *const WebPRGBABuffer = &mut (*(*p).output).u.RGBA;
    let mut dst: *mut uint8_t = ((*buf).rgba)
        .offset((y_pos as size_t).wrapping_mul((*buf).stride as libc::c_ulong) as isize);
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    while WebPRescalerHasPendingOutput((*p).scaler_y) != 0
        && WebPRescalerHasPendingOutput((*p).scaler_u) != 0
    {
        WebPRescalerExportRow((*p).scaler_y);
        WebPRescalerExportRow((*p).scaler_u);
        WebPRescalerExportRow((*p).scaler_v);
        convert
            .expect(
                "non-null function pointer",
            )(
            (*(*p).scaler_y).dst,
            (*(*p).scaler_u).dst,
            (*(*p).scaler_v).dst,
            dst,
            (*(*p).scaler_y).dst_width,
        );
        dst = dst.offset((*buf).stride as isize);
        num_lines_out += 1;
        num_lines_out;
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledRGB(
    io: *const VP8Io,
    p: *mut WebPDecParams,
) -> libc::c_int {
    let mb_h: libc::c_int = (*io).mb_h;
    let uv_mb_h: libc::c_int = mb_h + 1 as libc::c_int >> 1 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut uv_j: libc::c_int = 0 as libc::c_int;
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    while j < mb_h {
        let y_lines_in: libc::c_int = WebPRescalerImport(
            (*p).scaler_y,
            mb_h - j,
            ((*io).y)
                .offset(
                    (j as size_t).wrapping_mul((*io).y_stride as libc::c_ulong) as isize,
                ),
            (*io).y_stride,
        );
        j += y_lines_in;
        if WebPRescaleNeededLines((*p).scaler_u, uv_mb_h - uv_j) != 0 {
            let u_lines_in: libc::c_int = WebPRescalerImport(
                (*p).scaler_u,
                uv_mb_h - uv_j,
                ((*io).u)
                    .offset(
                        (uv_j as size_t).wrapping_mul((*io).uv_stride as libc::c_ulong)
                            as isize,
                    ),
                (*io).uv_stride,
            );
            let v_lines_in: libc::c_int = WebPRescalerImport(
                (*p).scaler_v,
                uv_mb_h - uv_j,
                ((*io).v)
                    .offset(
                        (uv_j as size_t).wrapping_mul((*io).uv_stride as libc::c_ulong)
                            as isize,
                    ),
                (*io).uv_stride,
            );
            uv_j += u_lines_in;
        }
        num_lines_out += ExportRGB(p, (*p).last_y + num_lines_out);
    }
    return num_lines_out;
}
unsafe extern "C" fn ExportAlpha(
    p: *mut WebPDecParams,
    mut y_pos: libc::c_int,
    mut max_lines_out: libc::c_int,
) -> libc::c_int {
    let buf: *const WebPRGBABuffer = &mut (*(*p).output).u.RGBA;
    let base_rgba: *mut uint8_t = ((*buf).rgba)
        .offset((y_pos as size_t).wrapping_mul((*buf).stride as libc::c_ulong) as isize);
    let colorspace: WEBP_CSP_MODE = (*(*p).output).colorspace;
    let alpha_first: libc::c_int = (colorspace as libc::c_uint
        == MODE_ARGB as libc::c_int as libc::c_uint
        || colorspace as libc::c_uint == MODE_Argb as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut dst: *mut uint8_t = base_rgba
        .offset(
            (if alpha_first != 0 { 0 as libc::c_int } else { 3 as libc::c_int }) as isize,
        );
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    let is_premult_alpha: libc::c_int = WebPIsPremultipliedMode(colorspace);
    let mut non_opaque: uint32_t = 0 as libc::c_int as uint32_t;
    let width: libc::c_int = (*(*p).scaler_a).dst_width;
    while WebPRescalerHasPendingOutput((*p).scaler_a) != 0
        && num_lines_out < max_lines_out
    {
        WebPRescalerExportRow((*p).scaler_a);
        non_opaque
            |= WebPDispatchAlpha
                .expect(
                    "non-null function pointer",
                )(
                (*(*p).scaler_a).dst,
                0 as libc::c_int,
                width,
                1 as libc::c_int,
                dst,
                0 as libc::c_int,
            ) as libc::c_uint;
        dst = dst.offset((*buf).stride as isize);
        num_lines_out += 1;
        num_lines_out;
    }
    if is_premult_alpha != 0 && non_opaque != 0 {
        WebPApplyAlphaMultiply
            .expect(
                "non-null function pointer",
            )(base_rgba, alpha_first, width, num_lines_out, (*buf).stride);
    }
    return num_lines_out;
}
unsafe extern "C" fn ExportAlphaRGBA4444(
    p: *mut WebPDecParams,
    mut y_pos: libc::c_int,
    mut max_lines_out: libc::c_int,
) -> libc::c_int {
    let buf: *const WebPRGBABuffer = &mut (*(*p).output).u.RGBA;
    let base_rgba: *mut uint8_t = ((*buf).rgba)
        .offset((y_pos as size_t).wrapping_mul((*buf).stride as libc::c_ulong) as isize);
    let mut alpha_dst: *mut uint8_t = base_rgba.offset(1 as libc::c_int as isize);
    let mut num_lines_out: libc::c_int = 0 as libc::c_int;
    let colorspace: WEBP_CSP_MODE = (*(*p).output).colorspace;
    let width: libc::c_int = (*(*p).scaler_a).dst_width;
    let is_premult_alpha: libc::c_int = WebPIsPremultipliedMode(colorspace);
    let mut alpha_mask: uint32_t = 0xf as libc::c_int as uint32_t;
    while WebPRescalerHasPendingOutput((*p).scaler_a) != 0
        && num_lines_out < max_lines_out
    {
        let mut i: libc::c_int = 0;
        WebPRescalerExportRow((*p).scaler_a);
        i = 0 as libc::c_int;
        while i < width {
            let alpha_value: uint32_t = (*((*(*p).scaler_a).dst).offset(i as isize)
                as libc::c_int >> 4 as libc::c_int) as uint32_t;
            *alpha_dst
                .offset(
                    (2 as libc::c_int * i) as isize,
                ) = ((*alpha_dst.offset((2 as libc::c_int * i) as isize) as libc::c_int
                & 0xf0 as libc::c_int) as libc::c_uint | alpha_value) as uint8_t;
            alpha_mask &= alpha_value;
            i += 1;
            i;
        }
        alpha_dst = alpha_dst.offset((*buf).stride as isize);
        num_lines_out += 1;
        num_lines_out;
    }
    if is_premult_alpha != 0 && alpha_mask != 0xf as libc::c_int as libc::c_uint {
        WebPApplyAlphaMultiply4444
            .expect(
                "non-null function pointer",
            )(base_rgba, width, num_lines_out, (*buf).stride);
    }
    return num_lines_out;
}
unsafe extern "C" fn EmitRescaledAlphaRGB(
    io: *const VP8Io,
    p: *mut WebPDecParams,
    mut expected_num_out_lines: libc::c_int,
) -> libc::c_int {
    if !((*io).a).is_null() {
        let scaler: *mut WebPRescaler = (*p).scaler_a;
        let mut lines_left: libc::c_int = expected_num_out_lines;
        let y_end: libc::c_int = (*p).last_y + lines_left;
        while lines_left > 0 as libc::c_int {
            let row_offset: int64_t = (*scaler).src_y as int64_t
                - (*io).mb_y as libc::c_long;
            WebPRescalerImport(
                scaler,
                (*io).mb_h + (*io).mb_y - (*scaler).src_y,
                ((*io).a).offset((row_offset * (*io).width as libc::c_long) as isize),
                (*io).width,
            );
            lines_left
                -= ((*p).emit_alpha_row)
                    .expect(
                        "non-null function pointer",
                    )(p, y_end - lines_left, lines_left);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn InitRGBRescaler(
    io: *const VP8Io,
    p: *mut WebPDecParams,
) -> libc::c_int {
    let has_alpha: libc::c_int = WebPIsAlphaMode((*(*p).output).colorspace);
    let out_width: libc::c_int = (*io).scaled_width;
    let out_height: libc::c_int = (*io).scaled_height;
    let uv_in_width: libc::c_int = (*io).mb_w + 1 as libc::c_int >> 1 as libc::c_int;
    let uv_in_height: libc::c_int = (*io).mb_h + 1 as libc::c_int >> 1 as libc::c_int;
    let work_size: size_t = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(out_width as size_t);
    let mut work: *mut rescaler_t = 0 as *mut rescaler_t;
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_size1: uint64_t = 0;
    let mut tmp_size2: uint64_t = 0;
    let mut total_size: uint64_t = 0;
    let mut rescaler_size: size_t = 0;
    let mut scalers: *mut WebPRescaler = 0 as *mut WebPRescaler;
    let num_rescalers: libc::c_int = if has_alpha != 0 {
        4 as libc::c_int
    } else {
        3 as libc::c_int
    };
    tmp_size1 = (num_rescalers as uint64_t).wrapping_mul(work_size);
    tmp_size2 = (num_rescalers as uint64_t).wrapping_mul(out_width as libc::c_ulong);
    total_size = tmp_size1
        .wrapping_mul(::core::mem::size_of::<rescaler_t>() as libc::c_ulong)
        .wrapping_add(
            tmp_size2.wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
    rescaler_size = (num_rescalers as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<WebPRescaler>() as libc::c_ulong)
        .wrapping_add(31 as libc::c_int as libc::c_ulong);
    total_size = (total_size as libc::c_ulong).wrapping_add(rescaler_size) as uint64_t
        as uint64_t;
    if CheckSizeOverflow(total_size) == 0 {
        return 0 as libc::c_int;
    }
    (*p).memory = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, total_size);
    if ((*p).memory).is_null() {
        return 0 as libc::c_int;
    }
    work = (*p).memory as *mut rescaler_t;
    tmp = work.offset(tmp_size1 as isize) as *mut uint8_t;
    scalers = (((work as *const uint8_t)
        .offset(total_size as isize)
        .offset(-(rescaler_size as isize)) as uintptr_t)
        .wrapping_add(31 as libc::c_int as libc::c_ulong)
        & !(31 as libc::c_int as uintptr_t)) as *mut WebPRescaler;
    (*p).scaler_y = &mut *scalers.offset(0 as libc::c_int as isize) as *mut WebPRescaler;
    (*p).scaler_u = &mut *scalers.offset(1 as libc::c_int as isize) as *mut WebPRescaler;
    (*p).scaler_v = &mut *scalers.offset(2 as libc::c_int as isize) as *mut WebPRescaler;
    (*p)
        .scaler_a = if has_alpha != 0 {
        &mut *scalers.offset(3 as libc::c_int as isize) as *mut WebPRescaler
    } else {
        0 as *mut WebPRescaler
    };
    if WebPRescalerInit(
        (*p).scaler_y,
        (*io).mb_w,
        (*io).mb_h,
        tmp.offset((0 as libc::c_int * out_width) as isize),
        out_width,
        out_height,
        0 as libc::c_int,
        1 as libc::c_int,
        work.offset((0 as libc::c_int as libc::c_ulong).wrapping_mul(work_size) as isize),
    ) == 0
        || WebPRescalerInit(
            (*p).scaler_u,
            uv_in_width,
            uv_in_height,
            tmp.offset((1 as libc::c_int * out_width) as isize),
            out_width,
            out_height,
            0 as libc::c_int,
            1 as libc::c_int,
            work
                .offset(
                    (1 as libc::c_int as libc::c_ulong).wrapping_mul(work_size) as isize,
                ),
        ) == 0
        || WebPRescalerInit(
            (*p).scaler_v,
            uv_in_width,
            uv_in_height,
            tmp.offset((2 as libc::c_int * out_width) as isize),
            out_width,
            out_height,
            0 as libc::c_int,
            1 as libc::c_int,
            work
                .offset(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(work_size) as isize,
                ),
        ) == 0
    {
        return 0 as libc::c_int;
    }
    (*p)
        .emit = Some(
        EmitRescaledRGB
            as unsafe extern "C" fn(*const VP8Io, *mut WebPDecParams) -> libc::c_int,
    );
    WebPInitYUV444Converters();
    if has_alpha != 0 {
        if WebPRescalerInit(
            (*p).scaler_a,
            (*io).mb_w,
            (*io).mb_h,
            tmp.offset((3 as libc::c_int * out_width) as isize),
            out_width,
            out_height,
            0 as libc::c_int,
            1 as libc::c_int,
            work
                .offset(
                    (3 as libc::c_int as libc::c_ulong).wrapping_mul(work_size) as isize,
                ),
        ) == 0
        {
            return 0 as libc::c_int;
        }
        (*p)
            .emit_alpha = Some(
            EmitRescaledAlphaRGB
                as unsafe extern "C" fn(
                    *const VP8Io,
                    *mut WebPDecParams,
                    libc::c_int,
                ) -> libc::c_int,
        );
        if (*(*p).output).colorspace as libc::c_uint
            == MODE_RGBA_4444 as libc::c_int as libc::c_uint
            || (*(*p).output).colorspace as libc::c_uint
                == MODE_rgbA_4444 as libc::c_int as libc::c_uint
        {
            (*p)
                .emit_alpha_row = Some(
                ExportAlphaRGBA4444
                    as unsafe extern "C" fn(
                        *mut WebPDecParams,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        } else {
            (*p)
                .emit_alpha_row = Some(
                ExportAlpha
                    as unsafe extern "C" fn(
                        *mut WebPDecParams,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        WebPInitAlphaProcessing();
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn CustomSetup(mut io: *mut VP8Io) -> libc::c_int {
    let p: *mut WebPDecParams = (*io).opaque as *mut WebPDecParams;
    let colorspace: WEBP_CSP_MODE = (*(*p).output).colorspace;
    let is_rgb: libc::c_int = WebPIsRGBMode(colorspace);
    let is_alpha: libc::c_int = WebPIsAlphaMode(colorspace);
    (*p).memory = 0 as *mut libc::c_void;
    (*p).emit = None;
    (*p).emit_alpha = None;
    (*p).emit_alpha_row = None;
    if WebPIoInitFromOptions(
        (*p).options,
        io,
        (if is_alpha != 0 { MODE_YUV as libc::c_int } else { MODE_YUVA as libc::c_int })
            as WEBP_CSP_MODE,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if is_alpha != 0 && WebPIsPremultipliedMode(colorspace) != 0 {
        WebPInitUpsamplers();
    }
    if (*io).use_scaling != 0 {
        let ok: libc::c_int = if is_rgb != 0 {
            InitRGBRescaler(io, p)
        } else {
            InitYUVRescaler(io, p)
        };
        if ok == 0 {
            return 0 as libc::c_int;
        }
    } else {
        if is_rgb != 0 {
            WebPInitSamplers();
            (*p)
                .emit = Some(
                EmitSampledRGB
                    as unsafe extern "C" fn(
                        *const VP8Io,
                        *mut WebPDecParams,
                    ) -> libc::c_int,
            );
            if (*io).fancy_upsampling != 0 {
                let uv_width: libc::c_int = (*io).mb_w + 1 as libc::c_int
                    >> 1 as libc::c_int;
                (*p)
                    .memory = WebPSafeMalloc(
                    1 as libc::c_ulonglong as uint64_t,
                    ((*io).mb_w + 2 as libc::c_int * uv_width) as size_t,
                );
                if ((*p).memory).is_null() {
                    return 0 as libc::c_int;
                }
                (*p).tmp_y = (*p).memory as *mut uint8_t;
                (*p).tmp_u = ((*p).tmp_y).offset((*io).mb_w as isize);
                (*p).tmp_v = ((*p).tmp_u).offset(uv_width as isize);
                (*p)
                    .emit = Some(
                    EmitFancyRGB
                        as unsafe extern "C" fn(
                            *const VP8Io,
                            *mut WebPDecParams,
                        ) -> libc::c_int,
                );
                WebPInitUpsamplers();
            }
        } else {
            (*p)
                .emit = Some(
                EmitYUV
                    as unsafe extern "C" fn(
                        *const VP8Io,
                        *mut WebPDecParams,
                    ) -> libc::c_int,
            );
        }
        if is_alpha != 0 {
            (*p)
                .emit_alpha = if colorspace as libc::c_uint
                == MODE_RGBA_4444 as libc::c_int as libc::c_uint
                || colorspace as libc::c_uint
                    == MODE_rgbA_4444 as libc::c_int as libc::c_uint
            {
                Some(
                    EmitAlphaRGBA4444
                        as unsafe extern "C" fn(
                            *const VP8Io,
                            *mut WebPDecParams,
                            libc::c_int,
                        ) -> libc::c_int,
                )
            } else if is_rgb != 0 {
                Some(
                    EmitAlphaRGB
                        as unsafe extern "C" fn(
                            *const VP8Io,
                            *mut WebPDecParams,
                            libc::c_int,
                        ) -> libc::c_int,
                )
            } else {
                Some(
                    EmitAlphaYUV
                        as unsafe extern "C" fn(
                            *const VP8Io,
                            *mut WebPDecParams,
                            libc::c_int,
                        ) -> libc::c_int,
                )
            };
            if is_rgb != 0 {
                WebPInitAlphaProcessing();
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn CustomPut(mut io: *const VP8Io) -> libc::c_int {
    let p: *mut WebPDecParams = (*io).opaque as *mut WebPDecParams;
    let mb_w: libc::c_int = (*io).mb_w;
    let mb_h: libc::c_int = (*io).mb_h;
    let mut num_lines_out: libc::c_int = 0;
    if mb_w <= 0 as libc::c_int || mb_h <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    num_lines_out = ((*p).emit).expect("non-null function pointer")(io, p);
    if ((*p).emit_alpha).is_some() {
        ((*p).emit_alpha).expect("non-null function pointer")(io, p, num_lines_out);
    }
    (*p).last_y += num_lines_out;
    return 1 as libc::c_int;
}
unsafe extern "C" fn CustomTeardown(mut io: *const VP8Io) {
    let p: *mut WebPDecParams = (*io).opaque as *mut WebPDecParams;
    WebPSafeFree((*p).memory);
    (*p).memory = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn WebPInitCustomIo(params: *mut WebPDecParams, io: *mut VP8Io) {
    (*io).put = Some(CustomPut as unsafe extern "C" fn(*const VP8Io) -> libc::c_int);
    (*io).setup = Some(CustomSetup as unsafe extern "C" fn(*mut VP8Io) -> libc::c_int);
    (*io).teardown = Some(CustomTeardown as unsafe extern "C" fn(*const VP8Io) -> ());
    (*io).opaque = params as *mut libc::c_void;
}
