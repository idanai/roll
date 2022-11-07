mod dice {
	use std::num::ParseIntError;
	// Errors that may occure when parsing Dice
	pub enum Error{
		Parse,
		MoreThanOneD(Dice), // still parses the first Dice
		NoD,
	}

	impl From<ParseIntError> for Error {
		fn from(_: ParseIntError) -> Self{
			Self::Parse
		}
	}

	// Contains two numbers, the first: a number of dice, the second: number of faces on each die.
	// can be used to roll dice with an external random number generator
	#[derive(Clone, Copy)]
	pub struct Dice {
		pub num: u32,
		pub die: u32,
	}

	impl Dice {
		// returns a lazy iterator over @self.num randomly generated numbers in a range [1, @self.die]
		pub fn roll<'a>(&self, rng: &'a mut rand::rngs::ThreadRng) -> impl Iterator<Item = u32> + 'a {
			use rand::Rng;
			let n = if self.always_zero() {0} else {self.num};

			let dice = *self;
			(0..n).map(move |_| rng.gen_range(1..=dice.die))
		}

		// test if the result of rolling Dice always returns 0
		pub fn always_zero(&self) -> bool {
			self.num == 0 || self.die == 0
		}
	}

	// Can parse the following format: <Optional uint><'d' or 'D'><uint>
	// Examples: d4, 2d6, 1d8, 0d10, 12d0
	impl std::str::FromStr for Dice {
		type Err = Error;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let mut numbers = s.split(['d', 'D']);

			let mut next = || -> Result<&str, Error> {
				numbers.next().ok_or(Error::NoD)
			};

	//		let num = {
	//			let s = next()?;
	//			Result::<u32, Error>::Ok( if s.is_empty() {1} else {s.parse()?} )
	//		}?;
			let s = next()?;

			let dice = Dice {
				num: if s.is_empty() {1} else {s.parse()?},
				die: next()?.parse()?
			};

			// expecting a single 'd'
			if numbers.next().is_some() {
				return Err(Error::MoreThanOneD(dice));
			}

			Ok(dice)
		}
	}
}

fn main(){
	use dice::*;
	use termion::{style, color};

	let mut args: Vec<String> = std::env::args().collect();

	if args.len() == 1 {
		println!("Try {} help", args[0]);
		return;
	}

	args[1].make_ascii_lowercase();
	if args[1].contains("help") {
		println!(
"Generates random-number-sequences in a dice rolling format.
Test by entering: {} d4 2d6 0d10 20d0", args[0]);
		return;
	}

	let mut rng = rand::thread_rng(); // random number generator

	println!("{}  Dice\t╬  Sum\t╬  Avg\t╬  Rolls{}", style::Bold, style::Reset); // table header

	for arg in &args[1..] {
		match arg.parse::<Dice>() {
			Ok(dice) => {
				let rolls: Vec<u32> = dice.roll(&mut rng).collect();
				let sum: u32 = rolls.iter().sum();

				print!("{arg}\t│{bold}{dye}{sum}{reset}\t│",
					arg = arg,
					bold = style::Bold,
					dye = color::Fg(color::Cyan),
					reset = style::Reset,
					sum = sum,
				);

				if rolls.len() > 1 {
					let avg = if sum == 0 {0f32} else {sum as f32 / rolls.len() as f32};
					print!("{:.2}\t│{:?}", avg, rolls);
				}else{
					print!("\t│");
				}
				println!();
			}
			Err(_) => println!("{fg}{bold}{arg}{reset}\t│{bg}FAIL{reset}\t│\t│",
				arg=arg,
				bold=style::Bold,
				bg=color::Bg(color::Red),
				fg=color::Fg(color::Red),
				reset=style::Reset)
		}
	}
}
