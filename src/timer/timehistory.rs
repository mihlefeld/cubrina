use super::DisplayTime;
#[derive(PartialEq, Clone)]
pub struct TimerHistory {
    pub time_list: Vec<DisplayTime>,
}

impl TimerHistory {
    pub fn new() -> Self {
        Self {
            time_list: Vec::new()
        }
    }

    pub fn push(&mut self, time: DisplayTime) -> &mut Self {
        self.time_list.push(time);
        self
    }
}