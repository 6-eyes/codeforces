use std::io::{Read, Write, stdin, stdout};

fn main() {
	let mut s = String::new();
	stdin().read_to_string(&mut s).expect("unable to read from standard input");

	let mut out = stdout();
	solve(&s).into_iter().for_each(|a| writeln!(out, "{}", if a { 1 } else { 0 }).expect("unable to write to stdout"));
}

macro_rules! parse {
	($iter:expr, $t:ty) => ($iter.next().expect("unable to parse next value").parse::<$t>().expect("unable to parse value"));
}

fn solve(input: &str) -> Vec<bool> {
	let mut iter = input.split_ascii_whitespace();
	let (n, q) = (parse!(iter, usize), parse!(iter, usize));

	#[derive(Debug)]
	struct Dsu {
		parent: Vec<usize>,
		/// The size of the group
		///
		/// This is used to determine the smaller group of the two
		size: Vec<usize>,
	}

	impl Dsu {
		fn new(n: usize) -> Self {
			Self {
				parent: (0..n).collect(),
				size: vec!{ 1; n },
			}
		}

		fn group(&mut self, e: usize) -> usize {
			if self.parent[e] != e {
				self.parent[e] = self.group(self.parent[e]);
			}

			self.parent[e]
		}

		fn join(&mut self, x: usize, y: usize) {
			let mut group_x = self.group(x);
			let mut group_y = self.group(y);

			if group_x == group_y { return; }

			if self.size[group_x] < self.size[group_y] {
				std::mem::swap(&mut group_x, &mut group_y);
			}

			// group y is smaller
			self.parent[group_y] = group_x;
			self.size[group_x] += self.size[group_y];
		}
	}

	let mut dsu = Dsu::new(n);
	let mut ans = Vec::new();
	for _ in 0..q {
		let (a, x, y) = (parse!(iter, u8), parse!(iter, usize), parse!(iter, usize));

		match a {
			0 => dsu.join(x, y),
			1 => ans.push(dsu.group(x) == dsu.group(y)),
			v => panic!("invalid action {v}"),
		}
	}

	ans
}

#[cfg(test)]
mod tests {
    use crate::solve;

	#[test]
	fn test() {
		let input = r#"4 7
1 0 1
0 0 1
0 2 3
1 0 1
1 1 2
0 0 2
1 1 3
"#;

		let output = {
			let output = r#"0
1
0
1
"#;
			output.lines().map(|s| s != "0").collect::<Vec<bool>>()
		};

		assert_eq!(solve(input), output);

	}
}
