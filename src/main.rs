mod parser;
mod tests;
use colored::Colorize;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use parser::*;

use std::collections::HashMap;
fn get_value(factor: Factor, variables: &HashMap<String, f32>, time: f32) -> f32 {
    match factor {
        Factor::Number(number) => number,
        Factor::Variable(variable_name) => *variables.get(&variable_name).unwrap(),
        Factor::Sin(v) => match *v {
            Operation::Calculation((l, op, right)) => eval(*l, op, *right, &variables, time).sin(),
            Operation::Identity(val) => get_value(val, variables, time).sin(),
            _ => unimplemented!(),
        },
        Factor::Cos(v) => match *v {
            Operation::Calculation((l, op, right)) => eval(*l, op, *right, &variables, time).cos(),
            Operation::Identity(val) => get_value(val, variables, time).cos(),
            _ => unimplemented!(),
        },
        Factor::Time => time,
        _ => unimplemented!(),
    }
}

fn eval(
    first: Operation,
    op: Builtin,
    second: Operation,
    variables: &HashMap<String, f32>,
    life_time: f32,
) -> f32 {
    let first = match first {
        Operation::Identity(first) => get_value(first, variables, life_time),
        Operation::Calculation((first, op, second)) => {
            eval(*first, op, *second, variables, life_time)
        }
        _ => unimplemented!(),
    };
    let second = match second {
        Operation::Identity(second) => get_value(second, variables, life_time),
        Operation::Calculation((first, op, second)) => {
            eval(*first, op, *second, variables, life_time)
        }
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
    life_time: f32,
) -> bool {
    match (first, second) {
        (
            Operation::Calculation((left2, op2, right2)),
            Operation::Calculation((left3, op3, right3)),
        ) => match op {
            Builtin::Greater => {
                if eval(*left2, op2, *right2, variables, life_time)
                    > eval(*left3, op3, *right3, variables, life_time)
                {
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
                if eval_boolean_expr(*left2, op2, *right2, variables, life_time)
                    && eval_boolean_expr(*left3, op3, *right3, variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::Or => {
                if eval_boolean_expr(*left2, op2, *right2, variables, life_time)
                    || eval_boolean_expr(*left3, op3, *right3, variables, life_time)
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
                if get_value(val1, variables, life_time) > get_value(val2, &variables, life_time) {
                    true
                } else {
                    false
                }
            }
            Builtin::GreaterOrEqual => {
                if get_value(val1, variables, life_time) >= get_value(val2, &variables, life_time) {
                    true
                } else {
                    false
                }
            }
            Builtin::Equal => {
                if get_value(val1, variables, life_time) == get_value(val2, &variables, life_time) {
                    true
                } else {
                    false
                }
            }
            Builtin::LesserOrEqual => {
                if get_value(val1, variables, life_time) <= get_value(val2, &variables, life_time) {
                    true
                } else {
                    false
                }
            }
            Builtin::Lesser => {
                if get_value(val1, variables, life_time) < get_value(val2, &variables, life_time) {
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        },
        (Operation::Calculation((left2, op2, right2)), Operation::Identity(val)) => match op {
            Builtin::Greater => {
                if eval(*left2, op2, *right2, &variables, life_time)
                    > get_value(val, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::GreaterOrEqual => {
                if eval(*left2, op2, *right2, &variables, life_time)
                    >= get_value(val, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::Equal => {
                if eval(*left2, op2, *right2, &variables, life_time)
                    == get_value(val, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::LesserOrEqual => {
                if eval(*left2, op2, *right2, &variables, life_time)
                    <= get_value(val, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::Lesser => {
                if eval(*left2, op2, *right2, &variables, life_time)
                    < get_value(val, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            _ => unimplemented!(),
        },
        (Operation::Identity(val), Operation::Calculation((left2, op2, right2))) => match op {
            Builtin::Greater => {
                if get_value(val, &variables, life_time)
                    > eval(*left2, op2, *right2, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::GreaterOrEqual => {
                if get_value(val, &variables, life_time)
                    >= eval(*left2, op2, *right2, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::Equal => {
                if get_value(val, &variables, life_time)
                    == eval(*left2, op2, *right2, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::Lesser => {
                if get_value(val, &variables, life_time)
                    < eval(*left2, op2, *right2, &variables, life_time)
                {
                    true
                } else {
                    false
                }
            }
            Builtin::LesserOrEqual => {
                if get_value(val, &variables, life_time)
                    <= eval(*left2, op2, *right2, &variables, life_time)
                {
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
                    if eval_boolean_expr(*left2, op2, *right2, &variables, life_time) && val {
                        true
                    } else {
                        false
                    }
                }
                Builtin::Or => {
                    if eval_boolean_expr(*left2, op2, *right2, &variables, life_time) || val {
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
                    if val && eval_boolean_expr(*left2, op2, *right2, &variables, life_time) {
                        true
                    } else {
                        false
                    }
                }
                Builtin::Or => {
                    if val || eval_boolean_expr(*left2, op2, *right2, &variables, life_time) {
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
    life_time: f32,
) -> Vec<Command> {
    let mut nodes: Vec<Command> = Vec::new();
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
                                nodes.push(Command::Instantiation(node));
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                Operation::Identity(Factor::Boolean(false)) => {
                    // false condition
                }
                Operation::Condition((left, op, right)) => {
                    if eval_boolean_expr(*left, op, *right, &variables, life_time) {
                        found = true;
                        for command in commands {
                            match command {
                                Command::Declaration(d) => {
                                    nodes.push(Command::Declaration(d));
                                }
                                Command::Instantiation(node) => {
                                    nodes.push(Command::Instantiation(node));
                                }
                                Command::ConditionalBlock(branches2) => {
                                    let c =
                                        eval_conditional_block(branches2, &variables, life_time);
                                    for elem in c {
                                        nodes.push(elem);
                                    }
                                }
                                Command::For((n, cmds)) => {
                                    let c = eval_for(n, cmds, &variables, life_time);
                                    for elem in c {
                                        nodes.push(elem);
                                    }
                                }
                                Command::ResetMove => nodes.push(Command::ResetMove),
                                Command::Move((x, y)) => nodes.push(Command::Move((x, y))),
                                Command::Color((r, g, b)) => nodes.push(Command::Color((r, g, b))),
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
                    for command in commands {
                        match command {
                            Command::Instantiation(node) => {
                                nodes.push(Command::Instantiation(node));
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                Operation::Identity(Factor::Boolean(false)) => {}
                Operation::Condition((left, op, right)) => {
                    if eval_boolean_expr(*left, op, *right, &variables, life_time) {
                        found = true;
                        for command in commands {
                            match command {
                                Command::Instantiation(node) => {
                                    nodes.push(Command::Instantiation(node));
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
                            nodes.push(Command::Instantiation(node));
                        }
                        _ => unimplemented!(),
                    }
                }
            }
        }
    }
    nodes
}

fn eval_for(
    times: i32,
    commands: Vec<Command>,
    variables: &HashMap<String, f32>,
    life_time: f32,
) -> Vec<Command> {
    let mut c: Vec<Command> = Vec::new();
    for _ in 0..times {
        for l in commands.clone() {
            match l {
                Command::Instantiation(nd) => {
                    c.push(Command::Instantiation(nd));
                }
                Command::Declaration((name, value)) => {
                    // let (name, value) = declare_variable((name, value), &variables, life_time);
                    // v.insert(name, value);
                    c.push(Command::Declaration((name, value)));
                }
                Command::ConditionalBlock(cb) => {
                    let nodes = eval_conditional_block(cb, &variables, life_time);
                    for elem in nodes {
                        c.push(elem);
                    }
                }
                Command::For((times, commands)) => {
                    let nodes = eval_for(times, commands, variables, life_time);
                    for elem in nodes {
                        c.push(elem);
                    }
                }
                Command::ResetMove => c.push(Command::ResetMove),
                Command::Move((x, y)) => c.push(Command::Move((x, y))),
                Command::Color((r, g, b)) => c.push(Command::Color((r, g, b))),
            }
        }
    }
    c
}

fn declare_variable(
    (name, value): (String, Operation),
    variables: &HashMap<String, f32>,
    life_time: f32,
) -> (String, f32) {
    match value {
        Operation::Identity(factor) => (name, get_value(factor, variables, life_time)),
        Operation::Calculation((first, op, second)) => {
            (name, eval(*first, op, *second, variables, life_time))
        }
        _ => unimplemented!(),
    }
}

// BUGS TO SOLVE
// true || false are parsed as variables so the command **true: 71.7** will be parsed
// it's not possible to **y: -x**
//

fn main() {
    let content = "";
    let (rest, ast) = parser(content).unwrap();
    dbg!(ast.clone());
    let mut variables: HashMap<String, f32> = HashMap::new();
    let mut nodes: Vec<Command> = vec![];
    for expression in ast {
        match expression {
            Command::Declaration(declaration) => {
                let (name, value) = declare_variable(declaration, &variables, 0.0);
                variables.insert(name, value);
            }
            Command::Instantiation(node) => nodes.push(Command::Instantiation(node)),
            Command::ConditionalBlock(branches) => {
                let tmp = eval_conditional_block(branches, &variables, 0.0);
                for elem in tmp {
                    nodes.push(elem);
                }
            }
            Command::For((times, commands)) => {
                let tmp = eval_for(times, commands, &variables, 0.0);
                for elem in tmp {
                    nodes.push(elem);
                }
            }
            _ => unimplemented!(),
        }
    }
    dbg!(rest);
    dbg!(nodes);
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    text_edit: String,
    //variables: HashMap<String, f32>,
    instructions: Vec<Command>,
    life_time: f32,
    stream: audio::Stream<Audio>,
}

struct Audio {
    phase: f64,
    hz: f64,
}

struct Ids {
    text_edit: widget::Id,
    _life_time: widget::Id,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(3));
    app.new_window()
        .with_dimensions(720, 500)
        .event(window_event)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .mouse_entered(mouse_entered)
        .mouse_exited(mouse_exited)
        .moved(window_moved)
        .resized(window_resized)
        .focused(window_focused)
        .unfocused(window_unfocused)
        .closed(window_closed)
        .build()
        .unwrap();
    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();
    ui.fonts_mut()
        .insert_from_file("F25_Bank_Printer.ttf")
        .unwrap();
    // Generate some ids for our widgets.
    let ids = Ids {
        text_edit: ui.generate_widget_id(),
        _life_time: ui.generate_widget_id(),
    };

    // Init our variables
    let text_edit = "".to_string();
    let life_time = 0.0;
    //let variables = HashMap::new();
    let instructions: Vec<Command> = Vec::new();
    let m = Audio {
        phase: 0.0,
        hz: 2440.0,
    };

    let audio_host = audio::Host::new();
    let stream = audio_host
        .new_output_stream(m)
        .render(audio)
        .build()
        .unwrap();
    Model {
        ui,
        ids,
        text_edit,
        //variables,
        instructions,
        life_time,
        stream,
    }
}

fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    println!("{}", sample_rate);
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI * audio.phase as f32).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
            println!("{}", sine_amp);
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();
    model.life_time = _app.time;
    if let Some(edit) = widget::TextEdit::new(&model.text_edit)
        .color(color::WHITE)
        .font_size(16)
        .top_left_with_margin(10.0)
        .line_spacing(2.5)
        .restrict_to_height(false)
        .set(model.ids.text_edit, ui)
    {
        model.text_edit = edit;
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let mut variables: HashMap<String, f32> = HashMap::new();
    let draw = app.draw();
    let life_time = model.life_time;
    let mut position: (f32, f32) = (0.0, 0.0);
    let _std_value = 10.0;
    draw.background().rgb(0.83, 0.83, 0.83);
    let mut color = rgb(1.0, 1.0, 1.0);
    for x in model.instructions.clone() {
        match x {
            Command::Declaration((name, value)) => {
                let (name, value) =
                    declare_variable((name.to_string(), value.clone()), &variables, life_time);
                variables.insert(name, value);
            }
            Command::Instantiation(nd) => match nd {
                Node::Circle(v) => match v {
                    Operation::Calculation((l, op, r)) => {
                        let val = eval(*l, op, *r, &variables, life_time);
                        dbg!(val);
                        draw.ellipse()
                            .x_y(position.0, position.1)
                            .w_h(val, val)
                            .color(color);
                    }
                    Operation::Identity(f) => match f {
                        Factor::Number(val) => {
                            draw.ellipse()
                                .x_y(position.0, position.1)
                                .w_h(val, val)
                                .color(color);
                        }
                        Factor::Variable(var_name) => {
                            draw.ellipse()
                                .x_y(position.0, position.1)
                                .w_h(
                                    get_value(
                                        Factor::Variable(var_name.to_string()),
                                        &variables,
                                        life_time,
                                    ),
                                    get_value(
                                        Factor::Variable(var_name.to_string()),
                                        &variables,
                                        life_time,
                                    ),
                                )
                                .color(color);
                        }
                        Factor::Boolean(_) => unimplemented!(),
                        Factor::Sin(val) => match *val {
                            Operation::Calculation((l, op, r)) => {
                                let val = eval(*l, op, *r, &variables, life_time).sin();
                                dbg!(val);
                                draw.ellipse()
                                    .x_y(position.0, position.1)
                                    .w_h(val, val)
                                    .color(color);
                            }
                            Operation::Identity(v) => match v {
                                Factor::Number(value) => {
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(value.sin(), value.sin())
                                        .color(color);
                                }
                                Factor::Variable(v) => {
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(
                                            get_value(
                                                Factor::Variable(v.to_string()),
                                                &variables,
                                                life_time,
                                            )
                                            .sin(),
                                            get_value(
                                                Factor::Variable(v.to_string()),
                                                &variables,
                                                life_time,
                                            )
                                            .sin(),
                                        )
                                        .color(color);
                                }
                                Factor::Time => {
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(life_time.sin(), life_time.sin())
                                        .color(color);
                                }
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        },
                        Factor::Cos(val) => match *val {
                            Operation::Calculation((l, op, r)) => {
                                let val = eval(*l, op, *r, &variables, life_time).cos();
                                dbg!(val);
                                draw.ellipse()
                                    .x_y(position.0, position.1)
                                    .w_h(val, val)
                                    .color(color);
                            }
                            Operation::Identity(v) => match v {
                                Factor::Number(value) => {
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(value.cos(), value.cos())
                                        .color(color);
                                }
                                Factor::Variable(v) => {
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(
                                            get_value(
                                                Factor::Variable(v.to_string()),
                                                &variables,
                                                life_time,
                                            )
                                            .cos(),
                                            get_value(
                                                Factor::Variable(v.to_string()),
                                                &variables,
                                                life_time,
                                            )
                                            .cos(),
                                        )
                                        .color(color);
                                }
                                Factor::Time => {
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(life_time.cos(), life_time.cos())
                                        .color(color);
                                }
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        },
                        Factor::Time => unimplemented!(),
                    },
                    _ => unimplemented!(),
                },
                Node::Square((width, height)) => {
                    let width = match width {
                        Operation::Calculation((l, op, right)) => {
                            eval(*l, op, *right, &variables, life_time)
                        }
                        Operation::Identity(l) => match l {
                            Factor::Number(value) => value,
                            Factor::Variable(var) => {
                                get_value(Factor::Variable(var), &variables, life_time)
                            }
                            Factor::Sin(val) => match *val {
                                Operation::Calculation((l, op, r)) => {
                                    eval(*l, op, *r, &variables, life_time).sin()
                                }
                                Operation::Identity(val) => match val {
                                    Factor::Variable(l) => {
                                        get_value(Factor::Variable(l), &variables, life_time).sin()
                                    }
                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    };

                    let height = match height {
                        Operation::Calculation((l, op, right)) => {
                            eval(*l, op, *right, &variables, life_time)
                        }
                        Operation::Identity(l) => match l {
                            Factor::Number(value) => value,
                            Factor::Variable(var) => {
                                get_value(Factor::Variable(var), &variables, life_time)
                            }
                            Factor::Sin(val) => match *val {
                                Operation::Calculation((l, op, r)) => {
                                    eval(*l, op, *r, &variables, life_time).sin()
                                }
                                Operation::Identity(val) => match val {
                                    Factor::Variable(l) => {
                                        get_value(Factor::Variable(l), &variables, life_time).sin()
                                    }
                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    };

                    draw.quad()
                        .x_y(position.0, position.1)
                        .w_h(width, height)
                        .color(color);
                }
            },
            Command::ConditionalBlock(branches) => {
                let tmp = eval_conditional_block(branches, &variables, life_time);
                for elem in tmp {
                    match elem {
                        Command::Instantiation(node) => match node {
                            Node::Circle(v) => match v {
                                Operation::Calculation((l, op, r)) => {
                                    let a = eval(*l, op, *r, &variables, life_time);
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(a, a)
                                        .color(color);
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => {
                                        draw.ellipse()
                                            .x_y(position.0, position.1)
                                            .w_h(val, val)
                                            .color(color);
                                    }
                                    Factor::Variable(var_name) => {
                                        draw.ellipse()
                                            .x_y(position.0, position.1)
                                            .w_h(
                                                get_value(
                                                    Factor::Variable(var_name.to_string()),
                                                    &variables,
                                                    life_time,
                                                ),
                                                get_value(
                                                    Factor::Variable(var_name.to_string()),
                                                    &variables,
                                                    life_time,
                                                ),
                                            )
                                            .color(color);
                                    }
                                    Factor::Boolean(_) => unimplemented!(),
                                    Factor::Sin(val) => match *val {
                                        Operation::Calculation((l, op, r)) => {
                                            let val = eval(*l, op, *r, &variables, life_time).sin();
                                            dbg!(val);
                                            draw.ellipse()
                                                .x_y(position.0, position.1)
                                                .w_h(val, val)
                                                .color(color);
                                        }
                                        Operation::Identity(v) => match v {
                                            Factor::Number(value) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(value.sin(), value.sin())
                                                    .color(color);
                                            }
                                            Factor::Variable(v) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .sin(),
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .sin(),
                                                    )
                                                    .color(color);
                                            }
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
                                    Factor::Cos(val) => match *val {
                                        Operation::Calculation((l, op, r)) => {
                                            let val = eval(*l, op, *r, &variables, life_time).cos();
                                            dbg!(val);
                                            draw.ellipse()
                                                .x_y(position.0, position.1)
                                                .w_h(val, val)
                                                .color(color);
                                        }
                                        Operation::Identity(v) => match v {
                                            Factor::Number(value) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(value.cos(), value.cos())
                                                    .color(color);
                                            }
                                            Factor::Variable(v) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .cos(),
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .cos(),
                                                    )
                                                    .color(color);
                                            }
                                            Factor::Time => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(life_time.cos(), life_time.cos())
                                                    .color(color);
                                            }
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
                                    Factor::Time => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            },
                            Node::Square((width, height)) => {
                                let width = match width {
                                    Operation::Calculation((l, op, right)) => {
                                        eval(*l, op, *right, &variables, life_time)
                                    }
                                    Operation::Identity(l) => match l {
                                        Factor::Number(value) => value,
                                        Factor::Variable(var) => {
                                            get_value(Factor::Variable(var), &variables, life_time)
                                        }
                                        _ => unimplemented!(),
                                    },
                                    _ => unimplemented!(),
                                };

                                let height = match height {
                                    Operation::Calculation((l, op, right)) => {
                                        eval(*l, op, *right, &variables, life_time)
                                    }
                                    Operation::Identity(l) => match l {
                                        Factor::Number(value) => value,
                                        Factor::Variable(var) => {
                                            get_value(Factor::Variable(var), &variables, life_time)
                                        }
                                        _ => unimplemented!(),
                                    },
                                    _ => unimplemented!(),
                                };

                                draw.quad()
                                    .x_y(position.0, position.1)
                                    .w_h(width, height)
                                    .color(color);
                            }
                        },
                        Command::Move((x, y)) => {
                            position.0 += match x {
                                Operation::Calculation((l, op, right)) => {
                                    eval(*l, op, *right, &variables, life_time)
                                }
                                Operation::Identity(l) => match l {
                                    Factor::Number(value) => value,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            };
                            position.1 += match y {
                                Operation::Calculation((l, op, right)) => {
                                    eval(*l, op, *right, &variables, life_time)
                                }
                                Operation::Identity(l) => match l {
                                    Factor::Number(value) => value,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            };
                        }
                        Command::ResetMove => {
                            position.0 = 0.0;
                            position.1 = 0.0;
                        }
                        Command::Color((r, g, b)) => {
                            let r = match r {
                                Operation::Calculation((left, op, right)) => {
                                    eval(*left, op, *right, &variables, life_time)
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => val,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => 0.0,
                                },
                                _ => unimplemented!(),
                            };
                            let g = match g {
                                Operation::Calculation((left, op, right)) => {
                                    eval(*left, op, *right, &variables, life_time)
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => val,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => 0.0,
                                },
                                _ => unimplemented!(),
                            };
                            let b = match b {
                                Operation::Calculation((left, op, right)) => {
                                    eval(*left, op, *right, &variables, life_time)
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => val,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => 0.0,
                                },
                                _ => unimplemented!(),
                            };
                            color = rgb(r, g, b);
                        }
                        _ => unimplemented!(),
                    }
                }
            }
            Command::For((times, commands)) => {
                let tmp = eval_for(times, commands, &variables, life_time);
                for elem in tmp {
                    match elem {
                        Command::ConditionalBlock(branches) => {
                            let tmp2 = eval_conditional_block(branches, &variables, life_time);
                            for elem2 in tmp2 {
                                match elem2 {
                                    Command::Instantiation(node) => match node {
                                        Node::Circle(v) => match v {
                                            Operation::Calculation((l, op, r)) => {
                                                let a = eval(*l, op, *r, &variables, life_time);
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(a, a)
                                                    .color(color);
                                            }
                                            Operation::Identity(f) => match f {
                                                Factor::Number(val) => {
                                                    draw.ellipse()
                                                        .x_y(position.0, position.1)
                                                        .w_h(val, val)
                                                        .color(color);
                                                }
                                                Factor::Variable(var_name) => {
                                                    draw.ellipse()
                                                        .x_y(position.0, position.1)
                                                        .w_h(
                                                            get_value(
                                                                Factor::Variable(
                                                                    var_name.to_string(),
                                                                ),
                                                                &variables,
                                                                life_time,
                                                            ),
                                                            get_value(
                                                                Factor::Variable(
                                                                    var_name.to_string(),
                                                                ),
                                                                &variables,
                                                                life_time,
                                                            ),
                                                        )
                                                        .color(color);
                                                }
                                                Factor::Boolean(_) => unimplemented!(),
                                                Factor::Sin(val) => match *val {
                                                    Operation::Calculation((l, op, r)) => {
                                                        let val =
                                                            eval(*l, op, *r, &variables, life_time)
                                                                .sin();
                                                        dbg!(val);
                                                        draw.ellipse()
                                                            .x_y(position.0, position.1)
                                                            .w_h(val, val)
                                                            .color(color);
                                                    }
                                                    Operation::Identity(v) => match v {
                                                        Factor::Number(value) => {
                                                            draw.ellipse()
                                                                .x_y(position.0, position.1)
                                                                .w_h(value.sin(), value.sin())
                                                                .color(color);
                                                        }
                                                        Factor::Variable(v) => {
                                                            draw.ellipse()
                                                                .x_y(position.0, position.1)
                                                                .w_h(
                                                                    get_value(
                                                                        Factor::Variable(
                                                                            v.to_string(),
                                                                        ),
                                                                        &variables,
                                                                        life_time,
                                                                    )
                                                                    .sin(),
                                                                    get_value(
                                                                        Factor::Variable(
                                                                            v.to_string(),
                                                                        ),
                                                                        &variables,
                                                                        life_time,
                                                                    )
                                                                    .sin(),
                                                                )
                                                                .color(color);
                                                        }
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                Factor::Cos(val) => match *val {
                                                    Operation::Calculation((l, op, r)) => {
                                                        let val =
                                                            eval(*l, op, *r, &variables, life_time)
                                                                .cos();
                                                        dbg!(val);
                                                        draw.ellipse()
                                                            .x_y(position.0, position.1)
                                                            .w_h(val, val)
                                                            .color(color);
                                                    }
                                                    Operation::Identity(v) => match v {
                                                        Factor::Number(value) => {
                                                            draw.ellipse()
                                                                .x_y(position.0, position.1)
                                                                .w_h(value.cos(), value.cos())
                                                                .color(color);
                                                        }
                                                        Factor::Variable(v) => {
                                                            draw.ellipse()
                                                                .x_y(position.0, position.1)
                                                                .w_h(
                                                                    get_value(
                                                                        Factor::Variable(
                                                                            v.to_string(),
                                                                        ),
                                                                        &variables,
                                                                        life_time,
                                                                    )
                                                                    .cos(),
                                                                    get_value(
                                                                        Factor::Variable(
                                                                            v.to_string(),
                                                                        ),
                                                                        &variables,
                                                                        life_time,
                                                                    )
                                                                    .cos(),
                                                                )
                                                                .color(color);
                                                        }
                                                        Factor::Time => {
                                                            draw.ellipse()
                                                                .x_y(position.0, position.1)
                                                                .w_h(
                                                                    life_time.cos(),
                                                                    life_time.cos(),
                                                                )
                                                                .color(color);
                                                        }
                                                        _ => unimplemented!(),
                                                    },
                                                    _ => unimplemented!(),
                                                },
                                                Factor::Time => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        },
                                        Node::Square((width, height)) => {
                                            let width = match width {
                                                Operation::Calculation((l, op, right)) => {
                                                    eval(*l, op, *right, &variables, life_time)
                                                }
                                                Operation::Identity(l) => match l {
                                                    Factor::Number(value) => value,
                                                    Factor::Variable(var) => get_value(
                                                        Factor::Variable(var),
                                                        &variables,
                                                        life_time,
                                                    ),
                                                    _ => unimplemented!(),
                                                },
                                                _ => unimplemented!(),
                                            };

                                            let height = match height {
                                                Operation::Calculation((l, op, right)) => {
                                                    eval(*l, op, *right, &variables, life_time)
                                                }
                                                Operation::Identity(l) => match l {
                                                    Factor::Number(value) => value,
                                                    Factor::Variable(var) => get_value(
                                                        Factor::Variable(var),
                                                        &variables,
                                                        life_time,
                                                    ),
                                                    _ => unimplemented!(),
                                                },
                                                _ => unimplemented!(),
                                            };

                                            draw.quad()
                                                .x_y(position.0, position.1)
                                                .w_h(width, height)
                                                .color(color);
                                        }
                                    },
                                    Command::Move((x, y)) => {
                                        position.0 += match x {
                                            Operation::Calculation((l, op, right)) => {
                                                eval(*l, op, *right, &variables, life_time)
                                            }
                                            Operation::Identity(l) => match l {
                                                Factor::Number(value) => value,
                                                Factor::Variable(var) => get_value(
                                                    Factor::Variable(var),
                                                    &variables,
                                                    life_time,
                                                ),
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        };
                                        position.1 += match y {
                                            Operation::Calculation((l, op, right)) => {
                                                eval(*l, op, *right, &variables, life_time)
                                            }
                                            Operation::Identity(l) => match l {
                                                Factor::Number(value) => value,
                                                Factor::Variable(var) => get_value(
                                                    Factor::Variable(var),
                                                    &variables,
                                                    life_time,
                                                ),
                                                _ => unimplemented!(),
                                            },
                                            _ => unimplemented!(),
                                        };
                                    }
                                    Command::ResetMove => {
                                        position.0 = 0.0;
                                        position.1 = 0.0;
                                    }
                                    _ => unimplemented!(),
                                }
                            }
                        }
                        Command::Instantiation(node) => match node {
                            Node::Circle(v) => match v {
                                Operation::Calculation((l, op, r)) => {
                                    let a = eval(*l, op, *r, &variables, life_time);
                                    draw.ellipse()
                                        .x_y(position.0, position.1)
                                        .w_h(a, a)
                                        .color(color);
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => {
                                        draw.ellipse()
                                            .x_y(position.0, position.1)
                                            .w_h(val, val)
                                            .color(color);
                                    }
                                    Factor::Variable(var_name) => {
                                        draw.ellipse()
                                            .x_y(position.0, position.1)
                                            .w_h(
                                                get_value(
                                                    Factor::Variable(var_name.to_string()),
                                                    &variables,
                                                    life_time,
                                                ),
                                                get_value(
                                                    Factor::Variable(var_name.to_string()),
                                                    &variables,
                                                    life_time,
                                                ),
                                            )
                                            .color(color);
                                    }
                                    Factor::Boolean(_) => unimplemented!(),
                                    Factor::Sin(val) => match *val {
                                        Operation::Calculation((l, op, r)) => {
                                            let val = eval(*l, op, *r, &variables, life_time).sin();
                                            draw.ellipse()
                                                .x_y(position.0, position.1)
                                                .w_h(val, val)
                                                .color(color);
                                        }
                                        Operation::Identity(v) => match v {
                                            Factor::Number(value) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(value.sin(), value.sin())
                                                    .color(color);
                                            }
                                            Factor::Variable(v) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .sin(),
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .sin(),
                                                    )
                                                    .color(color);
                                            }
                                            Factor::Time => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(life_time.sin(), life_time.sin())
                                                    .color(color);
                                            }
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
                                    Factor::Cos(val) => match *val {
                                        Operation::Calculation((l, op, r)) => {
                                            let val = eval(*l, op, *r, &variables, life_time).cos();
                                            dbg!(val);
                                            draw.ellipse()
                                                .x_y(position.0, position.1)
                                                .w_h(val, val)
                                                .color(color);
                                        }
                                        Operation::Identity(v) => match v {
                                            Factor::Number(value) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(value.cos(), value.cos())
                                                    .color(color);
                                            }
                                            Factor::Variable(v) => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .cos(),
                                                        get_value(
                                                            Factor::Variable(v.to_string()),
                                                            &variables,
                                                            life_time,
                                                        )
                                                        .cos(),
                                                    )
                                                    .color(color);
                                            }
                                            Factor::Time => {
                                                draw.ellipse()
                                                    .x_y(position.0, position.1)
                                                    .w_h(life_time.cos(), life_time.cos())
                                                    .color(color);
                                            }
                                            _ => unimplemented!(),
                                        },
                                        _ => unimplemented!(),
                                    },
                                    Factor::Time => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            },
                            Node::Square((width, height)) => {
                                let width = match width {
                                    Operation::Calculation((l, op, right)) => {
                                        eval(*l, op, *right, &variables, life_time)
                                    }
                                    Operation::Identity(l) => match l {
                                        Factor::Number(value) => value,
                                        Factor::Variable(var) => {
                                            get_value(Factor::Variable(var), &variables, life_time)
                                        }
                                        _ => unimplemented!(),
                                    },
                                    _ => unimplemented!(),
                                };

                                let height = match height {
                                    Operation::Calculation((l, op, right)) => {
                                        eval(*l, op, *right, &variables, life_time)
                                    }
                                    Operation::Identity(l) => match l {
                                        Factor::Number(value) => value,
                                        Factor::Variable(var) => {
                                            get_value(Factor::Variable(var), &variables, life_time)
                                        }
                                        _ => unimplemented!(),
                                    },
                                    _ => unimplemented!(),
                                };

                                draw.quad()
                                    .x_y(position.0, position.1)
                                    .w_h(width, height)
                                    .color(color);
                            }
                        },
                        Command::Move((x, y)) => {
                            position.0 += match x {
                                Operation::Calculation((l, op, right)) => {
                                    eval(*l, op, *right, &variables, life_time)
                                }
                                Operation::Identity(l) => match l {
                                    Factor::Number(value) => value,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            };
                            position.1 += match y {
                                Operation::Calculation((l, op, right)) => {
                                    eval(*l, op, *right, &variables, life_time)
                                }
                                Operation::Identity(l) => match l {
                                    Factor::Number(value) => value,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            };
                        }
                        Command::ResetMove => {
                            position.0 = 0.0;
                            position.1 = 0.0;
                        }
                        Command::Color((r, g, b)) => {
                            let r = match r {
                                Operation::Calculation((left, op, right)) => {
                                    eval(*left, op, *right, &variables, life_time)
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => val,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => 0.0,
                                },
                                _ => unimplemented!(),
                            };
                            let g = match g {
                                Operation::Calculation((left, op, right)) => {
                                    eval(*left, op, *right, &variables, life_time)
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => val,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => 0.0,
                                },
                                _ => unimplemented!(),
                            };
                            let b = match b {
                                Operation::Calculation((left, op, right)) => {
                                    eval(*left, op, *right, &variables, life_time)
                                }
                                Operation::Identity(f) => match f {
                                    Factor::Number(val) => val,
                                    Factor::Variable(var) => {
                                        get_value(Factor::Variable(var), &variables, life_time)
                                    }
                                    _ => 0.0,
                                },
                                _ => unimplemented!(),
                            };
                            color = rgb(r, g, b);
                        }
                        Command::Declaration(d) => {
                            let (name, value) = declare_variable(d, &variables, 0.0);
                            variables.insert(name, value);
                        }
                        _ => unimplemented!(),
                    }
                }
            }
            Command::Move((x, y)) => {
                position.0 += match x {
                    Operation::Calculation((l, op, right)) => {
                        eval(*l, op, *right, &variables, life_time)
                    }
                    Operation::Identity(l) => match l {
                        Factor::Number(value) => value,
                        Factor::Variable(var) => {
                            get_value(Factor::Variable(var), &variables, life_time)
                        }
                        Factor::Sin(op) => match *op {
                            Operation::Calculation((l, op, right)) => {
                                eval(*l, op, *right, &variables, life_time)
                            }
                            Operation::Identity(val) => match val {
                                Factor::Time => life_time.sin(),
                                _ => unimplemented!(),
                            },
                            _ => {
                                println!("istruzione move non valida..");
                                0.0
                            }
                        },
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                };
                position.1 += match y {
                    Operation::Calculation((l, op, right)) => {
                        eval(*l, op, *right, &variables, life_time)
                    }
                    Operation::Identity(l) => match l {
                        Factor::Number(value) => value,
                        Factor::Variable(var) => {
                            get_value(Factor::Variable(var), &variables, life_time)
                        }
                        Factor::Sin(op) => match *op {
                            Operation::Calculation((l, op, right)) => {
                                eval(*l, op, *right, &variables, life_time)
                            }
                            Operation::Identity(val) => match val {
                                Factor::Time => life_time.sin(),
                                _ => unimplemented!(),
                            },
                            _ => {
                                println!("istruzione move non valida..");
                                0.0
                            }
                        },
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                };
            }
            Command::ResetMove => {
                position.0 = 0.0;
                position.1 = 0.0;
            }
            Command::Color((r, g, b)) => {
                let r = match r {
                    Operation::Calculation((left, op, right)) => {
                        eval(*left, op, *right, &variables, life_time)
                    }
                    Operation::Identity(f) => match f {
                        Factor::Number(val) => val,
                        Factor::Variable(var) => {
                            get_value(Factor::Variable(var), &variables, life_time)
                        }
                        _ => 0.0,
                    },
                    _ => unimplemented!(),
                };
                let g = match g {
                    Operation::Calculation((left, op, right)) => {
                        eval(*left, op, *right, &variables, life_time)
                    }
                    Operation::Identity(f) => match f {
                        Factor::Number(val) => val,
                        Factor::Variable(var) => {
                            get_value(Factor::Variable(var), &variables, life_time)
                        }
                        _ => 0.0,
                    },
                    _ => unimplemented!(),
                };
                let b = match b {
                    Operation::Calculation((left, op, right)) => {
                        eval(*left, op, *right, &variables, life_time)
                    }
                    Operation::Identity(f) => match f {
                        Factor::Number(val) => val,
                        Factor::Variable(var) => {
                            get_value(Factor::Variable(var), &variables, life_time)
                        }
                        _ => 0.0,
                    },
                    _ => unimplemented!(),
                };
                color = rgb(r, g, b);
            }
        }
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            // when user press Lcontrol
            if key == nannou::prelude::Key::LControl {
                //model.variables = HashMap::new();
                if let Ok((remaining, ast)) = parser::parser(&model.text_edit) {
                    println!("{:#?}", parser::parser(&model.text_edit));
                    if remaining == "" {
                        model.instructions = ast;
                    } else {
                        println!("{}", "CAN'T UPDATE ABSTRACT SYNTAX TREE".red().bold());
                        println!("{:#?}", parser::parser(&model.text_edit));
                        println!("{} {}", "error:".red().bold(), &remaining.red().bold());
                    }
                }
            }
        }
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            raw: _,
            simple: _,
        } => {}
        Event::DeviceEvent(_device_id, _event) => {}
        Event::Update(_dt) => {}
        Event::Awakened => {}
        Event::Suspended(_b) => {}
    }
}

// all the events must be implemented!

fn raw_window_event(_app: &App, _model: &mut Model, _event: nannou::winit::WindowEvent) {}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Backslash => {
            if model.stream.is_playing() {
                model.stream.pause().unwrap();
            } else {
                model.stream.play().unwrap();
            }
        }
        _ => {}
    }
}

fn key_released(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_wheel(_app: &App, _model: &mut Model, _dt: MouseScrollDelta, _phase: TouchPhase) {}

fn mouse_entered(_app: &App, _model: &mut Model) {}

fn mouse_exited(_app: &App, _model: &mut Model) {}

fn window_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn window_resized(_app: &App, _model: &mut Model, _dim: Vector2) {}

fn window_focused(_app: &App, _model: &mut Model) {}

fn window_unfocused(_app: &App, _model: &mut Model) {}

fn window_closed(_app: &App, _model: &mut Model) {}
