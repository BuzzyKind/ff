
pub mod main_init { 
    use std::fs;
    use std::env;
    use std::mem;
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
        let args: Vec<String> = env::args().collect();
        let mode: Mode = select_mode(&args[1]);
        if mode == Mode::Error { return Err(()); };
        let directories = check_files(PathBuf::from(args[2].clone())); // remove clone
        return Ok(directories); 
    }
    fn select_mode(mode: &String) -> Mode {
        let result: Mode = match &mode as &str {
            "check" => Mode::Check,
            "build" => Mode::Build,
            "run"   => Mode::Run,
                 _  => Mode::Error
        };
        return result;
    }
    fn get_directories(directory: PathBuf) -> Vec<fs::DirEntry> {
        let read_directory: fs::ReadDir = fs::read_dir(directory).expect("This is not a directory");
        let directory_entry: Vec<fs::DirEntry> = read_directory.map( |result| result.expect("aaa") ).collect();
        //println!("{:?}", directory_entry);
        return directory_entry;
    }
    fn check_files(directory: PathBuf) -> Vec<fs::DirEntry> {
        let directories: &mut Vec<fs::DirEntry> = &mut get_directories(directory);
        let mut i = 0;
        while i < directories.len() {
            let file_type = directories[i].file_type().expect("Smth went wrong ig");
            if file_type.is_dir() {
                directories.append(&mut check_files(directories[i].path()))
            }
            i += 1;
        }
        return mem::replace(directories, Vec::new());
    }
}