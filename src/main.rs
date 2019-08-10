mod parser;
// nannou
use nannou::app::Draw;
use nannou::prelude::*;
use nannou::ui::prelude::*;
// std
use std::collections::HashMap;
// Atom
use colored::*;
use parser::Command;
fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    position: Point2,
    text_edit: String,
    variables: HashMap<String, f32>,
    instructions: Vec<Command>,
}

struct Ids {
    text_edit: widget::Id,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(3));
    app.new_window()
        .with_dimensions(720, 720)
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
        .insert_from_file("timesnewarial.ttf")
        .unwrap();
    // Generate some ids for our widgets.
    let ids = Ids {
        text_edit: ui.generate_widget_id(),
    };

    // Init our variables
    let position = pt2(0.0, 0.0);
    let text_edit = "bufu".to_owned();
    let variables = HashMap::new();
    let instructions: Vec<Command> = Vec::new();

    Model {
        ui,
        ids,
        position,
        text_edit,
        variables,
        instructions,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    for edit in widget::TextEdit::new(&model.text_edit)
        .color(color::WHITE)
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
    let mut position: (f32, f32) = (0.0, 0.0);
    let std_value = 10.0;
    draw.background().rgb(0.39, 0.39, 0.39);
    let mut color = rgb(1.0, 1.0, 1.0);
    for x in &model.instructions {
        match x {
            Command::DeclareVariable((_, _)) => {}
            // actually implementing box space0 alpha1 pattern (to recognize "box x", or "box y")
            // it should also cover other patterns
            Command::DrawShapeWVariable((shape, y)) => {
                if let Some(val) = model.variables.get(y) {
                    c(
                        &draw,
                        shape.as_ref(),
                        &position.0,
                        &position.1,
                        val,
                        val,
                        (color.red, color.green, color.blue),
                    );
                }
            }
            // ?????????????????????????????
            // Here we assing to position the value of instruction move so we can
            // draw all our object in the right position
            //
            Command::Move(bufu) => {
                position.0 = position.0 + bufu.0;
                position.1 = position.1 + bufu.1;
            }
            Command::DrawShapeWf32((shape, f32value)) => {
                c(
                    &draw,
                    shape.as_ref(),
                    &position.0,
                    &position.1,
                    f32value,
                    f32value,
                    (color.red, color.green, color.blue),
                );
            }
            Command::Color((r, g, b)) => (color = rgb(*r, *g, *b)),
            Command::DrawShape(shape) => {
                c(
                    &draw,
                    shape.as_ref(),
                    &position.0,
                    &position.1,
                    &std_value,
                    &std_value,
                    (color.red, color.green, color.blue),
                );
            }
            Command::DrawShapeWf32f32((shape, val1, val2)) => {
                c(
                    &draw,
                    shape.as_ref(),
                    &position.0,
                    &position.1,
                    val1,
                    val2,
                    (color.red, color.green, color.blue),
                );
            }
            Command::DrawShape2Variables((shape, var1, var2)) => {
                if let Some(val1) = model.variables.get(var1) {
                    if let Some(val2) = model.variables.get(var2) {
                        c(
                            &draw,
                            shape.as_ref(),
                            &position.0,
                            &position.1,
                            val1,
                            val2,
                            (color.red, color.green, color.blue),
                        );
                    }
                }
            }
            Command::For((times, v)) => {
                let mut tmpHashMap = HashMap::new();
                let times = times.parse::<i32>().unwrap();
                for n in 0..times {
                    for cmd in v {
                        match cmd {
                            Command::DeclareVariable((key, value)) => {
                                tmpHashMap.insert(key.to_string(), *value);
                            }
                            Command::DrawShapeWVariable((shape, y)) => {
                                if let Some(val) = model.variables.get(y) {
                                    c(
                                        &draw,
                                        shape.as_ref(),
                                        &position.0,
                                        &position.1,
                                        val,
                                        val,
                                        (color.red, color.green, color.blue),
                                    );
                                }
                            }

                            Command::Move((x, y)) => {
                                position.0 = position.0 + x;
                                position.1 = position.1 + y;
                            }
                            Command::DrawShapeWf32((shape, f32value)) => {
                                c(
                                    &draw,
                                    shape.as_ref(),
                                    &position.0,
                                    &position.1,
                                    f32value,
                                    f32value,
                                    (color.red, color.green, color.blue),
                                );
                            }
                            Command::Color(_) => (),
                            Command::DrawShape(_) => (),
                            Command::DrawShapeWf32f32(_) => (),
                            Command::DrawShape2Variables(_) => (),
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    //
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();

    fn c(d: &Draw, shape: &str, x: &f32, y: &f32, val1: &f32, val2: &f32, color: (f32, f32, f32)) {
        match shape {
            "box" => {
                d.quad()
                    .x_y(*x, *y)
                    .w_h(*val1, *val2)
                    .color(rgb(color.0, color.1, color.2));
            }
            "circle" => {
                d.ellipse()
                    .x_y(*x, *y)
                    .w_h(*val1, *val2)
                    .color(rgb(color.0, color.1, color.2));
            }
            _ => (),
        }
    }
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            if key == nannou::prelude::Key::LControl {
                if let Ok((remaining, ast)) = parser::parser(&model.text_edit) {
                    println!("{:#?}", parser::parser(&model.text_edit));

                    // updating AST only if parser success and there isn't nothing left to parse
                    if remaining == "" {
                        model.instructions = ast;
                        for x in model.instructions.to_owned() {
                            match x {
                                Command::DeclareVariable((key, value)) => {
                                    model.variables.insert(key, value);
                                }
                                Command::DrawShapeWVariable(_) => {}

                                Command::Move((x, y)) => (model.position = pt2(x, y)),
                                Command::DrawShapeWf32(_) => (),
                                Command::Color(_) => (),
                                Command::DrawShape(_) => (),
                                Command::DrawShapeWf32f32(_) => (),
                                Command::DrawShape2Variables(_) => (),
                                Command::For((times, v)) => {}
                            }
                        }
                    } else {
                        println!("not updating AST");
                        println!("{:#?}", parser::parser(&model.text_edit));
                        println!("error: {}", &remaining.red());
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
