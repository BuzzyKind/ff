#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;  // temporary/dev stuff

mod utils;
mod init_clap;
use init_clap::main_init;
mod lexer;
use lexer::main_lexer;

fn main() { // add notification for cases when compiler sees nothing / improve CLI in general

    let _profiler = dhat::Profiler::new_heap();

    let init_info = main_init::initialize();
    println!("{:?}", init_info); // remove this print

    let lexed = main_lexer::run_lexer(init_info.files);
    println!("{:?}", lexed); // remove this print
}