extern crate avutil52;
extern crate avcodec54;
extern crate avformat54;
extern crate avdevice54;
extern crate avfilter3;
extern crate swresample0;
extern crate swscale2;

pub fn main() {
    let get_major = |v: uint| {v >> 16};
    let get_minor = |v: uint| {v >> 8 & 0xff};
    let get_micro = |v: uint| {v & 0xff};
    let to_str = |v| {format!("{}.{}.{}", get_major(v), get_minor(v), get_micro(v))};

    println!("avutil     : {} ({})", to_str(avutil52::version()), avutil52::license());
    println!("avcodec    : {} ({})", to_str(avcodec54::version()), avcodec54::license());
    println!("avformat   : {} ({})", to_str(avformat54::version()), avformat54::license());
    println!("avdevice   : {} ({})", to_str(avdevice54::version()), avdevice54::license());
    println!("avfilter   : {} ({})", to_str(avfilter3::version()), avfilter3::license());
    println!("swresample : {} ({})", to_str(swresample0::version()), swresample0::license());
    println!("swscale    : {} ({})", to_str(swscale2::version()), swscale2::license());
}
