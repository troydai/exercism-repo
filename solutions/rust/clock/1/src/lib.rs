#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;

        Clock::from_minutes(total_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_minutes = self.hours * 60 + self.minutes;
        total_minutes += minutes;

        Clock::from_minutes(total_minutes)
    }

    fn from_minutes(minute: i32) -> Self {
        let mut total_minutes = minute % (24 * 60);
        if total_minutes < 0 {
            total_minutes += 24 * 60;
        }

        Clock {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}