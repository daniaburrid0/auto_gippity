use crate::apis::call_request::call_gpt;
use crate::models::general::llm::Message;

use super::command_line::PrintCommand;

pub fn extends_ai_functions(ai_function: fn(&str) -> &'static str, fun_input: &str) -> Message {
    let ai_function_str = ai_function(fun_input);

    // Extend the string to encurage to only printing the output
    let msg = format!(
        "FUNCTION: {} 
        INSTRUCTION: you are a funtion printer. You ONLY print the results of funtions. 
        Nothing else. No commentary. Here is the input to the function: {}.
        Print out what the function will return",
        ai_function_str, fun_input
    );
    // return the message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Perform call to llm gpt
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend ai function
    let extended_msg = extends_ai_functions(function_pass, &msg_context);

    // Print current status
    PrintCommand::AiCall.print_agent_message(agent_position, agent_operation);

    // Get llm response
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    // Handle success
    let llm_resposne = match llm_response_res {
        Ok(llm_response) => llm_response,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to get llm response"),
    };
    return llm_resposne;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ai_funtions::aifunc_managing::convert_user_input_to_goal;
    #[test]
    fn test_extends_ai_functions() {
        let extended_messages: Message =
            extends_ai_functions(convert_user_input_to_goal, "dummy var");
        assert!(extended_messages.role == "system".to_string());
        dbg!(extended_messages.content);
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_fun_param = "Build me a webserver for making stock predictions".to_string();
        let llm_response = ai_task_request(
            ai_fun_param,
            "Managin Agent",
            "test test test test",
            convert_user_input_to_goal,
        )
        .await;
        dbg!(llm_response);
    }
}
