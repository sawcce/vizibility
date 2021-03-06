mod parser;
use parser::lexer::{
    tokenize,
    from_str_vec,
    Token,
};

use parser::parser::{
    VizibilityParser,
    program
};

fn main() {
    let tokens = from_str_vec(vec![
        (r"\bfn\b", Token::Fn),
        (r"\bdo\b", Token::Do),
        (r"\bend\b", Token::End),
        (r"\blet\b", Token::Let),
        (r"\bmut\b", Token::Mut),
        (r"\(", Token::LParen),
        (r"\)", Token::RParen),
        ("=", Token::Equals),
        (r"(\w)(\w|[0-9])*", Token::Identifier),
        (r"(\s)+", Token::Skipped),
    ]);

    let result = tokenize(tokens, "fn hewo() end");

    //let joined: Vec<String> = result.iter().map(|s| format!("{}", s)).collect();
    //println!("{}", joined.join(" "));

    let parser = VizibilityParser::new(result, "fn hewo() end");
    let result = program(parser);
    
    match result {
        Ok(val) => println!("Program => {:?}", val),
        Err(error) => println!("{}", error),
    }
}