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

fn convert(i) -> (u32, String){
    if (i <= 13) 
        let i = (i,"C")
    if ((i > 13) && (i<= 26)) 
        let i = (i - 13, "D")
    if ((i > 26) && (i<= 39)) 
        let i = (i - 26, "H")
    else 
        let i = (i - 39,"S")
}
    
fn get_fst((u32,u32)->(u32)){
    match &*hand{
        [a,b,c]
            let (*a,..,..) = *a
    }
}
    
fn get_snd((u32,u32)->(u32)){
    match &*hand{
        [a,b,c]
            let (..,*b,..) = *b
    }
}
    
fn get_thd((u32,u32)->(u32)){
    match &*hand{
        [a,b,c]
            let (..,..,*c) = *c
    }
}
