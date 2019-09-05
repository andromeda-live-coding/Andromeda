// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, multispace0, one_of, space0};
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::{fold_many0, many0};
use nom::number::complete::float;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

// ok
// it recognizes a variable name like "x", "y", "xy", "myVariablE"
fn variable_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    map(alpha1, |x: &str| x)(input)
}

// ok
// it recognizes pattern **x: expr**
fn declare_variable_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((variable_parser, tag(":"), space0, expr)),
        |(name, _, _, value)| Command::DeclareVariable((name.to_string(), value)),
    )(input)
}

// ok
fn declare_box(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(tag("box"), |shape: &str| {
        Command::DrawShape(shape.to_string())
    })(input)
}

/*fn declare_circle(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(tag("circle"), |shape: &str| {
        Command::DrawShape(shape.to_string())
    })(input)
}*/

//it recognizes pattern **box alpha** (where alpha is a variable)
fn declare_box_with_variable_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((alt((tag("box"), tag("circle"))), space0, variable_parser)),
        |(x, _, value)| Command::DrawShapeWVariable((x.to_string(), value.to_string())),
    )(input)
}

// ok
// it recognizes pattern **box expr**
// it not recognizes pattern **box variable**
fn declare_box_f32_parser(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((tag("box"), space0, expr)),
        |(x, _, value): (&str, _, f32)| Command::DrawShapeWf32f32((x.to_string(), value, value)),
    )(input)
}

fn declare_box_f32_f32(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((tag("box"), space0, float, space0, float)),
        |(shape, _, val1, _, val2): (&str, _, _, _, _)| {
            Command::DrawShapeWf32f32((shape.to_string(), val1, val2))
        },
    )(input)
}

/*fn declare_box_with_2variables(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    map(
        tuple((
            tag("box"),
            space0,
            variable_parser,
            space0,
            variable_parser,
        )),
        |(shape, _, var1, _, var2): (&str, _, &str, _, &str)| {
            Command::DrawShape2Variables((shape.to_string(), var1.to_string(), var2.to_string()))
        },
    )(input)
}*/

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

// connecting all simple parsers
pub fn parser(input: &str) -> IResult<&str, Vec<Command>, VerboseError<&str>> {
    many0(terminated(
        alt((
            preceded(multispace0, declare_box_f32_parser),
            preceded(multispace0, for_parser),
            preceded(multispace0, declare_box_with_f32_var),
            preceded(multispace0, declare_box_with_var_f32),
            preceded(multispace0, declare_variable_parser),
            preceded(multispace0, declare_box_f32_f32),
            preceded(multispace0, declare_box_with_variable_parser),
            preceded(multispace0, declare_box),
            preceded(multispace0, move_parser),
            //preceded(multispace0, declare_box_with_2variables),
            preceded(multispace0, reset_move_parser),
            preceded(multispace0, color_parser),
        )),
        multispace0,
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Expr(f32),
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

// it recognizes a variable
fn variab_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    map(alpha1, |x: &str| x)(input)
}

fn builtin(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    map(one_of("+-*/"), |x| x.to_string())(input)
}
