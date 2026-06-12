use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, ops::RangeBounds, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
	let mut s = String::new();
	stdin().read_to_string(&mut s).map_err(Error::Input)?;

	print!("{}", solve(&s)?);

	Ok(())
}

macro_rules! parse {
	($iter: expr) => {
		$iter.next().ok_or(Error::Iter)?.parse::<usize>()?
	};
}

fn solve(input: &str) -> Result<String, Error> {

	let mut iter = input.split_ascii_whitespace();
	let (n, q) = (parse!(iter), parse!(iter));

	let mut b = 0;
	// track [ rows(type 1), cols(type 2) ]
	let mut f = [FenwickTree::new(q + 1, 0), FenwickTree::new(q + 1, 0)];
	// the last seen change on the [ rows: no rows are marked initially, cols: all cols are marked initially ]
	let mut l = [ vec!{ None; n + 1 }, vec!{ Some(0); n + 1 } ];
	let mut ans = String::new();

	for idx in 1..=q {
		// t => type is either 1 or 2
		// x => row/column number. 1 indexed.
		let (t, x) = (parse!(iter) - 1, parse!(iter));

		if let Some(last_seen_query_idx) = l[t][x] {
			// find the number last occurences of opposite type after the last seen on the row x
			let other_kind = f[1 - t].sum(last_seen_query_idx..idx);
			match t {
				0 => b += other_kind,
				1 => b -= other_kind,
				t => unreachable!("invalid type {t}"),
			}

			f[t].add(last_seen_query_idx, -1);
		}
		else {
			// first change
			// only valid for rows which are initially `None`. once defined, these are set to Some
			// cols will never
			b += n as isize;
		}

		// update the visited count
		f[t].add(idx, 1);
		// update last seen
		l[t][x] = Some(idx);

		use std::fmt::Write;
		writeln!(ans, "{b}")?;
	}

	Ok(ans)
}

/// Source:  https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/fenwicktree.rs
#[derive(Debug)]
struct FenwickTree{
	data: Vec<isize>,
	init: isize,
}

impl FenwickTree {
	fn new(n: usize, init: isize) -> Self {
		Self{
			data: vec!{ init; n },
			init,
		}
	}

	/// Adds the value to the given index.
	/// 
	/// **Complexity:** O(log(n))
	fn add(&mut self, mut idx: usize, val: isize) {
		// fenwick trees are 1 indexed
		idx += 1;
		// iterate till the end of the array
		while idx <= self.data.len() {
			// the value for index 1 is stored at data[0], for index 2, it is stored at data[1] and so on.
			self.data[idx - 1] += val;
			// jump to the next index which is idx & -idx
			idx += idx & idx.wrapping_neg();
		}
	}

	/// Fetches the sum of the provided range
	/// 
	/// **Complexity:** O(log(n))
	fn sum(&self, range: impl RangeBounds<usize>) -> isize {
		let accum = |mut idx| {
			let mut sum = self.init;

			while idx > 0 {
				sum += self.data[idx - 1];
				idx &= idx - 1;
			}

			sum
		};

		use std::ops::Bound::*;
		let r = match range.end_bound() {
		    Included(r) => r + 1,
		    Excluded(r) => *r,
		    Unbounded => self.data.len(),
		};

		let l = match range.start_bound() {
		    Included(l) => *l,
		    Excluded(l) => l + 1,
		    Unbounded => return accum(r),
		};

		accum(r) - accum(l)
	}
}

#[derive(Debug)]
enum Error {
	Input(std::io::Error),
	Iter,
	Parse(ParseIntError),
	Write(std::fmt::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Input(e) => write!(f, "unable to fetch input: {e}"),
			Error::Iter => write!(f, "unable to fetch from iterator"),
			Error::Parse(e) => write!(f, "error parsing element: {e}"),
			Error::Write(e) => write!(f, "error writing to the string: {e}"),
		}
	}
}

impl From<ParseIntError> for Error {
	fn from(value: ParseIntError) -> Self {
		Self::Parse(value)
	}
}

impl From<std::fmt::Error> for Error {
	fn from(value: std::fmt::Error) -> Self {
		Self::Write(value)
	}
}

impl Termination for Error {
	fn report(self) -> std::process::ExitCode {
		match self {
			Error::Input(_) => ExitCode::from(1),
			Error::Iter => ExitCode::from(2),
			Error::Parse(_) => ExitCode::from(3),
			Error::Write(_) => ExitCode::from(4),
		}
	}
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
	use crate::{solve, FenwickTree};
	use std::ops::Bound::*;

	#[test]
	fn test_1() {
		let input = r#"3 4
1 1
1 3
2 2
1 1
"#;

		let output = r#"3
6
4
5
"#.to_string();

		std::assert_matches!(solve(input), Ok(o) if o == output);
	}

	#[test]
	fn test_2() {
		let input = r#"300000 1
2 300000
"#;

		let output = r#"0
"#.to_string();

		std::assert_matches!(solve(input), Ok(o) if o == output);
	}

	#[test]
	fn test_fenwick() {
		let mut bit = FenwickTree::new(5, 0);

		for i in 0..5 {
			// fn add(index, value)
			bit.add(i, i as isize + 1);
		}

		assert_eq!(bit.sum(0..5), 15);
		assert_eq!(bit.sum(0..4), 10);
		assert_eq!(bit.sum(1..3), 5);
		assert_eq!(bit.sum(..), 15);
		assert_eq!(bit.sum(..2), 3);
		assert_eq!(bit.sum(..=2), 6);
		assert_eq!(bit.sum(1..), 14);
		assert_eq!(bit.sum(1..=3), 9);
		assert_eq!(bit.sum((Excluded(0), Included(2))), 5);
	}
}
