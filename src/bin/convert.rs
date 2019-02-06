use stew_lib::{get_app_skeleton, run};

fn main() -> Result<(), String> {
    let matches = get_app_skeleton("stew").get_matches();

    run(&matches)
}
