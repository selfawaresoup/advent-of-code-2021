use std::{collections::HashMap, hash::Hash};

use regex::Regex;
static INPUT:&str = include_str!("day-05-input");

fn get_input() -> Vec<Line>{
	let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
	let mut lines = vec![];

	for entry in String::from(INPUT).split("\n") {
		let caps = re.captures(entry).unwrap();
		let x1 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
		let y1 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
		let x2 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
		let y2 = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

		lines.push(Line {
			start: Point { x: x1, y: y1 },
			end:   Point { x: x2, y: y2 }
		})
	}

	lines
}

fn clamp(n: i64, min: i64, max: i64) -> i64 {
	if n > max { return max; }
	if n < min { return min; }
	n
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
	pub x: i64,
	pub y: i64
}

struct Line {
	start: Point,
	end: Point,
}

impl Line {
	pub fn get_direction(&self) -> (i64, i64) {
		let dx = clamp(self.end.x - self.start.x, -1, 1);
		let dy = clamp(self.end.y - self.start.y, -1, 1);
		(dx, dy)
	}

	pub fn get_points(&self) -> Vec<Point> {
		let mut points = vec![];
		let (dx, dy) = self.get_direction();

		let mut p = self.start.clone();

		loop {
			points.push(p.clone());

			if p.x == self.end.x && p.y == self.end.y {
				break;
			}

			p.x += dx;
			p.y += dy;
		}

		points
	}
}

fn main() {
	let lines = get_input();

	let mut field: HashMap<Point, u64> = HashMap::new();

	for line in lines {

		let (dx, dy) = line.get_direction();
		if dx.abs() + dy.abs() > 1 { continue; }

		for point in line.get_points() {
			if let Some(c) = field.get_mut(&point) {
				*c += 1;
			} else {
				field.insert(point, 1);
			}
		}
	}

	// for y in 0..1000 {
	// 	for x in 0..1000 {
	// 		let p = Point{x,y};
	// 		let c = field.get(&p).unwrap_or(&0);
	// 		let s = match c {
	// 			0 => " ".to_string(),
	// 			_ => c.to_string(),
	// 		};
	// 		print!("{}", s);
	// 	}
	// 	println!("\n");
	// }

	let mut count = 0;

	for (_, c) in field {
		if c >= 2 {
			count += 1;
		}
	}

	println!("{count}");
}
