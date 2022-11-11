use std::collections::VecDeque;

static INPUT:&str = include_str!("day-04-input");

fn parse_pulled_numbers(s: String) -> Vec<u64> {
	let items = s.split(",");
	let mut numbers: Vec<u64> = vec![];

	for item in items {
		numbers.push(item.parse::<u64>().unwrap());
	}
	numbers
}

struct Cell {
	pub number: u64,
	pub marked: bool,
}

impl Cell {
	pub fn mark(&mut self, n: u64) {
		if self.number == n {
			self.marked = true;
		}
	}
}

struct Board {
	cells: Vec<Vec<Cell>>,
	pub done: bool,
}

impl Board {
	pub fn parse(board_str: String) -> Self {
    let mut cells = vec![];
	 	
		for line in board_str.split("\n") {
			let mut row = vec![];
	
			for number in line.split_whitespace() {
				let cell = Cell{
					number: number.parse::<u64>().unwrap(),
					marked: false,
				};
				row.push(cell);
			}

			cells.push(row);
		}

		Self{cells, done: false}
	}

	pub fn mark(&mut self, n: u64) {
		if self.done {return;}
		
		for row in 0..5 {
			for col in 0..5 {
				self.cells[col][row].mark(n)
			}
		}
	}

	pub fn check_for_bingo(&mut self) -> bool {
		if self.done { return false; }

		for row in 0..5 {
			let mut bingo = true;
			for col in 0..5 {
				bingo = bingo && self.cells[row][col].marked;
			}
			if bingo {
				self.done = true;
				return true;
			}
		}

		for col in 0..5 {
			let mut bingo = true;
			for row in 0..5 {
				bingo = bingo && self.cells[row][col].marked;
			}
			if bingo {
				self.done = true;
				return true;
			}
		}

		false
	}

	pub fn sum_unmarked(&self) -> u64 {
		let mut sum = 0;
		for row in &self.cells {
			for cell in row {
				if !cell.marked { sum += cell.number; }
			}
		}
		sum
	}
}

fn get_input() -> (Vec<u64>, Vec<Board>) {
	let input = String::from(INPUT);
	let mut sections = VecDeque::from_iter(input.split("\n\n"));

	let pulled_numbers = parse_pulled_numbers(String::from(sections.pop_front().unwrap()));

	let mut boards: Vec<Board> = vec![];

	while let Some(b) = sections.pop_front() {
		boards.push(Board::parse(String::from(b)));
	}

	(pulled_numbers, boards)
}

fn main() {
	let (numbers, mut boards) = get_input();

	let mut bingo = false;
	let mut winner_sum = 0;
	let mut winner_number = 0;

	for n in numbers {
		for i in 0..boards.len() {
			boards[i].mark(n);
			if boards[i].check_for_bingo() {
				winner_sum = boards[i].sum_unmarked();
				winner_number = n;
				bingo = true;
			}
		}
		if bingo {
			
			//break;
		}
	}

	println!("Winner number: {winner_number}");
	println!("Winner sum:    {winner_sum}");
	println!("Score:         {}", winner_number * winner_sum);
}
