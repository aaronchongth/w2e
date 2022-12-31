use futures::select;
use reqwest::Client;
use serde_json::json;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    env_logger::init();

    println!("Opening session...");
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let key_expr: String = String::from("magic_mirror/w2e");
    println!("Declaring Subscriber on '{}'...", &key_expr);

    let subscriber =
        session.declare_subscriber(&key_expr).res().await.unwrap();

    println!("Enter 'Ctrl+C' to quit...");
    loop {
        select!(
            sample = subscriber.recv_async() => {
                let sample = sample.unwrap();
                println!(">> [Subscriber] Received {} ('{}': '{}')",
                    sample.kind, sample.key_expr.as_str(), sample.value);

                // let choice: &String = sample.value.unwrap();
                let message_body = json!({
                    "message": format!("{}", sample.value),
                });
                let request_url = "http://localhost:8080/IFTTT";
                let response = Client::new()
                    .post(request_url)
                    .json(&message_body)
                    .send()
                    .await;
                println!("Response: {:?}", response);
            },
        );
    }
}