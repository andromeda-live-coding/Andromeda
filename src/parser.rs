// IT'S LIKE IT VISIT ast 2 TIMES ???????????????????????????????????????????????

// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0, one_of, space0};
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::{preceded, tuple};
use nom::IResult;

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

// it recognizes pattern **move float float**
fn move_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((tag("move"), space0, float, space0, float)),
        |(_, _, val1, _, val2)| Command::Move((val1, val2)),
    )(input)
}

// it recognizes pattern **color f32 f32 f32**
fn color_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((tag("color"), space0, float, space0, float, space0, float)),
        |(_, _, r, _, g, _, b)| Command::Color((r, g, b)),
    )(input)
}

// connecting all simple parsers
pub fn parser(input: &str) -> IResult<&str, Vec<Command>, VerboseError<&str>> {
    many0(alt((
        preceded(multispace0, declare_variable_parser),
        preceded(multispace0, declare_box_f32_parser),
        preceded(multispace0, declare_box_with_variable_parser),
        preceded(multispace0, declare_box),
        preceded(multispace0, move_parser),
        preceded(multispace0, color_parser),
    )))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    // box | circle
    DrawShape(String),
    // box x | circle y
    DrawShapeWVariable((String, String)),
    // x: f32
    DeclareVariable((String, f32)),
    // move f32 f32
    Move((f32, f32)),
    DrawShapeWf32((String, f32)),
    Color((f32, f32, f32)),
}

fn _check_syntax(content: &str) -> bool {
    match parser(content) {
        Ok(not_parsed) => {
            if not_parsed.0 == "" {
                true
            } else {
                false
            }
        }
        Err(_) => (false),
    }
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
