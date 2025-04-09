pub enum TokenKind{
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
}

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}


impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self { start, end, literal }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}


pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}


pub struct Lexer<'a>{
    input: &'a str,
    current_pass: usize,
}

impl<'a> Lexer<'a>{
    pub fn new(input: &'a str) -> Self{
        Self {input, current_pass: 0}
    }
    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pass > self.input.len() {
            return None;
        }


        if self.current_pass >= self.input.len() {
            self.current_pass += 1;
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, "\0".to_string())
            ));
        }
        let start   = self.current_pass;
        let c = self.current_char();
        let mut kind = TokenKind::Bad;
        if self::is_number_start(&c){
            let number: i64 = self.tokenize_number();
            kind = TokenKind::Number(number);
        }

        let end = self.current_pass;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);
        Some(Token::new(kind, span))

    }
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10) 
    }
    fn current_char(&self) -> char {
        self.input.chars().nth(self.current_pass).unwrap()
    }
    fn consume_char(&mut self) -> Option<char> {
        let c = self.current_char();
        self.current_pass += 1;
        Some(c)
    }

}