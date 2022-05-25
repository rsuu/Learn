use chrono::{format::ParseError, DateTime, FixedOffset, Utc};
use time::{Date, Instant};

fn get_time() -> String {
    format!("{:?}", Instant::now())
    //  Utc::now().to_rfc3339()
}
fn get_fmt_time(time: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    DateTime::parse_from_rfc3339(time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fmt_time() {
        let _time = get_fmt_time(&get_time()).unwrap();
    }
}
