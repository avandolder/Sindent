use sindent::lexer::scan;

fn main() {
  let src = r#"a

    b c 1

  3
4"#;
  println!("{:#?}", scan(src));
}
