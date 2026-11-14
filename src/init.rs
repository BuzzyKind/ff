
pub mod main_init { 
    use std::fs;
    use std::env;
    use std::mem;
    use std::process;
    use std::path::PathBuf;

    #[derive(PartialEq, Eq)]
    enum Mode {
        Check,
        Build,
        Run,
        Error
    }
    pub fn initialize() -> Result<Vec<fs::DirEntry>, ()> {
        //get args from terminal (path, args)
        let args: Vec<String> = env::args().collect(); // vec of strings
        //println!("c");
        let mode: Mode = select_mode(&args[1]);
        //println!("d");
        if mode == Mode::Error { return Err(()); };
        let directories = check_files(PathBuf::from(args[2].clone()));
        return Ok(directories); 
    }
    fn select_mode(mode: &String) -> Mode {
        //let tm = &mode as &str == "run";
        //println!("{}", tm);
        //println!("{}", mode);
        let result: Mode = match &mode as &str {
            "check" => Mode::Check,
            "build" => Mode::Build,
            "run"   => Mode::Run,
                 _  => Mode::Error
        };
        return result;
    }
    fn get_directories(directory: PathBuf) -> Vec<fs::DirEntry> {
        let read_directory = fs::read_dir(directory).expect("This is not a directory");
        let mut directory_entry: Vec<fs::DirEntry> = vec![]; for entry in read_directory { directory_entry.push(entry.expect("Smth went wrong ig"));};
        println!("{:?}", directory_entry);
        return directory_entry;
    }
    fn check_files(directory: PathBuf) -> Vec<fs::DirEntry> {
        let mut directories: &mut Vec<fs::DirEntry> = &mut get_directories(directory);
        let mut i = 0;
        while i < directories.len() {
            let file_type = directories[i].file_type().expect("Smth went wrong ig");
            //println!("b");
            if file_type.is_dir() {
                //println!("a");
                directories.append(&mut check_files(directories[i].path()))
            }
            i += 1;
        }
        return mem::replace(directories, Vec::new());
    }
}