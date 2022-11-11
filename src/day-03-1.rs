static INPUT:&str = include_str!("day-03-input");

fn get_input() -> Vec<i16> {
	let input = String::from(INPUT);
	let lines = input.split("\n");

	let mut data: Vec<i16> = vec![];

	for line in lines {
		let mut entry = 0b000000000000;

		for ch in line.chars() {
			entry <<= 1;
			if ch == '1' { entry += 1; }
		}
		
		data.push(entry);
	}

	data
}

fn main() {
	let data = get_input();
	let mut gamma = 0b0;
	let mut epsilon = 0b0;

	for i in 0..=11 {
		// println!("i: {i}");
		let mut ones = 0;
		let mut total = 0;
		for entry in &data {
			let tmp = entry >> i;
			let bit = tmp & 0b1;
			ones += bit;
			total += 1;
		}

		let mut gamma_tmp = 0b0;
		let mut epsilon_tmp = 0b0;

		if ones > total/2 {
			gamma_tmp += 1;
		} else {
			epsilon_tmp += 1;
		}
		gamma_tmp <<= i;
		epsilon_tmp <<= i;

		gamma |= gamma_tmp;
		epsilon |= epsilon_tmp;
	}

	println!("gamma:   {:012b}", gamma);
	println!("epsilon: {:012b}", epsilon);
	println!("power:   {}", gamma * epsilon);
}
