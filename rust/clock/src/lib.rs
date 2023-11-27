use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes } 
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hours > 9 {
            if self.minutes > 9 {
                write!(f, "{:?}:{:?}", self.hours, self.minutes)
            } else {
                write!(f, "{:?}:0{:?}", self.hours, self.minutes)
            }
        } else {
            if self.minutes > 9 {
                write!(f, "0{:?}:{:?}", self.hours, self.minutes)
            }else {
                write!(f, "0{:?}:0{:?}", self.hours, self.minutes)
            }
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}