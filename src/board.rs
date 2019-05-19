use std::fs::File;
use std::io::{ BufReader, BufRead };

pub struct Board {
	bits: Vec<bool>,
	width: usize,
}

impl Board {
	pub fn equals(&self, other: &Board) -> bool {
		(self.bits.len() == other.bits.len()) &&
			self.bits.iter()
				.zip(&other.bits)
				.all(|(a,b)| *a == *b)
	}

	pub fn set(&mut self, x: usize, y: usize, value: bool) {
		self.bits[x + y * self.width] = value;
	}

	pub fn get(&self, x: usize, y: usize) -> bool {
		self.bits[x + y * self.width]
	}

	pub fn print(&self) {
		for y in 0..self.width {
			for x in 0..self.width {
				print!("{}", if self.bits[x + y * self.width] { " #" } else { " ." });
			}
			println!("");
		}
	}

	pub fn read(filename: &str) -> Board {
		let f     = BufReader::new(File::open(filename).expect("open failed"));
		let mut width = 0;
		let mut bits  = vec![false; 0];

		for line in f.lines() {
			let l = line.expect("lines failed");
			let trimmed = l.trim();
			if width == 0 {
				width = trimmed.len();
			}

			assert_eq!(trimmed.len(), width);
			bits.extend(trimmed.chars().map(|x| x == '#'));
		}

		assert_eq!(bits.len(), width * width);

		return Board {
			bits: bits,
			width: width,
		}
	}
}