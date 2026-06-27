use chrono::{DateTime, Local};

pub fn format_last_modified(dt: &DateTime<Local>) -> String {
    let now = Local::now();
    let diff = now.signed_duration_since(*dt);
    let days = diff.num_days();

    if days == 0 {
        "today".to_string()
    } else if days == 1 {
        "yesterday".to_string()
    } else if days < 7 {
        format!("{days} days ago")
    } else if days < 30 {
        format!("{} weeks ago", days / 7)
    } else if days < 365 {
        format!("{} months ago", days / 30)
    } else {
        format!("{} years ago", days / 365)
    }
}
