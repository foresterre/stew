use clap::AppSettings;
use clap::Arg;
use stew_lib::get_app_skeleton;
use stew_lib::operations::operation_by_name;
use stew_lib::operations::OpArg;
use stew_lib::run;
use stew_lib::run_display_licenses;

const COMMAND_NAME: &str = "huerotate";
const ARG1: &str = "VALUE";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(ARG1)
                .help(
                    "Rotate the hue of each pixel of the input image by VALUE degrees. \
                    A complete revolution is 360 degrees. This means that 0 degrees is the same as 360 degrees, \
                    and in both cases, the hue of the pixels of the input image do not change.",
                )
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(1),
        )
        .global_setting(AppSettings::AllowLeadingHyphen);
    ;

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches)
    } else {
        match matches.value_of(ARG1) {
            Some(v) => {
                let op = operation_by_name(COMMAND_NAME, OpArg::Integer(parse_i32(v)?));

                run(&matches, Some(op?))
            }
            _ => Err("Huerotate requires exactly 1 argument (32 bit integer).".to_string()),
        }
    }
}

fn parse_i32(input: &str) -> Result<i32, String> {
    input.parse::<i32>().map_err(|_| {
        "The argument of the huerotate command should be an integer (i32).".to_string()
    })
}
