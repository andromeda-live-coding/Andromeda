mod parser;
// nannou
use nannou::prelude::*;
use nannou::ui::prelude::*;
// bufu
use std::collections::HashMap;
// Atom
use parser::Atom;

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
    color: Rgb,
    position: Point2,
    text_edit: String,
    variables: HashMap<String, f32>,
    instructions: Vec<Atom>,
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
    let color = rgb(0.9, 0.4, 0.3);
    let text_edit = "bufu".to_owned();
    let variables = HashMap::new();
    let instructions: Vec<Atom> = Vec::new();

    Model {
        ui,
        ids,
        position,
        color,
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
    let mut pos: (f32, f32) = (0.0, 0.0);
    draw.background().rgb(0.39, 0.39, 0.39);
    for x in &model.instructions {
        match x {
            Atom::Vval((_, _)) => {}
            Atom::Num(_) => (),
            // actually implementing box space0 alpha1 pattern (to recognize "box x", or "box y")
            // it should also cover other patterns
            Atom::Keyword((x, y)) => {
                if let Some(val) = model.variables.get(y) {
                    match x.as_ref() {
                        "box" => {
                            draw.quad()
                                .x_y(pos.0, pos.1)
                                .w_h(*val, *val)
                                .color(model.color);
                        }
                        "circle" => {
                            draw.ellipse()
                                .x_y(pos.0, pos.1)
                                .w_h(*val, *val)
                                .color(model.color);
                        }
                        _ => (),
                    }
                }
            }
            Atom::Move(bufu) => (pos = *bufu),
        }
    }
    //////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////
    // TODO: parse the text only on keyboard input
    //
    //
    //
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            if key == nannou::prelude::Key::LControl {
                println!("{:?}", parser::parser(&model.text_edit));
                match parser::parser(&model.text_edit) {
                    Ok(ast) => {
                        model.instructions = ast.1;
                        for x in model.instructions.to_owned() {
                            match x {
                                Atom::Vval((key, value)) => {
                                    model.variables.insert(key, value);
                                }
                                Atom::Num(_) => (),

                                Atom::Keyword((_, _)) => {}

                                Atom::Move((x, y)) => (model.position = pt2(x, y)),
                            }
                        }
                    }
                    Err(_) => (),
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
