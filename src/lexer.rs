/// Lexer's for lisps are generally pretty simple.
/// This one is somewhat more complicated, as it needs to
/// track indentation levels on lines.
pub(crate) struct Lexer<'src> {
  src: &'src str,
  line: usize,
  offset: usize,
}

pub(crate) struct Token<'src> {
  span: &'src str,
  line: usize,
  start_offset: usize,
  end_offset: usize,
  kind: TokenKind<'src>,
}

pub(crate) enum TokenKind<'src> {
  Id(&'src str),
  Number(&'src str),
  LBracket,
  RBracket,
  LParen,
  RParen,
}

impl<'src> Iterator for Lexer<'src> {
  type Item = Token<'src>;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}
