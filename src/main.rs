mod ai_funtions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let user_response = get_user_response("What webserver are we building today?");

    dbg!(user_response);
}
