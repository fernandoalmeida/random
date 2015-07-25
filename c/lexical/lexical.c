/*
  A lexical analyzer for arithmetic expressions described by the gramar:

  <expr> -> <term> {(+|-)<term>}
  <term> -> <factor> {(*|/)<factor>}
  <factor> -> id | int_constant | ( <expr>)
 */

#include <stdlib.h>
#include <stdio.h>
#include <ctype.h>

/* Variables */
char lexeme [100];
char nextChar;
int charClass;
int lexLen;
int token;
int nextToken;
FILE *in_fp, *fopen();

/* Function */
void lex();
void getNonBlank();
void getChar();
void addChar();
void expr();
void term();
void factor();
void error();

/* Character classes */
#define LETTER 0
#define DIGIT 1
#define UNKNOWN 99

/* Tokens */
#define INT_LIT 10
#define IDENT 11
#define ASSIGN_OP 20
#define ADD_OP 21
#define SUB_OP 22
#define MULT_OP 23
#define DIV_OP 24
#define LEFT_PAREN 25
#define RIGHT_PAREN 26

int main() {
  printf(
	 "============================================\n"	\
	 "<expr> -> <term> {(+|-)<term>}\n"			\
	 "<term> -> <factor> {(*|/)<factor>}\n"			\
	 "<expr> -> id | int_constant | ( <expr>)\n"		\
	 "============================================\n\n"
	 );

  in_fp = fopen("expression.in", "r");

  if (in_fp == NULL) {
    printf("ERROR: Cannot open `expression.in` file");
  } else {
    getChar();

    do {
      lex();
      expr();
    } while (nextToken != EOF);
  }

  return 0;
}

void lex() {
  lexLen = 0;
  getNonBlank();

  switch (charClass) {
  case LETTER:
    addChar();
    getChar();

    while(charClass == LETTER || charClass == DIGIT) {
      addChar();
      getChar();
    }
    nextToken = IDENT;

    break;
  case DIGIT:
    addChar();
    getChar();
    while(charClass == DIGIT) {
      addChar();
      getChar();
    }
    nextToken = INT_LIT;

    break;
  case UNKNOWN:
    addChar();

    switch (nextChar) {
    case '(':
      nextToken = LEFT_PAREN;
      break;
    case ')':
      nextToken = RIGHT_PAREN;
      break;
    case '=':
      nextToken = ASSIGN_OP;
      break;
    case '+':
      nextToken = ADD_OP;
      break;
    case '-':
      nextToken = SUB_OP;
      break;
    case '*':
      nextToken = MULT_OP;
      break;
    case '/':
      nextToken = DIV_OP;
      break;
    default:
      nextToken = EOF;
      break;
    }

    getChar();

    break;
  case EOF:
    nextToken = EOF;
    lexeme[0] = 'E';
    lexeme[1] = 'O';
    lexeme[2] = 'F';
    lexeme[3] = 0;

    break;
  }

  printf("token: %d - lexeme: %s\n", nextToken, lexeme);
}

void getNonBlank() {
  while(isspace(nextChar))
    getChar();
}

void getChar() {
  nextChar = getc(in_fp);

  if (nextChar != EOF) {
    if (isalpha(nextChar))
      charClass = LETTER;
    else if (isdigit(nextChar))
      charClass = DIGIT;
    else
      charClass = UNKNOWN;
  } else {
    charClass = EOF;
  }
}

void addChar() {
  if (lexLen <= 98) {
    lexeme[lexLen++] = nextChar;
    lexeme[lexLen] = 0;
  } else {
    printf("ERROR: Lexeme is too long");
  }
}

/* Parses strings in the language generated by the rule:
   <expr> -> <term> {(+ | -) <term>}
 */
void expr() {
  printf("<expr>\n");
  term();
  while(nextToken == ADD_OP || nextToken == SUB_OP) {
    lex();
    term();
  }
  printf("</expr>\n");
}

/*
  Parses strings in the language generated by the rule:
  <term> -> <factor> {(* | /) <factor>}
 */
void term() {
  printf("<term>\n");
  factor();
  while(nextToken == MULT_OP || nextToken == DIV_OP) {
    lex();
    factor();
  }
  printf("</term>\n");
}

/*
  Parses strings in the language generated by the rule:
  <factor> -> id | int_constant | (<expr>)
 */
void factor() {
  printf("<factor>\n");
  if (nextToken == IDENT || nextToken == INT_LIT) {
    lex();
  } else {
    if (nextToken == LEFT_PAREN) {
      lex();
      expr();
      if (nextToken == RIGHT_PAREN) {
	lex();
      } else {
	error();
      }
    } else {
      error();
    }
  }
  printf("</factor>\n");
}

void error() {
  printf("ERROR: Invalid expression\n");
  exit(1);
}
