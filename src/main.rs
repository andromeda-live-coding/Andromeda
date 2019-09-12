mod parser;
use parser::{parser, Builtin, Expression, Factor, Node, Operation};
use std::collections::HashMap;

fn get_value(factor: Factor, variables: &HashMap<String, f32>) -> f32 {
    match factor {
        Factor::Number(number) => number,
        Factor::Variable(variable_name) => *variables.get(&variable_name).unwrap(),
    }
}

fn calculate(
    first: Operation,
    op: Builtin,
    second: Operation,
    variables: &HashMap<String, f32>,
) -> f32 {
    let first = match first {
        Operation::Identity(first) => get_value(first, variables),
        Operation::Calculation((first, op, second)) => calculate(*first, op, *second, variables),
    };
    let second = match second {
        Operation::Identity(second) => get_value(second, variables),
        Operation::Calculation((first, op, second)) => calculate(*first, op, *second, variables),
    };
    match op {
        Builtin::Plus => first + second,
        Builtin::Minus => first - second,
        Builtin::Div => first / second,
        Builtin::Mult => first * second,
    }
}

fn declare_variable(
    (name, value): (String, Operation),
    variables: &HashMap<String, f32>,
) -> (String, f32) {
    match value {
        Operation::Identity(factor) => (name, get_value(factor, variables)),
        Operation::Calculation((first, op, second)) => {
            (name, calculate(*first, op, *second, variables))
        }
    }
}

fn main() {
    // Tiè
    let content = "x: 2\ny: x\nx: 1\nz: ((x + (2 + 3)) * y) / 2\nsquare y z";
    let (_, ast) = parser(content).unwrap();
    let mut variables: HashMap<String, f32> = HashMap::new();
    let mut nodes: Vec<Node> = vec![];
    for expression in ast {
        match expression {
            Expression::Declaration(declaration) => {
                let (name, value) = declare_variable(declaration, &variables);
                variables.insert(name, value);
            }
            Expression::Instantiation(node) => nodes.push(node),
        }
    }
    assert_eq!(*variables.get("x").unwrap(), 1.0);
    assert_eq!(*variables.get("y").unwrap(), 2.0);
    assert_eq!(*variables.get("z").unwrap(), 6.0);
}
