import           Custom

main = do
    print(fNum 1) -- 3

    print(fNums 1 2) -- 3

    print(fEq 1) -- 1

    print(fOrd 1) -- 1

    print(ff 1) -- True
    print(ff 0) -- False

    print(fff 1 2 3) -- -4

    print(ffm 0) -- 0
    print(ffm 3) -- 6

    print( ffw(1,2,3) )


    print(showEven 4)
    print(showBoolean True)




fNum :: Num a => a -> a
fNum a = a + 1


fNums :: Num a => a -> a -> a
fNums a b = a + b


fEq :: Eq a => a -> a
fEq a = a


fOrd :: Ord a => a -> a
fOrd a = a


ff :: Int -> Bool
ff 0 = False
ff a = True

fff :: Int -> Int -> Int -> Int -- fff :: a -> b -> c -> a-(b+c)
fff a b c = a-(b+c)


ffm :: Integer -> Integer
ffm n
    | n == 0 = 1 -- if (n == 0) { return 1;}
    | n /= 0 = n * ffm (n-1)


ffw :: (Float, Float, Float) -> (Float, Float, Float)
ffw (a,b,c) = (x1, x2, x3) where
    x1 = a * a
    x2 = b * b
    x3 = c * c


{-
-- ## Too many parameters for class
    -- first way

{-#
LANGUAGE MultiParamTypeClasses
, FunctionalDependencies
#-}


class Set a b | a -> b where
    exists :: a -> (b -> Bool) -> Bool
-}


{-
-- ## Too many parameters for class
    -- second way

class Set f where
    exists :: f a -> (a -> Bool) -> Bool
-}
