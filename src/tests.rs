use super::parser::{
    boolean_expr, command_if, expr, parser, Builtin, Command, ConditionalBuiltin, Factor, Node,
    Operation,
};

#[test]
fn left_recursive() {
    let (rest, expr) = expr("1+2+3+4").unwrap();
    assert_eq!(rest, "");
    assert_eq!(
        expr,
        Operation::Calculation((
            Box::new(Operation::Calculation((
                Box::new(Operation::Calculation((
                    Box::new(Operation::Identity(Factor::Number(1.0))),
                    Builtin::Plus,
                    Box::new(Operation::Identity(Factor::Number(2.0)))
                ))),
                Builtin::Plus,
                Box::new(Operation::Identity(Factor::Number(3.0)))
            ))),
            Builtin::Plus,
            Box::new(Operation::Identity(Factor::Number(4.0)))
        ))
    );
}

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
    assert_eq!(rest, "");
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
