use crate::src::{dec::webp_dec::{self, WebPGetFeaturesInternal}, demux::anim_decode, enc::{config_enc, picture_csp_enc, picture_enc, webp_enc}, mux::{anim_encode, muxedit, muxread}, utils::{utils,types}};

pub use webp_dec::{
	WebPDecodeRGBA,
	WebPDecodeRGB,
};
pub use config_enc::WebPValidateConfig;
pub use picture_csp_enc::{
	WebPPictureImportRGBA,
	WebPPictureImportRGB,
};
pub use picture_enc::{
	WebPMemoryWrite,
	WebPPictureFree,
	WebPMemoryWriterInit,
};
pub use webp_enc::WebPEncode;
pub use anim_encode::{
	WebPAnimEncoderOptionsInitInternal,
	WebPAnimEncoderNewInternal,
	WebPAnimEncoderAdd,
	WebPAnimEncoderDelete,
	WebPMuxError,
	WebPAnimEncoderOptions,
	WebPAnimEncoderAssemble,
	WebPAnimEncoderGetError,
};
pub use anim_decode::{
	WebPAnimDecoderOptions,
	WebPAnimDecoderOptionsInitInternal,
	WebPAnimDecoderNewInternal,
	WebPAnimInfo,
	WebPAnimDecoderGetInfo,
	WebPAnimDecoderHasMoreFrames,
	WebPAnimDecoderGetNext,
	WebPAnimDecoderReset,
	WebPAnimDecoderDelete,
};
pub use muxread::WebPMuxCreateInternal;
pub use muxedit::{
	WebPMuxSetAnimationParams,
	WebPMuxAssemble,
	WebPMuxDelete,
};
pub use types::{
	WebPPicture,
	WebPEncodingError,
	WebPConfig,
	WebPBitstreamFeatures,
	WebPData,
	WebPMuxAnimParams,
};
pub use utils::WebPFree;

pub const WEBP_CSP_MODE_MODE_RGBA:u32 =anim_decode::MODE_RGBA;
pub const WEBP_CSP_MODE_MODE_RGB:u32 =anim_decode::MODE_RGB;
pub const VP8_ENC_ERROR_INVALID_CONFIGURATION:WebPEncodingError=types::VP8_ENC_ERROR_INVALID_CONFIGURATION;
pub const VP8_STATUS_OK:i32=0;
pub const WEBP_MUX_OK:i32=muxedit::WEBP_MUX_OK;
pub const WEBP_DECODER_ABI_VERSION:i32=521;

pub fn WebPGetMuxABIVersion()->i32{
	264
}
pub fn WebPGetDemuxABIVersion()->i32{
	263
}

pub fn WebPDataInit(data: &mut WebPData) {
    *data = WebPData {
        bytes: core::ptr::null_mut(),
        size: 0,
    }
}
pub unsafe fn WebPDataClear(data: &mut WebPData) {
    WebPFree(data.bytes as *mut _);
    WebPDataInit(data);
}
pub unsafe fn WebPGetFeatures(
    arg1: *const u8,
    arg2: usize,
    arg3: *mut WebPBitstreamFeatures,
) -> i32 {
    WebPGetFeaturesInternal(arg1, arg2 as _, arg3, WEBP_DECODER_ABI_VERSION) as _
}
pub fn new_webp_config()->Result<WebPConfig, ()>{
	unsafe{
		let mut out = core::mem::MaybeUninit::uninit();
		if config_enc::WebPConfigInitInternal(
			out.as_mut_ptr(),
			config_enc::WEBP_PRESET_DEFAULT,
			75.0,
			WEBP_DECODER_ABI_VERSION,
		) != 0{
			Ok(out.assume_init())
		} else {
			Err(())
		}
	}
}
pub fn new_webp_picture()->Result<WebPPicture, ()>{
	unsafe{
        let mut out = core::mem::MaybeUninit::uninit();
        if picture_enc::WebPPictureInitInternal(out.as_mut_ptr(),521) != 0{
            Ok(out.assume_init())
        } else {
            Err(())
        }
	}
}
