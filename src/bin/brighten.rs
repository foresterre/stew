use clap::AppSettings;
use clap::Arg;
use stew_lib::get_app_skeleton;
use stew_lib::operations::operation_by_name;
use stew_lib::operations::OpArg;
use stew_lib::run;

const COMMAND_NAME: &str = "brighten";
const FIRST_ARG: &str = "VALUE";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(FIRST_ARG)
                .help(
                    "VALUE represents the amount to brighten by. This value can both be positive \
                     (increase brightness) or negative (decrease brightness). \
                     The VALUE should be a 32 bit integer.",
                )
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true) // TODO{}: This doesn't seem to work without
                .required_unless_one(&["license", "dep_licenses"])
                .index(1),
        )
        .global_setting(AppSettings::AllowLeadingHyphen);

    let matches = app.get_matches();

    if let Some(input1) = matches.value_of(FIRST_ARG) {
        let parsed = input1
            .parse::<i32>()
            .map_err(|_| "The argument of the brighten command should be an integer.")?;

        let op = operation_by_name(COMMAND_NAME, OpArg::Integer(parsed));

        run(&matches, Some(op?))
    } else {
        Err(format!("{} definition was unexpected.", COMMAND_NAME))
    }
}
