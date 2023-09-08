use std::time::SystemTime;
use moon_calc::Moon as MoonImpl;
use leptos::{component, IntoView};

pub struct Moon {
    moon: MoonImpl,
}

impl Moon {

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
        if self.moon.phase_name() == "New Moon" {
            vec![0., 1., -1.]
        } else if self.moon.phase_name() == "Full Moon"  {
            vec![0., -1., 1.]
        } else {
            vec![-1., 0., 0. ]
        }
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

#[component]
pub fn MoonAppMenu(cx: leptos::Scope) -> impl IntoView {
    let moon = Moon::new(SystemTime::now());
    let emoji = moon.phase_emoji();
    let phase = moon.phase_name();
    leptos::view! { cx,
        <div
          id="app"
          class="mx-auto dark:flex hidden flex-col justify-center items-center space-y-4 text-stone-200"
        >
          <p style="font-size: 180px">{emoji}</p>
          <h1 class="text-6xl font-bold underline text-center">Lunar Harvest</h1>
          <h2 class="text-5xl font-bold text-center">{phase}</h2>
          <p class="text-1xl text-center" hx-get="/game" hx-target="#app" hx-swap="outerHTML">
            <em>Come back on a a new moon.</em>
          </p>
        </div>
        <div
          id="app"
          class="mx-auto flex dark:hidden flex-col space-y-4 text-xl"
        >
          <p>Enable the night,</p>
          <p>Dark skies bring secret features,</p>
          <p>Mode of moon whispers.</p>
        </div>
    }
}
