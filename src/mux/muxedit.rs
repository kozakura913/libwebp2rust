use ::libc;

use crate::src::utils::types::{WebPChunk, WebPData, WebPMux, WebPMuxAnimParams, WebPMuxImage};
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn WebPFree(ptr: *mut libc::c_void);
    fn VP8LCheckSignature(data: *const uint8_t, size: size_t) -> libc::c_int;
    fn ChunkListDelete(chunk_list: *mut *mut WebPChunk);
    fn MuxImageDelete(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage;
    fn WebPMuxCreateInternal(
        _: *const WebPData,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut WebPMux;
    fn ChunkRelease(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn ChunkSetHead(
        chunk: *mut WebPChunk,
        chunk_list: *mut *mut WebPChunk,
    ) -> WebPMuxError;
    fn ChunkAssignData(
        chunk: *mut WebPChunk,
        data: *const WebPData,
        copy_data: libc::c_int,
        tag: uint32_t,
    ) -> WebPMuxError;
    fn ChunkGetIndexFromTag(tag: uint32_t) -> CHUNK_INDEX;
    fn ChunkInit(chunk: *mut WebPChunk);
    fn ChunkGetIdFromTag(tag: uint32_t) -> WebPChunkId;
    fn MuxGetChunkListFromId(
        mux: *const WebPMux,
        id: WebPChunkId,
    ) -> *mut *mut WebPChunk;
    fn ChunkDelete(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn ChunkGetTagFromFourCC(fourcc: *const libc::c_char) -> uint32_t;
    fn MuxImageRelease(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage;
    fn MuxImagePush(
        wpi: *const WebPMuxImage,
        wpi_list: *mut *mut WebPMuxImage,
    ) -> WebPMuxError;
    fn MuxImageFinalize(wpi: *mut WebPMuxImage) -> libc::c_int;
    static kChunks: [ChunkInfo; 11];
    fn MuxImageInit(wpi: *mut WebPMuxImage);
    fn MuxImageDeleteNth(
        wpi_list: *mut *mut WebPMuxImage,
        nth: uint32_t,
    ) -> WebPMuxError;
    fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: WebPChunkId,
        num_elements: *mut libc::c_int,
    ) -> WebPMuxError;
    fn MuxValidate(mux: *const WebPMux) -> WebPMuxError;
    fn ChunkListEmit(chunk_list: *const WebPChunk, dst: *mut uint8_t) -> *mut uint8_t;
    fn MuxImageEmit(wpi: *const WebPMuxImage, dst: *mut uint8_t) -> *mut uint8_t;
    fn MuxEmitRiffHeader(data: *mut uint8_t, size: size_t) -> *mut uint8_t;
    fn ChunkListDiskSize(chunk_list: *const WebPChunk) -> size_t;
    fn MuxImageDiskSize(wpi: *const WebPMuxImage) -> size_t;
    fn MuxHasAlpha(images: *const WebPMuxImage) -> libc::c_int;
    fn MuxImageGetNth(
        wpi_list: *mut *const WebPMuxImage,
        nth: uint32_t,
        wpi: *mut *mut WebPMuxImage,
    ) -> WebPMuxError;
    fn MuxImageCount(wpi_list: *const WebPMuxImage, id: WebPChunkId) -> libc::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type WebPFeatureFlags = libc::c_uint;
pub const ALL_VALID_FLAGS: WebPFeatureFlags = 62;
pub const ICCP_FLAG: WebPFeatureFlags = 32;
pub const ALPHA_FLAG: WebPFeatureFlags = 16;
pub const EXIF_FLAG: WebPFeatureFlags = 8;
pub const XMP_FLAG: WebPFeatureFlags = 4;
pub const ANIMATION_FLAG: WebPFeatureFlags = 2;
pub type WebPMuxAnimDispose = libc::c_uint;
pub const WEBP_MUX_DISPOSE_BACKGROUND: WebPMuxAnimDispose = 1;
pub const WEBP_MUX_DISPOSE_NONE: WebPMuxAnimDispose = 0;
pub type WebPMuxAnimBlend = libc::c_uint;
pub const WEBP_MUX_NO_BLEND: WebPMuxAnimBlend = 1;
pub const WEBP_MUX_BLEND: WebPMuxAnimBlend = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxFrameInfo {
    pub bitstream: WebPData,
    pub x_offset: libc::c_int,
    pub y_offset: libc::c_int,
    pub duration: libc::c_int,
    pub id: WebPChunkId,
    pub dispose_method: WebPMuxAnimDispose,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [uint32_t; 1],
}
pub type WebPChunkId = libc::c_uint;
pub const WEBP_CHUNK_NIL: WebPChunkId = 10;
pub const WEBP_CHUNK_UNKNOWN: WebPChunkId = 9;
pub const WEBP_CHUNK_XMP: WebPChunkId = 8;
pub const WEBP_CHUNK_EXIF: WebPChunkId = 7;
pub const WEBP_CHUNK_IMAGE: WebPChunkId = 6;
pub const WEBP_CHUNK_ALPHA: WebPChunkId = 5;
pub const WEBP_CHUNK_DEPRECATED: WebPChunkId = 4;
pub const WEBP_CHUNK_ANMF: WebPChunkId = 3;
pub const WEBP_CHUNK_ANIM: WebPChunkId = 2;
pub const WEBP_CHUNK_ICCP: WebPChunkId = 1;
pub const WEBP_CHUNK_VP8X: WebPChunkId = 0;
pub type WebPMuxError = libc::c_int;
pub const WEBP_MUX_NOT_ENOUGH_DATA: WebPMuxError = -4;
pub const WEBP_MUX_MEMORY_ERROR: WebPMuxError = -3;
pub const WEBP_MUX_BAD_DATA: WebPMuxError = -2;
pub const WEBP_MUX_INVALID_ARGUMENT: WebPMuxError = -1;
pub const WEBP_MUX_NOT_FOUND: WebPMuxError = 0;
pub const WEBP_MUX_OK: WebPMuxError = 1;
pub const IDX_UNKNOWN: CHUNK_INDEX = 9;
pub type CHUNK_INDEX = libc::c_uint;
pub const IDX_LAST_CHUNK: CHUNK_INDEX = 11;
pub const IDX_NIL: CHUNK_INDEX = 10;
pub const IDX_XMP: CHUNK_INDEX = 8;
pub const IDX_EXIF: CHUNK_INDEX = 7;
pub const IDX_VP8L: CHUNK_INDEX = 6;
pub const IDX_VP8: CHUNK_INDEX = 5;
pub const IDX_ALPHA: CHUNK_INDEX = 4;
pub const IDX_ANMF: CHUNK_INDEX = 3;
pub const IDX_ANIM: CHUNK_INDEX = 2;
pub const IDX_ICCP: CHUNK_INDEX = 1;
pub const IDX_VP8X: CHUNK_INDEX = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChunkInfo {
    pub tag: uint32_t,
    pub id: WebPChunkId,
    pub size: uint32_t,
}
#[inline]
unsafe extern "C" fn WebPDataInit(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        memset(
            webp_data as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<WebPData>() as libc::c_ulong,
        );
    }
}
#[inline]
unsafe extern "C" fn WebPDataClear(mut webp_data: *mut WebPData) {
    if !webp_data.is_null() {
        WebPFree((*webp_data).bytes as *mut libc::c_void);
        WebPDataInit(webp_data);
    }
}
#[inline]
unsafe extern "C" fn WebPMuxCreate(
    mut bitstream: *const WebPData,
    mut copy_data: libc::c_int,
) -> *mut WebPMux {
    return WebPMuxCreateInternal(bitstream, copy_data, 0x109 as libc::c_int);
}
#[inline]
unsafe extern "C" fn IsWPI(mut id: WebPChunkId) -> libc::c_int {
    match id as libc::c_uint {
        3 | 5 | 6 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[inline]
unsafe extern "C" fn GetLE16(data: *const uint8_t) -> libc::c_int {
    return (*data.offset(0 as libc::c_int as isize) as libc::c_int) << 0 as libc::c_int
        | (*data.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn GetLE24(data: *const uint8_t) -> libc::c_int {
    return GetLE16(data)
        | (*data.offset(2 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int;
}
#[inline]
unsafe extern "C" fn PutLE16(data: *mut uint8_t, mut val: libc::c_int) {
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (val >> 0 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (val >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE24(data: *mut uint8_t, mut val: libc::c_int) {
    PutLE16(data, val & 0xffff as libc::c_int);
    *data
        .offset(
            2 as libc::c_int as isize,
        ) = (val >> 16 as libc::c_int & 0xff as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn PutLE32(data: *mut uint8_t, mut val: uint32_t) {
    PutLE16(data, (val & 0xffff as libc::c_int as libc::c_uint) as libc::c_int);
    PutLE16(
        data.offset(2 as libc::c_int as isize),
        (val >> 16 as libc::c_int) as libc::c_int,
    );
}
unsafe extern "C" fn MuxInit(mux: *mut WebPMux) {
    memset(
        mux as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPMux>() as libc::c_ulong,
    );
    (*mux).canvas_width_ = 0 as libc::c_int;
    (*mux).canvas_height_ = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPNewInternal(mut version: libc::c_int) -> *mut WebPMux {
    if version >> 8 as libc::c_int != 0x109 as libc::c_int >> 8 as libc::c_int {
        return 0 as *mut WebPMux
    } else {
        let mux: *mut WebPMux = WebPSafeMalloc(
            1 as libc::c_ulonglong as uint64_t,
            ::core::mem::size_of::<WebPMux>() as libc::c_ulong,
        ) as *mut WebPMux;
        if !mux.is_null() {
            MuxInit(mux);
        }
        return mux;
    };
}
unsafe extern "C" fn DeleteAllImages(wpi_list: *mut *mut WebPMuxImage) {
    while !(*wpi_list).is_null() {
        *wpi_list = MuxImageDelete(*wpi_list);
    }
}
unsafe extern "C" fn MuxRelease(mux: *mut WebPMux) {
    DeleteAllImages(&mut (*mux).images_);
    ChunkListDelete(&mut (*mux).vp8x_);
    ChunkListDelete(&mut (*mux).iccp_);
    ChunkListDelete(&mut (*mux).anim_);
    ChunkListDelete(&mut (*mux).exif_);
    ChunkListDelete(&mut (*mux).xmp_);
    ChunkListDelete(&mut (*mux).unknown_);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxDelete(mut mux: *mut WebPMux) {
    if !mux.is_null() {
        MuxRelease(mux);
        WebPSafeFree(mux as *mut libc::c_void);
    }
}
unsafe extern "C" fn MuxSet(
    mux: *mut WebPMux,
    mut tag: uint32_t,
    data: *const WebPData,
    mut copy_data: libc::c_int,
) -> WebPMuxError {
    let mut chunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: 0 as *const uint8_t,
            size: 0,
        },
        next_: 0 as *mut WebPChunk,
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    let idx: CHUNK_INDEX = ChunkGetIndexFromTag(tag);
    ChunkInit(&mut chunk);
    if idx as libc::c_uint == IDX_VP8X as libc::c_int as libc::c_uint {
        err = ChunkAssignData(&mut chunk, data, copy_data, tag);
        if err as libc::c_int == WEBP_MUX_OK as libc::c_int {
            err = ChunkSetHead(&mut chunk, &mut (*mux).vp8x_);
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                ChunkRelease(&mut chunk);
            }
        }
        return err;
    }
    if idx as libc::c_uint == IDX_ICCP as libc::c_int as libc::c_uint {
        err = ChunkAssignData(&mut chunk, data, copy_data, tag);
        if err as libc::c_int == WEBP_MUX_OK as libc::c_int {
            err = ChunkSetHead(&mut chunk, &mut (*mux).iccp_);
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                ChunkRelease(&mut chunk);
            }
        }
        return err;
    }
    if idx as libc::c_uint == IDX_ANIM as libc::c_int as libc::c_uint {
        err = ChunkAssignData(&mut chunk, data, copy_data, tag);
        if err as libc::c_int == WEBP_MUX_OK as libc::c_int {
            err = ChunkSetHead(&mut chunk, &mut (*mux).anim_);
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                ChunkRelease(&mut chunk);
            }
        }
        return err;
    }
    if idx as libc::c_uint == IDX_EXIF as libc::c_int as libc::c_uint {
        err = ChunkAssignData(&mut chunk, data, copy_data, tag);
        if err as libc::c_int == WEBP_MUX_OK as libc::c_int {
            err = ChunkSetHead(&mut chunk, &mut (*mux).exif_);
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                ChunkRelease(&mut chunk);
            }
        }
        return err;
    }
    if idx as libc::c_uint == IDX_XMP as libc::c_int as libc::c_uint {
        err = ChunkAssignData(&mut chunk, data, copy_data, tag);
        if err as libc::c_int == WEBP_MUX_OK as libc::c_int {
            err = ChunkSetHead(&mut chunk, &mut (*mux).xmp_);
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                ChunkRelease(&mut chunk);
            }
        }
        return err;
    }
    if idx as libc::c_uint == IDX_UNKNOWN as libc::c_int as libc::c_uint {
        err = ChunkAssignData(&mut chunk, data, copy_data, tag);
        if err as libc::c_int == WEBP_MUX_OK as libc::c_int {
            err = ChunkSetHead(&mut chunk, &mut (*mux).unknown_);
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                ChunkRelease(&mut chunk);
            }
        }
        return err;
    }
    return err;
}
unsafe extern "C" fn CreateFrameData(
    mut width: libc::c_int,
    mut height: libc::c_int,
    info: *const WebPMuxFrameInfo,
    frame: *mut WebPData,
) -> WebPMuxError {
    let mut frame_bytes: *mut uint8_t = 0 as *mut uint8_t;
    let frame_size: size_t = kChunks[IDX_ANMF as libc::c_int as usize].size as size_t;
    frame_bytes = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, frame_size)
        as *mut uint8_t;
    if frame_bytes.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    PutLE24(
        frame_bytes.offset(0 as libc::c_int as isize),
        (*info).x_offset / 2 as libc::c_int,
    );
    PutLE24(
        frame_bytes.offset(3 as libc::c_int as isize),
        (*info).y_offset / 2 as libc::c_int,
    );
    PutLE24(frame_bytes.offset(6 as libc::c_int as isize), width - 1 as libc::c_int);
    PutLE24(frame_bytes.offset(9 as libc::c_int as isize), height - 1 as libc::c_int);
    PutLE24(frame_bytes.offset(12 as libc::c_int as isize), (*info).duration);
    *frame_bytes
        .offset(
            15 as libc::c_int as isize,
        ) = ((if (*info).blend_method as libc::c_uint
        == WEBP_MUX_NO_BLEND as libc::c_int as libc::c_uint
    {
        2 as libc::c_int
    } else {
        0 as libc::c_int
    })
        | (if (*info).dispose_method as libc::c_uint
            == WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int as libc::c_uint
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        })) as uint8_t;
    (*frame).bytes = frame_bytes;
    (*frame).size = frame_size;
    return WEBP_MUX_OK;
}
unsafe extern "C" fn GetImageData(
    bitstream: *const WebPData,
    image: *mut WebPData,
    alpha: *mut WebPData,
    is_lossless: *mut libc::c_int,
) -> WebPMuxError {
    WebPDataInit(alpha);
    if (*bitstream).size < 4 as libc::c_int as libc::c_ulong
        || memcmp(
            (*bitstream).bytes as *const libc::c_void,
            b"RIFF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
    {
        *image = *bitstream;
    } else {
        let mut wpi: *const WebPMuxImage = 0 as *const WebPMuxImage;
        let mux: *mut WebPMux = WebPMuxCreate(bitstream, 0 as libc::c_int);
        if mux.is_null() {
            return WEBP_MUX_BAD_DATA;
        }
        wpi = (*mux).images_;
        *image = (*(*wpi).img_).data_;
        if !((*wpi).alpha_).is_null() {
            *alpha = (*(*wpi).alpha_).data_;
        }
        WebPMuxDelete(mux);
    }
    *is_lossless = VP8LCheckSignature((*image).bytes, (*image).size);
    return WEBP_MUX_OK;
}
unsafe extern "C" fn DeleteChunks(
    mut chunk_list: *mut *mut WebPChunk,
    mut tag: uint32_t,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    while !(*chunk_list).is_null() {
        let chunk: *mut WebPChunk = *chunk_list;
        if (*chunk).tag_ == tag {
            *chunk_list = ChunkDelete(chunk);
            err = WEBP_MUX_OK;
        } else {
            chunk_list = &mut (*chunk).next_;
        }
    }
    return err;
}
unsafe extern "C" fn MuxDeleteAllNamedData(
    mux: *mut WebPMux,
    mut tag: uint32_t,
) -> WebPMuxError {
    let id: WebPChunkId = ChunkGetIdFromTag(tag);
    if IsWPI(id) != 0 {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return DeleteChunks(MuxGetChunkListFromId(mux, id), tag);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetChunk(
    mut mux: *mut WebPMux,
    mut fourcc: *const libc::c_char,
    mut chunk_data: *const WebPData,
    mut copy_data: libc::c_int,
) -> WebPMuxError {
    let mut tag: uint32_t = 0;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() || fourcc.is_null() || chunk_data.is_null()
        || ((*chunk_data).bytes).is_null()
        || (*chunk_data).size
            > (!(0 as libc::c_uint))
                .wrapping_sub(8 as libc::c_int as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    tag = ChunkGetTagFromFourCC(fourcc);
    err = MuxDeleteAllNamedData(mux, tag);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int
        && err as libc::c_int != WEBP_MUX_NOT_FOUND as libc::c_int
    {
        return err;
    }
    return MuxSet(mux, tag, chunk_data, copy_data);
}
unsafe extern "C" fn AddDataToChunkList(
    data: *const WebPData,
    mut copy_data: libc::c_int,
    mut tag: uint32_t,
    mut chunk_list: *mut *mut WebPChunk,
) -> WebPMuxError {
    let mut chunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: 0 as *const uint8_t,
            size: 0,
        },
        next_: 0 as *mut WebPChunk,
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    ChunkInit(&mut chunk);
    err = ChunkAssignData(&mut chunk, data, copy_data, tag);
    if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
        err = ChunkSetHead(&mut chunk, chunk_list);
        if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
            return WEBP_MUX_OK;
        }
    }
    ChunkRelease(&mut chunk);
    return err;
}
unsafe extern "C" fn SetAlphaAndImageChunks(
    bitstream: *const WebPData,
    mut copy_data: libc::c_int,
    wpi: *mut WebPMuxImage,
) -> WebPMuxError {
    let mut is_lossless: libc::c_int = 0 as libc::c_int;
    let mut image: WebPData = WebPData {
        bytes: 0 as *const uint8_t,
        size: 0,
    };
    let mut alpha: WebPData = WebPData {
        bytes: 0 as *const uint8_t,
        size: 0,
    };
    let mut err: WebPMuxError = GetImageData(
        bitstream,
        &mut image,
        &mut alpha,
        &mut is_lossless,
    );
    let image_tag: libc::c_int = (if is_lossless != 0 {
        kChunks[IDX_VP8L as libc::c_int as usize].tag
    } else {
        kChunks[IDX_VP8 as libc::c_int as usize].tag
    }) as libc::c_int;
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if !(alpha.bytes).is_null() {
        err = AddDataToChunkList(
            &mut alpha,
            copy_data,
            kChunks[IDX_ALPHA as libc::c_int as usize].tag,
            &mut (*wpi).alpha_,
        );
        if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
            return err;
        }
    }
    err = AddDataToChunkList(
        &mut image,
        copy_data,
        image_tag as uint32_t,
        &mut (*wpi).img_,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    return (if MuxImageFinalize(wpi) != 0 {
        WEBP_MUX_OK as libc::c_int
    } else {
        WEBP_MUX_INVALID_ARGUMENT as libc::c_int
    }) as WebPMuxError;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetImage(
    mut mux: *mut WebPMux,
    mut bitstream: *const WebPData,
    mut copy_data: libc::c_int,
) -> WebPMuxError {
    let mut wpi: WebPMuxImage = WebPMuxImage {
        header_: 0 as *mut WebPChunk,
        alpha_: 0 as *mut WebPChunk,
        img_: 0 as *mut WebPChunk,
        unknown_: 0 as *mut WebPChunk,
        width_: 0,
        height_: 0,
        has_alpha_: 0,
        is_partial_: 0,
        next_: 0 as *mut WebPMuxImage,
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() || bitstream.is_null() || ((*bitstream).bytes).is_null()
        || (*bitstream).size
            > (!(0 as libc::c_uint))
                .wrapping_sub(8 as libc::c_int as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if !((*mux).images_).is_null() {
        DeleteAllImages(&mut (*mux).images_);
    }
    MuxImageInit(&mut wpi);
    err = SetAlphaAndImageChunks(bitstream, copy_data, &mut wpi);
    if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
        err = MuxImagePush(&mut wpi, &mut (*mux).images_);
        if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
            return WEBP_MUX_OK;
        }
    }
    MuxImageRelease(&mut wpi);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxPushFrame(
    mut mux: *mut WebPMux,
    mut info: *const WebPMuxFrameInfo,
    mut copy_data: libc::c_int,
) -> WebPMuxError {
    let mut wpi: WebPMuxImage = WebPMuxImage {
        header_: 0 as *mut WebPChunk,
        alpha_: 0 as *mut WebPChunk,
        img_: 0 as *mut WebPChunk,
        unknown_: 0 as *mut WebPChunk,
        width_: 0,
        height_: 0,
        has_alpha_: 0,
        is_partial_: 0,
        next_: 0 as *mut WebPMuxImage,
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() || info.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*info).id as libc::c_uint != WEBP_CHUNK_ANMF as libc::c_int as libc::c_uint {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if ((*info).bitstream.bytes).is_null()
        || (*info).bitstream.size
            > (!(0 as libc::c_uint))
                .wrapping_sub(8 as libc::c_int as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if !((*mux).images_).is_null() {
        let image: *const WebPMuxImage = (*mux).images_;
        let image_id: uint32_t = if !((*image).header_).is_null() {
            ChunkGetIdFromTag((*(*image).header_).tag_) as libc::c_uint
        } else {
            WEBP_CHUNK_IMAGE as libc::c_int as libc::c_uint
        };
        if image_id != (*info).id as libc::c_uint {
            return WEBP_MUX_INVALID_ARGUMENT;
        }
    }
    MuxImageInit(&mut wpi);
    err = SetAlphaAndImageChunks(&(*info).bitstream, copy_data, &mut wpi);
    if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
        let mut frame: WebPData = WebPData {
            bytes: 0 as *const uint8_t,
            size: 0,
        };
        let tag: uint32_t = kChunks[IDX_ANMF as libc::c_int as usize].tag;
        let mut tmp: WebPMuxFrameInfo = *info;
        tmp.x_offset &= !(1 as libc::c_int);
        tmp.y_offset &= !(1 as libc::c_int);
        if tmp.x_offset < 0 as libc::c_int
            || tmp.x_offset >= (1 as libc::c_int) << 24 as libc::c_int
            || tmp.y_offset < 0 as libc::c_int
            || tmp.y_offset >= (1 as libc::c_int) << 24 as libc::c_int
            || (tmp.duration < 0 as libc::c_int
                || tmp.duration >= (1 as libc::c_int) << 24 as libc::c_int)
            || tmp.dispose_method as libc::c_uint
                != tmp.dispose_method as libc::c_uint & 1 as libc::c_int as libc::c_uint
        {
            err = WEBP_MUX_INVALID_ARGUMENT;
        } else {
            err = CreateFrameData(wpi.width_, wpi.height_, &mut tmp, &mut frame);
            if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
                err = AddDataToChunkList(
                    &mut frame,
                    1 as libc::c_int,
                    tag,
                    &mut wpi.header_,
                );
                WebPDataClear(&mut frame);
                if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
                    err = MuxImagePush(&mut wpi, &mut (*mux).images_);
                    if !(err as libc::c_int != WEBP_MUX_OK as libc::c_int) {
                        return WEBP_MUX_OK;
                    }
                }
            }
        }
    }
    MuxImageRelease(&mut wpi);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetAnimationParams(
    mut mux: *mut WebPMux,
    mut params: *const WebPMuxAnimParams,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    let mut data: [uint8_t; 6] = [0; 6];
    let anim: WebPData = {
        let mut init = WebPData {
            bytes: data.as_mut_ptr(),
            size: 6 as libc::c_int as size_t,
        };
        init
    };
    if mux.is_null() || params.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*params).loop_count < 0 as libc::c_int
        || (*params).loop_count >= (1 as libc::c_int) << 16 as libc::c_int
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxDeleteAllNamedData(mux, kChunks[IDX_ANIM as libc::c_int as usize].tag);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int
        && err as libc::c_int != WEBP_MUX_NOT_FOUND as libc::c_int
    {
        return err;
    }
    PutLE32(data.as_mut_ptr(), (*params).bgcolor);
    PutLE16(data.as_mut_ptr().offset(4 as libc::c_int as isize), (*params).loop_count);
    return MuxSet(
        mux,
        kChunks[IDX_ANIM as libc::c_int as usize].tag,
        &anim,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxSetCanvasSize(
    mut mux: *mut WebPMux,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if width < 0 as libc::c_int || height < 0 as libc::c_int
        || width > (1 as libc::c_int) << 24 as libc::c_int
        || height > (1 as libc::c_int) << 24 as libc::c_int
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (width as libc::c_ulong).wrapping_mul(height as uint64_t) as libc::c_ulonglong
        >= (1 as libc::c_ulonglong) << 32 as libc::c_int
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if width * height == 0 as libc::c_int && width | height != 0 as libc::c_int {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxDeleteAllNamedData(mux, kChunks[IDX_VP8X as libc::c_int as usize].tag);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int
        && err as libc::c_int != WEBP_MUX_NOT_FOUND as libc::c_int
    {
        return err;
    }
    (*mux).canvas_width_ = width;
    (*mux).canvas_height_ = height;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxDeleteChunk(
    mut mux: *mut WebPMux,
    mut fourcc: *const libc::c_char,
) -> WebPMuxError {
    if mux.is_null() || fourcc.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxDeleteAllNamedData(mux, ChunkGetTagFromFourCC(fourcc));
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxDeleteFrame(
    mut mux: *mut WebPMux,
    mut nth: uint32_t,
) -> WebPMuxError {
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxImageDeleteNth(&mut (*mux).images_, nth);
}
unsafe extern "C" fn GetFrameInfo(
    frame_chunk: *const WebPChunk,
    x_offset: *mut libc::c_int,
    y_offset: *mut libc::c_int,
    duration: *mut libc::c_int,
) -> WebPMuxError {
    let data: *const WebPData = &(*frame_chunk).data_;
    let expected_data_size: size_t = 16 as libc::c_int as size_t;
    if (*data).size != expected_data_size {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    *x_offset = 2 as libc::c_int
        * GetLE24(((*data).bytes).offset(0 as libc::c_int as isize));
    *y_offset = 2 as libc::c_int
        * GetLE24(((*data).bytes).offset(3 as libc::c_int as isize));
    *duration = GetLE24(((*data).bytes).offset(12 as libc::c_int as isize));
    return WEBP_MUX_OK;
}
unsafe extern "C" fn GetImageInfo(
    wpi: *const WebPMuxImage,
    x_offset: *mut libc::c_int,
    y_offset: *mut libc::c_int,
    duration: *mut libc::c_int,
    width: *mut libc::c_int,
    height: *mut libc::c_int,
) -> WebPMuxError {
    let frame_chunk: *const WebPChunk = (*wpi).header_;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    err = GetFrameInfo(frame_chunk, x_offset, y_offset, duration);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if !width.is_null() {
        *width = (*wpi).width_;
    }
    if !height.is_null() {
        *height = (*wpi).height_;
    }
    return WEBP_MUX_OK;
}
unsafe extern "C" fn GetAdjustedCanvasSize(
    mux: *const WebPMux,
    width: *mut libc::c_int,
    height: *mut libc::c_int,
) -> WebPMuxError {
    let mut wpi: *mut WebPMuxImage = 0 as *mut WebPMuxImage;
    wpi = (*mux).images_;
    if !((*wpi).next_).is_null() {
        let mut max_x: libc::c_int = 0 as libc::c_int;
        let mut max_y: libc::c_int = 0 as libc::c_int;
        while !wpi.is_null() {
            let mut x_offset: libc::c_int = 0 as libc::c_int;
            let mut y_offset: libc::c_int = 0 as libc::c_int;
            let mut duration: libc::c_int = 0 as libc::c_int;
            let mut w: libc::c_int = 0 as libc::c_int;
            let mut h: libc::c_int = 0 as libc::c_int;
            let err: WebPMuxError = GetImageInfo(
                wpi,
                &mut x_offset,
                &mut y_offset,
                &mut duration,
                &mut w,
                &mut h,
            );
            let max_x_pos: libc::c_int = x_offset + w;
            let max_y_pos: libc::c_int = y_offset + h;
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                return err;
            }
            if max_x_pos > max_x {
                max_x = max_x_pos;
            }
            if max_y_pos > max_y {
                max_y = max_y_pos;
            }
            wpi = (*wpi).next_;
        }
        *width = max_x;
        *height = max_y;
    } else {
        *width = (*wpi).width_;
        *height = (*wpi).height_;
    }
    return WEBP_MUX_OK;
}
unsafe extern "C" fn CreateVP8XChunk(mux: *mut WebPMux) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_OK;
    let mut flags: uint32_t = 0 as libc::c_int as uint32_t;
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut height: libc::c_int = 0 as libc::c_int;
    let mut data: [uint8_t; 10] = [0; 10];
    let vp8x: WebPData = {
        let mut init = WebPData {
            bytes: data.as_mut_ptr(),
            size: 10 as libc::c_int as size_t,
        };
        init
    };
    let mut images: *const WebPMuxImage = 0 as *const WebPMuxImage;
    images = (*mux).images_;
    if images.is_null() || ((*images).img_).is_null()
        || ((*(*images).img_).data_.bytes).is_null()
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxDeleteAllNamedData(mux, kChunks[IDX_VP8X as libc::c_int as usize].tag);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int
        && err as libc::c_int != WEBP_MUX_NOT_FOUND as libc::c_int
    {
        return err;
    }
    if !((*mux).iccp_).is_null() && !((*(*mux).iccp_).data_.bytes).is_null() {
        flags |= ICCP_FLAG as libc::c_int as libc::c_uint;
    }
    if !((*mux).exif_).is_null() && !((*(*mux).exif_).data_.bytes).is_null() {
        flags |= EXIF_FLAG as libc::c_int as libc::c_uint;
    }
    if !((*mux).xmp_).is_null() && !((*(*mux).xmp_).data_.bytes).is_null() {
        flags |= XMP_FLAG as libc::c_int as libc::c_uint;
    }
    if !((*images).header_).is_null() {
        if (*(*images).header_).tag_ == kChunks[IDX_ANMF as libc::c_int as usize].tag {
            flags |= ANIMATION_FLAG as libc::c_int as libc::c_uint;
        }
    }
    if MuxImageCount(images, WEBP_CHUNK_ALPHA) > 0 as libc::c_int {
        flags |= ALPHA_FLAG as libc::c_int as libc::c_uint;
    }
    err = GetAdjustedCanvasSize(mux, &mut width, &mut height);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if width > (1 as libc::c_int) << 24 as libc::c_int
        || height > (1 as libc::c_int) << 24 as libc::c_int
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if (*mux).canvas_width_ != 0 as libc::c_int
        || (*mux).canvas_height_ != 0 as libc::c_int
    {
        if width > (*mux).canvas_width_ || height > (*mux).canvas_height_ {
            return WEBP_MUX_INVALID_ARGUMENT;
        }
        width = (*mux).canvas_width_;
        height = (*mux).canvas_height_;
    }
    if flags == 0 as libc::c_int as libc::c_uint && ((*mux).unknown_).is_null() {
        return WEBP_MUX_OK;
    }
    if MuxHasAlpha(images) != 0 {
        flags |= ALPHA_FLAG as libc::c_int as libc::c_uint;
    }
    PutLE32(data.as_mut_ptr().offset(0 as libc::c_int as isize), flags);
    PutLE24(
        data.as_mut_ptr().offset(4 as libc::c_int as isize),
        width - 1 as libc::c_int,
    );
    PutLE24(
        data.as_mut_ptr().offset(7 as libc::c_int as isize),
        height - 1 as libc::c_int,
    );
    return MuxSet(
        mux,
        kChunks[IDX_VP8X as libc::c_int as usize].tag,
        &vp8x,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn MuxCleanup(mux: *mut WebPMux) -> WebPMuxError {
    let mut num_frames: libc::c_int = 0;
    let mut num_anim_chunks: libc::c_int = 0;
    let mut err: WebPMuxError = WebPMuxNumChunks(
        mux,
        kChunks[IDX_ANMF as libc::c_int as usize].id,
        &mut num_frames,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if num_frames == 1 as libc::c_int {
        let mut frame: *mut WebPMuxImage = 0 as *mut WebPMuxImage;
        err = MuxImageGetNth(
            &mut (*mux).images_ as *mut *mut WebPMuxImage as *mut *const WebPMuxImage,
            1 as libc::c_int as uint32_t,
            &mut frame,
        );
        if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
            return err;
        }
        if !((*frame).header_).is_null()
            && ((*mux).canvas_width_ == 0 as libc::c_int
                && (*mux).canvas_height_ == 0 as libc::c_int
                || (*frame).width_ == (*mux).canvas_width_
                    && (*frame).height_ == (*mux).canvas_height_)
        {
            ChunkDelete((*frame).header_);
            (*frame).header_ = 0 as *mut WebPChunk;
            num_frames = 0 as libc::c_int;
        }
    }
    err = WebPMuxNumChunks(
        mux,
        kChunks[IDX_ANIM as libc::c_int as usize].id,
        &mut num_anim_chunks,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if num_anim_chunks >= 1 as libc::c_int && num_frames == 0 as libc::c_int {
        err = MuxDeleteAllNamedData(mux, kChunks[IDX_ANIM as libc::c_int as usize].tag);
        if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
            return err;
        }
    }
    return WEBP_MUX_OK;
}
unsafe extern "C" fn ImageListDiskSize(mut wpi_list: *const WebPMuxImage) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    while !wpi_list.is_null() {
        size = (size as libc::c_ulong).wrapping_add(MuxImageDiskSize(wpi_list)) as size_t
            as size_t;
        wpi_list = (*wpi_list).next_;
    }
    return size;
}
unsafe extern "C" fn ImageListEmit(
    mut wpi_list: *const WebPMuxImage,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    while !wpi_list.is_null() {
        dst = MuxImageEmit(wpi_list, dst);
        wpi_list = (*wpi_list).next_;
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxAssemble(
    mut mux: *mut WebPMux,
    mut assembled_data: *mut WebPData,
) -> WebPMuxError {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut dst: *mut uint8_t = 0 as *mut uint8_t;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if assembled_data.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    memset(
        assembled_data as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPData>() as libc::c_ulong,
    );
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxCleanup(mux);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    err = CreateVP8XChunk(mux);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    size = (ChunkListDiskSize((*mux).vp8x_))
        .wrapping_add(ChunkListDiskSize((*mux).iccp_))
        .wrapping_add(ChunkListDiskSize((*mux).anim_))
        .wrapping_add(ImageListDiskSize((*mux).images_))
        .wrapping_add(ChunkListDiskSize((*mux).exif_))
        .wrapping_add(ChunkListDiskSize((*mux).xmp_))
        .wrapping_add(ChunkListDiskSize((*mux).unknown_))
        .wrapping_add(12 as libc::c_int as libc::c_ulong);
    data = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, size) as *mut uint8_t;
    if data.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    dst = MuxEmitRiffHeader(data, size);
    dst = ChunkListEmit((*mux).vp8x_, dst);
    dst = ChunkListEmit((*mux).iccp_, dst);
    dst = ChunkListEmit((*mux).anim_, dst);
    dst = ImageListEmit((*mux).images_, dst);
    dst = ChunkListEmit((*mux).exif_, dst);
    dst = ChunkListEmit((*mux).xmp_, dst);
    dst = ChunkListEmit((*mux).unknown_, dst);
    err = MuxValidate(mux);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        WebPSafeFree(data as *mut libc::c_void);
        data = 0 as *mut uint8_t;
        size = 0 as libc::c_int as size_t;
    }
    (*assembled_data).bytes = data;
    (*assembled_data).size = size;
    return err;
}
