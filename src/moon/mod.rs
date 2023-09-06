use std::time::SystemTime;
use moon_calc::Moon as MoonImpl;
use leptos::{component, IntoView};

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
mod moon_tests {
    use crate::moon::Moon;
    use chrono::{DateTime, Utc};

    #[test]
    fn it_works_on_new_moon() {
        let date = "2023-9-15T13:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        assert_eq!(moon.phase_name(), "New Moon");
        assert_eq!(moon.phase_emoji(), "🌑");
    }

    #[test]
    fn it_detects_full_moon() {
        let date = "2023-8-31T0:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let moon = Moon::new(date.into());
        assert_eq!(moon.phase_name(), "Full Moon");
        assert_eq!(moon.phase_emoji(), "🌕");
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
          <p class="text-1xl text-center"><em>Come back on a a new moon.</em></p>
        </div>
        <div
          id="app"
          class="mx-auto flex dark:hidden flex-col space-y-4"
        >
          <p>Enable the night,</p>
          <p>Dark skies bring secret features,</p>
          <p>Mode of moon whispers.</p>
        </div>
    }
}
