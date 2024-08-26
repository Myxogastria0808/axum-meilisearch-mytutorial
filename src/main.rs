use axum::{extract::Query, routing::get, Extension, Json, Router};
use dotenvy::dotenv;
use meilisearch_sdk::client::Client;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct SampleData {
    id: usize,
    title: String,
    family_name: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SearchWord {
    word: String,
}

#[tokio::main]
async fn main() {
    //let _ = runner().await;
    let _ = api().await;
}

//initial runner
async fn runner() {
    let client: Client = Client::new(
        "http://127.0.0.1:7700",
        Some("daea14da9387e21dc4ab483ac657551e9f302e3cb418c8c81216cd92c483c984"),
    )
    .unwrap();

    //set document name
    // An index is where the documents are stored.
    let sample_index: meilisearch_sdk::indexes::Index = client.index("sample");

    //add index data
    // Add some movies in the index. If the index 'movies' does not exist, Meilisearch creates it when you first add the documents.
    sample_index
        .add_documents(
            &[
                SampleData {
                    id: 1,
                    title: "Hello".to_string(),
                    family_name: vec![
                        "Alice".to_string(),
                        "Bob".to_string(),
                        "Charlie".to_string(),
                    ],
                },
                SampleData {
                    id: 2,
                    title: "Hello".to_string(),
                    family_name: vec!["Henly".to_string(), "Nyaki".to_string(), "Cacy".to_string()],
                },
                SampleData {
                    id: 3,
                    title: "Hello".to_string(),
                    family_name: vec!["Yamada".to_string(), "Bob".to_string(), "Kevin".to_string()],
                },
            ],
            Some("id"),
        )
        .await
        .unwrap();
}

//axum
async fn api() {
    // Declaration and initialization of static variable
    static MEILI_URL: OnceCell<String> = OnceCell::new();
    static ADMIN_API_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = MEILI_URL.set(env::var("MEILI_URL").expect("KEY not found in .env file."));
    let _ = ADMIN_API_KEY.set(env::var("ADMIN_API_KEY").expect("KEY not found in .env file."));
    let client = Client::new(MEILI_URL.get().unwrap(), Some(ADMIN_API_KEY.get().unwrap())).unwrap();
    //Router
    let app = Router::new()
        .route("/search", get(search_handler))
        .layer(Extension(client));
    //Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

//handler
async fn search_handler(
    Query(query): Query<SearchWord>,
    Extension(client): Extension<Client>,
) -> Json<Vec<SampleData>> {
    let result: Vec<SampleData> = client
        .index("sample")
        .search()
        .with_query(&query.word)
        .execute::<SampleData>()
        .await
        .unwrap()
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    Json(result)
}
