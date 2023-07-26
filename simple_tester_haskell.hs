module PokerTester where

    import Poker (deal)
    import Data.List (sort, intersect)
     
    perms = [   [ 9,  8,  7,  6,  5,  4,  3,  2,  1  ],  -- 1   2-6 Straight flush VS 1-5 straight flush
                [ 40, 41, 42, 43, 48, 49, 50, 51, 52 ],  -- 2   Royal flush VS straight flush
                [ 40, 41, 27, 28, 1,  14, 15, 42, 29 ],  -- 3   Four aces VS 2-full-of-A
                [ 30, 13, 27, 44, 12, 17, 33, 41, 43 ],  -- 4   3-fours VS 2-fours
                [ 27, 45, 3,  48, 44, 43, 41, 33, 12 ],  -- 5   Flush VS straight
                [ 17, 31, 30, 51, 44, 43, 41, 33, 12 ],  -- 6   3-fours VS 2-queens-2-fives
                [ 17, 39, 30, 52, 44, 25, 41, 51, 12 ],  -- 7   Q-full-of-K VS Q-full-of-4
                [ 11, 25, 9,  39, 50, 48, 3,  49, 45 ],  -- 8   9-K straight VS 9-J-two-pair
                [ 50, 26, 39, 3,  11, 27, 20, 48, 52 ],  -- 9   J-K-two-pair VS K-pair
                [ 40, 52, 46, 11, 48, 27, 29, 32, 37 ]  -- 10  A-pair VS J-pair
            ]
         
    sols  = [   [ "2C",  "3C",  "4C",  "5C",  "6C"  ],   -- 1   2-6 Straight flush
                [ "10S", "11S", "12S", "13S", "1S"  ],   -- 2   Royal flush
                [ "1C",  "1D",  "1H",  "1S"         ],   -- 3   Four aces
                [ "4D",  "4H",  "4S"                ],   -- 4   3-fours
                [ "2S",  "4S",  "5S",  "6S",  "9S"  ],   -- 5   Flush
                [ "4D",  "4H",  "4S"                ],   -- 6   3-fours
                [ "12C", "12D", "12S", "13H", "13S" ],   -- 7   Q-full-of-K
                [ "10S", "11S", "12D", "13H", "9S"  ],   -- 8   9-K straight
                [ "11C", "11S", "13H", "13S"        ],   -- 9   J-K-two-pair
                [ "1H",  "1S"                       ]   -- 10  A-pair
            ]

    testPerm n = do
        let input = perms !! n
        let shouldBe = sols !! n
        let youSaid = Poker.deal input
        let common = Data.List.intersect shouldBe youSaid
        if (length youSaid) > 5 
            then 0 
            else (fromIntegral $ length common) / (fromIntegral $ length shouldBe)
                
    testPerms 0 = []
    testPerms n = (testPerm $ n-1):testPerms(n-1)
    
    runTests = do
        let scores = testPerms (length perms)
        let pct = 100*(sum scores) / (fromIntegral $ length scores)
        let pct2 = (fromIntegral $ round (pct*10)) / 10.0
        let nPts = (fromIntegral $ round ((sum scores)*10)) / 10.0
        putStrLn $ show scores
        putStrLn $ show nPts ++ "/" ++ show (fromIntegral $ length scores) ++ " marks achieved"
        putStrLn $ show pct2 ++ "%"
        



    