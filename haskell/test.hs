{-# OPTIONS_GHC -Wno-unrecognised-pragmas #-}

import Data.Char (toUpper)

{-# HLINT ignore "Use sum" #-}

main = interact $ unlines . filter ((< 10) . length) . lines

doubleNumber :: (Ord a, Num a) => a -> a
doubleNumber x
    | x <= 100 = 2 * x
    | otherwise = x

removeNonUppercase :: [Char] -> [Char]
removeNonUppercase st = [c | c <- st, c `elem` ['A' .. 'Z']]

testWhere :: (RealFloat a) => a -> a -> String
testWhere noice nice
    | cool < 10 = "You're weird"
    | cool >= 10 = "woohoo you're not weird"
  where
    cool = noice / nice

-- This language is crazy like WTF this shit is so elegant
quicksort :: (Ord a) => [a] -> [a]
quicksort [] = []
quicksort (x : xs) =
    let smallerSorted = quicksort $ filter (< x) xs; biggerSorted = quicksort $ filter (>= x) xs
     in smallerSorted ++ [x] ++ biggerSorted

isGreaterThanTen :: (Num a, Ord a) => a -> Bool
isGreaterThanTen x = x >= 10

betterSum :: (Num a) => [a] -> a
betterSum = foldl (+) 0 -- this is equivilant to sum

testListComprehension :: (Num a, Ord a) => [a] -> [a]
testListComprehension xs = [x ^ 2 | x <- xs, x <= 10000]

descSort :: (Ord a) => [a] -> [a]
descSort = reverse . quicksort

testHaskell :: Show a => a -> IO ()
testHaskell = print

data WeirdType where
    Shape :: String -> WeirdType
