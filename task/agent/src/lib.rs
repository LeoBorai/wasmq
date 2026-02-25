use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::openrouter::Client;

use mate_task::{mate_handler, mate_object};

#[mate_object]
struct Params {
    api_key: String,
    model: String,
    prompt: String,
}

#[mate_object]
struct Response {
    response: String,
}

#[mate_handler]
async fn prompt_agent(params: Params) -> Result<Response> {
    wstd::runtime::block_on(async move {
        let client: Client = Client::new(params.api_key).unwrap();
        let model = client.agent(params.model).build();
        let response = model
            .prompt(params.prompt)
            .await
            .expect("Failed to prompt");

        Ok(Response { response })
    })
}
