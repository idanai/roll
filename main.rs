use rand::prelude::*;

type Int = u32;

struct DiceRoll {
	num : Int,
	die : Int,
}

impl DiceRoll {
	pub fn new(num: Int, die: Int) -> Self {
		Self{num, die}
	}

	pub fn roll(&self) -> Int {
		if self.always_zero() {
			return 0;
		}
		let mut rng = rand::thread_rng();
		(0..self.num).map(|_| rng.gen_range(1..=self.die)).sum()
	}

	pub fn always_zero(&self) -> bool {
		self.num == 0 || self.die == 0
	}

	fn util_push_die(&mut self, n: Int) {
		self.num = std::mem::replace(&mut self.die, n);
	}
}

#[derive(Debug)]
enum DiceRollParseError {
	Error
}

impl From<std::num::ParseIntError> for DiceRollParseError {
	fn from(_: std::num::ParseIntError) -> Self {
			Self::Error // Self::ParseIntError
	}
}

impl std::str::FromStr for DiceRoll {
	type Err = DiceRollParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut numbers = s.split(&['d', 'D']);

		let mut dice = DiceRoll {
			num: match numbers.next() {
				Some(s) => if s.is_empty() {1} else { s.parse()? },
				_ => return Err(Self::Err::Error)
			},
			die: match numbers.next() {
				Some(s) => s.parse()?,
				_ => return Err(Self::Err::Error)
			}
		};

		while let Some(n_str) = numbers.next() {
			let roll = dice.roll();
			// if the number of dice is zero at any iteration, jump to the last iteration
			if roll == 0 {
				return Ok(DiceRoll::new(0, numbers.last().unwrap().parse()?));
			}
			dice.die = roll;
			dice.util_push_die(n_str.parse()?);
		}

		Ok(dice)
	}
}

fn main() {
	let args : Vec<String> = std::env::args().collect();
	
	if args.len() == 1 {
		println!("Try \"{} help\"", args[0]);
		return;
	}

	if args[1].contains("help") {
		println!(
"Welcom to the dice rolling program '{program_name}' (made by Idany))!
Syntax: <optional number><'d' or 'D'><number><optional sequence of repeateing <'d' or 'D'><number>>
Examples:
	d4 = rolls a 4 sided die
	1d20 = rolls a 20 sided die
	2d6 = rolls two 6 sided dice
	1d6d4 = rolls a 6 sided die to determine how many 4 sided dice to roll (can concatenate as many dice as you like this way)
Type the following example in your CLI to see all possible syntaxes: {program_name} d20 1d8 2d6 1d6d4", program_name = args[0]);
		return;
	}

	for arg in &args[1..] {
		match arg.parse::<DiceRoll>() {
			Ok(dice) => println!("{}....{}", dice.roll(), arg),
			Err(e) => println!("{:?}....{}", e, arg)
		}
	}
}
