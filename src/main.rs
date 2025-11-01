#[macro_use]
extern crate rocket;

use std::path::PathBuf;
use dotenv::dotenv;
use openai::{chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole}, Credentials};
use rocket::{routes, Config, http::ContentType};

const PROMPT: &'static str = "\
You are managing the response for The Infinite Website, a website that has an infinite number of endpoints and paths. \
This is achieved using an AI-generated response based on the given path/endpoint. \
You will be provided with a path/endpoint, and you must respond ONLY with a fitting response. \
Do not include any indication of language (```json, ```xml, etc.), only respond in plain text.
This response could be HTML, JSON, or whatever seems to fit the requested path. \
The content of your response should match what the user may be trying to \
retrieve from the given path/endpoint as closely as possible. Ensure your response is correct and valid (CaSe SeNsItIvE) \
e.g. <!DOCTYPE html> instead of <!doctype html>. If possible for the selected response format, use CSS styling.\
";

#[get("/<path..>")]
async fn respond(path: PathBuf) -> (ContentType, String) {
	dotenv().ok();

	let full_path = String::from("/") + path.to_str().unwrap();
	let credentials = Credentials::from_env();

	println!("AI will provide response for this path: {}", &full_path);

	let messages = vec![
		ChatCompletionMessage {
			role: ChatCompletionMessageRole::System,
			content: Some(PROMPT.to_string()),
			name: None,
			function_call: None,
			tool_call_id: None,
			tool_calls: None
		},
		ChatCompletionMessage {
			role: ChatCompletionMessageRole::User,
			content: Some(full_path),
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
	rocket::build()
	.configure(Config {
		
		..Config::default()
	})
	.mount("/", routes![respond])
}