use actix_web::{get, HttpResponse, Responder};
use leptos::*;
use moon_phases::moon::component::MoonRender;

#[get("/game")]
pub async fn get_game() -> impl Responder {
    let html = ssr::render_to_string(|cx| view! { cx,
        <a-scene>
            <a-assets>
                <a-asset-item id="moon" src="/static/moon.glb"></a-asset-item>
            </a-assets>
          <a-camera
            look-controls
            sound="src: url(/static/bobs-adventure-theme.mp3); autoplay: true; loop: true;"
            light="color: #fff; intensity: .2; type: ambient;"
          >
            <a-entity id="hud" position="0 0.16 -0.5" scale="0.3 0.3 0.3" opacity="0.8" align="center">
              <a-text id="counter" value="" align="center"></a-text>
            </a-entity>
            <a-entity
              cursor="fuse: true; fuseTimeout: 500"
              position="0 0 -3"
              geometry="primitive: ring; radiusInner: 0.02; radiusOuter: 0.03"
              material="color: #fff; shader: flat"
            ></a-entity>
          </a-camera>
          <MoonRender />
          <a-entity
           environment="preset: starry; playArea: 1.2; ground: hills; grid: none; lightPosition: -5.12 -0.01 -2.68;"
          ></a-entity>
        </a-scene>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/game/moon/stats")]
pub async fn get_moon_stats() -> impl Responder {
    let html = ssr::render_to_string(|cx| view! { cx,
         <a-entity id="hud" position="0 0.16 -0.5" scale="0.3 0.3 0.3" opacity="0.8" align="center">
            <a-text id="counter" value="Good" align="center"></a-text>
         </a-entity>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[cfg(test)]
mod game_route_tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn it_has_ok_status() {
        let mut app = test::init_service(App::new().service(get_game)).await;
        let req = test::TestRequest::get().uri("/game").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn stats_has_ok_status() {
        let mut app = test::init_service(App::new().service(get_moon_stats)).await;
        let req = test::TestRequest::get().uri("/game/moon/stats").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}
