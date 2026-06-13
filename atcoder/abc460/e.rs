use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

     solve(&s)?.into_iter().for_each(|a| println!("{a}"));

    Ok(())
}

macro_rules! parse {
    ($iter: expr) => {
        $iter.next().ok_or(Error::Iter)?.parse::<usize>()?
    };
}

fn solve(input: &str) -> Result<Vec<usize>, Error> {
    const MOD: usize = 998244353;
    const MAX_DIGITS: usize = 19;

    let mut iter = input.split_ascii_whitespace();
    let t = parse!(iter);
    let p10: [usize; MAX_DIGITS + 1] = std::array::from_fn(|i| 10usize.pow(i as u32));
    let mut ans = Vec::with_capacity(t);

    /// Calculates the gcd/hcf of two numbers a and b
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    for _ in 0..t {
        let (n, m) = (parse!(iter), parse!(iter));
        // two numbers in 1..=n, (x, y) s.t., xy % m % M == (x + y) % m % M
        // d === is the number of digits in y
        // (x * 10^d + y) % m == (x + y) % m
        // (x * 10^d + y) - x - y is a multiple of m
        // (x * 10^d - x) is a multiple of m
        // x(10^d - 1) is a multiple of m
        // if g is the gcd of (10^d - 1) and m, then x is a multiple of m / g
        // the possible values of x then become: m/g, 2m/g, 3m/g, ... until N
        // therefore, total possible values of x: (n / (m / g))

        let mut local_ans = 0;
        for d in 1..=MAX_DIGITS {
            // the left limit is the first number with d digits
            // the right limit (non-inclusive) is the one with d + 1 digits.
            // this is capped at n + 1 because n is inclusive.
            let range = p10[d - 1]..p10[d].min(n + 1);
            if range.is_empty() { break; }

            let g = gcd(p10[d] - 1, m);

            // range gives the possible values of y
            // n / (m / g) gives the possible values of x
            // total possible values is the multiplication of these
            local_ans += (range.len() % MOD) * ((n / (m / g)) % MOD);
            local_ans %= MOD;
        }

        ans.push(local_ans);
    }

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
        let input = r#"4
3 2
123 456
20260530 460
123456789123456789 998244353
"#;

        let output = [3, 0, 922576091, 422081792];
        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}
