use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// Route to handle WebSocket connections
pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket {}, &req, stream)?;
    Ok(resp)
}

struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::App;
    use actix_web_actors::ws;
    use futures::StreamExt;
    use futures_util::sink::SinkExt;

    #[actix_rt::test]
    async fn test_websocket() {
        let mut srv = actix_test::start(|| App::new().route("/ws/", web::get().to(ws_index)));

        // Connect to the server
        let mut connection = srv.ws_at("/ws/").await.expect("Failed to connect");

        // Send a text message
        let send_text = "Hello WebSocket!";
        connection
            .send(ws::Message::Text(send_text.into()))
            .await
            .expect("Failed to send message");

        // Receive the message
        let msg = connection.next().await.expect("Failed to receive message");
        if let Ok(ws::Frame::Text(text)) = msg {
            assert_eq!(send_text, text);
        } else {
            panic!("Expected text frame");
        }
    }
}
