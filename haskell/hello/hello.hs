module Main where

main :: IO ()
main = do putStrLn ("Enter the first number: ")
          x <- getLine
          putStrLn ("Enter the second number: ")
          y <- getLine
          putStrLn . show $ read x + read y
