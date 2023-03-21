use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("!")]
    Not,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex("[^()&|!]+", |l| l.slice().to_string())]
    Phoneme(String),
    #[error]
    Error,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    AND,
    OR,
}

#[derive(Debug)]
pub enum Expression {
    Literal(String),
    Grouping(Box<Expression>),
    Not(Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! matches{
    ( $self:expr, $($x:ident),* ) => {
        match $self.tokens[$self.current]{
            $(
                Token::$x => {$self.current += 1; true},
            )*
            _ => false,
        }
    };
}

impl Parser {

    pub fn new(input: &str) -> Self{
        let tokens: Vec<Token> = Token::lexer(input).into_iter().collect();
        Parser{
            tokens,
            current: 0,
        }
    }

    // expression -> binary;
    // binary ->  unary (("&" | "|") unary)*;
    // unary -> "!" unary | terminal;
    // terminal -> [a-z] | "(" expression ")";

    pub fn expression(&mut self) -> Box<Expression> {
        self.binary()
    }

    fn binary(&mut self) -> Box<Expression> {
        let mut ex = self.unary();
        while self.current < self.tokens.len() && matches!(self, And, Or){
            let operator = match self.tokens[self.current - 1]{
                Token::And => BinaryOperator::AND,
                Token::Or => BinaryOperator::OR,
                _ => unreachable!(),
            };
            let ex2 = self.unary();
            ex = Box::new(Expression::Binary(ex, operator, ex2));
        }
        ex
    }

    fn unary(&mut self) -> Box<Expression>{
        if matches!(self, Not){
            Box::new(Expression::Not(self.unary()))
        }else{
            self.terminal()
        }
    }

    fn terminal(&mut self) -> Box<Expression>{
        if matches!(self, LParen){
            let x = self.expression();
            assert!(matches!(self, RParen));
            x
        }else{
            match &self.tokens[self.current]{
                Token::Phoneme(s) => {
                    self.current += 1;
                    Box::new(Expression::Literal(s.to_string()))
                }
                _ => unreachable!(),
            }
        }
    }

}
