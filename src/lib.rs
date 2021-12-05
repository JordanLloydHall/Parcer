
extern crate nom;

use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::number::complete::float;
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Expr {
    EVal(f32),
    EAdd(Box<Expr>, Box<Expr>),
    ESub(Box<Expr>, Box<Expr>),
    EMul(Box<Expr>, Box<Expr>),
    EDiv(Box<Expr>, Box<Expr>)
}

use Expr::*;

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, float, space0), |f| EVal(f))(input)
}

fn parse_atomic(input: &str) -> IResult<&str, Expr> {
    alt(
        (delimited(
            space0,
            delimited(char('('), parse_expr, char(')')),
            space0,
        ),
        parse_number
        )
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_atomic(input)?;
    let (input, exprs) = many0(tuple((alt((char('*'), char('/'))), parse_atomic)))(input)?;
    Ok((input, exprs.into_iter().fold(left, |acc, v| parse_op(v, acc))))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, exprs.into_iter().fold(left, |acc, v| parse_op(v, acc))))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => EAdd(Box::new(expr1), Box::new(expr2)),
        '-' => ESub(Box::new(expr1), Box::new(expr2)),
        '*' => EMul(Box::new(expr1), Box::new(expr2)),
        '/' => EDiv(Box::new(expr1), Box::new(expr2)),
        _ => panic!("Unknown Operation"),
    }
}


#[cfg(test)]
mod tests {
    use crate::parse_expr;
    use crate::Expr::*;
    #[test]
    fn parse_val() {
        let result = parse_expr("5");
        assert_eq!(result, Ok(("", EVal(5f32))));
    }

    #[test]
    fn parse_add() {
        let result = parse_expr("5 + 5.0");
        assert_eq!(result, Ok(("", EAdd(
            Box::new(EVal(5f32)), Box::new(EVal(5f32))
        ))));
    }

    #[test]
    fn parse_nested() {
        let result = parse_expr("5.0 * 5 - 5.0");
        assert_eq!(result, Ok(("", ESub(
            Box::new(
                EMul(Box::new(EVal(5f32)), 
                Box::new(EVal(5f32))
            )), 
            Box::new(EVal(5f32))
        ))));
    }
}
