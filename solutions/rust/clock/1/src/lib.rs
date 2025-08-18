use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i8,
    minutes: i8,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:02}:{:02}", self.hours, self.minutes).fmt(f)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Clock {
            hours: hours as i8,
            minutes: minutes as i8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes as i8,
        }
    }
}
