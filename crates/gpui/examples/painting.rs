use gpui::{
    Application, Background, Bounds, ColorSpace, Context, MouseDownEvent, Path, PathBuilder,
    PathStyle, Pixels, Point, Render, SharedString, StrokeOptions, Window, WindowBounds,
    WindowOptions, canvas, div, linear_color_stop, linear_gradient, point, prelude::*, px, rgb,
    size,
};

const DEFAULT_WINDOW_WIDTH: Pixels = px(1024.0);
const DEFAULT_WINDOW_HEIGHT: Pixels = px(768.0);

struct PaintingViewer {
    default_lines: Vec<(Path<Pixels>, Background)>,
    lines: Vec<Vec<Point<Pixels>>>,
    start: Point<Pixels>,
    dashed: bool,
    _painting: bool,
}

impl PaintingViewer {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let mut lines = vec![];

        // draw a Rust logo
        let mut builder = lyon::path::Path::svg_builder();
        lyon::extra::rust_logo::build_logo_path(&mut builder);
        // move down the Path
        let mut builder: PathBuilder = builder.into();
        builder.translate(point(px(10.), px(100.)));
        builder.scale(0.9);
        let path = builder.build().unwrap();
        lines.push((path, gpui::black().into()));

        // draw a lightening bolt ⚡
        let mut builder = PathBuilder::fill();
        builder.add_polygon(
            &[
                point(px(150.), px(200.)),
                point(px(200.), px(125.)),
                point(px(200.), px(175.)),
                point(px(250.), px(100.)),
            ],
            false,
        );
        let path = builder.build().unwrap();
        lines.push((path, rgb(0x1d4ed8).into()));

        // draw a ⭐
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(350.), px(100.)));
        builder.line_to(point(px(370.), px(160.)));
        builder.line_to(point(px(430.), px(160.)));
        builder.line_to(point(px(380.), px(200.)));
        builder.line_to(point(px(400.), px(260.)));
        builder.line_to(point(px(350.), px(220.)));
        builder.line_to(point(px(300.), px(260.)));
        builder.line_to(point(px(320.), px(200.)));
        builder.line_to(point(px(270.), px(160.)));
        builder.line_to(point(px(330.), px(160.)));
        builder.line_to(point(px(350.), px(100.)));
        let path = builder.build().unwrap();
        lines.push((
            path,
            linear_gradient(
                180.,
                linear_color_stop(rgb(0xFACC15), 0.7),
                linear_color_stop(rgb(0xD56D0C), 1.),
            )
            .color_space(ColorSpace::Oklab),
        ));

        // draw linear gradient
        let square_bounds = Bounds {
            origin: point(px(450.), px(100.)),
            size: size(px(200.), px(80.)),
        };
        let height = square_bounds.size.height;
        let horizontal_offset = height;
        let vertical_offset = px(30.);
        let mut builder = PathBuilder::fill();
        builder.move_to(square_bounds.bottom_left());
        builder.curve_to(
            square_bounds.origin + point(horizontal_offset, vertical_offset),
            square_bounds.origin + point(px(0.0), vertical_offset),
        );
        builder.line_to(square_bounds.top_right() + point(-horizontal_offset, vertical_offset));
        builder.curve_to(
            square_bounds.bottom_right(),
            square_bounds.top_right() + point(px(0.0), vertical_offset),
        );
        builder.line_to(square_bounds.bottom_left());
        let path = builder.build().unwrap();
        lines.push((
            path,
            linear_gradient(
                180.,
                linear_color_stop(gpui::blue(), 0.4),
                linear_color_stop(gpui::red(), 1.),
            ),
        ));

        // draw a pie chart
        let center = point(px(96.), px(96.));
        let pie_center = point(px(775.), px(155.));
        let segments = [
            (
                point(px(871.), px(155.)),
                point(px(747.), px(63.)),
                rgb(0x1374e9),
            ),
            (
                point(px(747.), px(63.)),
                point(px(679.), px(163.)),
                rgb(0xe13527),
            ),
            (
                point(px(679.), px(163.)),
                point(px(754.), px(249.)),
                rgb(0x0751ce),
            ),
            (
                point(px(754.), px(249.)),
                point(px(854.), px(210.)),
                rgb(0x209742),
            ),
            (
                point(px(854.), px(210.)),
                point(px(871.), px(155.)),
                rgb(0xfbc10a),
            ),
        ];

        for (start, end, color) in segments {
            let mut builder = PathBuilder::fill();
            builder.move_to(start);
            builder.arc_to(center, px(0.), false, false, end);
            builder.line_to(pie_center);
            builder.close();
            let path = builder.build().unwrap();
            lines.push((path, color.into()));
        }

        // draw a wave
        let options = StrokeOptions::default()
            .with_line_width(1.)
            .with_line_join(lyon::path::LineJoin::Bevel);
        let mut builder = PathBuilder::stroke(px(1.)).with_style(PathStyle::Stroke(options));
        builder.move_to(point(px(40.), px(320.)));
        for i in 1..50 {
            builder.line_to(point(
                px(40.0 + i as f32 * 10.0),
                px(320.0 + (i as f32 * 10.0).sin() * 40.0),
            ));
        }

        Self {
            default_lines: lines.clone(),
            lines: vec![],
            start: point(px(0.), px(0.)),
            dashed: false,
            _painting: false,
        }
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.lines.clear();
        cx.notify();
    }
}

fn button(
    text: &str,
    cx: &mut Context<PaintingViewer>,
    on_click: impl Fn(&mut PaintingViewer, &mut Context<PaintingViewer>) + 'static,
) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .child(text.to_string())
        .bg(gpui::black())
        .text_color(gpui::white())
        .active(|this| this.opacity(0.8))
        .flex()
        .px_3()
        .py_1()
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
}

impl Render for PaintingViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.request_animation_frame();

        let default_lines = self.default_lines.clone();
        let lines = self.lines.clone();
        let window_size = window.bounds().size;
        let scale = window_size.width / DEFAULT_WINDOW_WIDTH;
        let dashed = self.dashed;

        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_between()
                    .items_center()
                    .child("Mouse down any point and drag to draw lines (Hold on shift key to draw straight lines)")
                    .child(
                        div()
                            .flex()
                            .gap_x_2()
                            .child(button(
                                if dashed { "Solid" } else { "Dashed" },
                                cx,
                                move |this, _| this.dashed = !dashed,
                            ))
                            .child(button("Clear", cx, |this, cx| this.clear(cx))),
                    ),
            )
            .child(
                div()
                    .size_full()
                    .child(
                        canvas(
                            move |_, _, _| {},
                            move |_, _, window, _| {
                                for (path, color) in default_lines {
                                    window.paint_path(path.clone().scale(scale), color);
                                }

                                for points in lines {
                                    if points.len() < 2 {
                                        continue;
                                    }

                                    let mut builder = PathBuilder::stroke(px(1.));
                                    if dashed {
                                        builder = builder.dash_array(&[px(4.), px(2.)]);
                                    }
                                    for (i, p) in points.into_iter().enumerate() {
                                        if i == 0 {
                                            builder.move_to(p);
                                        } else {
                                            builder.line_to(p);
                                        }
                                    }

                                    if let Ok(path) = builder.build() {
                                        window.paint_path(path, gpui::black());
                                    }
                                }
                            },
                        )
                        .size_full(),
                    )
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, ev: &MouseDownEvent, _, _| {
                            this._painting = true;
                            this.start = ev.position;
                            let path = vec![ev.position];
                            this.lines.push(path);
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, ev: &gpui::MouseMoveEvent, _, cx| {
                        if !this._painting {
                            return;
                        }

                        let is_shifted = ev.modifiers.shift;
                        let mut pos = ev.position;
                        // When holding shift, draw a straight line
                        if is_shifted {
                            let dx = pos.x - this.start.x;
                            let dy = pos.y - this.start.y;
                            if dx.abs() > dy.abs() {
                                pos.y = this.start.y;
                            } else {
                                pos.x = this.start.x;
                            }
                        }

                        if let Some(path) = this.lines.last_mut() {
                            path.push(pos);
                        }

                        cx.notify();
                    }))
                    .on_mouse_up(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this._painting = false;
                        }),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                focus: true,
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| PaintingViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
