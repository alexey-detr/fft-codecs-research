use std::path::Path;

mod wav;

fn main() {
    let path = Path::new("a.wav");
    wav::read_header(path);
}
