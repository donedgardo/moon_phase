use std::time::SystemTime;
use moon_calc::Moon as MoonImpl;
use leptos::IntoView;

pub mod component;

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
    pub fn get_sun_relative_position(&self) -> Vec<f32> {
        match self.moon.phase_emoji() {
            "🌑" => vec![0., 1., -1.],
            "🌒" => vec![1., 0., -0.5],
            "🌓" =>  vec![1., 0., 0.],
            "🌔" => vec![1., 0., 1.],
            "🌕" =>  vec![0., -1., 1.],
            "🌖" => vec![-1., -1., 1.],
            "🌗" =>  vec![-1., 0., 0.],
            "🌘" =>  vec![-1., 0., -0.5],
            &_ => vec![0., 0., 0.]
        }
    }

    // in earth radii
    pub fn distance_from_earth_km(&self) -> f64 {
        self.moon.distance_km()
    }
}

#[cfg(test)]
mod moon_tests {
    use crate::moon::Moon;
    use chrono::{DateTime, Utc};

    #[test]
    fn it_works_on_new_moon() {
        let moon = create_new_moon();
        assert_eq!(moon.phase_name(), "New Moon");
        assert_eq!(moon.phase_emoji(), "🌑");
    }

    #[test]
    fn it_detects_full_moon() {
        let moon = create_full_moon();
        assert_eq!(moon.phase_name(), "Full Moon");
        assert_eq!(moon.phase_emoji(), "🌕");
    }

    #[test]
    fn it_calculates_sun_relative_position_for_new_moon () {
        let moon = create_new_moon();
        let pos: Vec<f32> = moon.get_sun_relative_position();
        assert_eq!(pos, vec![0., 1., -1.]);
    }

    #[test]
    fn it_calculates_sun_relative_position_for_full_moon () {
        let moon = create_full_moon();
        let pos: Vec<f32> = moon.get_sun_relative_position();
        assert_eq!(pos, vec![0., -1., 1.]);
    }

    #[test]
    fn it_calculates_last_quarter() {
        let date = "2023-9-7T0:00:00z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        assert_eq!(moon.phase_name(), "Last Quarter");
        assert_eq!(moon.get_sun_relative_position(), vec![-1., 0., 0.]);
        assert_eq!(moon.distance_from_earth_km(), 389790.7303662425)
    }
    #[test]
    fn it_calculates_waning_crescent() {
        let date = "2023-9-13T0:00:00z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        assert_eq!(moon.get_sun_relative_position(), vec![-1., 0., -0.5]);
        assert_eq!(moon.phase_name(), "Waning Crescent");
        assert_eq!(moon.phase_emoji(), "🌘");
    }

    fn create_full_moon() -> Moon {
        let date = "2023-8-31T0:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        moon
    }

    fn create_new_moon() -> Moon {
        let date = "2023-9-15T13:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        moon
    }
}
