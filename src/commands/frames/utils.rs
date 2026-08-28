use regex::Regex;

/// Removes any text enclosed in angle brackets, e.g. "<internal note>".
pub async fn strip_angle_brackets(text: &str) -> String {

    let re = Regex::new(r"<[^>]*>").unwrap();

    re.replace_all(text, "").trim().to_string()
}
