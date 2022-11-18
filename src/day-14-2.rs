use regex::Regex;
use std::collections::{LinkedList, HashMap};
use std::{cmp};

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
type PolymerStat = HashMap<(Element, Element), i128>;
type ElementCounter = HashMap<Element, i128>;

fn build_stat(mut polymer: Polymer) -> (PolymerStat, ElementCounter) {
	let mut stat = HashMap::new();
	let mut element_counter = ElementCounter::new();
	let mut element_a = polymer.pop_front().unwrap();
	element_counter.insert(element_a, 1);

	while let Some(element_b) = polymer.pop_front() {
		let pair = (element_a, element_b);

		match stat.get(&pair) {
			Some(c) => stat.insert(pair, c + 1),
			None => stat.insert(pair, 1)
		};

		match element_counter.get(&element_b) {
			Some(c) => element_counter.insert(element_b, c + 1),
			None => element_counter.insert(element_b, 1),
		};
		
		element_a = element_b;
	}


	(stat, element_counter)
}

fn step(mut stat: PolymerStat, insertions: &Insertions, mut element_counter: ElementCounter ) -> (PolymerStat, ElementCounter) {
	let mut applicable_insertions = vec![];
	for (pair, new_element) in insertions {
		if let Some(_) = stat.get(pair) {
			applicable_insertions.push((pair, new_element));
		}
	}

	let mut new_stat = PolymerStat::new();

	for (pair, new_element) in applicable_insertions {
		let (element_a, element_b) = *pair;

		// println!("checking pair {:?}", pair);
		match stat.remove(pair) {
			Some(pc) => {
				// println!("found pair {:?} {} times", pair, pc);
				let new_pair_a = (element_a, *new_element);
				let new_pair_b = (*new_element, element_b);

				// println!("inserting {} {} times", new_element, pc);

				match new_stat.get(&new_pair_a).cloned() {
					Some(npc) => new_stat.insert(new_pair_a, pc + npc),
					None => new_stat.insert(new_pair_a, pc),
				};
				
				match new_stat.get(&new_pair_b).cloned() {
					Some(npc) => new_stat.insert(new_pair_b, pc + npc),
					None => new_stat.insert(new_pair_b, pc),
				};

				match element_counter.get(new_element).cloned() {
					Some(ec) => element_counter.insert(*new_element, ec + pc),
					None => element_counter.insert(*new_element, pc),
				};
				
			}
			_ => {}
		};
	}
	stat = new_stat;

	(stat, element_counter)
}


fn main() {
	let (polymer, insertions) = get_input();

	let (mut stat, mut element_counter) = build_stat(polymer);

	for _ in 1..=40 {
		// println!("{:?}", stat);
		// println!("{:?}", element_counter);
		// println!();
		(stat, element_counter) = step(stat, &insertions, element_counter);
	}
	// println!("Done:");
	// println!("{:?}", stat);
	// println!("{:?}", element_counter);
	
	let mut min = i128::MAX;
	let mut max = 0;


	for (_, c) in element_counter {
		min = cmp::min(min, c);
		max = cmp::max(max, c);
	}

	println!("max: {}, min: {}, max-min: {}", max, min, max-min);
}
