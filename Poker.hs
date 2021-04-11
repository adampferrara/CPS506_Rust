module Poker where
	import Data.Maybe	-- for detecting when a certain pattern is not found
	import Data.List	-- for reverse sort function
	
	-- Helper function to convert list of Ints into tuples for ease of use
	convert i | i <= 13 = (i, "C")
			  | i > 13 && i <= 26 = (i - 13, "D")
			  | i > 26 && i <= 39 = (i - 26, "H")
			  | otherwise = (i - 39, "S")
	
	-- Helper functions to get values of a triple tuple
	getFst :: (a, b, c) -> a
	getFst (a, _, _) = a
	
	getSnd :: (a, b, c) -> b
	getSnd (_, b, _) = b
	
	getThd :: (a, b, c) -> c
	getThd (_, _, c) = c
	
	deal :: [ Int ] -> [ String ]
	deal [ c1, c2, c3, c4, c5, c6, c7, c8, c9 ] = [ win1, win2, win3, win4, win5 ] where
		-- Convert given integers into Cards for comparison purposes,
		-- divide cards into first player (a)'s hand, second player (b)'s hand, shared (s)
		a1 = convert c1
		b1 = convert c2
		a2 = convert c3
		b2 = convert c4
		s1 = convert c5
		s2 = convert c6
		s3 = convert c7
		s4 = convert c8
		s5 = convert c9
		
		-- Sort hands descending to make it easier to find the highest values
		-- The "find" helper functions will return hands in an ascending order
		handA = reverse (sort [ a1, a2, s1, s2, s3, s4, s5 ])
		handB = reverse (sort [ b1, b2, s1, s2, s3, s4, s5 ])
		
		-- Decide which hand is best using the best_hand helper function
		-- If hands are equal, break tie with highest card value
		-- Uses custom get functions to get values from a triple tuple
		bestA = bestHand handA
		bestB = bestHand handB
		
		winner
			| getFst bestA <= getFst bestB = getThd bestB
			| getFst bestA >= getFst bestB = getThd bestA
			| getFst bestA == getFst bestB && getSnd bestA <= getSnd bestB = getThd bestB
			| getFst bestA == getFst bestB && getSnd bestA >= getSnd bestB = getThd bestA
			| otherwise = getThd bestA
		
		w1 = winner!!0
		w2 = winner!!1
		w3 = winner!!2
		w4 = winner!!3
		w5 = winner!!4
		win1 = show (fst w1) ++ snd w1
		win2 = show (fst w2) ++ snd w2
		win3 = show (fst w3) ++ snd w3
		win4 = show (fst w4) ++ snd w4
		win5 = show (fst w5) ++ snd w5
	
	-- Helper function to find the highest-value hands for each participant
	-- If none match, assign the hand the value of its highest card
	
	bestHand :: [ (Int, String) ] -> ( Int, [ Int ], [ (Int, String) ] )
	bestHand [ c1, c2, c3, c4, c5, c6, c7 ]
		| not (isNothing (findRoyalFlush [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findRoyalFlush [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findStraightFlush [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findStraightFlush [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findFourOfKind [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findFourOfKind [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findFullHouse [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findFullHouse [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findFlush[ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findFlush [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findStraight [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findStraight [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findThreeOfKind [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findThreeOfKind [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findTwoPair [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findTwoPair [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findPair [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findPair [ c1, c2, c3, c4, c5, c6, c7 ])
		| not (isNothing (findAceHigh [ c1, c2, c3, c4, c5, c6, c7 ]))
		= fromJust (findAceHigh [ c1, c2, c3, c4, c5, c6, c7 ])
		| otherwise = ( 0, [fst c1], [ c5, c4, c3, c2, c1 ] )
	
	{-	Helper functions to detect certain card combinations
		Maybe monad applied to output so it can detect when the match isn't found
		Last step is recursion to have it search through rest of hand
		Each helper function returns a triple tuple that contains:
		• An Int to represent what kind of hand it is
		• A list of Ints to represent the highest values for pairs and such
		• The hand itself, a list of Card tuples
		To ensure that the highest values are actually detected instead of the lowest,
		right-side list-checking patterns are put first
	
		This didn't initially work as I thought it would, so I added extra constructors
		to help it detect certain patterns. This could probably be done more elegantly.
		Hands can only have up to 7 cards, so there are not many permutations needed.
		even so this is ugly as sin and I wish I knew a better way to handle it			-}
	findRoyalFlush :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findRoyalFlush [] = Nothing
	findRoyalFlush ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == 13 && x2 == 12 && x3 == 11 && x4 == 10 && x5 == 1 &&
		  s1 == s2 && s1 == s3 && s1 == s4 && s1 == s5
		= Just ( 10, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findRoyalFlush ((x1, s1):(x2, s2):(x3, s3):(x4, s4):_:(x5, s5):_)
		| x1 == 13 && x2 == 12 && x3 == 11 && x4 == 10 && x5 == 1 &&
		  s1 == s2 && s1 == s3 && s1 == s4 && s1 == s5
		= Just ( 10, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findRoyalFlush ((x1, s1):(x2, s2):(x3, s3):(x4, s4):_:_:(x5, s5):_)
		| x1 == 13 && x2 == 12 && x3 == 11 && x4 == 10 && x5 == 1 &&
		  s1 == s2 && s1 == s3 && s1 == s4 && s1 == s5
		= Just ( 10, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findRoyalFlush (_:xs) = findRoyalFlush xs
	
	findStraightFlush :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findStraightFlush [] = Nothing
	findStraightFlush ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 + 1 && x2 == x3 + 1 && x3 == x4 + 1 && x4 == x5 + 1 &&
		  s1 == s2 && s1 == s3 && s1 == s4 && s1 == s5
		= Just ( 9, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findStraightFlush (_:xs) = findStraightFlush xs
	
	findFourOfKind :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findFourOfKind [] = Nothing
	findFourOfKind ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x1 == x3 && x1 == x4
		= Just ( 8, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x2 == x3 && x2 == x4 && x2 == x5
		= Just ( 8, [x2], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findFourOfKind (_:xs) = findFourOfKind xs
	
	findFullHouse :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findFullHouse [] = Nothing
	findFullHouse ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x1 == x3 && x4 == x5
		= Just ( 7, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x3 == x4 && x3 == x5
		= Just ( 7, [x3, x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findFullHouse ((x1, s1):(x2, s2):(x3, s3):_:(x4, s4):(x5, s5):_)
		| x1 == x2 && x1 == x3 && x4 == x5
		= Just ( 7, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x3 == x4 && x3 == x5
		= Just ( 7, [x3, x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findFullHouse ((x1, s1):(x2, s2):(x3, s3):_:_:(x4, s4):(x5, s5):_)
		| x1 == x2 && x1 == x3 && x4 == x5
		= Just ( 7, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x3 == x4 && x3 == x5
		= Just ( 7, [x3, x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findFullHouse ((x1, s1):(x2, s2):_:(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x1 == x3 && x4 == x5
		= Just ( 7, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x3 == x4 && x3 == x5
		= Just ( 7, [x3, x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findFullHouse ((x1, s1):(x2, s2):_:_:(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x1 == x3 && x4 == x5
		= Just ( 7, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x3 == x4 && x3 == x5
		= Just ( 7, [x3, x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findFullHouse (_:xs) = findFullHouse xs
	
	findFlush :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findFlush [] = Nothing
	findFlush ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| s1 == s2 && s1 == s3 && s1 == s4 && s1 == s5
		= Just ( 6, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findFlush (_:xs) = findFlush xs
	
	findStraight :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findStraight [] = Nothing
	findStraight ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 + 1 && x2 == x3 + 1 && x3 == x4 + 1 && x4 == x5 + 1
		= Just ( 5, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findStraight (_:xs) = findStraight xs
	
	findThreeOfKind :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findThreeOfKind [] = Nothing
	findThreeOfKind ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x1 == x3 = Just ( 4, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x2 == x3 && x2 == x4 = Just ( 4, [x2], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x3 == x4 && x3 == x5 = Just ( 4, [x3], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findThreeOfKind (_:xs) = findThreeOfKind xs
	
	findTwoPair :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findTwoPair [] = Nothing
	findTwoPair ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x3 == x4 = Just ( 3, [x1, x3], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x4 == x5 = Just ( 3, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x2 == x3 && x4 == x5 = Just ( 3, [x2, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findTwoPair ((x1, s1):(x2, s2):_:(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x3 == x4 = Just ( 3, [x1, x3], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x4 == x5 = Just ( 3, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x2 == x3 && x4 == x5 = Just ( 3, [x2, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findTwoPair ((x1, s1):(x2, s2):_:_:(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 && x3 == x4 = Just ( 3, [x1, x3], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x1 == x2 && x4 == x5 = Just ( 3, [x1, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x2 == x3 && x4 == x5 = Just ( 3, [x2, x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findTwoPair (_:xs) = findTwoPair xs
	
	findPair :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findPair [] = Nothing
	findPair ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(x5, s5):_)
		| x1 == x2 = Just ( 2, [x1], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x2 == x3 = Just ( 2, [x2], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x3 == x4 = Just ( 2, [x3], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
		| x4 == x5 = Just ( 2, [x4], ((x5, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findPair (_:xs) = findPair xs
		
	findAceHigh :: [ (Int, String) ] -> Maybe ( Int, [Int], [ (Int, String) ] )
	findAceHigh [] = Nothing
	findAceHigh ((x1, s1):(x2, s2):(x3, s3):(x4, s4):(1, s5):_)
		= Just ( 1, [1], ((1, s5):(x4, s4):(x3, s3):(x2, s2):(x1, s1):[]) )
	findAceHigh (_:xs) = findAceHigh xs