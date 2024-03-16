#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod sharpyuv {
pub mod sharpyuv;
pub mod sharpyuv_cpu;
pub mod sharpyuv_csp;
pub mod sharpyuv_dsp;
pub mod sharpyuv_gamma;
pub mod sharpyuv_neon;
pub mod sharpyuv_sse2;
} // mod sharpyuv
pub mod src {
pub mod dec {
pub mod alpha_dec;
pub mod buffer_dec;
pub mod frame_dec;
pub mod idec_dec;
pub mod io_dec;
pub mod quant_dec;
pub mod tree_dec;
pub mod vp8_dec;
pub mod vp8l_dec;
pub mod webp_dec;
} // mod dec
pub mod demux {
pub mod anim_decode;
pub mod demux;
} // mod demux
pub mod dsp {
pub mod alpha_processing;
pub mod cost;
pub mod cpu;
pub mod dec;
pub mod dec_clip_tables;
pub mod enc;
pub mod filters;
pub mod lossless;
pub mod lossless_enc;
pub mod rescaler;
pub mod ssim;
pub mod upsampling;
pub mod yuv;
} // mod dsp
pub mod enc {
pub mod alpha_enc;
pub mod analysis_enc;
pub mod backward_references_cost_enc;
pub mod backward_references_enc;
pub mod config_enc;
pub mod cost_enc;
pub mod filter_enc;
pub mod frame_enc;
pub mod histogram_enc;
pub mod iterator_enc;
pub mod near_lossless_enc;
pub mod picture_csp_enc;
pub mod picture_enc;
pub mod picture_psnr_enc;
pub mod picture_rescale_enc;
pub mod picture_tools_enc;
pub mod predictor_enc;
pub mod quant_enc;
pub mod syntax_enc;
pub mod token_enc;
pub mod tree_enc;
pub mod vp8l_enc;
pub mod webp_enc;
} // mod enc
pub mod mux {
pub mod anim_encode;
pub mod muxedit;
pub mod muxinternal;
pub mod muxread;
} // mod mux
pub mod utils {
pub mod bit_reader_utils;
pub mod bit_writer_utils;
pub mod color_cache_utils;
pub mod filters_utils;
pub mod huffman_encode_utils;
pub mod huffman_utils;
pub mod palette;
pub mod quant_levels_dec_utils;
pub mod quant_levels_utils;
pub mod random_utils;
pub mod rescaler_utils;
pub mod thread_utils;
pub mod utils;
pub mod types;
} // mod utils
pub mod api;
} // mod src