use std::{cmp::Ordering, fmt};

#[derive(Debug)]
pub struct Clock {
    hours: i16,
    minutes: i16,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:02}:{:02}", self.hours, self.minutes).fmt(f)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let mut minutes = hours as i16*60 + minutes as i16;
        minutes%=24*60;
        if minutes < 0{
            minutes += 24*60;
        }
        Clock {
            hours: (minutes / 60) % 24,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        let mut minutes = self.hours*60 + self.minutes + minutes as i16;
        minutes%=24*60;
        if minutes < 0{
            minutes += 24*60;
        }
        Clock {
            hours: (minutes / 60) % 24,
            minutes: minutes % 60,
        }
    }
}
