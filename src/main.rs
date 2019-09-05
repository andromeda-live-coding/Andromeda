mod parser;
// nannou
use nannou::app::Draw;
use nannou::prelude::*;
use nannou::ui::prelude::*;
// std
use std::collections::HashMap;
// Atom
use colored::Colorize;
use parser::Command;
fn main() {
    let p = crate::parser::expr("+7-2*(9*8)");
    println!("{:#?}", p);
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
        .insert_from_file("F25_Bank_Printer.ttf")
        .unwrap();
    // Generate some ids for our widgets.
    let ids = Ids {
        text_edit: ui.generate_widget_id(),
    };

    // Init our variables
    let text_edit = "".to_string();
    let variables = HashMap::new();
    let instructions: Vec<Command> = Vec::new();

    Model {
        ui,
        ids,
        text_edit,
        variables,
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
    draw.background().rgb(0.39, 0.39, 0.39);
    let color = rgb(1.0, 1.0, 1.0);
    for x in &model.instructions {
        match x {
            Command::DeclareVariable((_, _)) => {}
            Command::DrawShapeWVariable((shape, y)) => {
                if let Some(val) = model.variables.get(y) {
                    draw_shape(
                        &draw,
                        shape.as_ref(),
                        position.0,
                        position.1,
                        *val,
                        *val,
                        (color.red, color.green, color.blue),
                    );
                }
            }
            Command::DrawShape(shape) => {
                draw_shape(
                    &draw,
                    shape.as_ref(),
                    position.0,
                    position.1,
                    std_value,
                    std_value,
                    (color.red, color.green, color.blue),
                );
            }
            Command::DrawShapeWf32f32((shape, val1, val2)) => {
                draw_shape(
                    &draw,
                    shape.as_ref(),
                    position.0,
                    position.1,
                    *val1,
                    *val2,
                    (color.red, color.green, color.blue),
                );
            }
            Command::DrawShape2Variables((shape, var1, var2)) => {
                if let Some(val1) = model.variables.get(var1) {
                    if let Some(val2) = model.variables.get(var2) {
                        draw_shape(
                            &draw,
                            shape.as_ref(),
                            position.0,
                            position.1,
                            *val1,
                            *val2,
                            (color.red, color.green, color.blue),
                        );
                    }
                }
            }

            Command::DrawShapeVf32((shape, var, val2)) => {
                if let Some(val1) = model.variables.get(var) {
                    draw_shape(
                        &draw,
                        shape.as_ref(),
                        position.0,
                        position.1,
                        *val1,
                        *val2,
                        (color.red, color.green, color.blue),
                    );
                }
            }

            Command::DrawShapef32V((shape, val1, var)) => {
                if let Some(val2) = model.variables.get(var) {
                    draw_shape(
                        &draw,
                        shape.as_ref(),
                        position.0,
                        position.1,
                        *val1,
                        *val2,
                        (color.red, color.green, color.blue),
                    );
                }
            }
        }
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();

    fn draw_shape(
        c: &Draw,
        shape: &str,
        x: f32,
        y: f32,
        val1: f32,
        val2: f32,
        color: (f32, f32, f32),
    ) {
        match shape {
            "box" => {
                c.quad()
                    .x_y(x, y)
                    .w_h(val1, val2)
                    .color(rgb(color.0, color.1, color.2));
            }
            "circle" => {
                c.ellipse()
                    .x_y(x, y)
                    .w_h(val1, val2)
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
                model.variables = HashMap::new();
                if let Ok((remaining, ast)) = parser::parser(&model.text_edit) {
                    println!("{:#?}", parser::parser(&model.text_edit));
                    if remaining == "" {
                        let mut semantic_analysis = true;
                        for x in ast.to_owned() {
                            match x {
                                Command::DeclareVariable((key, value)) => {
                                    model.variables.insert(key, value);
                                }

                                Command::DrawShape2Variables((_, var1, var2)) => {
                                    if model.variables.get(&var1).is_some() {
                                        if model.variables.get(&var2).is_some() {
                                        } else {
                                            semantic_analysis = false;
                                            println!(
                                                "{} {}",
                                                "error on variables:".red(),
                                                var2.red()
                                            );
                                        }
                                    } else {
                                        semantic_analysis = false;
                                        println!("{} {}", "error on variables:".red(), var1.red());
                                    }
                                }

                                Command::DrawShapeVf32((_, var, _)) => {
                                    if model.variables.get(&var).is_some() {
                                    } else {
                                        semantic_analysis = false;
                                        println!("{} {}", "error on variable:".red(), var.red());
                                    }
                                }

                                Command::DrawShapeWVariable((_, var)) => {
                                    if model.variables.get(&var).is_some() {
                                    } else {
                                        semantic_analysis = false;
                                        println!("{} {}", "error on variable:".red(), var.red());
                                    }
                                }

                                Command::DrawShapef32V((_, _, var)) => {
                                    if model.variables.get(&var).is_some() {
                                    } else {
                                        semantic_analysis = false;
                                        println!("{} {}", "error on variables:".red(), var.red());
                                    }
                                }
                                _ => (),
                            }
                        }
                        // updating AST only if parser success and there isn't nothing left to parse
                        if semantic_analysis {
                            model.instructions = ast;
                        }
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
