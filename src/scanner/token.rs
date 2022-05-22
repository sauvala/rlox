use std::fmt;

use super::token_type::TokenType;

pub struct Token {
		token_type: TokenType,
		lexeme: String,
		line: i32,
}

impl fmt::Display for Token {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
		}
}

// impl Token {
// 		pub fn to_string(&self) -> String {
// 				return format!("{}", self.token_type);
// 		}
// }
