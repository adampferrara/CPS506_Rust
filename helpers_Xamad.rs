/*
fn find_straight(vec: &[(u32, String)]) -> Option<(u32, [u32; 2], [(u32, &'static str); 2])> {
	
}

fn find_three_of_kind(vec: &[(u32, String)]) -> Option<(u32, [u32; 2], [(u32, &'static str); 2])> {
	
}

fn find_two_pair(vec: &[(u32, String)]) -> Option<(u32, [u32; 2], [(u32, &'static str); 2])> {
	
}

fn find_pair(vec: &[(u32, String)]) -> Option<(u32, [u32; 2], [(u32, &'static str); 2])> {
	
}
*/

fn find_ace_high(vec: &[(u32, String)]) -> Option<(u32, [u32; 2], [(u32, String); 2])> {
	match vec {
		[ (1, s1), .. ] => Some((1, [1, 0], [(1, s1), (3, "C")])),
		[ _a, .. ] => Some((1, [2, 0], [(3, "C"), (5, "S")])),
		_ => None,
	}
}

/*
pub fn deal(perm: &[u32;9]) -> Vec<String> {
	let mut hand_a = [perm[0], perm[2], perm[4], perm[5], perm[6], perm[7], perm[8]];
	hand_a.sort();
	hand_a.reverse();
	let mut hand_b = [perm[1], perm[3], perm[4], perm[5], perm[6], perm[7], perm[8]];
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
