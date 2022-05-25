module Custom (
   showEven,
   showBoolean
) where

showEven :: Int -> Bool
showEven x =
    if (x `div` 2) == 0
        then True
    else False



showBoolean :: Bool -> Int
showBoolean c =
    if c == True
        then 1
    else 0
