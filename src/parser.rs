// *****parser*****
// it recognizes *instructions* like "box x y"
// i need to convert variables in f32, if the variables were previously declared
// **IMPORTANT** MVC has just to do DrawShapeWf32(shape, val1, val2) where val1 and val2 are f32!
// i need an HashMap inside the parser because we need to keep track of all variables in the context.
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0, space0};
use nom::combinator::map;
use nom::error::VerboseError as Error;
use nom::multi::{fold_many0, many0};
use nom::number::complete::float;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    VariableName(String),
    // x: f32
    DeclareVariable((String, f32)),
    // box | circle
    DrawShape(String),
    // box var var
    DrawShapeWf32((String, f32, f32)),
}

#[derive(Debug, PartialEq)]
enum Token {
    Assignment((String, Vec<Operation>)),
    VariableName(String),
    Number(f32),
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Variable(String),
    Number(f32),
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Identity(Value),
    Plus((Value, Value)),
    Minus((Value, Value)),
    Mult((Value, Value)),
    Div((Value, Value)),
}

#[derive(Debug)]
struct Block {
    tokens: Vec<Token>,
}

impl Block {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

fn number(input: &str) -> IResult<&str, Token, Error<&str>> {
    map(float, |x| Token::Number(x))(input)
}

// it recognizes a variable name like "x", "y", "xy", "myVariablE"
fn variable_name(input: &str) -> IResult<&str, String, Error<&str>> {
    map(alpha1, |x: &str| x.to_string())(input)
}
// it recognizes pattern **x: expr**
fn assignment(input: &str) -> IResult<&str, (String, Vec<Operation>), Error<&str>> {
    map(
        tuple((variable_name, space0, char(':'), space0, expr)),
        |(variable_name, _, _, _, value)| (variable_name, value),
    )(input)
}

fn token(input: &str) -> IResult<&str, Token, Error<&str>> {
    alt((
        map(assignment, |(name, value)| Token::Assignment((name, value))),
        map(variable_name, |name| Token::VariableName(name)),
    ))(input)
}

fn tokens(input: &str) -> IResult<&str, Vec<Token>, Error<&str>> {
    many0(map(tuple((space0, token, space0)), |(_, token, _)| token))(input)
}

fn block(input: &str) -> IResult<&str, Block, Error<&str>> {
    map(tuple((tokens, tag("\n"))), |(tokens, _)| Block::new(tokens))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Block>, Error<&str>> {
    many0(block)(input)
}

fn value(input: &str) -> IResult<&str, Value, Error<&str>> {
    alt((
        map(alpha1, |variable: &str| {
            Value::Variable(variable.to_string())
        }),
        map(float, |number: f32| Value::Number(number)),
    ))(input)
}

// it recognizes pattern **box**
fn declare_box(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(tag("box"), |shape: &str| {
        Command::DrawShape(shape.to_string())
    })(input)
}

// *****box variable*****
// So here if we find ******box variable***** we have to find the value of the variable on the HashMap and set the command
// Command::DrawShapeWf32((shape.to_string(), val1, val2))
fn declare_cmp_box(input: &str) -> IResult<&str, Command, Error<&str>> {
    alt((
        map(
            tuple((tag("box"), space0, expr, space0, expr)),
            |(shape, _, val1, _, val2): (&str, _, _, _, _)| {
                Command::DrawShapeWf32((shape.to_string(), val1, val2))
            },
        ),
        map(
            tuple((tag("box"), space0, expr)),
            |(x, _, value): (&str, _, f32)| Command::DrawShapeWf32((x.to_string(), value, value)),
        ),
    ))(input)
}

// We parse any expr surrounded by parens, ignoring all whitespaces around those
// fn parens(i: &str) -> IResult<&str, f32, Error<&str>> {
//     delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
// }

fn factor(input: &str) -> IResult<&str, Value, Error<&str>> {
    map(tuple((space0, value, space0)), |(_, value, _)| value)(input)
}

fn term(input: &str) -> IResult<&str, Vec<Operation>, Error<&str>> {
    let (first_input, first) = factor(input)?;
    let (input, more) = many0(map(
        tuple((factor, alt((char('*'), char('/'))), factor)),
        |(first, op, second)| {
            if op == '*' {
                Operation::Mult((first, second))
            } else {
                Operation::Div((first, second))
            }
        },
    ))(input)?;
    if more.len() == 0 {
        Ok((first_input, vec![Operation::Identity(first)]))
    } else {
        Ok((input, more))
    }
}

fn expr(input: &str) -> IResult<&str, Vec<Operation>, Error<&str>> {
    let (input, mut operations) = term(input)?;
    let (input, mut more) = many0(map(
        tuple((factor, alt((char('+'), char('-'))), factor)),
        |(first, op, second)| {
            if op == '+' {
                Operation::Plus((first, second))
            } else {
                Operation::Minus((first, second))
            }
        },
    ))(input)?;
    let mut res = vec![];
    res.append(&mut operations);
    res.append(&mut more);
    Ok((input, res))
}

pub fn parser(input: &str) -> IResult<&str, Vec<Command>, Error<&str>> {
    many0(terminated(
        alt((
            preceded(multispace0, declare_cmp_box),
            preceded(multispace0, assignment),
            preceded(multispace0, declare_box),
        )),
        multispace0,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declare_variable() {
        let mut variables: HashMap<String, f32> = HashMap::new();
        let (_, commands) = parser("x: 2").unwrap();
        for command in commands {
            if let Command::DeclareVariable((name, value)) = command {
                variables.insert(name, value);
            }
        }
        assert_eq!(variables.get("x"), Some(&2.0));
    }
}
