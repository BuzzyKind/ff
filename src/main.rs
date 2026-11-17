mod init_serde;
use init_serde::main_init; // file_name().into_string()

fn main() {
    let directories = main_init::initialize();//.expect("Invalid Arguments"); // TODO: specify exactly which args are missing
    println!("{:?}", directories);
    let mode = directories.mode;
    let files = directories.files;


}