#[allow(non_camel_case_types)]

pub mod main_init { 

    use std::fs::*;
    use std::path::PathBuf;
    use std::io::Error;
    use std::ffi::OsStr;
    use clap::*;   // specify exact imports
    use walkdir::WalkDir;
    use walkdir::DirEntry as DirEntry_Walk; // walkdir is an experimental option
        
    #[derive(ValueEnum, Clone, Debug)]
    pub enum Mode  {
        check,
        build,
        run,
    }
    #[derive(Debug, Parser)]
    #[command(name = "frog")]
    struct Input {
        mode: Mode,
        directory: PathBuf,

        #[arg(short, long)]
        search_in_folders: bool,
        
        //#[arg(env = "frog_ff", default_value = r".\target\release\frog.exe")]    // experiment
        //exe_path: PathBuf,
        
        #[arg(num_args = 0..)]
        options: Vec<String>,
    }
    #[derive(Debug)]
    pub struct Init_Return {
        pub files: Vec<PathBuf>,
        pub mode: Mode,
    }
    pub fn initialize() -> Init_Return {
        // get args from terminal
        //let args: Vec<String> = env::args().skip(1).collect();

        let parsed: Input = Input::parse();

        return match parsed.mode {
            Mode::run | Mode::build | Mode::check => manage_files(parsed),
            _                                     => todo!("Implement other modes"),
        };
    }
    fn manage_files(parsed: Input) -> Init_Return {

        let files = if parsed.search_in_folders { 
            get_files(parsed.directory).expect("rrr") 
        } else { 
            get_files_wo_recursion(parsed.directory)
        };

        return Init_Return { files: files, mode: parsed.mode };
    }
    fn check_extension(dir: &PathBuf) -> bool {
        dir.extension()
           .and_then(OsStr::to_str)
           .map(|ext| ext.split(".").last()
                                    .unwrap()
                                    .eq_ignore_ascii_case("ff"))
            .unwrap_or(false)
    }
    fn get_files_wo_recursion(directory: PathBuf) -> Vec<PathBuf> {  // compact version / TODO: experiment with different versions
        return read_dir(directory).expect("Directory is invalid or not specified")
                                    .into_iter()
                                    .map(|dir| dir.expect("a"))
                                    .filter(|dir| !dir.file_type().expect("b").is_dir() && check_extension(&dir.path()))
                                    .map(|dir| dir.path())
                                    .collect();
    }
    fn get_files_walkdir_compact(directory: PathBuf) -> Vec<DirEntry_Walk> {
        return WalkDir::new(directory).into_iter()
                                      .map(|dir| dir.expect("a"))
                                      .filter(|dir| !dir.file_type().is_dir())
                                      .collect();
    }
    fn get_files_walkdir_loop(directory: PathBuf) -> Result<Vec<DirEntry_Walk>, Error> {
        let mut result = vec![];
        for entry in WalkDir::new(directory) {
            let entry = entry?;
            if entry.file_type().is_dir() {

            } else {
                result.push(entry);
            }
        }
        return Ok(result);
    }
    fn get_files_custom_stack(directory: PathBuf) -> Result<Vec<DirEntry>, Error> {
        let mut result = Vec::new();
        let mut stack: Vec<PathBuf> = vec![directory];

        while let Some(current_directory) = stack.pop() {
            for entry in read_dir(current_directory).expect("Directory is invalid or not specified") {
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
    // stack overflow >:]
    fn get_files(directory: PathBuf) -> Result<Vec<PathBuf>, Error> {
        let mut result: Vec<PathBuf> = vec![];
        for entry in read_dir(&directory).expect("Directory is invalid or not specified") {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                result.append(&mut get_files(path).unwrap());
            } else if check_extension(&path) {
                result.push(path);
            } else { }
        }
        return Ok(result);
    }
}