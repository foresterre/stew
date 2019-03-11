use combostew::get_app_skeleton;
use combostew::operations::operation_by_name;
use combostew::operations::OpArg;
use combostew::run;
use combostew::run_display_licenses;

const COMMAND_NAME: &str = "rotate270";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME);

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches, stew_lib::get_tool_name())
    } else {
        let op = operation_by_name(COMMAND_NAME, OpArg::Empty);

        run(&matches, Some(op?), stew_lib::get_tool_name())
    }
}
