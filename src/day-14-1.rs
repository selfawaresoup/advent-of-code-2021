use regex::Regex;
use std::collections::{LinkedList, HashMap};
use std::cmp;

static INPUT:&str = include_str!("day-14-input");

fn get_input() -> (Polymer, Insertions) {
	let input = String::from(INPUT);
	let (template_line, rules_block) = input.split_once("\n\n").unwrap();

	let polymer: LinkedList<Element> = template_line.chars().collect();

	let rule_re = Regex::new(r"^(\w\w) -> (\w)").unwrap();

	let mut insertions = HashMap::new();

	for line in rules_block.split("\n") {
		let captures = rule_re.captures(line).unwrap();
		let mut chars = captures.get(1).unwrap().as_str().chars();
		let pair = (chars.next().unwrap(), chars.next().unwrap());
		let insert = captures.get(2).unwrap().as_str().chars().next().unwrap();
		insertions.insert(pair, insert);
	}

	(polymer, insertions)
}


type Element = char;
type Polymer = LinkedList<Element>;
type Insertions = HashMap<(Element, Element), Element>;


fn step(mut polymer: Polymer, insertions: &Insertions) -> Polymer {
	let mut new_polymer = LinkedList::new();
	new_polymer.push_back(polymer.pop_front().unwrap());

	while let Some(element_b) = polymer.pop_front() {
		let element_a = new_polymer.pop_back().unwrap();
		let new_element = *insertions.get(&(element_a, element_b)).unwrap();

		new_polymer.push_back(element_a);
		new_polymer.push_back(new_element);
		new_polymer.push_back(element_b);
	}

	new_polymer
}

fn count(polymer: &Polymer) -> (i64, i64) {
	let mut min = i64::max_value();
	let mut max = 0;

	let mut counter: HashMap<Element, i64> = HashMap::new();

	for element in polymer {
		match counter.get(element) {
			Some(c) => {
				counter.insert(*element, c + 1);
			},
			None => {
				counter.insert(*element, 1);
			}
		}
	}

	for (_, n) in counter {
		min = cmp::min(min, n);
		max = cmp::max(max, n);
	} 

	(min, max)
}

fn main() {
	let (mut polymer, insertions) = get_input();

	for i in 1..=10 {
		polymer = step(polymer, &insertions);
		//println!("Step: {}, {:?}", i, polymer);
	}

	let (min, max) = count(&polymer);
	println!("max: {}, min: {}, max-min: {}", max, min, max-min);
}
