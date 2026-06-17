pub fn parse_flag(raw: &str) -> bool {
    let lowered = raw.trim().to_lowercase();
    lowered == "true" || lowered == "yes" || lowered == "1"
}
