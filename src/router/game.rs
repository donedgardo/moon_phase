use actix_web::{get, HttpResponse, Responder};

#[get("/game")]
pub async fn get() -> impl Responder {
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
             gltf-model="#moon"
             modify-materials
             position="-0.48859 38.86 -46.69"
             rotation="1.21 0 -0.48"
             scale="0.01 0.01 0.01"
          ></a-entity>
          <a-light type="directional" position="0 0 0" rotation="-90 0 0" target="#directionaltarget">
             <a-entity id="directionaltarget" position="-1 0 -1"></a-entity>
          </a-light>
          <a-entity
            environment="preset: starry; playArea: 1.2; ground: hills; grid: none;"
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
