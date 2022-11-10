use regex::Regex;
static INPUT:&str = include_str!("day-02-input");

struct Command {
	direction: Direction,
	amount: u64,
}

enum Direction {
	Forward,
	Down,
	Up,
}

fn get_input() -> Vec<Command> {
	let mut commands = vec![];
	let re = Regex::new(r"^(\w+) (\d+)$").unwrap();

	let input = String::from(INPUT);
	let lines = input.split("\n");

	for line in lines {
		let caps = re.captures(line).unwrap();
		let direction = match caps.get(1).unwrap().as_str() {
      "forward" => { Direction::Forward },
      "down" => { Direction::Down },
      "up" => { Direction::Up },
			_ => {continue;}
		};

		let amount = match caps.get(2).unwrap().as_str().parse::<u64>() {
      Ok(n) => { n },
			_ => {continue;}
		};

		commands.push(Command{direction, amount});
	}

	commands
}

fn main() {
	let commands = get_input();
	let mut x = 0;
	let mut y = 0;
	let mut aim = 0;

	for cmd in commands {
		match cmd.direction {
			Direction::Forward => {
				y += cmd.amount * aim;
				x += cmd.amount;
			}
			Direction::Down => { aim += cmd.amount ;}
			Direction::Up => { aim -= cmd.amount ;}
		}
	}

	println!("{}", x * y);
}
