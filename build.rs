fn main() { std::process::Command::new("sh").arg("-c").arg("x").spawn().unwrap(); }
