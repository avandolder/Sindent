/// Lexer's for lisps are generally pretty simple.
/// This one is somewhat more complicated, as it needs to
/// track indentation levels on lines.
pub(crate) struct Lexer<'src> {
  src: &'src str,
  line: usize,
  offset: usize,
  state: LexerState,
}

enum LexerState {
  Indent,
  Token,
}

#[derive(Debug)]
pub struct Token {
  line: usize,
  start_offset: usize,
  end_offset: usize,
  kind: TokenKind,
}

#[derive(Debug)]
pub(crate) enum TokenKind {
  Id,
  Number,
  String,
  LParen,
  RParen,
  Quote,
  Indent(usize),
  Eof,
}

impl<'src> Lexer<'src> {
  fn new(src: &'src str) -> Self {
    Lexer {
      src,
      line: 0,
      offset: 0,
      state: LexerState::Indent,
    }
  }

  fn next_char(&self) -> Option<char> {
    self.src.chars().next()
  }

  /// Scan whitespace characters, skipping empty
  /// lines, and return the indentation level of
  /// next empty line.
  fn scan_indent(&mut self) -> Token {
    let indent = self.src.chars().take_while(|&c| c == ' ' || c == '\t').count();

    // Check the character following the indent is a newline,
    // and if it is, continue scanning spaces.
    match self.src.chars().nth(indent) {
      Some(c) if c.is_ascii_whitespace() => {
        self.src = &self.src[indent + 1..];
        self.offset += indent + 1;
        self.line += 1;
        self.scan_indent()
      }
      _ => {
        self.src = &self.src[indent..];
        let start_offset = self.offset;
        self.offset += indent;
        self.state = LexerState::Token;
        Token {
          line: self.line,
          start_offset,
          end_offset: self.offset,
          kind: TokenKind::Indent(indent),
        }
      }
    }
  }

  /// Scan a number.
  fn scan_number(&self) -> (usize, TokenKind) {
    (1, TokenKind::Number)
  }

  /// Scan a string.
  fn scan_string(&self) -> (usize, TokenKind) {
    (1, TokenKind::String)
  }

  /// Scan an identifier.
  fn scan_id(&self) -> (usize, TokenKind) {
    (1, TokenKind::Id)
  }

  fn next_token(&mut self) -> Result<Token, String> {
    if let LexerState::Indent = self.state {
      return Ok(self.scan_indent());
    }

    loop {
      match self.next_char() {
        Some('\n') => {
          self.line += 1;
          self.src = &self.src[1..];
          self.offset += 1;
          self.state = LexerState::Indent;
          return self.next_token();
        }
        Some(c) if !c.is_ascii_whitespace() => break,
        None => break,
        _ => {}
      }
      self.src = &self.src[1..];
      self.offset += 1;
    }

    if self.src.is_empty() {
      return Ok(Token {
        line: self.line,
        start_offset: self.offset,
        end_offset: self.offset,
        kind: TokenKind::Eof,
      });
    }

    let (len, kind) = match self.next_char().unwrap() {
      '(' => (1, TokenKind::LParen),
      ')' => (1, TokenKind::RParen),
      '\'' => (1, TokenKind::Quote),
      '"' => self.scan_string(),
      c if c.is_ascii_digit() => self.scan_number(),
      c if c.is_alphabetic() => self.scan_id(),
      // TODO: add symbolic identifier support
      c => return Err(format!("unexpected character {} on line {}", c, self.line)),
    };

    let token = Token {
      line: self.line,
      start_offset: self.offset,
      end_offset: self.offset + len,
      kind,
    };

    self.src = &self.src[len..];
    self.offset += len;

    Ok(token)
  }
}

pub fn scan(src: &str) -> Result<Vec<Token>, String> {
  let mut lexer = Lexer::new(src);
  let mut tokens = vec![];
  
  loop {
    match lexer.next_token()? {
      Token { kind: TokenKind::Eof, .. } => return Ok(tokens),
      token => tokens.push(token),
    }
  }
}
