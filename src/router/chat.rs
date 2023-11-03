use actix_web::{get, HttpResponse, Responder};
use leptos::{view, IntoView};
use moon_phases::chat::{Chat, ChatSideBar};

#[get("/chat")]
pub async fn chat_get() -> impl Responder {
    let body = leptos::ssr::render_to_string(|cx| {
        view! { cx,
            <html>
                <head>
                    <script
                        src="https://unpkg.com/htmx.org@1.9.5"
                        integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
                        crossorigin="anonymous"
                    ></script>
                    <script src="https://unpkg.com/htmx.org/dist/ext/ws.js"></script>
                    <link href="/static/output.css" rel="stylesheet"/>
                </head>
                <body>
                    <div class="flex h-screen antialiased text-gray-800">
                        <div class="flex flex-row h-full w-full overflow-x-hidden">
                            <ChatSideBar/>
                            <Chat/>
                        </div>
                    </div>
                </body>
            </html>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
