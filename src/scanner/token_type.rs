use std::fmt;

#[allow(dead_code)]
pub enum TokenType {
		LeftParen,
		RightParen,
		LeftBrace,
		RightBrace,
		Comma,
		Dot,
		Minus,
		Plus,
		SemiColon,
		Slash,
		Star,

		Bang,
		BangEqual,
		Equal,
		EqualEqual,
		Greater,
		GreaterEqual,
		Less,
		LessEqual,

		Identifier,
		String,
		Number,

		And,
		Class,
		Else,
		False,
		Fun,
		For,
		If,
		Nil,
		Or,
		Print,
		Return,
		Super,
		This,
		True,
		Var,
		While,

		EOF,
}

impl fmt::Display for TokenType {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "{}", self)
		}
}
