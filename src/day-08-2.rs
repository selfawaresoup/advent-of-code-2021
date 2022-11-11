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

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Segment {
	A, B, C, D, E, F, G
}

type Signal = HashSet<Segment>;

fn find_first_by_segment_count(signals: &Vec<Signal>, count: usize) -> Signal {
	let filtered = filter_by_segment_count(signals, count);
	let first = filtered.first().unwrap();
	first.clone()
}

fn filter_by_segment_count(signals: &Vec<Signal>, count: usize) -> Vec<Signal> {
	let mut filtered = vec![];
	for s in signals {
		if s.len() == count { filtered.push(s.clone()); }
	}

	filtered
}

fn decode(entry: Entry) -> i64 {

	let signals = entry.0;
	let one = find_first_by_segment_count(&signals, 2);
	let four = find_first_by_segment_count(&signals, 4);
	let seven = find_first_by_segment_count(&signals, 3);
	let eight = find_first_by_segment_count(&signals, 7);

  let six_segments = filter_by_segment_count(&signals, 6);
	let mut zero: Option<Signal> = None; 
	let mut six: Option<Signal> = None;
	let mut nine: Option<Signal> = None;

	for s in six_segments {
		if s.is_superset(&one) {
			if s.is_superset(&four) {
				nine = Some(s);
			} else {
				zero = Some(s);
			}
		} else {
			six = Some(s);
		}
	}

	let mut two: Option<Signal> = None;
	let mut three: Option<Signal> = None;
	let mut five: Option<Signal> = None;

	let five_segments = filter_by_segment_count(&signals, 5);

	for s in five_segments {
		if s.is_superset(&one) {
			three = Some(s);
		} else {
			let n = nine.as_ref().unwrap();
			if n.is_superset(&s) {
				five = Some(s);
			} else {
				two = Some(s);
			}
		}
	}

	let signal_map = vec![
		(0, zero.unwrap()),
		(1, one),
		(2, two.unwrap()),
		(3, three.unwrap()),
		(4, four),
		(5, five.unwrap()),
		(6, six.unwrap()),
		(7, seven),
		(8, eight),
		(9, nine.unwrap()),
	];

	//println!("{:?}", signal_map);

	let digit_signals = entry.1;
	let mut sum = 0;
	let mut i = 4;

	for d in digit_signals {
		for (value, digit) in &signal_map {
			if d.eq(digit) {
				//println!("{:?}, {:?}", value, digit);
				i -= 1;
				sum += value * (10 as i64).pow(i);
			}
		}
	}
	sum
}

fn main() {
	let data = get_input();

	let mut sum = 0;

	for entry in data {
		sum += decode(entry);
	} 
	
	println!("{sum}");
}
