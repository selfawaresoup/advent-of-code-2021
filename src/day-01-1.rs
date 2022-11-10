static INPUT:&str = include_str!("day-01-input");

fn main() {
	// println!("{INPUT}");
	let input = String::from(INPUT);
	let lines = input.split("\n");

	let mut inc_count = 0;
	let mut last_depth:Option<u64> = None;
	for line in lines {
		let depth = line.parse::<u64>().unwrap();
		if let Some(ld) = last_depth {
			if depth > ld {
				inc_count += 1;
			}
		}
		last_depth = Some(depth)
	}

	println!("{inc_count}");
}
