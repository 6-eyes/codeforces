use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    println!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr) => {
        $iter.next().ok_or(Error::Iter)?.parse::<usize>()?
    };
}

fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, m) = (parse!(iter), parse!(iter));

    let (mut shari, mut neta) = (Vec::with_capacity(n), Vec::with_capacity(m));
    for _ in 0..n {
        shari.push(parse!(iter) * 2);
    }

    for _ in 0..m {
        neta.push(parse!(iter));
    }

    shari.sort_unstable();
    neta.sort_unstable();

    let ans = shari.into_iter().fold((0, 0), |(mut ans, mut pn), s| {
        if neta[pn] <= s {
            ans += 1;
            pn += 1;
        }
        (ans, pn)
    }).0;

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
        let input = r#"4 5
4 2 1 8
14 9 3 2 9
"#;

        std::assert_matches!(solve(input), Ok(3));
    }

    #[test]
    fn test_2() {
        let input = r#"3 3
5 5 3
11 1000 1000
"#;

        std::assert_matches!(solve(input), Ok(0));
    }

    #[test]
    fn test_3() {
        let input = r#"8 7
2 3 4 4 4 3 2 3
8 5 5 9 9 7 1
"#;

        std::assert_matches!(solve(input), Ok(5));
    }
}
