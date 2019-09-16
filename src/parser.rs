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
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::{delimited, preceded, terminated, tuple};
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
    Operation(Box<Operation>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Factor(Factor),
    Builtin(Builtin),
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
pub enum Command {
    Declaration((String, Operation)),
}

pub fn variable_name(input: &str) -> IResult<&str, String, Error<&str>> {
    map(alpha1, |x: &str| x.to_string())(input)
}

pub fn assignment(input: &str) -> IResult<&str, (String, Operation), Error<&str>> {
    map(
        tuple((variable_name, space0, char(':'), space0, expr)),
        |(variable_name, _, _, _, value)| (variable_name, value),
    )(input)
}

pub fn builtin(input: &str) -> IResult<&str, Atom, Error<&str>> {
    map(one_of("+-*/"), |op| match op {
        '+' => Atom::Builtin(Builtin::Plus),
        '-' => Atom::Builtin(Builtin::Minus),
        '*' => Atom::Builtin(Builtin::Mult),
        '/' => Atom::Builtin(Builtin::Div),
        _ => unreachable!(),
    })(input)
}

pub fn factor(input: &str) -> IResult<&str, Factor, Error<&str>> {
    map(
        tuple((
            space0,
            alt((
                delimited(
                    tag("("),
                    map(expr, |operation: Operation| {
                        Factor::Operation(Box::new(operation))
                    }),
                    tag(")"),
                ),
                map(alpha1, |variable: &str| {
                    Factor::Variable(variable.to_string())
                }),
                map(float, |number: f32| Factor::Number(number)),
            )),
            space0,
        )),
        |(_, fac, _)| fac,
    )(input)
}

pub fn atom(input: &str) -> IResult<&str, Atom, Error<&str>> {
    alt((builtin, map(factor, |f| Atom::Factor(f))))(input)
}

pub fn has_higher_precedence(first: &Builtin, second: &Builtin) -> bool {
    match first {
        Builtin::Mult | Builtin::Div => match second {
            Builtin::Plus => true,
            Builtin::Minus => true,
            _ => false,
        },
        Builtin::Plus | Builtin::Minus => false,
    }
}

pub fn expr(input: &str) -> IResult<&str, Operation, Error<&str>> {
    let (input, atoms) = many0(atom)(input)?;
    let mut factors: Vec<Operation> = vec![];
    let mut operators: Vec<Builtin> = vec![];

    for atom in atoms.clone().iter() {
        match atom {
            Atom::Factor(factor) => {
                factors.push(match factor {
                    Factor::Operation(operation) => *operation.clone(),
                    _ => Operation::Identity(factor.clone()),
                });
            }
            Atom::Builtin(operator) => {
                if operators.len() > 0 {
                    if has_higher_precedence(&operators.last().unwrap(), &operator) {
                        let op = operators.pop().unwrap();
                        let second = factors.pop().unwrap();
                        let first = factors.pop().unwrap();
                        factors.push(match op {
                            Builtin::Plus => first + second,
                            Builtin::Minus => first - second,
                            Builtin::Div => first / second,
                            Builtin::Mult => first * second,
                        })
                    }
                }
                operators.push(operator.clone());
            }
        }
    }

    while operators.len() > 0 {
        let op = operators.pop().unwrap();
        let second = factors.pop().unwrap();
        let first = factors.pop().unwrap();
        factors.push(match op {
            Builtin::Plus => first + second,
            Builtin::Minus => first - second,
            Builtin::Div => first / second,
            Builtin::Mult => first * second,
        })
    }

    // https://stackoverflow.com/questions/28256/equation-expression-parser-with-precedence#47717
    Ok((input, factors.first().unwrap().clone()))
}

pub fn parser(input: &str) -> IResult<&str, Vec<Command>, Error<&str>> {
    many0(terminated(
        preceded(
            multispace0,
            map(assignment, |(name, operation): (String, Operation)| {
                Command::Declaration((name, operation))
            }),
        ),
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
                Operation::Identity(Factor::Variable("y".to_string()))
                    + (Operation::Identity(Factor::Number(2.0))
                        + Operation::Identity(Factor::Variable("x".to_string())))
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
                Operation::Identity(Factor::Variable("y".to_string()))
                    + ((Operation::Identity(Factor::Number(2.0))
                        * Operation::Identity(Factor::Variable("x".to_string())))
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
}
