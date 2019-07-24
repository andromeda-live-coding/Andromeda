use nannou::prelude::*;
use nannou::ui::prelude::*;
fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
struct Model {
    ui: Ui,
    ids: Ids,
    resolution: usize,
    scale: f32,
    rotation: f32,
    color: i32, // Rgb
    position: Point2,
}

struct Ids {
    resolution: widget::Id,
    scale: widget::Id,
    rotation: widget::Id,
    random_color: widget::Id,
    position: widget::Id,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(3));

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();
    ui.fonts_mut().insert_from_file("timesnewarial.ttf").unwrap();
    // Generate some ids for our widgets.
    let ids = Ids {
        resolution: ui.generate_widget_id(),
        scale: ui.generate_widget_id(),
        rotation: ui.generate_widget_id(),
        random_color: ui.generate_widget_id(),
        position: ui.generate_widget_id(),
    };

    // Init our variables
    let resolution = 6;
    let scale = 200.0;
    let rotation = 0.0;
    let position = pt2(0.0, 0.0);
    let color = 22; // rgb(0.5, 0.4, 0.3)
    let a = Rgb{
        red: 1.0,
        green: 0.3,
        blue: 0.7,
    };
    Model {
        ui,
        ids,
        resolution,
        scale,
        rotation,
        position,
        color,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
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
        .down(100.0)
        .w_h(200.0, 60.0)
        .label("Random Color")
        .label_font_size(15)
        .rgb(1.0, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.random_color, ui)
    {
        model.color = 22; // rgb(random(), random(), random())
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
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    draw.background().rgb(0.02, 0.02, 0.02);

    draw.ellipse()
        .xy(model.position)
        .radius(model.scale)
        .resolution(model.resolution)
        .rotate(model.rotation)
        .color(RED); // model.color

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
    frame
}