use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

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
    let n = parse!(iter);
    let mut ans = String::new();

    for _ in 0..n {
        let (x1, y1, r1) = (parse!(iter), parse!(iter), parse!(iter));
        let (x2, y2, r2) = (parse!(iter), parse!(iter), parse!(iter));
        // (r2 + r1)^2 > (x2 - x1)^2 + (y2 - y1)^2 <-- no point
        // (r2 - r1)^2 < (x2 - x1)^2 + (y2 - y1)^2 <-- no point
        // (r2 - r1)^2 < d < (r2 + r1)^2
        let r = r1.abs_diff(r2).pow(2)..=(r1 + r2).pow(2);

        let d = |a: usize, b: usize| a.abs_diff(b).pow(2);
        let x = d(x1, x2);
        let y = d(y1, y2);
        let s = x + y;

        use std::fmt::Write;
        writeln!(ans, "{}", if r.contains(&s) { "Yes" } else { "No" })?;
    }

    Ok(ans)
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
			Error::Write(e) => write!(f, "error writing to string: {e}"),
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
    use crate::solve;


    #[test]
    fn test_1() {
        let input = r#"7
0 0 2 2 3 2
0 0 2 2 3 1
1 2 5 3 2 1
5 4 2 8 8 3
2 1 5 5 1 2
0 0 1 0 0 1
0 0 500000000 1 1000000000 500000000
"#;

        let output = r#"Yes
No
No
Yes
Yes
Yes
No
"#;

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}
