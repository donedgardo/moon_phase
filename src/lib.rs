use std::time::SystemTime;
use moon_calc::Moon as MoonImpl;

pub struct Moon {
    moon: MoonImpl,
}

impl Moon {
    pub fn new(time: SystemTime) -> Self {
        Self {
            moon: MoonImpl::new(time)
        }
    }
    pub fn phase_name(&self) -> &'static str {
        self.moon.phase_name()
    }
    pub fn phase_emoji(&self) -> &'static str {
        self.moon.phase_emoji()
    }
}

#[cfg(test)]
mod tests {
    use super::Moon;
    use chrono::{DateTime, Utc};

    #[test]
    fn it_works_on_new_moon() {
        let date = "2023-9-15T13:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        assert_eq!(moon.phase_name(), "New Moon");
        assert_eq!(moon.phase_emoji(), "🌑");
    }

    #[test]
    fn it_works_on_full_moon() {
        let date = "2023-8-31T0:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        assert_eq!(moon.phase_name(), "Full Moon");
        assert_eq!(moon.phase_emoji(), "🌕");
    }
}