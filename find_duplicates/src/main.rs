use std::collections::HashSet;

fn main() {
	let vector = vec!["Abacate", "Ameixa", "Banana", "Maçã", "Banana", "Maçã"];
	
	println!("{:?}", vector);

	let mut duplicate_counter = 0;

	let mut set = HashSet::new();

	for entry in vector.iter() {
		if set.contains(entry) {
			println!("{:?} is a duplicate.", entry);
			duplicate_counter += 1;
		} else {
			set.insert(entry);
		}
	}

	println!("{:?} duplicate(s) found.", duplicate_counter);
}
