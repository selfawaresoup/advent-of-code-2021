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

fn main() {
	let mut fish = get_input();

	for day in 1..=80 {
		let mut new_fish = vec![];
		for f in &mut fish {
			if *f == 0 {
				new_fish.push(8);
				*f = 6;
			} else {
				*f -= 1;
			}
		}

		fish.append(&mut new_fish);
		println!("Day {},  Fish: {}", day, fish.len());
	}
}
