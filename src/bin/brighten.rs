use clap::AppSettings;
use clap::Arg;
use combostew::operations::operation_by_name;
use combostew::operations::OpArg;
use combostew_cli::get_default_config;
use combostew_cli::run;
use combostew_cli::run_display_licenses;

const COMMAND_NAME: &str = "brighten";
const ARG1: &str = "VALUE";

fn main() -> Result<(), String> {
    let app = stew_lib::stew_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(ARG1)
                .help(
                    "VALUE represents the amount to brighten by. This value can both be positive \
                     (increase brightness) or negative (decrease brightness). \
                     The VALUE should be a 32 bit integer.",
                )
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(1),
        )
        .global_setting(AppSettings::AllowLeadingHyphen);

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches, stew_lib::get_tool_name(), Vec::new())
    } else {
        match matches.value_of(ARG1) {
            Some(v) => {
                let op = operation_by_name(COMMAND_NAME, OpArg::Integer(parse_i32(v)?));

                let config = get_default_config(&matches, stew_lib::get_tool_name(), Vec::new())?;
                run(&matches, &mut [op?], &config)
            }
            _ => Err("Brighten requires exactly 1 argument (32 bit integer).".to_string()),
        }
    }
}

fn parse_i32(input: &str) -> Result<i32, String> {
    input
        .parse::<i32>()
        .map_err(|_| "The argument of the brighten command should be an integer (i32).".to_string())
}
