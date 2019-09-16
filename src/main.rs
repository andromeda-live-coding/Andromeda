mod parser;
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
                Operation::Calculation((_first2, _op2, _second2)) => {
                    eval(_first2, _op2, _second2, variables)
                }
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
    let content = "x: 2\ny: x\nz: ((x + y + 2 * (9-2) + 12 - (9*2+ (2) )) * x     ) * 100 ";
    // TODO: Try with this content
    let _content2 = "x: 2\ny: x\nz: x + 2 + 3";
    //let content3 = "x: 2\ny: x\nz: x + y + 1 +1 ";
    let (_, ast) = parser(content).unwrap();
    dbg!(ast.clone());
    let mut variables: HashMap<String, f32> = HashMap::new();
    for expression in ast {
        match expression {
            Command::Declaration(declaration) => {
                let (name, value) = declare_variable(declaration, &variables);
                variables.insert(name, value);
            }
        }
    }
    dbg!(variables.clone());
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
    let draw = app.draw();
    let position: (f32, f32) = (0.0, 0.0);
    let std_value = 10.0;
    draw.background().rgb(0.71, 0.12, 0.71);
    let color = rgb(1.0, 1.0, 1.0);
    for x in &model.instructions {}
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
                        let mut _semantic_analysis = true;
                        for x in ast.to_owned() {
                            match x {
                                //Command::DeclareVariable((key, value)) => {
                                //model.variables.insert(key, value);

                                //Command::DrawShapeWf32((shape, val1, val2)) => {}
                                _ => (),
                            }
                        }
                        // updating AST only if parser success and there isn't nothing left to parse
                        if _semantic_analysis {
                            model.instructions = ast;
                        }
                    } else {
                        println!("bufu");
                        //println!("{}", "CAN'T UPDATE ABSTRACT SYNTAX TREE".red().bold());
                        //println!("{:#?}", parser::parser(&model.text_edit));
                        //println!("{} {}", "error:".red().bold(), &remaining.red().bold());
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
