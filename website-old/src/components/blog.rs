use chrono::TimeDelta;

pub mod card;
pub mod item;
pub mod tag;

pub fn to_read_time(words: usize) -> String {
    let time_delta = TimeDelta::seconds((words / 3) as i64);
    let seconds = time_delta.num_seconds();

    if seconds < 60 {
        return "<1 min".to_string();
    }

    let minutes = time_delta.num_minutes();

    if minutes < 60 {
        return format!("{} min", minutes);
    }

    "long read".to_string()
}
