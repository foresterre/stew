use clap::Arg;
use combostew::get_default_config;
use combostew::operations::operation_by_name;
use combostew::operations::OpArg;
use combostew::run;
use combostew::run_display_licenses;

const COMMAND_NAME: &str = "unsharpen";
const ARG1: &str = "σ";
const ARG2: &str = "THRESHOLD";

fn main() -> Result<(), String> {
    let app = stew_lib::stew_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(ARG1)
                .help(
                    "σ represents the amount to blur (Gaussian) by (contrasting the edges).",
                )
                .takes_value(true)
                .number_of_values(1)
                .required_unless_one(&["license", "dep_licenses"])
                .index(1),
        )
        .arg(
            Arg::with_name(ARG2)
                .help(
                    "THRESHOLD represents the minimal value which the difference between each specific \
                    pixel of the input image should have compared to same pixel blurred by σ, before the \
                    sharpen filter will be applied. Higher values sharpen less than lower values, because \
                    less values will surmount the set threshold.",
                )
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
        match (
            matches.value_of(ARG1),
            matches.value_of(ARG2)
        ) {
            (Some(w), Some(h)) => {
                let op = operation_by_name(
                    COMMAND_NAME,
                    OpArg::FloatingPointIntegerTuple2(
                        parse_f32(w)?,
                        parse_i32(h)?
                    ),
                );

                let config = get_default_config(&matches, stew_lib::get_tool_name(), Vec::new())?;
                run(&matches, &mut [op?], &config)
            }
            _ => Err("Unsharpen requires exactly 2 arguments, the first being a floating point number (32 bit) \
            and the second an integer (32 bit).".to_string()),
        }
    }
}

fn parse_f32(input: &str) -> Result<f32, String> {
    input.parse::<f32>().map_err(|_| {
        "The first argument of the unsharpen command should be a floating point number (f32)."
            .to_string()
    })
}

fn parse_i32(input: &str) -> Result<i32, String> {
    input.parse::<i32>().map_err(|_| {
        "The second argument of the unsharpen command should be an integer (i32).".to_string()
    })
}
