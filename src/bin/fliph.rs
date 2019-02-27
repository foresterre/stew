use stew_lib::get_app_skeleton;
use stew_lib::operations::operation_by_name;
use stew_lib::operations::OpArg;
use stew_lib::run;
use stew_lib::run_display_licenses;

const COMMAND_NAME: &str = "fliph";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME);

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches)
    } else {
        let op = operation_by_name(COMMAND_NAME, OpArg::Empty);

        run(&matches, Some(op?))
    }
}
