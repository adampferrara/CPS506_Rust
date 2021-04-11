fn find_ace_high(arr: &[i32]) -> Option<(i32, [i32; 2], [(i32, String); 2])> {
	match arr {
		[ 1, .. ] => Some((1, [1, 0], [(1, String::from("H")), (3, String::from("C"))])),
		[ _a, .. ] => Some((1, [2, 0], [(3, String::from("C")), (5, String::from("S"))])),
		_ => None,
	}
}

fn main() {
	let x = find_ace_high(&[1, 2, 3]);
	let x_unpacked = x.unwrap();
	println!("{}", x_unpacked.1[0]);
}
