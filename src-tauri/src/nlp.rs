use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use regex::Regex;

pub struct ParsedTask {
    pub title: String,
    pub scheduled_for: Option<i64>,
    pub context: Option<String>,
}

pub fn parse_task_input(input: &str) -> ParsedTask {
    let mut title = input.to_string();
    let mut scheduled_for = None;
    let mut context = None;

    // 1. Extract context (@context)
    let context_re = Regex::new(r"@(\w+)").unwrap();
    if let Some(caps) = context_re.captures(&title.clone()) {
        context = Some(caps[1].to_string());
        title = context_re.replace(&title, "").trim().to_string();
    }

    // 2. Parse "tomorrow"
    let tomorrow_re = Regex::new(r"(?i)\btomorrow\b").unwrap();
    if tomorrow_re.is_match(&title) {
        let now = Utc::now();
        let tomorrow = now + Duration::days(1);
        // Default to 9 AM tomorrow
        let scheduled = Utc.with_ymd_and_hms(
            tomorrow.year(),
            tomorrow.month(),
            tomorrow.day(),
            9,
            0,
            0
        ).unwrap();
        
        scheduled_for = Some(scheduled.timestamp());
        title = tomorrow_re.replace(&title, "").trim().to_string();
    }

    // 3. Parse "in X days"
    let in_days_re = Regex::new(r"(?i)\bin (\d+) days?\b").unwrap();
    if let Some(caps) = in_days_re.captures(&title.clone()) {
        if let Ok(days) = caps[1].parse::<i64>() {
            let now = Utc::now();
            let future = now + Duration::days(days);
            let scheduled = Utc.with_ymd_and_hms(
                future.year(),
                future.month(),
                future.day(),
                9,
                0,
                0
            ).unwrap();
            
            scheduled_for = Some(scheduled.timestamp());
            title = in_days_re.replace(&title, "").trim().to_string();
        }
    }
    
    // 4. Parse "next Monday" etc.
    // Simple implementation for MVP: check for "next [weekday]"
    let weekdays = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
    for (i, day_str) in weekdays.iter().enumerate() {
        let pattern = format!(r"(?i)\bnext {}\b", day_str);
        let re = Regex::new(&pattern).unwrap();
        if re.is_match(&title) {
            let now = Utc::now();
            let target_weekday = match i {
                0 => Weekday::Mon,
                1 => Weekday::Tue,
                2 => Weekday::Wed,
                3 => Weekday::Thu,
                4 => Weekday::Fri,
                5 => Weekday::Sat,
                6 => Weekday::Sun,
                _ => Weekday::Mon,
            };
            
            let mut days_to_add = 0;
            let mut current = now;
            loop {
                days_to_add += 1;
                current = current + Duration::days(1);
                if current.weekday() == target_weekday {
                    break;
                }
                if days_to_add > 14 { break; } // Safety break
            }
            
            // "next Monday" usually means the one next week, not just the upcoming one if today is Sunday.
            // But usually "next Monday" matches the upcoming Monday if it's far enough?
            // MVP Simplification: Just find the next occurrence of that day.
            
            let scheduled = Utc.with_ymd_and_hms(
                current.year(),
                current.month(),
                current.day(),
                9, 0, 0
            ).unwrap();
            
            scheduled_for = Some(scheduled.timestamp());
            title = re.replace(&title, "").trim().to_string();
            break;
        }
    }

    ParsedTask {
        title,
        scheduled_for,
        context,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_extraction() {
        let p = parse_task_input("Buy milk @groceries");
        assert_eq!(p.title, "Buy milk");
        assert_eq!(p.context, Some("groceries".to_string()));
    }

    #[test]
    fn test_tomorrow_parsing() {
        let p = parse_task_input("Call mom tomorrow");
        assert_eq!(p.title, "Call mom");
        assert!(p.scheduled_for.is_some());
    }
}
