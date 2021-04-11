fn find_ace_high(arr: &[(i32, &str)]) -> Option<(i32, [i32; 2], [(i32, &'static str); 2])> {
	match arr {
		[ (1, "H"), .. ] => Some((1, [1, 0], [(1, "H"), (3, "C")])),
		[ _a, .. ] => Some((1, [2, 0], [(3, "C"), (5, "S")])),
		_ => None,
	}
}

fn main() {
	let x = [(1, "C"), (2, "H")];
	let y = find_ace_high(&x);
	let y_unpacked = y.unwrap();
	println!("{}{}", y_unpacked.2[0].0, y_unpacked.2[0].1);
}
