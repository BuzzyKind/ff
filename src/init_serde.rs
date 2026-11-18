#[allow(non_camel_case_types)]

pub mod main_init { 
    
    use std::fs::*;
    use std::env;
    use std::path::PathBuf;
    use std::io::Error;
    use std::vec::IntoIter;
    use serde::de::IntoDeserializer;
    use serde::de::value::SeqDeserializer;
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Deserialize, Serialize)]
    pub enum Mode {
        check,
        build,
        run,
    }
    #[derive(Debug, Deserialize, Serialize)]
    struct Input {
        program_name: String,
        mode: Mode,
        directory: String,
       // options: Vec<String>,
    }
    #[derive(Debug)]
    pub struct Init_Return {
        pub files: Vec<DirEntry>,
        pub mode: Mode,
    }
    pub fn initialize() -> Init_Return {
        // get args from terminal (path, args)
        let args: Vec<String> = env::args().collect();
        let deser: Input = Input::deserialize::<SeqDeserializer<IntoIter<String>, _>>(
                                            <Vec<String> as IntoDeserializer<'_>>
                                            ::into_deserializer(args)).expect("Deserialization error");
        return Init_Return { files: get_files_wo_recursion(deser.directory.into()).expect("rrr"), mode: deser.mode };
    }
    fn get_files_wo_recursion(directory: PathBuf) -> Result<Vec<DirEntry>, Error> {
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
    fn get_files(directory: PathBuf) -> Result<Vec<DirEntry>, Error> {
        let mut result: Vec<DirEntry> = vec![];
        for entry in read_dir(directory).expect("Directory is invalid or not specified") {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                result.append(&mut get_files(entry.path()).unwrap())
            } else {
                result.push(entry);
            }
        }
        return Ok(result);
    }
}