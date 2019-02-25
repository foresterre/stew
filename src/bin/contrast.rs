use clap::AppSettings;
use clap::Arg;
use stew_lib::get_app_skeleton;
use stew_lib::operations::operation_by_name;
use stew_lib::operations::OpArg;
use stew_lib::run;

const COMMAND_NAME: &str = "contrast";
const FIRST_ARG: &str = "VALUE";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(FIRST_ARG)
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

    if let Some(input1) = matches.value_of(FIRST_ARG) {
        let parsed = input1
            .parse::<f32>()
            .map_err(|_| "The argument of the contrast command should be a floating point number (32 bits).")?;

        let op = operation_by_name(COMMAND_NAME, OpArg::FloatingPoint(parsed));

        run(&matches, Some(op?))
    } else {
        Err(format!("{} definition was unexpected.", COMMAND_NAME))
    }
}
