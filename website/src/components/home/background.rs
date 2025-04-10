use std::{collections::VecDeque, f64::consts::PI};

use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{function_component, html, use_effect_with, use_state, Html, NodeRef};
use yew_hooks::{use_interval, use_size};

const POINTS: usize = 20;
const LINES: usize = 30;
const MAX_LINES_FROM_POINT: usize = 3;

const JITTER: f64 = 0.1;
const GRAVITY_RADIUS: f64 = 200.0;

const FRAME_MSECS: u32 = 50;
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

#[derive(PartialEq)]
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
            context.set_stroke_style(&"white".into());
            let _ = context.arc(point.x, point.y, 4.0, 0.0, 2.0 * PI);
            context.stroke();

            // Draw any lines
            for outgoing_index in &point.outgoing {
                let outgoing = &self.points[*outgoing_index];

                context.begin_path();
                context.set_stroke_style(&"white".into());
                context.move_to(point.x, point.y);
                context.line_to(outgoing.x, outgoing.y);
                context.stroke();
            }
        }
    }
}

#[function_component(Background)]
pub fn background() -> Html {
    let size_ref = NodeRef::default();
    let canvas_ref = NodeRef::default();

    let size = use_size(size_ref.clone());
    let data = use_state(|| BackgroundData::new(size.0 as usize, size.1 as usize));

    // Resize canvas whenever wider div is resized
    {
        let size = size.clone();
        let data = data.clone();

        use_effect_with(size, move |size| {
            let new_data = BackgroundData::new(size.0 as usize, size.1 as usize);
            data.set(new_data);
        });
    }

    // Update background data every frame
    {
        let data = data.clone();

        use_interval(
            move || {
                data.set(data.new_frame());
            },
            FRAME_MSECS,
        );
    }

    // Draw on canvas whenever background data is updated
    {
        let canvas_ref = canvas_ref.clone();
        let size = size.clone();
        let data = data.clone();

        use_effect_with(data, move |data| {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            context.clear_rect(0.0, 0.0, size.0 as f64, size.1 as f64);
            data.draw(context);
        });
    }

    html! {
        <div class="size-full" ref={size_ref.clone()}>
            <canvas width={size.0.to_string()} height={size.1.to_string()} ref={canvas_ref.clone()} />
        </div>
    }
}
