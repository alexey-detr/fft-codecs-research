use std::path::Path;

mod wav;
mod codec;

fn main() {
    let path = Path::new("a.wav");
    wav::read_header(path);

    let codec = codec::Codec::new("lame");
    codec.run();
}
