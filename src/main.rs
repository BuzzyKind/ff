use std::time::Instant;

mod init_serde;
use init_serde::main_init;

fn main() {

    let now = Instant::now();

    let directories = main_init::initialize();
    println!("{:?}", directories);
    let mode = directories.mode;
    let files = directories.files;

    println!("{:?}", now.elapsed())

}
// latency stats for ./ directory without recursion (serde)
// 31, 64, 42, 38, 30, 32, 33, 28 ms
// latency stats for ./ directory with recursion (serde)
// 29, 31, 39, 28, 33, 42, 41, 38 ms
// latency stats for ./ directory without serde
// 30, 31, 37, 35, 43, 37, 31, 31