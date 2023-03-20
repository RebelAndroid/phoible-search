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
    Rparen,
    #[regex("[a-z]", |l| l.slice().to_string())]
    Phoneme(String),
    #[error]
    Error,
}

enum BinaryOperator {
    AND,
    OR,
}

enum Expression {
    Literal(String),
    Grouping(Box<Expression>),
    Unary(Box<Expression>),
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

    fn new(input: &str) -> Self{
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

    fn expression(&mut self) -> Expression {
        Self::binary(self)
    }

    fn binary(&mut self) -> Expression {
        let ex = unary(self);
    }

    fn unary(&mut self) -> Expression{
        Expression::Literal("".to_owned())
    }
}
