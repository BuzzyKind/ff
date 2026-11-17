// РАБОТАЕТ - НЕ ТРОГАЙ
pub mod main_init { 
    use std::fs::*;
    use std::env;
    use std::io::Error;
    use std::path::PathBuf;

    #[derive(PartialEq, Eq)]
    enum Mode {
        Check,
        Build,
        Run,
        Error
    }
    pub fn initialize() -> Result<Vec<DirEntry>, ()> {    //replace unit with enum
        //get args from terminal (path, args)
        let args: Vec<String> = env::args().collect();
        let mode: Mode = select_mode(args.get(1).expect("Mode is not specified"));
        if mode == Mode::Error { return Err(()); };
        let directories = get_directories(PathBuf::from(args.get(2).expect("Directory is not specified"))).expect("aaaa");
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
    fn get_directories(directory: PathBuf) -> Result<Vec<DirEntry>, Error> {
        let mut result = Vec::new();
        let mut stack: Vec<PathBuf> = vec![directory];

        while let Some(current_directory) = stack.pop() {
            for entry in read_dir(current_directory).expect("This is not a directory") {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    stack.push(entry.path());
                } else {
                    result.push(entry);
                }
            }
        }
        return Ok(result);
    }
    /*fn get_directories_depr(directory: PathBuf) -> Vec<DirEntry> {
        // my shitty replacement for recursion
        let mut buffer: Vec<DirEntry> = vec![];
        loop {
            let mut tracker = false;
            let read_directory = read_dir(&directory).expect("This is not a directory");
            for entry in read_directory {
                let entry = entry.expect("a");
                let is_dir: bool = entry.file_type().expect("b").is_dir();
                if is_dir {
                    directory = entry.path();
                    tracker = true;
                }
                if !is_dir {
                    buffer.push(entry);
                }
            }
            if tracker == true {
                continue;
            }
            break;
        };
        return buffer;
    }*/
}