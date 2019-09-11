mod parser;
use parser::{parser, Builtin, Expression, Factor, Operation};
use std::collections::HashMap;

fn get_value(factor: Factor, variables: &HashMap<String, f32>) -> f32 {
    match factor {
        Factor::Number(number) => number,
        Factor::Variable(variable_name) => *variables.get(&variable_name).unwrap(),
        _ => unimplemented!(),
    }
}

fn declare_variable(
    (name, value): (String, Operation),
    variables: &HashMap<String, f32>,
) -> (String, f32) {
    match value {
        Operation::Identity(factor) => (name, get_value(factor, variables)),
        Operation::Calculation((first, op, second)) => {
            let first = match *first {
                Operation::Identity(first) => get_value(first, variables),
                _ => unimplemented!(),
            };
            let second = match *second {
                Operation::Identity(second) => get_value(second, variables),
                _ => unimplemented!(),
            };
            match op {
                Builtin::Plus => (name, first + second),
                Builtin::Minus => (name, first - second),
                Builtin::Div => (name, first / second),
                Builtin::Mult => (name, first * second),
            }
        }
    }
}

fn main() {
    let content = "x: 2\ny: x\nz: x + 2";
    let (_, ast) = parser(content).unwrap();
    let mut variables: HashMap<String, f32> = HashMap::new();
    for expression in ast {
        match expression {
            Expression::Declaration(declaration) => {
                let (name, value) = declare_variable(declaration, &variables);
                variables.insert(name, value);
            }
        }
    }
    dbg!(variables.clone());
}
