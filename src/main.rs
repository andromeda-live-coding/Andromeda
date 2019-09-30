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

fn eval_boolean_expr(
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
                if eval_boolean_expr(*left2, op2, *right2, variables)
                    && eval_boolean_expr(*left3, op3, *right3, variables)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::Or => {
                if eval_boolean_expr(*left2, op2, *right2, variables)
                    || eval_boolean_expr(*left3, op3, *right3, variables)
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
                    if eval_boolean_expr(*left2, op2, *right2, &variables) && val {
                        true
                    } else {
                        false
                    }
                }
                Builtin::Or => {
                    if eval_boolean_expr(*left2, op2, *right2, &variables) || val {
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
                    if val && eval_boolean_expr(*left2, op2, *right2, &variables) {
                        true
                    } else {
                        false
                    }
                }
                Builtin::Or => {
                    if val || eval_boolean_expr(*left2, op2, *right2, &variables) {
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

fn eval_conditional_block(
    branches: Vec<(ConditionalBuiltin, Operation, Vec<Command>)>,
    variables: &HashMap<String, f32>,
) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut found = false;

    for (branch, pred, commands) in branches {
        if found {
            break;
        }
        match branch {
            ConditionalBuiltin::IfB => match pred {
                Operation::Identity(Factor::Boolean(true)) => {
                    found = true;
                    // commands
                    for command in commands {
                        match command {
                            Command::Instantiation(node) => {
                                nodes.push(node);
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                Operation::Identity(Factor::Boolean(false)) => {
                    // false condition
                }
                Operation::Condition((left, op, right)) => {
                    if eval_boolean_expr(*left, op, *right, &variables) {
                        found = true;
                        for command in commands {
                            match command {
                                Command::Declaration(_) => unimplemented!(),
                                Command::Instantiation(node) => {
                                    nodes.push(node);
                                }
                                // if if
                                Command::ConditionalBlock(branches2) => {
                                    // eval_conditional_block
                                    let c = eval_conditional_block(branches2, &variables);
                                    for elem in c {
                                        nodes.push(elem);
                                    }
                                }
                                Command::For((n, cmds)) => {
                                    let c = eval_for(n, cmds, &variables);
                                    for elem in c {
                                        nodes.push(elem);
                                    }
                                }
                            }
                        }
                    } else {

                    }
                }
                _ => unimplemented!(),
            },
            ConditionalBuiltin::ElseIfB => match pred {
                Operation::Identity(Factor::Boolean(true)) => {
                    found = true;
                    // commands
                    for command in commands {
                        match command {
                            Command::Instantiation(node) => {
                                nodes.push(node);
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                Operation::Identity(Factor::Boolean(false)) => {
                    // false condition
                }
                Operation::Condition((left, op, right)) => {
                    if eval_boolean_expr(*left, op, *right, &variables) {
                        found = true;
                        for command in commands {
                            match command {
                                Command::Instantiation(node) => {
                                    nodes.push(node);
                                }
                                _ => unimplemented!(),
                            }
                        }
                    } else {
                    }
                }
                _ => unimplemented!(),
            },
            ConditionalBuiltin::ElseB => {
                for command in commands {
                    match command {
                        Command::Instantiation(node) => {
                            nodes.push(node);
                        }
                        _ => unimplemented!(),
                    }
                }
            }
        }
    }
    nodes
}

fn eval_for(times: i32, commands: Vec<Command>, variables: &HashMap<String, f32>) -> Vec<Node> {
    let mut v: HashMap<String, f32> = HashMap::new();
    let mut c: Vec<Node> = Vec::new();
    for x in 0..times {
        for l in commands.clone() {
            match l {
                Command::Instantiation(nd) => {
                    c.push(nd);
                }
                Command::Declaration((name, value)) => {
                    let (name, value) = declare_variable((name, value), &variables);
                    v.insert(name, value);
                }
                Command::ConditionalBlock(cb) => {
                    let nodes = eval_conditional_block(cb, &variables);
                    for elem in nodes {
                        c.push(elem);
                    }
                }
                Command::For((times, commands)) => {
                    let nodes = eval_for(times, commands, variables);
                    for elem in nodes {
                        c.push(elem);
                    }
                }
            }
        }
    }
    c
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
    let content = "for 2 { if 1 > 1 square 2\n  end if }";
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
            Command::ConditionalBlock(branches) => {
                let tmp = eval_conditional_block(branches, &variables);
                for elem in tmp {
                    nodes.push(elem);
                }
            }
            Command::For((times, commands)) => {
                let tmp = eval_for(times, commands, &variables);
                for elem in tmp {
                    nodes.push(elem);
                }
            }
        }
    }
    dbg!(rest);
    dbg!(nodes);
}

// it parse for if //
// it parse nested if //
// but not evaluating

// BUGS TO SOLVE
// true || false are parsed as variables so the command **true: 71.7** will be parsed
// it is not possible to declare variables inside nested for or nested if
