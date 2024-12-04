use std::{collections::HashMap, env::args, fs, process::exit};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Mul,
    Do,
    Dont,
    Number(u64),
    LeftParen,
    RightParen,
    Comma,
    Other
}

pub struct Lexer {
    source: String,
    keywords: HashMap<String, Token>,
    tokens: Vec<Token>,

    start: u64,
    current: u64,
}

pub enum Statement {
    Expr(u64, u64),
    Do,
    Dont
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("mul".to_string(), Token::Mul);
        keywords.insert("do".to_string(), Token::Do);
        keywords.insert("don't".to_string(), Token::Dont);

        Self {
            source,
            keywords,
            tokens: Vec::new(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.to_vec()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.tokens.push(Token::LeftParen),
            ')' => self.tokens.push(Token::RightParen),
            ',' => self.tokens.push(Token::Comma),
            'm' | 'd' => self.keyword(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else {
                    self.tokens.push(Token::Other)
                }
            }
        }
    }

    fn keyword(&mut self) {
        let mut current_char: char = self.peek();
        while self.is_alpha(current_char) {
            self.advance();
            current_char = self.peek();
        }

        let text: String = self.source[self.start as usize..self.current as usize].to_string();
        let token = self.keywords.get(&text).or(Some(&Token::Other));
        self.tokens.push(token.unwrap().clone());
    }

    fn is_alpha(&mut self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '\''
    }

    fn number(&mut self) {
        let mut current_char: char = self.peek();
        while self.is_digit(current_char) {
            self.advance();
            current_char = self.peek();
        }
        let lexeme: String = self.source[(self.start as usize)..(self.current as usize)].to_string();
        self.tokens.push(
            Token::Number(lexeme.parse::<u64>().unwrap())
        );
    }

    fn peek(&mut self) -> char {
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn is_digit(&mut self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&mut self) -> bool {
        (self.current as usize) >= self.source.chars().count()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .chars()
            .nth((self.current - 1) as usize)
            .unwrap()
    }

}

struct Parser {
    tokens: Vec<Token>,
    current: u64,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse(&mut self) -> Vec<Statement> {
        let mut stmts: Vec<Statement> = Vec::new();

        while !self.is_at_end() {
            match self.tokens.get(self.current as usize).unwrap() {
                Token::Mul => {
                    if !matches!(self.next_token(), Token::Mul) {
                        continue;
                    }

                    if let Token::Mul = self.tokens.get(self.current as usize).unwrap() {
                        continue;
                    }

                    if !matches!(self.next_token(), Token::LeftParen) {
                        continue;
                    }

                    let left = match self.next_token() {
                        Token::Number(val) => val,
                        _ => {
                            continue;
                        }
                    };

                    if let Token::Mul = self.tokens.get(self.current as usize).unwrap() {
                        continue;
                    }

                    if !matches!(self.next_token(), Token::Comma) {
                        continue;
                    }

                    let right = match self.next_token() {
                        Token::Number(right) => right,
                        _ => {
                            continue;
                        }
                    };

                    if let Token::Mul = self.tokens.get(self.current as usize).unwrap() {
                        continue;
                    }

                    if !matches!(self.next_token(), Token::RightParen) {
                        continue;
                    }

                    stmts.push(Statement::Expr(left, right));
                }
                Token::Do => {
                    stmts.push(Statement::Do);
                    self.current += 1;
                }
                Token::Dont => {
                    stmts.push(Statement::Dont);
                    self.current += 1;
                }
                _ => {
                    self.current += 1;
                }
            }
        }

        stmts
    }

    fn next_token(&mut self) -> Token {
        self.advance();
        self.tokens.get((self.current - 1) as usize).unwrap().clone()
    }

   fn advance(&mut self) {
       self.current += 1;
   }

   fn is_at_end(&mut self) -> bool {
       (self.current as usize) >= self.tokens.len()
   }

}

fn main() {
    let program: String = match args().next() {
        Some(name) => name,
        None => {
            eprintln!("Program name missing.");
            exit(1);
        }
    };

    let filename: String = match args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Usage: {program} <filename>");
            exit(1);
        }
    };

    let source: String = fs::read_to_string(filename).unwrap();
    let mut lexer: Lexer = Lexer::new(source);
    let tokens: Vec<Token> = lexer.scan_tokens();
    println!("{:?}", tokens);

    let mut parser: Parser = Parser::new(tokens.clone());
    let stmts: Vec<Statement> = parser.parse();

    let mut result: u64 = 0;
    let mut is_on: bool = true;
    for stmt in &stmts {
        match stmt {
            Statement::Expr(left, right) => {
                if is_on {
                    result += left * right;
                }
            }
            Statement::Do => {
                is_on = true;
            }
            Statement::Dont => {
                is_on = false;
            }
        }
    }

    println!("{:?} expressions", stmts.len());
    println!(">> {}", result);

}

/*
    179380102   :: low
    180823518   :: not right
    184300131   :: not right
    183788984
    180521880   :: not right
    89099301    :: wtf
*/
