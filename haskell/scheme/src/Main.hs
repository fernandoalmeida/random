module Main where

import Text.ParserCombinators.Parsec hiding (spaces)
import System.Environment
import Control.Monad

data LispVal = Atom String
             | List [LispVal]
             | DottedList [LispVal] LispVal
             | Number Integer
             | String String
             | Bool Bool

main :: IO ()
main = do
  (expr:_) <- getArgs
  putStrLn (readExpr expr)

readExpr :: String -> String
readExpr input = case parse parseExpr "lisp" input of
  Left err -> "No match: " ++ show err
  Right val -> "Found value"

parseExpr :: Parser LispVal
parseExpr =  parseAtom
         <|> parseString
         <|> parseNumber

parseAtom :: Parser LispVal
parseAtom = do
  first <- letter <|> symbol
  rest <- many (letter <|> digit <|> symbol)
  let atom = first:rest
  return $ case atom of
    "#t" -> Bool True
    "#f" -> Bool False
    _ -> Atom atom

parseString :: Parser LispVal
parseString = do
  char '"'
  s <- many (escapedChar <|> noneOf ['\\', '"'])
  char '"'
  return $ String s

escapedChar :: Parser Char
escapedChar = do char '\\'
                 c <- oneOf ['\\', '"', 'n', 'r', 't']
                 return $ case c of
                          '\\' -> c
                          '"' -> c
                          'n' -> '\n'
                          'r' -> '\r'
                          't' -> '\t'

parseNumber :: Parser LispVal
parseNumber = do ((many1 digit) >>= (\x -> return $ (Number . read) x))

spaces :: Parser ()
spaces = skipMany1 space

symbol :: Parser Char
symbol = oneOf "!@#$%&*_-+=^~<>:?/|"
