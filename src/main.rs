mod init;
use init::main_init;

fn main() {
    let directories = main_init::initialize();
    println!("{:?}", directories);

}
