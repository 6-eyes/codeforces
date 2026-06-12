use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    let mut iter = s.split_ascii_whitespace();
    let (n, m) = (iter.next().ok_or(Error::Iter)?.parse::<u32>()?, iter.next().ok_or(Error::Iter)?.parse::<u32>()?);

    println!("{}", solve(n, m));

    Ok(())
}

fn solve(n: u32, mut m: u32) -> u32 {
    let mut ans = 0;
    while m > 0 {
        m = n % m;
        ans += 1;
    }

    ans
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
        assert_eq!(solve(8, 5), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(14, 6), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(460, 33), 5);
    }
}
