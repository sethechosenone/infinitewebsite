#[macro_use]
extern crate rocket;

use std::path::PathBuf;
use dotenv::dotenv;
use openai::{chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole}, Credentials};
use rocket::{routes, http::{ContentType, uri::Origin}, Data, data::ToByteUnit};

const SYSTEM_PROMPT: &'static str = "\
You are managing the response for The Infinite Website, a website that has an infinite number of endpoints and paths. \
This is achieved using an AI-generated response based on the given path/endpoint. \
You will be provided with a request method and path/endpoint, and you must respond ONLY with a fitting response. \
Do not include any indication of language (```json, ```xml, etc.), only respond in plain text.
This response could be HTML, JSON, or whatever seems to fit the requested path. \
The content of your response should match what the user may be trying to \
retrieve from the given path/endpoint as closely as possible. Ensure your response is correct and valid (CaSe SeNsItIvE) \
e.g. <!DOCTYPE html> instead of <!doctype html>. If possible for the selected response format, use CSS styling.";

#[get("/<path..>")]
async fn respond(path: PathBuf, uri: &Origin<'_>) -> (ContentType, String) {
	dotenv().ok();

	let query_string = uri.query().map(|q| q.as_str()).unwrap_or("");
	let full_request = if query_string.is_empty() {
		format!("GET /{}", path.display())
	} else {
		format!("GET /{}?{}", path.display(), query_string)
	};
	let credentials = Credentials::from_env();

	println!("AI will provide response for this path: {}", &full_request);

	let messages = vec![
		ChatCompletionMessage {
			role: ChatCompletionMessageRole::System,
			content: Some(SYSTEM_PROMPT.to_string()),
			name: None,
			function_call: None,
			tool_call_id: None,
			tool_calls: None
		},
		ChatCompletionMessage {
			role: ChatCompletionMessageRole::User,
			content: Some(full_request),
			name: None,
			function_call: None,
			tool_call_id: None,
			tool_calls: None
		}
	];
	
	println!("Messages:");
	println!(" - system: {:?}", messages[0].content.as_ref().unwrap());
	println!(" - user: {:?}", messages[1].content.as_ref().unwrap());
	
	let chat_completion = ChatCompletion::builder("gpt-5-nano", messages)
		.credentials(credentials)
		.create()
		.await
		.unwrap();

	let content = chat_completion.choices.first().unwrap().message.content.clone().unwrap_or(String::from("[no response]"));
	let content_type = if content.trim_start().starts_with("<!DOCTYPE") || content.trim_start().starts_with("<html") {
		ContentType::HTML
	} else if content.trim_start().starts_with("{") || content.trim_start().starts_with("[") {
		ContentType::JSON
	} else if content.trim_start().starts_with("<?xml") {
		ContentType::XML
	} else {
		ContentType::Plain
	};

	println!("Detected content type from AI response: {}", &content_type.to_string());
	
	(content_type, content)
}

#[post("/<path..>", data = "<body>")]
async fn respond_post(path: PathBuf, uri: &Origin<'_>, body: Data<'_>) -> (ContentType, String) {
	dotenv().ok();

	let query_string = uri.query().map(|q| q.as_str()).unwrap_or("");
	let body_string = match body.open(8.kilobytes()).into_string().await {
		Ok(s) => s.into_inner(),
		Err(_) => String::new()
	};
	
	let full_request = if query_string.is_empty() && body_string.is_empty() {
		format!("POST /{}", path.display())
	} else if query_string.is_empty() {
		format!("POST /{}  Body: {}", path.display(), body_string)
	} else if body_string.is_empty() {
		format!("POST /{}?{}", path.display(), query_string)
	} else {
		format!("POST /{}?{}  Body: {}", path.display(), query_string, body_string)
	};
	
	let credentials = Credentials::from_env();

	println!("AI will provide response for this POST request: {}", &full_request);

	let messages = vec![
		ChatCompletionMessage {
			role: ChatCompletionMessageRole::System,
			content: Some(SYSTEM_PROMPT.to_string()),
			name: None,
			function_call: None,
			tool_call_id: None,
			tool_calls: None
		},
		ChatCompletionMessage {
			role: ChatCompletionMessageRole::User,
			content: Some(full_request),
			name: None,
			function_call: None,
			tool_call_id: None,
			tool_calls: None
		}
	];
	
	println!("Messages:");
	println!(" - system: {:?}", messages[0].content.as_ref().unwrap());
	println!(" - user: {:?}", messages[1].content.as_ref().unwrap());
	
	let chat_completion = ChatCompletion::builder("gpt-5-nano", messages)
		.credentials(credentials)
		.create()
		.await
		.unwrap();

	let content = chat_completion.choices.first().unwrap().message.content.clone().unwrap_or(String::from("[no response]"));
	let content_type = if content.trim_start().starts_with("<!DOCTYPE") || content.trim_start().starts_with("<html") {
		ContentType::HTML
	} else if content.trim_start().starts_with("{") || content.trim_start().starts_with("[") {
		ContentType::JSON
	} else if content.trim_start().starts_with("<?xml") {
		ContentType::XML
	} else {
		ContentType::Plain
	};

	println!("Detected content type from AI response: {}", &content_type.to_string());
	
	(content_type, content)
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![respond, respond_post])
}