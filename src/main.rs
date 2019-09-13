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

fn eval_calculation(expr: Box<Operation>, variables: &HashMap<String, f32>) -> f32 {
    match *expr {
        Operation::Identity(value) => get_value(value, variables),
        Operation::Calculation((first, op, second)) => eval_calculation(second, variables),
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
                // TODO: This should be implemented // MAYBE NOT TO EVALUATE EXPRESSIONS (IT DO OTHER THINGS)
                Operation::Calculation((_first2, _op2, _second2)) => unimplemented!(),
            };
            let second = match *second {
                Operation::Identity(second) => get_value(second, variables),
                // TODO: This should be implemented
                Operation::Calculation((_first2, _op2, second2)) => {
                    match *second2 {
                        Operation::Identity(x) => get_value(x, variables),
                        // if second2 is not an identity, it means it is a Calculation where the third element of the tuple can be a Calculation..
                        // so we have to loop until the third element is a Identity!
                        Operation::Calculation((_first_r, _op_r, s_recursive)) => {
                            eval_calculation(s_recursive, variables)
                        }
                    }
                }
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
    // TODO: Try with this content
    let content2 = "x: 2\ny: x\nz: x + 2 + 3";
    //let content3 = "x: 2\ny: x\nz: x + y + 1 +1 ";
    let (_, ast) = parser(content2).unwrap();
    let mut variables: HashMap<String, f32> = HashMap::new();
    for expression in ast {
        match expression {
            Expression::Declaration(declaration) => {
                let (name, value) = declare_variable(declaration, &variables);
                variables.insert(name, value);
            }
        }
    }
    let (_, ast2) = parser(content2).unwrap();
    dbg!(ast2);
    dbg!(variables.clone());
}
