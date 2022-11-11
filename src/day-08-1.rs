use std::collections::{HashSet};

static INPUT:&str = include_str!("day-08-input");

type Entry = (Vec<Signal>, Vec<Signal>);

fn parse_signal(mut s: String) -> Signal {
	let mut signal = Signal::new();
	while let Some(c) = s.pop() {
		let seg = match c {
			'a' => Some(Segment::A),
			'b' => Some(Segment::B),
			'c' => Some(Segment::C),
			'd' => Some(Segment::D),
			'e' => Some(Segment::E),
			'f' => Some(Segment::F),
			'g' => Some(Segment::G),
			_ => None,
		};
		if let Some(segment) = seg {
			signal.insert(segment);
		}
	}
	signal
}

fn get_input() -> Vec<Entry> {
	let input = String::from(INPUT);
	let lines = input.split("\n");
	let mut entries = vec![];
	for line in lines {
		let (signals_str, digits_str) = line.split_once(" | ").unwrap();

		let mut signals = vec![];
		for s in signals_str.split(" ") {
			signals.push(parse_signal(s.to_string()));
		}

		let mut digits = vec![];
		for s in digits_str.split(" ") {
			digits.push(parse_signal(s.to_string()));
		}

		entries.push((signals, digits));
	}

	entries
}

#[derive(PartialEq, Eq, Hash)]
enum Segment {
	A, B, C, D, E, F, G
}

type Signal = HashSet<Segment>;

fn main() {
	let data = get_input();

	let mut ones = 0;
	let mut fours = 0;
	let mut sevens = 0;
	let mut eights = 0;

	for entry in data {
		let (_, digits) = entry;

		for digit in digits {
			if digit.len() == 2 { ones += 1;}
			if digit.len() == 3 { fours += 1;}
			if digit.len() == 4 { sevens += 1;}
			if digit.len() == 7 { eights += 1;}
		}
	}


	println!("1: {}, 4: {}, 7: {}, 8: {}", ones, fours, sevens, eights);
	println!("{}", ones + fours + sevens + eights);
}
