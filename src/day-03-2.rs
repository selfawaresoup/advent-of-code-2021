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

fn get_most_common_bit(data: &Vec<i16>, position: i8) -> i16 {
	let mut ones = 0;
	let total = data.len() as i16;

	for entry in data {
		let bit = (entry >> position) & 0b1;
		ones += bit;
	}

	if ones == 0 { return 0; } 
	if ones == total { return 1; } 
	
	if ones * 2 >= total {
		1
	} else {
		0
	}
}

fn get_least_common_bit(data: &Vec<i16>, position: i8) -> i16 {
	let mut ones = 0;
	let total = data.len() as i16;
	
	for entry in data {
		let bit = (entry >> position) & 0b1;
		ones += bit;
	}
	
	if ones == 0 { return 0; }
	if ones == total { return 1; } 

	if ones * 2 >= total {
		0
	} else {
		1
	}
}

fn get_o2_rating(data: Vec<i16>, position: i8) -> i16 {
	if data.len() == 1 { return data[0]; }

	let most_common_bit = get_most_common_bit(&data, position);

	let mut next_data = vec![];
	for entry in data {
		let bit = (entry >> position) & 0b1;
		if bit == most_common_bit {
			next_data.push(entry);
		}
	}

	get_o2_rating(next_data, position - 1)
}

fn get_co2_rating(data: Vec<i16>, position: i8) -> i16 {
	if data.len() == 1 { return data[0]; }

	let least_common_bit = get_least_common_bit(&data, position);

	let mut next_data = vec![];
	for entry in data {
		let bit = (entry >> position) & 0b1;
		if bit == least_common_bit {
			next_data.push(entry);
		}
	}
	get_co2_rating(next_data, position - 1)
}


fn main() {
	let data = get_input();

	let o2 = get_o2_rating(data.clone(), 11) as i64;

	let co2 = get_co2_rating(data.clone(), 11) as i64;

	println!("o2:  {o2}");
	println!("co2: {co2}");
	println!("power:   {}", o2 * co2);
}
