pub fn deal(perm: [u32; 9]) -> Vec<String> {
	let mut hand_a = vec![convert(perm[0]), convert(perm[2]), convert(perm[4].clone()),
						  convert(perm[5].clone()), convert(perm[6].clone()),
						  convert(perm[7].clone()), convert(perm[8].clone())];
	hand_a.sort();
	hand_a.reverse();
	
	let mut hand_b = vec![convert(perm[1]), convert(perm[3]), convert(perm[4].clone()),
						  convert(perm[5].clone()), convert(perm[6].clone()),
						  convert(perm[7].clone()), convert(perm[8].clone())];
	hand_b.sort();
	hand_b.reverse();
	
	let best_a = best_hand(hand_a);
	let best_b = best_hand(hand_b);
	
	let winner = match best_a {
		x if x.0 < best_b.0 => best_b.2,
		x if x.0 > best_b.0 => best_a.2,
		x if (x.0 == best_b.0) && (x.1 < best_b.1) => best_b.2
		x if (x.0 == best_b.0) && (x.1 > best_b.1) => best_a.2
		_ => best_a.2,
	};
	
	let mut win_final = Vec::new();
	win_final.push(format!("{}{}", winner[0].0, winner[0].1));
	win_final.push(format!("{}{}", winner[1].0, winner[1].1));
	win_final.push(format!("{}{}", winner[2].0, winner[2].1));
	win_final.push(format!("{}{}", winner[3].0, winner[3].1));
	win_final.push(format!("{}{}", winner[4].0, winner[4].1));
	return win_final;
}

fn find_straight(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
	match &*hand {
		[(x1, s1), (x2, s2), (x3, s3), (x4, s4), .. , (x5, s5)]
			if ((*x1 - *x2) == 1) && ((*x2 - *x3) == 1) && ((*x3 - *x4) == 1) && (*x5 == 1) => 
			Some((5, [14, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5), _, _] if ((*x1 - *x2) == 1) &&
			((*x2 - *x3) == 1) && ((*x3 - *x4) == 1) && ((*x4 - *x5) == 1) => 
			Some((5, [*x1, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5), _] if ((*x1 - *x2) == 1) &&
			((*x2 - *x3) == 1) && ((*x3 - *x4) == 1) && ((*x4 - *x5) == 1) => 
			Some((5, [*x1, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if ((*x1 - *x2) == 1) &&
			((*x2 - *x3) == 1) && ((*x3 - *x4) == 1) && ((*x4 - *x5) == 1) => 
			Some((5, [*x1, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		_ => None,
	}
}

fn find_three_of_kind(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
	match &*hand {
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x3 == *x4) && (*x3 == *x5) && (*x3 == 1) => 
			Some((4, [14, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x1 == *x3) => 
			Some((4, [*x1, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x2 == *x3) && (*x2 == *x4) => 
			Some((4, [*x2, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x3 == *x4) && (*x3 == *x5) => 
			Some((4, [*x3, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		_ => None,
	}
}

fn find_two_pair(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
	match &*hand {
		[(x1, s1), (x2, s2), .., (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x4 == *x5) && (*x4 == 1) => 
			Some((3, [14, *x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), _, (x3, s3), (x4, s4), (x5, s5)]
			if (*x2 == *x3) && (*x4 == *x5) && (*x4 == 1) => 
			Some((3, [14, *x2], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x2 == *x3) && (*x4 == *x5) && (*x4 == 1) => 
			Some((3, [14, *x2], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x3 == *x4) => 
			Some((3, [*x1, *x3], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x4 == *x5) => 
			Some((3, [*x1, *x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
			if (*x2 == *x3) && (*x4 == *x5) => 
			Some((3, [*x2, *x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), _, (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x3 == *x4) => 
			Some((3, [*x1, *x3], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), _, (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x4 == *x5) => 
			Some((3, [*x1, *x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), _, (x3, s3), (x4, s4), (x5, s5)]
			if (*x2 == *x3) && (*x4 == *x5) => 
			Some((3, [*x2, *x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), _, _, (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x3 == *x4) => 
			Some((3, [*x1, *x3], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), _, _, (x3, s3), (x4, s4), (x5, s5)]
			if (*x1 == *x2) && (*x4 == *x5) => 
			Some((3, [*x1, *x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), _, _, (x3, s3), (x4, s4), (x5, s5)]
			if (*x2 == *x3) && (*x4 == *x5) => 
			Some((3, [*x2, *x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		_ => None,
	}
}

fn find_pair(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
	match &*hand {
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if (*x5 == *x4) && (*x5 == 1) => 
			Some((2, [14, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if *x1 == *x2 => 
			Some((2, [*x1, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if *x2 == *x3 => 
			Some((2, [*x2, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if *x3 == *x4 => 
			Some((2, [*x3, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if *x4 == *x5 => 
			Some((2, [*x4, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		_ => None,
	}
}

fn find_ace_high(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
	match &*hand {
		[.., (x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)] if *x5 == 1 => 
			Some((1, [*x5, 0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
				 (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
		_ => None,
	}
}
