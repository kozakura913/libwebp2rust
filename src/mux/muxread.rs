use ::libc;

use crate::src::utils::types::{ChunkInfo, WebPChunk, WebPChunkId, WebPData, WebPMux, WebPMuxAnimBlend, WebPMuxAnimDispose, WebPMuxAnimParams, WebPMuxError, WebPMuxFrameInfo, WebPMuxImage, ALPHA_FLAG, CHUNK_INDEX, IDX_ANIM, IDX_ANMF, IDX_EXIF, IDX_ICCP, IDX_NIL, IDX_UNKNOWN, IDX_VP8, IDX_VP8L, IDX_VP8X, IDX_XMP, WEBP_CHUNK_ANIM, WEBP_CHUNK_ANMF, WEBP_CHUNK_IMAGE, WEBP_CHUNK_NIL, WEBP_CHUNK_VP8X, WEBP_MUX_BAD_DATA, WEBP_MUX_BLEND, WEBP_MUX_DISPOSE_BACKGROUND, WEBP_MUX_DISPOSE_NONE, WEBP_MUX_INVALID_ARGUMENT, WEBP_MUX_MEMORY_ERROR, WEBP_MUX_NOT_ENOUGH_DATA, WEBP_MUX_NOT_FOUND, WEBP_MUX_NO_BLEND, WEBP_MUX_OK};
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VP8GetInfo(
        data: *const uint8_t,
        data_size: size_t,
        chunk_size: size_t,
        width: *mut libc::c_int,
        height: *mut libc::c_int,
    ) -> libc::c_int;
    fn VP8LGetInfo(
        data: *const uint8_t,
        data_size: size_t,
        width: *mut libc::c_int,
        height: *mut libc::c_int,
        has_alpha: *mut libc::c_int,
    ) -> libc::c_int;
    fn WebPNewInternal(_: libc::c_int) -> *mut WebPMux;
    fn WebPMuxDelete(mux: *mut WebPMux);
    fn MuxImageDelete(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage;
    fn ChunkRelease(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn MuxValidate(mux: *const WebPMux) -> WebPMuxError;
    fn ChunkInit(chunk: *mut WebPChunk);
    fn ChunkAppend(
        chunk: *mut WebPChunk,
        chunk_list: *mut *mut *mut WebPChunk,
    ) -> WebPMuxError;
    fn MuxGetChunkListFromId(
        mux: *const WebPMux,
        id: WebPChunkId,
    ) -> *mut *mut WebPChunk;
    fn MuxImagePush(
        wpi: *const WebPMuxImage,
        wpi_list: *mut *mut WebPMuxImage,
    ) -> WebPMuxError;
    static kChunks: [ChunkInfo; 11];
    fn ChunkDelete(chunk: *mut WebPChunk) -> *mut WebPChunk;
    fn ChunkSetHead(
        chunk: *mut WebPChunk,
        chunk_list: *mut *mut WebPChunk,
    ) -> WebPMuxError;
    fn ChunkGetIdFromTag(tag: uint32_t) -> WebPChunkId;
    fn ChunkAssignData(
        chunk: *mut WebPChunk,
        data: *const WebPData,
        copy_data: libc::c_int,
        tag: uint32_t,
    ) -> WebPMuxError;
    fn MuxImageInit(wpi: *mut WebPMuxImage);
    fn ChunkGetTagFromFourCC(fourcc: *const libc::c_char) -> uint32_t;
    fn ChunkSearchList(
        first: *mut WebPChunk,
        nth: uint32_t,
        tag: uint32_t,
    ) -> *mut WebPChunk;
    fn ChunkGetIndexFromFourCC(fourcc: *const libc::c_char) -> CHUNK_INDEX;
    fn ChunkListEmit(chunk_list: *const WebPChunk, dst: *mut uint8_t) -> *mut uint8_t;
    fn MuxEmitRiffHeader(data: *mut uint8_t, size: size_t) -> *mut uint8_t;
    fn MuxImageGetNth(
        wpi_list: *mut *const WebPMuxImage,
        nth: uint32_t,
        wpi: *mut *mut WebPMuxImage,
    ) -> WebPMuxError;
    fn MuxImageCount(wpi_list: *const WebPMuxImage, id: WebPChunkId) -> libc::c_int;
    fn WebPSafeMalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

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
unsafe extern "C" fn WebPMuxNew() -> *mut WebPMux {
    return WebPNewInternal(0x109 as libc::c_int);
}
#[inline]
unsafe extern "C" fn ChunkDiskSize(mut chunk: *const WebPChunk) -> size_t {
    let data_size: size_t = (*chunk).data_.size;
    return SizeWithPadding(data_size);
}
#[inline]
unsafe extern "C" fn SizeWithPadding(mut chunk_size: size_t) -> size_t {
    return (8 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            chunk_size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                & !(1 as libc::c_uint) as libc::c_ulong,
        );
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
unsafe extern "C" fn GetLE32(data: *const uint8_t) -> uint32_t {
    return GetLE16(data) as libc::c_uint
        | (GetLE16(data.offset(2 as libc::c_int as isize)) as uint32_t)
            << 16 as libc::c_int;
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
unsafe extern "C" fn MuxGet(
    mux: *const WebPMux,
    mut idx: CHUNK_INDEX,
    mut nth: uint32_t,
    data: *mut WebPData,
) -> WebPMuxError {
    WebPDataInit(data);
    if idx as libc::c_uint == IDX_VP8X as libc::c_int as libc::c_uint {
        let chunk: *const WebPChunk = ChunkSearchList(
            (*mux).vp8x_,
            nth,
            kChunks[IDX_VP8X as libc::c_int as usize].tag,
        );
        if !chunk.is_null() {
            *data = (*chunk).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as libc::c_uint == IDX_ICCP as libc::c_int as libc::c_uint {
        let chunk_0: *const WebPChunk = ChunkSearchList(
            (*mux).iccp_,
            nth,
            kChunks[IDX_ICCP as libc::c_int as usize].tag,
        );
        if !chunk_0.is_null() {
            *data = (*chunk_0).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as libc::c_uint == IDX_ANIM as libc::c_int as libc::c_uint {
        let chunk_1: *const WebPChunk = ChunkSearchList(
            (*mux).anim_,
            nth,
            kChunks[IDX_ANIM as libc::c_int as usize].tag,
        );
        if !chunk_1.is_null() {
            *data = (*chunk_1).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as libc::c_uint == IDX_EXIF as libc::c_int as libc::c_uint {
        let chunk_2: *const WebPChunk = ChunkSearchList(
            (*mux).exif_,
            nth,
            kChunks[IDX_EXIF as libc::c_int as usize].tag,
        );
        if !chunk_2.is_null() {
            *data = (*chunk_2).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    if idx as libc::c_uint == IDX_XMP as libc::c_int as libc::c_uint {
        let chunk_3: *const WebPChunk = ChunkSearchList(
            (*mux).xmp_,
            nth,
            kChunks[IDX_XMP as libc::c_int as usize].tag,
        );
        if !chunk_3.is_null() {
            *data = (*chunk_3).data_;
            return WEBP_MUX_OK;
        } else {
            return WEBP_MUX_NOT_FOUND
        }
    }
    return WEBP_MUX_NOT_FOUND;
}
unsafe extern "C" fn ChunkVerifyAndAssign(
    mut chunk: *mut WebPChunk,
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut riff_size: size_t,
    mut copy_data: libc::c_int,
) -> WebPMuxError {
    let mut chunk_size: uint32_t = 0;
    let mut chunk_data: WebPData = WebPData {
        bytes: 0 as *const uint8_t,
        size: 0,
    };
    if data_size < 8 as libc::c_int as libc::c_ulong {
        return WEBP_MUX_NOT_ENOUGH_DATA;
    }
    chunk_size = GetLE32(data.offset(4 as libc::c_int as isize));
    if chunk_size
        > (!(0 as libc::c_uint))
            .wrapping_sub(8 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return WEBP_MUX_BAD_DATA;
    }
    let chunk_disk_size: size_t = SizeWithPadding(chunk_size as size_t);
    if chunk_disk_size > riff_size {
        return WEBP_MUX_BAD_DATA;
    }
    if chunk_disk_size > data_size {
        return WEBP_MUX_NOT_ENOUGH_DATA;
    }
    chunk_data.bytes = data.offset(8 as libc::c_int as isize);
    chunk_data.size = chunk_size as size_t;
    return ChunkAssignData(
        chunk,
        &mut chunk_data,
        copy_data,
        GetLE32(data.offset(0 as libc::c_int as isize)),
    );
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageFinalize(wpi: *mut WebPMuxImage) -> libc::c_int {
    let img: *const WebPChunk = (*wpi).img_;
    let image: *const WebPData = &(*img).data_;
    let is_lossless: libc::c_int = ((*img).tag_
        == kChunks[IDX_VP8L as libc::c_int as usize].tag) as libc::c_int;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut vp8l_has_alpha: libc::c_int = 0 as libc::c_int;
    let ok: libc::c_int = if is_lossless != 0 {
        VP8LGetInfo((*image).bytes, (*image).size, &mut w, &mut h, &mut vp8l_has_alpha)
    } else {
        VP8GetInfo((*image).bytes, (*image).size, (*image).size, &mut w, &mut h)
    };
    if ok != 0 {
        if is_lossless != 0 && !((*wpi).alpha_).is_null() {
            ChunkDelete((*wpi).alpha_);
            (*wpi).alpha_ = 0 as *mut WebPChunk;
        }
        (*wpi).width_ = w;
        (*wpi).height_ = h;
        (*wpi)
            .has_alpha_ = (vp8l_has_alpha != 0 || !((*wpi).alpha_).is_null())
            as libc::c_int;
    }
    return ok;
}
unsafe extern "C" fn MuxImageParse(
    chunk: *const WebPChunk,
    mut copy_data: libc::c_int,
    wpi: *mut WebPMuxImage,
) -> libc::c_int {
    let mut current_block: u64;
    let mut bytes: *const uint8_t = (*chunk).data_.bytes;
    let mut size: size_t = (*chunk).data_.size;
    let last: *const uint8_t = if bytes.is_null() {
        0 as *const uint8_t
    } else {
        bytes.offset(size as isize)
    };
    let mut subchunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: 0 as *const uint8_t,
            size: 0,
        },
        next_: 0 as *mut WebPChunk,
    };
    let mut subchunk_size: size_t = 0;
    let mut unknown_chunk_list: *mut *mut WebPChunk = &mut (*wpi).unknown_;
    ChunkInit(&mut subchunk);
    let hdr_size: size_t = 16 as libc::c_int as size_t;
    let temp: WebPData = {
        let mut init = WebPData {
            bytes: bytes,
            size: hdr_size,
        };
        init
    };
    if !(size < hdr_size) {
        if !(ChunkAssignData(&mut subchunk, &temp, copy_data, (*chunk).tag_)
            as libc::c_int != WEBP_MUX_OK as libc::c_int)
        {
            if !(ChunkSetHead(&mut subchunk, &mut (*wpi).header_) as libc::c_int
                != WEBP_MUX_OK as libc::c_int)
            {
                (*wpi).is_partial_ = 1 as libc::c_int;
                subchunk_size = (ChunkDiskSize(&mut subchunk))
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong);
                bytes = bytes.offset(subchunk_size as isize);
                size = (size as libc::c_ulong).wrapping_sub(subchunk_size) as size_t
                    as size_t;
                loop {
                    if !(bytes != last) {
                        current_block = 4488286894823169796;
                        break;
                    }
                    ChunkInit(&mut subchunk);
                    if ChunkVerifyAndAssign(&mut subchunk, bytes, size, size, copy_data)
                        as libc::c_int != WEBP_MUX_OK as libc::c_int
                    {
                        current_block = 3312907982919250996;
                        break;
                    }
                    match ChunkGetIdFromTag(subchunk.tag_) as libc::c_uint {
                        5 => {
                            if !((*wpi).alpha_).is_null() {
                                current_block = 3312907982919250996;
                                break;
                            }
                            if ChunkSetHead(&mut subchunk, &mut (*wpi).alpha_)
                                as libc::c_int != WEBP_MUX_OK as libc::c_int
                            {
                                current_block = 3312907982919250996;
                                break;
                            }
                            (*wpi).is_partial_ = 1 as libc::c_int;
                        }
                        6 => {
                            if !((*wpi).img_).is_null() {
                                current_block = 3312907982919250996;
                                break;
                            }
                            if ChunkSetHead(&mut subchunk, &mut (*wpi).img_)
                                as libc::c_int != WEBP_MUX_OK as libc::c_int
                            {
                                current_block = 3312907982919250996;
                                break;
                            }
                            if MuxImageFinalize(wpi) == 0 {
                                current_block = 3312907982919250996;
                                break;
                            }
                            (*wpi).is_partial_ = 0 as libc::c_int;
                        }
                        9 => {
                            if (*wpi).is_partial_ != 0 {
                                current_block = 3312907982919250996;
                                break;
                            }
                            if ChunkAppend(&mut subchunk, &mut unknown_chunk_list)
                                as libc::c_int != WEBP_MUX_OK as libc::c_int
                            {
                                current_block = 3312907982919250996;
                                break;
                            }
                        }
                        _ => {
                            current_block = 3312907982919250996;
                            break;
                        }
                    }
                    subchunk_size = ChunkDiskSize(&mut subchunk);
                    bytes = bytes.offset(subchunk_size as isize);
                    size = (size as libc::c_ulong).wrapping_sub(subchunk_size) as size_t
                        as size_t;
                }
                match current_block {
                    3312907982919250996 => {}
                    _ => {
                        if !((*wpi).is_partial_ != 0) {
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    ChunkRelease(&mut subchunk);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxCreateInternal(
    mut bitstream: *const WebPData,
    mut copy_data: libc::c_int,
    mut version: libc::c_int,
) -> *mut WebPMux {
    let mut current_block: u64;
    let mut riff_size: size_t = 0;
    let mut tag: uint32_t = 0;
    let mut end: *const uint8_t = 0 as *const uint8_t;
    let mut mux: *mut WebPMux = 0 as *mut WebPMux;
    let mut wpi: *mut WebPMuxImage = 0 as *mut WebPMuxImage;
    let mut data: *const uint8_t = 0 as *const uint8_t;
    let mut size: size_t = 0;
    let mut chunk: WebPChunk = WebPChunk {
        tag_: 0,
        owner_: 0,
        data_: WebPData {
            bytes: 0 as *const uint8_t,
            size: 0,
        },
        next_: 0 as *mut WebPChunk,
    };
    let mut chunk_list_ends: [*mut *mut WebPChunk; 11] = [
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
        0 as *mut *mut WebPChunk,
    ];
    ChunkInit(&mut chunk);
    if version >> 8 as libc::c_int != 0x109 as libc::c_int >> 8 as libc::c_int {
        return 0 as *mut WebPMux;
    }
    if bitstream.is_null() {
        return 0 as *mut WebPMux;
    }
    data = (*bitstream).bytes;
    size = (*bitstream).size;
    if data.is_null() {
        return 0 as *mut WebPMux;
    }
    if size < (12 as libc::c_int + 8 as libc::c_int) as libc::c_ulong {
        return 0 as *mut WebPMux;
    }
    if GetLE32(data.offset(0 as libc::c_int as isize))
        != ('R' as i32 | ('I' as i32) << 8 as libc::c_int
            | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
            | ('F' as i32 as uint32_t) << 24 as libc::c_int
        || GetLE32(data.offset(8 as libc::c_int as isize))
            != ('W' as i32 | ('E' as i32) << 8 as libc::c_int
                | ('B' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('P' as i32 as uint32_t) << 24 as libc::c_int
    {
        return 0 as *mut WebPMux;
    }
    mux = WebPMuxNew();
    if mux.is_null() {
        return 0 as *mut WebPMux;
    }
    tag = GetLE32(data.offset(12 as libc::c_int as isize));
    if !(tag != kChunks[IDX_VP8 as libc::c_int as usize].tag
        && tag != kChunks[IDX_VP8L as libc::c_int as usize].tag
        && tag != kChunks[IDX_VP8X as libc::c_int as usize].tag)
    {
        riff_size = GetLE32(data.offset(4 as libc::c_int as isize)) as size_t;
        if !(riff_size
            > (!(0 as libc::c_uint))
                .wrapping_sub(8 as libc::c_int as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        {
            riff_size = SizeWithPadding(riff_size);
            if !(riff_size < 8 as libc::c_int as libc::c_ulong) {
                if !(riff_size > size) {
                    if size > riff_size.wrapping_add(8 as libc::c_int as libc::c_ulong) {
                        size = riff_size.wrapping_add(8 as libc::c_int as libc::c_ulong);
                    }
                    end = data.offset(size as isize);
                    data = data.offset(12 as libc::c_int as isize);
                    size = (size as libc::c_ulong)
                        .wrapping_sub(12 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    wpi = WebPSafeMalloc(
                        1 as libc::c_ulonglong as uint64_t,
                        ::core::mem::size_of::<WebPMuxImage>() as libc::c_ulong,
                    ) as *mut WebPMuxImage;
                    if !wpi.is_null() {
                        MuxImageInit(wpi);
                        loop {
                            if !(data != end) {
                                current_block = 16415152177862271243;
                                break;
                            }
                            let mut data_size: size_t = 0;
                            let mut id: WebPChunkId = WEBP_CHUNK_VP8X;
                            if ChunkVerifyAndAssign(
                                &mut chunk,
                                data,
                                size,
                                riff_size,
                                copy_data,
                            ) as libc::c_int != WEBP_MUX_OK as libc::c_int
                            {
                                current_block = 18302400449631267483;
                                break;
                            }
                            data_size = ChunkDiskSize(&mut chunk);
                            id = ChunkGetIdFromTag(chunk.tag_);
                            match id as libc::c_uint {
                                5 => {
                                    if !((*wpi).alpha_).is_null() {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    if ChunkSetHead(&mut chunk, &mut (*wpi).alpha_)
                                        as libc::c_int != WEBP_MUX_OK as libc::c_int
                                    {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    (*wpi).is_partial_ = 1 as libc::c_int;
                                    current_block = 7018308795614528254;
                                }
                                6 => {
                                    if ChunkSetHead(&mut chunk, &mut (*wpi).img_) as libc::c_int
                                        != WEBP_MUX_OK as libc::c_int
                                    {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    if MuxImageFinalize(wpi) == 0 {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    (*wpi).is_partial_ = 0 as libc::c_int;
                                    current_block = 2447982090133940422;
                                }
                                3 => {
                                    if (*wpi).is_partial_ != 0 {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    if MuxImageParse(&mut chunk, copy_data, wpi) == 0 {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    ChunkRelease(&mut chunk);
                                    current_block = 2447982090133940422;
                                }
                                _ => {
                                    if (*wpi).is_partial_ != 0 {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    if (chunk_list_ends[id as usize]).is_null() {
                                        chunk_list_ends[id
                                            as usize] = MuxGetChunkListFromId(mux, id);
                                    }
                                    if ChunkAppend(
                                        &mut chunk,
                                        &mut *chunk_list_ends.as_mut_ptr().offset(id as isize),
                                    ) as libc::c_int != WEBP_MUX_OK as libc::c_int
                                    {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    if id as libc::c_uint
                                        == WEBP_CHUNK_VP8X as libc::c_int as libc::c_uint
                                    {
                                        if data_size
                                            < (8 as libc::c_int + 10 as libc::c_int) as libc::c_ulong
                                        {
                                            current_block = 18302400449631267483;
                                            break;
                                        }
                                        (*mux)
                                            .canvas_width_ = GetLE24(
                                            data.offset(12 as libc::c_int as isize),
                                        ) + 1 as libc::c_int;
                                        (*mux)
                                            .canvas_height_ = GetLE24(
                                            data.offset(15 as libc::c_int as isize),
                                        ) + 1 as libc::c_int;
                                        current_block = 7018308795614528254;
                                    } else {
                                        current_block = 7018308795614528254;
                                    }
                                }
                            }
                            match current_block {
                                2447982090133940422 => {
                                    if MuxImagePush(wpi, &mut (*mux).images_) as libc::c_int
                                        != WEBP_MUX_OK as libc::c_int
                                    {
                                        current_block = 18302400449631267483;
                                        break;
                                    }
                                    MuxImageInit(wpi);
                                }
                                _ => {}
                            }
                            data = data.offset(data_size as isize);
                            size = (size as libc::c_ulong).wrapping_sub(data_size)
                                as size_t as size_t;
                            ChunkInit(&mut chunk);
                        }
                        match current_block {
                            18302400449631267483 => {}
                            _ => {
                                if !((*wpi).is_partial_ != 0) {
                                    if !(MuxValidate(mux) as libc::c_int
                                        != WEBP_MUX_OK as libc::c_int)
                                    {
                                        MuxImageDelete(wpi);
                                        return mux;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ChunkRelease(&mut chunk);
    MuxImageDelete(wpi);
    WebPMuxDelete(mux);
    return 0 as *mut WebPMux;
}
unsafe extern "C" fn ValidateForSingleImage(mux: *const WebPMux) -> WebPMuxError {
    let num_images: libc::c_int = MuxImageCount((*mux).images_, WEBP_CHUNK_IMAGE);
    let num_frames: libc::c_int = MuxImageCount((*mux).images_, WEBP_CHUNK_ANMF);
    if num_images == 0 as libc::c_int {
        return WEBP_MUX_NOT_FOUND
    } else if num_images == 1 as libc::c_int && num_frames == 0 as libc::c_int {
        return WEBP_MUX_OK
    } else {
        return WEBP_MUX_INVALID_ARGUMENT
    };
}
unsafe extern "C" fn MuxGetCanvasInfo(
    mux: *const WebPMux,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut flags: *mut uint32_t,
) -> WebPMuxError {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut f: uint32_t = 0 as libc::c_int as uint32_t;
    let mut data: WebPData = WebPData {
        bytes: 0 as *const uint8_t,
        size: 0,
    };
    if MuxGet(mux, IDX_VP8X, 1 as libc::c_int as uint32_t, &mut data) as libc::c_int
        == WEBP_MUX_OK as libc::c_int
    {
        if data.size < 10 as libc::c_int as libc::c_ulong {
            return WEBP_MUX_BAD_DATA;
        }
        f = GetLE32((data.bytes).offset(0 as libc::c_int as isize));
        w = GetLE24((data.bytes).offset(4 as libc::c_int as isize)) + 1 as libc::c_int;
        h = GetLE24((data.bytes).offset(7 as libc::c_int as isize)) + 1 as libc::c_int;
    } else {
        let wpi: *const WebPMuxImage = (*mux).images_;
        w = (*mux).canvas_width_;
        h = (*mux).canvas_height_;
        if w == 0 as libc::c_int && h == 0 as libc::c_int
            && ValidateForSingleImage(mux) as libc::c_int == WEBP_MUX_OK as libc::c_int
        {
            w = (*wpi).width_;
            h = (*wpi).height_;
        }
        if !wpi.is_null() {
            if (*wpi).has_alpha_ != 0 {
                f |= ALPHA_FLAG as libc::c_int as libc::c_uint;
            }
        }
    }
    if (w as libc::c_ulong).wrapping_mul(h as uint64_t) as libc::c_ulonglong
        >= (1 as libc::c_ulonglong) << 32 as libc::c_int
    {
        return WEBP_MUX_BAD_DATA;
    }
    if !width.is_null() {
        *width = w;
    }
    if !height.is_null() {
        *height = h;
    }
    if !flags.is_null() {
        *flags = f;
    }
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetCanvasSize(
    mut mux: *const WebPMux,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) -> WebPMuxError {
    if mux.is_null() || width.is_null() || height.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxGetCanvasInfo(mux, width, height, 0 as *mut uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetFeatures(
    mut mux: *const WebPMux,
    mut flags: *mut uint32_t,
) -> WebPMuxError {
    if mux.is_null() || flags.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return MuxGetCanvasInfo(mux, 0 as *mut libc::c_int, 0 as *mut libc::c_int, flags);
}
unsafe extern "C" fn EmitVP8XChunk(
    dst: *mut uint8_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut flags: uint32_t,
) -> *mut uint8_t {
    let vp8x_size: size_t = (8 as libc::c_int + 10 as libc::c_int) as size_t;
    PutLE32(
        dst,
        ('V' as i32 | ('P' as i32) << 8 as libc::c_int
            | ('8' as i32) << 16 as libc::c_int) as libc::c_uint
            | ('X' as i32 as uint32_t) << 24 as libc::c_int,
    );
    PutLE32(dst.offset(4 as libc::c_int as isize), 10 as libc::c_int as uint32_t);
    PutLE32(dst.offset(8 as libc::c_int as isize), flags);
    PutLE24(
        dst.offset(8 as libc::c_int as isize).offset(4 as libc::c_int as isize),
        width - 1 as libc::c_int,
    );
    PutLE24(
        dst.offset(8 as libc::c_int as isize).offset(7 as libc::c_int as isize),
        height - 1 as libc::c_int,
    );
    return dst.offset(vp8x_size as isize);
}
unsafe extern "C" fn SynthesizeBitstream(
    wpi: *const WebPMuxImage,
    bitstream: *mut WebPData,
) -> WebPMuxError {
    let mut dst: *mut uint8_t = 0 as *mut uint8_t;
    let need_vp8x: libc::c_int = ((*wpi).alpha_
        != 0 as *mut libc::c_void as *mut WebPChunk) as libc::c_int;
    let vp8x_size: size_t = (if need_vp8x != 0 {
        8 as libc::c_int + 10 as libc::c_int
    } else {
        0 as libc::c_int
    }) as size_t;
    let alpha_size: size_t = if need_vp8x != 0 {
        ChunkDiskSize((*wpi).alpha_)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let size: size_t = (12 as libc::c_int as libc::c_ulong)
        .wrapping_add(vp8x_size)
        .wrapping_add(alpha_size)
        .wrapping_add(ChunkDiskSize((*wpi).img_));
    let data: *mut uint8_t = WebPSafeMalloc(1 as libc::c_ulonglong as uint64_t, size)
        as *mut uint8_t;
    if data.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    dst = MuxEmitRiffHeader(data, size);
    if need_vp8x != 0 {
        dst = EmitVP8XChunk(
            dst,
            (*wpi).width_,
            (*wpi).height_,
            ALPHA_FLAG as libc::c_int as uint32_t,
        );
        dst = ChunkListEmit((*wpi).alpha_, dst);
    }
    dst = ChunkListEmit((*wpi).img_, dst);
    (*bitstream).bytes = data;
    (*bitstream).size = size;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetChunk(
    mut mux: *const WebPMux,
    mut fourcc: *const libc::c_char,
    mut chunk_data: *mut WebPData,
) -> WebPMuxError {
    let mut idx: CHUNK_INDEX = IDX_VP8X;
    if mux.is_null() || fourcc.is_null() || chunk_data.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    idx = ChunkGetIndexFromFourCC(fourcc);
    if IsWPI(kChunks[idx as usize].id) != 0 {
        return WEBP_MUX_INVALID_ARGUMENT
    } else if idx as libc::c_uint != IDX_UNKNOWN as libc::c_int as libc::c_uint {
        return MuxGet(mux, idx, 1 as libc::c_int as uint32_t, chunk_data)
    } else {
        let chunk: *const WebPChunk = ChunkSearchList(
            (*mux).unknown_,
            1 as libc::c_int as uint32_t,
            ChunkGetTagFromFourCC(fourcc),
        );
        if chunk.is_null() {
            return WEBP_MUX_NOT_FOUND;
        }
        *chunk_data = (*chunk).data_;
        return WEBP_MUX_OK;
    };
}
unsafe extern "C" fn MuxGetImageInternal(
    wpi: *const WebPMuxImage,
    info: *mut WebPMuxFrameInfo,
) -> WebPMuxError {
    (*info).x_offset = 0 as libc::c_int;
    (*info).y_offset = 0 as libc::c_int;
    (*info).duration = 1 as libc::c_int;
    (*info).dispose_method = WEBP_MUX_DISPOSE_NONE;
    (*info).blend_method = WEBP_MUX_BLEND;
    (*info).id = ChunkGetIdFromTag((*(*wpi).img_).tag_);
    return SynthesizeBitstream(wpi, &mut (*info).bitstream);
}
unsafe extern "C" fn MuxGetFrameInternal(
    wpi: *const WebPMuxImage,
    frame: *mut WebPMuxFrameInfo,
) -> WebPMuxError {
    let is_frame: libc::c_int = ((*(*wpi).header_).tag_
        == kChunks[IDX_ANMF as libc::c_int as usize].tag) as libc::c_int;
    let mut frame_data: *const WebPData = 0 as *const WebPData;
    if is_frame == 0 {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    frame_data = &mut (*(*wpi).header_).data_;
    if (*frame_data).size
        < kChunks[IDX_ANMF as libc::c_int as usize].size as libc::c_ulong
    {
        return WEBP_MUX_BAD_DATA;
    }
    (*frame)
        .x_offset = 2 as libc::c_int
        * GetLE24(((*frame_data).bytes).offset(0 as libc::c_int as isize));
    (*frame)
        .y_offset = 2 as libc::c_int
        * GetLE24(((*frame_data).bytes).offset(3 as libc::c_int as isize));
    let bits: uint8_t = *((*frame_data).bytes).offset(15 as libc::c_int as isize);
    (*frame)
        .duration = GetLE24(((*frame_data).bytes).offset(12 as libc::c_int as isize));
    (*frame)
        .dispose_method = (if bits as libc::c_int & 1 as libc::c_int != 0 {
        WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int
    } else {
        WEBP_MUX_DISPOSE_NONE as libc::c_int
    }) as WebPMuxAnimDispose;
    (*frame)
        .blend_method = (if bits as libc::c_int & 2 as libc::c_int != 0 {
        WEBP_MUX_NO_BLEND as libc::c_int
    } else {
        WEBP_MUX_BLEND as libc::c_int
    }) as WebPMuxAnimBlend;
    (*frame).id = ChunkGetIdFromTag((*(*wpi).header_).tag_);
    return SynthesizeBitstream(wpi, &mut (*frame).bitstream);
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetFrame(
    mut mux: *const WebPMux,
    mut nth: uint32_t,
    mut frame: *mut WebPMuxFrameInfo,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    let mut wpi: *mut WebPMuxImage = 0 as *mut WebPMuxImage;
    if mux.is_null() || frame.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxImageGetNth(
        &(*mux).images_ as *const *mut WebPMuxImage as *mut *const WebPMuxImage,
        nth,
        &mut wpi,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if ((*wpi).header_).is_null() {
        return MuxGetImageInternal(wpi, frame)
    } else {
        return MuxGetFrameInternal(wpi, frame)
    };
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxGetAnimationParams(
    mut mux: *const WebPMux,
    mut params: *mut WebPMuxAnimParams,
) -> WebPMuxError {
    let mut anim: WebPData = WebPData {
        bytes: 0 as *const uint8_t,
        size: 0,
    };
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() || params.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = MuxGet(mux, IDX_ANIM, 1 as libc::c_int as uint32_t, &mut anim);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if anim.size < kChunks[WEBP_CHUNK_ANIM as libc::c_int as usize].size as libc::c_ulong
    {
        return WEBP_MUX_BAD_DATA;
    }
    (*params).bgcolor = GetLE32(anim.bytes);
    (*params).loop_count = GetLE16((anim.bytes).offset(4 as libc::c_int as isize));
    return WEBP_MUX_OK;
}
unsafe extern "C" fn ChunkGetIndexFromId(mut id: WebPChunkId) -> CHUNK_INDEX {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while kChunks[i as usize].id as libc::c_uint
        != WEBP_CHUNK_NIL as libc::c_int as libc::c_uint
    {
        if id as libc::c_uint == kChunks[i as usize].id as libc::c_uint {
            return i as CHUNK_INDEX;
        }
        i += 1;
        i;
    }
    return IDX_NIL;
}
unsafe extern "C" fn CountChunks(
    chunk_list: *const WebPChunk,
    mut tag: uint32_t,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut current: *const WebPChunk = 0 as *const WebPChunk;
    current = chunk_list;
    while !current.is_null() {
        if tag == 0 as libc::c_uint || (*current).tag_ == tag {
            count += 1;
            count;
        }
        current = (*current).next_;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn WebPMuxNumChunks(
    mut mux: *const WebPMux,
    mut id: WebPChunkId,
    mut num_elements: *mut libc::c_int,
) -> WebPMuxError {
    if mux.is_null() || num_elements.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if IsWPI(id) != 0 {
        *num_elements = MuxImageCount((*mux).images_, id);
    } else {
        let mut chunk_list: *const *mut WebPChunk = MuxGetChunkListFromId(mux, id);
        let idx: CHUNK_INDEX = ChunkGetIndexFromId(id);
        *num_elements = CountChunks(*chunk_list, kChunks[idx as usize].tag);
    }
    return WEBP_MUX_OK;
}
