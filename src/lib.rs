pub fn get_tool_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
