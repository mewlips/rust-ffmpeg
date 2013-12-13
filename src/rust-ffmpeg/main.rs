extern mod avutil;
extern mod avcodec;
extern mod avformat;
extern mod avdevice;
extern mod avfilter;
extern mod swresample;
extern mod swscale;

pub fn main() {
    let get_major = |v: uint| {v >> 16};
    let get_minor = |v: uint| {v >> 8 & 0xff};
    let get_micro = |v: uint| {v & 0xff};
    let to_str = |v| {format!("{}.{}.{}", get_major(v), get_minor(v), get_micro(v))};

    println!("avutil     : {}", to_str(avutil::version()));
    println!("avcodec    : {}", to_str(avcodec::version()));
    println!("avformat   : {}", to_str(avformat::version()));
    println!("avdevice   : {}", to_str(avdevice::version()));
    println!("avfilter   : {}", to_str(avfilter::version()));
    println!("swresample : {}", to_str(swresample::version()));
    println!("swscale    : {}", to_str(swscale::version()));
}
