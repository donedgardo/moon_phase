use crate::openai::TextMessage;
use crate::{chat, openai};
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use leptos::{view, IntoView};
use moon_phases::chat::{AiChatMessage, UserChatMessage};
use uuid::Uuid;

pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket {}, &req, stream)?;
    Ok(resp)
}

pub struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(payload)) => {
                match chat::parse_payload(payload.to_string().as_str()) {
                    Ok(payload) => {
                        let message_clone = payload.message.clone();
                        let user_chat_msg = leptos::ssr::render_to_string(move |cx| {
                            view! { cx,
                                <div hx-swap-oob="beforeend:#chat-history">
                                    <UserChatMessage msg=message_clone/>
                                </div>
                            }
                        });
                        ctx.text(user_chat_msg);
                        let my_uuid = Uuid::new_v4().to_string();
                        let my_uuid_clone = my_uuid.clone();
                        let ai_chat_msg = leptos::ssr::render_to_string(move |cx| {
                            view! { cx,
                                <div hx-swap-oob="beforeend:#chat-history">
                                    <AiChatMessage id=my_uuid_clone/>
                                </div>
                            }
                        });
                        ctx.text(ai_chat_msg);
                        let addr = ctx.address();
                        tokio::spawn(async move {
                            if let Err(err) = openai::consume_streaming_api(
                                "http://localhost:11434/api/generate",
                                "dndai",
                                &payload.message,
                                addr,
                                my_uuid.as_str(),
                            )
                            .await
                            {
                                eprintln!("Error: {}", err);
                            }
                        });
                    }
                    Err(_) => ctx.text("Error parsing"),
                }
            }
            _ => (),
        }
    }
}

impl Handler<TextMessage> for MyWebSocket {
    type Result = ();
    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

#[cfg(test)]
mod ws_tests {
    use super::*;
    use actix_web::App;
    use futures_util::{SinkExt, StreamExt};

    #[actix_rt::test]
    async fn test_websocket() {
        let mut srv = actix_test::start(|| App::new().route("/ws/", web::get().to(ws_index)));
        let mut connection = srv.ws_at("/ws/").await.expect("Failed to connect");
        let send_text = "Hello WebSocket!";
        connection
            .send(ws::Message::Text(send_text.into()))
            .await
            .expect("Failed to send message");
        let msg = connection.next().await.expect("Failed to receive message");
        if let Ok(ws::Frame::Text(text)) = msg {
            assert_eq!(text, "Error parsing");
        } else {
            panic!("Expected text frame");
        }
    }
}
