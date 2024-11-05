use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub vit);
pub mod ast;

fn main() {
    
    let expr = vit::ProgramParser::new()
    .parse("\
    let a = 0;
    loop {
        write a;
        a = a + 1;
        if a == 10 {
            break;
        }
    }
    write 'END OF THE PROGRAM\n';")
    .unwrap();
    println!("{expr:?}");
}
