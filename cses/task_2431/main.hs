main :: IO()
main = do
    _ <- getLine
    contents <- lines <$> getContents
    putStr $ unlines (map digitQueries contents)
    where
        digitQueries i = [digits !! (read i - 1::Int)]
        digits = foldr ((++) . show) "" [1..]
