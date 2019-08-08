extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/wirehair/wirehair.cpp")
        .file("src/wirehair/gf256.cpp")
        .file("src/wirehair/WirehairCodec.cpp")
        .file("src/wirehair/WirehairTools.cpp")
        .include("src/wirehair")
        .flag("-msse4.1")
        .shared_flag(true)
        .compile("wirehair");

    cc::Build::new()
        .cpp(true)
        .file("src/siamese/Logger.cpp")
        .file("src/siamese/PacketAllocator.cpp")
        .file("src/siamese/siamese.cpp")
        .file("src/siamese/SiameseCommon.cpp")
        .file("src/siamese/SiameseDecoder.cpp")
        .file("src/siamese/SiameseEncoder.cpp")
        .file("src/siamese/SiameseTools.cpp")
        .include("src/siamese")
        .flag("-msse4.1")
        .shared_flag(true)
        .compile("siamese");
}
