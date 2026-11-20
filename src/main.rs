mod init_clap;
use init_clap::main_init;

fn main() {

    let directories = main_init::initialize();
    println!("{:?}", directories); // remove this print
    let mode = directories.mode;
    let files = directories.files;
}