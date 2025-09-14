pub fn talking(text: &str) -> &str {
    let t = text.trim();
    if t.is_empty() {
        return "Just say something!";
    }

    let is_question = t.ends_with('?');
    let has_alpha = t.chars().any(|c| c.is_alphabetic());
    let all_upper = has_alpha && t.chars().all(|c| !c.is_lowercase());

    match (all_upper, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
