use std::vec;

static INPUT:&str = include_str!("day-10-input");

type Line = Vec<Token>;
type Program = Vec<Line>;


fn get_input() -> Program {
	let input = String::from(INPUT);
	let lines = input.split("\n");

	let mut program: Program = vec![];

	for l in lines {
		let mut line: Line = vec![];

		for c in l.chars() {
			if let Some(t) = Token::new(c) {
				line.push(t);
			}
		}
		program.push(line);
	}

	program
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Delimiter {
	Parenthesis,
	Bracket,
	AngleBracket,
	Brace,
}

#[derive(Clone, Debug)]
struct Token {
 pub delimiter: Delimiter,
 pub closing: bool,
}

impl Token {
	pub fn new(c: char) -> Option<Self> {
		match c {
			'(' => {
				Some(Token{delimiter: Delimiter::Parenthesis, closing: false})
			},
			')' => {
				Some(Token{delimiter: Delimiter::Parenthesis, closing: true})
			},
			'[' => {
				Some(Token{delimiter: Delimiter::Bracket, closing: false})
			},
			']' => {
				Some(Token{delimiter: Delimiter::Bracket, closing: true})
			},
			'<' => {
				Some(Token{delimiter: Delimiter::AngleBracket, closing: false})
			},
			'>' => {
				Some(Token{delimiter: Delimiter::AngleBracket, closing: true})
			},
			'{' => {
				Some(Token{delimiter: Delimiter::Brace, closing: false})
			},
			'}' => {
				Some(Token{delimiter: Delimiter::Brace, closing: true})
			},
			_ => None,
		}
	}

	pub fn matches(&self, token: &Self) -> bool {
		self.delimiter == token.delimiter && self.closing != token.closing
	}
}

impl ToString for Token {
	fn to_string(&self) -> String {
			if self.closing {
				match self.delimiter {
					Delimiter::Parenthesis => ")",
					Delimiter::Bracket => "]",
					Delimiter::AngleBracket => ">",
					Delimiter::Brace => "}",
				}.to_string()
			} else {
				match self.delimiter {
					Delimiter::Parenthesis => "(",
					Delimiter::Bracket => "[",
					Delimiter::AngleBracket => "<",
					Delimiter::Brace => "{",
				}.to_string()
				
			}
	}
}

fn check_line(line: &Line) -> Option<Token> {
	let mut stack: Vec<Token> = vec![];


	for token in line {
		if !token.closing {
			stack.push(token.to_owned());
			continue;
		}

		if stack.len() == 0 { return None; }

		let last = stack.pop().unwrap();

		if !last.matches(token) {
			return Some(token.to_owned());
		} 
		
	}

	None
}

fn main() {
	let program = get_input();
	let mut sum = 0;

	for line in program {
		if let Some(t) = check_line(&line) {
			sum += match t.delimiter {
				Delimiter::Parenthesis => 3,
				Delimiter::Bracket => 57,
				Delimiter::Brace => 1197,
				Delimiter::AngleBracket => 25137,
			}
		}
	}

	println!("{sum}");
}
