use reqwest::Client;
use serde::Deserialize;

use crate::apis::call_request::call_gpt;
use crate::models::general::llm::Message;
use crate::helpers::command_line::PrintCommand;
use serde::de::DeserializeOwned;

const CODE_TEMPLACE_PATH: &str = "/Users/dkkam/Projects/study/study-rust/auto_gpt/webtemplate/src/code_templace.rs";
const EXEC_MAIN_PATH: &str = "/Users/dkkam/Projects/study/study-rust/auto_gpt/webtemplate/src/main.rs";
const API_SCHEMA_PATH: &str = "/Users/dkkam/Projects/study/study-rust/auto_gpt/auto_gippity/src/schemas/api_schema.json";

pub fn extend_ai_function(ai_fun: fn(&str) ->&'static str, func_input: &str) -> Message {
    let ai_function_str = ai_fun(func_input);
    // dbg!(ai_function_str);

    // Extend the string to encourage only printing output
    let msg: String = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function {}.
    Print out what the function will return.",
    ai_function_str, func_input);
    // dbg!(msg);

    Message {
        role: "system".to_string(),
        content: msg
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    let llm_response_res = call_gpt(vec![extended_msg.clone()]).await;

    // Handle Success
    match llm_response_res {
        Ok(llm_response) => llm_response,
        Err(_) => {
            call_gpt(vec![extended_msg.clone()])
                .await
                .expect("Failed to get LLM response")
        }
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response = ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(&llm_response)
        .expect("Failed to decode LLM response");

    decoded_response
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Templace
pub fn read_code_template_contents() -> String {
    std::fs::read_to_string(CODE_TEMPLACE_PATH).expect("Failed to read code template")
}

// Save New backend code
pub fn save_backend_code(code: &str) {
    std::fs::write(EXEC_MAIN_PATH, code).expect("Failed to write backend code");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &str) {
    std::fs::write(API_SCHEMA_PATH, api_endpoints).expect("Failed to write API endpoints");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {
        let response = extend_ai_function(convert_user_input_to_goal, "dummy variable");
        assert_eq!(response.role, "system");
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param = "Build me webserver for making stock price api request.".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirement",
            convert_user_input_to_goal,
        )
        .await;
        
        assert!(res.len() > 20);
    }
}
