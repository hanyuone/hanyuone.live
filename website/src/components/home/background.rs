use std::{collections::VecDeque, f64::consts::PI};

use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{function_component, html, use_effect_with, use_state, Html, NodeRef};
use yew_hooks::use_interval;

const POINTS: usize = 20;
const LINES: usize = 50;
const MAX_LINES_FROM_POINT: usize = 5;
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
            BackgroundState::AddPoints => {
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
                    let incoming_point = &points[incoming];

                    if incoming_point.outgoing.len() >= MAX_LINES_FROM_POINT {
                        continue;
                    }

                    let outgoing = rng.random_range(0..POINTS);
                    let outgoing_point = &points[outgoing];

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

    fn new_frame(&self) -> Self {
        if self.last_update_frames > UPDATE_STATE_FRAMES {
            return self.update();
        }

        // Update point positions
        // TODO: complete once physics system figured out
        let points = self.points.clone();

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
            let _ = context.arc(point.x, point.y, 2.0, 0.0, 2.0 * PI);
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
    let node_ref = NodeRef::default();
    let data = use_state(|| BackgroundData::new(720, 480));

    {
        let data = data.clone();

        use_interval(
            move || {
                data.set(data.new_frame());
            },
            FRAME_MSECS,
        );
    }

    {
        let node_ref = node_ref.clone();
        let data = data.clone();

        use_effect_with(data, move |data| {
            let canvas = node_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            context.clear_rect(0.0, 0.0, 720.0, 480.0);
            data.draw(context);
        });
    }

    html! {
        <canvas width={720} height={480} ref={node_ref.clone()} />
    }
}
