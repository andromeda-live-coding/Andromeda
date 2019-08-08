// nom
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0, one_of, space0};
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::many0;
use nom::number::complete::float;
use nom::sequence::{preceded, tuple};
use nom::IResult;
// nannou
use nannou::prelude::*;
use nannou::ui::prelude::*;
// bufu
use std::collections::HashMap;

fn main() {
    let x = parser(
        "x: 10
                y: 6
                                
                                    
                                    
                                        
                                        box x
                                        
                                        
                                        bufu",
    );
    match x {
        Ok(ast) => {
            for element in ast.1 {
                println!("{:?}", element);
            }
        }
        Err(_) => (),
    }

    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    resolution: usize,
    scale: f32,
    rotation: f32,
    color: Rgb,
    position: Point2,
    text_edit: String,
    text: String,
}

struct Ids {
    resolution: widget::Id,
    scale: widget::Id,
    rotation: widget::Id,
    random_color: widget::Id,
    position: widget::Id,
    text_edit: widget::Id,
    text: widget::Id,
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
        resolution: ui.generate_widget_id(),
        scale: ui.generate_widget_id(),
        rotation: ui.generate_widget_id(),
        random_color: ui.generate_widget_id(),
        position: ui.generate_widget_id(),
        text_edit: ui.generate_widget_id(),
        text: ui.generate_widget_id(),
    };

    // Init our variables
    let resolution = 6;
    let scale = 200.0;
    let rotation = 0.0;
    let position = pt2(0.0, 0.0);
    let color = rgb(0.9, 0.4, 0.3);
    let text_edit = "bufu".to_owned();
    let text = "bufu".to_owned();
    Model {
        ui,
        ids,
        resolution,
        scale,
        rotation,
        position,
        color,
        text_edit,
        text,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.8)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.resolution as f32, 3.0, 15.0)
        .top_left_with_margin(20.0)
        .label("Resolution")
        .set(model.ids.resolution, ui)
    {
        model.resolution = value as usize;
    }

    for value in slider(model.scale, 10.0, 500.0)
        .down(10.0)
        .label("Scale")
        .set(model.ids.scale, ui)
    {
        model.scale = value;
    }

    for value in slider(model.rotation, -PI, PI)
        .down(10.0)
        .label("Rotation")
        .set(model.ids.rotation, ui)
    {
        model.rotation = value;
    }

    for _click in widget::Button::new()
        .top_left_with_margin(20.0)
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Random Color")
        .label_font_size(15)
        .rgb(1.0, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.random_color, ui)
    {
        model.color = rgb(random(), random(), random())
    }

    for (x, y) in widget::XYPad::new(
        model.position.x,
        -200.0,
        200.0,
        model.position.y,
        -200.0,
        200.0,
    )
    .down(10.0)
    .w_h(200.0, 200.0)
    .label("Position")
    .label_font_size(15)
    .rgb(0.3, 0.3, 0.3)
    .label_rgb(1.0, 1.0, 1.0)
    .border(0.0)
    .set(model.ids.position, ui)
    {
        model.position = Point2::new(x, y);
    }

    for edit in widget::TextEdit::new(&model.text_edit)
        .color(color::WHITE)
        .down(10.0)
        .line_spacing(2.5)
        .restrict_to_height(false)
        .set(model.ids.text_edit, ui)
    {
        model.text_edit = edit;
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // how to declare an HashMap
    let mut variables: HashMap<String, f32> = HashMap::new();
    let draw = app.draw();
    //let t = app.time;
    draw.background().rgb(0.09, 0.09, 0.09);

    draw.ellipse()
        .xy(model.position)
        .radius(model.scale)
        .resolution(model.resolution)
        .rotate(model.rotation)
        .color(model.color);

    let text_to_parse = parser(&model.text_edit);
    match text_to_parse {
        Ok(ast) => {
            for x in ast.1 {
                match x {
                    Atom::Vval((key, value)) => {
                        //draw.quad().w_h(value, value);
                        variables.insert(key, value);
                    }
                    Atom::Bool(val_bool) => (),
                    Atom::Num(val_f32) => (),
                    Atom::Keyword((x, y)) => {
                        if let Some(val) = variables.get(&y) {
                            draw.quad().w_h(*val, *val);
                        }
                    }
                }
            }
        }
        Err(err) => (),
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            if key == nannou::prelude::Key::R {
                println!("R");
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

fn window_focused(_app: &App, _model: &mut Model) {
    println!("focused!");
}

fn window_unfocused(_app: &App, _model: &mut Model) {
    println!("unfocused!");
}

fn window_closed(_app: &App, _model: &mut Model) {}

fn variable_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    map(alpha1, |x: &str| x)(input)
}

// it recognizes pattern **x: f32**
fn declare_variable(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    map(
        tuple((variable_parser, one_of(":="), space0, float)),
        |(name, _, _, value)| Atom::Vval((name.to_string(), value)),
    )(input)
}

// it recognizes pattern **box alpha** (where alpha is a variable)
fn declare_box_with_variable(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    map(
        tuple((tag("box"), space0, variable_parser)),
        |(_, _, value)| Atom::Keyword(("box".to_string(), value.to_string())),
    )(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Atom>, VerboseError<&str>> {
    many0(alt((
        preceded(multispace0, declare_variable),
        preceded(multispace0, declare_box_with_variable),
    )))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(f32),
    Keyword((String, String)),
    Bool(bool),
    Vval((String, f32)),
}
