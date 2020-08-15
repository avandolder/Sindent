/// Lexer's for lisps are generally pretty simple.
/// This one is somewhat more complicated, as it needs to
/// track indentation levels on lines.
pub(crate) struct Lexer<'src> {
  src: &'src str,
  line: usize,
  offset: usize,
}

impl<'src> Iterator for Lexer<'src> {
  type Item = Token<'src>;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}
