main = do
  print (doubleNumber 105)
  print (removeNonUppercase "hello World")
  print (testWhere 2000 200)

doubleNumber x =
  if x > 100
    then x
    else x * 2

removeNonUppercase :: [Char] -> [Char]
removeNonUppercase st = [c | c <- st, c `elem` ['A' .. 'Z']]

testWhere :: (RealFloat a) => a -> a -> String
testWhere noice nice
  | cool < 10 = "You're weird"
  | cool >= 10 = "woohoo you're not weird"
  where
    cool = noice / nice
