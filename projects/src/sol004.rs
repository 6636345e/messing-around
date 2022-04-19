use std::collections::HashMap;

fn main() {
	let user_string = String::from("abcabcbb");

	println!("{}", length_of_longest_substring(user_string));
}

pub fn length_of_longest_substring(s: String) -> i32 {
	let mut hash = HashMap::with_capacity(s.len());
	let mut max = 0;
	let mut start = 0;
	let mut end = 0;

	for (i, item) in s.chars().enumerate() {
		if let Some(j) = hash.get(&item) {
			//checks that current symbol is in the current window.
			if *j >= start {
				let curr = end - start;
				if max < curr {
					max = curr;
				}
				//move window
				start = *j + 1;
			}
		}
		end += 1;
		hash.insert(item, i);
	}
	let curr = end - start;
	if max < curr {
		max = curr;
	}
	max as i32
}
