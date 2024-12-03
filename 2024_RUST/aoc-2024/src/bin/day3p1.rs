use std::{collections::HashMap, env::args, fs, process::exit};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Mul,
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

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("mul".to_string(), Token::Mul);

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
            'm' => self.keyword(),
            _ => {
                if self.is_digit(c) {
                    self.number();
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
        let token = if text == "mul" || text.ends_with("mul") { Token::Mul } else { Token::Other };
        self.tokens.push(token);
    }

    fn is_alpha(&mut self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
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

    fn parse(&mut self) -> Vec<(u64, u64)> {
        let mut exprs: Vec<(u64, u64)> = Vec::new();

        while !self.is_at_end() {
            if !matches!(self.next_token(), Token::Mul) {
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

            if !matches!(self.next_token(), Token::Comma) {
                continue;
            }

            let right = match self.next_token() {
                Token::Number(right) => right,
                _ => {
                    continue;
                }
            };

            if !matches!(self.next_token(), Token::RightParen) {
                continue;
            }

            println!("mul ({:?}, {:?})", left, right);
            exprs.push((left, right));
        }

        exprs
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

    let mut parser: Parser = Parser::new(tokens.clone());
    let mul_exprs: Vec<(u64, u64)> = parser.parse();

    let mut result: u64 = 0;
    for expr in &mul_exprs {
        result += expr.0 * expr.1;
    }

    println!("{:?} expressions", mul_exprs.len());
    println!(">> {}", result);

}

/*
    179380102   :: low
    179380102
    180521880   :: not right
    89099301    :: wtf
*/
