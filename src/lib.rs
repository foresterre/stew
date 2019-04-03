use clap::App;
use combostew_cli::get_app_skeleton;

pub fn get_tool_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub fn stew_app_skeleton(name: &str) -> App<'static, 'static> {
    get_app_skeleton(name)
        .version(env!("CARGO_PKG_VERSION"))
        .about("This tool is part of the Stew image toolset. Stew is a set of image transformation tools, adapted from `sic`.")
        .after_help("For more information, visit: https://github.com/foresterre/stew")
}
