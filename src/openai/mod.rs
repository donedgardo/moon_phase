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
    embedding: Vec<f32>
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

pub async fn get_embeddings(content: &str, host: &str, model: &str) -> Result<Vec<f32>, reqwest::Error> {
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
    use chromadb::v1::{ChromaClient};
    use chromadb::v1::collection::{CollectionEntries, QueryOptions};
    use super::*;
    #[tokio::test]
    async fn get_embeddings_returns_empty() {
        let embeddings = get_embeddings("", "", "dndai").await.unwrap();
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

    #[ignore]
    #[tokio::test]
    async fn stores_embeddings_chroma_integration() {
        let client = ChromaClient::new(Default::default());
        let create_collection_response = client.get_or_create_collection("SRD-test", None);
        assert!(create_collection_response.is_ok());
        let collection = create_collection_response.unwrap();
        let pdf_chunk = "Mage Armor";
        let pdf_chunk_2 = "Shield";
        let embedded_pdf_chunk = get_embeddings(pdf_chunk, "http://localhost:11434", "dndai").await.unwrap();
        let embedded_pdf_chunk_2 = get_embeddings(pdf_chunk_2, "http://localhost:11434", "dndai").await.unwrap();
        let collection_entries = CollectionEntries {
            ids: vec![pdf_chunk, pdf_chunk_2],
            embeddings: Some(vec![embedded_pdf_chunk, embedded_pdf_chunk_2]),
            metadatas: None,
            documents: Some(vec![
                pdf_chunk.into(),
                pdf_chunk_2.into()
            ])
        };
        let collection_upsert_result = collection.upsert(collection_entries, None);
        assert!(collection_upsert_result.is_ok());

        let prompt = "Mage spells";
        let embedded_prompt =
            get_embeddings(prompt, "http://localhost:11434", "dndai").await.unwrap();
        let similarity_query = QueryOptions {
            query_texts: None,
            query_embeddings: Some(vec![embedded_prompt]),
            where_metadata: None,
            where_document: None,
            n_results: Some(1),
            include: None,
        };
        let query_result = collection.query(similarity_query, None).unwrap();
        println!("{:?}", query_result);
        assert!(query_result.documents.is_some());
        assert_eq!(query_result.ids[0][0], "Mage Armor");
    }
}