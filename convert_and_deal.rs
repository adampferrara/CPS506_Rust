fn best_hand(hand: Vec<(u32, String)>) -> (u32, [u32; 2], Vec<(u32, String)>) {
   
        if (find_royal_flush(*hand).is_none()) = (find_royal_flush(*hand).unwrap())
        
        if (find_straight_flush(*hand).is_none()) = (find_straight_flush(*hand).unwrap())
    
        if (find_four_of_kind(*hand).is_none()) = (find_four_of_kind(*hand).unwrap())
    
        if (find_full_house(*hand).is_none()) = (find_full_house(*hand).unwrap())
    
        if (find_flush(*hand).is_none()) = (find_flush(*hand).unwrap())
    
        if (find_straight(*hand).is_none()) = (find_straight(*hand).unwrap())
    
        if (find_three_of_kind(*hand).is_none()) = (find_three_of_kind(*hand).unwrap())
    
        if (find_two_pair(*hand).is_none()) = (find_two_pair(*hand).unwrap())
    
        if (find_pair(*hand).is_none()) = (find_pair(*hand).unwrap())
        
        if (find_ace_high(*hand).is_none()) = (find_ace_high(*hand).unwrap())
        
}
