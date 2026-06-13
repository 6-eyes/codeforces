use std::{collections::VecDeque, fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr) => {
        $iter.next().ok_or(Error::Iter)?
    };
}

fn solve(input: &str) -> Result<String, Error> {
    const DX: [isize; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
    const DY: [isize; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

    let mut iter = input.split_ascii_whitespace();
    let (h, w) = (parse!(iter).parse::<usize>()?, parse!(iter).parse::<usize>()?);

    // true if the cell is black, false otherwise
    let initial_grid = {
        let mut grid = vec!{ Vec::with_capacity(w); h };

        for v in grid.iter_mut() {
            for c in parse!(iter).chars() {
                let b = match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!("invalid input character {c}"),
                };

                v.push(b);
            }
        }

        grid
    };

    // apply operation once
    // this is required to make any while cell appear once
    let mut grid = vec!{ Vec::with_capacity(w); h };
    for i in 0..h {
        for j in 0..w {
            // for white cell
            let v = !initial_grid[i][j] && (0..8).any(|d| matches!((i.checked_add_signed(DX[d]), j.checked_add_signed(DY[d])), (Some(ni), Some(nj)) if ni < h && nj < w && initial_grid[ni][nj]));
            grid[i].push(v);
        }
    }
    
    // we use an even number here to handle the case of all white blocks. In this case, no matter the iterations, the cells will remain while.
    // if u32::MAX is used, the ans would contain black cells instead of white.
    const MAX: u32 = u32::MAX - 1;
    // initialize distance grid with MAX distance
    let mut dist = vec!{ vec!{ MAX; w }; h };
    let mut q = VecDeque::new();

    // fill q with black indices
    grid.iter().enumerate().for_each(|(i, r)| r.iter().enumerate().for_each(|(j, b)| if *b {
        dist[i][j] = 0;
        q.push_back((i, j));
    }));

    // fill distance
    // distance will only be updated if the value is u16::MAX. This will prevent overwriting by another block.
    // the new elements are added th the end of the queue. this means that the distance to a block once found (MAX gets overwritten by distance + 1) is the minimum distance.
    // BFS
    while let Some((i, j)) = q.pop_front() {
        (0..8).for_each(|d| if let (Some(ni), Some(nj)) = (i.checked_add_signed(DX[d]), j.checked_add_signed(DY[d])) && ni < h && nj < w && dist[ni][nj] == MAX {
            dist[ni][nj] = dist[i][j] + 1;
            q.push_back((ni, nj));
        });
    }

    let ans = dist.into_iter().fold(String::with_capacity(h * (w + 1)), |mut s, r| {
        use std::fmt::Write;
        r.into_iter().for_each(|d| write!(s, "{}", if d & 1 == 0 { '.' } else { '#' }).unwrap());
        // add new line
        writeln!(s).unwrap();
        s
    });

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
        let input = r#"3 4
#.#.
.#..
#...
"#.to_string();

        let output = r#"#.#.
.#..
#..#
"#;

        std::assert_matches!(solve(&input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = r#"3 3
###
###
###
"#;
        let output = r#"...
...
...
"#;
        std::assert_matches!(solve(&input), Ok(o) if o == output);
    }

    #[test]
    fn test_3() {
        let input = r#"5 7
.#.....
.......
..#....
.......
....#..
"#;
        let output = r#".#.##.#
....#..
#.#.###
#.....#
###.#.#
"#;
        std::assert_matches!(solve(&input), Ok(o) if o == output);
    }
}
