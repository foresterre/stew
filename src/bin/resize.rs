use clap::Arg;
use combostew::operations::operation_by_name;
use combostew::operations::OpArg;
use combostew_cli::get_default_config;
use combostew_cli::run;
use combostew_cli::run_display_licenses;

const COMMAND_NAME: &str = "resize";
const ARG1: &str = "NEW_WIDTH";
const ARG2: &str = "NEW_HEIGHT'";

fn main() -> Result<(), String> {
    let app = stew_lib::stew_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(ARG1)
                .help("NEW_WIDTH' is the output image's width dimension.")
                .takes_value(true)
                .number_of_values(1)
                .required_unless_one(&["license", "dep_licenses"])
                .index(1),
        )
        .arg(
            Arg::with_name(ARG2)
                .help("NEW_HEIGHT' is the output image's height dimension.")
                .takes_value(true)
                .number_of_values(1)
                .required_unless_one(&["license", "dep_licenses"])
                .index(2),
        );

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches, stew_lib::get_tool_name(), Vec::new())
    } else {
        match (matches.value_of(ARG1), matches.value_of(ARG2)) {
            (Some(w), Some(h)) => {
                let op = operation_by_name(
                    COMMAND_NAME,
                    OpArg::UnsignedIntegerTuple2(parse_u32(w)?, parse_u32(h)?),
                );

                let config = get_default_config(&matches, stew_lib::get_tool_name(), Vec::new())?;
                run(&matches, &mut [op?], &config)
            }
            _ => Err("Resize requires exactly 2 arguments (32 bit unsigned integer).".to_string()),
        }
    }
}

fn parse_u32(input: &str) -> Result<u32, String> {
    input.parse::<u32>().map_err(|_| {
        "Both arguments of the resize command should be an unsigned integers (u32).".to_string()
    })
}
