use std::time::SystemTime;
use actix_web::{get, HttpResponse, Responder};
use leptos::IntoAttribute;
use moon_phases::moon::Moon;

#[get("/game")]
pub async fn get() -> impl Responder {
    let moon = Moon::new(SystemTime::now());
    let pos = moon.get_sun_relative_position();
    let html = leptos::ssr::render_to_string(|cx| leptos::view! { cx,
        <a-scene cursor="rayOrigin: mouse">
          <a-assets>
            <a-asset-item id="moon" src="/static/moon.glb"></a-asset-item>
          </a-assets>
          <a-camera
              sound="src: url(/static/bobs-adventure-theme.mp3); autoplay: true; loop: true;"
              light="color: #fff; intensity: .2; type: ambient;"
          ></a-camera>
          <a-entity
             id="moon-obj"
             gltf-model="#moon"
             modify-materials
             position="0 35 -46.69"
             rotation="1.21 0 -0.48"
             scale="0.01 0.01 0.01">
            <a-light
              type="directional"
              intensity="4.47"
              position=move || format!("{} {} {}", pos[0], pos[1], pos[2])
              target="#moon-obj">
            </a-light>
          </a-entity>
          <a-entity
           environment="preset: starry; playArea: 1.2; ground: hills; grid: none; lightPosition: -5.12 -0.01 -2.68;"
          ></a-entity>
        </a-scene>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[cfg(test)]
mod game_route_tests {
    use super::get;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn it_has_ok_status() {
        let mut app = test::init_service(App::new().service(get)).await;
        let req = test::TestRequest::get().uri("/game").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}
