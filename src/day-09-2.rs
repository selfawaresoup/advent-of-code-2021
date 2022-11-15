static INPUT:&str = include_str!("day-09-input");
use std::collections::{HashSet, HashMap};

type Coord = (usize, usize);

struct HeightMap {
	map: HashMap<Coord, Point>,
	width: usize,
	height: usize,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
	x: usize,
	y: usize,
	h: usize,
}

impl HeightMap {
	pub fn new(data: Vec<Vec<usize>>) -> Self {
		let height = data.len();
		let width = data.get(0).unwrap().len();

		let mut map:HashMap<Coord, Point> = HashMap::new();

		for y in 0..height {
			for x in 0..width {
				let h = data[y][x];
				map.insert((x,y), Point{x,y,h});
			}
		}

		Self{map, width, height}
	}

	pub fn get(&self,	x: usize, y: usize) -> Option<Point> {
		match self.map.get(&(x,y)) {
			Some(p) => {
				Some(p.to_owned())
			},
			_ => None
		}
	}

	pub fn get_low_points(&self) -> Vec<Point> {
		let mut low_points = vec![];
		for ((x, y), p) in &self.map {
			let mut low = true;
			for n in self.get_neighbors(x.clone(), y.clone()) {
				low &= n.h > p.h;
			}
			if low {
				low_points.push(Point{x: p.x, y: p.y, h: p.h});
			}
			
		}
		low_points
	}

	pub fn get_neighbors(&self,	x: usize, y: usize) -> HashSet<Point> {
		let mut points: HashSet<Point> = HashSet::new();
		if x > 0 {
			points.insert(self.get(x - 1, y).unwrap());
		}

		if 	y > 0 {
			points.insert(self.get(x, y - 1).unwrap());
		}

		if 	x < self.width - 1 {
			points.insert(self.get(x + 1, y).unwrap());
		}

		if 	y < self.height - 1 {
			points.insert(self.get(x, y + 1).unwrap());
		}

		// println!("{x},{y}, N: {}", points.len());

		points
	}

	fn fill_basin(&self, mut basin: HashSet<Point>, p: &Point) -> HashSet<Point> {
		if p.h >= 9 { return basin; }
		if basin.contains(p) { return basin; }

		basin.insert(p.clone());
		for n in self.get_neighbors(p.x.clone(), p.y.clone()) {
			basin = self.fill_basin(basin, &n);
		}
		basin
	}

	pub fn get_basin(&self, low_point: &Point) -> HashSet<Point> {
		let mut basin: HashSet<Point> = HashSet::new();
		basin = self.fill_basin(basin, low_point);
		basin
	}

	fn render_basin(&self, basin: &HashSet<Point>) {
		for y in 0..self.height {
			for x in 0..self.width {
				let p = self.get(x, y).unwrap();
				if basin.contains(&p) {
					print!("#");
				} else {
					print!("+");
				}
			}
			print!("\n");
		}
		print!("\n");
	}

	fn render_map(&self) {
		for y in 0..self.height {
			for x in 0..self.width {
				let p = self.get(x, y).unwrap();
				if p.h == 9 {
					print!("#");
				} else {
					print!(" ");
				}
			}
			print!("\n");
		}
		print!("\n");
	}

	pub fn get_basins_prod(&self) -> i64 {
		let low_points = self.get_low_points();
		let mut prod: i64 = 1;

		let mut basins = vec![];

		for lp in low_points {
			basins.push(self.get_basin(&lp));
		}

		basins.sort_by(|b,a| a.len().cmp(&b.len()));
		basins.truncate(3);

		for b in basins {
			println!("{}", b.len());
			prod *= b.len() as i64;
		}

		prod
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
	let	prod = height_map.get_basins_prod();
	println!("{prod}");

	// let b = height_map.get_basin(&height_map.get(0,0).unwrap());
	// height_map.render_basin(&b);
}
