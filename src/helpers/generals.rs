use crate::models::general::llm::Message;

pub fn extends_ai_functions(ai_function: fn(&str)-> &'static str, fun_input: &str) -> Message  {
    let ai_function_str = ai_function(fun_input);
    
    // Extend the string to encurage to only printing the output
    let msg = format!("FUNCTION: {} 
        INSTRUCTION: you are a funtion printer. You ONLY print the results of funtions. 
        Nothing else. No commentary. Here is the input to the function: {}.
        Print out what the function will return",
        ai_function_str, fun_input);
    // return the message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::ai_funtions::aifunc_managing::convert_user_input_to_goal;
    #[test]
    fn test_extends_ai_functions() {
        let extended_messages:Message = extends_ai_functions(convert_user_input_to_goal, "dummy var");
        assert!(extended_messages.role == "system".to_string());
        dbg!(extended_messages.content);
    }
}