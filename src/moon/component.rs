use leptos::*;
use std::time::SystemTime;
use crate::moon::Moon;

#[component]
pub fn NewMoonHaiku(cx: Scope) -> impl IntoView {
    view! { cx,
        <p class="text-xl" hx-get="/game" hx-target="#app" hx-swap="outerHTML">
          "New moon in the sky,"<br/>
          "Code and dreams blend, take their flight,"<br />
          "Fresh start, endless night."
        </p>
    }
}

#[component]
pub fn MoonAppMenu(cx: Scope) -> impl IntoView {
    let moon = Moon::new(SystemTime::now());
    let emoji = moon.phase_emoji();
    let phase = moon.phase_name();
    view! { cx,
        <div
          id="app"
          class="mx-auto dark:flex hidden flex-col justify-center items-center space-y-4 text-stone-200"
        >
          <p style="font-size: 180px">{emoji}</p>
          <h1 class="text-6xl font-bold underline text-center">Lunar Harvest</h1>
          <h2 class="text-5xl font-bold text-center">{phase}</h2>
          <Show
            when=move || { emoji != "🌑" }
            fallback=move |_| view!{cx,  <NewMoonHaiku /> }
          >
            <p class="text-3xl text-center" hx-get="/game" hx-target="#app" hx-swap="outerHTML">
              <em>"Come back on a a new moon"</em>
            </p>
          </Show>
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


#[component]
pub fn MoonRender(cx: Scope) -> impl IntoView {
    let moon = Moon::new(SystemTime::now());
    let pos = moon.get_sun_relative_position();
    view! { cx,
        <a-entity
             id="moon-obj"
             gltf-model="#moon"
             modify-materials
             position="0 35 -46.69"
             rotation="1.21 0 -0.48"
             scale="0.01 0.01 0.01"
             hx-get="/game/moon/stats"
             hx-trigger="mouseenter once"
             hx-target="#hud"
             hx-swap="outerHTML"
        >
            <a-light
              type="directional"
              intensity="4.47"
              position=move || format!("{} {} {}", pos[0], pos[1], pos[2])
              target="#moon-obj">
            </a-light>
        </a-entity>
    }
}