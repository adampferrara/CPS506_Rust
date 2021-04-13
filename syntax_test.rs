fn main() {
	let x = (1, "C");
	let mut hand_a = vec![x, (5, "D"), (5, "S"), (7, "C"), (8, "H"), (10, "C"), (13, "D")];
	//let mut hand_b = vec![x, (1, "S")];
	hand_a.sort();
	hand_a.reverse();
	
	let y = match &*hand_a {
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if x5 == &1 => 
			Some((1, [1, 0], vec![(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)])),
		_ => None,
	};
	
	println!("{:?}", &hand_a);
	let y_unpacked = y.unwrap();
	println!("{}, {}, Card 1: {}{}",
			 y_unpacked.0, (y_unpacked.1 > [0, 0]), y_unpacked.2[0].0, y_unpacked.2[0].1);
}
