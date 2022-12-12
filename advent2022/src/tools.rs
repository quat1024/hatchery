use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

///A collection of small-integer related traits.
///A little bit num-traits inspired, but way smaller, cause num-traits seems to make my compile times really bad.
pub trait Int: Copy + Eq + Default + Ord + Div<Output = Self> + Rem<Output = Self> + Mul<Output = Self> {
	fn zero() -> Self {
		Default::default()
	}
	fn one() -> Self;
}

//Of course it can be implemented for other integer types too, but it starts to affect compile times.
//I'm also forgoing a macro to copy-paste this code for the same compile-time reason.
impl Int for usize {
	fn one() -> Self {
		1
	}
}

impl Int for isize {
	fn one() -> Self {
		1
	}
}

impl Int for i32 {
	fn one() -> Self {
		1
	}
}

///The greatest common divisor of two numbers, using euclid's method.
pub fn gcd<N>(a: N, b: N) -> N
where
	N: Int,
{
	if a == N::zero() {
		b
	} else if b == N::zero() {
		a
	} else {
		let (big, small) = (N::max(a, b), N::min(a, b));
		gcd(small, big % small)
	}
}

///The greatest common divisor of an iterator of numbers. Empty iterators return 0.
///This isn't on an extension trait; couldn't get that to work.
pub fn gcd_iter<N>(numbers: impl IntoIterator<Item = N>) -> N
where
	N: Int,
{
	numbers.into_iter().fold(N::zero(), |acc, n| gcd(acc, n))
}

///The least common multiple of two numbers.
pub fn lcm<N>(a: N, b: N) -> N
where
	N: Int,
{
	(a * b) / gcd(a, b)
}

///The least common multiple of an iterator of numbers. Empty iterators return 1.
///This isn't on an extension trait; couldn't get that to work.
pub fn lcm_iter<N>(numbers: impl IntoIterator<Item = N>) -> N
where
	N: Int,
{
	numbers.into_iter().fold(N::one(), |acc, n| lcm(acc, n))
}

/// Sometimes in `AoC` you get input that's a "list of lists", where two blank lines separate each list.
/// Splitting on \n\n works, usually, unless you're on Windows and get carriage returns too! This can happen if you use the clipboard.
///
/// So this function splits into chunks the "hard way". It scans a string line-by-line, copying a slice to each line into a bucket.
/// When a blank line is encountered, the bucket is added to the list-of-lists.
pub fn chunks<'a>(input: &'a str) -> Vec<Vec<&'a str>> {
	let mut chunks: Vec<Vec<&'a str>> = Vec::new();
	let mut bucket: Vec<&'a str> = Vec::new();

	for line in input.lines() {
		if line.trim().is_empty() {
			//finding an empty string to delimit each bucket
			//bucket.is_empty may be true if there's two blank lines in a row
			if !bucket.is_empty() {
				//stash away the current bucket, and make `bucket` point to a newly allocated one, in one step
				chunks.push(std::mem::take(&mut bucket));
			}
		} else {
			bucket.push(line);
		}
	}

	//the last one
	if !bucket.is_empty() {
		chunks.push(bucket);
	}

	chunks
}

///Grabs the first positive integer out of a string.
///Won't work on negative numbers, doesn't help you parse more than one, and has other shortcomings.
pub fn number_from_soup(line: &str) -> Option<usize> {
	let mut indexed_char_iter = line.chars().enumerate();

	let (start, _) = indexed_char_iter.find(|c| c.1.is_ascii_digit())?;
	if let Some((end, _)) = indexed_char_iter.find(|c| !c.1.is_ascii_digit()) {
		line[start..end].parse().ok()
	} else {
		line[start..].parse().ok()
	}
}

///Trait for borrowing two things from a slice at once.
///I pinched this entire trait, and its implementation, from Amos:
///<https://fasterthanli.me/series/advent-of-code-2022/part-5#borrow-checker-limitations-and-workarounds>
pub trait BorrowTwoMut<T> {
	fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T);
}

impl<T> BorrowTwoMut<T> for [T] {
	fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
		assert!(a != b);
		if a < b {
			let (l, r) = self.split_at_mut(b);
			(&mut l[a], &mut r[0])
		} else {
			let (l, r) = self.split_at_mut(a);
			(&mut r[0], &mut l[b])
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_gcd() {
		assert_eq!(gcd(20, 30), 10);
		assert_eq!(gcd(20, 35), 5);

		assert_eq!(gcd_iter([10, 20, 30]), 10);
		assert_eq!(gcd_iter([5, 10, 20]), 5);
		assert_eq!(gcd_iter([100, 105, 110]), 5);
	}

	#[test]
	fn test_lcm() {
		assert_eq!(lcm(20, 30), 60);
		assert_eq!(lcm(20, 10), 20);

		assert_eq!(lcm_iter([10, 20, 30]), 60);
		assert_eq!(lcm_iter([5, 10, 20]), 20);
	}

	#[test]
	fn test_chunks() {
		assert_eq!(
			chunks(
				"part1
part1
part1

part2
part2

part3"
			),
			vec![vec!["part1", "part1", "part1"], vec!["part2", "part2"], vec!["part3"],]
		);
	}

	#[test]
	fn test_number_from_soup() {
		assert_eq!(number_from_soup("Monkey 0:"), Some(0));
		assert_eq!(number_from_soup("12345 yeah"), Some(12345));
		assert_eq!(number_from_soup("If true: throw to monkey 2"), Some(2));
		assert_eq!(number_from_soup(""), None);
		assert_eq!(number_from_soup("No numbers here :("), None);
	}
}
