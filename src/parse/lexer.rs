use std::borrow::Borrow;

use util::{Marked, Mark};
use super::{intern, parser_panic};
use super::token::Token;
use std::io;

fn mark_for<R: io::Read>(lexer: &mut Lexer<R>) -> Mark {
    let ::rustlex::rt::RustLexPos { off, .. } = lexer._input.tok;
    Mark::new(off, off + lexer.yystr().len())
}

fn mark<R: io::Read>(tok: Token, lexer: &mut Lexer<R>) -> Option<Marked<Token>> {
    Some(Marked::new(tok, mark_for(lexer)))
}

macro_rules! some { ($x:expr) => { |lexer: &mut Lexer<R>| -> Option<Marked<Token>> { mark($x, lexer) } } }
macro_rules! none { () => { |_: &mut Lexer<R>| -> Option<Marked<Token>> { None } } }

pub type MarkedToken = Marked<Token>;

rustlex! Lexer {
    token MarkedToken;
    property comment_depth:usize = 0;

    let WHITESPACE = [' ' '\n' '\t' '\r' '\x09' '\x0A' '\x0B' '\x0C' '\x0D'];
    let ID = ['A'-'Z''a'-'z''_']['A'-'Z''a'-'z''0'-'9''_']*;
    let DECNUM = '0' | ['1'-'9']['0'-'9']*;
    let HEXNUM = '0'["xX"]['0'-'9''a'-'f''A'-'F']+;

    INITIAL {
        WHITESPACE => none!(),

        ID => |lexer: &mut Lexer<R>| {
            mark(Token::Ident(intern(lexer.yystr().borrow())), lexer)
        },

        '(' => some!(Token::Lparen),
        ')' => some!(Token::Rparen),
        '{' => some!(Token::Lbrace),
        '}' => some!(Token::Rbrace),

        ';' => some!(Token::Semi),
        '=' => some!(Token::Assign),
        "+=" => some!(Token::Pluseq),
        "-=" => some!(Token::Minuseq),
        "*=" => some!(Token::Stareq),
        "/=" => some!(Token::Slasheq),
        "%=" => some!(Token::Percenteq),

        '+' => some!(Token::Plus),
        '-' => some!(Token::Minus),
        '*' => some!(Token::Star),
        '/' => some!(Token::Slash),
        '%' => some!(Token::Percent),

        "--" => some!(Token::Decrement),

        "struct" => some!(Token::Struct),
        "typedef" => some!(Token::Typedef),
        "if" => some!(Token::If),
        "else" => some!(Token::Else),
        "while" => some!(Token::While),
        "for" => some!(Token::For),
        "continue" => some!(Token::Continue),
        "break" => some!(Token::Break),
        "assert" => some!(Token::Assert),
        "true" => some!(Token::True),
        "false" => some!(Token::False),
        "NULL" => some!(Token::Null),
        "alloc" => some!(Token::Alloc),
        "alloc_array" => some!(Token::Allocarray),
        "bool" => some!(Token::Bool),
        "void" => some!(Token::Void),
        "char" => some!(Token::Char),
        "string" => some!(Token::String),
        "return" => some!(Token::Return),
        "int" => some!(Token::Int),
        "main" => some!(Token::Main),

        DECNUM => |lexer: &mut Lexer<R>| {
            let n = lexer.yystr()[..].parse().unwrap();

            if n > 2u32.pow(31) {
                parser_panic(format!("Constant {} is too large", n),
                             mark_for(lexer));
            }

            mark(Token::Intconst(n), lexer)
        },

        HEXNUM => |lexer: &mut Lexer<R>| {
            let s = lexer.yystr();
            let i:u32 = u32::from_str_radix(&s[2..], 16).unwrap();
            mark(Token::Intconst(i), lexer)
        },

        "/*" => |lexer: &mut Lexer<R>| -> Option<Marked<Token>> {
            lexer.comment_depth += 1;
            lexer.COMMENT();
            None
        },

        "//" [^'\n']* => none!(),
    }

    COMMENT {
        . => none!(),

        "*/" => |lexer: &mut Lexer<R>| -> Option<Marked<Token>> {
            lexer.comment_depth -= 1;
            if lexer.comment_depth == 0 {
                lexer.INITIAL();
            }
            None
        },

        "/*" => |lexer: &mut Lexer<R>| -> Option<Marked<Token>> {
            lexer.comment_depth += 1;
            None
        },
    }
}
