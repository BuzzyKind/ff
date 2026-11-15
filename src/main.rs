mod init;
use init::main_init;

fn main() {
    let directories = main_init::initialize().expect("Missing some neccessary arguments"); // TODO: specify exactly which args are missing
    println!("{:?}", directories);



}