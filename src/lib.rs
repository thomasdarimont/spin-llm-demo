
use spin_sdk::http::{IntoResponse, Json, Params, Request, ResponseBuilder, Router};
use spin_sdk::http_component;
use spin_sdk::llm;
use spin_sdk::llm::InferencingModel;
use uuid::Uuid;

mod prompt;
mod convo;

use prompt::Prompt;
use crate::convo::Conversation;

const CONVERSATION_ID_HEADER: &str = "X-ConversationID";


/// A simple Spin HTTP component.
#[http_component]
fn handle_spin_llm_demo(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/", handle_prompt);

    Ok(router.handle(req))
}

fn handle_prompt(req: http::Request<Json<Prompt>>, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let text = req.body().text.clone();

    let mut conversation = match req.headers().get(CONVERSATION_ID_HEADER) {
        Some(id) => {
            Conversation::load(id.to_str()?)
        }
        None => Ok(Conversation::new(Uuid::new_v4().to_string()))
    }?;

    let prompt = conversation.get_prompt(&text);

    let options = llm::InferencingParams {
        max_tokens: 100,
        temperature: 0.7,
        ..Default::default()
    };

    let result = llm::infer_with_options(InferencingModel::Llama2Chat, &prompt, options)?;

    conversation.add_interaction(&text, &result.text);
    conversation.store()?;

    Ok(ResponseBuilder::new(http::status::StatusCode::OK)
        .header(CONVERSATION_ID_HEADER, &conversation.id)
        .body(result.text)
        .build())
}

