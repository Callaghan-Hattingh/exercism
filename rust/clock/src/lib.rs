use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_minutes = hours*60 + minutes;
        while total_minutes < 0 {
            total_minutes += 24*60;

        }
        let adjusted_hours =  (total_minutes/ 60)%24;
        let adjusted_minutes = total_minutes % 60;
        
        Clock { hours: adjusted_hours, minutes: adjusted_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_minutes = self.hours*60 + self.minutes + minutes;
        while total_minutes < 0 {
            total_minutes += 24*60;

        }
        let adjusted_hours =  (total_minutes/ 60)%24;
        let adjusted_minutes = total_minutes % 60;
        
        Clock { hours: adjusted_hours, minutes: adjusted_minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
