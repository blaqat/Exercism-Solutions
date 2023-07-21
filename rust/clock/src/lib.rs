// Clock that uses No Hours Version

use core::fmt;

const MINS_IN_HOURS: i32 = 60;
const HOURS_IN_DAYS: i32 = 24;
const MINS_IN_DAYS: i32 = MINS_IN_HOURS * HOURS_IN_DAYS;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Self {
            minutes: (minutes + (hours * MINS_IN_HOURS)).rem_euclid(MINS_IN_DAYS),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // unimplemented!("Add {minutes} minutes to existing Clock time");
        Self {
            minutes: (self.minutes + minutes).rem_euclid(MINS_IN_DAYS),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes
                .div_euclid(MINS_IN_HOURS)
                .rem_euclid(HOURS_IN_DAYS),
            self.minutes.rem_euclid(MINS_IN_HOURS)
        )
    }
}
