use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0, one_of, space0};
use nom::combinator::map;
use nom::error::VerboseError as Error;
use nom::multi::{fold_many0, many0, many_m_n};
use nom::number::complete::float;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone)]
pub enum Builtin {
    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Factor {
    Variable(String),
    Number(f32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Identity(Factor),
    Calculation((Box<Operation>, Builtin, Box<Operation>)),
}

impl Mul for Operation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Operation::Calculation((Box::new(self), Builtin::Mult, Box::new(rhs)))
    }
}

impl Div for Operation {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Operation::Calculation((Box::new(self), Builtin::Div, Box::new(rhs)))
    }
}

impl Add for Operation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Operation::Calculation((Box::new(self), Builtin::Plus, Box::new(rhs)))
    }
}

impl Sub for Operation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Operation::Calculation((Box::new(self), Builtin::Minus, Box::new(rhs)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Square((Operation, Operation)),
    Circle(Operation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Declaration((String, Operation)),
    Instantiation(Node),
}

pub fn mult(input: &str) -> IResult<&str, Builtin, Error<&str>> {
    map(one_of("*/"), |op| match op {
        '*' => Builtin::Mult,
        '/' => Builtin::Div,
        _ => unreachable!(),
    })(input)
}

pub fn sum(input: &str) -> IResult<&str, Builtin, Error<&str>> {
    map(one_of("+-"), |op| match op {
        '+' => Builtin::Plus,
        '-' => Builtin::Minus,
        _ => unreachable!(),
    })(input)
}

pub fn variable(input: &str) -> IResult<&str, Factor, Error<&str>> {
    map(alpha1, |name: &str| Factor::Variable(name.to_string()))(input)
}

pub fn number(input: &str) -> IResult<&str, Factor, Error<&str>> {
    map(float, |value: f32| Factor::Number(value))(input)
}

pub fn factor(input: &str) -> IResult<&str, Operation, Error<&str>> {
    map(
        tuple((
            space0,
            alt((
                delimited(tag("("), expr, tag(")")),
                map(variable, Operation::Identity),
                map(number, Operation::Identity),
            )),
            space0,
        )),
        |(_, fac, _)| fac,
    )(input)
}

fn term(i: &str) -> IResult<&str, Operation, Error<&str>> {
    let (i, init) = factor(i)?;

    fold_many0(
        pair(mult, factor),
        init,
        |acc, (op, val): (Builtin, Operation)| match op {
            Builtin::Mult => acc * val,
            Builtin::Div => acc / val,
            _ => unreachable!(),
        },
    )(i)
}

fn expr(i: &str) -> IResult<&str, Operation, Error<&str>> {
    let (i, init) = term(i)?;

    fold_many0(
        pair(sum, term),
        init,
        |acc, (op, val): (Builtin, Operation)| match op {
            Builtin::Plus => acc + val,
            Builtin::Minus => acc - val,
            _ => unreachable!(),
        },
    )(i)
}

pub fn variable_name(input: &str) -> IResult<&str, String, Error<&str>> {
    map(alpha1, |x: &str| x.to_string())(input)
}

pub fn assignment(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(
        tuple((variable_name, space0, char(':'), space0, expr)),
        |(variable_name, _, _, _, value)| Command::Declaration((variable_name, value)),
    )(input)
}

pub fn square(input: &str) -> IResult<&str, Node, Error<&str>> {
    map(
        pair(tag("square"), many_m_n(0, 2, expr)),
        |(_, params)| match (params.get(0), params.get(1)) {
            (Some(width), Some(height)) => Node::Square((width.clone(), height.clone())),
            (Some(size), None) => Node::Square((size.clone(), size.clone())),
            (None, None) => Node::Square((
                Operation::Identity(Factor::Number(1.0)),
                Operation::Identity(Factor::Number(1.0)),
            )),
            _ => unreachable!(),
        },
    )(input)
}

pub fn circle(input: &str) -> IResult<&str, Node, Error<&str>> {
    map(
        pair(tag("circle"), many_m_n(0, 1, expr)),
        |(_, params)| match params.first() {
            Some(radius) => Node::Circle(radius.clone()),
            None => Node::Circle(Operation::Identity(Factor::Number(1.0))),
        },
    )(input)
}

pub fn draw_shape(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(alt((circle, square)), Command::Instantiation)(input)
}

pub fn parser(input: &str) -> IResult<&str, Vec<Command>, Error<&str>> {
    many0(terminated(
        preceded(multispace0, alt((draw_shape, assignment))),
        multispace0,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declare_variable() {
        let (rest, commands) = parser("x: 2").unwrap();
        assert_eq!(
            commands[0],
            Command::Declaration(("x".to_string(), Operation::Identity(Factor::Number(2.0))))
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn declare_variable_with_expression_only_sum_of_two_elements() {
        let expression = "z: y + 2.0\n";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Declaration((
                "z".to_string(),
                Operation::Identity(Factor::Variable("y".to_string()))
                    + Operation::Identity(Factor::Number(2.0))
            ))
        );
    }

    #[test]
    fn declare_variable_with_expression_only_sum() {
        let expression = "z: y + 2.0 + x\n";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Declaration((
                "z".to_string(),
                (Operation::Identity(Factor::Variable("y".to_string()))
                    + Operation::Identity(Factor::Number(2.0)))
                    + Operation::Identity(Factor::Variable("x".to_string()))
            ))
        );
    }

    #[test]
    fn declare_variable_with_expression() {
        let expression = "z: y + 2.0 * x + 3\n";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Declaration((
                "z".to_string(),
                (Operation::Identity(Factor::Variable("y".to_string()))
                    + (Operation::Identity(Factor::Number(2.0)))
                        * Operation::Identity(Factor::Variable("x".to_string()))
                    + Operation::Identity(Factor::Number(3.0)))
            ))
        );
    }

    #[test]
    fn declare_variable_with_expression_and_parenthesis() {
        let expression = "z: (y + 2.0) * x + 3";
        let (rest, ast) = parser(expression).unwrap();

        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Declaration((
                "z".to_string(),
                (Operation::Identity(Factor::Variable("y".to_string()))
                    + Operation::Identity(Factor::Number(2.0)))
                    * Operation::Identity(Factor::Variable("x".to_string()))
                    + Operation::Identity(Factor::Number(3.0))
            ))
        )
    }

    #[test]
    fn declare_variable_with_complicate_expression() {
        let expression = "z: (1 * (2.0 + 5 / (4 - 2))) ";
        let (rest, _ast) = parser(expression).unwrap();

        assert_eq!(rest, "");
        // TODO: Assert AST result
    }

    #[test]
    fn square_with_no_params() {
        let expression = "square";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Instantiation(Node::Square((
                Operation::Identity(Factor::Number(1.0)),
                Operation::Identity(Factor::Number(1.0))
            )))
        );
    }

    #[test]
    fn circle_with_no_params() {
        let expression = "circle";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Instantiation(Node::Circle(Operation::Identity(Factor::Number(1.0))))
        );
    }

    #[test]
    fn square_with_one_params() {
        let expression = "square 17.22";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Instantiation(Node::Square((
                Operation::Identity(Factor::Number(17.22)),
                Operation::Identity(Factor::Number(17.22))
            )))
        );
    }

    #[test]
    fn circle_with_one_params() {
        let expression = "circle 29.93";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Instantiation(Node::Circle(Operation::Identity(Factor::Number(29.93))))
        );
    }

    #[test]
    fn square_with_two_params() {
        let expression = "square 17.22 22.17";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[0],
            Command::Instantiation(Node::Square((
                Operation::Identity(Factor::Number(17.22)),
                Operation::Identity(Factor::Number(22.17))
            )))
        );
    }

    #[test]
    fn declaration_and_instantiation() {
        let expression = "x: 1\n square x x + 3";
        let (rest, ast) = parser(expression).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ast[1],
            Command::Instantiation(Node::Square((
                Operation::Identity(Factor::Variable("x".to_string())),
                Operation::Calculation((
                    Box::new(Operation::Identity(Factor::Variable("x".to_string()))),
                    Builtin::Plus,
                    Box::new(Operation::Identity(Factor::Number(3.0)))
                ))
            )))
        );
    }

    #[test]
    fn shapes() {
        let expression = "z: (1 * (2.0 + 5 / (4 - 2)))\n square x\nsquare x+(13.2) 9.2\n circle x+23.9\n circle z\n circle (12.93*(2+(9-7.6/129.92)))\n circle";
        let (rest, _ast) = parser(expression).unwrap();

        assert_eq!(rest, "");
    }
}
