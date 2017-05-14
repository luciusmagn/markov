extern crate regex;
extern crate rand;

use regex::Regex;
use std::collections::HashMap;

fn main()
{
	let poetry = include_str!("kytice.txt");
	let word_re = match Regex::new(r" +")
	{
		Ok(r) => r,
		Err(_) => panic!("something went wrong"),
	};

	let words_vec: Vec<&str> = word_re.split(poetry).collect();
	let mut words = words_vec.iter();
	let mut transition = HashMap::new();
	let mut keys = Vec::new();
	loop
	{
		let word1 = match words.next()
		{
			Some(x) => x,
			None => break,
		};
		let word2 = match words.next()
		{
			Some(x) => x,
			None => break,
		};
		let word3 = match words.next()
		{
			Some(x) => x,
			None => break,
		};

		transition.insert((word1, word2), word3);
		keys.push((word1, word2))
	}

	let mut current: (&&str, &&str) = keys[rand::random::<usize>() % keys.len()];
	let mut third = transition.get(&current).unwrap();
	for _ in 0..100
	{
		print!("{} ", current.0);
		current = (current.1, third);
		third = match transition.get(&current)
		{
			Some(t) => t,
			None =>
			{
				current = keys[rand::random::<usize>() % keys.len()];
				transition.get(&current).unwrap()
			}
		};
	}
}
