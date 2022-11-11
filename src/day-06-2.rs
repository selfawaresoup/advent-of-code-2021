use std::collections::HashMap;

static INPUT:&str = include_str!("day-06-input");

fn get_input() -> Vec<u64> {
	let input = String::from(INPUT);
	let entries = input.split(",");

	let mut fish = vec![];

	for e in entries {
		let n = e.parse::<u64>().unwrap();
		fish.push(n);
	}

	fish
}

fn get_new_counters() -> HashMap<u64, u64> {
	let mut counters: HashMap<u64, u64> = HashMap::new();
	for i in 0..=8 {
		counters.insert(i, 0);
	}

	counters
}

fn day(counters: HashMap<u64, u64>) -> HashMap<u64, u64> {
	let zeroes = *counters.get(&0).unwrap();
	let mut new_counters = get_new_counters();

	for i in 0..=7 {
		let mut count = *counters.get(&(i+1)).unwrap();
		 if i == 6 {
			count += zeroes;
		 }

		 new_counters.insert(i, count);
	}

	new_counters.insert(8, zeroes);

	new_counters
}

fn sum(counters: &HashMap<u64, u64>) -> u64 {
	let mut sum = 0;
	for (_, count) in counters {
		sum += count;
	}
	sum
}

fn main() {
	let mut fish = get_input();

	let mut counters = get_new_counters();

	for f in fish {
		let c = counters.get_mut(&f).unwrap();
		*c += 1;
	}

	for _ in 1..=256 {
		counters = day(counters);
	}

	println!("{}", sum(&counters))
}
