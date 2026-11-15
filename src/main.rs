use std::env;

mod init;
use init::main_init;

fn main() {
    main_init::initialize();

    let args = env::args();
    println!("{:?}", args)
}
