use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
	let mut s = String::new();
	stdin().read_to_string(&mut s).map_err(Error::Input)?;

	println!("{}", solve(&s)?);

	Ok(())
}

macro_rules! next {
	($iter:expr) => ($iter.next().ok_or(Error::Iter)?);
}

fn solve(input: &str) -> Result<usize, Error> {
	let mut iter = input.split_ascii_whitespace();
	let (h, w, k) = (next!(iter).parse::<usize>()?, next!(iter).parse::<usize>()?, next!(iter).parse::<usize>()?);

	let mut grid = vec!{ vec!{ 0; w + 1 }; h + 1 };
	for i in 1..=h {
		let mut line = next!(iter).chars().map(|c| c as usize - '0' as usize);
		#[allow(clippy::needless_range_loop)]
		for j in 1..=w {
			grid[i][j] = grid[i - 1][j] + next!(line);
		}
	}

	// take row combinations
	let ans = (1..=h).flat_map(|s| (s..=h).map(move |e| (s, e))).map(|(s, e)| {
		let mut b = vec!{ 0; w + 1 };
		(1..=w).for_each(|i| b[i] = b[i - 1] + grid[e][i] - grid[s - 1][i]);

		// find pairs whose difference is equal to k in the sorted list b
		let (mut r1, mut r2) = (0, 0);
		// we only need to iterate to the second last element. the total length is `w + 1`
		(0..w).map(|l| {
			// the next pointer should be max of right element and the l + 1 element
			// find r1 where b[r] - b[l] >= k
			r1 = r1.max(l + 1);
			while r1 < w + 1 && b[r1] - b[l] < k { r1 += 1; }

			// find r2 where b[r] - b[l] > k
			r2 = r2.max(l + 1);
			while r2 < w + 1 && b[r2] - b[l] <= k { r2 += 1; }

			r2 - r1
		}).sum::<usize>()

	}).sum();

	Ok(ans)
}

#[derive(Debug)]
enum Error {
	Input(std::io::Error),
	Iter,
	Parse(ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	match self {
	        Error::Input(e) => write!(f, "unable to fetch input: {e}"),
	        Error::Iter => write!(f, "unable to fetch from iterator"),
	        Error::Parse(e) => write!(f, "error parsing element: {e}"),
	    }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
    	Self::Parse(value)
    }
}

impl Termination for Error {
    fn report(self) -> std::process::ExitCode {
        match self {
            Error::Input(_) => ExitCode::from(1),
            Error::Iter => ExitCode::from(2),
            Error::Parse(_) => ExitCode::from(3),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
	fn test_1() {
		let input = r#"3 4 3
1001
1101
0110
"#;

		std::assert_matches!(solve(input), Ok(8));
	}

    #[test]
	fn test_2() {
		let input = r#"5 4 20
0101
1010
0101
1010
0101
"#;

		std::assert_matches!(solve(input), Ok(0));
	}

	#[test]
	fn test_3() {
		let input = r#"15 20 17
10111101101100000100
01100000000010000011
01110010111000111000
11001100000111011000
10100001100011100010
01101000101010000101
10110001111110000100
10110011101100101101
01010001110011001001
01010110010001100110
01110100011110011110
01100000100111010010
11001101100111101100
10111100010101111011
00101101011100010000
"#;

		std::assert_matches!(solve(input), Ok(448));
	}
}
