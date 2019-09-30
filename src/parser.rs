use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, multispace0, one_of, space0};
use nom::combinator::{map, opt};
use nom::error::VerboseError as Error;
use nom::multi::{fold_many0, many0, many_m_n};
use nom::number::complete::float;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Builtin {
    Plus,
    Minus,
    Mult,
    Div,
    Greater,
    Lesser,
    GreaterOrEqual,
    LesserOrEqual,
    Equal,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Factor {
    Variable(String),
    Number(f32),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Identity(Factor),
    Calculation((Box<Operation>, Builtin, Box<Operation>)),
    Condition((Box<Operation>, Builtin, Box<Operation>)),
    Nil,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Square((Operation, Operation)),
    Circle(Operation),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionalBuiltin {
    IfB,
    ElseIfB,
    ElseB,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Declaration((String, Operation)),
    Instantiation(Node),
    ConditionalBlock(Vec<(ConditionalBuiltin, Operation, Vec<Command>)>),
    For((i32, Vec<Command>)),
    Move((Operation, Operation)),
    ResetMove,
}

pub fn number(input: &str) -> IResult<&str, Factor, Error<&str>> {
    map(float, |value: f32| Factor::Number(value))(input)
}

pub fn variable(input: &str) -> IResult<&str, Factor, Error<&str>> {
    map(alpha1, |v: &str| Factor::Variable(v.to_string()))(input)
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
    map(tuple((alt((circle, square)), multispace0)), |(v, _)| {
        Command::Instantiation(v)
    })(input)
}

pub fn command_if(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(
        tuple((
            tag("if"),
            boolean_expr,
            many0(alt((command_for, command_if, draw_shape, assignment))),
            many0(tuple((
                tag("else if"),
                boolean_expr,
                many0(alt((command_for, command_if, draw_shape, assignment))),
            ))),
            opt(tuple((
                tag("else"),
                multispace0,
                many0(alt((command_for, command_if, draw_shape, assignment))),
            ))),
            tag("end if"),
            multispace0,
        )),
        |(_, pred, then_branch, multiple_elif, maybe_else_branch, _, _)| {
            if multiple_elif.len() > 0 {
                if let Some((_, _, cmd)) = maybe_else_branch {
                    let mut my_vec: Vec<(ConditionalBuiltin, Operation, Vec<Command>)> = Vec::new();
                    my_vec.push((ConditionalBuiltin::IfB, pred, then_branch));
                    for (zz, c, ab) in multiple_elif {
                        match zz {
                            "if" => {
                                my_vec.push((ConditionalBuiltin::IfB, c, ab));
                            }
                            "else if" => {
                                my_vec.push((ConditionalBuiltin::ElseIfB, c, ab));
                            }
                            "else" => {
                                my_vec.push((ConditionalBuiltin::ElseB, c, ab));
                            }
                            _ => unimplemented!(),
                        }
                    }
                    my_vec.push((ConditionalBuiltin::ElseB, Operation::Nil, cmd));
                    Command::ConditionalBlock(my_vec)
                } else {
                    unimplemented!(); // else if ** no else ____ERROR
                }
            } else {
                let mut my_vec: Vec<(ConditionalBuiltin, Operation, Vec<Command>)> = Vec::new();
                if let Some((_, _, cmd)) = maybe_else_branch {
                    my_vec.push((ConditionalBuiltin::IfB, pred, then_branch));
                    my_vec.push((ConditionalBuiltin::ElseB, Operation::Nil, cmd));
                } else {
                    my_vec.push((ConditionalBuiltin::IfB, pred, then_branch));
                }
                Command::ConditionalBlock(my_vec)
            }
        },
    )(input)
}

pub fn condition(input: &str) -> IResult<&str, Operation, Error<&str>> {
    map(
        tuple((
            expr,
            alt((tag("<="), tag(">="), tag("="), tag("<"), tag(">"))),
            expr,
        )),
        |(left, op, right)| match op {
            "<=" => Operation::Condition((Box::new(left), Builtin::LesserOrEqual, Box::new(right))),
            ">=" => {
                Operation::Condition((Box::new(left), Builtin::GreaterOrEqual, Box::new(right)))
            }
            "=" => Operation::Condition((Box::new(left), Builtin::Equal, Box::new(right))),
            ">" => Operation::Condition((Box::new(left), Builtin::Greater, Box::new(right))),
            "<" => Operation::Condition((Box::new(left), Builtin::Lesser, Box::new(right))),
            _ => unreachable!(),
        },
    )(input)
}

fn boolean_factor(input: &str) -> IResult<&str, Operation, Error<&str>> {
    map(
        tuple((
            space0,
            alt((
                delimited(tag("("), boolean_expr, tag(")")),
                condition,
                map(tag("true"), |_| Operation::Identity(Factor::Boolean(true))),
                map(tag("false"), |_| {
                    Operation::Identity(Factor::Boolean(false))
                }),
            )),
            space0,
        )),
        |(_, fac, _)| fac,
    )(input)
}

fn boolean_term(input: &str) -> IResult<&str, Operation, Error<&str>> {
    let (rest, init) = boolean_factor(input)?;

    fold_many0(
        pair(tag("and"), boolean_factor),
        init,
        |acc, (op, val): (&str, Operation)| match op {
            "and" => Operation::Condition((Box::new(acc), Builtin::And, Box::new(val))),
            _ => unimplemented!(),
        },
    )(rest)
}

pub fn boolean_expr(input: &str) -> IResult<&str, Operation, Error<&str>> {
    let (rest, init) = boolean_term(input)?;

    fold_many0(
        pair(
            map(delimited(multispace0, tag("or"), multispace0), |_| {
                Builtin::Or
            }),
            boolean_term,
        ),
        init,
        |acc, (op, val): (Builtin, Operation)| match op {
            Builtin::Or => Operation::Condition((Box::new(acc), Builtin::Or, Box::new(val))),
            _ => unimplemented!(),
        },
    )(rest)
}

fn command_for(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(
        tuple((
            tag("for"),
            multispace0,
            digit1,
            multispace0,
            delimited(tag("{"), parser, tag("}")),
            multispace0,
        )),
        |(_, _, times, _, v, _): (_, _, &str, _, Vec<Command>, _)| {
            Command::For((FromStr::from_str(times).unwrap(), v))
        },
    )(input)
}

fn command_move(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(
        tuple((tag("move"), multispace0, expr, tag(","), multispace0, expr)),
        |(_, _, val1, _, _, val2)| Command::Move((val1, val2)),
    )(input)
}

fn command_reset_move(input: &str) -> IResult<&str, Command, Error<&str>> {
    map(delimited(multispace0, tag("reset_m"), multispace0), |_| {
        Command::ResetMove
    })(input)
}

pub fn parser(input: &str) -> IResult<&str, Vec<Command>, Error<&str>> {
    many0(terminated(
        preceded(
            multispace0,
            alt((
                command_reset_move,
                command_move,
                command_for,
                command_if,
                draw_shape,
                assignment,
            )),
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

    #[test]
    fn boolean_expression() {
        let content = "2 > 1";
        let (rest, ast) = boolean_expr(content).unwrap();
        assert_eq!(
            ast,
            Operation::Condition((
                Box::new(Operation::Identity(Factor::Number(2.0))),
                Builtin::Greater,
                Box::new(Operation::Identity(Factor::Number(1.0)))
            ))
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn boolean_expression_with_variables() {
        let content = " x <= y ";
        let (rest, ast) = boolean_expr(content).unwrap();
        assert_eq!(
            ast,
            Operation::Condition((
                Box::new(Operation::Identity(Factor::Variable("x".to_string()))),
                Builtin::LesserOrEqual,
                Box::new(Operation::Identity(Factor::Variable("y".to_string())))
            ))
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn boolean_expression2() {
        let content = " 2 < 1 and  3 > 2";
        let (rest, ast) = boolean_expr(content).unwrap();
        assert_eq!(
            ast,
            Operation::Condition((
                Box::new(Operation::Condition((
                    Box::new(Operation::Identity(Factor::Number(2.0))),
                    Builtin::Lesser,
                    Box::new(Operation::Identity(Factor::Number(1.0)))
                ))),
                Builtin::And,
                Box::new(Operation::Condition((
                    Box::new(Operation::Identity(Factor::Number(3.0))),
                    Builtin::Greater,
                    Box::new(Operation::Identity(Factor::Number(2.0)))
                ))),
            ))
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn if_command() {
        let content = "if x = 1 and (y >= x or x > 3) circle \n end if";
        let (rest, ast) = command_if(content).unwrap();
        assert_eq!(
            ast,
            Command::ConditionalBlock(vec![(
                ConditionalBuiltin::IfB,
                Operation::Condition((
                    Box::new(Operation::Condition((
                        Box::new(Operation::Identity(Factor::Variable("x".to_string()))),
                        Builtin::Equal,
                        Box::new(Operation::Identity(Factor::Number(1.0)))
                    ))),
                    Builtin::And,
                    Box::new(Operation::Condition((
                        Box::new(Operation::Condition((
                            Box::new(Operation::Identity(Factor::Variable("y".to_string()))),
                            Builtin::GreaterOrEqual,
                            Box::new(Operation::Identity(Factor::Variable("x".to_string())))
                        ))),
                        Builtin::Or,
                        Box::new(Operation::Condition((
                            Box::new(Operation::Identity(Factor::Variable("x".to_string()))),
                            Builtin::Greater,
                            Box::new(Operation::Identity(Factor::Number(3.0)))
                        )))
                    )))
                )),
                vec![Command::Instantiation(Node::Circle(Operation::Identity(
                    Factor::Number(1.0)
                )))]
            )])
        );
    }
}
