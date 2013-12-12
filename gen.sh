#!/bin/sh

GEN_DIR=gen
#FFMPEG_VERSIONS="1.0.8 1.2.4 2.0.2 2.1.1"
FFMPEG_VERSIONS="1.2.4"
BINDGEN=bindgen

mkdir -p ${GEN_DIR}
cd ${GEN_DIR}

fetch() {
    local ver="$1"
    mkdir -p $ver/tar && \
    (cd $ver/tar && \
        wget -c http://www.ffmpeg.org/releases/ffmpeg-${ver}.tar.bz2 && \
        cd .. && tar -xjf tar/ffmpeg-${ver}.tar.bz2)
}

build() {
    local ver="$1"
    mkdir -p $ver/build
    (cd $ver/build && \
        if [ "$ver" = "1.0.8" ]; then
            ../ffmpeg-${ver}/configure --enable-shared --disable-static --prefix=..
        else
            ../ffmpeg-${ver}/configure --enable-shared --disable-programs --disable-static --prefix=..
        fi && make -j8 install)
}

gen_rs() {
    local ver="$1"
    local inc="$ver/include"
    local rs="$ver/rs"
    local bindgen_opts="-I/usr/lib64/clang/3.3/include -I/usr-builtins -builtins -allow-bitfields"

    export LD_LIBRARY_PATH=$(llvm-config --libdir)
    export CPATH=${inc}

    mkdir -p ${rs}/{avcodec,avfilter,avformat,avdevice,swresample,swscale,avutil,avformat}

    for lib in avcodec avfilter avformat avdevice swresample swscale avutil avformat; do
        echo bindgen ${lib}...
        case $lib in
            "avutil")
                additional_matchs="-match rational.h -match dict.h -match pixfmt.h -match log.h \
                                   -match samplefmt.h -match mem.h -match error.h -match mathematics.h"
                additional_includes="-include ${inc}/lib${lib}/samplefmt.h \
                                     -include ${inc}/lib${lib}/dict.h"
                ;;
            "avformat")
                additional_matchs="-match avio.h"
                additional_includes="-include ${inc}/lib${lib}/avio.h"
                ;;
            *)
                additional_matchs=""
                additional_includes=""
                ;;
        esac
        $BINDGEN ${bindgen_opts} -match ${lib}.h ${additional_matchs} -l ${lib} \
                 -o ${rs}/${lib}/lib.rs ${inc}/lib${lib}/${lib}.h \
                 ${additional_includes}
    done

}

for version in ${FFMPEG_VERSIONS}; do
    fetch ${version} && build ${version} && gen_rs ${version}
done
