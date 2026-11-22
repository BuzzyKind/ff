pub mod main_lexer {
    use std::fs::*;
    use std::thread;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::any::type_name;
    use std::iter::Enumerate;
    use std::io::{BufReader, BufRead};

    #[derive(Debug)]
    pub struct SymbolTableEntry {
        is_empty: bool,
        name: String,
    }
    #[derive(Debug)]
    pub enum TokenType {
        Colon,             // :
        EqualsSign,        // =
        Semicolon,         // ;
        LeftBrace,         // {
        RightBrace,        // }
        Apostrophe,        // '
        Dot,               // .
        Space,             //
        LeftBracket,       // (
        RightBracket,      // )
        RightArrow,        // ->
        Commander,         // _
        Kind,              // *
        Name,              // name
        NamespaceSymbol,   // ::
        Caret,             // ^
    }
    // name : type = value;
    // name.arg : (type'modif -> * ) = {
    // 
    // };
    // name := {
    //    inner := 5;
    // }
    // result := name::inner;
    //
    #[derive(Debug)]
    pub struct Token {
        token_type: TokenType,
        location: (usize, usize),
        name: usize,
    }
    #[derive(Debug)]
    pub struct TokenizedCode {
        tokens: Vec<Token>,
        symbol_table: Vec<SymbolTableEntry>,
    }
    pub fn test(files: Vec<PathBuf>) -> Vec<Token> {
        return run_lexer(files);
    }
    /*pub fn run_lexer(files: Vec<DirEntry>) -> TokenizedCode {
        let unwrapped = unwrap_files(files);
        }*/
    static symbol_table: Mutex<Vec<SymbolTableEntry>> = Mutex::new(Vec::new());
    fn run_lexer(files: Vec<PathBuf>) -> Vec<Token> {
        let mut handles: Vec<_> = vec![];
        let mut result = vec![];
        for file in files {
            let handle = thread::spawn(|| {
                println!("{:?}", thread::current().id()); // remove this print
                /*use std::fs;

                let contents: String = fs::read_to_string(file).expect("Unable to read content of this file");
                let output: TokenizedCode = lexer(contents);
                return output;*/


                let open_file = File::open(file).expect("Unable to read content of this file");
                let reader = BufReader::new(open_file);

                let mut buffer = vec![];

                for (i, line) in reader.lines()
                                       .map(|line| line.expect("line"))
                                       .enumerate() {
                    buffer.append(&mut tokenizer(line, i));
                }
                return buffer;


                /*return reader.lines()
                             .map(|line| line.expect("line"))
                             .enumerate()
                             .lexer()
                             .collect::<Vec<(usize, String)>>();*/
            });
            handles.push(handle);
        }
        for handle in handles {
            //   "append(&mut " if BufReader is used
            result.append(&mut handle.join().unwrap())//.expect("One of the threads failed"));
        }
        return result;
    }
    fn tokenizer(line: String, line_number: usize) -> Vec<Token> {

        let mut token_vec: Vec<Token> = vec![];
        let mut i: usize = 0;
        while i <= line.len() {

            let chars: Vec<char> = line.chars().collect();
            let mut name: usize = 0;
            let token_type = match chars.get(i) {
                Some(':') => match chars.get(i + 1) { Some(':') => TokenType::NamespaceSymbol, Some(_) => TokenType::Colon, None => TokenType::Space},
                Some(' ') => TokenType::Space,
                Some(letter) => {
                    name = get_name_from_symbol_table({
                        let mut word: String = letter.to_string();
                        i += 1; 
                        loop {
                            let next_char = if let Some(ch) = chars.get(i) { ch } else { break word;};
                            if next_char.is_alphanumeric() || *next_char == '_' {
                                word.push(*next_char);
                            } else { break word; }
                            i += 1;
                        }
                    }); 

                    TokenType::Name
                },
                None => return token_vec,
            };
            token_vec.push( Token { token_type: token_type, location: (line_number + 1, i), name: name });
             /*name: get_name_from_symbol_table({
                    let mut buffer: String = name.to_string();
                    let mut j: usize = 1; 
                    loop {
                        let chars_plus = if let Some(ch) = chars.get(i + j) { ch } else { break buffer;};
                        if is_name(chars_plus) {
                            buffer.push(*chars_plus);
                            j += 1;
                            continue;
                        } else { break buffer;}
                    }
                })},*/
            /*token_vec.push(match &line[i..] {
                "^" => return token_vec,
                ":" => match &line[i+1..] { ":" => Token { token_type: TokenType::NamespaceSymbol, name: 0 }, _ => Token { token_type: TokenType::Colon, name: 0}, },
                _ => Token { token_type: TokenType::Name, name: get_name_from_symbol_table({
                    let mut buffer = String::from(&line[i..]);
                    let mut j: usize = 1; loop { if is_name(&String::from(&line[i+j..])) {
                        buffer.push_str(&line[i+j..]); j += 1; continue;
                    } else { break buffer;} }
                })},
            });*/
            i += 1;
        }
        return token_vec;
    }
    fn get_name_from_symbol_table(name: String) -> usize {
        for ( i, entry ) in symbol_table.lock().unwrap().iter().enumerate() {
            return symbol_table.lock().unwrap().iter().position(|_| entry.name == name ).unwrap_or(0);
        }
        return 0;
    }
    /*fn is_name(var: char) -> bool {
        var.is_alphanumeric() || var == '_'
    }*/
}