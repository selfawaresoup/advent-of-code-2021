use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT:&str = include_str!("day-12-input");

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Cave {
	name: String,
	small: bool
}

impl Cave {
	pub fn new(name: String) -> Self {
		let small = name.chars().next().unwrap().is_lowercase();

		Self{name, small}
	}
}

impl ToString for Cave {
	fn to_string(&self) -> String {
			self.name.clone()
	}
}

type Path = Vec<Cave>;

#[derive(Clone)]
struct CaveMap {
	map: HashMap<Cave, HashSet<Cave>>,
}

impl CaveMap {
	pub fn new() -> Self {
		CaveMap { map: HashMap::new() }
	}
	
	pub fn add(&mut self, a: Cave, b: Cave) {
		self.add_connection(&a, &b);
		self.add_connection(&b, &a);
	}

	fn add_connection(&mut self, a: &Cave, b: &Cave) {
		//println!("adding {} -> {}", a.name, b.name);
		if let Some(entry) = self.map.get_mut(&a) {
			entry.insert(b.to_owned());
		} else {
			let mut entry = HashSet::new();
			entry.insert(b.to_owned());
			self.map.insert(a.to_owned(), entry);
		}
	}

	pub fn get_next(&self, c: &Cave) -> Vec<Cave> {
		let mut next_caves = vec![];

		if let Some(caves) = self.map.get(&c) {
			for c in caves {
				next_caves.push(c.to_owned());
			}
		}

		next_caves
	}

	pub fn is_visited(path: &Path, c: &Cave) -> bool {
		for pc in path {
			if pc.name == c.name { return true; }
		}
		false
	}

	fn find_path_step(&self, mut paths: Vec<Path>) -> Vec<Path> {
		let mut current_path = paths.pop().unwrap();
		let current_cave = current_path.pop().unwrap();
		let current_name = current_cave.name.clone();

		if current_cave.small && Self::is_visited(&current_path, &current_cave) { return paths; }
		
		let next_caves = self.get_next(&current_cave);
		current_path.push(current_cave);

		if current_name.eq("end") {
			for c in &current_path {
				print!("{} ", c.to_string());
			}
			println!("");
			paths.push(current_path);
			return paths;
		}
		
		for nc in next_caves {
			let mut next_path = current_path.clone();
			next_path.push(nc);
			paths.push(next_path);
			paths = self.find_path_step(paths);
		}

		paths
	}

	pub fn find_paths(&self) -> Vec<Path> {
		let start_cave = Cave::new("start".to_string());
		let mut paths = vec![vec![start_cave]];
		paths = self.find_path_step(paths);
		paths
	}
}


fn get_input() -> CaveMap {
	let input = String::from(INPUT);
	let entries = input.split("\n");

	let mut cave_map = CaveMap::new();

	let re = Regex::new(r"^(\w+)-(\w+)").unwrap();

	for entry in entries {
		let captures = re.captures(entry).unwrap();
		let cave_name_a = captures.get(1).unwrap().as_str().to_string();
		let cave_name_b = captures.get(2).unwrap().as_str().to_string();

		let cave_a = Cave::new(cave_name_a);
		let cave_b = Cave::new(cave_name_b);

		cave_map.add(cave_a, cave_b);
	}

	cave_map
}


fn main() {
	let caves = get_input();

	let paths = caves.find_paths();
	println!("{}", paths.len());
}
