#!/bin/sh

mkdir -p lib bin

for lib in src/{avutil52,avcodec54,avformat54,avfilter3,avdevice54,swresample0,swscale2}; do
    echo Building $lib ...
    rustc -L lib --out-dir lib $lib/lib.rs
done

echo Building rust-ffmpeg ...
rustc -L lib --out-dir bin src/rust-ffmpeg/main.rs
