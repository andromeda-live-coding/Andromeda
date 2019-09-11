mod parser;
use parser::{parser, Expression};
use std::collections::HashMap;

fn extract_float_variables(ast: Expression) -> HashMap<String, f32> {
    unimplemented!()
}

fn calculate_result(ast: Expression) -> f32 {
    unimplemented!()
}

fn main() {
    let content = "x: 2\ny: (x + 1) * 2\n";
    let (rest, ast) = parser(content).unwrap();
    let variables = extract_float_variables()
    if calculate_result(ast, variables) == 6.0 {
        println!("YEAH");
    }
}
