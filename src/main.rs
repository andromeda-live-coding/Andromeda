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

// need something here
fn eval(
    f: Box<Operation>,
    op: Builtin,
    expr: Box<Operation>,
    variables: &HashMap<String, f32>,
) -> f32 {
    let first = match *f {
        Operation::Identity(value) => get_value(value, variables),
        //final
        Operation::Calculation((first, op, second)) => eval(first, op, second, variables),
    };

    let b = match *expr {
        Operation::Identity(value) => get_value(value, variables),
        //final
        Operation::Calculation((first, op, second)) => eval(first, op, second, variables),
    };
    match op {
        Builtin::Plus => first + b,
        Builtin::Minus => first - b,
        Builtin::Div => first / b,
        Builtin::Mult => first * b,
    }
}

fn declare_variable(
    (name, value): (String, Operation),
    variables: &HashMap<String, f32>,
) -> (String, f32) {
    //dbg!(value.clone());
    match value {
        Operation::Identity(factor) => (name, get_value(factor, variables)),
        Operation::Calculation((first, op, second)) => {
            let first = match *first {
                Operation::Identity(first) => get_value(first, variables),
                // TODO: This should be implemented // MAYBE NOT TO EVALUATE EXPRESSIONS (IT DO OTHER THINGS)
                Operation::Calculation((_first2, _op2, _second2)) => unimplemented!(), // OK
            };
            ///////////////////////////////////////////////////////////////////////////////////////
            let second = match *second {
                Operation::Identity(second) => get_value(second, variables), // OK
                // TODO: This should be implemented
                Operation::Calculation((first2, op2, second2)) => {
                    eval(first2, op2, second2, variables)
                }
            };
            ///////////////////////////////////////////////////////////////////////////////////////
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
    let content = "x: 2\ny: x\nz: x + 2 + 2 * 2 - 19 - ( 19 * 12.6) * 0.1";
    // TODO: Try with this content
    let content2 = "x: 2\ny: x\nz: x + 2 + 3";
    //let content3 = "x: 2\ny: x\nz: x + y + 1 +1 ";
    let (_, ast) = parser(content).unwrap();
    dbg!(ast.clone());
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
