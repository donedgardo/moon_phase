use actix::dev::ToEnvelope;
use actix::{Actor, Addr, Handler, Message};
use actix_web_actors::ws::WebsocketContext;
use leptos::{view, IntoAttribute};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenerateApiResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbeddingApiResponse {
    embedding: Vec<f64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequestPayload {
    model: String,
    prompt: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

pub async fn consume_streaming_api<T>(
    url: &str,
    model: &str,
    prompt: &str,
    address: Addr<T>,
    id: &str,
) -> Result<(), reqwest::Error>
where
    T: Actor<Context = WebsocketContext<T>> + Handler<TextMessage, Result = ()>,
    T::Context: ToEnvelope<T, TextMessage>,
{
    let client = Client::new();
    let payload = GenerateRequestPayload {
        model: model.to_string(),
        prompt: prompt.to_string(),
    };
    let mut response_stream = client.post(url).json(&payload).send().await?.bytes_stream();
    while let Some(chunk) = response_stream.next().await {
        match chunk {
            Ok(bytes) => {
                let json_str = std::str::from_utf8(&bytes).unwrap_or_default();
                for line in json_str.lines() {
                    let target = format!("beforeend:#ai-msg-{}", id.to_string());
                    let api_response: GenerateApiResponse =
                        serde_json::from_str(line).unwrap_or_default();
                    let formatted_response = api_response.response.replace("\n", "<br>");
                    let msg_part = leptos::ssr::render_to_string(move |cx| {
                        view! { cx, <div hx-swap-oob=target inner_html=formatted_response></div> }
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

pub async fn get_embeddings(content: &str, host: &str, model: &str) -> Result<Vec<f64>, reqwest::Error> {
    if content.is_empty() {
        return Ok(vec![]);
    }
    let client = Client::new();
    let payload = GenerateRequestPayload {
        model: model.to_string(),
        prompt: content.to_string(),
    };
    let response = client.post(format!("{host}/api/embeddings"))
        .json(&payload).send().await?;
    match response.json::<EmbeddingApiResponse>().await {
        Ok(response) => Ok(response.embedding),
        _ => Ok(vec![])
    }
}

#[cfg(test)]
mod openai_test {
    use super::*;
    #[tokio::test]
    async fn get_embeddings_returns_empty() {
        let server = mockito::Server::new();
        let host = server.url();
        let embeddings = get_embeddings("", &host, "dndai").await.unwrap();
        assert_eq!(embeddings.len(), 0);
    }
    #[tokio::test]
    async fn get_embeddings_returns_vec() {
        let mut server = mockito::Server::new();
        let host = server.url();
        let mock = server.mock("POST", "/api/embeddings")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"embedding\": [0.5]}")
            .create();

        let embeddings = get_embeddings("hello!", &host, "dndai").await.unwrap();
        mock.assert();
        assert_eq!(embeddings.len(), 1);
    }
}
