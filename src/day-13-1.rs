use std::collections::HashSet;
use regex::Regex;
use std::cmp::max;

static INPUT:&str = include_str!("day-13-input");

fn parse_instruction(entry: &str) -> Instruction {
	let entry_re = Regex::new(r"^fold along (\w)=(\d+)$").unwrap();
	let captures = entry_re.captures(&entry).unwrap();

	let dir = match captures.get(1).unwrap().as_str() {
		"y" => Direction::Horizontal,
		_ => Direction::Vertical,
	};
	let at = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

	Instruction{direction: dir, at}
}

fn parse_point(entry: &str) -> Point {
	let entry_re = Regex::new(r"^(\d+),(\d+)$").unwrap();
	let captures = entry_re.captures(&entry).unwrap();

	let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
	let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

	Point(x,y)
}

fn get_input() -> Grid {
	let input = String::from(INPUT);
	let (entries_block, instructions_block) = input.split_once("\n\n").unwrap();

	let mut grid = Grid::new();
	for point_entry in entries_block.split("\n") {
		grid.add_point(parse_point(point_entry ));
	}

	for instruction_entry in instructions_block.split("\n") {
		grid.add_instruction(parse_instruction(instruction_entry));
	}

	grid
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point(i64, i64);

#[derive(Debug)]
enum Direction {
	Horizontal,
	Vertical,
}

#[derive(Debug)]
struct Instruction {
	at: i64,
	direction: Direction
}

#[derive(Debug)]
struct Grid {
	map: HashSet<Point>,
	max_x: i64,
	max_y: i64,
	instructions: Vec<Instruction>
}

impl Grid {
	pub fn new() -> Self {
		Grid{map: HashSet::new(), instructions: vec![], max_x: 0, max_y: 0}
	}

	pub fn add_point(&mut self, point: Point) {
		self.max_x = max(self.max_x, point.0);
		self.max_y = max(self.max_y, point.1);
		self.map.insert(point);
	}

	pub fn add_instruction(&mut self, instruction: Instruction) {
		self.instructions.push(instruction);
	}

	fn re_measure(&self) -> (i64, i64) {
		let mut max_x = 0;
		let mut max_y = 0;
		for point in &self.map {
			max_x = max(max_x, point.0);
			max_y = max(max_y, point.1);
		}

		(max_x, max_y)
	}

	#[allow(dead_code)]
	pub fn render(&self) {
		for y in 0..=self.max_y {
			for x in 0..=self.max_x {
				if self.map.contains(&Point(x, y)) {
					print!("# ");
				} else {
					print!(". ");
				}
			}
			println!("");
		}
		println!("({} dots total)", self.map.len());
	}

	pub fn fold_once(&mut self) {
		let i = &self.instructions[0];
		self.map = Self::fold_map(&self.map, i);
		println!("({} dots total)\n", self.map.len());
		(self.max_x, self.max_y) = self.re_measure();
	}

	fn fold_map(map: &HashSet<Point>, instruction: &Instruction) -> HashSet<Point> {
		let mut new_map: HashSet<Point> = HashSet::new();
		let at = instruction.at;

		for point in map {
			let new_point = match instruction.direction {
				Direction::Horizontal => {
					let mut y = point.1;
					if y > at {
						y = 2 * at - y;
					}
					Point(point.0, y)
				}
				Direction::Vertical => {
					let mut x = point.0;
					if x > at {
						x = 2 * at - x;
					}
					Point(x, point.1)
					
				}
			};
			new_map.insert(new_point);
		}

		new_map
	}
}

fn main() {
	let mut grid = get_input();
	grid.fold_once();
}
