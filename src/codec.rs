use std::process::Command;

#[derive(Default)]
pub struct Codec {
    test_name: &'static str,
    name: &'static str,
    executable_path: &'static str,
    bitrate_template: &'static str,
    mode_template: &'static str,
    decoding_flag: &'static str,
}

impl Codec {
    pub fn new(executable_path: &'static str) -> Codec {
        let mut codec = Codec::default();
        codec.executable_path = executable_path;
        return codec;
    }

    pub fn run(&self) {
        let output = Command::new(self.executable_path).output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
