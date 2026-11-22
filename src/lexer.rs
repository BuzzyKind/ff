pub mod main_lexer {
    use std::fs::*;
    use std::mem;
    use std::thread;
    use std::sync::{Mutex, LazyLock};
    use std::path::PathBuf;
    use std::io::{BufReader, BufRead};
    use indexmap::IndexSet;

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
    #[derive(Debug)]
    pub struct Token {
        token_type: TokenType,
        location: (usize, usize),
        name: usize,
    }
    #[derive(Debug)]
    pub struct TokenizedCode {
        tokens: Vec<Token>,
        symbol_table: IndexSet<String>,
    }
    static SYMBOL_TABLE: LazyLock<Mutex<IndexSet<String>>> = LazyLock::new(|| Mutex::new(IndexSet::new())); // kinda nah
    pub fn run_lexer(files: Vec<PathBuf>) -> TokenizedCode {
        let mut handles = vec![];
        let mut result = vec![];
        for file in files {
            let handle = thread::spawn(|| {
                println!("{:?}", thread::current().id()); // remove this print

                let open_file = File::open(file).expect("Unable to read content of this file");
                let reader = BufReader::new(open_file);

                let mut tokens = vec![];

                for (line_number, line) in reader.lines()
                                                 .map(|line| line.expect("line"))
                                                 .enumerate() {
                    tokens.append(&mut tokenizer(line, line_number));
                }
                return tokens;
            });
            handles.push(handle);
        }
        for handle in handles {
            result.append(&mut handle.join().unwrap());
        }
        return TokenizedCode { tokens: result, symbol_table: mem::take(&mut SYMBOL_TABLE.lock().unwrap()) };
    }
    fn tokenizer(line: String, line_number: usize) -> Vec<Token> {

        let mut token_vec: Vec<Token> = vec![];
        let mut i: usize = 0;
        while i <= line.len() {
            let chars: Vec<char> = line.chars().collect();
            let mut name: usize = 0;
            let token_type = match chars.get(i) {
                Some(':') => match chars.get(i + 1) { Some(':') => { i += 1; TokenType::NamespaceSymbol }, Some(_) | None => TokenType::Colon},
                Some('=') => TokenType::EqualsSign,
                Some(';') => TokenType::Semicolon,         
                Some('{') => TokenType::LeftBrace,         
                Some('}') => TokenType::RightBrace,        
                Some('\'') => TokenType::Apostrophe,       
                Some('.') => TokenType::Dot,               
                Some('(') => TokenType::LeftBracket,       
                Some(')') => TokenType::RightBracket,      
                Some('-') => match chars.get(i + 1) { Some('>') => { i += 1; TokenType::RightArrow }, Some(_) | None => TokenType::Caret},    
                Some('_') => TokenType::Commander,         
                Some('*') => TokenType::Kind,                    
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
                    i -= 1;
                    TokenType::Name
                },
                None => return token_vec,
            };
            token_vec.push( Token { token_type: token_type, location: (line_number + 1, i), name: name });
            i += 1;
        }
        return token_vec;
    }
    fn get_name_from_symbol_table(name: String) -> usize {
        let index = SYMBOL_TABLE.lock().unwrap().get_index_of(&name);
        if index.is_some() {
            return index.unwrap();
        } else { 
            return SYMBOL_TABLE.lock().unwrap().insert_full(name).0;
        }
    }
}