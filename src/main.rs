mod parser;
use parser::lexer::{
    tokenize,
    from_str_vec,
    Token,
};

use parser::parser::{
    VizibilityParser
};

fn main() {
    let tokens = from_str_vec(vec![
        (r"\bfn\b", Token::Fn),
        (r"\bdo\b", Token::Do),
        (r"\bend\b", Token::Int),
        (r"\blet\b", Token::Let),
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

    let parser = VizibilityParser::new(result, "fn hewo() end");
    let result = parser.program();
    match result {
        Ok(val) => println!("{:?}", val),
        Err(err) => {}
    }
}
