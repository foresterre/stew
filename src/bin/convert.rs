use combostew::get_default_config;
use combostew::run;

const COMMAND_NAME: &str = "convert";

fn main() -> Result<(), String> {
    let app = stew_lib::stew_app_skeleton(COMMAND_NAME);
    let matches = app.get_matches();

    let config = get_default_config(&matches, stew_lib::get_tool_name(), Vec::new())?;
    run(&matches, &mut [], &config)
}
