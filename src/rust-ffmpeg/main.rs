extern mod avutil;
extern mod avcodec;
extern mod avformat;
extern mod avdevice;
extern mod avfilter;
extern mod swresample;
extern mod swscale;

pub fn main() {
    fn get_major(version: uint) -> uint {
        version >> 16
    }
    fn get_minor(version: uint) -> uint {
        (version >> 8) & 0xff
    }
    fn get_micro(version: uint) -> uint {
        version & 0xff
    }
    fn to_str(version: uint) -> ~str {
        format!("{}.{}.{}", get_major(version), get_minor(version), get_micro(version))
    }
    println(format!("avutil     : {}", to_str(avutil::version())));
    println(format!("avcodec    : {}", to_str(avcodec::version())));
    println(format!("avformat   : {}", to_str(avformat::version())));
    println(format!("avdevice   : {}", to_str(avdevice::version())));
    println(format!("avfilter   : {}", to_str(avfilter::version())));
    println(format!("swresample : {}", to_str(swresample::version())));
    println(format!("swscale    : {}", to_str(swscale::version())));
}
