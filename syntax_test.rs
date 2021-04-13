fn find_ace_high(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
	match &*hand {
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if *x5 == 1 => 
			Some((1, [*x5, 0], vec![(*x1, String::from(s1)), (*x2, String::from(s2)),
				 (*x3, String::from(s3)), (*x4, String::from(s4)), (*x5, String::from(s5))])),
		_ => None,
	}
}

fn main() {
	let x = (1, String::from("C"));
	let mut hand_a =
		vec![x, (5, String::from("D")), (5, String::from("S")), (7, String::from("C")),
				(8, String::from("H")), (10, String::from("S")), (13, String::from("D"))];
	//let mut hand_b = vec![x, (1, "S")];
	hand_a.sort();
	hand_a.reverse();
	
	/*
	let y = match &*hand_a {
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if x5 == &1 => 
			Some((1, [x5, &0], vec![(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)])),
		_ => None,
	};
	*/
	
	let y = find_ace_high(hand_a.clone());
	
	println!("{:?}", &hand_a);
	let y_unpacked = y.unwrap();
	println!("{}, {}, Card 1: {}{}",
			 y_unpacked.0, (y_unpacked.1 == [1, 0]), y_unpacked.2[0].0, y_unpacked.2[0].1);
}
