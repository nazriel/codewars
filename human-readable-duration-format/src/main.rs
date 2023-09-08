fn builder(seconds: u64, divisor: u64, remainder: u64, limit: u64, suffix: String) -> String {
    let value = seconds / divisor;
    let remainder = value % remainder;

    if value < 1 {
        return "".to_string();
    }
    if remainder == 0 {
        return "".to_string();
    }
    let prefix = if value < limit {
        ""
    } else {
        if seconds - value * divisor > 0 { ", " } else { " and " }
    };
    let suffix = if remainder == 1 { suffix } else { suffix + &"s".to_string() };

    format!("{}{} {}", prefix, remainder, suffix)
}

fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "now".to_string();
    }

    format!(
        "{years}{days}{hours}{minutes}{seconds}",
        years = builder(seconds, 60 * 60 * 24 * 365, 365, 1000, "year".to_string()),
        days = builder(seconds, 60 * 60 * 24, 365, 365, "day".to_string()),
        hours = builder(seconds, 60 * 60, 24, 24, "hour".to_string()),
        minutes = builder(seconds, 60, 60, 60, "minute".to_string()),
        seconds = builder(seconds, 1, 60, 60, "second".to_string())
    )
}

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(122), "2 minutes and 2 seconds");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
        assert_eq!(format_duration(8305145), "96 days, 2 hours, 59 minutes and 5 seconds");
        assert_eq!(format_duration(132030240), "4 years, 68 days, 3 hours and 4 minutes");
    }
}
