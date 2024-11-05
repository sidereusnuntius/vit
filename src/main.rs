use vit::parser;

fn main() {
    
    let expr = parser::Parser::new()
    .parse("c = 2 + -   2.0;")
    .unwrap();
    println!("{expr:?}");
}
