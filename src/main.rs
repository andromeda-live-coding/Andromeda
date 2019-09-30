mod parser;
use colored::Colorize;
use nannou::app::Draw;
use nannou::prelude::*;
use nannou::ui::prelude::*;
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
                                _ => unimplemented!(),
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
                _ => unimplemented!(),
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

// BUGS TO SOLVE
// true || false are parsed as variables so the command **true: 71.7** will be parsed
// it's not possible to **y: -x**
//

fn main() {
    let content = "x: 23\n y: -x";
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
}

struct Ids {
    text_edit: widget::Id,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(3));
    app.new_window()
        .with_dimensions(1280, 1024)
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
    };

    // Init our variables
    let text_edit = "".to_string();
    //let variables = HashMap::new();
    let instructions: Vec<Command> = Vec::new();

    Model {
        ui,
        ids,
        text_edit,
        //variables,
        instructions,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

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
    let mut position: (f32, f32) = (0.0, 0.0);
    let _std_value = 10.0;
    draw.background().rgb(0.39, 0.39, 0.39);
    let color = rgb(1.0, 1.0, 1.0);
    for x in model.instructions.clone() {
        match x {
            Command::Declaration((name, value)) => {
                let (name, value) = declare_variable((name.to_string(), value.clone()), &variables);
                variables.insert(name, value);
            }
            Command::Instantiation(nd) => match nd {
                Node::Circle(v) => match v {
                    Operation::Calculation((l, op, r)) => {
                        let a = eval(*l, op, *r, &variables);
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
                                    get_value(Factor::Variable(var_name.to_string()), &variables),
                                    get_value(Factor::Variable(var_name.to_string()), &variables),
                                )
                                .color(color);
                        }
                        Factor::Boolean(_) => unimplemented!(),
                    },
                    _ => unimplemented!(),
                },
                Node::Square((width, height)) => {
                    let width = match width {
                        Operation::Calculation((l, op, right)) => eval(*l, op, *right, &variables),
                        Operation::Identity(l) => match l {
                            Factor::Number(value) => value,
                            Factor::Variable(var) => get_value(Factor::Variable(var), &variables),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    };

                    let height = match height {
                        Operation::Calculation((l, op, right)) => eval(*l, op, *right, &variables),
                        Operation::Identity(l) => match l {
                            Factor::Number(value) => value,
                            Factor::Variable(var) => get_value(Factor::Variable(var), &variables),
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
                let tmp = eval_conditional_block(branches, &variables);
                for elem in tmp {
                    //nodes.push(elem);
                }
            }
            Command::For((times, commands)) => {
                let tmp = eval_for(times, commands, &variables);
                for elem in tmp {
                    //nodes.push(elem);
                }
            }
            Command::Move((x, y)) => {
                position.0 = match x {
                    Operation::Calculation((l, op, right)) => eval(*l, op, *right, &variables),
                    Operation::Identity(l) => match l {
                        Factor::Number(value) => value,
                        Factor::Variable(var) => get_value(Factor::Variable(var), &variables),
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                };
                position.1 = match y {
                    Operation::Calculation((l, op, right)) => eval(*l, op, *right, &variables),
                    Operation::Identity(l) => match l {
                        Factor::Number(value) => value,
                        Factor::Variable(var) => get_value(Factor::Variable(var), &variables),
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                };
            }
            Command::ResetMove => {
                position.0 = 0.0;
                position.1 = 0.0;
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

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}

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
