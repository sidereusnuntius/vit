use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub vit);
pub mod ast;

fn main() {
    
    let expr = vit::ProgramParser::new()
    .parse("c = 2 + -   2.0;")
    .unwrap();
    println!("{expr:?}");
}
