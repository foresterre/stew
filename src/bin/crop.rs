use clap::Arg;
use combostew::get_app_skeleton;
use combostew::operations::operation_by_name;
use combostew::operations::OpArg;
use combostew::run;
use combostew::run_display_licenses;

const COMMAND_NAME: &str = "crop";
const ARG1: &str = "LX";
const ARG2: &str = "LY";
const ARG3: &str = "RX";
const ARG4: &str = "RY";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(ARG1)
                .help(
                    "X component of the X and Y coordinates of the top left selection point. \
                     LX should be a 32 bit unsigned integer.",
                )
                .takes_value(true)
                .number_of_values(1)
                .required_unless_one(&["license", "dep_licenses"])
                .index(1),
        )
        .arg(
            Arg::with_name(ARG2)
                .help(
                    "Y component of the X and Y coordinates of the top left selection point. \
                     LY should be a 32 bit unsigned integer.",
                )
                .takes_value(true)
                .number_of_values(1)
                .required_unless_one(&["license", "dep_licenses"])
                .index(2),
        )
        .arg(
            Arg::with_name(ARG3)
                .help(
                    "X component of the X and Y coordinates of the bottom right selection point. \
                     RX should be a 32 bit unsigned integer.",
                )
                .takes_value(true)
                .number_of_values(1)
                .required_unless_one(&["license", "dep_licenses"])
                .index(3),
        )
        .arg(
            Arg::with_name(ARG4)
                .help(
                    "X component of the X and Y coordinates of the bottom right selection point. \
                     RY should be a 32 bit unsigned integer.",
                )
                .takes_value(true)
                .number_of_values(1)
                .required_unless_one(&["license", "dep_licenses"])
                .index(4),
        );

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches, stew_lib::get_tool_name())
    } else {
        match (
            matches.value_of(ARG1),
            matches.value_of(ARG2),
            matches.value_of(ARG3),
            matches.value_of(ARG4),
        ) {
            (Some(lx), Some(ly), Some(rx), Some(ry)) => {
                let op = operation_by_name(
                    COMMAND_NAME,
                    OpArg::UnsignedIntegerTuple4(
                        parse_u32(lx, 1)?,
                        parse_u32(ly, 2)?,
                        parse_u32(rx, 3)?,
                        parse_u32(ry, 4)?,
                    ),
                );

                run(&matches, Some(op?), stew_lib::get_tool_name())
            }
            _ => Err("Crop requires exactly 4 arguments (32 bit unsigned integer).".to_string()),
        }
    }
}

fn parse_u32(input: &str, nth: usize) -> Result<u32, String> {
    input.parse::<u32>().map_err(|_| {
        format!(
            "The {}{} argument of the crop command should be an unsigned integer (u32).",
            nth,
            nth_str(nth)
        )
    })
}

fn nth_str(n: usize) -> &'static str {
    match n {
        x if x > 3 || x == 0 => "th",
        x if x == 3 => "rd",
        x if x == 2 => "nd",
        x if x == 1 => "st",
        _ => "th",
    }
}
