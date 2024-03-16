use ::libc;
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
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn WebPGetFeaturesInternal(
        _: *const uint8_t,
        _: size_t,
        _: *mut WebPBitstreamFeatures,
        _: libc::c_int,
    ) -> VP8StatusCode;
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
pub struct WebPBitstreamFeatures {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub has_alpha: libc::c_int,
    pub has_animation: libc::c_int,
    pub format: libc::c_int,
    pub pad: [uint32_t; 5],
}
pub type VP8StatusCode = libc::c_uint;
pub const VP8_STATUS_NOT_ENOUGH_DATA: VP8StatusCode = 7;
pub const VP8_STATUS_USER_ABORT: VP8StatusCode = 6;
pub const VP8_STATUS_SUSPENDED: VP8StatusCode = 5;
pub const VP8_STATUS_UNSUPPORTED_FEATURE: VP8StatusCode = 4;
pub const VP8_STATUS_BITSTREAM_ERROR: VP8StatusCode = 3;
pub const VP8_STATUS_INVALID_PARAM: VP8StatusCode = 2;
pub const VP8_STATUS_OUT_OF_MEMORY: VP8StatusCode = 1;
pub const VP8_STATUS_OK: VP8StatusCode = 0;
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
pub type WebPMuxAnimDispose = libc::c_uint;
pub const WEBP_MUX_DISPOSE_BACKGROUND: WebPMuxAnimDispose = 1;
pub const WEBP_MUX_DISPOSE_NONE: WebPMuxAnimDispose = 0;
pub type WebPMuxAnimBlend = libc::c_uint;
pub const WEBP_MUX_NO_BLEND: WebPMuxAnimBlend = 1;
pub const WEBP_MUX_BLEND: WebPMuxAnimBlend = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPDemuxer {
    pub mem_: MemBuffer,
    pub state_: WebPDemuxState,
    pub is_ext_format_: libc::c_int,
    pub feature_flags_: uint32_t,
    pub canvas_width_: libc::c_int,
    pub canvas_height_: libc::c_int,
    pub loop_count_: libc::c_int,
    pub bgcolor_: uint32_t,
    pub num_frames_: libc::c_int,
    pub frames_: *mut Frame,
    pub frames_tail_: *mut *mut Frame,
    pub chunks_: *mut Chunk,
    pub chunks_tail_: *mut *mut Chunk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Chunk {
    pub data_: ChunkData,
    pub next_: *mut Chunk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChunkData {
    pub offset_: size_t,
    pub size_: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Frame {
    pub x_offset_: libc::c_int,
    pub y_offset_: libc::c_int,
    pub width_: libc::c_int,
    pub height_: libc::c_int,
    pub has_alpha_: libc::c_int,
    pub duration_: libc::c_int,
    pub dispose_method_: WebPMuxAnimDispose,
    pub blend_method_: WebPMuxAnimBlend,
    pub frame_num_: libc::c_int,
    pub complete_: libc::c_int,
    pub img_components_: [ChunkData; 2],
    pub next_: *mut Frame,
}
pub type WebPDemuxState = libc::c_int;
pub const WEBP_DEMUX_DONE: WebPDemuxState = 2;
pub const WEBP_DEMUX_PARSED_HEADER: WebPDemuxState = 1;
pub const WEBP_DEMUX_PARSING_HEADER: WebPDemuxState = 0;
pub const WEBP_DEMUX_PARSE_ERROR: WebPDemuxState = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemBuffer {
    pub start_: size_t,
    pub end_: size_t,
    pub riff_end_: size_t,
    pub buf_size_: size_t,
    pub buf_: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPIterator {
    pub frame_num: libc::c_int,
    pub num_frames: libc::c_int,
    pub x_offset: libc::c_int,
    pub y_offset: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub duration: libc::c_int,
    pub dispose_method: WebPMuxAnimDispose,
    pub complete: libc::c_int,
    pub fragment: WebPData,
    pub has_alpha: libc::c_int,
    pub blend_method: WebPMuxAnimBlend,
    pub pad: [uint32_t; 2],
    pub private_: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPChunkIterator {
    pub chunk_num: libc::c_int,
    pub num_chunks: libc::c_int,
    pub chunk: WebPData,
    pub pad: [uint32_t; 6],
    pub private_: *mut libc::c_void,
}
pub const PARSE_ERROR: ParseStatus = 2;
pub type ParseStatus = libc::c_uint;
pub const PARSE_NEED_MORE_DATA: ParseStatus = 1;
pub const PARSE_OK: ParseStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChunkParser {
    pub id: [uint8_t; 4],
    pub parse: Option::<unsafe extern "C" fn(*mut WebPDemuxer) -> ParseStatus>,
    pub valid: Option::<unsafe extern "C" fn(*const WebPDemuxer) -> libc::c_int>,
}
pub type WebPFormatFeature = libc::c_uint;
pub const WEBP_FF_FRAME_COUNT: WebPFormatFeature = 5;
pub const WEBP_FF_BACKGROUND_COLOR: WebPFormatFeature = 4;
pub const WEBP_FF_LOOP_COUNT: WebPFormatFeature = 3;
pub const WEBP_FF_CANVAS_HEIGHT: WebPFormatFeature = 2;
pub const WEBP_FF_CANVAS_WIDTH: WebPFormatFeature = 1;
pub const WEBP_FF_FORMAT_FLAGS: WebPFormatFeature = 0;
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
unsafe extern "C" fn WebPGetFeatures(
    mut data: *const uint8_t,
    mut data_size: size_t,
    mut features: *mut WebPBitstreamFeatures,
) -> VP8StatusCode {
    return WebPGetFeaturesInternal(data, data_size, features, 0x209 as libc::c_int);
}
static mut kMasterChunks: [ChunkParser; 4] = unsafe {
    [
        {
            let mut init = ChunkParser {
                id: [
                    'V' as i32 as uint8_t,
                    'P' as i32 as uint8_t,
                    '8' as i32 as uint8_t,
                    ' ' as i32 as uint8_t,
                ],
                parse: Some(
                    ParseSingleImage
                        as unsafe extern "C" fn(*mut WebPDemuxer) -> ParseStatus,
                ),
                valid: Some(
                    IsValidSimpleFormat
                        as unsafe extern "C" fn(*const WebPDemuxer) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = ChunkParser {
                id: [
                    'V' as i32 as uint8_t,
                    'P' as i32 as uint8_t,
                    '8' as i32 as uint8_t,
                    'L' as i32 as uint8_t,
                ],
                parse: Some(
                    ParseSingleImage
                        as unsafe extern "C" fn(*mut WebPDemuxer) -> ParseStatus,
                ),
                valid: Some(
                    IsValidSimpleFormat
                        as unsafe extern "C" fn(*const WebPDemuxer) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = ChunkParser {
                id: [
                    'V' as i32 as uint8_t,
                    'P' as i32 as uint8_t,
                    '8' as i32 as uint8_t,
                    'X' as i32 as uint8_t,
                ],
                parse: Some(
                    ParseVP8X as unsafe extern "C" fn(*mut WebPDemuxer) -> ParseStatus,
                ),
                valid: Some(
                    IsValidExtendedFormat
                        as unsafe extern "C" fn(*const WebPDemuxer) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = ChunkParser {
                id: [
                    '0' as i32 as uint8_t,
                    '0' as i32 as uint8_t,
                    '0' as i32 as uint8_t,
                    '0' as i32 as uint8_t,
                ],
                parse: None,
                valid: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn WebPGetDemuxVersion() -> libc::c_int {
    return (1 as libc::c_int) << 16 as libc::c_int
        | (3 as libc::c_int) << 8 as libc::c_int | 2 as libc::c_int;
}
unsafe extern "C" fn RemapMemBuffer(
    mem: *mut MemBuffer,
    mut data: *const uint8_t,
    mut size: size_t,
) -> libc::c_int {
    if size < (*mem).buf_size_ {
        return 0 as libc::c_int;
    }
    (*mem).buf_ = data;
    (*mem).buf_size_ = size;
    (*mem).end_ = (*mem).buf_size_;
    return 1 as libc::c_int;
}
unsafe extern "C" fn InitMemBuffer(
    mem: *mut MemBuffer,
    mut data: *const uint8_t,
    mut size: size_t,
) -> libc::c_int {
    memset(
        mem as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<MemBuffer>() as libc::c_ulong,
    );
    return RemapMemBuffer(mem, data, size);
}
#[inline]
unsafe extern "C" fn MemDataSize(mem: *const MemBuffer) -> size_t {
    return ((*mem).end_).wrapping_sub((*mem).start_);
}
#[inline]
unsafe extern "C" fn SizeIsInvalid(
    mem: *const MemBuffer,
    mut size: size_t,
) -> libc::c_int {
    return (size > ((*mem).riff_end_).wrapping_sub((*mem).start_)) as libc::c_int;
}
#[inline]
unsafe extern "C" fn Skip(mem: *mut MemBuffer, mut size: size_t) {
    (*mem)
        .start_ = ((*mem).start_ as libc::c_ulong).wrapping_add(size) as size_t
        as size_t;
}
#[inline]
unsafe extern "C" fn Rewind(mem: *mut MemBuffer, mut size: size_t) {
    (*mem)
        .start_ = ((*mem).start_ as libc::c_ulong).wrapping_sub(size) as size_t
        as size_t;
}
#[inline]
unsafe extern "C" fn GetBuffer(mem: *mut MemBuffer) -> *const uint8_t {
    return ((*mem).buf_).offset((*mem).start_ as isize);
}
#[inline]
unsafe extern "C" fn ReadByte(mem: *mut MemBuffer) -> uint8_t {
    let byte: uint8_t = *((*mem).buf_).offset((*mem).start_ as isize);
    Skip(mem, 1 as libc::c_int as size_t);
    return byte;
}
#[inline]
unsafe extern "C" fn ReadLE16s(mem: *mut MemBuffer) -> libc::c_int {
    let data: *const uint8_t = ((*mem).buf_).offset((*mem).start_ as isize);
    let val: libc::c_int = GetLE16(data);
    Skip(mem, 2 as libc::c_int as size_t);
    return val;
}
#[inline]
unsafe extern "C" fn ReadLE24s(mem: *mut MemBuffer) -> libc::c_int {
    let data: *const uint8_t = ((*mem).buf_).offset((*mem).start_ as isize);
    let val: libc::c_int = GetLE24(data);
    Skip(mem, 3 as libc::c_int as size_t);
    return val;
}
#[inline]
unsafe extern "C" fn ReadLE32(mem: *mut MemBuffer) -> uint32_t {
    let data: *const uint8_t = ((*mem).buf_).offset((*mem).start_ as isize);
    let val: uint32_t = GetLE32(data);
    Skip(mem, 4 as libc::c_int as size_t);
    return val;
}
unsafe extern "C" fn AddChunk(dmux: *mut WebPDemuxer, chunk: *mut Chunk) {
    *(*dmux).chunks_tail_ = chunk;
    (*chunk).next_ = 0 as *mut Chunk;
    (*dmux).chunks_tail_ = &mut (*chunk).next_;
}
unsafe extern "C" fn AddFrame(dmux: *mut WebPDemuxer, frame: *mut Frame) -> libc::c_int {
    let last_frame: *const Frame = *(*dmux).frames_tail_;
    if !last_frame.is_null() && (*last_frame).complete_ == 0 {
        return 0 as libc::c_int;
    }
    *(*dmux).frames_tail_ = frame;
    (*frame).next_ = 0 as *mut Frame;
    (*dmux).frames_tail_ = &mut (*frame).next_;
    return 1 as libc::c_int;
}
unsafe extern "C" fn SetFrameInfo(
    mut start_offset: size_t,
    mut size: size_t,
    mut frame_num: libc::c_int,
    mut complete: libc::c_int,
    features: *const WebPBitstreamFeatures,
    frame: *mut Frame,
) {
    (*frame).img_components_[0 as libc::c_int as usize].offset_ = start_offset;
    (*frame).img_components_[0 as libc::c_int as usize].size_ = size;
    (*frame).width_ = (*features).width;
    (*frame).height_ = (*features).height;
    (*frame).has_alpha_ |= (*features).has_alpha;
    (*frame).frame_num_ = frame_num;
    (*frame).complete_ = complete;
}
unsafe extern "C" fn StoreFrame(
    mut frame_num: libc::c_int,
    mut min_size: uint32_t,
    mem: *mut MemBuffer,
    frame: *mut Frame,
) -> ParseStatus {
    let mut current_block: u64;
    let mut alpha_chunks: libc::c_int = 0 as libc::c_int;
    let mut image_chunks: libc::c_int = 0 as libc::c_int;
    let mut done: libc::c_int = (MemDataSize(mem) < 8 as libc::c_int as libc::c_ulong
        || MemDataSize(mem) < min_size as libc::c_ulong) as libc::c_int;
    let mut status: ParseStatus = PARSE_OK;
    if done != 0 {
        return PARSE_NEED_MORE_DATA;
    }
    loop {
        let chunk_start_offset: size_t = (*mem).start_;
        let fourcc: uint32_t = ReadLE32(mem);
        let payload_size: uint32_t = ReadLE32(mem);
        let mut payload_size_padded: uint32_t = 0;
        let mut payload_available: size_t = 0;
        let mut chunk_size: size_t = 0;
        if payload_size
            > (!(0 as libc::c_uint))
                .wrapping_sub(8 as libc::c_int as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            return PARSE_ERROR;
        }
        payload_size_padded = payload_size
            .wrapping_add(payload_size & 1 as libc::c_int as libc::c_uint);
        payload_available = if payload_size_padded as libc::c_ulong > MemDataSize(mem) {
            MemDataSize(mem)
        } else {
            payload_size_padded as libc::c_ulong
        };
        chunk_size = (8 as libc::c_int as libc::c_ulong).wrapping_add(payload_available);
        if SizeIsInvalid(mem, payload_size_padded as size_t) != 0 {
            return PARSE_ERROR;
        }
        if payload_size_padded as libc::c_ulong > MemDataSize(mem) {
            status = PARSE_NEED_MORE_DATA;
        }
        match fourcc {
            1213221953 => {
                if alpha_chunks == 0 as libc::c_int {
                    alpha_chunks += 1;
                    alpha_chunks;
                    (*frame)
                        .img_components_[1 as libc::c_int as usize]
                        .offset_ = chunk_start_offset;
                    (*frame)
                        .img_components_[1 as libc::c_int as usize]
                        .size_ = chunk_size;
                    (*frame).has_alpha_ = 1 as libc::c_int;
                    (*frame).frame_num_ = frame_num;
                    Skip(mem, payload_available);
                    current_block = 18153031941552419006;
                } else {
                    current_block = 7248293206456826136;
                }
            }
            1278758998 => {
                if alpha_chunks > 0 as libc::c_int {
                    return PARSE_ERROR;
                }
                current_block = 5561034979529426474;
            }
            540561494 => {
                current_block = 5561034979529426474;
            }
            _ => {
                current_block = 7248293206456826136;
            }
        }
        match current_block {
            5561034979529426474 => {
                if image_chunks == 0 as libc::c_int {
                    let mut features: WebPBitstreamFeatures = WebPBitstreamFeatures {
                        width: 0,
                        height: 0,
                        has_alpha: 0,
                        has_animation: 0,
                        format: 0,
                        pad: [0; 5],
                    };
                    let vp8_status: VP8StatusCode = WebPGetFeatures(
                        ((*mem).buf_).offset(chunk_start_offset as isize),
                        chunk_size,
                        &mut features,
                    );
                    if status as libc::c_uint
                        == PARSE_NEED_MORE_DATA as libc::c_int as libc::c_uint
                        && vp8_status as libc::c_uint
                            == VP8_STATUS_NOT_ENOUGH_DATA as libc::c_int as libc::c_uint
                    {
                        return PARSE_NEED_MORE_DATA
                    } else if vp8_status as libc::c_uint
                        != VP8_STATUS_OK as libc::c_int as libc::c_uint
                    {
                        return PARSE_ERROR
                    }
                    image_chunks += 1;
                    image_chunks;
                    SetFrameInfo(
                        chunk_start_offset,
                        chunk_size,
                        frame_num,
                        (status as libc::c_uint
                            == PARSE_OK as libc::c_int as libc::c_uint) as libc::c_int,
                        &mut features,
                        frame,
                    );
                    Skip(mem, payload_available);
                    current_block = 18153031941552419006;
                } else {
                    current_block = 7248293206456826136;
                }
            }
            _ => {}
        }
        match current_block {
            7248293206456826136 => {
                Rewind(mem, 8 as libc::c_int as size_t);
                done = 1 as libc::c_int;
            }
            _ => {}
        }
        if (*mem).start_ == (*mem).riff_end_ {
            done = 1 as libc::c_int;
        } else if MemDataSize(mem) < 8 as libc::c_int as libc::c_ulong {
            status = PARSE_NEED_MORE_DATA;
        }
        if !(done == 0
            && status as libc::c_uint == PARSE_OK as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return status;
}
unsafe extern "C" fn NewFrame(
    mem: *const MemBuffer,
    mut min_size: uint32_t,
    mut actual_size: uint32_t,
    mut frame: *mut *mut Frame,
) -> ParseStatus {
    if SizeIsInvalid(mem, min_size as size_t) != 0 {
        return PARSE_ERROR;
    }
    if actual_size < min_size {
        return PARSE_ERROR;
    }
    if MemDataSize(mem) < min_size as libc::c_ulong {
        return PARSE_NEED_MORE_DATA;
    }
    *frame = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<Frame>() as libc::c_ulong,
    ) as *mut Frame;
    return (if (*frame).is_null() {
        PARSE_ERROR as libc::c_int
    } else {
        PARSE_OK as libc::c_int
    }) as ParseStatus;
}
unsafe extern "C" fn ParseAnimationFrame(
    dmux: *mut WebPDemuxer,
    mut frame_chunk_size: uint32_t,
) -> ParseStatus {
    let is_animation: libc::c_int = ((*dmux).feature_flags_
        & ANIMATION_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
    let anmf_payload_size: uint32_t = frame_chunk_size
        .wrapping_sub(16 as libc::c_int as libc::c_uint);
    let mut added_frame: libc::c_int = 0 as libc::c_int;
    let mut bits: libc::c_int = 0;
    let mem: *mut MemBuffer = &mut (*dmux).mem_;
    let mut frame: *mut Frame = 0 as *mut Frame;
    let mut start_offset: size_t = 0;
    let mut status: ParseStatus = NewFrame(
        mem,
        16 as libc::c_int as uint32_t,
        frame_chunk_size,
        &mut frame,
    );
    if status as libc::c_uint != PARSE_OK as libc::c_int as libc::c_uint {
        return status;
    }
    (*frame).x_offset_ = 2 as libc::c_int * ReadLE24s(mem);
    (*frame).y_offset_ = 2 as libc::c_int * ReadLE24s(mem);
    (*frame).width_ = 1 as libc::c_int + ReadLE24s(mem);
    (*frame).height_ = 1 as libc::c_int + ReadLE24s(mem);
    (*frame).duration_ = ReadLE24s(mem);
    bits = ReadByte(mem) as libc::c_int;
    (*frame)
        .dispose_method_ = (if bits & 1 as libc::c_int != 0 {
        WEBP_MUX_DISPOSE_BACKGROUND as libc::c_int
    } else {
        WEBP_MUX_DISPOSE_NONE as libc::c_int
    }) as WebPMuxAnimDispose;
    (*frame)
        .blend_method_ = (if bits & 2 as libc::c_int != 0 {
        WEBP_MUX_NO_BLEND as libc::c_int
    } else {
        WEBP_MUX_BLEND as libc::c_int
    }) as WebPMuxAnimBlend;
    if ((*frame).width_ as libc::c_ulong).wrapping_mul((*frame).height_ as uint64_t)
        as libc::c_ulonglong >= (1 as libc::c_ulonglong) << 32 as libc::c_int
    {
        WebPSafeFree(frame as *mut libc::c_void);
        return PARSE_ERROR;
    }
    start_offset = (*mem).start_;
    status = StoreFrame(
        (*dmux).num_frames_ + 1 as libc::c_int,
        anmf_payload_size,
        mem,
        frame,
    );
    if status as libc::c_uint != PARSE_ERROR as libc::c_int as libc::c_uint
        && ((*mem).start_).wrapping_sub(start_offset)
            > anmf_payload_size as libc::c_ulong
    {
        status = PARSE_ERROR;
    }
    if status as libc::c_uint != PARSE_ERROR as libc::c_int as libc::c_uint
        && is_animation != 0 && (*frame).frame_num_ > 0 as libc::c_int
    {
        added_frame = AddFrame(dmux, frame);
        if added_frame != 0 {
            (*dmux).num_frames_ += 1;
            (*dmux).num_frames_;
        } else {
            status = PARSE_ERROR;
        }
    }
    if added_frame == 0 {
        WebPSafeFree(frame as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn StoreChunk(
    dmux: *mut WebPDemuxer,
    mut start_offset: size_t,
    mut size: uint32_t,
) -> libc::c_int {
    let chunk: *mut Chunk = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<Chunk>() as libc::c_ulong,
    ) as *mut Chunk;
    if chunk.is_null() {
        return 0 as libc::c_int;
    }
    (*chunk).data_.offset_ = start_offset;
    (*chunk).data_.size_ = size as size_t;
    AddChunk(dmux, chunk);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ReadHeader(mem: *mut MemBuffer) -> ParseStatus {
    let min_size: size_t = (12 as libc::c_int + 8 as libc::c_int) as size_t;
    let mut riff_size: uint32_t = 0;
    if MemDataSize(mem) < min_size {
        return PARSE_NEED_MORE_DATA;
    }
    if memcmp(
        GetBuffer(mem) as *const libc::c_void,
        b"RIFF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) != 0
        || memcmp(
            (GetBuffer(mem)).offset(8 as libc::c_int as isize) as *const libc::c_void,
            b"WEBP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
    {
        return PARSE_ERROR;
    }
    riff_size = GetLE32((GetBuffer(mem)).offset(4 as libc::c_int as isize));
    if riff_size < 8 as libc::c_int as libc::c_uint {
        return PARSE_ERROR;
    }
    if riff_size
        > (!(0 as libc::c_uint))
            .wrapping_sub(8 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return PARSE_ERROR;
    }
    (*mem)
        .riff_end_ = riff_size.wrapping_add(8 as libc::c_int as libc::c_uint) as size_t;
    if (*mem).buf_size_ > (*mem).riff_end_ {
        (*mem).end_ = (*mem).riff_end_;
        (*mem).buf_size_ = (*mem).end_;
    }
    Skip(mem, 12 as libc::c_int as size_t);
    return PARSE_OK;
}
unsafe extern "C" fn ParseSingleImage(dmux: *mut WebPDemuxer) -> ParseStatus {
    let min_size: size_t = 8 as libc::c_int as size_t;
    let mem: *mut MemBuffer = &mut (*dmux).mem_;
    let mut frame: *mut Frame = 0 as *mut Frame;
    let mut status: ParseStatus = PARSE_OK;
    let mut image_added: libc::c_int = 0 as libc::c_int;
    if !((*dmux).frames_).is_null() {
        return PARSE_ERROR;
    }
    if SizeIsInvalid(mem, min_size) != 0 {
        return PARSE_ERROR;
    }
    if MemDataSize(mem) < min_size {
        return PARSE_NEED_MORE_DATA;
    }
    frame = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<Frame>() as libc::c_ulong,
    ) as *mut Frame;
    if frame.is_null() {
        return PARSE_ERROR;
    }
    status = StoreFrame(
        1 as libc::c_int,
        0 as libc::c_int as uint32_t,
        &mut (*dmux).mem_,
        frame,
    );
    if status as libc::c_uint != PARSE_ERROR as libc::c_int as libc::c_uint {
        let has_alpha: libc::c_int = ((*dmux).feature_flags_
            & ALPHA_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
        if has_alpha == 0
            && (*frame).img_components_[1 as libc::c_int as usize].size_
                > 0 as libc::c_int as libc::c_ulong
        {
            (*frame)
                .img_components_[1 as libc::c_int as usize]
                .offset_ = 0 as libc::c_int as size_t;
            (*frame)
                .img_components_[1 as libc::c_int as usize]
                .size_ = 0 as libc::c_int as size_t;
            (*frame).has_alpha_ = 0 as libc::c_int;
        }
        if (*dmux).is_ext_format_ == 0 && (*frame).width_ > 0 as libc::c_int
            && (*frame).height_ > 0 as libc::c_int
        {
            (*dmux).state_ = WEBP_DEMUX_PARSED_HEADER;
            (*dmux).canvas_width_ = (*frame).width_;
            (*dmux).canvas_height_ = (*frame).height_;
            (*dmux).feature_flags_
                |= (if (*frame).has_alpha_ != 0 {
                    ALPHA_FLAG as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_uint;
        }
        if AddFrame(dmux, frame) == 0 {
            status = PARSE_ERROR;
        } else {
            image_added = 1 as libc::c_int;
            (*dmux).num_frames_ = 1 as libc::c_int;
        }
    }
    if image_added == 0 {
        WebPSafeFree(frame as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn ParseVP8XChunks(dmux: *mut WebPDemuxer) -> ParseStatus {
    let mut current_block: u64;
    let is_animation: libc::c_int = ((*dmux).feature_flags_
        & ANIMATION_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
    let mem: *mut MemBuffer = &mut (*dmux).mem_;
    let mut anim_chunks: libc::c_int = 0 as libc::c_int;
    let mut status: ParseStatus = PARSE_OK;
    loop {
        let mut store_chunk: libc::c_int = 1 as libc::c_int;
        let chunk_start_offset: size_t = (*mem).start_;
        let fourcc: uint32_t = ReadLE32(mem);
        let chunk_size: uint32_t = ReadLE32(mem);
        let mut chunk_size_padded: uint32_t = 0;
        if chunk_size
            > (!(0 as libc::c_uint))
                .wrapping_sub(8 as libc::c_int as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            return PARSE_ERROR;
        }
        chunk_size_padded = chunk_size
            .wrapping_add(chunk_size & 1 as libc::c_int as libc::c_uint);
        if SizeIsInvalid(mem, chunk_size_padded as size_t) != 0 {
            return PARSE_ERROR;
        }
        match fourcc {
            1480085590 => return PARSE_ERROR,
            1213221953 | 540561494 | 1278758998 => {
                if anim_chunks > 0 as libc::c_int || is_animation != 0 {
                    return PARSE_ERROR;
                }
                Rewind(mem, 8 as libc::c_int as size_t);
                status = ParseSingleImage(dmux);
                current_block = 12199444798915819164;
            }
            1296649793 => {
                if chunk_size_padded < 6 as libc::c_int as libc::c_uint {
                    return PARSE_ERROR;
                }
                if MemDataSize(mem) < chunk_size_padded as libc::c_ulong {
                    status = PARSE_NEED_MORE_DATA;
                    current_block = 12199444798915819164;
                } else if anim_chunks == 0 as libc::c_int {
                    anim_chunks += 1;
                    anim_chunks;
                    (*dmux).bgcolor_ = ReadLE32(mem);
                    (*dmux).loop_count_ = ReadLE16s(mem);
                    Skip(
                        mem,
                        chunk_size_padded.wrapping_sub(6 as libc::c_int as libc::c_uint)
                            as size_t,
                    );
                    current_block = 12199444798915819164;
                } else {
                    store_chunk = 0 as libc::c_int;
                    current_block = 9216587133067411178;
                }
            }
            1179471425 => {
                if anim_chunks == 0 as libc::c_int {
                    return PARSE_ERROR;
                }
                status = ParseAnimationFrame(dmux, chunk_size_padded);
                current_block = 12199444798915819164;
            }
            1346585417 => {
                store_chunk = ((*dmux).feature_flags_
                    & ICCP_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
                current_block = 9216587133067411178;
            }
            1179211845 => {
                store_chunk = ((*dmux).feature_flags_
                    & EXIF_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
                current_block = 9216587133067411178;
            }
            542133592 => {
                store_chunk = ((*dmux).feature_flags_
                    & XMP_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
                current_block = 9216587133067411178;
            }
            _ => {
                current_block = 9216587133067411178;
            }
        }
        match current_block {
            9216587133067411178 => {
                if chunk_size_padded as libc::c_ulong <= MemDataSize(mem) {
                    if store_chunk != 0 {
                        if StoreChunk(
                            dmux,
                            chunk_start_offset,
                            (8 as libc::c_int as libc::c_uint).wrapping_add(chunk_size),
                        ) == 0
                        {
                            return PARSE_ERROR;
                        }
                    }
                    Skip(mem, chunk_size_padded as size_t);
                } else {
                    status = PARSE_NEED_MORE_DATA;
                }
            }
            _ => {}
        }
        if (*mem).start_ == (*mem).riff_end_ {
            break;
        }
        if MemDataSize(mem) < 8 as libc::c_int as libc::c_ulong {
            status = PARSE_NEED_MORE_DATA;
        }
        if !(status as libc::c_uint == PARSE_OK as libc::c_int as libc::c_uint) {
            break;
        }
    }
    return status;
}
unsafe extern "C" fn ParseVP8X(dmux: *mut WebPDemuxer) -> ParseStatus {
    let mem: *mut MemBuffer = &mut (*dmux).mem_;
    let mut vp8x_size: uint32_t = 0;
    if MemDataSize(mem) < 8 as libc::c_int as libc::c_ulong {
        return PARSE_NEED_MORE_DATA;
    }
    (*dmux).is_ext_format_ = 1 as libc::c_int;
    Skip(mem, 4 as libc::c_int as size_t);
    vp8x_size = ReadLE32(mem);
    if vp8x_size
        > (!(0 as libc::c_uint))
            .wrapping_sub(8 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return PARSE_ERROR;
    }
    if vp8x_size < 10 as libc::c_int as libc::c_uint {
        return PARSE_ERROR;
    }
    vp8x_size = (vp8x_size as libc::c_uint)
        .wrapping_add(vp8x_size & 1 as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    if SizeIsInvalid(mem, vp8x_size as size_t) != 0 {
        return PARSE_ERROR;
    }
    if MemDataSize(mem) < vp8x_size as libc::c_ulong {
        return PARSE_NEED_MORE_DATA;
    }
    (*dmux).feature_flags_ = ReadByte(mem) as uint32_t;
    Skip(mem, 3 as libc::c_int as size_t);
    (*dmux).canvas_width_ = 1 as libc::c_int + ReadLE24s(mem);
    (*dmux).canvas_height_ = 1 as libc::c_int + ReadLE24s(mem);
    if ((*dmux).canvas_width_ as libc::c_ulong)
        .wrapping_mul((*dmux).canvas_height_ as uint64_t) as libc::c_ulonglong
        >= (1 as libc::c_ulonglong) << 32 as libc::c_int
    {
        return PARSE_ERROR;
    }
    Skip(mem, vp8x_size.wrapping_sub(10 as libc::c_int as libc::c_uint) as size_t);
    (*dmux).state_ = WEBP_DEMUX_PARSED_HEADER;
    if SizeIsInvalid(mem, 8 as libc::c_int as size_t) != 0 {
        return PARSE_ERROR;
    }
    if MemDataSize(mem) < 8 as libc::c_int as libc::c_ulong {
        return PARSE_NEED_MORE_DATA;
    }
    return ParseVP8XChunks(dmux);
}
unsafe extern "C" fn IsValidSimpleFormat(dmux: *const WebPDemuxer) -> libc::c_int {
    let frame: *const Frame = (*dmux).frames_;
    if (*dmux).state_ as libc::c_int == WEBP_DEMUX_PARSING_HEADER as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*dmux).canvas_width_ <= 0 as libc::c_int
        || (*dmux).canvas_height_ <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*dmux).state_ as libc::c_int == WEBP_DEMUX_DONE as libc::c_int && frame.is_null()
    {
        return 0 as libc::c_int;
    }
    if (*frame).width_ <= 0 as libc::c_int || (*frame).height_ <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn CheckFrameBounds(
    frame: *const Frame,
    mut exact: libc::c_int,
    mut canvas_width: libc::c_int,
    mut canvas_height: libc::c_int,
) -> libc::c_int {
    if exact != 0 {
        if (*frame).x_offset_ != 0 as libc::c_int
            || (*frame).y_offset_ != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if (*frame).width_ != canvas_width || (*frame).height_ != canvas_height {
            return 0 as libc::c_int;
        }
    } else {
        if (*frame).x_offset_ < 0 as libc::c_int || (*frame).y_offset_ < 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if (*frame).width_ + (*frame).x_offset_ > canvas_width {
            return 0 as libc::c_int;
        }
        if (*frame).height_ + (*frame).y_offset_ > canvas_height {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn IsValidExtendedFormat(dmux: *const WebPDemuxer) -> libc::c_int {
    let is_animation: libc::c_int = ((*dmux).feature_flags_
        & ANIMATION_FLAG as libc::c_int as libc::c_uint != 0) as libc::c_int;
    let mut f: *const Frame = (*dmux).frames_;
    if (*dmux).state_ as libc::c_int == WEBP_DEMUX_PARSING_HEADER as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*dmux).canvas_width_ <= 0 as libc::c_int
        || (*dmux).canvas_height_ <= 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*dmux).loop_count_ < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*dmux).state_ as libc::c_int == WEBP_DEMUX_DONE as libc::c_int
        && ((*dmux).frames_).is_null()
    {
        return 0 as libc::c_int;
    }
    if (*dmux).feature_flags_ & !(ALL_VALID_FLAGS as libc::c_int) as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    while !f.is_null() {
        let cur_frame_set: libc::c_int = (*f).frame_num_;
        while !f.is_null() && (*f).frame_num_ == cur_frame_set {
            let image: *const ChunkData = ((*f).img_components_).as_ptr();
            let alpha: *const ChunkData = ((*f).img_components_)
                .as_ptr()
                .offset(1 as libc::c_int as isize);
            if is_animation == 0 && (*f).frame_num_ > 1 as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*f).complete_ != 0 {
                if (*alpha).size_ == 0 as libc::c_int as libc::c_ulong
                    && (*image).size_ == 0 as libc::c_int as libc::c_ulong
                {
                    return 0 as libc::c_int;
                }
                if (*alpha).size_ > 0 as libc::c_int as libc::c_ulong
                    && (*alpha).offset_ > (*image).offset_
                {
                    return 0 as libc::c_int;
                }
                if (*f).width_ <= 0 as libc::c_int || (*f).height_ <= 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
            } else {
                if (*dmux).state_ as libc::c_int == WEBP_DEMUX_DONE as libc::c_int {
                    return 0 as libc::c_int;
                }
                if (*alpha).size_ > 0 as libc::c_int as libc::c_ulong
                    && (*image).size_ > 0 as libc::c_int as libc::c_ulong
                    && (*alpha).offset_ > (*image).offset_
                {
                    return 0 as libc::c_int;
                }
                if !((*f).next_).is_null() {
                    return 0 as libc::c_int;
                }
            }
            if (*f).width_ > 0 as libc::c_int && (*f).height_ > 0 as libc::c_int
                && CheckFrameBounds(
                    f,
                    (is_animation == 0) as libc::c_int,
                    (*dmux).canvas_width_,
                    (*dmux).canvas_height_,
                ) == 0
            {
                return 0 as libc::c_int;
            }
            f = (*f).next_;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn InitDemux(dmux: *mut WebPDemuxer, mem: *const MemBuffer) {
    (*dmux).state_ = WEBP_DEMUX_PARSING_HEADER;
    (*dmux).loop_count_ = 1 as libc::c_int;
    (*dmux).bgcolor_ = 0xffffffff as libc::c_uint;
    (*dmux).canvas_width_ = -(1 as libc::c_int);
    (*dmux).canvas_height_ = -(1 as libc::c_int);
    (*dmux).frames_tail_ = &mut (*dmux).frames_;
    (*dmux).chunks_tail_ = &mut (*dmux).chunks_;
    (*dmux).mem_ = *mem;
}
unsafe extern "C" fn CreateRawImageDemuxer(
    mem: *mut MemBuffer,
    mut demuxer: *mut *mut WebPDemuxer,
) -> ParseStatus {
    let mut features: WebPBitstreamFeatures = WebPBitstreamFeatures {
        width: 0,
        height: 0,
        has_alpha: 0,
        has_animation: 0,
        format: 0,
        pad: [0; 5],
    };
    let status: VP8StatusCode = WebPGetFeatures(
        (*mem).buf_,
        (*mem).buf_size_,
        &mut features,
    );
    *demuxer = 0 as *mut WebPDemuxer;
    if status as libc::c_uint != VP8_STATUS_OK as libc::c_int as libc::c_uint {
        return (if status as libc::c_uint
            == VP8_STATUS_NOT_ENOUGH_DATA as libc::c_int as libc::c_uint
        {
            PARSE_NEED_MORE_DATA as libc::c_int
        } else {
            PARSE_ERROR as libc::c_int
        }) as ParseStatus;
    }
    let dmux: *mut WebPDemuxer = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<WebPDemuxer>() as libc::c_ulong,
    ) as *mut WebPDemuxer;
    let frame: *mut Frame = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<Frame>() as libc::c_ulong,
    ) as *mut Frame;
    if !(dmux.is_null() || frame.is_null()) {
        InitDemux(dmux, mem);
        SetFrameInfo(
            0 as libc::c_int as size_t,
            (*mem).buf_size_,
            1 as libc::c_int,
            1 as libc::c_int,
            &mut features,
            frame,
        );
        if !(AddFrame(dmux, frame) == 0) {
            (*dmux).state_ = WEBP_DEMUX_DONE;
            (*dmux).canvas_width_ = (*frame).width_;
            (*dmux).canvas_height_ = (*frame).height_;
            (*dmux).feature_flags_
                |= (if (*frame).has_alpha_ != 0 {
                    ALPHA_FLAG as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_uint;
            (*dmux).num_frames_ = 1 as libc::c_int;
            *demuxer = dmux;
            return PARSE_OK;
        }
    }
    WebPSafeFree(dmux as *mut libc::c_void);
    WebPSafeFree(frame as *mut libc::c_void);
    return PARSE_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxInternal(
    mut data: *const WebPData,
    mut allow_partial: libc::c_int,
    mut state: *mut WebPDemuxState,
    mut version: libc::c_int,
) -> *mut WebPDemuxer {
    let mut parser: *const ChunkParser = 0 as *const ChunkParser;
    let mut partial: libc::c_int = 0;
    let mut status: ParseStatus = PARSE_ERROR;
    let mut mem: MemBuffer = MemBuffer {
        start_: 0,
        end_: 0,
        riff_end_: 0,
        buf_size_: 0,
        buf_: 0 as *const uint8_t,
    };
    let mut dmux: *mut WebPDemuxer = 0 as *mut WebPDemuxer;
    if !state.is_null() {
        *state = WEBP_DEMUX_PARSE_ERROR;
    }
    if version >> 8 as libc::c_int != 0x107 as libc::c_int >> 8 as libc::c_int {
        return 0 as *mut WebPDemuxer;
    }
    if data.is_null() || ((*data).bytes).is_null()
        || (*data).size == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut WebPDemuxer;
    }
    if InitMemBuffer(&mut mem, (*data).bytes, (*data).size) == 0 {
        return 0 as *mut WebPDemuxer;
    }
    status = ReadHeader(&mut mem);
    if status as libc::c_uint != PARSE_OK as libc::c_int as libc::c_uint {
        if status as libc::c_uint == PARSE_ERROR as libc::c_int as libc::c_uint {
            status = CreateRawImageDemuxer(&mut mem, &mut dmux);
            if status as libc::c_uint == PARSE_OK as libc::c_int as libc::c_uint {
                if !state.is_null() {
                    *state = WEBP_DEMUX_DONE;
                }
                return dmux;
            }
        }
        if !state.is_null() {
            *state = (if status as libc::c_uint
                == PARSE_NEED_MORE_DATA as libc::c_int as libc::c_uint
            {
                WEBP_DEMUX_PARSING_HEADER as libc::c_int
            } else {
                WEBP_DEMUX_PARSE_ERROR as libc::c_int
            }) as WebPDemuxState;
        }
        return 0 as *mut WebPDemuxer;
    }
    partial = (mem.buf_size_ < mem.riff_end_) as libc::c_int;
    if allow_partial == 0 && partial != 0 {
        return 0 as *mut WebPDemuxer;
    }
    dmux = WebPSafeCalloc(
        1 as libc::c_ulonglong as uint64_t,
        ::core::mem::size_of::<WebPDemuxer>() as libc::c_ulong,
    ) as *mut WebPDemuxer;
    if dmux.is_null() {
        return 0 as *mut WebPDemuxer;
    }
    InitDemux(dmux, &mut mem);
    status = PARSE_ERROR;
    parser = kMasterChunks.as_ptr();
    while ((*parser).parse).is_some() {
        if memcmp(
            ((*parser).id).as_ptr() as *const libc::c_void,
            GetBuffer(&mut (*dmux).mem_) as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            status = ((*parser).parse).expect("non-null function pointer")(dmux);
            if status as libc::c_uint == PARSE_OK as libc::c_int as libc::c_uint {
                (*dmux).state_ = WEBP_DEMUX_DONE;
            }
            if status as libc::c_uint
                == PARSE_NEED_MORE_DATA as libc::c_int as libc::c_uint && partial == 0
            {
                status = PARSE_ERROR;
            }
            if status as libc::c_uint != PARSE_ERROR as libc::c_int as libc::c_uint
                && ((*parser).valid).expect("non-null function pointer")(dmux) == 0
            {
                status = PARSE_ERROR;
            }
            if status as libc::c_uint == PARSE_ERROR as libc::c_int as libc::c_uint {
                (*dmux).state_ = WEBP_DEMUX_PARSE_ERROR;
            }
            break;
        } else {
            parser = parser.offset(1);
            parser;
        }
    }
    if !state.is_null() {
        *state = (*dmux).state_;
    }
    if status as libc::c_uint == PARSE_ERROR as libc::c_int as libc::c_uint {
        WebPDemuxDelete(dmux);
        return 0 as *mut WebPDemuxer;
    }
    return dmux;
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxDelete(mut dmux: *mut WebPDemuxer) {
    let mut c: *mut Chunk = 0 as *mut Chunk;
    let mut f: *mut Frame = 0 as *mut Frame;
    if dmux.is_null() {
        return;
    }
    f = (*dmux).frames_;
    while !f.is_null() {
        let cur_frame: *mut Frame = f;
        f = (*f).next_;
        WebPSafeFree(cur_frame as *mut libc::c_void);
    }
    c = (*dmux).chunks_;
    while !c.is_null() {
        let cur_chunk: *mut Chunk = c;
        c = (*c).next_;
        WebPSafeFree(cur_chunk as *mut libc::c_void);
    }
    WebPSafeFree(dmux as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxGetI(
    mut dmux: *const WebPDemuxer,
    mut feature: WebPFormatFeature,
) -> uint32_t {
    if dmux.is_null() {
        return 0 as libc::c_int as uint32_t;
    }
    match feature as libc::c_uint {
        0 => return (*dmux).feature_flags_,
        1 => return (*dmux).canvas_width_ as uint32_t,
        2 => return (*dmux).canvas_height_ as uint32_t,
        3 => return (*dmux).loop_count_ as uint32_t,
        4 => return (*dmux).bgcolor_,
        5 => return (*dmux).num_frames_ as uint32_t,
        _ => {}
    }
    return 0 as libc::c_int as uint32_t;
}
unsafe extern "C" fn GetFrame(
    dmux: *const WebPDemuxer,
    mut frame_num: libc::c_int,
) -> *const Frame {
    let mut f: *const Frame = 0 as *const Frame;
    f = (*dmux).frames_;
    while !f.is_null() {
        if frame_num == (*f).frame_num_ {
            break;
        }
        f = (*f).next_;
    }
    return f;
}
unsafe extern "C" fn GetFramePayload(
    mem_buf: *const uint8_t,
    frame: *const Frame,
    data_size: *mut size_t,
) -> *const uint8_t {
    *data_size = 0 as libc::c_int as size_t;
    if !frame.is_null() {
        let image: *const ChunkData = ((*frame).img_components_).as_ptr();
        let alpha: *const ChunkData = ((*frame).img_components_)
            .as_ptr()
            .offset(1 as libc::c_int as isize);
        let mut start_offset: size_t = (*image).offset_;
        *data_size = (*image).size_;
        if (*alpha).size_ > 0 as libc::c_int as libc::c_ulong {
            let inter_size: size_t = if (*image).offset_
                > 0 as libc::c_int as libc::c_ulong
            {
                ((*image).offset_)
                    .wrapping_sub(((*alpha).offset_).wrapping_add((*alpha).size_))
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            start_offset = (*alpha).offset_;
            *data_size = (*data_size as libc::c_ulong)
                .wrapping_add(((*alpha).size_).wrapping_add(inter_size)) as size_t
                as size_t;
        }
        return mem_buf.offset(start_offset as isize);
    }
    return 0 as *const uint8_t;
}
unsafe extern "C" fn SynthesizeFrame(
    dmux: *const WebPDemuxer,
    frame: *const Frame,
    iter: *mut WebPIterator,
) -> libc::c_int {
    let mem_buf: *const uint8_t = (*dmux).mem_.buf_;
    let mut payload_size: size_t = 0 as libc::c_int as size_t;
    let payload: *const uint8_t = GetFramePayload(mem_buf, frame, &mut payload_size);
    if payload.is_null() {
        return 0 as libc::c_int;
    }
    (*iter).frame_num = (*frame).frame_num_;
    (*iter).num_frames = (*dmux).num_frames_;
    (*iter).x_offset = (*frame).x_offset_;
    (*iter).y_offset = (*frame).y_offset_;
    (*iter).width = (*frame).width_;
    (*iter).height = (*frame).height_;
    (*iter).has_alpha = (*frame).has_alpha_;
    (*iter).duration = (*frame).duration_;
    (*iter).dispose_method = (*frame).dispose_method_;
    (*iter).blend_method = (*frame).blend_method_;
    (*iter).complete = (*frame).complete_;
    (*iter).fragment.bytes = payload;
    (*iter).fragment.size = payload_size;
    return 1 as libc::c_int;
}
unsafe extern "C" fn SetFrame(
    mut frame_num: libc::c_int,
    iter: *mut WebPIterator,
) -> libc::c_int {
    let mut frame: *const Frame = 0 as *const Frame;
    let dmux: *const WebPDemuxer = (*iter).private_ as *mut WebPDemuxer;
    if dmux.is_null() || frame_num < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if frame_num > (*dmux).num_frames_ {
        return 0 as libc::c_int;
    }
    if frame_num == 0 as libc::c_int {
        frame_num = (*dmux).num_frames_;
    }
    frame = GetFrame(dmux, frame_num);
    if frame.is_null() {
        return 0 as libc::c_int;
    }
    return SynthesizeFrame(dmux, frame, iter);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxGetFrame(
    mut dmux: *const WebPDemuxer,
    mut frame: libc::c_int,
    mut iter: *mut WebPIterator,
) -> libc::c_int {
    if iter.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        iter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPIterator>() as libc::c_ulong,
    );
    (*iter).private_ = dmux as *mut libc::c_void;
    return SetFrame(frame, iter);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxNextFrame(mut iter: *mut WebPIterator) -> libc::c_int {
    if iter.is_null() {
        return 0 as libc::c_int;
    }
    return SetFrame((*iter).frame_num + 1 as libc::c_int, iter);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxPrevFrame(mut iter: *mut WebPIterator) -> libc::c_int {
    if iter.is_null() {
        return 0 as libc::c_int;
    }
    if (*iter).frame_num <= 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    return SetFrame((*iter).frame_num - 1 as libc::c_int, iter);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxReleaseIterator(mut iter: *mut WebPIterator) {}
unsafe extern "C" fn ChunkCount(
    dmux: *const WebPDemuxer,
    mut fourcc: *const libc::c_char,
) -> libc::c_int {
    let mem_buf: *const uint8_t = (*dmux).mem_.buf_;
    let mut c: *const Chunk = 0 as *const Chunk;
    let mut count: libc::c_int = 0 as libc::c_int;
    c = (*dmux).chunks_;
    while !c.is_null() {
        let header: *const uint8_t = mem_buf.offset((*c).data_.offset_ as isize);
        if memcmp(
            header as *const libc::c_void,
            fourcc as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            count += 1;
            count;
        }
        c = (*c).next_;
    }
    return count;
}
unsafe extern "C" fn GetChunk(
    dmux: *const WebPDemuxer,
    mut fourcc: *const libc::c_char,
    mut chunk_num: libc::c_int,
) -> *const Chunk {
    let mem_buf: *const uint8_t = (*dmux).mem_.buf_;
    let mut c: *const Chunk = 0 as *const Chunk;
    let mut count: libc::c_int = 0 as libc::c_int;
    c = (*dmux).chunks_;
    while !c.is_null() {
        let header: *const uint8_t = mem_buf.offset((*c).data_.offset_ as isize);
        if memcmp(
            header as *const libc::c_void,
            fourcc as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            count += 1;
            count;
        }
        if count == chunk_num {
            break;
        }
        c = (*c).next_;
    }
    return c;
}
unsafe extern "C" fn SetChunk(
    mut fourcc: *const libc::c_char,
    mut chunk_num: libc::c_int,
    iter: *mut WebPChunkIterator,
) -> libc::c_int {
    let dmux: *const WebPDemuxer = (*iter).private_ as *mut WebPDemuxer;
    let mut count: libc::c_int = 0;
    if dmux.is_null() || fourcc.is_null() || chunk_num < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    count = ChunkCount(dmux, fourcc);
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if chunk_num == 0 as libc::c_int {
        chunk_num = count;
    }
    if chunk_num <= count {
        let mem_buf: *const uint8_t = (*dmux).mem_.buf_;
        let chunk: *const Chunk = GetChunk(dmux, fourcc, chunk_num);
        (*iter)
            .chunk
            .bytes = mem_buf
            .offset((*chunk).data_.offset_ as isize)
            .offset(8 as libc::c_int as isize);
        (*iter)
            .chunk
            .size = ((*chunk).data_.size_)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong);
        (*iter).num_chunks = count;
        (*iter).chunk_num = chunk_num;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxGetChunk(
    mut dmux: *const WebPDemuxer,
    mut fourcc: *const libc::c_char,
    mut chunk_num: libc::c_int,
    mut iter: *mut WebPChunkIterator,
) -> libc::c_int {
    if iter.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        iter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPChunkIterator>() as libc::c_ulong,
    );
    (*iter).private_ = dmux as *mut libc::c_void;
    return SetChunk(fourcc, chunk_num, iter);
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxNextChunk(
    mut iter: *mut WebPChunkIterator,
) -> libc::c_int {
    if !iter.is_null() {
        let fourcc: *const libc::c_char = ((*iter).chunk.bytes as *const libc::c_char)
            .offset(-(8 as libc::c_int as isize));
        return SetChunk(fourcc, (*iter).chunk_num + 1 as libc::c_int, iter);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxPrevChunk(
    mut iter: *mut WebPChunkIterator,
) -> libc::c_int {
    if !iter.is_null() && (*iter).chunk_num > 1 as libc::c_int {
        let fourcc: *const libc::c_char = ((*iter).chunk.bytes as *const libc::c_char)
            .offset(-(8 as libc::c_int as isize));
        return SetChunk(fourcc, (*iter).chunk_num - 1 as libc::c_int, iter);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPDemuxReleaseChunkIterator(
    mut iter: *mut WebPChunkIterator,
) {}
