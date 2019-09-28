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
    first: Operation,
    op: Builtin,
    second: Operation,
    variables: &HashMap<String, f32>,
) -> bool {
    match (first, second) {
        (
            Operation::Calculation((left2, op2, right2)),
            Operation::Calculation((left3, op3, right3)),
        ) => match op {
            Builtin::Greater => {
                if eval(*left2, op2, *right2, variables) > eval(*left3, op3, *right3, variables) {
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        },
        (
            Operation::Condition((left2, op2, right2)),
            Operation::Condition((left3, op3, right3)),
        ) => match op {
            Builtin::And => {
                if eval_if(*left2, op2, *right2, variables)
                    > eval_if(*left3, op3, *right3, variables)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::Or => {
                if eval_if(*left2, op2, *right2, variables)
                    || eval_if(*left3, op3, *right3, variables)
                {
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        },
        (Operation::Identity(val1), Operation::Identity(val2)) => match op {
            Builtin::Greater => {
                if get_value(val1, variables) > get_value(val2, &variables) {
                    println!("identity1 > identity2");
                    true
                } else {
                    false
                }
            }
            Builtin::GreaterOrEqual => {
                if get_value(val1, variables) >= get_value(val2, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::Equal => {
                if get_value(val1, variables) == get_value(val2, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::LesserOrEqual => {
                if get_value(val1, variables) <= get_value(val2, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::Lesser => {
                if get_value(val1, variables) < get_value(val2, &variables) {
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        },
        (Operation::Calculation((left2, op2, right2)), Operation::Identity(val)) => match op {
            Builtin::Greater => {
                if eval(*left2, op2, *right2, &variables) > get_value(val, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::GreaterOrEqual => {
                if eval(*left2, op2, *right2, &variables) >= get_value(val, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::Equal => {
                if eval(*left2, op2, *right2, &variables) == get_value(val, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::LesserOrEqual => {
                if eval(*left2, op2, *right2, &variables) <= get_value(val, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::Lesser => {
                if eval(*left2, op2, *right2, &variables) < get_value(val, &variables) {
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        },
        (Operation::Identity(val), Operation::Calculation((left2, op2, right2))) => match op {
            Builtin::Greater => {
                if get_value(val, &variables) > eval(*left2, op2, *right2, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::GreaterOrEqual => {
                if get_value(val, &variables) >= eval(*left2, op2, *right2, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::Equal => {
                if get_value(val, &variables) == eval(*left2, op2, *right2, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::Lesser => {
                if get_value(val, &variables) < eval(*left2, op2, *right2, &variables) {
                    true
                } else {
                    false
                }
            }
            Builtin::LesserOrEqual => {
                if get_value(val, &variables) <= eval(*left2, op2, *right2, &variables) {
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        },
        (Operation::Condition((left2, op2, right2)), Operation::Identity(Factor::Boolean(val))) => {
            match op {
                Builtin::And => {
                    if eval_if(*left2, op2, *right2, &variables) && val {
                        true
                    } else {
                        false
                    }
                }
                Builtin::Or => {
                    if eval_if(*left2, op2, *right2, &variables) || val {
                        true
                    } else {
                        false
                    }
                }
                _ => unimplemented!(),
            }
        }
        (Operation::Identity(Factor::Boolean(val)), Operation::Condition((left2, op2, right2))) => {
            match op {
                Builtin::And => {
                    if val && eval_if(*left2, op2, *right2, &variables) {
                        true
                    } else {
                        false
                    }
                }
                Builtin::Or => {
                    if val || eval_if(*left2, op2, *right2, &variables) {
                        true
                    } else {
                        false
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
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
    let _content = "x: 2\n
        y: x\n
        x: 1\n
        z: ((x + (2 + 3)) * y) / 2\n
        square z+x (19.1*2)\n
        square\n 
        circle\n
        if 12.6 = x+ 19.91\n
         square  19.2\n
          else  
          circle  17.1 
          end if";
    let _content2 = "circle      \n x: 2\n if 2+x = 5\n square \n\n else circle\n  end if";
    let content3 = "x: 23 \n if 24+x > 7 and true circle\n end if\n";
    let (rest, ast) = parser(content3).unwrap();
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
            Command::ConditionalBlock(branches) => {
                for (branch, pred, commands) in branches {
                    match branch {
                        ConditionalBuiltin::IfB => match pred {
                            Operation::Identity(Factor::Boolean(true)) => {
                                println!("condition true");
                            }
                            Operation::Identity(Factor::Boolean(false)) => {
                                println!("condition false")
                            }
                            // condition
                            Operation::Condition((left, op, right)) => {
                                if eval_if(*left, op, *right, &variables) {
                                    println!("done bitch");
                                }
                            }
                            _ => unimplemented!(),
                        },
                        ConditionalBuiltin::ElseIfB => {}
                        ConditionalBuiltin::ElseB => {}
                    }
                }
            }
        }
    }
    // assert_eq!(*variables.get("x").unwrap(), 1.0);
    // assert_eq!(*variables.get("y").unwrap(), 2.0);
    // assert_eq!(*variables.get("z").unwrap(), 6.0);
    // assert_eq!(rest, "");
    dbg!(rest);
}

// BUGS TO SOLVE
// true || false are parsed as variables
