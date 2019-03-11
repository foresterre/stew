use combostew::get_app_skeleton;
use combostew::run;

const COMMAND_NAME: &str = "convert";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME);
    let matches = app.get_matches();

    run(&matches, None, stew_lib::get_tool_name())
}
