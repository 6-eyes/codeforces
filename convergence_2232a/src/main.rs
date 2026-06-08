use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).expect("unable to parse input");

    let mut iter = s.lines();
    let tests = iter.next().expect("unable to read number of tests").parse::<usize>().expect("unable to parse test cases count");

    for _ in 0..tests {
        let num_elements = iter.next().expect("unable to read number of elements for test").parse::<usize>().expect("unable to parse number of elements");

        let elements = {
            let mut v = Vec::with_capacity(num_elements);
            iter.next().expect("unable to read data").split_whitespace().for_each(|s| {
                let e = s.parse::<usize>().expect("unable to parse element");
                v.push(e);
            });

            assert_eq!(v.len(), num_elements, "invalid number of elements received, expected: {}, received: {}", num_elements, v.len());
            v
        };

        println!("{}", get_min_calls(elements));
    }
}

fn get_min_calls(mut elements: Vec<usize>) -> usize {
    elements.sort_unstable();

    let mid_idx = elements.len() / 2;
    let (mut left, mut right) = (0, 0);
    for e in elements.iter() {
        if *e < elements[mid_idx] { left += 1 }
        if *e > elements[mid_idx] { right += 1 }
    }

    left.max(right)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let elements = vec!{ 1, 2, 3, 4, 5 };
        assert_eq!(2, super::get_min_calls(elements));
    }

    #[test]
    fn test_2() {
        let elements = vec!{ 1, 1, 1, 2, 2 };
        assert_eq!(2, super::get_min_calls(elements));
    }

    #[test]
    fn test_3() {
        let elements = vec!{ 3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5 };
        assert_eq!(5, super::get_min_calls(elements));
    }

    #[test]
    fn test_4() {
        let elements = vec!{ 1, 2, 2, 2, 2 };
        assert_eq!(1, super::get_min_calls(elements));
    }
}
