use typed_arena::Arena;

use crate::ast::{Ast, AstKind};

/// `Parser` keeps track of the current state of parsing, as well
/// as owning the AST nodes the parser creates.
pub(crate) struct Parser<'a> {
  arena: Arena<Ast<'a>>,

  /// `indent` is a stack of indentation levels the parser uses to
  /// track the various blocks. Each level corresponds to the number
  /// of spaces (or tabs) it is indented (don't mix spaces and tabs!).
  /// An empty stack indicates a level of 0.
  indent: Vec<usize>,
}
