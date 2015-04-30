extern crate avutil52;
extern crate avcodec54;
extern crate avcodec55;
extern crate avformat54;
extern crate avformat55;
extern crate avdevice54;
extern crate avdevice55;
extern crate avfilter3;
extern crate avfilter4;
extern crate swresample0;
extern crate swscale2;

pub fn main() {
    let get_major = |v: u32| {v >> 16};
    let get_minor = |v: u32| {v >> 8 & 0xff};
    let get_micro = |v: u32| {v & 0xff};
    let to_str = |v| {format!("{}.{}.{}", get_major(v), get_minor(v), get_micro(v))};

    println!("avutil52     : {} ({})", to_str(avutil52::version()), avutil52::license());
    println!("avcodec54    : {} ({})", to_str(avcodec54::version()), avcodec54::license());
    println!("avcodec55    : {} ({})", to_str(avcodec55::version()), avcodec55::license());
    println!("avformat54   : {} ({})", to_str(avformat54::version()), avformat54::license());
    println!("avformat55   : {} ({})", to_str(avformat55::version()), avformat55::license());
    println!("avdevice54   : {} ({})", to_str(avdevice54::version()), avdevice54::license());
    println!("avdevice55   : {} ({})", to_str(avdevice55::version()), avdevice55::license());
    println!("avfilter3   : {} ({})", to_str(avfilter3::version()), avfilter3::license());
    println!("avfilter4   : {} ({})", to_str(avfilter4::version()), avfilter4::license());
    println!("swresample0 : {} ({})", to_str(swresample0::version()), swresample0::license());
    println!("swscale2    : {} ({})", to_str(swscale2::version()), swscale2::license());
}
