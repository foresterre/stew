use clap::AppSettings;
use clap::Arg;
use stew_lib::get_app_skeleton;
use stew_lib::operations::operation_by_name;
use stew_lib::operations::OpArg;
use stew_lib::run;
use stew_lib::run_display_licenses;

const COMMAND_NAME: &str = "contrast";
const ARG1: &str = "VALUE";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(ARG1)
                .help(
                    "VALUE represents the amount to adjust the contrast by. This value can both be positive \
                     (increase contrast) or negative (decrease contrast). \
                     The VALUE should be a 32 bit floating point number.",
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
        run_display_licenses(&matches)
    } else {
        match matches.value_of(ARG1) {
            Some(v) => {
                let op = operation_by_name(COMMAND_NAME, OpArg::FloatingPoint(parse_f32(v)?));

                run(&matches, Some(op?))
            }
            _ => Err("Contrast requires exactly 1 argument (32 bit floating point).".to_string()),
        }
    }
}

fn parse_f32(input: &str) -> Result<f32, String> {
    input
        .parse::<f32>()
        .map_err(|_| "The argument of the contrast command should be an integer (i32).".to_string())
}
