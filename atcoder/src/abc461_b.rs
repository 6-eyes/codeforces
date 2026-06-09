use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
	let mut s = String::new();
	stdin().read_to_string(&mut s).map_err(Error::Input)?;

	println!("{}", lie_detector(&s)?);

	Ok(())
}

fn lie_detector(input: &str) -> Result<&'static str, Error> {
	let mut iter = input.split_ascii_whitespace();
	let n = iter.next().ok_or(Error::Iter)?.parse::<usize>()?;

	let claim = {
		let mut c = Vec::with_capacity(n);
		for _ in 0..n {
			c.push(iter.next().ok_or(Error::Iter)?.parse::<u8>()?);
		}
		c
	};

	let truth = {
		let mut t = Vec::with_capacity(n);
		for _ in 0..n {
			t.push(iter.next().ok_or(Error::Iter)?.parse::<u8>()?);
		}
		t
	};

	let b = claim.iter().enumerate().all(|(i, c)| truth[*c as usize - 1] == i as u8 + 1);

	Ok(if b { "Yes" } else { "No" })
}

#[derive(Debug)]
enum Error {
	Input(std::io::Error),
	Iter,
	Parse(ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
    	Self::Parse(value)
    }
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
    use crate::lie_detector;

	#[test]
	fn test_1() {
		let input = r#"3
3 1 2
2 3 1
"#;

		std::assert_matches!(lie_detector(input), Ok("Yes"));
	}

	#[test]
	fn test_2() {
		let input = r#"4
1 2 3 4
1 3 2 4
"#;

		std::assert_matches!(lie_detector(input), Ok("No"));
	}

	#[test]
	fn test_3() {
		let input = r#"5
2 4 5 1 3
4 1 5 2 3
"#;

		std::assert_matches!(lie_detector(input), Ok("Yes"));
	}
}
