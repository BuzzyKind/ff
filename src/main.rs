mod init;
use init::main_init;

fn main() {
    let directories = main_init::initialize().expect("Invalid Arguments"); // TODO: specify exactly which args are missing
    println!("{:?}", directories);



}