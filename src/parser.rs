// *****parser*****
// it recognizes *instructions* like "box x y"
// i need to convert variables in f32, if the variables were previously declared
// **IMPORTANT** MVC has just to do DrawShapeWf32(shape, val1, val2) where val1 and val2 are f32!
// i need an HashMap inside the parser because we need to keep track of all variables in the context.
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0, space0};
use nom::combinator::map;
use nom::error::VerboseError;
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

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f32),
    Variable(String),
}

fn number(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(float, |x| Token::Number(x))(input)
}

// it recognizes a variable name like "x", "y", "xy", "myVariablE"
fn variable(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    map(alpha1, |x: &str| x.to_string())(input)
}

// it recognizes pattern **x: expr**
fn variable_declaration(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((variable, space0, tag(":"), space0, expr)),
        |(name, _, _, _, value)| Command::DeclareVariable((name, value)),
    )(input)
}

// it recognizes pattern **box**
fn declare_box(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(tag("box"), |shape: &str| {
        Command::DrawShape(shape.to_string())
    })(input)
}

// *****box variable*****
// So here if we find ******box variable***** we have to find the value of the variable on the HashMap and set the command
// Command::DrawShapeWf32((shape.to_string(), val1, val2))
fn declare_cmp_box(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
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
fn parens(i: &str) -> IResult<&str, f32, VerboseError<&str>> {
    delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
}

fn factor(i: &str) -> IResult<&str, f32, VerboseError<&str>> {
    alt((map(delimited(space0, float, space0), |x| x), parens))(i)
}

fn term(i: &str) -> IResult<&str, f32, VerboseError<&str>> {
    let (i, init) = factor(i)?;
    fold_many0(
        pair(alt((char('*'), char('/'))), factor),
        init,
        |acc, (op, val): (char, f32)| {
            if op == '*' {
                acc * val
            } else {
                acc / val
            }
        },
    )(i)
}

pub fn expr(i: &str) -> IResult<&str, f32, VerboseError<&str>> {
    let (i, init) = term(i)?;
    fold_many0(
        pair(alt((char('+'), char('-'))), term),
        init,
        |acc, (op, val): (char, f32)| {
            if op == '+' {
                acc + val
            } else {
                acc - val
            }
        },
    )(i)
}

pub fn parser(input: &str) -> IResult<&str, Vec<Command>, VerboseError<&str>> {
    many0(terminated(
        alt((
            preceded(multispace0, declare_cmp_box),
            preceded(multispace0, variable_declaration),
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
