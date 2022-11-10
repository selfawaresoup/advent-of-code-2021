use std::collections::VecDeque;

static INPUT:&str = include_str!("day-01-input");

fn get_input() -> Vec<u64> {
	let s = String::from(INPUT);
	let mut input = vec![];
	let lines = s.split("\n");
	
	for line in lines {
		let depth = line.parse::<u64>().unwrap();
		input.push(depth);
	}
	
	input
}

fn main() {
	let mut window:VecDeque<u64> = VecDeque::new();
	let mut inc_count = 0;
	let mut last_depth: Option<u64> = None;

	for d in get_input() {
		window.push_back(d);
		if window.len() == 3 {
			let sum = window.iter().sum();
			if let Some(ld) = last_depth {
				if sum > ld {
					inc_count += 1;
				}
			}
			last_depth = Some(sum);
			window.pop_front();
		}
	}

	println!("{inc_count}");
}
