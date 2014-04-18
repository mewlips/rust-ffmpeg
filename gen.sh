#!/bin/sh

GEN_DIR=gen
#FFMPEG_VERSIONS="1.2.6 2.1.4 2.2.1"
FFMPEG_VERSIONS="1.2.6"
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

get_major_version() {
    version_h=$1
    sed 's/  */ /g' $version_h | grep 'VERSION_MAJOR [0-9]' | cut -d ' ' -f 3
}

gen_rs() {
    local ver="$1"
    local inc="$ver/include"
    local rs="$ver/rs"
    if [ -d /usr/lib64/clang/3.4 ]; then
        local clang_ver=3.4
    elif [ -d /usr/lib64/clang/3.3 ]; then
        local clang_ver=3.3
    fi
    local bindgen_opts="-I/usr/lib64/clang/$clang_ver/include -I/usr-builtins -builtins -allow-bitfields"

    export LD_LIBRARY_PATH=$(llvm-config --libdir)
    export CPATH=${inc}

    for lib in avcodec avfilter avformat avdevice swresample swscale avutil avformat; do
        echo bindgen ${lib}...
        case $lib in
            "avutil")
                additional_matchs="-match rational.h -match dict.h -match pixfmt.h -match log.h \
                                   -match samplefmt.h -match mem.h -match error.h -match mathematics.h \
                                   -match time.h -match channel_layout.h -match opt.h"
                additional_includes="-include ${inc}/lib${lib}/samplefmt.h \
                                     -include ${inc}/lib${lib}/dict.h \
                                     -include ${inc}/lib${lib}/time.h \
                                     -include ${inc}/lib${lib}/channel_layout.h \
                                     -include ${inc}/lib${lib}/opt.h"
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

        local rs_file="${rs}/${lib}$(get_major_version ${inc}/lib${lib}/version.h)/lib.rs"
        mkdir -p $(dirname ${rs_file})
        $BINDGEN ${bindgen_opts} -match ${lib}.h ${additional_matchs} -l ${lib} \
                 -o $rs_file ${inc}/lib${lib}/${lib}.h \
                 ${additional_includes}

        if [ ${lib} != "avutil" ]; then
            for ty in Enum_AVMediaType Enum_AVPictureType AVRational AVDictionary \
                      AVClass Enum_AVPixelFormat Enum_AVSampleFormat; do
                sed "s/$ty,/avutil52::$ty,/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty)/avutil52::$ty)/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty /avutil52::$ty /g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty;/avutil52::$ty;/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty>/avutil52::$ty>/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
            done
        fi
        if [ ${lib} != "avcodec" ]; then
            for ty in AVCodec Enum_AVCodecID AVPacket AVCodecContext Enum_AVDiscard \
                      Struct_AVCodecParserContext AVFrame; do
                sed "s/$ty,/avcodec54::$ty,/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty)/avcodec54::$ty)/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty /avcodec54::$ty /g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty;/avcodec54::$ty;/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
                sed "s/$ty>/avcodec54::$ty>/g" < ${rs_file} > ${rs_file}.new
                mv ${rs_file}.new ${rs_file}
            done
        fi
    done

}

for version in ${FFMPEG_VERSIONS}; do
    #fetch ${version} && build ${version} && gen_rs ${version}
    gen_rs ${version}
done
