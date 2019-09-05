// *****parser*****
// i need to convert variables in f32 here!
// MVC has just to do DrawShapeWf32(shape, val1, val2) where val1 and val2 are f32!
// we need an Hashmap inside the parser because we need to keep track of all variables in the context.
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0, space0};
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::{fold_many0, many0};
use nom::number::complete::float;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

// it recognizes a variable name like "x", "y", "xy", "myVariablE"
fn variable_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    map(alpha1, |x: &str| x)(input)
}

// it recognizes pattern **x: expr**
fn declare_variable_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((variable_parser, tag(":"), space0, expr)),
        |(name, _, _, value)| Command::DeclareVariable((name.to_string(), value)),
    )(input)
}

// it recognizes pattern **box**
fn declare_box(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(tag("box"), |shape: &str| {
        Command::DrawShape(shape.to_string())
    })(input)
}

fn declare_box_with_f32_var_or_var_f32_or_var_var_or_f32_f32(
    input: &str,
) -> IResult<&str, Command, VerboseError<&str>> {
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
            preceded(
                multispace0,
                declare_box_with_f32_var_or_var_f32_or_var_var_or_f32_f32,
            ),
            preceded(multispace0, declare_variable_parser),
            preceded(multispace0, declare_box),
        )),
        multispace0,
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    // x: f32
    DeclareVariable((String, f32)),
    // box | circle
    DrawShape(String),
    // box var var
    DrawShapeWf32((String, f32, f32)),
}
