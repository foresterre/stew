use stew_lib::get_app_skeleton;
use stew_lib::run;

const COMMAND_NAME: &str = "convert";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME);
    let matches = app.get_matches();

    run(&matches, None)
}
