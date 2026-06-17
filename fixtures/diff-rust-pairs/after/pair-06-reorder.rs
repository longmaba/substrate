pub fn label(value: &str) -> String {
    format!("label:{}", normalize(value))
}

fn normalize(value: &str) -> String {
    value.trim().to_lowercase()
}
