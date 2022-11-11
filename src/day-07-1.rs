static INPUT:&str = include_str!("day-07-input");

fn get_input() -> Vec<i64> {
	let input = String::from(INPUT);
	let entries = input.split(",");

	let mut numbers = vec![];

	for entry in entries {
		numbers.push(entry.parse::<i64>().unwrap());
	}

	numbers
}

fn main() {
	let raw_distances = get_input();

	let mut distances: Vec<(i64, i64)> = vec![];

	for d1 in &raw_distances {
		let mut cost = 0;
		for d2 in &raw_distances {
			cost += (d2 - d1).abs();
		}

		distances.push((*d1, cost));
	}

	distances.sort_by(|b,a| a.1.cmp(&b.1));

	let cheapest = distances.pop().unwrap();

	println!("distance: {}, cost: {}", cheapest.0, cheapest.1);
}
