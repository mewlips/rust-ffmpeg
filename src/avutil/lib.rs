#[crate_id = "avutil#54"];
#[license = "MIT"];
#[crate_type = "lib"];
/* automatically generated by rust-bindgen */
use std::libc::{c_void,c_int,c_uint,c_schar,size_t,uint8_t,int64_t,c_double,uint64_t,c_float};

pub type __int128_t = c_void;
pub type __uint128_t = c_void;
pub type __builtin_va_list = [__va_list_tag, ..1u];
pub type va_list = __builtin_va_list;
pub type Enum_AVMediaType = c_int;
pub static AVMEDIA_TYPE_UNKNOWN: c_int = -1;
pub static AVMEDIA_TYPE_VIDEO: c_int = 0;
pub static AVMEDIA_TYPE_AUDIO: c_int = 1;
pub static AVMEDIA_TYPE_DATA: c_int = 2;
pub static AVMEDIA_TYPE_SUBTITLE: c_int = 3;
pub static AVMEDIA_TYPE_ATTACHMENT: c_int = 4;
pub static AVMEDIA_TYPE_NB: c_int = 5;
pub type Enum_AVPictureType = c_uint;
pub static AV_PICTURE_TYPE_NONE: c_uint = 0;
pub static AV_PICTURE_TYPE_I: c_uint = 1;
pub static AV_PICTURE_TYPE_P: c_uint = 2;
pub static AV_PICTURE_TYPE_B: c_uint = 3;
pub static AV_PICTURE_TYPE_S: c_uint = 4;
pub static AV_PICTURE_TYPE_SI: c_uint = 5;
pub static AV_PICTURE_TYPE_SP: c_uint = 6;
pub static AV_PICTURE_TYPE_BI: c_uint = 7;
#[deriving(Clone)]
pub struct Struct_AVRational {
    num: c_int,
    den: c_int,
}
pub type AVRational = Struct_AVRational;
pub type Enum_AVRounding = c_uint;
pub static AV_ROUND_ZERO: c_uint = 0;
pub static AV_ROUND_INF: c_uint = 1;
pub static AV_ROUND_DOWN: c_uint = 2;
pub static AV_ROUND_UP: c_uint = 3;
pub static AV_ROUND_NEAR_INF: c_uint = 5;
pub static AV_ROUND_PASS_MINMAX: c_uint = 8192;
pub type AVClassCategory = c_uint;
pub static AV_CLASS_CATEGORY_NA: c_uint = 0;
pub static AV_CLASS_CATEGORY_INPUT: c_uint = 1;
pub static AV_CLASS_CATEGORY_OUTPUT: c_uint = 2;
pub static AV_CLASS_CATEGORY_MUXER: c_uint = 3;
pub static AV_CLASS_CATEGORY_DEMUXER: c_uint = 4;
pub static AV_CLASS_CATEGORY_ENCODER: c_uint = 5;
pub static AV_CLASS_CATEGORY_DECODER: c_uint = 6;
pub static AV_CLASS_CATEGORY_FILTER: c_uint = 7;
pub static AV_CLASS_CATEGORY_BITSTREAM_FILTER: c_uint = 8;
pub static AV_CLASS_CATEGORY_SWSCALER: c_uint = 9;
pub static AV_CLASS_CATEGORY_SWRESAMPLER: c_uint = 10;
pub static AV_CLASS_CATEGORY_NB: c_uint = 11;
pub struct Struct_AVClass {
    class_name: *c_schar,
    item_name: extern "C" fn(arg1: *mut c_void) -> *c_schar,
    option: *Struct_AVOption,
    version: c_int,
    log_level_offset_offset: c_int,
    parent_log_context_offset: c_int,
    child_next: extern "C" fn(arg1: *mut c_void, arg2: *mut c_void)
                    -> *mut c_void,
    child_class_next: extern "C" fn(arg1: *Struct_AVClass) -> *Struct_AVClass,
    category: AVClassCategory,
    get_category: extern "C" fn(arg1: *mut c_void) -> AVClassCategory,
    query_ranges: extern "C" fn
                      (arg1: *mut *mut Struct_AVOptionRanges,
                       arg2: *mut c_void, arg3: *c_schar, arg4: c_int)
                      -> c_int,
}
pub type AVClass = Struct_AVClass;
pub type Enum_AVPixelFormat = c_int;
pub static AV_PIX_FMT_NONE: c_int = -1;
pub static AV_PIX_FMT_YUV420P: c_int = 0;
pub static AV_PIX_FMT_YUYV422: c_int = 1;
pub static AV_PIX_FMT_RGB24: c_int = 2;
pub static AV_PIX_FMT_BGR24: c_int = 3;
pub static AV_PIX_FMT_YUV422P: c_int = 4;
pub static AV_PIX_FMT_YUV444P: c_int = 5;
pub static AV_PIX_FMT_YUV410P: c_int = 6;
pub static AV_PIX_FMT_YUV411P: c_int = 7;
pub static AV_PIX_FMT_GRAY8: c_int = 8;
pub static AV_PIX_FMT_MONOWHITE: c_int = 9;
pub static AV_PIX_FMT_MONOBLACK: c_int = 10;
pub static AV_PIX_FMT_PAL8: c_int = 11;
pub static AV_PIX_FMT_YUVJ420P: c_int = 12;
pub static AV_PIX_FMT_YUVJ422P: c_int = 13;
pub static AV_PIX_FMT_YUVJ444P: c_int = 14;
pub static AV_PIX_FMT_XVMC_MPEG2_MC: c_int = 15;
pub static AV_PIX_FMT_XVMC_MPEG2_IDCT: c_int = 16;
pub static AV_PIX_FMT_UYVY422: c_int = 17;
pub static AV_PIX_FMT_UYYVYY411: c_int = 18;
pub static AV_PIX_FMT_BGR8: c_int = 19;
pub static AV_PIX_FMT_BGR4: c_int = 20;
pub static AV_PIX_FMT_BGR4_BYTE: c_int = 21;
pub static AV_PIX_FMT_RGB8: c_int = 22;
pub static AV_PIX_FMT_RGB4: c_int = 23;
pub static AV_PIX_FMT_RGB4_BYTE: c_int = 24;
pub static AV_PIX_FMT_NV12: c_int = 25;
pub static AV_PIX_FMT_NV21: c_int = 26;
pub static AV_PIX_FMT_ARGB: c_int = 27;
pub static AV_PIX_FMT_RGBA: c_int = 28;
pub static AV_PIX_FMT_ABGR: c_int = 29;
pub static AV_PIX_FMT_BGRA: c_int = 30;
pub static AV_PIX_FMT_GRAY16BE: c_int = 31;
pub static AV_PIX_FMT_GRAY16LE: c_int = 32;
pub static AV_PIX_FMT_YUV440P: c_int = 33;
pub static AV_PIX_FMT_YUVJ440P: c_int = 34;
pub static AV_PIX_FMT_YUVA420P: c_int = 35;
pub static AV_PIX_FMT_VDPAU_H264: c_int = 36;
pub static AV_PIX_FMT_VDPAU_MPEG1: c_int = 37;
pub static AV_PIX_FMT_VDPAU_MPEG2: c_int = 38;
pub static AV_PIX_FMT_VDPAU_WMV3: c_int = 39;
pub static AV_PIX_FMT_VDPAU_VC1: c_int = 40;
pub static AV_PIX_FMT_RGB48BE: c_int = 41;
pub static AV_PIX_FMT_RGB48LE: c_int = 42;
pub static AV_PIX_FMT_RGB565BE: c_int = 43;
pub static AV_PIX_FMT_RGB565LE: c_int = 44;
pub static AV_PIX_FMT_RGB555BE: c_int = 45;
pub static AV_PIX_FMT_RGB555LE: c_int = 46;
pub static AV_PIX_FMT_BGR565BE: c_int = 47;
pub static AV_PIX_FMT_BGR565LE: c_int = 48;
pub static AV_PIX_FMT_BGR555BE: c_int = 49;
pub static AV_PIX_FMT_BGR555LE: c_int = 50;
pub static AV_PIX_FMT_VAAPI_MOCO: c_int = 51;
pub static AV_PIX_FMT_VAAPI_IDCT: c_int = 52;
pub static AV_PIX_FMT_VAAPI_VLD: c_int = 53;
pub static AV_PIX_FMT_YUV420P16LE: c_int = 54;
pub static AV_PIX_FMT_YUV420P16BE: c_int = 55;
pub static AV_PIX_FMT_YUV422P16LE: c_int = 56;
pub static AV_PIX_FMT_YUV422P16BE: c_int = 57;
pub static AV_PIX_FMT_YUV444P16LE: c_int = 58;
pub static AV_PIX_FMT_YUV444P16BE: c_int = 59;
pub static AV_PIX_FMT_VDPAU_MPEG4: c_int = 60;
pub static AV_PIX_FMT_DXVA2_VLD: c_int = 61;
pub static AV_PIX_FMT_RGB444LE: c_int = 62;
pub static AV_PIX_FMT_RGB444BE: c_int = 63;
pub static AV_PIX_FMT_BGR444LE: c_int = 64;
pub static AV_PIX_FMT_BGR444BE: c_int = 65;
pub static AV_PIX_FMT_GRAY8A: c_int = 66;
pub static AV_PIX_FMT_BGR48BE: c_int = 67;
pub static AV_PIX_FMT_BGR48LE: c_int = 68;
pub static AV_PIX_FMT_YUV420P9BE: c_int = 69;
pub static AV_PIX_FMT_YUV420P9LE: c_int = 70;
pub static AV_PIX_FMT_YUV420P10BE: c_int = 71;
pub static AV_PIX_FMT_YUV420P10LE: c_int = 72;
pub static AV_PIX_FMT_YUV422P10BE: c_int = 73;
pub static AV_PIX_FMT_YUV422P10LE: c_int = 74;
pub static AV_PIX_FMT_YUV444P9BE: c_int = 75;
pub static AV_PIX_FMT_YUV444P9LE: c_int = 76;
pub static AV_PIX_FMT_YUV444P10BE: c_int = 77;
pub static AV_PIX_FMT_YUV444P10LE: c_int = 78;
pub static AV_PIX_FMT_YUV422P9BE: c_int = 79;
pub static AV_PIX_FMT_YUV422P9LE: c_int = 80;
pub static AV_PIX_FMT_VDA_VLD: c_int = 81;
pub static AV_PIX_FMT_GBRP: c_int = 82;
pub static AV_PIX_FMT_GBRP9BE: c_int = 83;
pub static AV_PIX_FMT_GBRP9LE: c_int = 84;
pub static AV_PIX_FMT_GBRP10BE: c_int = 85;
pub static AV_PIX_FMT_GBRP10LE: c_int = 86;
pub static AV_PIX_FMT_GBRP16BE: c_int = 87;
pub static AV_PIX_FMT_GBRP16LE: c_int = 88;
pub static AV_PIX_FMT_YUVA422P_LIBAV: c_int = 89;
pub static AV_PIX_FMT_YUVA444P_LIBAV: c_int = 90;
pub static AV_PIX_FMT_YUVA420P9BE: c_int = 91;
pub static AV_PIX_FMT_YUVA420P9LE: c_int = 92;
pub static AV_PIX_FMT_YUVA422P9BE: c_int = 93;
pub static AV_PIX_FMT_YUVA422P9LE: c_int = 94;
pub static AV_PIX_FMT_YUVA444P9BE: c_int = 95;
pub static AV_PIX_FMT_YUVA444P9LE: c_int = 96;
pub static AV_PIX_FMT_YUVA420P10BE: c_int = 97;
pub static AV_PIX_FMT_YUVA420P10LE: c_int = 98;
pub static AV_PIX_FMT_YUVA422P10BE: c_int = 99;
pub static AV_PIX_FMT_YUVA422P10LE: c_int = 100;
pub static AV_PIX_FMT_YUVA444P10BE: c_int = 101;
pub static AV_PIX_FMT_YUVA444P10LE: c_int = 102;
pub static AV_PIX_FMT_YUVA420P16BE: c_int = 103;
pub static AV_PIX_FMT_YUVA420P16LE: c_int = 104;
pub static AV_PIX_FMT_YUVA422P16BE: c_int = 105;
pub static AV_PIX_FMT_YUVA422P16LE: c_int = 106;
pub static AV_PIX_FMT_YUVA444P16BE: c_int = 107;
pub static AV_PIX_FMT_YUVA444P16LE: c_int = 108;
pub static AV_PIX_FMT_VDPAU: c_int = 109;
pub static AV_PIX_FMT_RGBA64BE: c_int = 291;
pub static AV_PIX_FMT_RGBA64LE: c_int = 292;
pub static AV_PIX_FMT_BGRA64BE: c_int = 293;
pub static AV_PIX_FMT_BGRA64LE: c_int = 294;
pub static AV_PIX_FMT_0RGB: c_int = 295;
pub static AV_PIX_FMT_RGB0: c_int = 296;
pub static AV_PIX_FMT_0BGR: c_int = 297;
pub static AV_PIX_FMT_BGR0: c_int = 298;
pub static AV_PIX_FMT_YUVA444P: c_int = 299;
pub static AV_PIX_FMT_YUVA422P: c_int = 300;
pub static AV_PIX_FMT_YUV420P12BE: c_int = 301;
pub static AV_PIX_FMT_YUV420P12LE: c_int = 302;
pub static AV_PIX_FMT_YUV420P14BE: c_int = 303;
pub static AV_PIX_FMT_YUV420P14LE: c_int = 304;
pub static AV_PIX_FMT_YUV422P12BE: c_int = 305;
pub static AV_PIX_FMT_YUV422P12LE: c_int = 306;
pub static AV_PIX_FMT_YUV422P14BE: c_int = 307;
pub static AV_PIX_FMT_YUV422P14LE: c_int = 308;
pub static AV_PIX_FMT_YUV444P12BE: c_int = 309;
pub static AV_PIX_FMT_YUV444P12LE: c_int = 310;
pub static AV_PIX_FMT_YUV444P14BE: c_int = 311;
pub static AV_PIX_FMT_YUV444P14LE: c_int = 312;
pub static AV_PIX_FMT_GBRP12BE: c_int = 313;
pub static AV_PIX_FMT_GBRP12LE: c_int = 314;
pub static AV_PIX_FMT_GBRP14BE: c_int = 315;
pub static AV_PIX_FMT_GBRP14LE: c_int = 316;
pub static AV_PIX_FMT_NB: c_int = 317;
pub static PIX_FMT_NONE: c_int = -1;
pub static PIX_FMT_YUV420P: c_int = 0;
pub static PIX_FMT_YUYV422: c_int = 1;
pub static PIX_FMT_RGB24: c_int = 2;
pub static PIX_FMT_BGR24: c_int = 3;
pub static PIX_FMT_YUV422P: c_int = 4;
pub static PIX_FMT_YUV444P: c_int = 5;
pub static PIX_FMT_YUV410P: c_int = 6;
pub static PIX_FMT_YUV411P: c_int = 7;
pub static PIX_FMT_GRAY8: c_int = 8;
pub static PIX_FMT_MONOWHITE: c_int = 9;
pub static PIX_FMT_MONOBLACK: c_int = 10;
pub static PIX_FMT_PAL8: c_int = 11;
pub static PIX_FMT_YUVJ420P: c_int = 12;
pub static PIX_FMT_YUVJ422P: c_int = 13;
pub static PIX_FMT_YUVJ444P: c_int = 14;
pub static PIX_FMT_XVMC_MPEG2_MC: c_int = 15;
pub static PIX_FMT_XVMC_MPEG2_IDCT: c_int = 16;
pub static PIX_FMT_UYVY422: c_int = 17;
pub static PIX_FMT_UYYVYY411: c_int = 18;
pub static PIX_FMT_BGR8: c_int = 19;
pub static PIX_FMT_BGR4: c_int = 20;
pub static PIX_FMT_BGR4_BYTE: c_int = 21;
pub static PIX_FMT_RGB8: c_int = 22;
pub static PIX_FMT_RGB4: c_int = 23;
pub static PIX_FMT_RGB4_BYTE: c_int = 24;
pub static PIX_FMT_NV12: c_int = 25;
pub static PIX_FMT_NV21: c_int = 26;
pub static PIX_FMT_ARGB: c_int = 27;
pub static PIX_FMT_RGBA: c_int = 28;
pub static PIX_FMT_ABGR: c_int = 29;
pub static PIX_FMT_BGRA: c_int = 30;
pub static PIX_FMT_GRAY16BE: c_int = 31;
pub static PIX_FMT_GRAY16LE: c_int = 32;
pub static PIX_FMT_YUV440P: c_int = 33;
pub static PIX_FMT_YUVJ440P: c_int = 34;
pub static PIX_FMT_YUVA420P: c_int = 35;
pub static PIX_FMT_VDPAU_H264: c_int = 36;
pub static PIX_FMT_VDPAU_MPEG1: c_int = 37;
pub static PIX_FMT_VDPAU_MPEG2: c_int = 38;
pub static PIX_FMT_VDPAU_WMV3: c_int = 39;
pub static PIX_FMT_VDPAU_VC1: c_int = 40;
pub static PIX_FMT_RGB48BE: c_int = 41;
pub static PIX_FMT_RGB48LE: c_int = 42;
pub static PIX_FMT_RGB565BE: c_int = 43;
pub static PIX_FMT_RGB565LE: c_int = 44;
pub static PIX_FMT_RGB555BE: c_int = 45;
pub static PIX_FMT_RGB555LE: c_int = 46;
pub static PIX_FMT_BGR565BE: c_int = 47;
pub static PIX_FMT_BGR565LE: c_int = 48;
pub static PIX_FMT_BGR555BE: c_int = 49;
pub static PIX_FMT_BGR555LE: c_int = 50;
pub static PIX_FMT_VAAPI_MOCO: c_int = 51;
pub static PIX_FMT_VAAPI_IDCT: c_int = 52;
pub static PIX_FMT_VAAPI_VLD: c_int = 53;
pub static PIX_FMT_YUV420P16LE: c_int = 54;
pub static PIX_FMT_YUV420P16BE: c_int = 55;
pub static PIX_FMT_YUV422P16LE: c_int = 56;
pub static PIX_FMT_YUV422P16BE: c_int = 57;
pub static PIX_FMT_YUV444P16LE: c_int = 58;
pub static PIX_FMT_YUV444P16BE: c_int = 59;
pub static PIX_FMT_VDPAU_MPEG4: c_int = 60;
pub static PIX_FMT_DXVA2_VLD: c_int = 61;
pub static PIX_FMT_RGB444LE: c_int = 62;
pub static PIX_FMT_RGB444BE: c_int = 63;
pub static PIX_FMT_BGR444LE: c_int = 64;
pub static PIX_FMT_BGR444BE: c_int = 65;
pub static PIX_FMT_GRAY8A: c_int = 66;
pub static PIX_FMT_BGR48BE: c_int = 67;
pub static PIX_FMT_BGR48LE: c_int = 68;
pub static PIX_FMT_YUV420P9BE: c_int = 69;
pub static PIX_FMT_YUV420P9LE: c_int = 70;
pub static PIX_FMT_YUV420P10BE: c_int = 71;
pub static PIX_FMT_YUV420P10LE: c_int = 72;
pub static PIX_FMT_YUV422P10BE: c_int = 73;
pub static PIX_FMT_YUV422P10LE: c_int = 74;
pub static PIX_FMT_YUV444P9BE: c_int = 75;
pub static PIX_FMT_YUV444P9LE: c_int = 76;
pub static PIX_FMT_YUV444P10BE: c_int = 77;
pub static PIX_FMT_YUV444P10LE: c_int = 78;
pub static PIX_FMT_YUV422P9BE: c_int = 79;
pub static PIX_FMT_YUV422P9LE: c_int = 80;
pub static PIX_FMT_VDA_VLD: c_int = 81;
pub static PIX_FMT_GBRP: c_int = 82;
pub static PIX_FMT_GBRP9BE: c_int = 83;
pub static PIX_FMT_GBRP9LE: c_int = 84;
pub static PIX_FMT_GBRP10BE: c_int = 85;
pub static PIX_FMT_GBRP10LE: c_int = 86;
pub static PIX_FMT_GBRP16BE: c_int = 87;
pub static PIX_FMT_GBRP16LE: c_int = 88;
pub static PIX_FMT_RGBA64BE: c_int = 291;
pub static PIX_FMT_RGBA64LE: c_int = 292;
pub static PIX_FMT_BGRA64BE: c_int = 293;
pub static PIX_FMT_BGRA64LE: c_int = 294;
pub static PIX_FMT_0RGB: c_int = 295;
pub static PIX_FMT_RGB0: c_int = 296;
pub static PIX_FMT_0BGR: c_int = 297;
pub static PIX_FMT_BGR0: c_int = 298;
pub static PIX_FMT_YUVA444P: c_int = 299;
pub static PIX_FMT_YUVA422P: c_int = 300;
pub static PIX_FMT_YUV420P12BE: c_int = 301;
pub static PIX_FMT_YUV420P12LE: c_int = 302;
pub static PIX_FMT_YUV420P14BE: c_int = 303;
pub static PIX_FMT_YUV420P14LE: c_int = 304;
pub static PIX_FMT_YUV422P12BE: c_int = 305;
pub static PIX_FMT_YUV422P12LE: c_int = 306;
pub static PIX_FMT_YUV422P14BE: c_int = 307;
pub static PIX_FMT_YUV422P14LE: c_int = 308;
pub static PIX_FMT_YUV444P12BE: c_int = 309;
pub static PIX_FMT_YUV444P12LE: c_int = 310;
pub static PIX_FMT_YUV444P14BE: c_int = 311;
pub static PIX_FMT_YUV444P14LE: c_int = 312;
pub static PIX_FMT_GBRP12BE: c_int = 313;
pub static PIX_FMT_GBRP12LE: c_int = 314;
pub static PIX_FMT_GBRP14BE: c_int = 315;
pub static PIX_FMT_GBRP14LE: c_int = 316;
pub static PIX_FMT_NB: c_int = 317;
pub type Enum_AVSampleFormat = c_int;
pub static AV_SAMPLE_FMT_NONE: c_int = -1;
pub static AV_SAMPLE_FMT_U8: c_int = 0;
pub static AV_SAMPLE_FMT_S16: c_int = 1;
pub static AV_SAMPLE_FMT_S32: c_int = 2;
pub static AV_SAMPLE_FMT_FLT: c_int = 3;
pub static AV_SAMPLE_FMT_DBL: c_int = 4;
pub static AV_SAMPLE_FMT_U8P: c_int = 5;
pub static AV_SAMPLE_FMT_S16P: c_int = 6;
pub static AV_SAMPLE_FMT_S32P: c_int = 7;
pub static AV_SAMPLE_FMT_FLTP: c_int = 8;
pub static AV_SAMPLE_FMT_DBLP: c_int = 9;
pub static AV_SAMPLE_FMT_NB: c_int = 10;
pub struct Struct_AVDictionaryEntry {
    key: *mut c_schar,
    value: *mut c_schar,
}
pub type AVDictionaryEntry = Struct_AVDictionaryEntry;
pub type Struct_AVDictionary = c_void;
pub type AVDictionary = Struct_AVDictionary;
pub type Enum_AVMatrixEncoding = c_uint;
pub static AV_MATRIX_ENCODING_NONE: c_uint = 0;
pub static AV_MATRIX_ENCODING_DOLBY: c_uint = 1;
pub static AV_MATRIX_ENCODING_DPLII: c_uint = 2;
pub static AV_MATRIX_ENCODING_NB: c_uint = 3;
pub type Struct_AVBPrint = c_void;
pub type Enum_AVOptionType = c_uint;
pub static AV_OPT_TYPE_FLAGS: c_uint = 0;
pub static AV_OPT_TYPE_INT: c_uint = 1;
pub static AV_OPT_TYPE_INT64: c_uint = 2;
pub static AV_OPT_TYPE_DOUBLE: c_uint = 3;
pub static AV_OPT_TYPE_FLOAT: c_uint = 4;
pub static AV_OPT_TYPE_STRING: c_uint = 5;
pub static AV_OPT_TYPE_RATIONAL: c_uint = 6;
pub static AV_OPT_TYPE_BINARY: c_uint = 7;
pub static AV_OPT_TYPE_CONST: c_uint = 128;
pub static AV_OPT_TYPE_IMAGE_SIZE: c_uint = 1397316165;
pub static AV_OPT_TYPE_PIXEL_FMT: c_uint = 1346784596;
pub static AV_OPT_TYPE_SAMPLE_FMT: c_uint = 1397116244;
pub static FF_OPT_TYPE_FLAGS: c_uint = 0;
pub static FF_OPT_TYPE_INT: c_uint = 1;
pub static FF_OPT_TYPE_INT64: c_uint = 2;
pub static FF_OPT_TYPE_DOUBLE: c_uint = 3;
pub static FF_OPT_TYPE_FLOAT: c_uint = 4;
pub static FF_OPT_TYPE_STRING: c_uint = 5;
pub static FF_OPT_TYPE_RATIONAL: c_uint = 6;
pub static FF_OPT_TYPE_BINARY: c_uint = 7;
pub static FF_OPT_TYPE_CONST: c_uint = 128;
pub struct Struct_AVOption {
    name: *c_schar,
    help: *c_schar,
    offset: c_int,
    _type: Enum_AVOptionType,
    default_val: Union_Unnamed1,
    min: c_double,
    max: c_double,
    flags: c_int,
    unit: *c_schar,
}
pub struct Union_Unnamed1 {
    data: [u64, ..1u],
}
impl Union_Unnamed1 {
    pub fn i64(&mut self) -> *mut int64_t {
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    }
    pub fn dbl(&mut self) -> *mut c_double {
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    }
    pub fn str(&mut self) -> *mut *c_schar {
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    }
    pub fn q(&mut self) -> *mut AVRational {
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    }
}
pub type AVOption = Struct_AVOption;
pub struct Struct_AVOptionRange {
    _str: *c_schar,
    value_min: c_double,
    value_max: c_double,
    component_min: c_double,
    component_max: c_double,
    is_range: c_int,
}
pub type AVOptionRange = Struct_AVOptionRange;
pub struct Struct_AVOptionRanges {
    range: *mut *mut AVOptionRange,
    nb_ranges: c_int,
}
pub type AVOptionRanges = Struct_AVOptionRanges;
pub type Enum_Unnamed2 = c_uint;
pub static AV_OPT_FLAG_IMPLICIT_KEY: c_uint = 1;
pub type __va_list_tag = Struct___va_list_tag;
pub struct Struct___va_list_tag {
    gp_offset: c_uint,
    fp_offset: c_uint,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
}
pub static AV_CH_FRONT_LEFT              : uint64_t = 0x00000001;
pub static AV_CH_FRONT_RIGHT             : uint64_t = 0x00000002;
pub static AV_CH_FRONT_CENTER            : uint64_t = 0x00000004;
pub static AV_CH_LOW_FREQUENCY           : uint64_t = 0x00000008;
pub static AV_CH_BACK_LEFT               : uint64_t = 0x00000010;
pub static AV_CH_BACK_RIGHT              : uint64_t = 0x00000020;
pub static AV_CH_FRONT_LEFT_OF_CENTER    : uint64_t = 0x00000040;
pub static AV_CH_FRONT_RIGHT_OF_CENTER   : uint64_t = 0x00000080;
pub static AV_CH_BACK_CENTER             : uint64_t = 0x00000100;
pub static AV_CH_SIDE_LEFT               : uint64_t = 0x00000200;
pub static AV_CH_SIDE_RIGHT              : uint64_t = 0x00000400;
pub static AV_CH_TOP_CENTER              : uint64_t = 0x00000800;
pub static AV_CH_TOP_FRONT_LEFT          : uint64_t = 0x00001000;
pub static AV_CH_TOP_FRONT_CENTER        : uint64_t = 0x00002000;
pub static AV_CH_TOP_FRONT_RIGHT         : uint64_t = 0x00004000;
pub static AV_CH_TOP_BACK_LEFT           : uint64_t = 0x00008000;
pub static AV_CH_TOP_BACK_CENTER         : uint64_t = 0x00010000;
pub static AV_CH_TOP_BACK_RIGHT          : uint64_t = 0x00020000;
pub static AV_CH_STEREO_LEFT             : uint64_t = 0x20000000;
pub static AV_CH_STEREO_RIGHT            : uint64_t = 0x40000000;
pub static AV_CH_WIDE_LEFT               : uint64_t = 0x0000000080000000u64;
pub static AV_CH_WIDE_RIGHT              : uint64_t = 0x0000000100000000u64;
pub static AV_CH_SURROUND_DIRECT_LEFT    : uint64_t = 0x0000000200000000u64;
pub static AV_CH_SURROUND_DIRECT_RIGHT   : uint64_t = 0x0000000400000000u64;
pub static AV_CH_LOW_FREQUENCY_2         : uint64_t = 0x0000000800000000u64;
pub static AV_CH_LAYOUT_NATIVE           : uint64_t = 0x8000000000000000u64;
pub static AV_CH_LAYOUT_MONO             : uint64_t = (AV_CH_FRONT_CENTER);
pub static AV_CH_LAYOUT_STEREO           : uint64_t = (AV_CH_FRONT_LEFT|AV_CH_FRONT_RIGHT);
pub static AV_CH_LAYOUT_2POINT1          : uint64_t = (AV_CH_LAYOUT_STEREO|AV_CH_LOW_FREQUENCY);
pub static AV_CH_LAYOUT_2_1              : uint64_t = (AV_CH_LAYOUT_STEREO|AV_CH_BACK_CENTER);
pub static AV_CH_LAYOUT_SURROUND         : uint64_t = (AV_CH_LAYOUT_STEREO|AV_CH_FRONT_CENTER);
pub static AV_CH_LAYOUT_3POINT1          : uint64_t = (AV_CH_LAYOUT_SURROUND|AV_CH_LOW_FREQUENCY);
pub static AV_CH_LAYOUT_4POINT0          : uint64_t = (AV_CH_LAYOUT_SURROUND|AV_CH_BACK_CENTER);
pub static AV_CH_LAYOUT_4POINT1          : uint64_t = (AV_CH_LAYOUT_4POINT0|AV_CH_LOW_FREQUENCY);
pub static AV_CH_LAYOUT_2_2              : uint64_t = (AV_CH_LAYOUT_STEREO|AV_CH_SIDE_LEFT|AV_CH_SIDE_RIGHT);
pub static AV_CH_LAYOUT_QUAD             : uint64_t = (AV_CH_LAYOUT_STEREO|AV_CH_BACK_LEFT|AV_CH_BACK_RIGHT);
pub static AV_CH_LAYOUT_5POINT0          : uint64_t = (AV_CH_LAYOUT_SURROUND|AV_CH_SIDE_LEFT|AV_CH_SIDE_RIGHT);
pub static AV_CH_LAYOUT_5POINT1          : uint64_t = (AV_CH_LAYOUT_5POINT0|AV_CH_LOW_FREQUENCY);
pub static AV_CH_LAYOUT_5POINT0_BACK     : uint64_t = (AV_CH_LAYOUT_SURROUND|AV_CH_BACK_LEFT|AV_CH_BACK_RIGHT);
pub static AV_CH_LAYOUT_5POINT1_BACK     : uint64_t = (AV_CH_LAYOUT_5POINT0_BACK|AV_CH_LOW_FREQUENCY);
pub static AV_CH_LAYOUT_6POINT0          : uint64_t = (AV_CH_LAYOUT_5POINT0|AV_CH_BACK_CENTER);
pub static AV_CH_LAYOUT_6POINT0_FRONT    : uint64_t = (AV_CH_LAYOUT_2_2|AV_CH_FRONT_LEFT_OF_CENTER|AV_CH_FRONT_RIGHT_OF_CENTER);
pub static AV_CH_LAYOUT_HEXAGONAL        : uint64_t = (AV_CH_LAYOUT_5POINT0_BACK|AV_CH_BACK_CENTER);
pub static AV_CH_LAYOUT_6POINT1          : uint64_t = (AV_CH_LAYOUT_5POINT1|AV_CH_BACK_CENTER);
pub static AV_CH_LAYOUT_6POINT1_BACK     : uint64_t = (AV_CH_LAYOUT_5POINT1_BACK|AV_CH_BACK_CENTER);
pub static AV_CH_LAYOUT_6POINT1_FRONT    : uint64_t = (AV_CH_LAYOUT_6POINT0_FRONT|AV_CH_LOW_FREQUENCY);
pub static AV_CH_LAYOUT_7POINT0          : uint64_t = (AV_CH_LAYOUT_5POINT0|AV_CH_BACK_LEFT|AV_CH_BACK_RIGHT);
pub static AV_CH_LAYOUT_7POINT0_FRONT    : uint64_t = (AV_CH_LAYOUT_5POINT0|AV_CH_FRONT_LEFT_OF_CENTER|AV_CH_FRONT_RIGHT_OF_CENTER);
pub static AV_CH_LAYOUT_7POINT1          : uint64_t = (AV_CH_LAYOUT_5POINT1|AV_CH_BACK_LEFT|AV_CH_BACK_RIGHT);
pub static AV_CH_LAYOUT_7POINT1_WIDE     : uint64_t = (AV_CH_LAYOUT_5POINT1|AV_CH_FRONT_LEFT_OF_CENTER|AV_CH_FRONT_RIGHT_OF_CENTER);
pub static AV_CH_LAYOUT_7POINT1_WIDE_BACK: uint64_t = (AV_CH_LAYOUT_5POINT1_BACK|AV_CH_FRONT_LEFT_OF_CENTER|AV_CH_FRONT_RIGHT_OF_CENTER);
pub static AV_CH_LAYOUT_OCTAGONAL        : uint64_t = (AV_CH_LAYOUT_5POINT0|AV_CH_BACK_LEFT|AV_CH_BACK_CENTER|AV_CH_BACK_RIGHT);
pub static AV_CH_LAYOUT_STEREO_DOWNMIX   : uint64_t = (AV_CH_STEREO_LEFT|AV_CH_STEREO_RIGHT);
#[link(name = "avutil")]
extern "C" {
    pub fn avutil_version() -> c_uint;
    pub fn avutil_configuration() -> *c_schar;
    pub fn avutil_license() -> *c_schar;
    pub fn av_get_media_type_string(media_type: Enum_AVMediaType) -> *c_schar;
    pub fn av_get_picture_type_char(pict_type: Enum_AVPictureType) -> c_schar;
    pub fn av_strerror(errnum: c_int, errbuf: *mut c_schar,
                       errbuf_size: size_t) -> c_int;
    pub fn av_malloc(size: size_t) -> *mut c_void;
    pub fn av_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn av_realloc_f(ptr: *mut c_void, nelem: size_t, elsize: size_t) ->
     *mut c_void;
    pub fn av_free(ptr: *mut c_void);
    pub fn av_mallocz(size: size_t) -> *mut c_void;
    pub fn av_calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn av_strdup(s: *c_schar) -> *mut c_schar;
    pub fn av_freep(ptr: *mut c_void);
    pub fn av_dynarray_add(tab_ptr: *mut c_void, nb_ptr: *mut c_int,
                           elem: *mut c_void);
    pub fn av_max_alloc(max: size_t);
    pub fn av_memcpy_backptr(dst: *mut uint8_t, back: c_int, cnt: c_int);
    pub fn av_reduce(dst_num: *mut c_int, dst_den: *mut c_int, num: int64_t,
                     den: int64_t, max: int64_t) -> c_int;
    pub fn av_mul_q(b: AVRational, c: AVRational) -> AVRational;
    pub fn av_div_q(b: AVRational, c: AVRational) -> AVRational;
    pub fn av_add_q(b: AVRational, c: AVRational) -> AVRational;
    pub fn av_sub_q(b: AVRational, c: AVRational) -> AVRational;
    pub fn av_d2q(d: c_double, max: c_int) -> AVRational;
    pub fn av_nearer_q(q: AVRational, q1: AVRational, q2: AVRational) ->
     c_int;
    pub fn av_find_nearest_q_idx(q: AVRational, q_list: *AVRational) -> c_int;
    pub fn av_gcd(a: int64_t, b: int64_t) -> int64_t;
    pub fn av_rescale(a: int64_t, b: int64_t, c: int64_t) -> int64_t;
    pub fn av_rescale_rnd(a: int64_t, b: int64_t, c: int64_t,
                          arg1: Enum_AVRounding) -> int64_t;
    pub fn av_rescale_q(a: int64_t, bq: AVRational, cq: AVRational) ->
     int64_t;
    pub fn av_rescale_q_rnd(a: int64_t, bq: AVRational, cq: AVRational,
                            arg1: Enum_AVRounding) -> int64_t;
    pub fn av_compare_ts(ts_a: int64_t, tb_a: AVRational, ts_b: int64_t,
                         tb_b: AVRational) -> c_int;
    pub fn av_compare_mod(a: uint64_t, b: uint64_t, _mod: uint64_t) ->
     int64_t;
    pub fn av_rescale_delta(in_tb: AVRational, in_ts: int64_t,
                            fs_tb: AVRational, duration: c_int,
                            last: *mut int64_t, out_tb: AVRational) ->
     int64_t;
    pub fn av_log(avcl: *mut c_void, level: c_int, fmt: *c_schar, ...);
    pub fn av_vlog(avcl: *mut c_void, level: c_int, fmt: *c_schar,
                   arg1: va_list);
    pub fn av_log_get_level() -> c_int;
    pub fn av_log_set_level(arg1: c_int);
    pub fn av_log_set_callback(arg1:
                                   extern "C" fn
                                       (arg1: *mut c_void, arg2: c_int,
                                        arg3: *c_schar,
                                        arg4: *mut __va_list_tag));
    pub fn av_log_default_callback(ptr: *mut c_void, level: c_int,
                                   fmt: *c_schar, vl: va_list);
    pub fn av_default_item_name(ctx: *mut c_void) -> *c_schar;
    pub fn av_default_get_category(ptr: *mut c_void) -> AVClassCategory;
    pub fn av_log_format_line(ptr: *mut c_void, level: c_int, fmt: *c_schar,
                              vl: va_list, line: *mut c_schar,
                              line_size: c_int, print_prefix: *mut c_int);
    pub fn av_log_set_flags(arg: c_int);
    pub fn av_get_sample_fmt_name(sample_fmt: Enum_AVSampleFormat) ->
     *c_schar;
    pub fn av_get_sample_fmt(name: *c_schar) -> Enum_AVSampleFormat;
    pub fn av_get_alt_sample_fmt(sample_fmt: Enum_AVSampleFormat,
                                 planar: c_int) -> Enum_AVSampleFormat;
    pub fn av_get_packed_sample_fmt(sample_fmt: Enum_AVSampleFormat) ->
     Enum_AVSampleFormat;
    pub fn av_get_planar_sample_fmt(sample_fmt: Enum_AVSampleFormat) ->
     Enum_AVSampleFormat;
    pub fn av_get_sample_fmt_string(buf: *mut c_schar, buf_size: c_int,
                                    sample_fmt: Enum_AVSampleFormat) ->
     *mut c_schar;
    pub fn av_get_bits_per_sample_fmt(sample_fmt: Enum_AVSampleFormat) ->
     c_int;
    pub fn av_get_bytes_per_sample(sample_fmt: Enum_AVSampleFormat) -> c_int;
    pub fn av_sample_fmt_is_planar(sample_fmt: Enum_AVSampleFormat) -> c_int;
    pub fn av_samples_get_buffer_size(linesize: *mut c_int,
                                      nb_channels: c_int, nb_samples: c_int,
                                      sample_fmt: Enum_AVSampleFormat,
                                      align: c_int) -> c_int;
    pub fn av_samples_fill_arrays(audio_data: *mut *mut uint8_t,
                                  linesize: *mut c_int, buf: *uint8_t,
                                  nb_channels: c_int, nb_samples: c_int,
                                  sample_fmt: Enum_AVSampleFormat,
                                  align: c_int) -> c_int;
    pub fn av_samples_alloc(audio_data: *mut *mut uint8_t,
                            linesize: *mut c_int, nb_channels: c_int,
                            nb_samples: c_int,
                            sample_fmt: Enum_AVSampleFormat, align: c_int) ->
     c_int;
    pub fn av_samples_copy(dst: *mut *mut uint8_t, src: **mut uint8_t,
                           dst_offset: c_int, src_offset: c_int,
                           nb_samples: c_int, nb_channels: c_int,
                           sample_fmt: Enum_AVSampleFormat) -> c_int;
    pub fn av_samples_set_silence(audio_data: *mut *mut uint8_t,
                                  offset: c_int, nb_samples: c_int,
                                  nb_channels: c_int,
                                  sample_fmt: Enum_AVSampleFormat) -> c_int;
    pub fn av_dict_get(m: *mut AVDictionary, key: *c_schar,
                       prev: *AVDictionaryEntry, flags: c_int) ->
     *mut AVDictionaryEntry;
    pub fn av_dict_count(m: *AVDictionary) -> c_int;
    pub fn av_dict_set(pm: *mut *mut AVDictionary, key: *c_schar,
                       value: *c_schar, flags: c_int) -> c_int;
    pub fn av_dict_parse_string(pm: *mut *mut AVDictionary, str: *c_schar,
                                key_val_sep: *c_schar, pairs_sep: *c_schar,
                                flags: c_int) -> c_int;
    pub fn av_dict_copy(dst: *mut *mut AVDictionary, src: *mut AVDictionary,
                        flags: c_int);
    pub fn av_dict_free(m: *mut *mut AVDictionary);
    pub fn av_gettime() -> int64_t;
    pub fn av_usleep(usec: c_uint) -> c_int;
    pub fn av_get_channel_layout(name: *c_schar) -> uint64_t;
    pub fn av_get_channel_layout_string(buf: *mut c_schar, buf_size: c_int,
                                        nb_channels: c_int,
                                        channel_layout: uint64_t);
    pub fn av_bprint_channel_layout(bp: *mut Struct_AVBPrint,
                                    nb_channels: c_int,
                                    channel_layout: uint64_t);
    pub fn av_get_channel_layout_nb_channels(channel_layout: uint64_t) ->
     c_int;
    pub fn av_get_default_channel_layout(nb_channels: c_int) -> int64_t;
    pub fn av_get_channel_layout_channel_index(channel_layout: uint64_t,
                                               channel: uint64_t) -> c_int;
    pub fn av_channel_layout_extract_channel(channel_layout: uint64_t,
                                             index: c_int) -> uint64_t;
    pub fn av_get_channel_name(channel: uint64_t) -> *c_schar;
    pub fn av_get_channel_description(channel: uint64_t) -> *c_schar;
    pub fn av_get_standard_channel_layout(index: c_uint,
                                          layout: *mut uint64_t,
                                          name: *mut *c_schar) -> c_int;
    pub fn av_find_opt(obj: *mut c_void, name: *c_schar, unit: *c_schar,
                       mask: c_int, flags: c_int) -> *AVOption;
    pub fn av_set_string3(obj: *mut c_void, name: *c_schar, val: *c_schar,
                          alloc: c_int, o_out: *mut *AVOption) -> c_int;
    pub fn av_set_double(obj: *mut c_void, name: *c_schar, n: c_double) ->
     *AVOption;
    pub fn av_set_q(obj: *mut c_void, name: *c_schar, n: AVRational) ->
     *AVOption;
    pub fn av_set_int(obj: *mut c_void, name: *c_schar, n: int64_t) ->
     *AVOption;
    pub fn av_get_double(obj: *mut c_void, name: *c_schar,
                         o_out: *mut *AVOption) -> c_double;
    pub fn av_get_q(obj: *mut c_void, name: *c_schar, o_out: *mut *AVOption)
     -> AVRational;
    pub fn av_get_int(obj: *mut c_void, name: *c_schar, o_out: *mut *AVOption)
     -> int64_t;
    pub fn av_get_string(obj: *mut c_void, name: *c_schar,
                         o_out: *mut *AVOption, buf: *mut c_schar,
                         buf_len: c_int) -> *c_schar;
    pub fn av_next_option(obj: *mut c_void, last: *AVOption) -> *AVOption;
    pub fn av_opt_show2(obj: *mut c_void, av_log_obj: *mut c_void,
                        req_flags: c_int, rej_flags: c_int) -> c_int;
    pub fn av_opt_set_defaults(s: *mut c_void);
    pub fn av_opt_set_defaults2(s: *mut c_void, mask: c_int, flags: c_int);
    pub fn av_set_options_string(ctx: *mut c_void, opts: *c_schar,
                                 key_val_sep: *c_schar, pairs_sep: *c_schar)
     -> c_int;
    pub fn av_opt_set_from_string(ctx: *mut c_void, opts: *c_schar,
                                  shorthand: **c_schar, key_val_sep: *c_schar,
                                  pairs_sep: *c_schar) -> c_int;
    pub fn av_opt_free(obj: *mut c_void);
    pub fn av_opt_flag_is_set(obj: *mut c_void, field_name: *c_schar,
                              flag_name: *c_schar) -> c_int;
    pub fn av_opt_set_dict(obj: *mut c_void,
                           options: *mut *mut Struct_AVDictionary) -> c_int;
    pub fn av_opt_get_key_value(ropts: *mut *c_schar, key_val_sep: *c_schar,
                                pairs_sep: *c_schar, flags: c_uint,
                                rkey: *mut *mut c_schar,
                                rval: *mut *mut c_schar) -> c_int;
    pub fn av_opt_eval_flags(obj: *mut c_void, o: *AVOption, val: *c_schar,
                             flags_out: *mut c_int) -> c_int;
    pub fn av_opt_eval_int(obj: *mut c_void, o: *AVOption, val: *c_schar,
                           int_out: *mut c_int) -> c_int;
    pub fn av_opt_eval_int64(obj: *mut c_void, o: *AVOption, val: *c_schar,
                             int64_out: *mut int64_t) -> c_int;
    pub fn av_opt_eval_float(obj: *mut c_void, o: *AVOption, val: *c_schar,
                             float_out: *mut c_float) -> c_int;
    pub fn av_opt_eval_double(obj: *mut c_void, o: *AVOption, val: *c_schar,
                              double_out: *mut c_double) -> c_int;
    pub fn av_opt_eval_q(obj: *mut c_void, o: *AVOption, val: *c_schar,
                         q_out: *mut AVRational) -> c_int;
    pub fn av_opt_find(obj: *mut c_void, name: *c_schar, unit: *c_schar,
                       opt_flags: c_int, search_flags: c_int) -> *AVOption;
    pub fn av_opt_find2(obj: *mut c_void, name: *c_schar, unit: *c_schar,
                        opt_flags: c_int, search_flags: c_int,
                        target_obj: *mut *mut c_void) -> *AVOption;
    pub fn av_opt_next(obj: *mut c_void, prev: *AVOption) -> *AVOption;
    pub fn av_opt_child_next(obj: *mut c_void, prev: *mut c_void) ->
     *mut c_void;
    pub fn av_opt_child_class_next(parent: *AVClass, prev: *AVClass) ->
     *AVClass;
    pub fn av_opt_set(obj: *mut c_void, name: *c_schar, val: *c_schar,
                      search_flags: c_int) -> c_int;
    pub fn av_opt_set_int(obj: *mut c_void, name: *c_schar, val: int64_t,
                          search_flags: c_int) -> c_int;
    pub fn av_opt_set_double(obj: *mut c_void, name: *c_schar, val: c_double,
                             search_flags: c_int) -> c_int;
    pub fn av_opt_set_q(obj: *mut c_void, name: *c_schar, val: AVRational,
                        search_flags: c_int) -> c_int;
    pub fn av_opt_set_bin(obj: *mut c_void, name: *c_schar, val: *uint8_t,
                          size: c_int, search_flags: c_int) -> c_int;
    pub fn av_opt_set_image_size(obj: *mut c_void, name: *c_schar, w: c_int,
                                 h: c_int, search_flags: c_int) -> c_int;
    pub fn av_opt_set_pixel_fmt(obj: *mut c_void, name: *c_schar,
                                fmt: Enum_AVPixelFormat, search_flags: c_int)
     -> c_int;
    pub fn av_opt_set_sample_fmt(obj: *mut c_void, name: *c_schar,
                                 fmt: Enum_AVSampleFormat,
                                 search_flags: c_int) -> c_int;
    pub fn av_opt_get(obj: *mut c_void, name: *c_schar, search_flags: c_int,
                      out_val: *mut *mut uint8_t) -> c_int;
    pub fn av_opt_get_int(obj: *mut c_void, name: *c_schar,
                          search_flags: c_int, out_val: *mut int64_t) ->
     c_int;
    pub fn av_opt_get_double(obj: *mut c_void, name: *c_schar,
                             search_flags: c_int, out_val: *mut c_double) ->
     c_int;
    pub fn av_opt_get_q(obj: *mut c_void, name: *c_schar, search_flags: c_int,
                        out_val: *mut AVRational) -> c_int;
    pub fn av_opt_get_image_size(obj: *mut c_void, name: *c_schar,
                                 search_flags: c_int, w_out: *mut c_int,
                                 h_out: *mut c_int) -> c_int;
    pub fn av_opt_get_pixel_fmt(obj: *mut c_void, name: *c_schar,
                                search_flags: c_int,
                                out_fmt: *mut Enum_AVPixelFormat) -> c_int;
    pub fn av_opt_get_sample_fmt(obj: *mut c_void, name: *c_schar,
                                 search_flags: c_int,
                                 out_fmt: *mut Enum_AVSampleFormat) -> c_int;
    pub fn av_opt_ptr(avclass: *AVClass, obj: *mut c_void, name: *c_schar) ->
     *mut c_void;
    pub fn av_opt_freep_ranges(ranges: *mut *mut AVOptionRanges);
    pub fn av_opt_query_ranges(arg1: *mut *mut AVOptionRanges,
                               obj: *mut c_void, key: *c_schar, flags: c_int)
     -> c_int;
    pub fn av_opt_query_ranges_default(arg1: *mut *mut AVOptionRanges,
                                       obj: *mut c_void, key: *c_schar,
                                       flags: c_int) -> c_int;
}

pub fn version() -> uint{
    unsafe {
        avutil_version() as uint
    }
}
pub fn license() -> ~str {
    unsafe {
        std::str::raw::from_c_str(avutil_license())
    }
}
pub static AV_NOPTS_VALUE: int64_t = 0x8000000000000000u64 as int64_t;
pub static AV_TIME_BASE: c_int = 1000000;
pub static AV_TIME_BASE_Q: Struct_AVRational = Struct_AVRational {num: 1, den: AV_TIME_BASE};
pub fn av_cmp_q(a: AVRational, b: AVRational) -> c_int {
    let tmp: int64_t = (a.num as int64_t) * (b.den as int64_t) - (b.num as int64_t) * (a.den as int64_t);

    if tmp != 0 {
        (((tmp ^ (a.den as int64_t) ^ (b.den as int64_t))>>63)|1) as c_int
    } else if b.den != 0 && a.den != 0 {
        0
    } else if a.num != 0 && b.num != 0 {
        (a.num>>31) - (b.num>>31)
    } else {
        -2147483647-1 //INT_MIN
    }
}
pub fn av_q2d(a: AVRational) -> c_double {
    (a.num as c_double) / (a.den as c_double)
}
