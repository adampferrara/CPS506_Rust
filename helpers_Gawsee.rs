fn find_royal_flush(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
    match &*hand{
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == 13) && (*x2 == 12) && (*x3 == 11) && (*x4 == 10) && (*x5 == 1) &&
            (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5) =>
            Some((10,[*x1,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == 13) && (*x2 == 12) && (*x3 == 11) && (*x4 == 10) && (*x5 == 1) &&
            (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5) =>
            Some((10,[*x1,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == 13) && (*x2 == 12) && (*x3 == 11) && (*x4 == 10) && (*x5 == 1) &&
            (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5) =>
            Some((10,[*x1,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == 13) && (*x2 == 12) && (*x3 == 11) && (*x4 == 10) && (*x5 == 1) &&
            (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5) =>
            Some((10,[*x1,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        _=> None,
    }
}

fn find_straight_flush(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
    match &*hand{
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if ((*x1 - *x2) == 1) && ((*x2 - *x3) == 1) && ((*x3 - *x4) == 1) && ((*x4 - *x5) == 1) && 
            (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5) =>
            Some((9,[*x1,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        _ => None,
    }
}
        
fn find_four_of_a_kind(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
    match &*hand{
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x2 == 1) && (*x3 == 1) && (*x4 == 1) && (*x5 == 1) =>
            Some(8,[14], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x1 == *x4) =>
            Some(8,[*x2,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x2 == *x3) && (*x2 == *x4) && (*x2 == *x5) =>
            Some(8,[*x2,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        _ => None,
    }
}
    
fn find_full_house(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
    match &*hand{
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == 1) && (*x4 == 1) && (*x5 == 1) =>
            Some(7,[14,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == 1) && (*x4 == 1) && (*x5 == 1) =>
            Some(7,[14,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == 1) && (*x4 == 1) && (*x5 == 1) =>
            Some(7,[14,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == 1) && (*x5 == 1) =>
            Some(7,[*x1,14], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == 1) && (*x5 == 1) =>
            Some(7,[*x1,14], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == 1) && (*x5 == 1) =>
            Some(7,[*x1,14], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == *x5) =>
            Some(7,[*x1,*x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == *x4) && (*x3 == *x5) =>
            Some(7,[*x3,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == *x5) =>
            Some(7,[*x1,*x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == *x4) && (*x3 == *x5) =>
            Some(7,[*x3,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == *x5) =>
            Some(7,[*x1,*x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == *x4) && (*x3 == *x5) =>
            Some(7,[*x3,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == *x5) =>
            Some(7,[*x1,*x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == *x4) && (*x3 == *x5) =>
            Some(7,[*x3,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x1 == *x3) && (*x4 == *x5) =>
            Some(7,[*x1,*x4], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (*x1 == *x2) && (*x3 == *x4) && (*x3 == *x5) =>
            Some(7,[*x3,*x1], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
        _ => None,
    }
}

fn find_flush(hand: Vec<(u32, String)>) -> Option<(u32, [u32; 2], Vec<(u32, String)>)> {
    match &*hand{
        [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5)
                Some(6,[*x1,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                    (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),
       [..,(x1, s1), (x2, s2), (x3, s3), (x4, s4), (x5, s5)]
            if (s1 == s2) && (s1 == s3) && (s1 == s4) && (s1 == s5)
                Some(6,[*x1,0], vec![(*x5, String::from(s5)), (*x4, String::from(s4)),
                    (*x3, String::from(s3)), (*x2, String::from(s2)), (*x1, String::from(s1))])),),
        _ => None,
    }
}

        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        

 
 
        
