mod parser;
use parser::lexer::{
    tokenize,
    from_str_vec,
    Token,
};

fn main() {
    let tokens = from_str_vec(vec![
        (r"\bfn\b", Token::Fn),
        (r"\bdo\b", Token::Do),
        (r"\bend\b", Token::Int),
        (r"\blet\", Token::Let),
        (r"\bmut\b", Token::Mut),
        (r"\(", Token::LParen),
        (r"\)", Token::RParen),
        ("=", Token::Equals),
        (r"(\w)(\w|[0-9])*", Token::Identifier),
        (r"(\s)+", Token::Skipped),
    ]);

    let result = tokenize(tokens, "fn hewo() end");

    let joined: Vec<String> = result.iter().map(|s| format!("{}", s)).collect();
    println!("{}", joined.join(" "));
}



pub enum IntType {
    I1(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}
