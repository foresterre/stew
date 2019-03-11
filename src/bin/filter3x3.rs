use clap::AppSettings;
use clap::Arg;
use combostew::get_app_skeleton;
use combostew::operations::operation_by_name;
use combostew::operations::OpArg;
use combostew::run;
use combostew::run_display_licenses;

const COMMAND_NAME: &str = "filter3x3";
const ARG1: &str = "F1";
const ARG2: &str = "F2";
const ARG3: &str = "F3";
const ARG4: &str = "S1";
const ARG5: &str = "S2";
const ARG6: &str = "S3";
const ARG7: &str = "T1";
const ARG8: &str = "T2";
const ARG9: &str = "T3";

fn main() -> Result<(), String> {
    let app = get_app_skeleton(COMMAND_NAME)
        .arg(
            Arg::with_name(ARG1)
                .help("First element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(1),
        )
        .arg(
            Arg::with_name(ARG2)
                .help("Second element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(2),
        )
        .arg(
            Arg::with_name(ARG3)
                .help("Third element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(3),
        )
        .arg(
            Arg::with_name(ARG4)
                .help("Fourth element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(4),
        )
        .arg(
            Arg::with_name(ARG5)
                .help("Fifth element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(5),
        )
        .arg(
            Arg::with_name(ARG6)
                .help("Sixth element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(6),
        )
        .arg(
            Arg::with_name(ARG7)
                .help("Seventh element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(7),
        )
        .arg(
            Arg::with_name(ARG8)
                .help("Eighth element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(8),
        )
        .arg(
            Arg::with_name(ARG9)
                .help("Ninth element of the 3x3 box filter (32 bit floating point).")
                .takes_value(true)
                .number_of_values(1)
                .allow_hyphen_values(true)
                .required_unless_one(&["license", "dep_licenses"])
                .index(9),
        )
        .global_setting(AppSettings::AllowLeadingHyphen);

    let matches = app.get_matches();
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    if license_display {
        run_display_licenses(&matches, stew_lib::get_tool_name())
    } else {
        // uh oh
        match (
            matches.value_of(ARG1),
            matches.value_of(ARG2),
            matches.value_of(ARG3),
            matches.value_of(ARG4),
            matches.value_of(ARG5),
            matches.value_of(ARG6),
            matches.value_of(ARG7),
            matches.value_of(ARG8),
            matches.value_of(ARG9),
        ) {
            (
                Some(f1),
                Some(f2),
                Some(f3),
                Some(s1),
                Some(s2),
                Some(s3),
                Some(t1),
                Some(t2),
                Some(t3),
            ) => {
                let arr: [f32; 9] = [
                    parse_fp32(f1)?,
                    parse_fp32(f2)?,
                    parse_fp32(f3)?,
                    parse_fp32(s1)?,
                    parse_fp32(s2)?,
                    parse_fp32(s3)?,
                    parse_fp32(t1)?,
                    parse_fp32(t2)?,
                    parse_fp32(t3)?,
                ];

                let op = operation_by_name(COMMAND_NAME, OpArg::FloatingPointArray9(arr));

                run(&matches, Some(op?), stew_lib::get_tool_name())
            }
            _ => Err(
                "Filter3x3 requires exactly 9 arguments (32 bit floating point number)."
                    .to_string(),
            ),
        }
    }
}

fn parse_fp32(input: &str) -> Result<f32, String> {
    input.parse::<f32>().map_err(|_| {
        "Arguments of the filter3x3 command should be floating point numbers (f32).".to_string()
    })
}
