extern crate avutil;
extern crate avcodec;
extern crate avformat;
extern crate avdevice;
extern crate avfilter;
extern crate swresample;
extern crate swscale;

pub fn main() {
    let get_major = |v: uint| {v >> 16};
    let get_minor = |v: uint| {v >> 8 & 0xff};
    let get_micro = |v: uint| {v & 0xff};
    let to_str = |v| {format!("{}.{}.{}", get_major(v), get_minor(v), get_micro(v))};

    println!("avutil     : {} ({})", to_str(avutil::version()), avutil::license());
    println!("avcodec    : {} ({})", to_str(avcodec::version()), avcodec::license());
    println!("avformat   : {} ({})", to_str(avformat::version()), avformat::license());
    println!("avdevice   : {} ({})", to_str(avdevice::version()), avdevice::license());
    println!("avfilter   : {} ({})", to_str(avfilter::version()), avfilter::license());
    println!("swresample : {} ({})", to_str(swresample::version()), swresample::license());
    println!("swscale    : {} ({})", to_str(swscale::version()), swscale::license());
}
