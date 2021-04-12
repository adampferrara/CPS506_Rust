fn find_ace_high(arr: &[(u32, &str)]) -> Option<(u32, [u32; 2], [(u32, &'static str); 2])> {
	match arr {
		[ (1, s1), .. ] => Some((1, [1, 0], [(1, s1), (3, "C")])),
		[ _a, .. ] => Some((1, [2, 0], [(3, "C"), (5, "S")])),
		_ => None,
	}
}

/*
fn deal(&[c1, c2, c3, c4, c5, c6, c7, c8, c9]: &[u32, u32, u32, u32, u32, u32, u32, u32, u32]) -> &[&str] {
	let mut hand_a = [c1, c3, c5, c6, c7, c8, c9];
	hand_a.sort();
	hand_a.reverse();
	let mut hand_b = [c2, c4, c5, c6, c7, c8, c9];
	hand_b.sort();
	hand_b.reverse();
	
	let best_a = best_hand(hand_a);
	let best_b = best_hand(hand_b);
	
	let winner = match
}
*/

fn main() {
	let x = [(1, "C"), (2, "H")];
	let y = find_ace_high(&x);
	let y_unpacked = y.unwrap();
	println!("{}{}", y_unpacked.2[0].0, y_unpacked.2[0].1);
}
