static INPUT:&str = include_str!("day-11-input");
use std::collections::{HashSet, HashMap};

type Coord = (usize, usize);

struct OctopusMap {
	map: HashMap<Coord, Octopus>,
	width: usize,
	height: usize,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Clone)]
struct Octopus {
	x: usize,
	y: usize,
	energy: usize,
	flashed: bool,
}

impl OctopusMap {
	pub fn new(data: Vec<Vec<usize>>) -> Self {
		let mut map:HashMap<Coord, Octopus> = HashMap::new();

		for y in 0..10 {
			for x in 0..10 {
				let energy = data[y][x];
				map.insert((x,y), Octopus{x,y,energy, flashed: false});
			}
		}

		Self{map, width: 10, height: 10}
	}

	pub fn get(&self,	x: usize, y: usize) -> Option<Octopus> {
		match self.map.get(&(x,y)) {
			Some(p) => {
				Some(p.to_owned())
			},
			_ => None
		}
	}

	pub fn set(&mut self, x: usize, y: usize, octopus: Octopus) {
		self.map.insert((x, y), octopus);
	}

	pub fn get_neighbors(&self,	x: usize, y: usize) -> HashSet<Octopus> {
		let mut octopuses: HashSet<Octopus> = HashSet::new();

		let offsets: Vec<(i64, i64)> = vec![
			(-1, -1), ( 0, -1), (1, -1),
			(-1,  0),           (1,  0),
			(-1,  1), ( 0,  1), (1,  1),
		];

		for off in offsets {
			let x2 = (x as i64) + off.0; 
			let y2 = (y as i64) + off.1;
			if x2 < 0 || x2 >= self.width as i64 || y2 < 0 || y2 >= self.height as i64 { continue; }

			if let Some(p) = self.get(x2 as usize, y2 as usize) {
				octopuses.insert(p);
			}
		}

		//println!("{},{}, {}", x, y, octopuses.len());

		octopuses
	}

	#[allow(dead_code)]
	pub fn render_map(&self) {
		for y in 0..self.height {
			for x in 0..self.width {
				let o = self.get(x, y).unwrap();
				let f = if o.flashed {"*"} else {" "};
				print!("{:02}{} ", o.energy, f);
			}
			print!("\n");
		}
		print!("\n");
	}

	fn get_charged(&self) -> Option<Vec<Octopus>> {
		let mut octopuses = vec![];

		for (_, o) in &self.map {
			if o.energy > 9 && !o.flashed { octopuses.push(o.clone()); }
		}

		if octopuses.len() > 0 {
			// println!("charged {}", octopuses.len());
			Some(octopuses)
		} else {
			// println!("none charged");
			None
		}
	}

	pub fn step(&mut self) -> usize {
		let mut flashes = 0;

		for (_, octopus) in self.map.iter_mut() {
			if octopus.flashed {
				octopus.flashed = false;
			}
		}
		
		for (_, octopus) in self.map.iter_mut() {
			octopus.energy += 1;
		}
		
		while let Some(charged) = self.get_charged() {
			for mut octopus in charged {
				octopus.flashed = true;
				self.set(octopus.x, octopus.y, octopus.clone());
				flashes += 1;

				for mut neighbor in self.get_neighbors(octopus.x, octopus.y) {
					neighbor.energy += 1;
					self.set(neighbor.x, neighbor.y, neighbor);
				}
			}
		}

		for (_, octopus) in self.map.iter_mut() {
			if octopus.flashed {
				octopus.energy = 0;
			}
		}
		
		//self.render_map();

		flashes
	}
}

fn get_input() -> OctopusMap {
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

	OctopusMap::new(map)
}


fn main() {
	let mut octopus_map = get_input();
	let mut i = 0;
	
	loop {
		i += 1;
		let flashes = octopus_map.step();
		println!("Step: {i}, Flashes: {flashes}");
		if flashes == 100 { break; } 
	}


}
