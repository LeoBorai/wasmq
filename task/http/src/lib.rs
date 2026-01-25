use serde_json::Value;
use wstd::http::{Body, BodyExt, Client, Method, Request};

use mate_task::{mate_handler, mate_object};

#[mate_object]
struct Config {
    api_url: String,
    data: Value,
}

#[mate_object]
struct Response {
    status: u16,
    body: Value,
}

#[mate_handler]
async fn send_http_request(config: Config) -> Result<Response> {
    wstd::runtime::block_on(async move {
        let client = Client::new();
        let mut request = Request::builder();
        request = request.uri(config.api_url).method(Method::POST);

        let body = Body::from_json(&config.data).expect("Bad body");
        let request = request.body(body).unwrap();
        let response = client.send(request).await.unwrap();
        let status = response.status().as_u16();
        let body = response.into_body().into_boxed_body().collect().await?;
        let bytes = body.to_bytes();
        let json = serde_json::from_slice::<serde_json::Value>(&bytes)?;

        Ok(Response { status, body: json })
    })
}
