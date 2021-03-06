#![crate_name = "avformat54"]
#![crate_type = "dylib"]
#![allow(non_camel_case_types, non_snake_case)]
/* automatically generated by rust-bindgen */
extern crate libc;
extern crate avutil52 as avutil;
extern crate avcodec54 as avcodec;
use libc::{uint8_t,int64_t,uint64_t,FILE};

pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVIOInterruptCB {
    pub callback: ::std::option::Option<extern "C" fn(arg1:
                                                          *mut ::libc::c_void)
                                            -> ::libc::c_int>,
    pub opaque: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct_AVIOInterruptCB {
    fn clone(&self) -> Struct_AVIOInterruptCB { *self }
}
impl ::std::default::Default for Struct_AVIOInterruptCB {
    fn default() -> Struct_AVIOInterruptCB { unsafe { ::std::mem::zeroed() } }
}
pub type AVIOInterruptCB = Struct_AVIOInterruptCB;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVIOContext {
    pub av_class: *const avutil::AVClass,
    pub buffer: *mut ::libc::c_uchar,
    pub buffer_size: ::libc::c_int,
    pub buf_ptr: *mut ::libc::c_uchar,
    pub buf_end: *mut ::libc::c_uchar,
    pub opaque: *mut ::libc::c_void,
    pub read_packet: ::std::option::Option<extern "C" fn(opaque:
                                                             *mut ::libc::c_void,
                                                         buf: *mut uint8_t,
                                                         buf_size:
                                                             ::libc::c_int)
                                               -> ::libc::c_int>,
    pub write_packet: ::std::option::Option<extern "C" fn(opaque:
                                                              *mut ::libc::c_void,
                                                          buf: *mut uint8_t,
                                                          buf_size:
                                                              ::libc::c_int)
                                                -> ::libc::c_int>,
    pub seek: ::std::option::Option<extern "C" fn(opaque: *mut ::libc::c_void,
                                                  offset: int64_t,
                                                  whence: ::libc::c_int)
                                        -> int64_t>,
    pub pos: int64_t,
    pub must_flush: ::libc::c_int,
    pub eof_reached: ::libc::c_int,
    pub write_flag: ::libc::c_int,
    pub max_packet_size: ::libc::c_int,
    pub checksum: ::libc::c_ulong,
    pub checksum_ptr: *mut ::libc::c_uchar,
    pub update_checksum: ::std::option::Option<extern "C" fn(checksum:
                                                                 ::libc::c_ulong,
                                                             buf:
                                                                 *const uint8_t,
                                                             size:
                                                                 ::libc::c_uint)
                                                   -> ::libc::c_ulong>,
    pub error: ::libc::c_int,
    pub read_pause: ::std::option::Option<extern "C" fn(opaque:
                                                            *mut ::libc::c_void,
                                                        pause: ::libc::c_int)
                                              -> ::libc::c_int>,
    pub read_seek: ::std::option::Option<extern "C" fn(opaque:
                                                           *mut ::libc::c_void,
                                                       stream_index:
                                                           ::libc::c_int,
                                                       timestamp: int64_t,
                                                       flags: ::libc::c_int)
                                             -> int64_t>,
    pub seekable: ::libc::c_int,
    pub maxsize: int64_t,
    pub direct: ::libc::c_int,
    pub bytes_read: int64_t,
    pub seek_count: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVIOContext {
    fn clone(&self) -> Struct_AVIOContext { *self }
}
impl ::std::default::Default for Struct_AVIOContext {
    fn default() -> Struct_AVIOContext { unsafe { ::std::mem::zeroed() } }
}
pub type AVIOContext = Struct_AVIOContext;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVFrac {
    pub val: int64_t,
    pub num: int64_t,
    pub den: int64_t,
}
impl ::std::clone::Clone for Struct_AVFrac {
    fn clone(&self) -> Struct_AVFrac { *self }
}
impl ::std::default::Default for Struct_AVFrac {
    fn default() -> Struct_AVFrac { unsafe { ::std::mem::zeroed() } }
}
pub type AVFrac = Struct_AVFrac;
pub enum Struct_AVCodecTag { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVProbeData {
    pub filename: *const ::libc::c_char,
    pub buf: *mut ::libc::c_uchar,
    pub buf_size: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVProbeData {
    fn clone(&self) -> Struct_AVProbeData { *self }
}
impl ::std::default::Default for Struct_AVProbeData {
    fn default() -> Struct_AVProbeData { unsafe { ::std::mem::zeroed() } }
}
pub type AVProbeData = Struct_AVProbeData;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVOutputFormat {
    pub name: *const ::libc::c_char,
    pub long_name: *const ::libc::c_char,
    pub mime_type: *const ::libc::c_char,
    pub extensions: *const ::libc::c_char,
    pub audio_codec: avcodec::Enum_AVCodecID,
    pub video_codec: avcodec::Enum_AVCodecID,
    pub subtitle_codec: avcodec::Enum_AVCodecID,
    pub flags: ::libc::c_int,
    pub codec_tag: *const *const Struct_AVCodecTag,
    pub priv_class: *const avutil::AVClass,
    pub next: *mut Struct_AVOutputFormat,
    pub priv_data_size: ::libc::c_int,
    pub write_header: ::std::option::Option<extern "C" fn(arg1:
                                                              *mut Struct_AVFormatContext)
                                                -> ::libc::c_int>,
    pub write_packet: ::std::option::Option<extern "C" fn(arg1:
                                                              *mut Struct_AVFormatContext,
                                                          pkt: *mut avcodec::AVPacket)
                                                -> ::libc::c_int>,
    pub write_trailer: ::std::option::Option<extern "C" fn(arg1:
                                                               *mut Struct_AVFormatContext)
                                                 -> ::libc::c_int>,
    pub interleave_packet: ::std::option::Option<extern "C" fn(arg1:
                                                                   *mut Struct_AVFormatContext,
                                                               out:
                                                                   *mut avcodec::AVPacket,
                                                               _in:
                                                                   *mut avcodec::AVPacket,
                                                               flush:
                                                                   ::libc::c_int)
                                                     -> ::libc::c_int>,
    pub query_codec: ::std::option::Option<extern "C" fn(id: avcodec::Enum_AVCodecID,
                                                         std_compliance:
                                                             ::libc::c_int)
                                               -> ::libc::c_int>,
    pub get_output_timestamp: ::std::option::Option<extern "C" fn(s:
                                                                      *mut Struct_AVFormatContext,
                                                                  stream:
                                                                      ::libc::c_int,
                                                                  dts:
                                                                      *mut int64_t,
                                                                  wall:
                                                                      *mut int64_t)
                                                        -> ()>,
}
impl ::std::clone::Clone for Struct_AVOutputFormat {
    fn clone(&self) -> Struct_AVOutputFormat { *self }
}
impl ::std::default::Default for Struct_AVOutputFormat {
    fn default() -> Struct_AVOutputFormat { unsafe { ::std::mem::zeroed() } }
}
pub type AVOutputFormat = Struct_AVOutputFormat;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVInputFormat {
    pub name: *const ::libc::c_char,
    pub long_name: *const ::libc::c_char,
    pub flags: ::libc::c_int,
    pub extensions: *const ::libc::c_char,
    pub codec_tag: *const *const Struct_AVCodecTag,
    pub priv_class: *const avutil::AVClass,
    pub next: *mut Struct_AVInputFormat,
    pub raw_codec_id: ::libc::c_int,
    pub priv_data_size: ::libc::c_int,
    pub read_probe: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut AVProbeData)
                                              -> ::libc::c_int>,
    pub read_header: ::std::option::Option<extern "C" fn(arg1:
                                                             *mut Struct_AVFormatContext)
                                               -> ::libc::c_int>,
    pub read_packet: ::std::option::Option<extern "C" fn(arg1:
                                                             *mut Struct_AVFormatContext,
                                                         pkt: *mut avcodec::AVPacket)
                                               -> ::libc::c_int>,
    pub read_close: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut Struct_AVFormatContext)
                                              -> ::libc::c_int>,
    pub read_seek: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut Struct_AVFormatContext,
                                                       stream_index:
                                                           ::libc::c_int,
                                                       timestamp: int64_t,
                                                       flags: ::libc::c_int)
                                             -> ::libc::c_int>,
    pub read_timestamp: ::std::option::Option<extern "C" fn(s:
                                                                *mut Struct_AVFormatContext,
                                                            stream_index:
                                                                ::libc::c_int,
                                                            pos: *mut int64_t,
                                                            pos_limit:
                                                                int64_t)
                                                  -> int64_t>,
    pub read_play: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut Struct_AVFormatContext)
                                             -> ::libc::c_int>,
    pub read_pause: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut Struct_AVFormatContext)
                                              -> ::libc::c_int>,
    pub read_seek2: ::std::option::Option<extern "C" fn(s:
                                                            *mut Struct_AVFormatContext,
                                                        stream_index:
                                                            ::libc::c_int,
                                                        min_ts: int64_t,
                                                        ts: int64_t,
                                                        max_ts: int64_t,
                                                        flags: ::libc::c_int)
                                              -> ::libc::c_int>,
}
impl ::std::clone::Clone for Struct_AVInputFormat {
    fn clone(&self) -> Struct_AVInputFormat { *self }
}
impl ::std::default::Default for Struct_AVInputFormat {
    fn default() -> Struct_AVInputFormat { unsafe { ::std::mem::zeroed() } }
}
pub type AVInputFormat = Struct_AVInputFormat;
pub type Enum_AVStreamParseType = ::libc::c_uint;
pub const AVSTREAM_PARSE_NONE: ::libc::c_uint = 0;
pub const AVSTREAM_PARSE_FULL: ::libc::c_uint = 1;
pub const AVSTREAM_PARSE_HEADERS: ::libc::c_uint = 2;
pub const AVSTREAM_PARSE_TIMESTAMPS: ::libc::c_uint = 3;
pub const AVSTREAM_PARSE_FULL_ONCE: ::libc::c_uint = 4;
pub const AVSTREAM_PARSE_FULL_RAW: ::libc::c_uint = 1463898624;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVIndexEntry {
    pub pos: int64_t,
    pub timestamp: int64_t,
    pub _bindgen_bitfield_1_: ::libc::c_int,
    pub min_distance: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVIndexEntry {
    fn clone(&self) -> Struct_AVIndexEntry { *self }
}
impl ::std::default::Default for Struct_AVIndexEntry {
    fn default() -> Struct_AVIndexEntry { unsafe { ::std::mem::zeroed() } }
}
pub type AVIndexEntry = Struct_AVIndexEntry;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVStream {
    pub index: ::libc::c_int,
    pub id: ::libc::c_int,
    pub codec: *mut avcodec::AVCodecContext,
    pub r_frame_rate: avutil::AVRational,
    pub priv_data: *mut ::libc::c_void,
    pub pts: Struct_AVFrac,
    pub time_base: avutil::AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: ::libc::c_int,
    pub discard: avcodec::Enum_AVDiscard,
    pub sample_aspect_ratio: avutil::AVRational,
    pub metadata: *mut avutil::AVDictionary,
    pub avg_frame_rate: avutil::AVRational,
    pub attached_pic: avcodec::AVPacket,
    pub info: *mut Struct_Unnamed1,
    pub pts_wrap_bits: ::libc::c_int,
    pub reference_dts: int64_t,
    pub first_dts: int64_t,
    pub cur_dts: int64_t,
    pub last_IP_pts: int64_t,
    pub last_IP_duration: ::libc::c_int,
    pub probe_packets: ::libc::c_int,
    pub codec_info_nb_frames: ::libc::c_int,
    pub stream_identifier: ::libc::c_int,
    pub interleaver_chunk_size: int64_t,
    pub interleaver_chunk_duration: int64_t,
    pub need_parsing: Enum_AVStreamParseType,
    pub parser: *mut avcodec::Struct_AVCodecParserContext,
    pub last_in_packet_buffer: *mut Struct_AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [int64_t; 17usize],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: ::libc::c_int,
    pub index_entries_allocated_size: ::libc::c_uint,
    pub request_probe: ::libc::c_int,
    pub skip_to_keyframe: ::libc::c_int,
    pub skip_samples: ::libc::c_int,
    pub nb_decoded_frames: ::libc::c_int,
    pub mux_ts_offset: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVStream {
    fn clone(&self) -> Struct_AVStream { *self }
}
impl ::std::default::Default for Struct_AVStream {
    fn default() -> Struct_AVStream { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub last_dts: int64_t,
    pub duration_gcd: int64_t,
    pub duration_count: ::libc::c_int,
    pub duration_error: *mut ::libc::c_void,
    pub codec_info_duration: int64_t,
    pub codec_info_duration_fields: int64_t,
    pub found_decoder: ::libc::c_int,
    pub last_duration: int64_t,
    pub fps_first_dts: int64_t,
    pub fps_first_dts_idx: ::libc::c_int,
    pub fps_last_dts: int64_t,
    pub fps_last_dts_idx: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Struct_Unnamed1 { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type AVStream = Struct_AVStream;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVProgram {
    pub id: ::libc::c_int,
    pub flags: ::libc::c_int,
    pub discard: avcodec::Enum_AVDiscard,
    pub stream_index: *mut ::libc::c_uint,
    pub nb_stream_indexes: ::libc::c_uint,
    pub metadata: *mut avutil::AVDictionary,
    pub program_num: ::libc::c_int,
    pub pmt_pid: ::libc::c_int,
    pub pcr_pid: ::libc::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVProgram {
    fn clone(&self) -> Struct_AVProgram { *self }
}
impl ::std::default::Default for Struct_AVProgram {
    fn default() -> Struct_AVProgram { unsafe { ::std::mem::zeroed() } }
}
pub type AVProgram = Struct_AVProgram;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVChapter {
    pub id: ::libc::c_int,
    pub time_base: avutil::AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut avutil::AVDictionary,
}
impl ::std::clone::Clone for Struct_AVChapter {
    fn clone(&self) -> Struct_AVChapter { *self }
}
impl ::std::default::Default for Struct_AVChapter {
    fn default() -> Struct_AVChapter { unsafe { ::std::mem::zeroed() } }
}
pub type AVChapter = Struct_AVChapter;
pub type Enum_AVDurationEstimationMethod = ::libc::c_uint;
pub const AVFMT_DURATION_FROM_PTS: ::libc::c_uint = 0;
pub const AVFMT_DURATION_FROM_STREAM: ::libc::c_uint = 1;
pub const AVFMT_DURATION_FROM_BITRATE: ::libc::c_uint = 2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVFormatContext {
    pub av_class: *const avutil::AVClass,
    pub iformat: *mut Struct_AVInputFormat,
    pub oformat: *mut Struct_AVOutputFormat,
    pub priv_data: *mut ::libc::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: ::libc::c_int,
    pub nb_streams: ::libc::c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [::libc::c_char; 1024usize],
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: ::libc::c_int,
    pub packet_size: ::libc::c_uint,
    pub max_delay: ::libc::c_int,
    pub flags: ::libc::c_int,
    pub probesize: ::libc::c_uint,
    pub max_analyze_duration: ::libc::c_int,
    pub key: *const uint8_t,
    pub keylen: ::libc::c_int,
    pub nb_programs: ::libc::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: avcodec::Enum_AVCodecID,
    pub audio_codec_id: avcodec::Enum_AVCodecID,
    pub subtitle_codec_id: avcodec::Enum_AVCodecID,
    pub max_index_size: ::libc::c_uint,
    pub max_picture_buffer: ::libc::c_uint,
    pub nb_chapters: ::libc::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut avutil::AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: ::libc::c_int,
    pub error_recognition: ::libc::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: ::libc::c_int,
    pub ts_id: ::libc::c_int,
    pub audio_preload: ::libc::c_int,
    pub max_chunk_duration: ::libc::c_int,
    pub max_chunk_size: ::libc::c_int,
    pub use_wallclock_as_timestamps: ::libc::c_int,
    pub avoid_negative_ts: ::libc::c_int,
    pub avio_flags: ::libc::c_int,
    pub duration_estimation_method: Enum_AVDurationEstimationMethod,
    pub skip_initial_bytes: ::libc::c_uint,
    pub correct_ts_overflow: ::libc::c_uint,
    pub seek2any: ::libc::c_int,
    pub packet_buffer: *mut Struct_AVPacketList,
    pub packet_buffer_end: *mut Struct_AVPacketList,
    pub data_offset: int64_t,
    pub raw_packet_buffer: *mut Struct_AVPacketList,
    pub raw_packet_buffer_end: *mut Struct_AVPacketList,
    pub parse_queue: *mut Struct_AVPacketList,
    pub parse_queue_end: *mut Struct_AVPacketList,
    pub raw_packet_buffer_remaining_size: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVFormatContext {
    fn clone(&self) -> Struct_AVFormatContext { *self }
}
impl ::std::default::Default for Struct_AVFormatContext {
    fn default() -> Struct_AVFormatContext { unsafe { ::std::mem::zeroed() } }
}
pub type AVFormatContext = Struct_AVFormatContext;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVPacketList {
    pub pkt: avcodec::AVPacket,
    pub next: *mut Struct_AVPacketList,
}
impl ::std::clone::Clone for Struct_AVPacketList {
    fn clone(&self) -> Struct_AVPacketList { *self }
}
impl ::std::default::Default for Struct_AVPacketList {
    fn default() -> Struct_AVPacketList { unsafe { ::std::mem::zeroed() } }
}
pub type AVPacketList = Struct_AVPacketList;
pub type __builtin_va_list = [__va_list_tag; 1usize];
pub type __va_list_tag = Struct___va_list_tag;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct___va_list_tag {
    fn clone(&self) -> Struct___va_list_tag { *self }
}
impl ::std::default::Default for Struct___va_list_tag {
    fn default() -> Struct___va_list_tag { unsafe { ::std::mem::zeroed() } }
}

pub const AVSEEK_FLAG_BACKWARD: ::libc::c_int = 1;
pub const AVSEEK_FLAG_BYTE: ::libc::c_int = 2;
pub const AVSEEK_FLAG_ANY: ::libc::c_int = 4;
pub const AVSEEK_FLAG_FRAME: ::libc::c_int = 8;

#[link(name = "avformat")]
extern "C" {
    pub fn avio_check(url: *const ::libc::c_char, flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn avio_alloc_context(buffer: *mut ::libc::c_uchar,
                              buffer_size: ::libc::c_int,
                              write_flag: ::libc::c_int,
                              opaque: *mut ::libc::c_void,
                              read_packet:
                                  ::std::option::Option<extern "C" fn(opaque:
                                                                          *mut ::libc::c_void,
                                                                      buf:
                                                                          *mut uint8_t,
                                                                      buf_size:
                                                                          ::libc::c_int)
                                                            -> ::libc::c_int>,
                              write_packet:
                                  ::std::option::Option<extern "C" fn(opaque:
                                                                          *mut ::libc::c_void,
                                                                      buf:
                                                                          *mut uint8_t,
                                                                      buf_size:
                                                                          ::libc::c_int)
                                                            -> ::libc::c_int>,
                              seek:
                                  ::std::option::Option<extern "C" fn(opaque:
                                                                          *mut ::libc::c_void,
                                                                      offset:
                                                                          int64_t,
                                                                      whence:
                                                                          ::libc::c_int)
                                                            -> int64_t>)
     -> *mut AVIOContext;
    pub fn avio_w8(s: *mut AVIOContext, b: ::libc::c_int) -> ();
    pub fn avio_write(s: *mut AVIOContext, buf: *const ::libc::c_uchar,
                      size: ::libc::c_int) -> ();
    pub fn avio_wl64(s: *mut AVIOContext, val: uint64_t) -> ();
    pub fn avio_wb64(s: *mut AVIOContext, val: uint64_t) -> ();
    pub fn avio_wl32(s: *mut AVIOContext, val: ::libc::c_uint) -> ();
    pub fn avio_wb32(s: *mut AVIOContext, val: ::libc::c_uint) -> ();
    pub fn avio_wl24(s: *mut AVIOContext, val: ::libc::c_uint) -> ();
    pub fn avio_wb24(s: *mut AVIOContext, val: ::libc::c_uint) -> ();
    pub fn avio_wl16(s: *mut AVIOContext, val: ::libc::c_uint) -> ();
    pub fn avio_wb16(s: *mut AVIOContext, val: ::libc::c_uint) -> ();
    pub fn avio_put_str(s: *mut AVIOContext, str: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn avio_put_str16le(s: *mut AVIOContext, str: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn avio_seek(s: *mut AVIOContext, offset: int64_t,
                     whence: ::libc::c_int) -> int64_t;
    pub fn avio_skip(s: *mut AVIOContext, offset: int64_t) -> int64_t;
    pub fn avio_size(s: *mut AVIOContext) -> int64_t;
    pub fn url_feof(s: *mut AVIOContext) -> ::libc::c_int;
    pub fn avio_printf(s: *mut AVIOContext, fmt: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn avio_flush(s: *mut AVIOContext) -> ();
    pub fn avio_read(s: *mut AVIOContext, buf: *mut ::libc::c_uchar,
                     size: ::libc::c_int) -> ::libc::c_int;
    pub fn avio_r8(s: *mut AVIOContext) -> ::libc::c_int;
    pub fn avio_rl16(s: *mut AVIOContext) -> ::libc::c_uint;
    pub fn avio_rl24(s: *mut AVIOContext) -> ::libc::c_uint;
    pub fn avio_rl32(s: *mut AVIOContext) -> ::libc::c_uint;
    pub fn avio_rl64(s: *mut AVIOContext) -> uint64_t;
    pub fn avio_rb16(s: *mut AVIOContext) -> ::libc::c_uint;
    pub fn avio_rb24(s: *mut AVIOContext) -> ::libc::c_uint;
    pub fn avio_rb32(s: *mut AVIOContext) -> ::libc::c_uint;
    pub fn avio_rb64(s: *mut AVIOContext) -> uint64_t;
    pub fn avio_get_str(pb: *mut AVIOContext, maxlen: ::libc::c_int,
                        buf: *mut ::libc::c_char, buflen: ::libc::c_int)
     -> ::libc::c_int;
    pub fn avio_get_str16le(pb: *mut AVIOContext, maxlen: ::libc::c_int,
                            buf: *mut ::libc::c_char, buflen: ::libc::c_int)
     -> ::libc::c_int;
    pub fn avio_get_str16be(pb: *mut AVIOContext, maxlen: ::libc::c_int,
                            buf: *mut ::libc::c_char, buflen: ::libc::c_int)
     -> ::libc::c_int;
    pub fn avio_open(s: *mut *mut AVIOContext, url: *const ::libc::c_char,
                     flags: ::libc::c_int) -> ::libc::c_int;
    pub fn avio_open2(s: *mut *mut AVIOContext, url: *const ::libc::c_char,
                      flags: ::libc::c_int, int_cb: *const AVIOInterruptCB,
                      options: *mut *mut avutil::AVDictionary) -> ::libc::c_int;
    pub fn avio_close(s: *mut AVIOContext) -> ::libc::c_int;
    pub fn avio_closep(s: *mut *mut AVIOContext) -> ::libc::c_int;
    pub fn avio_open_dyn_buf(s: *mut *mut AVIOContext) -> ::libc::c_int;
    pub fn avio_close_dyn_buf(s: *mut AVIOContext, pbuffer: *mut *mut uint8_t)
     -> ::libc::c_int;
    pub fn avio_enum_protocols(opaque: *mut *mut ::libc::c_void,
                               output: ::libc::c_int)
     -> *const ::libc::c_char;
    pub fn avio_pause(h: *mut AVIOContext, pause: ::libc::c_int)
     -> ::libc::c_int;
    pub fn avio_seek_time(h: *mut AVIOContext, stream_index: ::libc::c_int,
                          timestamp: int64_t, flags: ::libc::c_int)
     -> int64_t;
    pub fn av_get_packet(s: *mut AVIOContext, pkt: *mut avcodec::AVPacket,
                         size: ::libc::c_int) -> ::libc::c_int;
    pub fn av_append_packet(s: *mut AVIOContext, pkt: *mut avcodec::AVPacket,
                            size: ::libc::c_int) -> ::libc::c_int;
    pub fn av_fmt_ctx_get_duration_estimation_method(ctx:
                                                         *const AVFormatContext)
     -> Enum_AVDurationEstimationMethod;
    pub fn avformat_version() -> ::libc::c_uint;
    pub fn avformat_configuration() -> *const ::libc::c_char;
    pub fn avformat_license() -> *const ::libc::c_char;
    pub fn av_register_all() -> ();
    pub fn av_register_input_format(format: *mut AVInputFormat) -> ();
    pub fn av_register_output_format(format: *mut AVOutputFormat) -> ();
    pub fn avformat_network_init() -> ::libc::c_int;
    pub fn avformat_network_deinit() -> ::libc::c_int;
    pub fn av_iformat_next(f: *mut AVInputFormat) -> *mut AVInputFormat;
    pub fn av_oformat_next(f: *mut AVOutputFormat) -> *mut AVOutputFormat;
    pub fn avformat_alloc_context() -> *mut AVFormatContext;
    pub fn avformat_free_context(s: *mut AVFormatContext) -> ();
    pub fn avformat_get_class() -> *const avutil::AVClass;
    pub fn avformat_new_stream(s: *mut AVFormatContext, c: *const avcodec::AVCodec)
     -> *mut AVStream;
    pub fn av_new_program(s: *mut AVFormatContext, id: ::libc::c_int)
     -> *mut AVProgram;
    pub fn avformat_alloc_output_context(format: *const ::libc::c_char,
                                         oformat: *mut AVOutputFormat,
                                         filename: *const ::libc::c_char)
     -> *mut AVFormatContext;
    pub fn avformat_alloc_output_context2(ctx: *mut *mut AVFormatContext,
                                          oformat: *mut AVOutputFormat,
                                          format_name: *const ::libc::c_char,
                                          filename: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn av_find_input_format(short_name: *const ::libc::c_char)
     -> *mut AVInputFormat;
    pub fn av_probe_input_format(pd: *mut AVProbeData,
                                 is_opened: ::libc::c_int)
     -> *mut AVInputFormat;
    pub fn av_probe_input_format2(pd: *mut AVProbeData,
                                  is_opened: ::libc::c_int,
                                  score_max: *mut ::libc::c_int)
     -> *mut AVInputFormat;
    pub fn av_probe_input_format3(pd: *mut AVProbeData,
                                  is_opened: ::libc::c_int,
                                  score_ret: *mut ::libc::c_int)
     -> *mut AVInputFormat;
    pub fn av_probe_input_buffer(pb: *mut AVIOContext,
                                 fmt: *mut *mut AVInputFormat,
                                 filename: *const ::libc::c_char,
                                 logctx: *mut ::libc::c_void,
                                 offset: ::libc::c_uint,
                                 max_probe_size: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn avformat_open_input(ps: *mut *mut AVFormatContext,
                               filename: *const ::libc::c_char,
                               fmt: *mut AVInputFormat,
                               options: *mut *mut avutil::AVDictionary)
     -> ::libc::c_int;
    pub fn av_demuxer_open(ic: *mut AVFormatContext) -> ::libc::c_int;
    pub fn av_find_stream_info(ic: *mut AVFormatContext) -> ::libc::c_int;
    pub fn avformat_find_stream_info(ic: *mut AVFormatContext,
                                     options: *mut *mut avutil::AVDictionary)
     -> ::libc::c_int;
    pub fn av_find_program_from_stream(ic: *mut AVFormatContext,
                                       last: *mut AVProgram, s: ::libc::c_int)
     -> *mut AVProgram;
    pub fn av_find_best_stream(ic: *mut AVFormatContext,
                               _type: avutil::Enum_AVMediaType,
                               wanted_stream_nb: ::libc::c_int,
                               related_stream: ::libc::c_int,
                               decoder_ret: *mut *mut avcodec::AVCodec,
                               flags: ::libc::c_int) -> ::libc::c_int;
    pub fn av_read_packet(s: *mut AVFormatContext, pkt: *mut avcodec::AVPacket)
     -> ::libc::c_int;
    pub fn av_read_frame(s: *mut AVFormatContext, pkt: *mut avcodec::AVPacket)
     -> ::libc::c_int;
    pub fn av_seek_frame(s: *mut AVFormatContext, stream_index: ::libc::c_int,
                         timestamp: int64_t, flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn avformat_seek_file(s: *mut AVFormatContext,
                              stream_index: ::libc::c_int, min_ts: int64_t,
                              ts: int64_t, max_ts: int64_t,
                              flags: ::libc::c_int) -> ::libc::c_int;
    pub fn av_read_play(s: *mut AVFormatContext) -> ::libc::c_int;
    pub fn av_read_pause(s: *mut AVFormatContext) -> ::libc::c_int;
    pub fn av_close_input_file(s: *mut AVFormatContext) -> ();
    pub fn avformat_close_input(s: *mut *mut AVFormatContext) -> ();
    pub fn av_new_stream(s: *mut AVFormatContext, id: ::libc::c_int)
     -> *mut AVStream;
    pub fn av_set_pts_info(s: *mut AVStream, pts_wrap_bits: ::libc::c_int,
                           pts_num: ::libc::c_uint, pts_den: ::libc::c_uint)
     -> ();
    pub fn avformat_write_header(s: *mut AVFormatContext,
                                 options: *mut *mut avutil::AVDictionary)
     -> ::libc::c_int;
    pub fn av_write_frame(s: *mut AVFormatContext, pkt: *mut avcodec::AVPacket)
     -> ::libc::c_int;
    pub fn av_interleaved_write_frame(s: *mut AVFormatContext,
                                      pkt: *mut avcodec::AVPacket) -> ::libc::c_int;
    pub fn av_interleave_packet_per_dts(s: *mut AVFormatContext,
                                        out: *mut avcodec::AVPacket,
                                        pkt: *mut avcodec::AVPacket,
                                        flush: ::libc::c_int)
     -> ::libc::c_int;
    pub fn av_write_trailer(s: *mut AVFormatContext) -> ::libc::c_int;
    pub fn av_guess_format(short_name: *const ::libc::c_char,
                           filename: *const ::libc::c_char,
                           mime_type: *const ::libc::c_char)
     -> *mut AVOutputFormat;
    pub fn av_guess_codec(fmt: *mut AVOutputFormat,
                          short_name: *const ::libc::c_char,
                          filename: *const ::libc::c_char,
                          mime_type: *const ::libc::c_char,
                          _type: avutil::Enum_AVMediaType) -> avcodec::Enum_AVCodecID;
    pub fn av_get_output_timestamp(s: *mut Struct_AVFormatContext,
                                   stream: ::libc::c_int, dts: *mut int64_t,
                                   wall: *mut int64_t) -> ::libc::c_int;
    pub fn av_hex_dump(f: *mut FILE, buf: *const uint8_t, size: ::libc::c_int)
     -> ();
    pub fn av_hex_dump_log(avcl: *mut ::libc::c_void, level: ::libc::c_int,
                           buf: *const uint8_t, size: ::libc::c_int) -> ();
    pub fn av_pkt_dump2(f: *mut FILE, pkt: *mut avcodec::AVPacket,
                        dump_payload: ::libc::c_int, st: *mut AVStream) -> ();
    pub fn av_pkt_dump_log2(avcl: *mut ::libc::c_void, level: ::libc::c_int,
                            pkt: *mut avcodec::AVPacket, dump_payload: ::libc::c_int,
                            st: *mut AVStream) -> ();
    pub fn av_codec_get_id(tags: *const *const Struct_AVCodecTag,
                           tag: ::libc::c_uint) -> avcodec::Enum_AVCodecID;
    pub fn av_codec_get_tag(tags: *const *const Struct_AVCodecTag,
                            id: avcodec::Enum_AVCodecID) -> ::libc::c_uint;
    pub fn av_codec_get_tag2(tags: *const *const Struct_AVCodecTag,
                             id: avcodec::Enum_AVCodecID, tag: *mut ::libc::c_uint)
     -> ::libc::c_int;
    pub fn av_find_default_stream_index(s: *mut AVFormatContext)
     -> ::libc::c_int;
    pub fn av_index_search_timestamp(st: *mut AVStream, timestamp: int64_t,
                                     flags: ::libc::c_int) -> ::libc::c_int;
    pub fn av_add_index_entry(st: *mut AVStream, pos: int64_t,
                              timestamp: int64_t, size: ::libc::c_int,
                              distance: ::libc::c_int, flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn av_url_split(proto: *mut ::libc::c_char, proto_size: ::libc::c_int,
                        authorization: *mut ::libc::c_char,
                        authorization_size: ::libc::c_int,
                        hostname: *mut ::libc::c_char,
                        hostname_size: ::libc::c_int,
                        port_ptr: *mut ::libc::c_int,
                        path: *mut ::libc::c_char, path_size: ::libc::c_int,
                        url: *const ::libc::c_char) -> ();
    pub fn av_dump_format(ic: *mut AVFormatContext, index: ::libc::c_int,
                          url: *const ::libc::c_char,
                          is_output: ::libc::c_int) -> ();
    pub fn av_get_frame_filename(buf: *mut ::libc::c_char,
                                 buf_size: ::libc::c_int,
                                 path: *const ::libc::c_char,
                                 number: ::libc::c_int) -> ::libc::c_int;
    pub fn av_filename_number_test(filename: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn av_sdp_create(ac: *mut *mut AVFormatContext,
                         n_files: ::libc::c_int, buf: *mut ::libc::c_char,
                         size: ::libc::c_int) -> ::libc::c_int;
    pub fn av_match_ext(filename: *const ::libc::c_char,
                        extensions: *const ::libc::c_char) -> ::libc::c_int;
    pub fn avformat_query_codec(ofmt: *mut AVOutputFormat,
                                codec_id: avcodec::Enum_AVCodecID,
                                std_compliance: ::libc::c_int)
     -> ::libc::c_int;
    pub fn avformat_get_riff_video_tags() -> *const Struct_AVCodecTag;
    pub fn avformat_get_riff_audio_tags() -> *const Struct_AVCodecTag;
    pub fn av_guess_sample_aspect_ratio(format: *mut AVFormatContext,
                                        stream: *mut AVStream,
                                        frame: *mut avutil::AVFrame) -> avutil::AVRational;
    pub fn avformat_match_stream_specifier(s: *mut AVFormatContext,
                                           st: *mut AVStream,
                                           spec: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn avformat_queue_attached_pictures(s: *mut AVFormatContext) -> ();
}

pub fn version() -> u32 {
    unsafe { avformat_version() as u32 }
}

pub fn license() -> &'static str {
    std::str::from_utf8(unsafe { std::ffi::CStr::from_ptr(avformat_license()) }.to_bytes()).ok().expect("invalid utf8")
}
