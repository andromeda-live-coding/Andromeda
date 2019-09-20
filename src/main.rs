mod parser;
use parser::*;
use std::collections::HashMap;

fn get_value(factor: Factor, variables: &HashMap<String, f32>) -> f32 {
    match factor {
        Factor::Number(number) => number,
        Factor::Variable(variable_name) => *variables.get(&variable_name).unwrap(),
        _ => unimplemented!(),
    }
}

fn eval(first: Operation, op: Builtin, second: Operation, variables: &HashMap<String, f32>) -> f32 {
    let first = match first {
        Operation::Identity(first) => get_value(first, variables),
        Operation::Calculation((first, op, second)) => eval(*first, op, *second, variables),
        _ => unimplemented!(),
    };
    let second = match second {
        Operation::Identity(second) => get_value(second, variables),
        Operation::Calculation((first, op, second)) => eval(*first, op, *second, variables),
        _ => unimplemented!(),
    };
    match op {
        Builtin::Plus => first + second,
        Builtin::Minus => first - second,
        Builtin::Div => first / second,
        Builtin::Mult => first * second,
        _ => unreachable!(),
    }
}

fn eval_if(
    pred: Operation,
    true_branch: Vec<Command>,
    false_branch: Vec<Command>,
    variables: &HashMap<String, f32>,
) -> bool {
    true
    /*let first = match first {
        Operation::Identity(first) => get_value(first, variables),
        Operation::Calculation((first, op, second)) => eval(*first, op, *second, variables),
    };
    let second = match second {
        Operation::Identity(second) => get_value(second, variables),
        Operation::Calculation((first, op, second)) => eval(*first, op, *second, variables),
    };

    match op {
        Builtin::Equal => {
            if first == second {
                true
            } else {
                false
            }
        }
        Builtin::Lesser => {
            if first < second {
                true
            } else {
                false
            }
        }
        Builtin::Greater => {
            if first > second {
                true
            } else {
                false
            }
        }
        Builtin::LesserOrEqual => {
            if first <= second {
                true
            } else {
                false
            }
        }
        Builtin::GreaterOrEqual => {
            if first >= second {
                true
            } else {
                false
            }
        }
        _ => unimplemented!(),
    }*/
}

fn declare_variable(
    (name, value): (String, Operation),
    variables: &HashMap<String, f32>,
) -> (String, f32) {
    match value {
        Operation::Identity(factor) => (name, get_value(factor, variables)),
        Operation::Calculation((first, op, second)) => (name, eval(*first, op, *second, variables)),
        _ => unimplemented!(),
    }
}

fn main() {
    let content =
        "x: 2\ny: x\nx: 1\nz: ((x + (2 + 3)) * y) / 2\nsquare z+x (19.1*2)\n square\n circle\nif 12.6 = x+ 19.91\n square  \n else  circle  \n end if";
    let content2 = "x: 2\n if 2+x = 5      circle   \n   else     circle  \n   end if  ";
    let (rest, ast) = parser(content).unwrap();
    dbg!(ast.clone());
    let mut variables: HashMap<String, f32> = HashMap::new();
    let mut nodes: Vec<Node> = vec![];
    for expression in ast {
        match expression {
            Command::Declaration(declaration) => {
                let (name, value) = declare_variable(declaration, &variables);
                variables.insert(name, value);
            }
            Command::Instantiation(node) => nodes.push(node),
            Command::CommandIf((x, y, z)) => {
                if eval_if(x, y, z, &variables) {
                    println!("true")
                } else {
                    println!("false")
                }
            }
            _ => unimplemented!(),
        }
    }
    //assert_eq!(*variables.get("x").unwrap(), 1.0);
    //assert_eq!(*variables.get("y").unwrap(), 2.0);
    //assert_eq!(*variables.get("z").unwrap(), 6.0);
    //assert_eq!(rest, "");
    dbg!(rest);
}
