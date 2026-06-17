pub fn parse_flag(raw: &str) -> bool {
    match raw {
        "true" => true,
        "yes" => true,
        _ => false,
    }
}
