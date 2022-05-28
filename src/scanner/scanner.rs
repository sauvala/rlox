use super::{token::Token, token_type::TokenType};

#[derive(Default, Debug)]
pub struct Scanner<'a> {
		pub source: &'a str,
		pub tokens:	Vec<Token>,
		start: usize,
		current: usize,
		line: usize,
}

impl<'a> Scanner<'a> {
		pub fn new<'b: 'a>(src: &'b str) -> Self {
				Self {
						source:	src,
						tokens: Vec::new(),
						start: 0,
						current: 0,
						line: 0,
				}
		}

		pub fn scan_tokens(&mut self) {
				self.start = 0;
				self.current = 0;
				while self.not_at_end() {
						self.start = self.current;
						self.scan_token();
				}

				self.add_token(TokenType::EOF);
		}

		fn scan_token(&mut self) {
				let c = self.advance();
				match c {
						'(' =>	self.add_token(TokenType::LeftParen),
						')' =>	self.add_token(TokenType::RightParen),
						_ => println!("Unregnized char"),
				}
		}

		fn not_at_end(&self) -> bool {
			self.current < self.source.len() - 1
		}

		fn advance(&mut self) -> char {
				let c = self.source.chars().nth(self.current).unwrap();
				self.current += 1;
				c
		}

		fn add_token(&mut self, token_t: TokenType) {
				let l = String::from(&self.source[self.start..self.current]);
				let token = Token{
						token_type: token_t,
						lexeme: l,
						line: 0,
				};
				self.tokens.push(token);
		}
}
