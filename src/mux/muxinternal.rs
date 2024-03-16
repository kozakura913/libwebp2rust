use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPMalloc(size: size_t) -> *mut libc::c_void;
    fn WebPFree(ptr: *mut libc::c_void);
    fn WebPMuxGetFeatures(mux: *const WebPMux, flags: *mut uint32_t) -> WebPMuxError;
    fn WebPMuxNumChunks(
        mux: *const WebPMux,
        id: WebPChunkId,
        num_elements: *mut libc::c_int,
    ) -> WebPMuxError;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPData {
    pub bytes: *const uint8_t,
    pub size: size_t,
}
pub type WebPFeatureFlags = libc::c_uint;
pub const ALL_VALID_FLAGS: WebPFeatureFlags = 62;
pub const ICCP_FLAG: WebPFeatureFlags = 32;
pub const ALPHA_FLAG: WebPFeatureFlags = 16;
pub const EXIF_FLAG: WebPFeatureFlags = 8;
pub const XMP_FLAG: WebPFeatureFlags = 4;
pub const ANIMATION_FLAG: WebPFeatureFlags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMux {
    pub images_: *mut WebPMuxImage,
    pub iccp_: *mut WebPChunk,
    pub exif_: *mut WebPChunk,
    pub xmp_: *mut WebPChunk,
    pub anim_: *mut WebPChunk,
    pub vp8x_: *mut WebPChunk,
    pub unknown_: *mut WebPChunk,
    pub canvas_width_: libc::c_int,
    pub canvas_height_: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPChunk {
    pub tag_: uint32_t,
    pub owner_: libc::c_int,
    pub data_: WebPData,
    pub next_: *mut WebPChunk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPMuxImage {
    pub header_: *mut WebPChunk,
    pub alpha_: *mut WebPChunk,
    pub img_: *mut WebPChunk,
    pub unknown_: *mut WebPChunk,
    pub width_: libc::c_int,
    pub height_: libc::c_int,
    pub has_alpha_: libc::c_int,
    pub is_partial_: libc::c_int,
    pub next_: *mut WebPMuxImage,
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
pub type CHUNK_INDEX = libc::c_uint;
pub const IDX_LAST_CHUNK: CHUNK_INDEX = 11;
pub const IDX_NIL: CHUNK_INDEX = 10;
pub const IDX_UNKNOWN: CHUNK_INDEX = 9;
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
unsafe extern "C" fn WebPDataCopy(
    mut src: *const WebPData,
    mut dst: *mut WebPData,
) -> libc::c_int {
    if src.is_null() || dst.is_null() {
        return 0 as libc::c_int;
    }
    WebPDataInit(dst);
    if !((*src).bytes).is_null() && (*src).size != 0 as libc::c_int as libc::c_ulong {
        (*dst).bytes = WebPMalloc((*src).size) as *mut uint8_t;
        if ((*dst).bytes).is_null() {
            return 0 as libc::c_int;
        }
        memcpy(
            (*dst).bytes as *mut libc::c_void,
            (*src).bytes as *const libc::c_void,
            (*src).size,
        );
        (*dst).size = (*src).size;
    }
    return 1 as libc::c_int;
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
unsafe extern "C" fn PutLE32(data: *mut uint8_t, mut val: uint32_t) {
    PutLE16(data, (val & 0xffff as libc::c_int as libc::c_uint) as libc::c_int);
    PutLE16(
        data.offset(2 as libc::c_int as isize),
        (val >> 16 as libc::c_int) as libc::c_int,
    );
}
#[no_mangle]
pub static mut kChunks: [ChunkInfo; 11] = [
    {
        let mut init = ChunkInfo {
            tag: ('V' as i32 | ('P' as i32) << 8 as libc::c_int
                | ('8' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('X' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_VP8X,
            size: 10 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('I' as i32 | ('C' as i32) << 8 as libc::c_int
                | ('C' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('P' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_ICCP,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('A' as i32 | ('N' as i32) << 8 as libc::c_int
                | ('I' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('M' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_ANIM,
            size: 6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('A' as i32 | ('N' as i32) << 8 as libc::c_int
                | ('M' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('F' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_ANMF,
            size: 16 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('A' as i32 | ('L' as i32) << 8 as libc::c_int
                | ('P' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('H' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_ALPHA,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('V' as i32 | ('P' as i32) << 8 as libc::c_int
                | ('8' as i32) << 16 as libc::c_int) as libc::c_uint
                | (' ' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_IMAGE,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('V' as i32 | ('P' as i32) << 8 as libc::c_int
                | ('8' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('L' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_IMAGE,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('E' as i32 | ('X' as i32) << 8 as libc::c_int
                | ('I' as i32) << 16 as libc::c_int) as libc::c_uint
                | ('F' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_EXIF,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: ('X' as i32 | ('M' as i32) << 8 as libc::c_int
                | ('P' as i32) << 16 as libc::c_int) as libc::c_uint
                | (' ' as i32 as uint32_t) << 24 as libc::c_int,
            id: WEBP_CHUNK_XMP,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: 0 as libc::c_uint,
            id: WEBP_CHUNK_UNKNOWN,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
    {
        let mut init = ChunkInfo {
            tag: 0 as libc::c_uint,
            id: WEBP_CHUNK_NIL,
            size: -(1 as libc::c_int) as uint32_t,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn WebPGetMuxVersion() -> libc::c_int {
    return (1 as libc::c_int) << 16 as libc::c_int
        | (3 as libc::c_int) << 8 as libc::c_int | 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkInit(chunk: *mut WebPChunk) {
    memset(
        chunk as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPChunk>() as libc::c_ulong,
    );
    (*chunk).tag_ = 0 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkRelease(chunk: *mut WebPChunk) -> *mut WebPChunk {
    let mut next: *mut WebPChunk = 0 as *mut WebPChunk;
    if chunk.is_null() {
        return 0 as *mut WebPChunk;
    }
    if (*chunk).owner_ != 0 {
        WebPDataClear(&mut (*chunk).data_);
    }
    next = (*chunk).next_;
    ChunkInit(chunk);
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetIndexFromTag(mut tag: uint32_t) -> CHUNK_INDEX {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while kChunks[i as usize].tag != 0 as libc::c_uint {
        if tag == kChunks[i as usize].tag {
            return i as CHUNK_INDEX;
        }
        i += 1;
        i;
    }
    return IDX_UNKNOWN;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetIdFromTag(mut tag: uint32_t) -> WebPChunkId {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while kChunks[i as usize].tag != 0 as libc::c_uint {
        if tag == kChunks[i as usize].tag {
            return kChunks[i as usize].id;
        }
        i += 1;
        i;
    }
    return WEBP_CHUNK_UNKNOWN;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetTagFromFourCC(
    mut fourcc: *const libc::c_char,
) -> uint32_t {
    return (*fourcc.offset(0 as libc::c_int as isize) as libc::c_int
        | (*fourcc.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | (*fourcc.offset(2 as libc::c_int as isize) as libc::c_int)
            << 16 as libc::c_int) as libc::c_uint
        | (*fourcc.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkGetIndexFromFourCC(
    mut fourcc: *const libc::c_char,
) -> CHUNK_INDEX {
    let tag: uint32_t = ChunkGetTagFromFourCC(fourcc);
    return ChunkGetIndexFromTag(tag);
}
unsafe extern "C" fn ChunkSearchNextInList(
    mut chunk: *mut WebPChunk,
    mut tag: uint32_t,
) -> *mut WebPChunk {
    while !chunk.is_null() && (*chunk).tag_ != tag {
        chunk = (*chunk).next_;
    }
    return chunk;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkSearchList(
    mut first: *mut WebPChunk,
    mut nth: uint32_t,
    mut tag: uint32_t,
) -> *mut WebPChunk {
    let mut iter: uint32_t = nth;
    first = ChunkSearchNextInList(first, tag);
    if first.is_null() {
        return 0 as *mut WebPChunk;
    }
    loop {
        iter = iter.wrapping_sub(1);
        if !(iter != 0 as libc::c_int as libc::c_uint) {
            break;
        }
        let mut next_chunk: *mut WebPChunk = ChunkSearchNextInList((*first).next_, tag);
        if next_chunk.is_null() {
            break;
        }
        first = next_chunk;
    }
    return if nth > 0 as libc::c_int as libc::c_uint
        && iter > 0 as libc::c_int as libc::c_uint
    {
        0 as *mut WebPChunk
    } else {
        first
    };
}
#[no_mangle]
pub unsafe extern "C" fn ChunkAssignData(
    mut chunk: *mut WebPChunk,
    data: *const WebPData,
    mut copy_data: libc::c_int,
    mut tag: uint32_t,
) -> WebPMuxError {
    if tag == kChunks[IDX_VP8X as libc::c_int as usize].tag
        || tag == kChunks[IDX_ANIM as libc::c_int as usize].tag
    {
        copy_data = 1 as libc::c_int;
    }
    ChunkRelease(chunk);
    if !data.is_null() {
        if copy_data != 0 {
            if WebPDataCopy(data, &mut (*chunk).data_) == 0 {
                return WEBP_MUX_MEMORY_ERROR;
            }
            (*chunk).owner_ = 1 as libc::c_int;
        } else {
            (*chunk).data_ = *data;
        }
    }
    (*chunk).tag_ = tag;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkSetHead(
    chunk: *mut WebPChunk,
    chunk_list: *mut *mut WebPChunk,
) -> WebPMuxError {
    let mut new_chunk: *mut WebPChunk = 0 as *mut WebPChunk;
    if !(*chunk_list).is_null() {
        return WEBP_MUX_NOT_FOUND;
    }
    new_chunk = WebPSafeMalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<WebPChunk>() as libc::c_ulong,
    ) as *mut WebPChunk;
    if new_chunk.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    *new_chunk = *chunk;
    (*chunk).owner_ = 0 as libc::c_int;
    (*new_chunk).next_ = 0 as *mut WebPChunk;
    *chunk_list = new_chunk;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkAppend(
    chunk: *mut WebPChunk,
    chunk_list: *mut *mut *mut WebPChunk,
) -> WebPMuxError {
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if (**chunk_list).is_null() {
        err = ChunkSetHead(chunk, *chunk_list);
    } else {
        let mut last_chunk: *mut WebPChunk = **chunk_list;
        while !((*last_chunk).next_).is_null() {
            last_chunk = (*last_chunk).next_;
        }
        err = ChunkSetHead(chunk, &mut (*last_chunk).next_);
        if err as libc::c_int == WEBP_MUX_OK as libc::c_int {
            *chunk_list = &mut (*last_chunk).next_;
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkDelete(chunk: *mut WebPChunk) -> *mut WebPChunk {
    let next: *mut WebPChunk = ChunkRelease(chunk);
    WebPSafeFree(chunk as *mut libc::c_void);
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkListDelete(chunk_list: *mut *mut WebPChunk) {
    while !(*chunk_list).is_null() {
        *chunk_list = ChunkDelete(*chunk_list);
    }
}
unsafe extern "C" fn ChunkEmit(
    chunk: *const WebPChunk,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    let chunk_size: size_t = (*chunk).data_.size;
    PutLE32(dst.offset(0 as libc::c_int as isize), (*chunk).tag_);
    PutLE32(dst.offset(4 as libc::c_int as isize), chunk_size as uint32_t);
    memcpy(
        dst.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        (*chunk).data_.bytes as *const libc::c_void,
        chunk_size,
    );
    if chunk_size & 1 as libc::c_int as libc::c_ulong != 0 {
        *dst
            .offset(
                (8 as libc::c_int as libc::c_ulong).wrapping_add(chunk_size) as isize,
            ) = 0 as libc::c_int as uint8_t;
    }
    return dst.offset(ChunkDiskSize(chunk) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ChunkListEmit(
    mut chunk_list: *const WebPChunk,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    while !chunk_list.is_null() {
        dst = ChunkEmit(chunk_list, dst);
        chunk_list = (*chunk_list).next_;
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn ChunkListDiskSize(mut chunk_list: *const WebPChunk) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    while !chunk_list.is_null() {
        size = (size as libc::c_ulong).wrapping_add(ChunkDiskSize(chunk_list)) as size_t
            as size_t;
        chunk_list = (*chunk_list).next_;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageInit(wpi: *mut WebPMuxImage) {
    memset(
        wpi as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPMuxImage>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageRelease(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage {
    let mut next: *mut WebPMuxImage = 0 as *mut WebPMuxImage;
    if wpi.is_null() {
        return 0 as *mut WebPMuxImage;
    }
    ChunkListDelete(&mut (*wpi).header_);
    ChunkListDelete(&mut (*wpi).alpha_);
    ChunkListDelete(&mut (*wpi).img_);
    ChunkListDelete(&mut (*wpi).unknown_);
    next = (*wpi).next_;
    MuxImageInit(wpi);
    return next;
}
unsafe extern "C" fn GetChunkListFromId(
    wpi: *const WebPMuxImage,
    mut id: WebPChunkId,
) -> *mut *mut WebPChunk {
    match id as libc::c_uint {
        3 => return &(*wpi).header_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        5 => return &(*wpi).alpha_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        6 => return &(*wpi).img_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        _ => return 0 as *mut *mut WebPChunk,
    };
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageCount(
    mut wpi_list: *const WebPMuxImage,
    mut id: WebPChunkId,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut current: *const WebPMuxImage = 0 as *const WebPMuxImage;
    current = wpi_list;
    while !current.is_null() {
        if id as libc::c_uint == WEBP_CHUNK_NIL as libc::c_int as libc::c_uint {
            count += 1;
            count;
        } else {
            let wpi_chunk: *const WebPChunk = *GetChunkListFromId(current, id);
            if !wpi_chunk.is_null() {
                let wpi_chunk_id: WebPChunkId = ChunkGetIdFromTag((*wpi_chunk).tag_);
                if wpi_chunk_id as libc::c_uint == id as libc::c_uint {
                    count += 1;
                    count;
                }
            }
        }
        current = (*current).next_;
    }
    return count;
}
unsafe extern "C" fn SearchImageToGetOrDelete(
    mut wpi_list: *mut *mut WebPMuxImage,
    mut nth: uint32_t,
    location: *mut *mut *mut WebPMuxImage,
) -> libc::c_int {
    let mut count: uint32_t = 0 as libc::c_int as uint32_t;
    *location = wpi_list;
    if nth == 0 as libc::c_int as libc::c_uint {
        nth = MuxImageCount(*wpi_list, WEBP_CHUNK_NIL) as uint32_t;
        if nth == 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
    }
    while !(*wpi_list).is_null() {
        let cur_wpi: *mut WebPMuxImage = *wpi_list;
        count = count.wrapping_add(1);
        count;
        if count == nth {
            return 1 as libc::c_int;
        }
        wpi_list = &mut (*cur_wpi).next_;
        *location = wpi_list;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImagePush(
    mut wpi: *const WebPMuxImage,
    mut wpi_list: *mut *mut WebPMuxImage,
) -> WebPMuxError {
    let mut new_wpi: *mut WebPMuxImage = 0 as *mut WebPMuxImage;
    while !(*wpi_list).is_null() {
        let cur_wpi: *mut WebPMuxImage = *wpi_list;
        if ((*cur_wpi).next_).is_null() {
            break;
        }
        wpi_list = &mut (*cur_wpi).next_;
    }
    new_wpi = WebPSafeMalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<WebPMuxImage>() as libc::c_ulong,
    ) as *mut WebPMuxImage;
    if new_wpi.is_null() {
        return WEBP_MUX_MEMORY_ERROR;
    }
    *new_wpi = *wpi;
    (*new_wpi).next_ = 0 as *mut WebPMuxImage;
    if !(*wpi_list).is_null() {
        (**wpi_list).next_ = new_wpi;
    } else {
        *wpi_list = new_wpi;
    }
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageDelete(wpi: *mut WebPMuxImage) -> *mut WebPMuxImage {
    let next: *mut WebPMuxImage = MuxImageRelease(wpi);
    WebPSafeFree(wpi as *mut libc::c_void);
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageDeleteNth(
    mut wpi_list: *mut *mut WebPMuxImage,
    mut nth: uint32_t,
) -> WebPMuxError {
    if SearchImageToGetOrDelete(wpi_list, nth, &mut wpi_list) == 0 {
        return WEBP_MUX_NOT_FOUND;
    }
    *wpi_list = MuxImageDelete(*wpi_list);
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageGetNth(
    mut wpi_list: *mut *const WebPMuxImage,
    mut nth: uint32_t,
    mut wpi: *mut *mut WebPMuxImage,
) -> WebPMuxError {
    if SearchImageToGetOrDelete(
        wpi_list as *mut *mut WebPMuxImage,
        nth,
        &mut wpi_list as *mut *mut *const WebPMuxImage as *mut *mut *mut WebPMuxImage,
    ) == 0
    {
        return WEBP_MUX_NOT_FOUND;
    }
    *wpi = *wpi_list as *mut WebPMuxImage;
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageDiskSize(wpi: *const WebPMuxImage) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    if !((*wpi).header_).is_null() {
        size = (size as libc::c_ulong).wrapping_add(ChunkDiskSize((*wpi).header_))
            as size_t as size_t;
    }
    if !((*wpi).alpha_).is_null() {
        size = (size as libc::c_ulong).wrapping_add(ChunkDiskSize((*wpi).alpha_))
            as size_t as size_t;
    }
    if !((*wpi).img_).is_null() {
        size = (size as libc::c_ulong).wrapping_add(ChunkDiskSize((*wpi).img_)) as size_t
            as size_t;
    }
    if !((*wpi).unknown_).is_null() {
        size = (size as libc::c_ulong).wrapping_add(ChunkListDiskSize((*wpi).unknown_))
            as size_t as size_t;
    }
    return size;
}
unsafe extern "C" fn ChunkEmitSpecial(
    header: *const WebPChunk,
    mut total_size: size_t,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    let header_size: size_t = (*header).data_.size;
    let offset_to_next: size_t = total_size
        .wrapping_sub(8 as libc::c_int as libc::c_ulong);
    PutLE32(dst.offset(0 as libc::c_int as isize), (*header).tag_);
    PutLE32(dst.offset(4 as libc::c_int as isize), offset_to_next as uint32_t);
    memcpy(
        dst.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        (*header).data_.bytes as *const libc::c_void,
        header_size,
    );
    if header_size & 1 as libc::c_int as libc::c_ulong != 0 {
        *dst
            .offset(
                (8 as libc::c_int as libc::c_ulong).wrapping_add(header_size) as isize,
            ) = 0 as libc::c_int as uint8_t;
    }
    return dst.offset(ChunkDiskSize(header) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn MuxImageEmit(
    wpi: *const WebPMuxImage,
    mut dst: *mut uint8_t,
) -> *mut uint8_t {
    if !((*wpi).header_).is_null() {
        dst = ChunkEmitSpecial((*wpi).header_, MuxImageDiskSize(wpi), dst);
    }
    if !((*wpi).alpha_).is_null() {
        dst = ChunkEmit((*wpi).alpha_, dst);
    }
    if !((*wpi).img_).is_null() {
        dst = ChunkEmit((*wpi).img_, dst);
    }
    if !((*wpi).unknown_).is_null() {
        dst = ChunkListEmit((*wpi).unknown_, dst);
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn MuxHasAlpha(mut images: *const WebPMuxImage) -> libc::c_int {
    while !images.is_null() {
        if (*images).has_alpha_ != 0 {
            return 1 as libc::c_int;
        }
        images = (*images).next_;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MuxEmitRiffHeader(
    data: *mut uint8_t,
    mut size: size_t,
) -> *mut uint8_t {
    PutLE32(
        data.offset(0 as libc::c_int as isize),
        ('R' as i32 | ('I' as i32) << 8 as libc::c_int
            | ('F' as i32) << 16 as libc::c_int) as libc::c_uint
            | ('F' as i32 as uint32_t) << 24 as libc::c_int,
    );
    PutLE32(
        data.offset(4 as libc::c_int as isize),
        (size as uint32_t).wrapping_sub(8 as libc::c_int as libc::c_uint),
    );
    PutLE32(
        data.offset(4 as libc::c_int as isize).offset(4 as libc::c_int as isize),
        ('W' as i32 | ('E' as i32) << 8 as libc::c_int
            | ('B' as i32) << 16 as libc::c_int) as libc::c_uint
            | ('P' as i32 as uint32_t) << 24 as libc::c_int,
    );
    return data.offset(12 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn MuxGetChunkListFromId(
    mut mux: *const WebPMux,
    mut id: WebPChunkId,
) -> *mut *mut WebPChunk {
    match id as libc::c_uint {
        0 => return &(*mux).vp8x_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        1 => return &(*mux).iccp_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        2 => return &(*mux).anim_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        7 => return &(*mux).exif_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        8 => return &(*mux).xmp_ as *const *mut WebPChunk as *mut *mut WebPChunk,
        _ => return &(*mux).unknown_ as *const *mut WebPChunk as *mut *mut WebPChunk,
    };
}
unsafe extern "C" fn IsNotCompatible(
    mut feature: libc::c_int,
    mut num_items: libc::c_int,
) -> libc::c_int {
    return ((feature != 0 as libc::c_int) as libc::c_int
        != (num_items > 0 as libc::c_int) as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ValidateChunk(
    mux: *const WebPMux,
    mut idx: CHUNK_INDEX,
    mut feature: WebPFeatureFlags,
    mut vp8x_flags: uint32_t,
    mut max: libc::c_int,
    mut num: *mut libc::c_int,
) -> WebPMuxError {
    let err: WebPMuxError = WebPMuxNumChunks(mux, kChunks[idx as usize].id, num);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if max > -(1 as libc::c_int) && *num > max {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if feature as libc::c_uint != 0 as WebPFeatureFlags as libc::c_uint
        && IsNotCompatible((vp8x_flags & feature as libc::c_uint) as libc::c_int, *num)
            != 0
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    return WEBP_MUX_OK;
}
#[no_mangle]
pub unsafe extern "C" fn MuxValidate(mux: *const WebPMux) -> WebPMuxError {
    let mut num_iccp: libc::c_int = 0;
    let mut num_exif: libc::c_int = 0;
    let mut num_xmp: libc::c_int = 0;
    let mut num_anim: libc::c_int = 0;
    let mut num_frames: libc::c_int = 0;
    let mut num_vp8x: libc::c_int = 0;
    let mut num_images: libc::c_int = 0;
    let mut num_alpha: libc::c_int = 0;
    let mut flags: uint32_t = 0;
    let mut err: WebPMuxError = WEBP_MUX_NOT_FOUND;
    if mux.is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if ((*mux).images_).is_null() {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    err = WebPMuxGetFeatures(mux, &mut flags);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_ICCP,
        ICCP_FLAG,
        flags,
        1 as libc::c_int,
        &mut num_iccp,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_EXIF,
        EXIF_FLAG,
        flags,
        1 as libc::c_int,
        &mut num_exif,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    err = ValidateChunk(mux, IDX_XMP, XMP_FLAG, flags, 1 as libc::c_int, &mut num_xmp);
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_ANIM,
        0 as WebPFeatureFlags,
        flags,
        1 as libc::c_int,
        &mut num_anim,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_ANMF,
        0 as WebPFeatureFlags,
        flags,
        -(1 as libc::c_int),
        &mut num_frames,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    let has_animation: libc::c_int = (flags
        & ANIMATION_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
    if has_animation != 0
        && (num_anim == 0 as libc::c_int || num_frames == 0 as libc::c_int)
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if has_animation == 0
        && (num_anim == 1 as libc::c_int || num_frames > 0 as libc::c_int)
    {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if has_animation == 0 {
        let mut images: *const WebPMuxImage = (*mux).images_;
        if images.is_null() || !((*images).next_).is_null() {
            return WEBP_MUX_INVALID_ARGUMENT;
        }
        if (*mux).canvas_width_ > 0 as libc::c_int {
            if (*images).width_ != (*mux).canvas_width_
                || (*images).height_ != (*mux).canvas_height_
            {
                return WEBP_MUX_INVALID_ARGUMENT;
            }
        }
    }
    err = ValidateChunk(
        mux,
        IDX_VP8X,
        0 as WebPFeatureFlags,
        flags,
        1 as libc::c_int,
        &mut num_vp8x,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    err = ValidateChunk(
        mux,
        IDX_VP8,
        0 as WebPFeatureFlags,
        flags,
        -(1 as libc::c_int),
        &mut num_images,
    );
    if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
        return err;
    }
    if num_vp8x == 0 as libc::c_int && num_images != 1 as libc::c_int {
        return WEBP_MUX_INVALID_ARGUMENT;
    }
    if MuxHasAlpha((*mux).images_) != 0 {
        if num_vp8x > 0 as libc::c_int {
            if flags & ALPHA_FLAG as libc::c_int as libc::c_uint == 0 {
                return WEBP_MUX_INVALID_ARGUMENT;
            }
        } else {
            err = WebPMuxNumChunks(mux, WEBP_CHUNK_ALPHA, &mut num_alpha);
            if err as libc::c_int != WEBP_MUX_OK as libc::c_int {
                return err;
            }
            if num_alpha > 0 as libc::c_int {
                return WEBP_MUX_INVALID_ARGUMENT;
            }
        }
    }
    return WEBP_MUX_OK;
}
