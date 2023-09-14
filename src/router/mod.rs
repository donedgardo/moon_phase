use actix_web::{get, HttpResponse, Responder};

pub mod home;
pub mod game;
pub mod box_change;

#[get("/auth")]
pub async fn auth_get() -> impl Responder {
    let body = leptos::ssr::render_to_string(|cx| leptos::view! { cx,
        <form post="/auth">
          <h1>Login</h1>
          <label for="email">Email:</label>
          <input type="email" name="email"  />
          <label for="password">Password:</label>
          <input type="password" name="password" />
          <button>Login</button>
        </form>
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[cfg(test)]
mod auth_get_tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn it_has_ok_status() {
        let mut app = test::init_service(App::new().service(auth_get)).await;
        let req = test::TestRequest::get().uri("/auth").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }
}

