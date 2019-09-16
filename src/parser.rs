// *****parser*****
// it recognizes *instructions* like "box x y"
// i need to convert variables in f32, if the variables were previously declared
// **IMPORTANT** MVC has just to do DrawShapeWf32(shape, val1, val2) where val1 and val2 are f32!
// i need an HashMap inside the parser because we need to keep track of all variables in the context.
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0, one_of, space0};
use nom::combinator::map;
use nom::error::VerboseError as Error;
use nom::multi::{fold_many0, many0};
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
    Square((f32, f32)),
    Circle(f32),
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

pub fn node_initialization(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(alt((tag("square"), tag("box"))), |node| match node {
        "square" => Command::Instantiation(Node::Square((1.0, 1.0))),
        "circle" => Command::Instantiation(Node::Circle(1.0)),
        _ => unreachable!(),
    })(input)
}

pub fn parser(input: &str) -> IResult<&str, Vec<Command>, Error<&str>> {
    many0(terminated(
        preceded(multispace0, alt((assignment, node_initialization))),
        multispace0,
    ))(input)
}
