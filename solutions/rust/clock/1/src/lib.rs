use std::fmt;

const MINUTES_IN_DAY: i32 = 1440;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_minutes = hours * 60 + minutes;
        if total_minutes < 0 {
            total_minutes += (1 - total_minutes / MINUTES_IN_DAY) * MINUTES_IN_DAY;
        }
        Self {
            hours: total_minutes / 60 % 24,
            minutes: total_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
