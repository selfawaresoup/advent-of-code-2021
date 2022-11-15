static INPUT:&str = include_str!("day-09-input");

struct HeightMap {
	data: Vec<Vec<usize>>,
	width: usize,
	height: usize
}

impl HeightMap {
	pub fn new(data: Vec<Vec<usize>>) -> Self {
		let height = data.len();
		let width = data.get(0).unwrap().len();
		Self{data, width, height}
	}

	pub fn get(&self,	x: i8, y: i8) -> Option<usize> {
		let row = self.data.get(y as usize)?;
		let point = row.get(x as usize)?;
		Some(*point)
	}

	pub fn get_neighbors(&self,	x: i8, y: i8) -> Vec<Option<usize>> {
		vec![
			self.get(x - 1, y),
			self.get(x + 1, y),
			self.get(x, y - 1),
			self.get(x, y + 1),
		]
	}

	pub fn get_low_points(&self) -> Vec<usize> {
		let mut low_points = vec![];
		for y in 0..self.height {
			for x in 0..self.width {
				let mut low = true;
				let p = self.get(x as i8, y as i8).unwrap();
				for n in self.get_neighbors(x as i8, y as i8) {
					if let Some(h) = n {
						low &= h > p;
					}
				}
				if low {low_points.push(p);}
			}
		}
		low_points
	}
}

fn get_input() -> HeightMap {
	let input = String::from(INPUT);
	let lines = input.split("\n");

	let mut map = vec![];

	for line in lines {
		let mut row = vec![];
		for c in line.chars() {
			row.push(c.to_digit(10).unwrap() as usize);
		}
		map.push(row);
	}

	HeightMap::new(map)
}


fn main() {
	let height_map = get_input();
	let low_points = height_map.get_low_points();

	let mut sum: usize = 0;

	for h in low_points {
		sum += 1 + h;
	}
	println!("{sum}");
}
