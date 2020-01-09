module Main where

main :: IO ()
main = do
  putStrLn ("Enter a name")
  name <- getLine
  putStrLn ("Hello, " ++ name)
