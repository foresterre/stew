use clap::Arg;
use stew_lib::get_app_skeleton;
use stew_lib::operations::operation_by_name;
use stew_lib::operations::OpArg;
use stew_lib::run;

const COMMAND_NAME: &str = "blur";
const FIRST_ARG: &str = "σ";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME).arg(
        Arg::with_name(FIRST_ARG)
            .help(
                "σ represents the amount to blur by. It should be a 32 bit floating point number.",
            )
            .required(true),
    );

    let matches = app.get_matches();

    if let Some(input1) = matches.value_of(FIRST_ARG) {
        let parsed = input1
            .parse::<f32>()
            .map_err(|_| "The first argument of the blur command should be a floating point.")?;

        let op = operation_by_name(COMMAND_NAME, OpArg::FloatingPoint(parsed));

        run(&matches, Some(op?))
    } else {
        Err(format!("{} definition was unexpected.", COMMAND_NAME))
    }
}
