pub fn format_user(
    id: u32,
    name: &str,
) -> String
{
    format!("{}:{}", id, name)
}
