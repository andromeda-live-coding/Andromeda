// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace0, one_of, space0};
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

use nom::{
    character::complete::char, character::complete::space0 as space, multi::fold_many0,
    sequence::pair,
};

// it recognizes a variable name like "x", "y", "xy", "myVariablE"
fn variable_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    map(alpha1, |x: &str| x)(input)
}

// it recognizes pattern **x: f32**
fn declare_variable_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((variable_parser, one_of(":="), space0, float)),
        |(name, _, _, value)| Command::DeclareVariable((name.to_string(), value)),
    )(input)
}

fn declare_box(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(alt((tag("box"), tag("circle"))), |shape: &str| {
        Command::DrawShape(shape.to_string())
    })(input)
}

// it recognizes pattern **box alpha** (where alpha is a variable)
fn declare_box_with_variable_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((alt((tag("box"), tag("circle"))), space0, variable_parser)),
        |(x, _, value)| Command::DrawShapeWVariable((x.to_string(), value.to_string())),
    )(input)
}

fn declare_box_f32_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((alt((tag("box"), tag("circle"))), space0, float)),
        |(x, _, value): (&str, _, f32)| Command::DrawShapeWf32((x.to_string(), value)),
    )(input)
}

fn declare_box_f32_f32(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((
            alt((tag("box"), tag("circle"))),
            space0,
            float,
            space0,
            float,
        )),
        |(shape, _, val1, _, val2): (&str, _, _, _, _)| {
            Command::DrawShapeWf32f32((shape.to_string(), val1, val2))
        },
    )(input)
}

fn declare_box_with_2variables(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((
            alt((tag("box"), tag("circle"))),
            space0,
            variable_parser,
            space0,
            variable_parser,
        )),
        |(shape, _, var1, _, var2): (&str, _, &str, _, &str)| {
            Command::DrawShape2Variables((shape.to_string(), var1.to_string(), var2.to_string()))
        },
    )(input)
}

fn declare_box_with_var_f32(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((
            alt((tag("box"), tag("circle"))),
            space0,
            variable_parser,
            space0,
            float,
        )),
        |(shape, _, var1, _, val2): (&str, _, &str, _, f32)| {
            Command::DrawShapeVf32((shape.to_string(), var1.to_string(), val2))
        },
    )(input)
}

fn declare_box_with_f32_var(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((
            alt((tag("box"), tag("circle"))),
            space0,
            float,
            space0,
            variable_parser,
        )),
        |(shape, _, val1, _, var2): (&str, _, f32, _, &str)| {
            Command::DrawShapef32V((shape.to_string(), val1, var2.to_string()))
        },
    )(input)
}

// it recognizes pattern **move float float**
fn move_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((tag("move"), space0, float, space0, float)),
        |(_, _, val1, _, val2)| Command::Move((val1, val2)),
    )(input)
}

fn reset_move_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(tag("reset_m"), |_| Command::ResetMove)(input)
}

// it recognizes pattern **color f32 f32 f32**
fn color_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((tag("color"), space0, float, space0, float, space0, float)),
        |(_, _, r, _, g, _, b)| Command::Color((r, g, b)),
    )(input)
}

fn for_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((
            tag("for"),
            space0,
            digit1,
            space0,
            delimited(tag("("), parser, tag(")")),
        )),
        |(_, _, times, _, v): (_, _, &str, _, Vec<Command>)| Command::For((times.to_string(), v)),
    )(input)
}

// connecting all simple parsers
pub fn parser(input: &str) -> IResult<&str, Vec<Command>, VerboseError<&str>> {
    many0(terminated(
        alt((
            preceded(multispace0, for_parser),
            preceded(multispace0, declare_box_with_f32_var),
            preceded(multispace0, declare_box_with_var_f32),
            preceded(multispace0, declare_variable_parser),
            preceded(multispace0, declare_box_f32_f32),
            preceded(multispace0, declare_box_f32_parser),
            preceded(multispace0, declare_box_with_variable_parser),
            preceded(multispace0, declare_box),
            preceded(multispace0, move_parser),
            preceded(multispace0, declare_box_with_2variables),
            preceded(multispace0, reset_move_parser),
            preceded(multispace0, color_parser),
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
    // box x | circle y
    DrawShapeWVariable((String, String)),
    // box f32
    DrawShapeWf32((String, f32)),
    // box f32 f32
    DrawShapeWf32f32((String, f32, f32)),
    // box x y
    DrawShape2Variables((String, String, String)),

    DrawShapeVf32((String, String, f32)),
    DrawShapef32V((String, f32, String)),

    // move f32 f32
    Move((f32, f32)),
    ResetMove,
    // color f32 f32 f32
    Color((f32, f32, f32)),

    For((String, Vec<Command>)),
}

fn _check_syntax(content: &str) -> bool {
    match parser(content) {
        Ok(not_parsed) => not_parsed.0 == "",
        Err(_) => (false),
    }
}

// We parse any expr surrounded by parens, ignoring all whitespaces around those
fn parens(i: &str) -> IResult<&str, f32> {
    delimited(space, delimited(tag("("), expr, tag(")")), space)(i)
}

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
fn factor(i: &str) -> IResult<&str, f32> {
    alt((map(delimited(space, float, space), |x| x), parens))(i)
}

// We read an initial factor and for each time we find
// a * or / operator followed by another factor, we do
// the math by folding everything
fn term(i: &str) -> IResult<&str, f32> {
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
//

fn expr(i: &str) -> IResult<&str, f32> {
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

#[cfg(test)]
mod tests {
    use super::_check_syntax;
    #[test]
    fn test_valid() {
        let content = "x: 12.3\nbox x";
        assert_eq!(_check_syntax(content), true);
    }

    #[test]
    fn test_invalid_1() {
        let content = "asldkjasldkjal";
        assert_eq!(_check_syntax(content), false);
    }

    #[test]
    fn test_invalid_2() {
        let content = "x: 123.4\nbox x\n uncorrect syntax";
        assert_eq!(_check_syntax(content), false);
    }
}
