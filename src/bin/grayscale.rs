use combostew::operations::operation_by_name;
use combostew::operations::OpArg;
use combostew_cli::get_default_config;
use combostew_cli::run;
use combostew_cli::run_display_licenses;

const COMMAND_NAME: &str = "grayscale";

fn main() -> Result<(), String> {
    let app = stew_lib::stew_app_skeleton(COMMAND_NAME);

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches, stew_lib::get_tool_name(), Vec::new())
    } else {
        let op = operation_by_name(COMMAND_NAME, OpArg::Empty);

        let config = get_default_config(&matches, stew_lib::get_tool_name(), Vec::new())?;
        run(&matches, &mut [op?], &config)
    }
}
