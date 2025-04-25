#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
pub struct DisplayTime {
    pub millis: i64
}

impl DisplayTime {
    pub fn new() -> Self {
        Self { millis:0 }
    }
    pub fn display(&self) -> String {
        let millis_in_hour = 1000 * 60 * 60;
        let millis_in_minute = millis_in_hour / 60;
        let millis_in_second = millis_in_minute / 60;
        let mut millis = self.millis;
        let hours = millis / millis_in_hour;
        millis -= hours * millis_in_hour;
        let minutes = millis / millis_in_minute;
        millis -= minutes * millis_in_minute;
        let seconds = millis / millis_in_second;
        millis -= seconds * millis_in_second;
        return match (hours, minutes, seconds) {
            (0, 0, 0) => format!("0.{:03}", millis),
            (0, 0, s) => format!("{}.{:03}", s, millis),
            (0, m, s) => format!("{}:{:02}.{:03}", m, s, millis),
            (h, m, s) => format!("{}:{:02}:{:02}.{:03}", h, m, s, millis)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_display_string() {
        let example = "1:23.898".to_string();
        let t = DisplayTime { millis: 83_898 };
        assert_eq!(t.display(), example)
    }

    #[test]
    fn check_display_string_0_min() {
        let example = "1:00.898".to_string();
        let t = DisplayTime { millis: 60_898 };
        assert_eq!(t.display(), example)
    }

    #[test]
    fn check_display_string_1_sec() {
        let example = "1.898".to_string();
        let t = DisplayTime { millis: 1_898 };
        assert_eq!(t.display(), example)
    }
}