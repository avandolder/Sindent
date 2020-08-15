/// An AST node, along with information about it's source location.
pub(crate) struct Ast<'a> {
  kind: AstKind<'a>,
  line: usize,
  start_offset: usize,
  end_offset: usize,
  span: &'a str,
}

/// The kind of an AST node.
pub(crate) enum AstKind<'a> {
  List(Vec<&'a Ast<'a>>),
  Quote(Vec<&'a Ast<'a>>),
  Number(f64),
  String(&'a str),
  Id(&'a str),
}
