use vit::parser::Parser;

#[test]
fn even_or_odd() {
    let program = Parser::new().parse("
        let num;
        write 'Input a number: ';
        read num;
        write 'The number is ';
        if num % 2 == 0 {
            write 'even.\\n';
        } else {
            write 'odd.\\n'; 
        }
    ").unwrap();

    let result = vit::vit::build(program).unwrap();
    assert_eq!(result, "ldc \"Input a number: \"\nwri\nlda #0\nrd\nsto\nldc \"The number is \"\nwri\nlod #0\nlod #0\nldc 2\ndiv\nto int\nldc 2\nmul\nsub\nldc 0\nequ\nfjp F0\nldc \"even.\\n\"\nwri\nujp E0\nF0:\nldc \"odd.\\n\"\nwri\nE0:\nstp\n")

}