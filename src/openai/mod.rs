use crate::router::ws::MyWebSocket;
use actix::{Addr, Message};
use leptos::{view, IntoView};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

//Open ai response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenAiApiResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiRequestPayload {
    model: String,
    prompt: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

pub async fn consume_streaming_api(
    url: &str,
    model: &str,
    prompt: &str,
    address: Addr<MyWebSocket>,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let payload = OpenAiRequestPayload {
        model: model.to_string(),
        prompt: prompt.to_string(),
    };
    let mut response_stream = client.post(url).json(&payload).send().await?.bytes_stream();
    while let Some(chunk) = response_stream.next().await {
        match chunk {
            Ok(bytes) => {
                let json_str = std::str::from_utf8(&bytes).unwrap_or_default();
                for line in json_str.lines() {
                    let api_response: OpenAiApiResponse =
                        serde_json::from_str(line).unwrap_or_default();
                    let msg_part = leptos::ssr::render_to_string(move |cx| {
                        view! { cx, <div hx-swap-oob="beforeend:#ai-msg-1">{api_response.response}</div> }
                    });
                    address.do_send(TextMessage(msg_part));
                }
            }
            Err(err) => {
                eprintln!("Error while receiving chunk: {}", err);
            }
        }
    }
    Ok(())
}
