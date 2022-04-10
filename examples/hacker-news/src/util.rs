use chrono::{DateTime, Local};

pub fn time_ago(start: DateTime<Local>) -> String {
    let now = chrono::Local::now();
    let since = now - start;

    let mins = since.num_minutes();
    let hours = since.num_hours();
    let days = since.num_days();

    if days == 1 {
        "1 day ago".to_string()
    } else if days > 1 {
        format!("{} days ago", days)
    } else if hours == 1 {
        "1 hour ago".to_string()
    } else if hours > 1 {
        format!("{} hours ago", hours)
    } else if mins <= 1 {
        "1 minute ago".to_string()
    } else {
        format!("{} minutes ago", mins)
    }
}
