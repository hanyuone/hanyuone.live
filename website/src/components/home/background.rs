use std::{collections::VecDeque, f64::consts::PI};

use leptos::{
    html::{Canvas, Div},
    prelude::*,
    wasm_bindgen::JsCast,
    web_sys::{CanvasRenderingContext2d, HtmlCanvasElement},
};
use leptos_use::{use_interval, use_resize_observer, UseIntervalReturn};
use rand::RngExt;

const POINTS: usize = 20;
const LINES: usize = 30;
const MAX_LINES_FROM_POINT: usize = 3;

const JITTER: f64 = 0.1;
const GRAVITY_RADIUS: f64 = 200.0;

const FRAME_MSECS: u64 = 50;
const UPDATE_STATE_FRAMES: usize = 20;

#[derive(Clone, PartialEq)]
struct BackgroundPoint {
    x: f64,
    y: f64,
    outgoing: Vec<usize>,
}

impl BackgroundPoint {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            outgoing: vec![],
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BackgroundState {
    AddPoints,
    AddLines,
    Full,
}

#[derive(Clone, PartialEq)]
struct BackgroundData {
    width: usize,
    height: usize,
    points: VecDeque<BackgroundPoint>,
    lines: usize,
    state: BackgroundState,
    last_update_frames: usize,
}

impl BackgroundData {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            points: VecDeque::new(),
            lines: 0,
            state: BackgroundState::AddPoints,
            last_update_frames: 0,
        }
    }

    fn update(&self) -> Self {
        let mut updated = Self::new(self.width, self.height);
        let mut points = self.points.clone();

        let mut rng = rand::rng();

        updated.state = self.state;

        match updated.state {
            BackgroundState::AddPoints => 'add_points: {
                if self.width == 0 || self.height == 0 {
                    break 'add_points;
                }

                let x = rng.random_range(0.0..self.width as f64);
                let y = rng.random_range(0.0..self.height as f64);

                points.push_back(BackgroundPoint::new(x, y));

                if points.len() >= POINTS {
                    updated.state = BackgroundState::AddLines;
                }
            }
            BackgroundState::AddLines => {
                let (incoming, outgoing) = loop {
                    let incoming = rng.random_range(0..POINTS);
                    let incoming_point = points.get(incoming).unwrap();

                    if incoming_point.outgoing.len() >= MAX_LINES_FROM_POINT {
                        continue;
                    }

                    let outgoing = rng.random_range(0..POINTS);
                    let outgoing_point = points.get(outgoing).unwrap();

                    if !(incoming_point.outgoing.contains(&outgoing)
                        || outgoing_point.outgoing.contains(&incoming))
                    {
                        break (incoming, outgoing);
                    }
                };

                points[incoming].outgoing.push(outgoing);
                updated.lines = self.lines + 1;

                if updated.lines >= LINES {
                    updated.state = BackgroundState::Full;
                }
            }
            BackgroundState::Full => {
                // We remove a point. By default, we remove the *frontmost* point
                // of our queue every time (so every point eventually gets deleted)
                points.pop_front();

                // We make sure each line goes to the right point by decrementing
                // indices
                for point in &mut points {
                    point.outgoing = point
                        .outgoing
                        .iter()
                        .filter(|x| **x > 0)
                        .map(|idx| idx - 1)
                        .collect::<Vec<_>>();
                }

                updated.state = BackgroundState::AddPoints;
            }
        }

        updated.points = points;
        updated
    }

    fn new_frame_points(&self) -> VecDeque<BackgroundPoint> {
        let mut points = self.points.clone();
        let points_len = points.len();

        // Gravity simulation
        for point_index in 0..points_len {
            let mut delta_x = 0.0;
            let mut delta_y = 0.0;

            for other_point_index in 0..points_len {
                let point = points.get(point_index).unwrap();
                let other_point = points.get(other_point_index).unwrap();

                if other_point_index == point_index {
                    continue;
                }

                // Vector from `point` to `other_point`
                let dist_x = other_point.x - point.x;
                let dist_y = other_point.y - point.y;
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();

                // We want the points to *separate* from each other, so make sure
                // the delta is going in the *opposite* direction
                let gravity = (1.0 - (dist / GRAVITY_RADIUS)).clamp(0.0, 1.0).powf(2.0);
                delta_x -= dist_x * gravity;
                delta_y -= dist_y * gravity;
            }

            let point = points.get_mut(point_index).unwrap();

            // Add delta
            point.x += delta_x / (points_len as f64);
            point.y += delta_y / (points_len as f64);

            // Add jitter
            let mut rng = rand::rng();
            let angle = rng.random_range(0.0..2.0 * PI);

            point.x += angle.cos() * JITTER;
            point.y += angle.sin() * JITTER;

            // If point is out of bounds, wrap around
            let width = self.width as f64;
            let height = self.height as f64;

            // To stop jitters from "bouncing" points between either end
            // of the screen, introduce a "buffer" when we wrap a point around
            if point.x < 0.0 {
                point.x += width - (JITTER * 100.0);
            } else if point.x > width {
                point.x -= width - (JITTER * 100.0);
            }

            if point.y < 0.0 {
                point.y += height - (JITTER * 100.0);
            } else if point.y > height {
                point.y -= height - (JITTER * 100.0);
            }
        }

        points
    }

    fn new_frame(&self) -> Self {
        if self.last_update_frames > UPDATE_STATE_FRAMES {
            return self.update();
        }

        let points = self.new_frame_points();

        Self {
            width: self.width,
            height: self.height,
            points,
            lines: self.lines,
            state: self.state,
            last_update_frames: self.last_update_frames + 1,
        }
    }

    fn draw(&self, context: CanvasRenderingContext2d) {
        for point in &self.points {
            // Draw the point itself
            context.begin_path();
            context.set_stroke_style_str("white");
            let _ = context.arc(point.x, point.y, 4.0, 0.0, 2.0 * PI);
            context.stroke();

            // Draw any lines
            for outgoing_index in &point.outgoing {
                let outgoing = &self.points[*outgoing_index];

                context.begin_path();
                context.set_stroke_style_str("white");
                context.move_to(point.x, point.y);
                context.line_to(outgoing.x, outgoing.y);
                context.stroke();
            }
        }
    }
}

#[island]
pub fn Background() -> impl IntoView {
    let size_ref = NodeRef::<Div>::new();
    let canvas_ref = NodeRef::<Canvas>::new();

    let UseIntervalReturn {
        counter,
        reset: _,
        is_active: _,
        pause: _,
        resume: _,
    } = use_interval(FRAME_MSECS);

    let (data, set_data) = signal(BackgroundData::new(0, 0));

    use_resize_observer(size_ref, move |entries, _| {
        let rect = entries[0].content_rect();

        set_data.set(BackgroundData::new(
            rect.width() as usize,
            rect.height() as usize,
        ));
    });

    // Update data on every frame
    Effect::watch(
        move || counter.get(),
        move |_, _, _| set_data.set(data.get_untracked().new_frame()),
        false,
    );

    // Draw on canvas whenever data gets updated
    Effect::watch(
        move || data.get(),
        move |data, _, _| {
            let canvas: HtmlCanvasElement = canvas_ref.get_untracked().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            context.clear_rect(0.0, 0.0, data.width as f64, data.height as f64);
            data.draw(context);
        },
        false,
    );

    view! {
        <div class="size-full" node_ref=size_ref>
            <canvas width={move || data.get().width} height={move || data.get().height} node_ref=canvas_ref />
        </div>
    }
}
