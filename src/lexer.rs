pub struct Token{
    pub token_type: TokenType,

    pub line: usize,
    pub col: usize,

    start: usize,
    length: usize,
}


#[derive(PartialEq, Eq)]
pub enum TokenType{
    Plus, Minus, Start, Slash,
    LeftParen, RightParen,

    IntLiteral(u32),

    EOF,

    Error,
}


impl Token{
    pub fn to_string<'a>(&'a self, source: &'a str) -> &str{
        if self.token_type ==  TokenType::EOF{
            "EOF"
        }
        else{
            &source[self.start..(self.start+self.length)]
        }
    }   
}




pub enum LexerOutput{
    LexInfo{
        file_text: String,
        tokens: Vec<Token>,
        errors: Vec<String>,
        can_compile: bool,
    },
    Failure(String)
}


pub fn lex() -> LexerOutput{
    let input: thread::Result<Vec<String>> = catch_unwind(||{args().collect()});
    if input.is_err(){
        return LexerOutput::Failure(String::from("error: could not read command line arguments"));
    }
    let input: Vec<String> = input.ok().expect("should be valid as error handled earlier");
    if input.len() > 2
    {
        return LexerOutput::Failure(String::from("error: too many command line arguments"));
    }
    if input.len() == 1
    {
        return LexerOutput::Failure("error: not enough command line arguments".to_string());
    }
    let file_path: &String = &input[1];
    let file_text: Result<String, Error> = read_to_string(file_path);
    if file_text.is_err()
    {
        return LexerOutput::Failure(String::from(format!("error: could not open file \"{file_path}\"")));
    }

    








}

