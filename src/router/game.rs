use actix_web::{get, HttpResponse, Responder};

#[get("/game")]
pub async fn get() -> impl Responder {
    let html = leptos::ssr::render_to_string(|cx| leptos::view! { cx,
        <html>
          <head>
            <script src="https://aframe.io/releases/1.4.0/aframe.min.js"></script>
          </head>
          <body>
            <a-scene>
              <a-box position="-1 0.5 -3" rotation="0 45 0" color="#4CC3D9"></a-box>
              <a-sphere position="0 1.25 -5" radius="1.25" color="#EF2D5E"></a-sphere>
              <a-cylinder position="1 0.75 -3" radius="0.5" height="1.5" color="#FFC65D"></a-cylinder>
              <a-plane position="0 0 -4" rotation="-90 0 0" width="4" height="4" color="#7BC8A4"></a-plane>
              <a-sky color="#ECECEC"></a-sky>
            </a-scene>
          </body>
        </html>
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
