// *****parser*****
// it recognizes *instructions* like "box x y"
// i need to convert variables in f32, if the variables were previously declared
// **IMPORTANT** MVC has just to do DrawShapeWf32(shape, val1, val2) where val1 and val2 are f32!
// i need an HashMap inside the parser because we need to keep track of all variables in the context.
use nom::branch::alt;
use nom::character::complete::{alpha1, char, multispace0, one_of, space0};
use nom::combinator::map;
use nom::error::VerboseError as Error;
use nom::multi::{fold_many0, many0, many_till};
use nom::number::complete::float;
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

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
pub enum Atom {
    Factor(Factor),
    Builtin(Builtin),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Identity(Factor),
    Calculation((Box<Operation>, Builtin, Box<Operation>)),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Constant(Atom),
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

    for (index, atom) in atoms.clone().iter().enumerate() {
        match atom {
            Atom::Factor(factor) => {
                factors.push(Operation::Identity(factor.clone()));
            }
            Atom::Builtin(operator) => {
                if operators.len() > 0 {
                    if has_higher_precedence(&operators.last().unwrap(), &operator) {
                        let op = operators.pop().unwrap();
                        let second = factors.pop().unwrap();
                        let first = factors.pop().unwrap();
                        factors.push(Operation::Calculation((
                            Box::new(first),
                            op,
                            Box::new(second),
                        )))
                    }
                }
                operators.push(operator.clone());
            }
        }
    }

    // TODO: Apply the shunting yard algorithm here
    // https://stackoverflow.com/questions/28256/equation-expression-parser-with-precedence#47717
    dbg!(factors.clone());
    dbg!(operators.clone());
    // TODO: Remove this
    Ok(("", Operation::Identity(Factor::Number(1.0))))
}

pub fn parser(input: &str) -> IResult<&str, Vec<Expression>, Error<&str>> {
    many0(terminated(
        preceded(
            multispace0,
            map(assignment, |(name, operation): (String, Operation)| {
                Expression::Declaration((name, operation))
            }),
        ),
        multispace0,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // #[test]
    // fn declare_variable() {
    //     let mut variables: HashMap<String, f32> = HashMap::new();
    //     let (_, commands) = parser("x: 2").unwrap();
    //     for command in commands {
    //         if let Command::DeclareVariable((name, value)) = command {
    //             variables.insert(name, eval(value));
    //         }
    //     }
    //     assert_eq!(variables.get("x"), Some(&2.0));
    // }
    // #[test]
    // fn test_simple_expression() {
    //     let expression = "x: 3\ny: 1\nz: y * 2.0\n";
    //     let (rest, ast) = parser(expression).unwrap();
    //     assert_eq!(
    //         ast[0],
    //         Command::DeclareVariable((
    //             "x".to_string(),
    //             vec![Operation::Identity(Value::Number(3.0))]
    //         ))
    //     );
    //     assert_eq!(
    //         ast[1],
    //         Command::DeclareVariable((
    //             "y".to_string(),
    //             vec![Operation::Identity(Value::Number(1.0))]
    //         ))
    //     );
    //     assert_eq!(
    //         ast[2],
    //         Command::DeclareVariable((
    //             "z".to_string(),
    //             vec![Operation::Mult((
    //                 Value::Variable("y".to_string()),
    //                 Value::Number(2.0)
    //             ))]
    //         ))
    //     );
    //     assert_eq!(rest, "");
    // }

    #[test]
    fn test_several_operations() {
        let expression = "z: y + 2.0 * x + 3\n";
        let (rest, ast) = parser(expression).unwrap();
        // dbg!(ast.clone());
        assert_eq!(false, true);
        // assert_eq!(
        //     ast[0],
        //     Command::DeclareVariable((
        //         "z".to_string(),
        //         vec![Operation::Plus((
        //             Factor::Variable("y".to_string()),
        //             ::Operation(Box::new(Operation::Mult((
        //                 Value::Number(2.0),
        //                 Value::Variable("x".to_string())
        //             ))))
        //         ))]
        //     ))
        // );
    }
}
